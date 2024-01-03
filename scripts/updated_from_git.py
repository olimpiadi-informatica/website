#!/usr/bin/env python3
import lib
import subprocess
import datetime


def updated_from_git(post: lib.Post):
    if 'extra' in post.header and 'override_updated' in post.header['extra']:
        post.header['updated'] = post.header['extra']['override_updated']
    else:
        updated = int(subprocess.check_output(
            ["git", "log", "-1", "--format=%ct", post.path]))
        post.header['updated'] = datetime.datetime.fromtimestamp(updated)


lib.mutate_all_posts(updated_from_git)
