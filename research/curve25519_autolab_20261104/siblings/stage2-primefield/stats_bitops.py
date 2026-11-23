#!/usr/bin/env python3
"""Stage 1 bit-op ratios (E1, E2): stats_bitops.py <session dir> <out json>.

Ratio of means of each record's own `work.bitops_per_sqrt_r` (bitop/v1, as
recorded by ecbench), candidate over `tw2cj-lazy`, paired by
(workload_id, round), with a paired bootstrap 95 percent interval
(4000 resamples, seed 20261110). Verified, non-warmup records only.
Computes no price of its own.
"""
import json, random, sys

sess, outf = sys.argv[1:3]
recs = [json.loads(l) for l in open(sess + '/records.jsonl')]
recs = [r for r in recs if not r['warmup'] and r['outcome']['status'] == 'verified']
key = lambda r: (r['workload']['workload_id'], r['round'])
by = {}
for r in recs:
    by.setdefault(r['arm'], {})[key(r)] = r['work']['bitops_per_sqrt_r']
base = 'tw2cj-lazy'
out = {'session': sess, 'statistic': 'ratio of means of bitops_per_sqrt_r, paired by (workload, round)',
       'means': {a: sum(v.values()) / len(v) for a, v in by.items()},
       'n': {a: len(v) for a, v in by.items()}, 'ratios': {}}
rng = random.Random(20261110)
for a in sorted(by):
    for den in (base, 'rho-neg'):
        if a == den or den not in by:
            continue
        ks = sorted(set(by[a]) & set(by[den]))
        num = [by[a][k] for k in ks]; dd = [by[den][k] for k in ks]
        pt = sum(num) / sum(dd)
        bs = []
        for _ in range(4000):
            idx = [rng.randrange(len(ks)) for _ in ks]
            bs.append(sum(num[i] for i in idx) / sum(dd[i] for i in idx))
        bs.sort()
        out['ratios'][f'{a} / {den}'] = {'pairs': len(ks), 'ratio': pt,
                                         'ci95': [bs[int(0.025 * 4000)], bs[int(0.975 * 4000) - 1]]}
json.dump(out, open(outf, 'w'), indent=1)
for k, v in out['ratios'].items():
    print(f"{k:45s} {v['ratio']:.3f} [{v['ci95'][0]:.3f}, {v['ci95'][1]:.3f}] n={v['pairs']}")
