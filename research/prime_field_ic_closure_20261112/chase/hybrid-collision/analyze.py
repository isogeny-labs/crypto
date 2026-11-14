#!/usr/bin/env python3
"""Summarise runs: mean cost per arm at w in {0,0.1,1}, ratio to matched rho
(paired by instance) with 95% percentile bootstrap. [pcc:unverified]"""
import json, sys, random, math
from collections import defaultdict
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from hyb import cost

rows = [json.loads(l) for f in sys.argv[1:] for l in open(f)]
W = (0, 0.1, 1)
# instance key -> rho costs
rho = {}
for r in rows:
    if r['arm'] == 'rho':
        rho[(r['bits'], r['curve'], r['inst'], r['neg'])] = r['events']['rho']

def ev(r, kind):
    e = r['events']
    if kind == 'HYB-U':
        c = [e[k] for k in ('dep', 'rho') if k in e]
        return min(c, key=lambda s: s['steps']) if c else None
    return e.get({'IC-G': 'gaudry', 'IC-1dep': 'dep', 'RHO': 'rho'}[kind])

groups = defaultdict(list)
for r in rows:
    kinds = ['RHO'] if r['arm'] == 'rho' else ['IC-G', 'IC-1dep', 'HYB-U']
    for k in kinds:
        groups[(r['bits'], r['arm'], r['cfg'], 0, r['neg'], k)].append(r)

def boot(pairs, B=2000, seed=1):
    rng = random.Random(seed); n = len(pairs); out = []
    for _ in range(B):
        s = [pairs[rng.randrange(n)] for _ in range(n)]
        out.append(sum(a for a, _ in s) / sum(b for _, b in s))
    out.sort(); return out[int(.025 * B)], out[int(.975 * B)]

print('| bits | arm | V | m | walk | stop | N | table | mean hits | hits/V | ' +
      ' | '.join(f'cost w={w}' for w in W) + ' | ' +
      ' | '.join(f'/rho-matched w={w} [95% CI]' for w in W) + ' | /rho-neg w=0.1 |')
print('|' + '---|' * (12 + 2 * len(W)))
summary = []
for key in sorted(groups, key=lambda k: (k[0], k[1], k[2], k[3], k[4], k[5])):
    bits, arm, cfg, _, neg, kind = key
    rs = groups[key]
    V = sum(r['V'] for r in rs) / len(rs); m = sum(r['m'] for r in rs) / len(rs)
    evs = [(r, ev(r, kind)) for r in rs]
    if any(e is None for _, e in evs):
        nmiss = sum(e is None for _, e in evs)
        print(f'| {bits} | {arm} | {V} | {m} | {"neg" if neg else "plain"} | {kind} | {len(rs)} | – | missing {nmiss} |')
        continue
    line = [bits, arm, f'{V:.0f}', f'{m:.0f}', 'neg' if neg else 'plain', kind, len(rs), round(sum(r['table_size'] for r in rs)/len(rs)) if arm != 'rho' else 0]
    mh = sum(e['hits'] for _, e in evs) / len(evs)
    line += [f'{mh:.1f}', f'{mh / V:.2f}' if V else '–']
    costs = {w: sum(cost(e, w) for _, e in evs) / len(evs) for w in W}
    line += [f'{costs[w]:.0f}' for w in W]
    rec = dict(bits=bits, arm=arm, V=V, m=m, neg=neg, stop=kind, N=len(rs), mean_hits=mh,
               cost={str(w): costs[w] for w in W}, ratio={}, ci={})
    for w in W:
        pairs = [(cost(e, w), cost(rho[(r['bits'], r['curve'], r['inst'], neg)], w)) for r, e in evs]
        ratio = sum(a for a, _ in pairs) / sum(b for _, b in pairs)
        lo, hi = boot(pairs)
        line.append(f'{ratio:.2f} [{lo:.2f},{hi:.2f}]')
        rec['ratio'][str(w)] = ratio; rec['ci'][str(w)] = [lo, hi]
    pn = [(cost(e, 0.1), cost(rho[(r['bits'], r['curve'], r['inst'], True)], 0.1)) for r, e in evs]
    rn = sum(a for a, _ in pn) / sum(b for _, b in pn)
    line.append(f'{rn:.2f}'); rec['ratio_vs_rhoneg_w0.1'] = rn
    summary.append(rec)
    print('| ' + ' | '.join(str(x) for x in line) + ' |')
json.dump(summary, open('summary.json', 'w'), indent=1)
