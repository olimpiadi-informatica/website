#!/usr/bin/env bash

set -xe

cd "$(realpath "$(dirname "$0")/..")"

exec 100>/tmp/website_deploy.lock
flock 100

git checkout .
git pull
git submodule init
git submodule update --recursive

./scripts/updated_from_git.py
./scripts/download_gallery_images.py

zola build

# Copy the output to public-prod. If we're here the build run successfully,
# so failed builds won't mess up the website
rsync -a public/ public-prod/

echo "All done!"
