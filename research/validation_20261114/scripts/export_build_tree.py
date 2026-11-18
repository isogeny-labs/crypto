#!/usr/bin/env python3
"""Export the files cargo needs to build the `crypto` package at COMMIT into DEST.

Validator V, 2026-11-14. Reads blobs with `git show`/`git archive` only (never
touches the shared working tree). Base set: every top-level path except the big
data trees; then every Cargo.toml `path =`, every include_bytes!/include_str!
and every #[path = ...] reachable from the exported Rust files, resolved
relative to the including file, added from the same commit.
Usage: export_build_tree.py REPO COMMIT DEST
"""
import os, re, subprocess, sys, io, tarfile

repo, commit, dest = sys.argv[1], sys.argv[2], sys.argv[3]
BIG = {"research", "docs", "benchmarks", "experiments", "ecc2k130", "papers", "website",
       "index.html", "taskq", "gpu", "pcc", "journal", "knowledge", "reports", "output",
       "figures", "paper", "lab", "hdl", "sage", "coordination", "integrations", "infra",
       "problems", "backlog", "ecdb", "reproduction.lock.json", "manifest.json"}

def git(*a):
    return subprocess.run(["git", "-C", repo, *a], check=True, capture_output=True).stdout

alltree = git("ls-tree", "-r", "--name-only", commit).decode().splitlines()
allset = set(alltree)
dirs = set()
for p in alltree:
    parts = p.split("/")
    for i in range(1, len(parts)):
        dirs.add("/".join(parts[:i]))

want = set(p for p in alltree if p.split("/")[0] not in BIG)

def add_path(p):
    p = os.path.normpath(p)
    if p in allset:
        want.add(p); return True
    if p in dirs:
        pre = p + "/"
        for q in alltree:
            if q.startswith(pre):
                want.add(q)
        return True
    return False

missing = []
# Cargo.toml path entries (package and nested manifests)
pat_path = re.compile(r'path\s*=\s*"([^"]+)"')
pat_inc = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)')
pat_modpath = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
pat_concat = re.compile(r'include_(?:str|bytes)!\(\s*concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"([^"]+)"')
seen = set()
changed = True
while changed:
    changed = False
    for p in sorted(want):
        if p in seen: continue
        seen.add(p)
        if p.endswith("Cargo.toml"):
            base = os.path.dirname(p)
            txt = git("show", f"{commit}:{p}").decode(errors="replace")
            for m in pat_path.finditer(txt):
                q = os.path.join(base, m.group(1))
                if not add_path(q): missing.append((p, q))
                changed = True
        elif p.endswith(".rs"):
            base = os.path.dirname(p)
            try:
                txt = git("show", f"{commit}:{p}").decode(errors="replace")
            except subprocess.CalledProcessError:
                continue
            for m in pat_inc.finditer(txt):
                q = os.path.join(base, m.group(1))
                if not add_path(q): missing.append((p, q))
                changed = True
            for m in pat_modpath.finditer(txt):
                q = os.path.join(base, m.group(1))
                if not add_path(q): missing.append((p, q))
                changed = True
            for m in pat_concat.finditer(txt):
                # relative to the crate manifest dir: find nearest Cargo.toml upward
                d = base
                while d and (d + "/Cargo.toml") not in allset:
                    d = os.path.dirname(d)
                q = os.path.join(d, m.group(1).lstrip("/"))
                if not add_path(q): missing.append((p, q))
                changed = True

os.makedirs(dest, exist_ok=True)
lst = sorted(want)
with open(os.path.join(dest, "..", os.path.basename(dest) + ".paths.txt"), "w") as f:
    f.write("\n".join(lst) + "\n")
# git archive in chunks
for i in range(0, len(lst), 2000):
    chunk = lst[i:i+2000]
    data = git("archive", "--format=tar", commit, "--", *chunk)
    tarfile.open(fileobj=io.BytesIO(data)).extractall(dest, filter="tar")
print(f"exported {len(lst)} files from {commit} to {dest}")
for a, b in missing:
    print("unresolved:", a, "->", b)
