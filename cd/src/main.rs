use std::{
    fs::{self, create_dir_all, read_link, remove_dir_all},
    path::PathBuf,
    sync::Arc,
};

use atomic_file_install::atomic_symlink_file;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use clap::Parser;
use cmd_lib::{run_cmd, spawn_with_output};
use color_eyre::eyre::{OptionExt, Result};
use reqwest::{header, Client};
use serde::Deserialize;
use tempfile::TempDir;
use tokio::sync::Mutex;
use tracing::{info, warn};
use walkdir::WalkDir;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to main deployment
    main_path: PathBuf,

    /// Path to nightly folder
    nightly_path: PathBuf,

    /// Secret code to authenticate requests
    secret: String,
}

#[derive(Debug)]
struct AppStateInner {
    config: Cli,
    main_mutex: Mutex<()>,
    nightly_mutex: Mutex<()>,
}

#[derive(Debug, Clone)]
struct AppState {
    state: Arc<AppStateInner>,
}

#[tokio::main]
async fn main() {
    use tracing_subscriber::{
        filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
    };

    let general_log = fmt::layer().with_filter(EnvFilter::from_default_env());
    let error_traces = tracing_error::ErrorLayer::default().with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(general_log)
        .with(error_traces)
        .init();

    color_eyre::install().unwrap();

    let mut config = Cli::parse();

    config.main_path = config.main_path.canonicalize().unwrap();
    config.nightly_path = config.nightly_path.canonicalize().unwrap();

    let state = AppState {
        state: Arc::new(AppStateInner {
            config,
            main_mutex: Mutex::new(()),
            nightly_mutex: Mutex::new(()),
        }),
    };

    let app = Router::new()
        .route("/main", post(run_main))
        .route("/nightly", post(run_nightly))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
fn build_inner(
    sha: Option<String>,
    build_base_path: PathBuf,
    repo_path: PathBuf,
    storage_path: PathBuf,
    output_symlink: Option<PathBuf>,
) -> Result<String> {
    let base_url = sha
        .as_ref()
        .map(|x| format!("https://{}.nightly.olimpiadi-informatica.it/", x));
    let target = {
        run_cmd!(git -C $repo_path checkout .)?;
        run_cmd!(git -C $repo_path clean -f -d)?;
        let sha = if let Some(sha) = sha {
            run_cmd!(git -C $repo_path fetch origin $sha)?;
            run_cmd!(git -C $repo_path checkout $sha)?;
            sha
        } else {
            run_cmd!(git -C $repo_path pull)?;
            spawn_with_output!(git -C $repo_path rev-parse HEAD)?.wait_with_output()?
        };
        fs::create_dir_all(&build_base_path)?;
        let target = build_base_path.join(sha);
        if !target.exists() {
            run_cmd!(git -C $repo_path submodule init)?;
            run_cmd!(git -C $repo_path submodule update --recursive)?;
            const LIB_PY: &[u8] = include_bytes!("../../scripts/lib.py");
            const UPDATED_FROM_GIT_PY: &[u8] = include_bytes!("../../scripts/updated_from_git.py");
            const DOWNLOAD_GALLERY_IMAGES_PY: &[u8] =
                include_bytes!("../../scripts/download_gallery_images.py");
            let scripts = TempDir::new_in(&repo_path).unwrap();
            fs::write(scripts.path().join("lib.py"), LIB_PY).unwrap();
            fs::write(
                scripts.path().join("updated_from_git.py"),
                UPDATED_FROM_GIT_PY,
            )
            .unwrap();
            fs::write(
                scripts.path().join("download_gallery_images.py"),
                DOWNLOAD_GALLERY_IMAGES_PY,
            )
            .unwrap();
            let script_dir = scripts.path();
            run_cmd!(cd $repo_path; /usr/bin/env python3 $script_dir/updated_from_git.py)?;
            run_cmd!(cd $repo_path; /usr/bin/env python3 $script_dir/download_gallery_images.py)?;
            if let Some(base_url) = &base_url {
                run_cmd!(cd $repo_path; unshare -nU zola build -u $base_url)?;
            } else {
                run_cmd!(cd $repo_path; unshare -nU zola build)?;
            }
            run_cmd!(cd $repo_path; cp -rl public/ $target)?;
            for entry in WalkDir::new(&target) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    let path = entry.path();
                    let contents = fs::read(path)?;
                    let hash = blake3::hash(&contents).to_hex().to_string();
                    let storage_file_path =
                        storage_path.join(&hash[0..2]).join(&hash[2..4]).join(hash);
                    if !storage_file_path.exists() {
                        create_dir_all(storage_file_path.parent().unwrap())?;
                        fs::rename(path, &storage_file_path)?;
                    } else {
                        fs::remove_file(path)?;
                    }
                    fs::hard_link(&storage_file_path, path)?;
                }
            }
        }
        target
    };

    if let Some(os) = output_symlink {
        let os_base_dir = os.parent().ok_or_eyre("symlink target is root?")?;
        let current = read_link(&os).map(|x| os_base_dir.join(x)).ok();
        let relative_target = pathdiff::diff_paths(&target, &os_base_dir)
            .ok_or_eyre("could not find relative path from prod symlink to build dir")?;
        atomic_symlink_file(&relative_target, &os)?;
        if let Some(current) = current {
            if current != target {
                if let Err(e) = remove_dir_all(&current) {
                    warn!(
                        "Error removing old directory {}: {e}",
                        current.to_string_lossy()
                    );
                }
            }
        }
    };

    Ok(base_url.unwrap_or_else(|| "https://www.olimpiadi-informatica.it/".to_string()))
}

#[tracing::instrument]
async fn report_status(sha: &Option<String>, gh_token: &Option<String>, status: String) {
    let Some(sha) = sha else {
        return;
    };
    let Some(gh_token) = gh_token else {
        return;
    };
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {gh_token}").parse().unwrap(),
    );
    headers.insert("User-Agent", "OII website CI".parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let client = Client::new();
    info!("updating GitHub status");
    let res = client
        .post(format!(
            "https://api.github.com/repos/olimpiadi-informatica/website/statuses/{sha}"
        ))
        .headers(headers)
        .body(status)
        .send()
        .await;
    match res {
        Err(err) => {
            warn!("{}", err);
        }
        Ok(resp) => match resp.text().await {
            Ok(resp) => info!("{}", resp),
            Err(err) => warn!("{}", err),
        },
    }
}

#[tracing::instrument(skip(mutex))]
async fn build(
    sha: Option<String>,
    build_base_path: PathBuf,
    repo_path: PathBuf,
    storage_path: PathBuf,
    output_symlink: Option<PathBuf>,
    gh_token: Option<String>,
    mutex: &Mutex<()>,
) -> Result<()> {
    report_status(
        &sha,
        &gh_token,
        "{\"state\":\"pending\",\"description\":\"building...\",\"context\":\"deploy\"}"
            .to_string(),
    )
    .await;
    let result = {
        let _g = mutex.lock().await;
        build_inner(
            sha.clone(),
            build_base_path,
            repo_path,
            storage_path,
            output_symlink,
        )
    };
    match result {
        Err(e) => {
            report_status(&sha, &gh_token, "{\"state\":\"failure\",\"description\":\"The build failed!\",\"context\":\"deploy\"}".to_string()).await;
            Err(e)
        }
        Ok(url) => {
            report_status(&sha, &gh_token, format!("{{\"state\":\"success\",\"target_url\":\"{url}\",\"description\":\"The build succeeded!\",\"context\":\"deploy\"}}")).await;
            Ok(())
        }
    }
}

#[derive(Deserialize, Debug)]
struct Params {
    secret: String,
    sha: Option<String>,
    gh_token: Option<String>,
}

#[tracing::instrument(skip(state))]
async fn run_main(State(state): State<AppState>, mut params: Json<Params>) -> StatusCode {
    let state = state.state;
    if state.config.secret != params.secret {
        return StatusCode::UNAUTHORIZED;
    }
    let build_base_path = state.config.main_path.join("builds");
    let storage_path = state.config.nightly_path.join("all_files");
    let repo_path = state.config.main_path.clone();
    let output_symlink = state.config.main_path.join("public-prod");

    if let Err(_) = build(
        None,
        build_base_path,
        repo_path,
        storage_path,
        Some(output_symlink),
        params.gh_token.take(),
        &state.main_mutex,
    )
    .await
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}

#[tracing::instrument(skip(state))]
async fn run_nightly(State(state): State<AppState>, mut params: Json<Params>) -> StatusCode {
    let state = state.state;
    if state.config.secret != params.secret {
        return StatusCode::UNAUTHORIZED;
    }
    let build_base_path = state.config.nightly_path.join("builds");
    let storage_path = state.config.nightly_path.join("all_files");
    let repo_path = state.config.nightly_path.join("website");

    if let Err(_) = build(
        params.sha.take(),
        build_base_path,
        repo_path,
        storage_path,
        None,
        params.gh_token.take(),
        &state.nightly_mutex,
    )
    .await
    {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::OK
}
