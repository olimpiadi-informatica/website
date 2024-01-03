#!/usr/bin/env bash

set -xe
cd "$(dirname "$0")/.."

./scripts/updated_from_git.py
./scripts/download_gallery_images.py

# There is no Zola packages for Ubuntu, build using the docker image
docker run -u "$(id -u):$(id -g)" -v $PWD:/app --workdir /app ghcr.io/getzola/zola:v0.18.0 build

# Copy the output to public-prod. If we're here the build run successfully,
# so failed builds won't mess up the website
rsync -a public/ public-prod/

echo "All done!"
