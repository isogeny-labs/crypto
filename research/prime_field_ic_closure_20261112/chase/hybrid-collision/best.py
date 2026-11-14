#!/usr/bin/env python3
"""Best config per (bits, family, walk, stop) by mean cost at w=0.1, and its
paired ratio to rho-plain and rho-neg at w in {0,0.1,1} with 95% bootstrap CI.
Post-hoc min over a 2-5 point grid => mild downward selection bias. [pcc:unverified]"""
import json, sys, random
from collections import defaultdict
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from hyb import cost

rows = [json.loads(l) for f in sys.argv[1:] for l in open(f)]
W = (0, 0.1, 1)
rho = {(r['bits'], r['curve'], r['inst'], r['neg']): r['events']['rho'] for r in rows if r['arm'] == 'rho'}
def ev(r, kind):
    e = r['events']
    if kind == 'HYB-U':
        c = [e[k] for k in ('dep', 'rho') if k in e]
        return min(c, key=lambda s: s['steps']) if len(c) else None
    return e.get({'IC-G': 'gaudry', 'IC-1dep': 'dep', 'RHO': 'rho'}[kind])
def boot(pairs, B=2000, seed=1):
    rng = random.Random(seed); n = len(pairs); out = []
    for _ in range(B):
        s = [pairs[rng.randrange(n)] for _ in range(n)]
        out.append(sum(a for a, _ in s) / sum(b for _, b in s))
    out.sort(); return out[int(.025 * B)], out[int(.975 * B)]
g = defaultdict(list)
for r in rows:
    for k in (['RHO'] if r['arm'] == 'rho' else ['IC-G', 'IC-1dep', 'HYB-U']):
        g[(r['bits'], r['arm'], r['cfg'], r['neg'], k)].append(r)
best = {}
for key, rs in g.items():
    bits, arm, cfg, neg, kind = key
    evs = [(r, ev(r, kind)) for r in rs]
    if any(e is None for _, e in evs): continue
    c = sum(cost(e, 0.1) for _, e in evs) / len(evs)
    k2 = (bits, arm, neg, kind)
    if k2 not in best or c < best[k2][0]: best[k2] = (c, cfg, evs)
print('| bits | family | walk | stop | V | m | table | hits | ' + ' | '.join(f'w={w}: /rho-plain [CI]' for w in W) + ' | ' + ' | '.join(f'w={w}: /rho-neg [CI]' for w in W) + ' |')
print('|' + '---|' * (8 + 2 * len(W)))
out = []
for k2 in sorted(best):
    bits, arm, neg, kind = k2
    c, cfg, evs = best[k2]
    V = sum(r['V'] for r, _ in evs) / len(evs); m = sum(r['m'] for r, _ in evs) / len(evs)
    T = sum(r['table_size'] for r, _ in evs) / len(evs); h = sum(e['hits'] for _, e in evs) / len(evs)
    cells = []; rec = dict(bits=bits, family=arm, walk='neg' if neg else 'plain', stop=kind, V=V, m=m, table=T, hits=h)
    for ref in (False, True):
        for w in W:
            pairs = [(cost(e, w), cost(rho[(r['bits'], r['curve'], r['inst'], ref)], w)) for r, e in evs]
            ratio = sum(a for a, _ in pairs) / sum(b for _, b in pairs); lo, hi = boot(pairs)
            cells.append(f'{ratio:.2f} [{lo:.2f},{hi:.2f}]')
            rec[f'{"neg" if ref else "plain"}_w{w}'] = [ratio, lo, hi]
    out.append(rec)
    print(f'| {bits} | {arm} | {"neg" if neg else "plain"} | {kind} | {V:.0f} | {m:.0f} | {T:.0f} | {h:.1f} | ' + ' | '.join(cells) + ' |')
json.dump(out, open('best.json', 'w'), indent=1)
# stop-rule effect: IC-1dep vs IC-G on the same cfg (paired), w=0.1
print('\n| bits | family | walk | V | IC-G cost | IC-1dep cost | IC-G / IC-1dep [CI] | hits G | hits 1dep |')
print('|---|---|---|---|---|---|---|---|---|')
for key, rs in sorted(g.items()):
    bits, arm, cfg, neg, kind = key
    if kind != 'IC-G' or arm not in ('m2', 'm3'): continue
    pairs = [(cost(ev(r, 'IC-G'), 0.1), cost(ev(r, 'IC-1dep'), 0.1)) for r in rs if ev(r, 'IC-G') and ev(r, 'IC-1dep')]
    if len(pairs) < len(rs): continue
    lo, hi = boot(pairs); ra = sum(a for a, _ in pairs) / sum(b for _, b in pairs)
    hg = sum(ev(r, 'IC-G')['hits'] for r in rs) / len(rs); h1 = sum(ev(r, 'IC-1dep')['hits'] for r in rs) / len(rs)
    V = sum(r['V'] for r in rs) / len(rs)
    print(f'| {bits} | {arm} | {"neg" if neg else "plain"} | {V:.0f} | {sum(a for a,_ in pairs)/len(pairs):.0f} | {sum(b for _,b in pairs)/len(pairs):.0f} | {ra:.2f} [{lo:.2f},{hi:.2f}] | {hg:.1f} | {h1:.1f} |')
