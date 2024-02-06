# website
Official website for the Italian Olympiads in Informatics, available at [olimpiadi-informatica.it](https://olimpiadi-informatica.it)
.
## Contributing
You can either fork this repository and open a pull request with your changes, or use the graphical editor available at [edit.olimpiadi-informatica.it](https://edit.olimpiadi-informatica.it) (which will fork and open PRs for you as needed)

## Testing locally

You'll need to install `zola`. On arch, `extra/zola` provides it. On Ubuntu there is no package, but you can download the zola binary directly from [github.com/getzola/zola/releases](https://github.com/getzola/zola/releases)

First, prepare the gallery images
```
./scripts/download_gallery_images.py
```

Then, run `zola serve` to build the website.
