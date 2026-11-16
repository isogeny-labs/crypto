#!/usr/bin/env python3
"""Overlay the shared checkout's research/ and docs/ into the scratch build tree
by symlinks, without replacing any file the export put there (Validator V).

The unit tests of band/compare/challenge/bounds read committed fixtures at
runtime through CARGO_MANIFEST_DIR (and one walks every research/**/records.jsonl),
so the scratch tree needs them; copying 786 MB of records does not fit the disk.
Exported files (HEAD blobs) win; everything else resolves into the checkout,
read-only in practice (the tests write only to the system temp dir). The crate
directory research/pencil_large_prime_20261029 (a path dependency) is left alone.
Writes the list of links it made to LINKS so they can be removed.
Usage: overlay_links.py CHECKOUT TREE LINKS [remove]
"""
import os, sys

src, dst, listing = sys.argv[1], sys.argv[2], sys.argv[3]
if len(sys.argv) > 4 and sys.argv[4] == "remove":
    n = 0
    for l in open(listing).read().split("\n"):
        if l and os.path.islink(l):
            os.unlink(l); n += 1
    print("removed", n); sys.exit()
SKIP = {os.path.join("research", "pencil_large_prime_20261029")}
made = []


def overlay(rel):
    s, d = os.path.join(src, rel), os.path.join(dst, rel)
    for name in sorted(os.listdir(s)):
        r = os.path.join(rel, name)
        if r in SKIP:
            continue
        sp, dp = os.path.join(src, r), os.path.join(dst, r)
        if not os.path.lexists(dp):
            os.symlink(sp, dp); made.append(dp)
        elif os.path.isdir(dp) and not os.path.islink(dp) and os.path.isdir(sp):
            overlay(r)


for top in ("research", "docs"):
    os.makedirs(os.path.join(dst, top), exist_ok=True)
    overlay(top)
open(listing, "w").write("\n".join(made) + "\n")
print("links", len(made))
