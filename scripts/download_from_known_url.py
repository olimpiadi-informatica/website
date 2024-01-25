#!/usr/bin/env python3
import lib


def download_from_known_url(post: lib.Post):
    if 'extra' in post.header and 'gallery' in post.header['extra']:
        for info in post.header['extra']['gallery']:
            info['url'] = "https://media.olimpiadi-informatica.it/" + \
                post.name() + "/" + info['filename']


lib.mutate_all_posts(download_from_known_url)
