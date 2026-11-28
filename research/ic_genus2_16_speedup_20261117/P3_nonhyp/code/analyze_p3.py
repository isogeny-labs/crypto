#!/usr/bin/env python3
"""P3 analysis (frozen with PROTOCOL.md). Every number [pcc:unverified].

Per (d, m, p, curve): for each arm, the relation phase (enumeration + relation
work, FPM) and the LA bracket [la_low, la_high]; totals low/high. Best arm per
curve = the arm of minimum total_low among non-exhausted arms. Rho = expected
negation-map steps sqrt(pi N / 4), N = p^g, times the measured hyperelliptic
Cantor step cost of the same genus (a lower bound for a non-hyperelliptic Pic
step, so conservative for IC); the plain walk sqrt(pi N / 2) is also printed.
Slopes: OLS of log(best total_low) on log p, geometric mean over curves per p.
"""
import glob, json, math, sys
from collections import defaultdict
STEP = {3: 290.3, 4: 479.8, 6: 1164.9}   # measured, see PROTOCOL sec 4
rows = []
for f in sys.argv[1:] or glob.glob('raw/*.jsonl'):
    for l in open(f):
        rows.append(json.loads(l))
by = defaultdict(list)
for r in rows:
    by[(r['d'], r['m'], r['g'], r['p'], r['curve_index'])].append(r)
cell = defaultdict(list)
dups_counted = 0
for k, rs in sorted(by.items()):
    d, m, g, p, c = k
    ok = [r for r in rs if not r['exhausted']]
    for r in rs:
        if r['useful'] > r['lines_tried'] - r['rel_dup_lines']:
            dups_counted += 1
    if not ok:
        cell[(d, m, g, p)].append(None); continue
    b = min(ok, key=lambda r: r['fpm_total_low'])
    rho_neg = math.sqrt(math.pi * p ** g / 4) * STEP[g]
    rho_pl = math.sqrt(math.pi * p ** g / 2) * STEP[g]
    cell[(d, m, g, p)].append(dict(arm=f"{b['arm']}{b['cF'] or ''}", rel=b['fpm_relphase'], lo=b['fpm_total_low'],
                                   hi=b['fpm_total'], rho=rho_neg, rho_pl=rho_pl,
                                   g12=sum(r['g1_fail'] + r['g2_fail'] for r in rs)))
gm = lambda xs: math.exp(sum(math.log(x) for x in xs) / len(xs))
print('| d | m | g | p | curves solved | best arm(s) | rel phase | total low | total high | low/rho_neg | high/rho_neg | low/rho_plain | G1+G2 fails |')
print('|---|---|---|---|---|---|---|---|---|---|---|---|---|')
series = defaultdict(list)
for k in sorted(cell):
    d, m, g, p = k
    v = [x for x in cell[k] if x]
    if not v:
        print(f'| {d} | {m} | {g} | {p} | 0/{len(cell[k])} | - | - | - | - | - | - | - | - |'); continue
    lo, hi, rel = gm([x['lo'] for x in v]), gm([x['hi'] for x in v]), gm([x['rel'] for x in v])
    rho, rpl = v[0]['rho'], v[0]['rho_pl']
    arms = ','.join(sorted(set(x['arm'] for x in v)))
    series[(d, m, g)].append((p, lo, hi, rel))
    print(f"| {d} | {m} | {g} | {p} | {len(v)}/{len(cell[k])} | {arms} | {rel:.3g} | {lo:.3g} | {hi:.3g} | {lo/rho:.3g} | {hi/rho:.3g} | {lo/rpl:.3g} | {sum(x['g12'] for x in v)} |")
def slope(pts):
    xs = [math.log(a) for a, _ in pts]; ys = [math.log(b) for _, b in pts]
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
print()
print('| d | m | g | primes | slope total_low | slope total_high | slope rel phase | slope total_low, 4 largest p |')
print('|---|---|---|---|---|---|---|---|')
for k, s in sorted(series.items()):
    if len(s) < 3: continue
    s.sort()
    lo = slope([(p, a) for p, a, _, _ in s]); hi = slope([(p, b) for p, _, b, _ in s]); rl = slope([(p, c) for p, _, _, c in s])
    l4 = slope([(p, a) for p, a, _, _ in s[-4:]]) if len(s) >= 4 else float('nan')
    print(f"| {k[0]} | {k[1]} | {k[2]} | {','.join(str(p) for p, *_ in s)} | {lo:.3f} | {hi:.3f} | {rl:.3f} | {l4:.3f} |")
print(f"\nrows: {len(rows)}; rows where useful exceeds distinct lines tried: {dups_counted}")
