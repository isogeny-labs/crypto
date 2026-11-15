#!/usr/bin/env python3
"""Tabulate out/main.jsonl: one row per run (tag, NV, D, one_in, W_dims, sec)."""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(here, 'out', 'main.jsonl'))]
print('| tag | n | seed/rseed | sols | NV | D | 1 in W_D | W_dims (cum.) | ncols | sec | RSS MB |')
print('|---|---|---|---|---|---|---|---|---|---|---|')
for r in rows:
    h, s = r['hdr'], r['res']
    if s is None:
        print('| %s | %s | %s | %s | %s | - | rc=%s (no result) | | | | |' % (r['tag'], h['n'], h.get('seed') if 'rseed' not in h else 'r%s' % h['rseed'], h['sols'], h['NV'], r['rc']))
        continue
    sd = 'r%s' % h['rseed'] if 'rseed' in h else h['seed']
    print('| %s | %d | %s | %d | %d | %d | %s | %s | %d | %.1f | %d |' % (r['tag'], h['n'], sd, h['sols'], s['NV'], s['D'],
          'yes' if s['one_in'] else 'no', s['W_dims'], s['ncols'], s['sec'], s['maxrss_kb'] // 1024))
