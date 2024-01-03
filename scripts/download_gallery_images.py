#!/usr/bin/env python3
import lib
import subprocess

import os


os.chdir(os.path.join(os.path.dirname(__file__), ".."))

children = []

for post in lib.all_posts():
    if 'extra' in post.header and 'gallery' in post.header['extra']:
        d = os.path.join("static", "images", "gallery", post.name())
        os.makedirs(d, exist_ok=True)
        for info in post.header['extra']['gallery']:
            fname = info['filename']
            dest = os.path.join(d, fname)
            if not os.path.exists(dest):
                children.append(subprocess.Popen(
                    ["wget", "-4", "-O", dest, "-o", "/dev/null", info['url']]
                ))

for child in children:
    child.wait()
