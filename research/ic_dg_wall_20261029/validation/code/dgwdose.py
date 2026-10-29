#!/usr/bin/env python3
"""Per-pair wall saving (srenew - dg) against removed guard probes: OLS slope and intercept. Usage: dgwdose.py <evidence_dir> <out.json>"""
import json, collections, sys
E = sys.argv[1]
out = {}
for b in (24, 28, 32):
    by = collections.defaultdict(dict)
    for l in open(f'{E}/P{b}/wall/eks-results/session/records.jsonl'):
        r = json.loads(l)
        if r['warmup'] or r['arm'] not in ('srenew', 'dg', 'srenew-aa'): continue
        by[(r['workload']['workload_id'], r['round'])][r['arm']] = r
    pr = lambda r: sum((p.get('native') or {}).get('target_guard_probes', 0) for p in r['phases'])
    xs = []; ys = []; sw = []
    for v in by.values():
        xs.append(pr(v['srenew']) - pr(v['dg'])); ys.append(v['srenew']['time']['solve_wall_ns'] - v['dg']['time']['solve_wall_ns']); sw.append(v['srenew']['time']['solve_wall_ns'])
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    sl = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs); ic = my - sl * mx
    out[b] = {'slope_ns_per_removed_probe': sl, 'intercept_ns': ic, 'mean_saving_ns': my, 'intercept_share_of_mean_saving': ic / my, 'mean_removed_probes': mx, 'min_removed': min(xs), 'max_removed': max(xs)}
json.dump(out, open(sys.argv[2], 'w'), indent=1)
