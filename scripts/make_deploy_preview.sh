#!/usr/bin/env bash

set -xe

SHA="$1"
BASE_DESTINATION=$(realpath "$2")
STATUS_TOKEN="$3"

cd "$(realpath "$(dirname "$0")/..")"

[ -z "$STATUS_TOKEN" ] && echo "Usage: $0 SHA DESTINATION STATUS_TOKEN"
[ -z "$STATUS_TOKEN" ] && exit 1

exec 100>/tmp/website_preview.lock
flock 100

git checkout .
git fetch origin "$SHA"
git checkout "$SHA"
git submodule init
git submodule update --recursive

DESTINATION="$BASE_DESTINATION/$SHA"

./scripts/updated_from_git.py
./scripts/download_gallery_images.py

BASE_URL="https://$SHA.nightly.olimpiadi-informatica.it/"

zola build -u $BASE_URL

rm -rf "$DESTINATION"
cp -rl public/ "$DESTINATION"

curl -L \
  -X POST \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer $STATUS_TOKEN" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/olimpiadi-informatica/website/statuses/$SHA \
  -d "{\"state\":\"success\",\"target_url\":\"$BASE_URL\",\"description\":\"The build succeeded!\",\"context\":\"deploy\"}"


echo "All done!"
