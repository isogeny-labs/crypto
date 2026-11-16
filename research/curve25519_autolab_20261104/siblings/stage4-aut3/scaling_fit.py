#!/usr/bin/env python3
"""Stage 4 (aut=3) scaling fit (derived statistic; `ecbench trend` takes binary-field slugs only).

For each shape and arm pair (B / A) and each unit (GAE = cost.total_gae, bit-ops =
work.bitops_per_sqrt_r): per workload, the ratio of B's mean to A's mean over the paired
(workload, round) runs; OLS slope of ln(ratio) on ln r over the 15 workloads' curves...
precisely: one point per workload (5 curves x 6 targets x 3 sizes = 90). 95 percent interval
from 4000 cluster-bootstrap resamples of workloads within each size (seed 20261111).
Verified, non-warmup records only. Prints and writes scaling.json.
"""
import json, math, random, sys
D = 'research/curve25519_autolab_20261104/siblings/stage4-aut3'
PAIRS = [('tw2cj-lazy', 'tw2cj-lazy-aut3'), ('sib8-p512', 'sib8-p512-aut3'), ('rho-cubic', 'sib8-p512-aut3'),
         ('rho-cubic', 'sib8-p512'), ('rho-neg', 'rho-cubic')]
PJ = []
UNITS = {'gae': lambda r: r['cost']['total_gae'], 'bitops': lambda r: r['work']['bitops_per_sqrt_r'] * math.sqrt(int(r['workload']['curve']['r']))}

def load(shape):
    by = {}  # (size, workload) -> arm -> [values per unit]
    for b in (34, 37, 40):
        for l in open(f'{D}/sessions/{shape}-{b}/records.jsonl'):
            r = json.loads(l)
            if r['warmup'] or r['outcome']['status'] != 'verified':
                continue
            k = (b, r['workload']['workload_id'], int(r['workload']['curve']['r']))
            by.setdefault(k, {}).setdefault(r['arm'], []).append(r)
    return by

def slope(pts):
    n = len(pts); mx = sum(x for x, _ in pts) / n; my = sum(y for _, y in pts) / n
    sxx = sum((x - mx) ** 2 for x, _ in pts)
    return sum((x - mx) * (y - my) for x, y in pts) / sxx, my

out = {}
rng = random.Random(20261111)
for shape in ('j0b7',):
    by = load(shape)
    for a, b in PAIRS + (PJ if shape == 'j0b7' else []):
        for u, f in UNITS.items():
            pts = {}
            for (size, wid, r), arms in by.items():
                if a in arms and b in arms:
                    ma = sum(map(f, arms[a])) / len(arms[a]); mb = sum(map(f, arms[b])) / len(arms[b])
                    pts.setdefault(size, []).append((math.log(r), math.log(mb / ma)))
            allp = [p for v in pts.values() for p in v]
            s0, _ = slope(allp)
            bs = []
            for _ in range(4000):
                smp = [p for v in pts.values() for p in (rng.choice(v) for _ in v)]
                bs.append(slope(smp)[0])
            bs.sort()
            per_size = {s: math.exp(sum(y for _, y in v) / len(v)) for s, v in sorted(pts.items())}
            key = f'{shape}: {b} / {a} [{u}]'
            out[key] = {'slope_ln_ratio_per_ln_r': s0, 'ci95': [bs[100], bs[3899]], 'workloads': len(allp),
                        'geomean_ratio_by_p_bits': per_size}
            print(f"{key:44s} slope {s0:+.4f} [{bs[100]:+.4f}, {bs[3899]:+.4f}]  " +
                  "  ".join(f"p{s}:{v:.3f}" for s, v in per_size.items()))
json.dump(out, open(f'{D}/scaling.json', 'w'), indent=1)
