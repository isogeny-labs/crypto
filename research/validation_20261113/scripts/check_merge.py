#!/usr/bin/env python3
"""Merged remeasure.json rows vs the part files they came from (every field)."""
import json, sys
R = '/home/user/crypto/research/negation_remeasure_20261113'
M = json.load(open(f'{R}/remeasure.json'))['targets']
parts = {'v3': ['remeasure_v3.json'], 'cover_pushforward': ['remeasure_cover.json'],
         'tower_large': ['remeasure_tower.json', 'remeasure_tower_p32003.json', 'remeasure_tower_p64007.json']}
def flat(v, pre=''):
    if isinstance(v, dict):
        for k, x in v.items(): yield from flat(x, f'{pre}/{k}')
    elif isinstance(v, list):
        for i, x in enumerate(v): yield from flat(x, f'{pre}[{i}]')
    else:
        yield pre, v
out = {'rows': 0, 'fields': 0, 'diffs': [], 'only_in_merged': [], 'only_in_part': []}
for t, files in parts.items():
    prow = {}
    for f in files:
        d = json.load(open(f'{R}/{f}'))
        rows = d['rows'] if 'rows' in d else d[t]['rows'] if t in d else None
        if rows is None:
            rows = [r for k, v in d.items() if isinstance(v, dict) and 'rows' in v for r in v['rows']]
        for r in rows:
            prow[r['p']] = r
    for r in M[t]['rows']:
        out['rows'] += 1
        a = dict(flat(prow[r['p']])); b = dict(flat(r))
        for k in a:
            if k in b:
                out['fields'] += 1
                if a[k] != b[k]: out['diffs'].append([t, r['p'], k, a[k], b[k]])
            else: out['only_in_part'].append([t, r['p'], k])
        for k in b:
            if k not in a: out['only_in_merged'].append([t, r['p'], k])
json.dump(out, open(sys.argv[1], 'w'), indent=1)
print('rows', out['rows'], 'fields compared', out['fields'], 'diffs', len(out['diffs']))
import collections
print('only_in_merged keys', collections.Counter(k.split('[')[0] for _, _, k in out['only_in_merged']))
print('only_in_part keys', collections.Counter(k.split('[')[0] for _, _, k in out['only_in_part']))
for d in out['diffs'][:10]: print(d)
