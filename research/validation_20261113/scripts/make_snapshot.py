#!/usr/bin/env python3
"""Copy the build inputs of the working tree into a frozen snapshot dir and hash them.

Usage: make_snapshot.py REPO DEST MANIFEST_OUT
Copies Cargo.toml, Cargo.lock, src/, path dependencies, every manifest target path,
and every include_str!/include_bytes! target reachable from copied .rs files.
"""
import os, re, shutil, sys, hashlib

repo, dest, man_out = sys.argv[1:4]
repo = os.path.abspath(repo)
dest = os.path.abspath(dest)
copied = set()

def cp(rel):
    rel = os.path.normpath(rel)
    src = os.path.join(repo, rel)
    if rel in copied or not os.path.exists(src):
        return
    d = os.path.join(dest, rel)
    if os.path.isdir(src):
        shutil.copytree(src, d, dirs_exist_ok=True, symlinks=True,
                        ignore=shutil.ignore_patterns('target'))
    else:
        os.makedirs(os.path.dirname(d), exist_ok=True)
        shutil.copy2(src, d)
    copied.add(rel)

for f in ['Cargo.toml', 'Cargo.lock']:
    cp(f)
cp('src')
toml = open(os.path.join(repo, 'Cargo.toml')).read()
for m in re.finditer(r'path\s*=\s*"([^"]+)"', toml):
    cp(m.group(1))

inc = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)')
pathattr = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
changed = True
seen_rs = set()
while changed:
    changed = False
    for root, _, files in os.walk(dest):
        for fn in files:
            if not fn.endswith('.rs'):
                continue
            p = os.path.join(root, fn)
            if p in seen_rs:
                continue
            seen_rs.add(p)
            changed = True
            txt = open(p, errors='replace').read()
            for m in list(inc.finditer(txt)) + list(pathattr.finditer(txt)):
                target = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
                rel = os.path.relpath(target, dest)
                if rel.startswith('..'):
                    continue
                if not os.path.exists(target):
                    cp(rel)

# manifest of every file in the snapshot
rows = []
for root, _, files in os.walk(dest):
    for fn in files:
        p = os.path.join(root, fn)
        rel = os.path.relpath(p, dest)
        h = hashlib.sha256(open(p, 'rb').read()).hexdigest()
        rows.append(f"{h}  {rel}")
rows.sort(key=lambda r: r[66:])
open(man_out, 'w').write('\n'.join(rows) + '\n')
print(len(rows), 'files')
