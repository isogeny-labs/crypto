#!/usr/bin/env python3
"""Compute the set of repository paths needed to build the crate at a commit:
Rust source trees, explicit Cargo target paths, path deps, and the closure of
include_bytes!/include_str!/include!/#[path] references."""
import os, re, subprocess, sys
REPO, REV = sys.argv[1], sys.argv[2]
def git(*a):
    return subprocess.run(["git", "-C", REPO, *a], check=True, capture_output=True).stdout
allfiles = set(git("ls-tree", "-r", "--name-only", REV).decode().splitlines())
roots = ["Cargo.toml", "Cargo.lock", "src", "crates", "isogeny_algos", "tools/factor-base",
         "tools/volcano-cli", "research/pencil_large_prime_20261029",
         "research/five_leads_e2e_20261019/native", "examples", "benches", "tests"]
toml = git("show", f"{REV}:Cargo.toml").decode()
for m in re.finditer(r'path\s*=\s*"([^"]+)"', toml):
    v = m.group(1)
    roots.append(os.path.dirname(v) if v in allfiles else v)
sel = set()
def add_root(r):
    r = r.rstrip("/")
    if r in allfiles:
        sel.add(r); return
    pre = r + "/"
    for f in allfiles:
        if f.startswith(pre): sel.add(f)
for r in roots: add_root(r)
# closure over include macros and #[path]
pat = re.compile(r'(?:include_bytes!|include_str!|include!)\s*\(\s*(?:concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*)?"([^"]+)"')
pathattr = re.compile(r'#\s*\[\s*path\s*=\s*"([^"]+)"')
done = set(); missing = set()
changed = True
while changed:
    changed = False
    for f in sorted(sel - done):
        done.add(f)
        if not f.endswith(".rs"): continue
        try: txt = git("show", f"{REV}:{f}").decode("utf-8", "replace")
        except subprocess.CalledProcessError: continue
        for m in list(pat.finditer(txt)) + list(pathattr.finditer(txt)):
            rel = m.group(1)
            cands = [os.path.normpath(os.path.join(os.path.dirname(f), rel))]
            # CARGO_MANIFEST_DIR-relative: try the root crate too
            cands.append(os.path.normpath(rel.lstrip("/")))
            hit = False
            for c in cands:
                if c in allfiles:
                    if c not in sel: sel.add(c); changed = True
                    hit = True; break
                if any(x.startswith(c + "/") for x in ()):
                    pass
            if not hit: missing.add((f, rel))
print("\n".join(sorted(sel)))
sys.stderr.write(f"{len(sel)} files; {len(missing)} unresolved refs\n")
for f, rel in sorted(missing)[:40]: sys.stderr.write(f"  unresolved {f}: {rel}\n")
