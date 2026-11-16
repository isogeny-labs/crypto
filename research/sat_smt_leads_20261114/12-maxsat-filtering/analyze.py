#!/usr/bin/env python3
"""Summarise results_raw.json -> results.json + markdown table on stdout."""
import json, math, sys, statistics as st
d = json.load(open('results_raw.json'))
rows = d['rows']
val = json.load(open('validation.json'))
cells = {}
for r in rows:
    cells.setdefault((r['N'], r['excess_pct']), []).append(r)


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs)) if xs else None


summ = []
print('| N | excess | reps | G0 cols | G1 cols | G2 cols | G3 cols (restarts) | RC2 OPT / cols | ANY OPT / improved / cols | best MaxSAT/greedy cols | cost_sel ratio | cost_sge ratio (gm) | RC2 conflicts (OPT) | ANY conflicts |')
print('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|')
for (N, e), rs in sorted(cells.items()):
    rs = [r for r in rs if 'skip' not in r]
    nskip = len(cells[(N, e)]) - len(rs)
    if not rs:
        print(f'| {N} | {e}% | 0 (+{nskip} skipped: no excess after singleton removal) |' + ' - |' * 11)
        summ.append(dict(N=N, excess_pct=e, reps=0, skipped=nskip))
        continue
    m = lambda f: st.mean(f(r) for r in rs)
    s = dict(N=N, excess_pct=e, reps=len(rs), skipped=nskip, g0=m(lambda r: r['g0_cols']),
             g1=m(lambda r: r['g1']['cols']), g2=m(lambda r: r['g2']['cols']))
    if 'g3' not in rs[0]:
        s['g1_cost_sel'] = m(lambda r: r['g1']['cost_sel']); s['g1_cost_sge'] = m(lambda r: r['g1']['cost_sge'])
        s['g2_cost_sel'] = m(lambda r: r['g2']['cost_sel']); s['g2_cost_sge'] = m(lambda r: r['g2']['cost_sge'])
        print(f"| {N} | {e}% | {len(rs)} | {s['g0']:.1f} | {s['g1']:.1f} | {s['g2']:.1f} | not run | not run | not run | - | - | - | - | - |")
        summ.append(s)
        continue
    s['g3'] = m(lambda r: r['g3']['cols']); s['g3_restarts'] = m(lambda r: r['g3_restarts'])
    opt = [r for r in rs if r['rc2']['status'] == 'OPT']
    aopt = [r for r in rs if r['any']['status'] == 'OPT']
    s['rc2_opt'] = len(opt); s['any_opt'] = len(aopt)
    s['rc2_cols'] = st.mean(r['rc2']['cols'] for r in opt) if opt else None
    s['any_cols'] = m(lambda r: r['any']['cols'])
    s['any_improved_instances'] = sum(1 for r in rs if r['any']['cols'] < r['greedy_cols'])
    rc, rsel, rsge = [], [], []
    for r in rs:
        gcost_sel = r['g3']['cost_sel']
        gcost_sge = min(r['g1']['cost_sge'], r['g2']['cost_sge'], r['g3']['cost_sge'])
        cands = [r['any']] + ([r['rc2']] if r['rc2']['status'] == 'OPT' else [])
        b = min(cands, key=lambda c: c['cols'])
        rc.append(b['cols'] / r['greedy_cols'])
        rsel.append(b['cost_sel'] / gcost_sel)
        rsge.append(min(c['cost_sge'] for c in cands) / gcost_sge)
    s['ratio_cols'] = gm(rc); s['ratio_cost_sel'] = gm(rsel); s['ratio_cost_sge'] = gm(rsge)
    s['ratio_cost_sge_min'] = min(rsge); s['ratio_cost_sge_max'] = max(rsge)
    s['ratio_cost_sel_min'] = min(rsel)
    s['rc2_conflicts_gm'] = gm([max(1, r['rc2']['conflicts']) for r in opt]) if len(opt) == len(rs) else None
    s['any_conflicts_mean'] = m(lambda r: r['any']['conflicts'])
    summ.append(s)
    f = lambda x, p='.1f': '-' if x is None else format(x, p)
    print(f"| {N} | {e}% | {len(rs)}{' (+%d skipped)' % nskip if nskip else ''} | {s['g0']:.1f} | {s['g1']:.1f} | {s['g2']:.1f} | {s['g3']:.1f} ({s['g3_restarts']:.0f}) | "
          f"{len(opt)}/{len(rs)} / {f(s['rc2_cols'])} | {len(aopt)}/{len(rs)} / {s['any_improved_instances']} / {s['any_cols']:.1f} | "
          f"{s['ratio_cols']:.4f} | {s['ratio_cost_sel']:.4f} | {s['ratio_cost_sge']:.4f} [{s['ratio_cost_sge_min']:.3f},{s['ratio_cost_sge_max']:.3f}] | {f(s['rc2_conflicts_gm'], '.0f')} | {s['any_conflicts_mean']:.0f} |")

# slopes
def fit(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

slopes = {}
for e in (5, 20, 100):
    pts = [(math.log2(s['N']), math.log2(s['rc2_conflicts_gm'])) for s in summ
           if s['excess_pct'] == e and s.get('rc2_conflicts_gm')]
    slopes[f'rc2_log2conf_per_log2N_e{e}'] = dict(points=pts, slope=fit(*zip(*pts)) if len(pts) >= 3 else None)
    # greedy fraction scaling: cols kept / N for G2 (baseline arm), all sizes
    pts = [(math.log2(s['N']), math.log2(s['g2'])) for s in summ if s['excess_pct'] == e and s.get('g2')]
    slopes[f'g2_log2cols_per_log2N_e{e}'] = dict(points=pts, slope=fit(*zip(*pts)) if len(pts) >= 3 else None)
print()
for k, v in slopes.items():
    print(k, 'slope =', v['slope'], 'n_points =', len(v['points']))
json.dump(dict(note='pilot, all numbers [pcc:unverified]', T=d['T'], validation=val, cell_summary=summ,
               slopes=slopes, rows=rows), open('results.json', 'w'), indent=1)
