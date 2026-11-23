"""Apply the frozen PROTOCOL rule to widths*.jsonl. Usage: analyse.py OUT.md files..."""
import json, sys, statistics as st
from collections import defaultdict
rows = [json.loads(l) for f in sys.argv[2:] for l in open(f)]
grp = defaultdict(dict)
for r in rows:
    grp[(r['bits'], r['B_rule'])][r['arm']] = r
lines = ['| bits | p | r | B rule | arm | B | W2 | W2/N | band | W3 | W3/N | band | top-B energy (N) | max|F^|/B (N) | pinned m3 max-cut (N) |',
         '|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|']
verdict_cells = []
pc_ok = True
for key in sorted(grp, key=lambda k: (k[0], k[1])):
    g = grp[key]
    N = [g[f'N_random_s{s}'] for s in range(3) if f'N_random_s{s}' in g]
    def stat(field, f=lambda x: x):
        v = [f(n[field]) if not callable(field) else field(n) for n in N]
        m = st.mean(v); sd = st.pstdev(v) if len(v) > 1 else 0.0
        return m, sd
    out = {}
    for j in ('W2', 'W3'):
        m, sd = stat(j)
        out[j] = (m, max(3 * sd / m if m else 0, 0.02))
    spec = N[0]['spectrum'] is not None
    eN = st.mean(n['spectrum']['top_B'] for n in N) if spec else float('nan')
    mxN = st.mean(n['spectrum']['max_over_B'] for n in N) if spec else float('nan')
    pinN = st.mean(max(n['pinned_m3'].values()) for n in N)
    for arm in ('A1_fibre_randS', 'A2_fibre_intS', 'I_interval_x', 'P_AP_logs'):
        a = g[arm]
        r2 = a['W2'] / out['W2'][0]; r3 = a['W3'] / out['W3'][0]
        pin = max(a['pinned_m3'].values())
        lines.append(f"| {a['bits']} | {a['p']} | {a['r']} | {key[1]} | {arm} | {a['B']} | {a['W2']} | {r2:.3f} | {out['W2'][1]:.3f} | {a['W3']} | {r3:.3f} | {out['W3'][1]:.3f} | {(a['spectrum'] or {}).get('top_B', float('nan')):.4f} ({eN:.4f}) | {(a['spectrum'] or {}).get('max_over_B', float('nan')):.3f} ({mxN:.3f}) | {pin:.2f} ({pinN:.2f}) |")
        if arm.startswith('A'):
            within = all(abs(rr - 1) <= max(band, 0.10) for rr, band in ((r2, out['W2'][1]), (r3, out['W3'][1])))
            verdict_cells.append((key, arm, r2, r3, within))
        if arm == 'P_AP_logs' and a['B'] >= 32 and r2 > 0.2:
            pc_ok = False
summary = [f'positive control passes (P_AP_logs W2/N <= 0.2 for all B>=32): {pc_ok}',
           f'fibre cells within band (|ratio-1| <= max(band,0.10)) for W2 and W3: {sum(c[4] for c in verdict_cells)}/{len(verdict_cells)}',
           f'min fibre W2 ratio: {min(c[2] for c in verdict_cells):.4f}; min W3 ratio: {min(c[3] for c in verdict_cells):.4f}',
           f'max fibre W2 ratio: {max(c[2] for c in verdict_cells):.4f}; max W3 ratio: {max(c[3] for c in verdict_cells):.4f}']
open(sys.argv[1], 'w').write('\n'.join(summary) + '\n\n' + '\n'.join(lines) + '\n')
print('\n'.join(summary))
