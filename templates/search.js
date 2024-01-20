async function loadLunr() {
  for (const url of ["elasticlunr.min.js", "lunr.stemmer.support.js", "lunr.it.js"]) {
    const script = document.createElement("script");
    script.type = "text/javascript";
    script.src = url;
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
const searchResultsList = document.getElementById("search-results-list");

const alertPlaceholder = document.getElementById('search-alert-placeholder')
const appendAlert = (message, type) => {
  const wrapper = document.createElement('div')
  wrapper.innerHTML = [
    `<div class="alert alert-${type} alert-dismissible" role="alert">`,
    `   <div>${message}</div>`,
    '   <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>',
    '</div>'
  ].join('')
  alertPlaceholder.append(wrapper)
}

async function search() {
  await lunrLoaded;
  if (index == null) {
    const indexData = await (await fetch("/search_index.it.json")).json();
    index = elasticlunr.Index.load(indexData);
    index.use(lunr.it);
  }
  const val = document.getElementById("searchbar").value;
  if (val === "") return;
  const results = index.search(val, {});
  if (results.length === 0) {
    appendAlert(`Impossibile trovare ${val}`, "warning");
    return;
  }
  searchResults.style.display = "block";
  searchResultsList.innerHTML = "";
  for (let i = 0; i < Math.min(results.length, maxResults); i++) {
    const li = document.createElement("li");
    // TODO: consider adding a summary of the article.
    li.innerHTML = [
      `<div class="search-result-item">`,
      `<a class="link-secondary link-underline-opacity-0" href=${results[i].ref}><h3>${results[i].doc.title}</h3></a>`,
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

