#!/usr/bin/env bash

set -xe

SHA="$1"
DESTINATION=$(realpath "$2")
STATUS_TOKEN="$3"

cd "$(realpath "$(dirname "$0")/..")"

[ -z "$STATUS_TOKEN" ] && echo "Usage: $0 SHA DESTINATION STATUS_TOKEN"
[ -z "$STATUS_TOKEN" ] && exit 1

exec 100>/tmp/website_preview.lock
flock 100

git checkout .
git fetch origin "$SHA"
git checkout "$SHA"

./scripts/updated_from_git.py
./scripts/download_gallery_images.py

# There is no Zola packages for Ubuntu, build using the docker image
docker run -u "$(id -u):$(id -g)" -v "$PWD:/app" --workdir /app ghcr.io/getzola/zola:v0.18.0 build

rm -rf "$DESTINATION"
cp -rl public/ "$DESTINATION"

curl -L \
  -X POST \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $STATUS_TOKEN" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/olimpiadi-informatica/website/statuses/$SHA \
  -d "{\"state\":\"success\",\"target_url\":\"https://$SHA.nightly.olimpiadi-informatica.it/\",\"description\":\"The build succeeded!\",\"context\":\"deploy\"}"


echo "All done!"
