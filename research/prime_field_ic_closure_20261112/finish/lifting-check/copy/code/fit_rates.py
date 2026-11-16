"""Power-law fits of realisation / success rates against p.  [pcc:unverified]
Weighted least squares of log(rate) on log(p) over cells with a nonzero count (weight = count);
zero-count cells are listed with their 95% upper bound and not fitted."""
import os, json, math
OUT = os.path.join(os.path.dirname(__file__), '..', 'out')
S = json.load(open(os.path.join(OUT, 'summary.json')))
MC = json.load(open(os.path.join(OUT, 'collinear_mc.json')))


def wfit(pts):
    # pts: (log p, log rate, weight)
    W = sum(w for _, _, w in pts)
    mx = sum(w * x for x, _, w in pts) / W
    my = sum(w * y for _, y, w in pts) / W
    sxx = sum(w * (x - mx) ** 2 for x, _, w in pts)
    sxy = sum(w * (x - mx) * (y - my) for x, y, w in pts)
    return sxy / sxx if sxx > 0 else float('nan')


lines = ['# Rate fits  [pcc:unverified]', '']
lines.append('Collinear Monte Carlo (planted P1+P2+P3 = O mod p, blind balanced lifts; realised iff the integer lifts are collinear):')
lines.append('')
lines.append('| bits | p (mean) | samples | collinear | rate | rate*p | rate*p/ln p |')
lines.append('|---|---|---|---|---|---|---|')
pts = []
for m in MC:
    lines.append(f"| {m['bits']} | {m['p_mean']:.0f} | {m['samples']} | {m['collinear']} | {m['rate']:.3e} | {m['rate_times_p']:.2f} | {m['rate_times_p']/math.log(m['p_mean']):.3f} |")
    if m['collinear']:
        pts.append((math.log(m['p_mean']), math.log(m['rate']), m['collinear']))
lines.append('')
lines.append(f'log-log slope of the collinear MC rate vs p: {wfit(pts):.3f}')
lines.append('')
lines.append('Pipeline arms (realised planted relation per trial, and honest-arm non-trivial relations per trial):')
lines.append('')
lines.append('| arm | r | cells with k>0 | slope d log(rate)/d log(p) | per-cell k/n (bits) |')
lines.append('|---|---|---|---|---|')
for arm in ('PC', 'PS', 'PD', 'POS', 'X'):
    for r in range(2, 7):
        cells = sorted([s for s in S if s['arm'] == arm and s['r'] == r], key=lambda s: s['bits'])
        if not cells:
            continue
        P = []
        desc = []
        for s in cells:
            if arm == 'X':
                k, n = s['rel_nontrivial'], s['n']
            else:
                k, n = s['planted_realized'], s['planted_n']
            desc.append(f"{k}/{n} ({s['bits']})")
            if k:
                P.append((math.log(s['p_mean']), math.log(k / n), k))
        sl = wfit(P) if len(P) >= 2 else float('nan')
        lines.append(f"| {arm} | {r} | {len(P)} | {sl:.2f} | {'; '.join(desc)} |")
txt = '\n'.join(lines) + '\n'
open(os.path.join(OUT, 'rate_fits.md'), 'w').write(txt)
print(txt)
