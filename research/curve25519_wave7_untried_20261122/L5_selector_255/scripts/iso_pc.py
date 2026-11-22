#!/usr/bin/env python3
"""Arm I instrument identity (PROTOCOL.md P0-iso): every row this run's iso-census wrote for (k, t) cells that the
isogeny-seed study also measured (k = 20, 24; t = 4..7; seeds 0..1999) must equal that study's row exactly.
Reads research/curve25519_isogeny_seed_20261103/data/k{20,24}.jsonl (read-only reference, not imported code).
Usage: iso_pc.py <iso_dir>"""
import glob, json, os, re, sys
ref_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', '..', 'curve25519_isogeny_seed_20261103', 'data')
out = dict(cells=[])
for k in (20, 24):
    ref = {}
    for l in open(os.path.join(ref_dir, f'k{k}.jsonl')):
        d = json.loads(l); ref[(d['t'], d['seed'])] = d
    for f in sorted(glob.glob(os.path.join(sys.argv[1], f'iso_k{k}_t*.jsonl'))):
        t = int(re.search(r'_t(\d+)\.', f).group(1))
        rows = [json.loads(l) for l in open(f)]
        cmp = [r for r in rows if (r['t'], r['seed']) in ref]
        if not cmp: continue
        out['cells'].append(dict(k=k, t=t, compared=len(cmp), equal=sum(ref[(r['t'], r['seed'])] == r for r in cmp)))
out['pass'] = bool(out['cells']) and all(c['compared'] == c['equal'] for c in out['cells'])
print(json.dumps(out, indent=1))
