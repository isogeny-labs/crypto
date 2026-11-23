#!/usr/bin/env python3
"""Compare a replayed part file with the committed one: every field except timing
(and fields derived from timing). usage: compare_replay.py COMMITTED REPLAY OUT.json [PRIMES]"""
import json, sys, re
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
primes = set(int(x) for x in sys.argv[4].split(',')) if len(sys.argv) > 4 else None
TIMING = re.compile(r'(class_times_ns|t_add_ns|wall_s|total_cal|ratio_neg_cal|class_neg_cal|class_best_cal|'
                    r'ranked_at_calibrated_price|transfer_wall_s|^/args|^/wall_s|/calibration/host)')
def flat(v, pre=''):
    if isinstance(v, dict):
        for k, x in v.items(): yield from flat(x, f'{pre}/{k}')
    elif isinstance(v, list):
        for i, x in enumerate(v): yield from flat(x, f'{pre}[{i}]')
    else:
        yield pre, v
def rows(d):
    return {r['p']: r for r in d['rows']}
ra, rb = rows(a), rows(b)
keys = sorted(set(rb) if primes is None else primes)
out = {'committed': sys.argv[1], 'replay': sys.argv[2], 'primes': keys, 'compared': 0, 'timing_skipped': 0,
       'diffs': [], 'missing': []}
for p in keys:
    fa, fb = dict(flat(ra[p])), dict(flat(rb[p]))
    for k, v in fa.items():
        if TIMING.search(k):
            out['timing_skipped'] += 1; continue
        if k not in fb:
            out['missing'].append([p, k]); continue
        out['compared'] += 1
        if v != fb[k]:
            out['diffs'].append([p, k, v, fb[k]])
    for k in fb:
        if k not in fa and not TIMING.search(k): out['missing'].append([p, 'only in replay: ' + k])
out['top_level_ok'] = [a.get('ok'), b.get('ok')]
json.dump(out, open(sys.argv[3], 'w'), indent=1)
print(f"primes {keys}: compared {out['compared']} non-timing values, skipped {out['timing_skipped']} timing values; "
      f"diffs {len(out['diffs'])}; missing {len(out['missing'])}; ok committed/replay {out['top_level_ok']}")
for d in out['diffs'][:15]: print('  ', d)
for d in out['missing'][:10]: print('  ', d)
