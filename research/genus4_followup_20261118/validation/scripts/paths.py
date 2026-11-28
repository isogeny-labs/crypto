#!/usr/bin/env python3
"""Validator: tracked paths needed to build crate `crypto` at COMMIT.
Manifest + lock, every target path the manifest names, src/ build.rs benches tests examples of every
manifest reached, path dependencies (whole dirs, recursively), and include_str!/include_bytes!/#[path]
targets reached from any listed .rs file, to a fixed point.  Usage: paths.py REPO COMMIT"""
import os, re, subprocess, sys
repo, commit = sys.argv[1], sys.argv[2]
def git(*a): return subprocess.run(["git", "-C", repo, *a], check=True, capture_output=True).stdout
tracked = set(git("ls-tree", "-r", "--name-only", commit).decode().splitlines())
def show(p): return git("show", f"{commit}:{p}").decode(errors="replace")
want, manifests = set(), set()
def prefix(d):
    d = d.rstrip("/") + "/"
    want.update(t for t in tracked if t.startswith(d))
def manifest(d):
    m = f"{d}/Cargo.toml" if d else "Cargo.toml"
    if m in manifests or m not in tracked: return
    manifests.add(m); want.add(m)
    for extra in ("Cargo.lock", "build.rs", "README.md"):
        q = f"{d}/{extra}" if d else extra
        if q in tracked: want.add(q)
    for sub in ("src", "benches", "tests", "examples"):
        prefix(f"{d}/{sub}" if d else sub)
    for mm in re.finditer(r'path\s*=\s*"([^"]+)"', show(m)):
        p = os.path.normpath(os.path.join(d, mm.group(1)))
        if p in tracked: want.add(p)
        elif f"{p}/Cargo.toml" in tracked: prefix(p); manifest(p)
        else: prefix(p)
manifest("")
pat = re.compile(r'(?:include_str!|include_bytes!)\s*\(\s*"([^"]+)"|#\[path\s*=\s*"([^"]+)"\]')
seen = set(); changed = True
while changed:
    changed = False
    for f in sorted(want - seen):
        seen.add(f)
        if not f.endswith(".rs"): continue
        try: txt = show(f)
        except subprocess.CalledProcessError: continue
        for m in pat.finditer(txt):
            rel = m.group(1) or m.group(2); base = os.path.dirname(f)
            cands = [os.path.normpath(os.path.join(base, rel))]
            if m.group(2): cands.append(os.path.normpath(os.path.join(os.path.splitext(f)[0], rel)))
            for c in cands:
                if c in tracked and c not in want: want.add(c); changed = True
print("\n".join(sorted(want)))
