#!/usr/bin/env python3
"""Lossless JSON mirrors of the git-ignored data files (data/**/*.npz, data/**/*.jsonl) into data_json/.

npz  -> {"source", "source_sha256", "kind": "npz", "arrays": {name: {"dtype", "shape", "data": flat list}}}
jsonl -> {"source", "source_sha256", "kind": "jsonl", "records": [one JSON object per line]}
A mirror larger than 900 KB is split into numbered parts <name>.partNN.json; npz arrays are split along the
flattened data, jsonl along records; each part carries "part", "parts" and its slice.
Run from the lead directory:  python3 -I export_json.py            (export + exact reload check)
                              python3 -I export_json.py --check    (reload check only)
The check rebuilds every array (dtype, shape, values) and every record list and compares them exactly with the
source files; it exits non-zero on any difference."""
import glob, hashlib, json, os, sys
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, 'data_json')
LIMIT = 900 * 1024

def sha(path): return hashlib.sha256(open(path, 'rb').read()).hexdigest()

def sources():
    fs = glob.glob(os.path.join(HERE, 'data', '**', '*.npz'), recursive=True) + \
         glob.glob(os.path.join(HERE, 'data', '**', '*.jsonl'), recursive=True)
    return sorted(os.path.relpath(f, HERE) for f in fs)

def mirror_base(rel):
    return os.path.join(OUT, rel[len('data/'):].replace('/', '__').rsplit('.', 1)[0] + '.' + rel.rsplit('.', 1)[1])

def dump(obj): return json.dumps(obj, separators=(',', ':'))

def export(rel):
    src = os.path.join(HERE, rel); base = mirror_base(rel); head = dict(source=rel, source_sha256=sha(src))
    if rel.endswith('.npz'):
        z = np.load(src)
        arrays = {k: dict(dtype=str(z[k].dtype), shape=list(z[k].shape), data=z[k].ravel().tolist()) for k in z.files}
        whole = dump(dict(head, kind='npz', arrays=arrays))
        if len(whole) <= LIMIT:
            open(base + '.json', 'w').write(whole); return [base + '.json']
        # split: each part holds slices [lo, hi) of the flattened arrays
        items = [(k, v) for k, v in arrays.items()]; parts = []; cur = {}; size = 0
        for k, v in items:
            data = v['data']; i = 0
            while i < len(data):
                room = max(1000, (LIMIT - 20000 - size) // 12)
                j = min(len(data), i + room)
                cur[k + '@' + str(i)] = dict(dtype=v['dtype'], shape=v['shape'], lo=i, data=data[i:j])
                size += len(dump(data[i:j])); i = j
                if size > LIMIT - 40000: parts.append(cur); cur = {}; size = 0
        if cur: parts.append(cur)
    else:
        recs = [json.loads(l) for l in open(src) if l.strip()]
        whole = dump(dict(head, kind='jsonl', records=recs))
        if len(whole) <= LIMIT:
            open(base + '.json', 'w').write(whole); return [base + '.json']
        parts = []; cur = []; size = 0
        for r in recs:
            s = len(dump(r)) + 1
            if size + s > LIMIT - 4000 and cur: parts.append(cur); cur = []; size = 0
            cur.append(r); size += s
        if cur: parts.append(cur)
    out = []
    for n, p in enumerate(parts):
        fn = f'{base}.part{n:02d}.json'
        body = dict(head, kind='npz_part' if rel.endswith('.npz') else 'jsonl_part', part=n, parts=len(parts))
        body['slices' if rel.endswith('.npz') else 'records'] = p
        s = dump(body); assert len(s) <= LIMIT, (fn, len(s))
        open(fn, 'w').write(s); out.append(fn)
    return out

def reload(rel):
    """Rebuild the source content from its mirror files."""
    base = mirror_base(rel)
    files = [base + '.json'] if os.path.exists(base + '.json') else sorted(glob.glob(base + '.part*.json'))
    docs = [json.load(open(f)) for f in files]
    assert all(d['source'] == rel for d in docs)
    if rel.endswith('.npz'):
        if docs[0]['kind'] == 'npz':
            return {k: np.array(v['data'], dtype=v['dtype']).reshape(v['shape']) for k, v in docs[0]['arrays'].items()}, docs[0]['source_sha256']
        acc = {}
        for d in docs:
            for key, v in d['slices'].items():
                k = key.rsplit('@', 1)[0]; acc.setdefault(k, (v['dtype'], v['shape'], []))[2].append((v['lo'], v['data']))
        out = {}
        for k, (dt, shp, chunks) in acc.items():
            flat = [x for _, c in sorted(chunks, key=lambda q: q[0]) for x in c]
            out[k] = np.array(flat, dtype=dt).reshape(shp)
        return out, docs[0]['source_sha256']
    recs = docs[0]['records'] if docs[0]['kind'] == 'jsonl' else [r for d in sorted(docs, key=lambda d: d['part']) for r in d['records']]
    return recs, docs[0]['source_sha256']

def check(rel):
    src = os.path.join(HERE, rel); got, h = reload(rel)
    if h != sha(src): return False
    if rel.endswith('.npz'):
        z = np.load(src)
        return set(z.files) == set(got) and all(z[k].dtype == got[k].dtype and z[k].shape == got[k].shape
                                                and np.array_equal(z[k], got[k]) for k in z.files)
    return [json.loads(l) for l in open(src) if l.strip()] == got

def main():
    os.makedirs(OUT, exist_ok=True)
    srcs = sources()
    if '--check' not in sys.argv:
        for f in glob.glob(os.path.join(OUT, '*.json')): os.remove(f)
        written = [w for rel in srcs for w in export(rel)]
        lines = [f'{sha(w)}  {os.path.relpath(w, HERE)}' for w in sorted(written)]
        open(os.path.join(OUT, 'SHA256SUMS'), 'w').write('\n'.join(lines) + '\n')
    bad = [rel for rel in srcs if not check(rel)]
    big = [f for f in glob.glob(os.path.join(OUT, '*.json')) if os.path.getsize(f) > LIMIT]
    print(json.dumps(dict(sources=len(srcs), mirrors=len(glob.glob(os.path.join(OUT, '*.json'))),
                          exact_reload_ok=len(srcs) - len(bad), failures=bad, over_900KB=big)))
    sys.exit(1 if bad or big else 0)

if __name__ == '__main__':
    main()
