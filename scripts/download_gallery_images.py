#!/usr/bin/env python3
import lib
import urllib3
import os
import sys
from concurrent.futures import ThreadPoolExecutor

NUM_THREADS = 32

http = urllib3.PoolManager(num_pools=NUM_THREADS)


def fetch_and_save(url_and_dest) -> None:
    url, dest = url_and_dest
    print(f"Downloading {url} to {dest}", file=sys.stderr)
    response = http.request("GET", url)
    with open(dest, 'wb') as f:
        f.write(response.data)


urls_and_dests = []

for post in lib.all_posts():
    if 'extra' in post.header and 'gallery' in post.header['extra']:
        d: str = os.path.join("static", "images", "gallery", post.name())
        os.makedirs(d, exist_ok=True)
        for info in post.header['extra']['gallery']:
            url = urllib3.util.parse_url(info['url'])
            assert url.netloc == "media.olimpiadi-informatica.it"
            assert url.scheme == "https"
            fname = info['filename'].replace("/", "")
            dest: str = os.path.join(d, fname)
            if not os.path.exists(dest):
                urls_and_dests.append((info['url'], dest))

ThreadPoolExecutor(max_workers=32).map(fetch_and_save, urls_and_dests)
