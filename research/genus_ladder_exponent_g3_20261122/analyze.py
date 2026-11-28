#!/usr/bin/env python3
import json, math, random, sys
R9 = '../genus_ladder_wallclock_vs_rho_20261122/raw/'
OLD = [65537, 131101, 262147]; NEW = [524309, 1048583]; PR = OLD + NEW
IC, RH, NS = 'C_double_lp+pre+sge(8,3)', 'R_rho_neg', 'C_double_lp+pre'
def path(p): return (R9 if p in OLD else 'raw/') + f'run_{p}.jsonl'
def load(p):
    rows = [json.loads(l) for l in open(path(p))]
    runs = {}; skips = []
    for r in rows:
        if r['kind'] == 'run': runs[(r['curve_seed'], r['seed'], r['arm'])] = r
        elif r['kind'] == 'skip': skips.append(r)
    inst = [r for r in rows if r['kind'] == 'instance']
    return runs, skips, inst
D = {p: load(p) for p in PR}
# metrics: name -> (arm, f(row))
M = {
 'wall_ic': (IC, lambda r: r['wall_ns'] / 1e9), 'wall_rho': (RH, lambda r: r['wall_ns'] / 1e9),
 'wall_nosge': (NS, lambda r: r['wall_ns'] / 1e9),
 'cnt_rel': (IC, lambda r: r['cantor_mul'] + 10 * r['cantor_inv'] + r['rel_mul']),
 'cnt_la': (IC, lambda r: r['la_mul']),
 'cnt_rho': (RH, lambda r: r['cantor_mul'] + 10 * r['cantor_inv']),
}
def vals(p):
    runs = D[p][0]; cs = sorted({k[0] for k in runs}); sd = sorted({k[1] for k in runs})
    return cs, sd, {m: {(c, s): f(runs[(c, s, a)]) for c in cs for s in sd} for m, (a, f) in M.items()}
V = {p: vals(p) for p in PR}
def pmean(p, m, samp=None):
    cs, sd, v = V[p]
    if samp is None: samp = [(c, sd) for c in cs]
    return sum(sum(v[m][(c, s)] for s in ss) / len(ss) for c, ss in samp) / len(samp)
def slope(xs, ys):
    lx = [math.log(x) for x in xs]; ly = [math.log(y) for y in ys]
    mx = sum(lx) / len(lx); my = sum(ly) / len(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)
def draw(rng):
    out = {}
    for p in PR:
        cs, sd, _ = V[p]
        out[p] = [(c, [rng.choice(sd) for _ in sd]) for c in (rng.choice(cs) for _ in cs)]
    return out
NB = 4000; rng = random.Random(20261122)
draws = [draw(rng) for _ in range(NB)]
def q(a):
    a = sorted(a); return a[int(.025 * len(a))], a[int(.975 * len(a)) - 1]
res = {'primes': {}, 'exp': {}, 'e1': {}}
for p in PR:
    runs, skips, inst = D[p]
    P = {'n_curves': len(V[p][0]), 'n_runs': len(runs), 'n_skips': len(skips),
         'all_certified': all(r['certified'] for r in runs.values()),
         'stopped': [k for k, r in runs.items() if not r['certified']]}
    for m in M: P['mean_' + m] = pmean(p, m)
    rat = [pmean(p, 'wall_ic', d[p]) / pmean(p, 'wall_rho', d[p]) for d in draws]
    P['ratio'] = pmean(p, 'wall_ic') / pmean(p, 'wall_rho'); P['ratio_ci'] = q(rat)
    rat2 = [pmean(p, 'wall_nosge', d[p]) / pmean(p, 'wall_rho', d[p]) for d in draws]
    P['ratio_nosge'] = pmean(p, 'wall_nosge') / pmean(p, 'wall_rho'); P['ratio_nosge_ci'] = q(rat2)
    P['per_curve'] = {c: dict(ic_s=sum(V[p][2]['wall_ic'][(c, s)] for s in V[p][1]) / len(V[p][1]),
                              rho_s=sum(V[p][2]['wall_rho'][(c, s)] for s in V[p][1]) / len(V[p][1])) for c in V[p][0]}
    P['curve_seeds'] = V[p][0]; P['orders'] = [(i['curve_seed'], i['order'], i['r']) for i in inst]
    P['rho_steps_mean'] = sum(r.get('steps', r.get('walk_ops', 0)) for r in runs.values() if r['arm'] == RH) / max(1, len(V[p][0]) * len(V[p][1]))
    P['order_search_wall_s'] = [i['order_search_wall_s'] for i in inst]
    res['primes'][p] = P
def fit(fn):
    pt = slope(PR, [fn(p, None) for p in PR])
    bs = [slope(PR, [fn(p, d[p]) for p in PR]) for d in draws]
    return pt, bs
for m in M:
    pt, bs = fit(lambda p, s, m=m: pmean(p, m, s))
    res['exp'][m] = dict(point=pt, ci=q(bs))
    M[m] = M[m] + (bs,)
dif = [a - b for a, b in zip(M['wall_ic'][2], M['wall_rho'][2])]
res['exp']['wall_diff_ic_minus_rho'] = dict(point=res['exp']['wall_ic']['point'] - res['exp']['wall_rho']['point'], ci=q(dif))
# also three-prime (R9 only) and new-binary-only descriptives for context
res['exp3_point'] = {m: slope(OLD, [pmean(p, m) for p in OLD]) for m in M}
ci_ic, ci_rho = res['exp']['wall_ic']['ci'], res['exp']['wall_rho']['ci']
res['separation_holds'] = ci_ic[1] < ci_rho[0]
json.dump(res, open('analysis.json', 'w'), indent=1, default=str)
# ---- tables
o = []; w = o.append
w('## T1 per-prime wall (s) and IC+SGE / rho ratio `[pcc:unverified]`\n')
w('Old primes: R9 rows (6 curves x 2 seeds). New primes: this run (3 curves x 2 seeds). 95 % hierarchical bootstrap, 4000 draws.\n')
w('| p | curves x seeds | runs certified | mean wall IC+SGE (s) | mean wall no-SGE (s) | mean wall rho (s) | IC+SGE / rho | 95 % interval | no-SGE / rho |')
w('|--:|--:|:-:|--:|--:|--:|--:|:--|--:|')
for p in PR:
    P = res['primes'][p]
    w(f"| {p} | {P['n_curves']} x {len(V[p][1])} | {P['n_runs']} ({'all' if P['all_certified'] else 'NOT all'}) | {P['mean_wall_ic']:.2f} | {P['mean_wall_nosge']:.2f} | {P['mean_wall_rho']:.2f} | {P['ratio']:.3f} | [{P['ratio_ci'][0]:.3f}, {P['ratio_ci'][1]:.3f}] | {P['ratio_nosge']:.3f} |")
w('\n## T2 per-prime mean counts\n')
w('| p | relation phase (cantor_mul+10 inv+rel_mul) | la_mul | rho (cantor_mul+10 inv) | mean rho walk steps |')
w('|--:|--:|--:|--:|--:|')
for p in PR:
    P = res['primes'][p]
    w(f"| {p} | {P['mean_cnt_rel']:.4g} | {P['mean_cnt_la']:.4g} | {P['mean_cnt_rho']:.4g} | {P['rho_steps_mean']:.4g} |")
w('\n## T3 exponents over the five primes (log-log least squares on per-prime means; 95 % bootstrap over curves, then seeds, within prime)\n')
w('| quantity | exponent | 95 % interval | reference |'); w('|--|--:|:--|--|')
lab = [('wall_ic', 'wall, IC+SGE(8,3)', ''), ('wall_rho', 'wall, negation rho', ''), ('wall_nosge', 'wall, IC no SGE', ''),
       ('cnt_rel', 'count, IC relation phase', '4/3'), ('cnt_la', 'count, IC linear algebra (la_mul)', '3/2'), ('cnt_rho', 'count, rho', '3/2')]
for m, t, ref in lab:
    e = res['exp'][m]; w(f"| {t} | {e['point']:.3f} | [{e['ci'][0]:.3f}, {e['ci'][1]:.3f}] | {ref} |")
e = res['exp']['wall_diff_ic_minus_rho']; w(f"| wall exponent difference IC - rho (paired draws; secondary) | {e['point']:.3f} | [{e['ci'][0]:.3f}, {e['ci'][1]:.3f}] | 0 |")
w(f"\nThree-prime (R9 only) point exponents for reference: IC {res['exp3_point']['wall_ic']:.3f}, rho {res['exp3_point']['wall_rho']:.3f}.")
w(f"\nSeparation test (IC upper end < rho lower end): IC [{ci_ic[0]:.3f}, {ci_ic[1]:.3f}], rho [{ci_rho[0]:.3f}, {ci_rho[1]:.3f}] -> **{'HOLDS' if res['separation_holds'] else 'DOES NOT HOLD'}**.")
w('\n## T4 per-curve mean wall (s) at the new primes\n')
w('| p | curve seed | IC+SGE | rho | ratio |'); w('|--:|--:|--:|--:|--:|')
for p in NEW:
    for c, v in res['primes'][p]['per_curve'].items(): w(f"| {p} | {c} | {v['ic_s']:.1f} | {v['rho_s']:.1f} | {v['ic_s']/v['rho_s']:.3f} |")
open('tables.md', 'w').write('\n'.join(o) + '\n')
# ---- svg
SURF, INK, INK2, GRID = '#fcfcfb', '#0b0b0b', '#52514e', '#e4e3df'
W_, H_ = 760, 460; L, B, T, Rr = 80, 80, 56, 30
allv = [V[p][2][m][k] for p in PR for m in ('wall_ic', 'wall_rho') for k in V[p][2][m]]
lo, hi = math.floor(math.log10(min(allv))), math.ceil(math.log10(max(allv)))
x0, x1 = math.log10(PR[0]) - .05, math.log10(PR[-1]) + .05
X = lambda p: L + (math.log10(p) - x0) / (x1 - x0) * (W_ - L - Rr)
Y = lambda v: H_ - B - (math.log10(v) - lo) / (hi - lo) * (H_ - B - T)
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W_} {H_}" font-family="sans-serif" font-size="12"><rect width="{W_}" height="{H_}" fill="{SURF}"/>']
s.append(f'<text x="{L}" y="26" font-size="15" fill="{INK}">Genus 3: wall time against p, IC (2LP+pre+SGE(8,3)) and negation rho</text>')
s.append(f'<text x="{L}" y="44" fill="{INK2}">dots: per-curve mean wall; rings: per-prime mean; lines: log-log fits over five primes. [pcc:unverified]</text>')
for k in range(lo, hi + 1):
    s.append(f'<line x1="{L}" x2="{W_-Rr}" y1="{Y(10**k):.1f}" y2="{Y(10**k):.1f}" stroke="{GRID}"/><text x="{L-8}" y="{Y(10**k)+4:.1f}" text-anchor="end" fill="{INK2}">{10**k:g} s</text>')
for p in PR:
    s.append(f'<line x1="{X(p):.1f}" x2="{X(p):.1f}" y1="{T}" y2="{H_-B}" stroke="{GRID}"/><text x="{X(p):.1f}" y="{H_-B+18}" text-anchor="middle" fill="{INK2}">{p}</text>')
s.append(f'<text x="{(L+W_-Rr)/2}" y="{H_-B+38}" text-anchor="middle" fill="{INK}">p (log scale)</text>')
for m, c, lbl in (('wall_ic', '#2a78d6', 'IC + SGE(8,3)'), ('wall_rho', '#eb6834', 'negation rho')):
    e = res['exp'][m]; ys = [math.log(res['primes'][p]['mean_' + m]) for p in PR]; xs = [math.log(p) for p in PR]
    a = sum(ys) / 5 - e['point'] * sum(xs) / 5
    f = lambda p: math.exp(a + e['point'] * math.log(p))
    s.append(f'<line x1="{X(PR[0]):.1f}" x2="{X(PR[-1]):.1f}" y1="{Y(f(PR[0])):.1f}" y2="{Y(f(PR[-1])):.1f}" stroke="{c}" stroke-width="2"/>')
    for p in PR:
        cs, sd, v = V[p]
        for ci, cc in enumerate(cs):
            val = sum(v[m][(cc, ss)] for ss in sd) / len(sd)
            s.append(f'<circle cx="{X(p)+(-5 if m=="wall_ic" else 5):.1f}" cy="{Y(val):.1f}" r="3" fill="{c}" fill-opacity=".55"/>')
        s.append(f'<circle cx="{X(p)+(-5 if m=="wall_ic" else 5):.1f}" cy="{Y(res["primes"][p]["mean_"+m]):.1f}" r="7" fill="none" stroke="{INK}" stroke-width="1.5"/>')
yl = H_ - 22
for i, (m, c, lbl) in enumerate((('wall_ic', '#2a78d6', 'IC + SGE(8,3)'), ('wall_rho', '#eb6834', 'negation rho'))):
    e = res['exp'][m]; xx = L + i * 300
    s.append(f'<line x1="{xx}" x2="{xx+22}" y1="{yl-4}" y2="{yl-4}" stroke="{c}" stroke-width="2"/><text x="{xx+28}" y="{yl}" fill="{INK2}">{lbl}: exponent {e["point"]:.2f} [{e["ci"][0]:.2f}, {e["ci"][1]:.2f}]</text>')
s.append('</svg>')
open('fig_exponent.svg', 'w').write('\n'.join(s))
print('ok')
