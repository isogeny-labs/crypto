#!/usr/bin/env python3
"""Validator: JSON mirrors are lossless (data_json/* vs data/*; analysis/gains.json vs analysis/gains.csv)."""
import json, os, glob, hashlib
import numpy as np
BASE = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L5_selector_255'
out = dict(mirrors=0, ok=0, bad=[])
for f in sorted(glob.glob(os.path.join(BASE, 'data_json', '*.json'))):
    d = json.load(open(f)); src = os.path.join(BASE, d['source'])
    raw = open(src, 'rb').read(); out['mirrors'] += 1
    good = hashlib.sha256(raw).hexdigest() == d['source_sha256']
    if d['kind'] == 'npz':
        z = np.load(src)
        good &= set(z.files) == set(d['arrays'])
        for k, a in d['arrays'].items():
            arr = np.array(a['data'], dtype=a['dtype']).reshape(a['shape'])
            good &= str(z[k].dtype) == a['dtype'] and list(z[k].shape) == a['shape'] and np.array_equal(arr, z[k])
    else:
        recs = [json.loads(l) for l in raw.decode().splitlines() if l.strip()]
        good &= recs == d['records']
    if good: out['ok'] += 1
    else: out['bad'].append(os.path.basename(f))
g = json.load(open(os.path.join(BASE, 'analysis', 'gains.json')))
txt = ','.join(g['columns']) + '\n' + ''.join(','.join(r) + '\n' for r in g['rows'])
out['gains_json_rebuild_sha256'] = hashlib.sha256(txt.encode()).hexdigest()
out['gains_csv_sha256'] = hashlib.sha256(open(os.path.join(BASE, 'analysis', 'gains.csv'), 'rb').read()).hexdigest()
out['gains_csv_sha256_recorded'] = open(os.path.join(BASE, 'analysis', 'gains.csv.sha256')).read().split()[0]
out['gains_json_rows'] = len(g['rows'])
print(json.dumps(out, indent=1))
