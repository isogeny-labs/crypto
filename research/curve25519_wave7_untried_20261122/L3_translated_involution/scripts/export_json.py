#!/usr/bin/env python3
"""Lossless JSON mirrors of data/**/*.npz (the npz files are git-ignored by the artifact policy).

  python3 export_json.py           # write ../data_json/<k>/<arm>[.partN].json and ../data_json/SHA256SUMS
  python3 export_json.py --check   # reload every mirror and compare every array with the npz exactly

Each JSON file: {"source": "data/k16/E.x.npz", "source_sha256": ..., "part": i, "parts": n,
"arrays": {name: {"dtype": ..., "shape": [...], "data": [...]}}}. Files are split by whole arrays so
that none exceeds 900 KB.
"""
import argparse, glob, hashlib, json, os, sys
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.normpath(os.path.join(HERE, '..'))
LIMIT = 900 * 1024


def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()


def npz_files():
    return sorted(glob.glob(os.path.join(ROOT, 'data', '**', '*.npz'), recursive=True))


def out_base(f):
    rel = os.path.relpath(f, os.path.join(ROOT, 'data'))
    return os.path.join(ROOT, 'data_json', rel[:-4])


def export():
    for f in npz_files():
        d = np.load(f)
        enc = {n: dict(dtype=str(d[n].dtype), shape=list(d[n].shape), data=d[n].ravel().tolist()) for n in d.files}
        sizes = {n: len(json.dumps(v, separators=(',', ':'))) for n, v in enc.items()}
        parts, cur, size = [], [], 0
        for n in d.files:
            if cur and size + sizes[n] > LIMIT - 4096:
                parts.append(cur); cur, size = [], 0
            cur.append(n); size += sizes[n]
        parts.append(cur)
        base = out_base(f)
        os.makedirs(os.path.dirname(base), exist_ok=True)
        for old in glob.glob(base + '.*json'):
            os.remove(old)
        for i, names in enumerate(parts):
            path = base + ('.json' if len(parts) == 1 else f'.part{i + 1}.json')
            doc = dict(source=os.path.relpath(f, ROOT), source_sha256=sha(f), part=i + 1, parts=len(parts),
                       arrays={n: enc[n] for n in names})
            open(path, 'w').write(json.dumps(doc, separators=(',', ':')))
            assert os.path.getsize(path) <= LIMIT, path
    files = sorted(glob.glob(os.path.join(ROOT, 'data_json', '**', '*.json'), recursive=True))
    with open(os.path.join(ROOT, 'data_json', 'SHA256SUMS'), 'w') as fh:
        for p in files:
            fh.write(f'{sha(p)}  {os.path.relpath(p, os.path.join(ROOT, "data_json"))}\n')


def check():
    bad, n_arr = 0, 0
    for f in npz_files():
        d = np.load(f)
        got = {}
        for p in sorted(glob.glob(out_base(f) + '.json') + glob.glob(out_base(f) + '.part*.json')):
            doc = json.load(open(p))
            assert doc['source_sha256'] == sha(f), p
            for n, v in doc['arrays'].items():
                got[n] = np.array(v['data'], dtype=v['dtype']).reshape(v['shape'])
        if set(got) != set(d.files):
            print('array set mismatch', f); bad += 1
            continue
        for n in d.files:
            n_arr += 1
            if got[n].dtype != d[n].dtype or got[n].shape != d[n].shape or not np.array_equal(got[n], d[n]):
                print('mismatch', f, n); bad += 1
    print(f'{len(npz_files())} npz files, {n_arr} arrays, {bad} mismatches')
    return bad


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('--check', action='store_true')
    a = ap.parse_args()
    if not a.check:
        export()
    sys.exit(1 if check() else 0)
