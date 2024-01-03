#!/usr/bin/env bash

set -xe

[ -z "$2" ] && echo "Usage: $0 INPUT.json OUTPUT.md" 
[ -z "$2" ] && exit 1

jq '.feed.entry[]."media$group" | { filename: ."media$title"."$t", url: ."media$content"[].url }' < "$1" | jq -s | sed 's/\.CR2"/.JPG"/' > /tmp/imagelist

NAME=$(basename "$(echo $2)" | cut -f 1 -d .)
IMAGE1=$(jq ".[0].filename" /tmp/imagelist | xargs echo)

cat > /tmp/script.py << EOF
import toml
import json
import sys

data = {
  "draft": False,
  "title": "PLACEHOLDER",
  "date": "2014-02-28T00:00:00",
  "extra": {
    "thumbnail": "/images/gallery/${NAME}/${IMAGE1}",
    "override_updated": "2017-08-04T11:37:15",
    "gallery": json.load(sys.stdin),
  }
}
print("+++")
print(toml.dumps(data))
print("+++")
EOF

python3 /tmp/script.py < /tmp/imagelist > "$2"
