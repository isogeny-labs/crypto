#!/usr/bin/env python3
"""Summarise results-*.json: per-candidate field ops, totals, IC/rho ratios. [pcc:unverified]"""
import sys, json, math, statistics as st
sys.path.insert(0, __file__.rsplit('/', 1)[0])
from toy import bitop, wordram

def walk(r, k):
    # walk-phase ops: total minus pre-loop, minus restart charges (3*bits affine adds each)
    nb = r['n'].bit_length(); per = {'I': 1, 'M': 2, 'S': 1, 'A': 6, 'adds': 1}
    nres = r.get('restarts', 0) if r['arm'].startswith('ic') else r.get('restart', 0) - (16 if 'L16' in r['arm'] else 1)
    return r[k] - r['pre_' + k] - max(nres, 0) * 3 * nb * per[k]

def price(r, pr, which='total'):
    if which == 'total':
        f = sum(r[k] * pr[k] for k in 'IMSA')
    else:
        f = sum(walk(r, k) * pr[k] for k in 'IMSA')
    return f + pr['probe'] * (r['probe'] + r['guard'] + r['insert'])

def gae(r, w):
    native = r['second'] + r['probe'] + r['guard'] + r['insert'] + r['rowop'] + r['lift']
    return r['adds'] + r['dbls'] + w * native

def gm(xs): return math.exp(sum(math.log(x) for x in xs) / len(xs))

files = sys.argv[1:]
for fn in files:
    D = json.load(open(fn)); R = D['records']
    arms = []
    for r in R:
        if r['arm'] not in arms: arms.append(r['arm'])
    print(f'\n=== {fn}: bits={D["bits"]} theta=2^-{D["theta_bits"]}, records={len(R)}, all ok={all(r["ok"] for r in R)}')
    by = {a: [r for r in R if r['arm'] == a] for a in arms}
    print('\nPer-unit walk counts (means). IC unit = candidate (probe-eligible point); rho unit = walk step.')
    print(f'{"arm":20s} {"ok":>5s} {"units/sqrt(n)":>13s} {"I/unit":>7s} {"M/unit":>7s} {"S/unit":>7s} {"A/unit":>7s} {"adds/unit":>9s} {"cand/row":>8s} {"minus%rows":>10s} {"dup":>4s}')
    for a in arms:
        rs = by[a]; ic = a.startswith('ic')
        units = [(r['cand'] if ic else r['steps'] + r['second']) for r in rs]
        if not ic: units = [r['steps'] + (r['second'] if 'store-all-side' in a else 0) for r in rs]
        u = sum(units)
        line = f'{a:20s} {sum(r["ok"] for r in rs):>2d}/{len(rs):<2d} {st.mean(un / math.sqrt(r["n"]) for un, r in zip(units, rs)):13.3f}'
        for k in ('I', 'M', 'S', 'A', 'adds'):
            line += f' {sum(walk(r, k) for r in rs) / u:7.3f}' if k != 'adds' else f' {sum(walk(r, k) for r in rs) / u:9.3f}'
        if ic:
            line += f' {sum(r["cand"] for r in rs) / sum(r["rows"] for r in rs):8.1f} {100 * sum(r["minus_rows"] for r in rs) / sum(r["rows"] for r in rs):10.1f} {sum(r["dup_rows"] for r in rs):4d}'
        print(line)
    # phase split for IC arms in GAE w=0
    print('\nIC phase split, GAE at w=0 (means per run): table build | walk set-up+restarts | walk')
    for a in arms:
        if not a.startswith('ic'): continue
        rs = by[a]; nb = rs[0]['n'].bit_length()
        tb = st.mean(r['setup_adds'] for r in rs)
        tot = st.mean(gae(r, 0) for r in rs)
        wk = st.mean(walk(r, 'adds') for r in rs)
        print(f'  {a:16s} total {tot:9.0f}  table {tb:7.0f} ({100*tb/tot:4.1f}%)  set-up/restarts {tot-tb-wk:7.0f} ({100*(tot-tb-wk)/tot:4.1f}%)  walk {wk:8.0f} ({100*wk/tot:4.1f}%)')
    # totals and ratios
    print('\nTotal cost per run, ratio of means; IC arms / rho arms (paired by target, geometric mean of per-target ratios in brackets)')
    schemes = [('GAE w=0', lambda r: gae(r, 0)), ('GAE w=0.1', lambda r: gae(r, 0.1)), ('GAE w=1', lambda r: gae(r, 1))]
    for b in (20, 64, 256):
        schemes.append((f'bitop b={b}', lambda r, b=b: price(r, bitop(b))))
        schemes.append((f'wordram b={b}', lambda r, b=b: price(r, wordram(b))))
    ics = [a for a in arms if a.startswith('ic')]
    rhos = [a for a in arms if not a.startswith('ic')]
    hdr = f'{"scheme":16s}' + ''.join(f' {a[:16]:>16s}' for a in arms)
    print('\nMean total cost / mean(rho-neg-L1) total cost:')
    print(hdr)
    for name, fnc in schemes:
        base = st.mean(fnc(r) for r in by['rho-neg-L1'])
        print(f'{name:16s}' + ''.join(f' {st.mean(fnc(r) for r in by[a]) / base:16.3f}' for a in arms))
    print('\nIC / best-rho (best rho arm chosen per scheme among DP-rho arms; store-all excluded) and paired/single:')
    for name, fnc in schemes:
        dprho = {a: st.mean(fnc(r) for r in by[a]) for a in rhos if a.startswith('rho')}
        best = min(dprho, key=dprho.get)
        s1 = st.mean(fnc(r) for r in by['ic-single-L1']); p1 = st.mean(fnc(r) for r in by['ic-paired-L1'])
        s16 = st.mean(fnc(r) for r in by['ic-single-L16']); p16 = st.mean(fnc(r) for r in by['ic-paired-L16'])
        bi = min(s1, s16); bp = min(p1, p16)
        print(f'  {name:16s} best-rho={best:17s} best-single-IC/rho={bi / dprho[best]:7.2f}  best-paired-IC/rho={bp / dprho[best]:7.2f}  paired/single (L1 {p1/s1:.3f}, L16 {p16/s16:.3f})')
    print('\nWalk-only cost per unit (no table, no set-up), priced; IC per candidate vs rho-neg per step:')
    for name, pr in [('bitop b=20', bitop(20)), ('bitop b=256', bitop(256)), ('wordram b=20', wordram(20)), ('wordram b=256', wordram(256))]:
        row = f'  {name:14s}'
        ref = None
        for a in arms:
            rs = by[a]; ic = a.startswith('ic')
            u = sum((r['cand'] if ic else r['steps'] + (r['second'] if 'store-all-side' in a else 0)) for r in rs)
            v = sum(sum(walk(r, k) * pr[k] for k in 'IMSA') for r in rs) / u
            if a == 'rho-neg-L1': ref = v
            row += f' {a}={v:,.0f}'
        print(row)
