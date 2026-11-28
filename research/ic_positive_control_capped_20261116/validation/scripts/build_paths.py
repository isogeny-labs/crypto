#!/usr/bin/env python3
"""List the tracked paths a scratch build of crate `crypto` at COMMIT needs:
manifest, lock, src, path dependencies (recursively), Cargo target paths,
and files that src includes via include_str!/include_bytes!/#[path]."""
import os, re, subprocess, sys
REPO, COMMIT = sys.argv[1], sys.argv[2]
def git(*a):
    return subprocess.run(["git", "-C", REPO, *a], check=True, capture_output=True).stdout
tracked = set(git("ls-tree", "-r", "--name-only", COMMIT).decode().splitlines())
def show(p):
    return git("show", f"{COMMIT}:{p}").decode(errors="replace")
want = set()
def add_dir(d):
    d = d.rstrip("/") + "/"
    for t in tracked:
        if t.startswith(d):
            want.add(t)
def manifest(dirp):
    m = f"{dirp}/Cargo.toml" if dirp else "Cargo.toml"
    if m not in tracked:
        return
    want.add(m)
    lock = f"{dirp}/Cargo.lock" if dirp else "Cargo.lock"
    if lock in tracked: want.add(lock)
    txt = show(m)
    for mm in re.finditer(r'path\s*=\s*"([^"]+)"', txt):
        p = os.path.normpath(os.path.join(dirp, mm.group(1))) if dirp else os.path.normpath(mm.group(1))
        if p in tracked:
            want.add(p)
            # a target file; also scan its directory? keep file only
        elif f"{p}/Cargo.toml" in tracked and f"{p}/Cargo.toml" not in want:
            add_dir(p)
            manifest(p)
        else:
            add_dir(p)
    for d in ["src", "build.rs", "benches", "tests", "examples"]:
        q = f"{dirp}/{d}" if dirp else d
        if q in tracked: want.add(q)
        else: add_dir(q)
manifest("")
# includes, iterate to a fixed point
pat = re.compile(r'(?:include_str!|include_bytes!)\s*\(\s*"([^"]+)"|#\[path\s*=\s*"([^"]+)"\]')
seen = set()
changed = True
while changed:
    changed = False
    for f in sorted(want - seen):
        seen.add(f)
        if not f.endswith(".rs"):
            continue
        try:
            txt = show(f)
        except subprocess.CalledProcessError:
            continue
        for m in pat.finditer(txt):
            rel = m.group(1) or m.group(2)
            base = os.path.dirname(f)
            cand = [os.path.normpath(os.path.join(base, rel))]
            if m.group(2):  # #[path] inside a non-mod.rs file resolves relative to a dir named after the file
                stem = os.path.splitext(f)[0]
                cand.append(os.path.normpath(os.path.join(stem, rel)))
            for c in cand:
                if c in tracked and c not in want:
                    want.add(c); changed = True
print("\n".join(sorted(want)))
