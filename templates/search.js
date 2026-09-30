async function loadLunr() {
  for (const url of ["elasticlunr.min.js", "lunr.stemmer.support.js", "lunr.it.js"]) {
    const script = document.createElement("script");
    script.type = "text/javascript";
    script.src = "/" + url;
    await new Promise((resolve) => {
      script.onload = resolve;
      document.body.appendChild(script);
    });
  }
}

const lunrLoaded = loadLunr();

let index = null;
const maxResults = 15;

const searchResults = document.getElementById("search-results");
const searchInput = document.getElementById("searchbar");

async function search() {
  await lunrLoaded;
  if (index == null) {
    const indexData = await (await fetch("/search_index.it.json")).json();
    index = elasticlunr.Index.load(indexData);
    index.use(lunr.it);
  }
  const val = searchInput.value.trim();
  if (val === "") {
    searchResults.style.display = "none";
    return;
  }
  const queryNumbers = val.replaceAll('-', ' ').split(" ").filter((x) => x.match(/^\d+$/) && x != "");
  const results = index.search(val, {fields: {title: {boost: 3}, body: {boost: 1}, description: {boost: 2}}});
  for (const result of results) {
    result.score += result.doc.title.replaceAll('-', ' ').split(" ").some((x) => queryNumbers.some((y) => x == y)) ? 6 : 0;
    result.score += result.doc.description.replaceAll('-', ' ').split(" ").some((x) => queryNumbers.some((y) => x == y)) ? 4 : 0;
    result.score += result.doc.body.replaceAll('-', ' ').split(" ").some((x) => queryNumbers.some((y) => x == y)) ? 2 : 0;
  }
  results.sort((a, b) => b.score - a.score);
  searchResults.style.display = "block";
  searchResults.innerHTML = "";

  if (results.length === 0) {
    const div = document.createElement("div");
    div.className = "alert alert-warning py-2 px-3 mb-0 small text-center";
    div.innerHTML = `Nessun risultato per <b>${val}</b>`;
    searchResults.appendChild(div);
    return;
  }

  const headerDiv = document.createElement("div");
  headerDiv.className = "d-flex justify-content-between align-items-center mb-2 pb-2 border-bottom";
  headerDiv.innerHTML = `
    <span class="small fw-semibold text-secondary">${results.length} risultat${results.length === 1 ? 'o' : 'i'}</span>
    <button type="button" class="btn-close btn-close-sm" style="font-size: 0.7rem;" aria-label="Chiudi" onclick="document.getElementById('search-results').style.display='none'"></button>
  `;
  searchResults.appendChild(headerDiv);

  const searchResultsList = document.createElement("ul");
  searchResults.appendChild(searchResultsList);
  for (let i = 0; i < Math.min(results.length, maxResults); i++) {
    const li = document.createElement("li");
    li.className = "search-result-item";
    let ref = results[i].ref;
    if (ref in specialURIs) {
      ref = specialURIs[ref];
    }
    const desc = results[i].doc.description ? `<p class="small text-secondary mb-0 text-truncate">${results[i].doc.description}</p>` : '';
    li.innerHTML = `
      <a class="text-decoration-none text-body d-block" href="${ref}">
        <h6 class="text-primary mb-1 fw-semibold">${results[i].doc.title}</h6>
        ${desc}
      </a>
    `;
    searchResultsList.appendChild(li);
  }
}

let searchTimer = null;
if (searchInput) {
  searchInput.addEventListener('input', () => {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(search, 200);
  });
  searchInput.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      searchResults.style.display = 'none';
    }
  });
}

window.addEventListener('click', function (e) {
  if (searchResults.style.display === "block" && !searchResults.contains(e.target) && e.target !== searchInput) {
    searchResults.style.display = "none";
  }
});
