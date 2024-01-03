import toml
import glob
import os
import typing


class Post:
    def __init__(self, path: str):
        with open(path, 'r') as f:
            data = f.read()
        _, hdr, post = data.split("+++", 2)
        self.header = toml.loads(hdr)
        self.post = post
        self.path = path

    def write(self):
        with open(self.path, 'w') as f:
            f.write("+++\n")
            toml.dump(self.header, f)
            f.write("+++")
            f.write(self.post)

    def name(self) -> str:
        return self.path.split("/")[-1][:-3]


def all_posts() -> list[Post]:
    path = os.path.join(os.path.dirname(__file__), "..", "content")
    ret = []
    for post in glob.glob(os.path.join(path, "**", "*.md*"), recursive=True):
        if post.endswith("/_index.md"):
            continue
        post_path = os.path.realpath(post)
        post = Post(post_path)
        ret.append(post)
    return ret


def mutate_all_posts(cb: typing.Callable[[Post], None]):
    for post in all_posts():
        cb(post)
        post.write()
