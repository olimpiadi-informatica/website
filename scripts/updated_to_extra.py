#!/usr/bin/env python3
import lib


def updated_to_extra(post: lib.Post):
    if 'updated' in post.header:
        if 'extra' not in post.header:
            post.header['extra'] = dict()
        post.header['extra']['override_updated'] = post.header['updated']
        del post.header['updated']


lib.mutate_all_posts(updated_to_extra)
