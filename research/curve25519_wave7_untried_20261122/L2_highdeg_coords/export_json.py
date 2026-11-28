#!/usr/bin/env python3
"""Lossless JSON mirrors of the git-ignored binary outputs (data/*.npz, smoke/*.npz).

Each NPZ becomes data_json/<dir>/<name>.json: {"source", "source_sha256", "arrays":
{key: {"dtype", "shape", "data"}}}. A mirror above 900 KB is split into numbered
parts <name>.partNN.json, each holding a slice of the rows. `--check` reloads every
mirror, rebuilds each array with its dtype and shape, and compares it exactly
(np.array_equal plus dtype and shape) with the NPZ; it also checks the recorded
NPZ sha256. Writes data_json/SHA256SUMS."""
import argparse, glob, hashlib, json, os, sys
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = ['data', 'smoke']
LIMIT = 900 * 1024


def sha(fn):
    return hashlib.sha256(open(fn, 'rb').read()).hexdigest()


def enc(a):
    return dict(dtype=str(a.dtype), shape=list(a.shape), data=a.tolist())


def export():
    out = []
    for d in SRC:
        os.makedirs(f'{HERE}/data_json/{d}', exist_ok=True)
        for fn in sorted(glob.glob(f'{HERE}/{d}/*.npz')):
            z = np.load(fn)
            base = f'{HERE}/data_json/{d}/' + os.path.basename(fn)[:-4]
            doc = dict(source=f'{d}/{os.path.basename(fn)}', source_sha256=sha(fn),
                       arrays={k: enc(z[k]) for k in z.files})
            s = json.dumps(doc, separators=(',', ':'))
            if len(s) <= LIMIT:
                open(base + '.json', 'w').write(s); out.append(base + '.json')
                continue
            # split the largest 2-D array by rows; scalars go in part 00
            rows = z['rows']; nparts = len(s) // LIMIT + 1
            for i, chunk in enumerate(np.array_split(np.arange(rows.shape[0]), nparts)):
                part = dict(doc, part=i, parts=nparts, row_start=int(chunk[0]),
                            arrays={k: (enc(rows[chunk]) if k == 'rows' else enc(z[k])) for k in z.files})
                pfn = f'{base}.part{i:02d}.json'
                open(pfn, 'w').write(json.dumps(part, separators=(',', ':'))); out.append(pfn)
    with open(f'{HERE}/data_json/SHA256SUMS', 'w') as fh:
        for fn in sorted(out):
            fh.write(f'{sha(fn)}  {os.path.relpath(fn, f"{HERE}/data_json")}\n')
    print(f'wrote {len(out)} mirror files')


def check():
    bad = 0; n = 0
    for d in SRC:
        for fn in sorted(glob.glob(f'{HERE}/{d}/*.npz')):
            z = np.load(fn)
            base = f'{HERE}/data_json/{d}/' + os.path.basename(fn)[:-4]
            parts = sorted(glob.glob(base + '.part*.json')) or [base + '.json']
            docs = [json.load(open(p)) for p in parts]
            ok = all(doc['source_sha256'] == sha(fn) for doc in docs)
            for k in z.files:
                segs = [np.array(doc['arrays'][k]['data'], dtype=doc['arrays'][k]['dtype']).reshape(doc['arrays'][k]['shape']) for doc in docs]
                a = np.concatenate(segs) if (len(segs) > 1 and k == 'rows') else segs[0]
                ok &= a.dtype == z[k].dtype and a.shape == z[k].shape and np.array_equal(a, z[k])
            n += 1; bad += not ok
            if not ok:
                print('MISMATCH', fn)
    for line in open(f'{HERE}/data_json/SHA256SUMS'):
        h, p = line.split()
        if sha(f'{HERE}/data_json/{p}') != h:
            print('SHA MISMATCH', p); bad += 1
    print(f'reload check: {n} NPZ files, {bad} mismatches')
    return bad == 0


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('--check', action='store_true')
    a = ap.parse_args()
    if not a.check:
        export()
    sys.exit(0 if check() else 1)
