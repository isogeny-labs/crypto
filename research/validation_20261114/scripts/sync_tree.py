#!/usr/bin/env python3
"""Make DEST byte-equal to SRC, touching only files whose bytes differ (Validator V).

Used to switch the scratch build tree between commits without changing the
mtimes of unchanged files, so cargo rebuilds only what the commit changed.
Usage: sync_tree.py SRC DEST
"""
import filecmp, os, shutil, sys

src, dst = sys.argv[1], sys.argv[2]
want = set()
changed = added = removed = 0
for root, dirs, files in os.walk(src):
    rel = os.path.relpath(root, src)
    for f in files:
        rp = os.path.normpath(os.path.join(rel, f))
        want.add(rp)
        s, d = os.path.join(src, rp), os.path.join(dst, rp)
        if os.path.exists(d):
            if not filecmp.cmp(s, d, shallow=False):
                shutil.copyfile(s, d); changed += 1
        else:
            os.makedirs(os.path.dirname(d), exist_ok=True)
            shutil.copyfile(s, d); added += 1
for root, dirs, files in os.walk(dst):
    rel = os.path.relpath(root, dst)
    for f in files:
        rp = os.path.normpath(os.path.join(rel, f))
        if rp not in want:
            os.remove(os.path.join(dst, rp)); removed += 1
print(f"changed {changed}, added {added}, removed {removed}")
