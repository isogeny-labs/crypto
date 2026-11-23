#!/usr/bin/env python3
"""Minimal source bundle for building `la_engine_bench` from a git commit.

The full tree is ~5 GB; Cargo needs only the manifest, every declared target
path, path dependencies, and the files pulled in by include!/include_str!/
include_bytes!/#[path]. Files are taken from `git ls-files` at HEAD (tracked
content only), so the bundle is a pure function of the commit.
Usage: make_bundle.py OUT.tar.gz   (prints the file count and sha256)
"""
import hashlib, os, re, subprocess, sys, tarfile, tomllib

root = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()
os.chdir(root)
tracked = set(subprocess.check_output(["git", "ls-files", "-z"], text=True).split("\0")) - {""}
want = set()
dirs = ["src", "examples"]

def add_dir(d):
    d = d.rstrip("/") + "/"
    for f in tracked:
        if f.startswith(d) and not f.endswith((".jsonl", ".parquet")) or f == d.rstrip("/"):
            want.add(f)

def scan_manifest(path):
    base = os.path.dirname(path)
    m = tomllib.load(open(path, "rb"))
    want.add(path)
    lock = os.path.join(base, "Cargo.lock")
    if lock in tracked: want.add(lock)
    if os.path.join(base, "build.rs").lstrip("./") in tracked: want.add(os.path.join(base, "build.rs").lstrip("./"))
    for kind in ("bin", "example", "bench", "test"):
        for t in m.get(kind, []):
            if "path" in t: want.add(os.path.normpath(os.path.join(base, t["path"])))
    lib = m.get("lib", {})
    if "path" in lib: want.add(os.path.normpath(os.path.join(base, lib["path"])))
    for sect in ("dependencies", "dev-dependencies", "build-dependencies"):
        for name, spec in (m.get(sect) or {}).items():
            if isinstance(spec, dict) and "path" in spec:
                dep = os.path.normpath(os.path.join(base, spec["path"]))
                if os.path.join(dep, "Cargo.toml") not in want:
                    add_dir(dep); scan_manifest(os.path.join(dep, "Cargo.toml"))

scan_manifest("Cargo.toml")
for d in dirs: add_dir(d)
pat = re.compile(r'(?:include(?:_str|_bytes)?!\(\s*"([^"]+)"|#\[path\s*=\s*"([^"]+)"\])')
todo = [f for f in want if f.endswith(".rs")]
seen = set()
while todo:
    f = todo.pop()
    if f in seen or f not in tracked: continue
    seen.add(f)
    for a, b in pat.findall(open(f, encoding="utf-8", errors="replace").read()):
        rel = a or b
        # #[path] in a non-mod.rs file resolves relative to the module dir; try both.
        cands = [os.path.normpath(os.path.join(os.path.dirname(f), rel))]
        if b and not f.endswith(("mod.rs", "lib.rs", "main.rs")):
            cands.append(os.path.normpath(os.path.join(f[:-3], rel)))
        for c in cands:
            if c in tracked:
                want.add(c)
                if c.endswith(".rs"): todo.append(c)
missing = sorted(w for w in want if w not in tracked)
files = sorted(w for w in want if w in tracked)
out = sys.argv[1]
with tarfile.open(out, "w:gz") as tf:
    for f in files:
        ti = tf.gettarinfo(f, arcname=f); ti.mtime = 0; ti.uid = ti.gid = 0; ti.uname = ti.gname = ""
        with open(f, "rb") as fh: tf.addfile(ti, fh)
print(f"files={len(files)} missing_declared={len(missing)} sha256={hashlib.sha256(open(out,'rb').read()).hexdigest()} bytes={os.path.getsize(out)}")
for m in missing[:20]: print("  not tracked:", m)
