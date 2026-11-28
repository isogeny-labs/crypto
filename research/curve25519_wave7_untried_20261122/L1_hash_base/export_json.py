#!/usr/bin/env python3
"""Lossless JSON mirrors of out/*.npz (artifact policy: no binaries in Git).
usage: export_json.py [--check]   writes out_json/<name>.json (numbered parts if > 900 KB),
out/SHA256SUMS, out_json/SHA256SUMS; --check reloads the JSON and compares every array exactly."""
import glob, hashlib, json, os, sys
import numpy as np
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = f'{HERE}/out'; J = f'{HERE}/out_json'
LIMIT = 900_000

def sha(path): return hashlib.sha256(open(path, 'rb').read()).hexdigest()

def encode(f):
    d = np.load(f); arrs = {}
    for k in d.files:
        a = d[k]
        arrs[k] = dict(dtype=str(a.dtype), shape=list(a.shape), data=a.tolist() if a.dtype.kind != 'U' else str(a))
    return arrs

def export():
    os.makedirs(J, exist_ok=True)
    for f in sorted(glob.glob(f'{OUT}/*.npz')):
        name = os.path.basename(f)[:-4]; arrs = encode(f)
        txt = json.dumps(dict(source=os.path.basename(f), arrays=arrs), separators=(',', ':'))
        if len(txt) <= LIMIT:
            open(f'{J}/{name}.json', 'w').write(txt); continue
        parts = [{}]; size = 0                      # split per array, then per row block
        for k, v in arrs.items():
            s = len(json.dumps(v, separators=(',', ':')))
            if size + s > LIMIT and parts[-1]: parts.append({}); size = 0
            parts[-1][k] = v; size += s
        for i, p in enumerate(parts):
            open(f'{J}/{name}.part{i+1:02d}.json', 'w').write(json.dumps(dict(source=os.path.basename(f), part=i + 1, of=len(parts), arrays=p), separators=(',', ':')))
    open(f'{OUT}/SHA256SUMS', 'w').write(''.join(f'{sha(f)}  {os.path.basename(f)}\n' for f in sorted(glob.glob(f'{OUT}/*.npz'))))
    open(f'{J}/SHA256SUMS', 'w').write(''.join(f'{sha(f)}  {os.path.basename(f)}\n' for f in sorted(glob.glob(f'{J}/*.json'))))

def check():
    bad = 0; n = 0
    for f in sorted(glob.glob(f'{OUT}/*.npz')):
        name = os.path.basename(f)[:-4]; d = np.load(f); got = {}
        for p in sorted(glob.glob(f'{J}/{name}.json') + glob.glob(f'{J}/{name}.part*.json')):
            got.update(json.load(open(p))['arrays'])
        assert set(got) == set(d.files), name
        for k in d.files:
            a = d[k]; v = got[k]
            b = np.array(v['data'], dtype=v['dtype']).reshape(v['shape']) if a.dtype.kind != 'U' else np.array(v['data'])
            ok = a.dtype == b.dtype and a.shape == b.shape and np.array_equal(a, b); bad += not ok; n += 1
    print(f'checked {n} arrays, mismatches {bad}'); return bad

if __name__ == '__main__':
    export(); sys.exit(1 if check() else 0)
