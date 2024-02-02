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
const maxResults = 20;

const searchResults = document.getElementById("search-results");

async function search() {
  await lunrLoaded;
  if (index == null) {
    const indexData = await (await fetch("/search_index.it.json")).json();
    index = elasticlunr.Index.load(indexData);
    index.use(lunr.it);
  }
  const val = document.getElementById("searchbar").value;
  if (val === "") return;
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
    div.classList = ["alert alert-warning"];
    div.innerHTML = `Nessun risultato per <b>${val}</b>`;
    searchResults.appendChild(div);
    return;
  }
  const searchResultsList = document.createElement("ul");
  searchResults.appendChild(searchResultsList);
  for (let i = 0; i < Math.min(results.length, maxResults); i++) {
    const li = document.createElement("li");
    // TODO: consider adding a summary of the article / first few words (results[i].doc.body).
    let ref = results[i].ref;
    if (ref in specialURIs) {
      ref = specialURIs[ref];
    }
    li.innerHTML = [
      `<div class="search-result-item">`,
      `<a class="link-secondary link-underline-opacity-0" href=${ref}><h3>${results[i].doc.title}</h3></a>`,
      `</div>`
    ].join("");
    searchResultsList.appendChild(li);
  }
}

window.addEventListener('click', function (e) {
  if (searchResults.style.display === "block" && !searchResults.contains(e.target)) {
    searchResults.style.display = "none";
  }
});

