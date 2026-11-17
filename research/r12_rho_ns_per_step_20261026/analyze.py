#!/usr/bin/env python3
"""R12: paired ns/step tables and graph from the recorded data in data/.

Usage: python3 analyze.py   (run from this directory)
Writes results.json and ../../benchmarks/20261026-r12-rho-ns-per-step/ns_per_step.svg
Only recorded values are read; nothing is re-measured.
"""
import collections, glob, gzip, json, statistics as st

# ---- ecbench session: per-step wall of the strong and the tuned rho -------
recs = [json.loads(l) for l in gzip.open('data/ecbench_records.jsonl.gz', 'rt')]
recs = [r for r in recs if not r['warmup']]
cell = collections.defaultdict(lambda: [0, 0])  # (n, target, arm) -> wall, steps
for r in recs:
    c = r['counters']
    steps = c.get('walk_steps') or c.get('steps')
    k = (r['workload']['curve']['field_degree'], r['workload']['target_index'], r['arm'])
    cell[k][0] += r['time']['solve_wall_ns']
    cell[k][1] += steps
ns = {k: v[0] / v[1] for k, v in cell.items()}
sizes = sorted({k[0] for k in ns})
ecb = []
for n in sizes:
    tg = sorted({k[1] for k in ns if k[0] == n})
    tot = {a: [sum(cell[(n, t, a)][0] for t in tg), sum(cell[(n, t, a)][1] for t in tg)]
           for a in ('rho-strong', 'rho-strong-aa', 'rho-tuned')}
    m = {a: v[0] / v[1] for a, v in tot.items()}
    lose = sum(1 for t in tg if ns[(n, t, 'rho-tuned')] <= ns[(n, t, 'rho-strong')])
    ecb.append({
        'n': n, 'cells': len(tg),
        'strong_ns_per_step': m['rho-strong'], 'strong_aa_ns_per_step': m['rho-strong-aa'],
        'tuned_ns_per_step': m['rho-tuned'],
        'tuned_over_strong': m['rho-tuned'] / m['rho-strong'],
        'aa_over_strong': m['rho-strong-aa'] / m['rho-strong'],
        'cells_where_tuned_not_slower': lose,
    })

# ---- N83 worker: wide vs fold backend, ABBA -------------------------------
rows = []
for f in sorted(glob.glob('data/n83/*.json')):
    i, t1, t2, b = f.split('/')[-1][:-5].split('_')
    d = json.load(open(f))
    ga = d['outcome']['charges_setup_and_solve']['group_additions']
    rows.append((int(i), t1 + '_' + t2, b, d['online_interval_ns'] / ga, ga))
n83 = []
for t in sorted({r[1] for r in rows}):
    w = [r[3] for r in rows if r[1] == t and r[2] == 'wide']
    fo = [r[3] for r in rows if r[1] == t and r[2] == 'fold']
    n83.append({'target': t, 'steps': [r[4] for r in rows if r[1] == t][0],
                'wide_ns_per_step': st.mean(w), 'fold_ns_per_step': st.mean(fo),
                'wide_over_fold': st.mean(w) / st.mean(fo), 'runs_per_arm': len(w),
                'wide_range': [min(w), max(w)], 'fold_range': [min(fo), max(fo)]})

json.dump({'ecbench_small_n': ecb, 'n83_wide_vs_fold': n83}, open('results.json', 'w'), indent=1)
for e in ecb:
    print(e)
for e in n83:
    print(e)

# ---- graph ------------------------------------------------------------------
W, H = 760, 380
bars = []
for e in ecb:
    bars.append((f"n={e['n']} strong", e['strong_ns_per_step'], 0))
    bars.append((f"n={e['n']} tuned", e['tuned_ns_per_step'], 1))
for e in n83:
    bars.append((f"m=83 {e['target'][-1]} fold", e['fold_ns_per_step'], 2))
    bars.append((f"m=83 {e['target'][-1]} wide", e['wide_ns_per_step'], 3))
colors = ['#2a6f97', '#c0504d', '#3a7d44', '#8a6d3b']
ymax = max(b[1] for b in bars) * 1.1
x0, y0, bw, gap = 60, 40, 48, 14
out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="11">',
       f'<rect width="{W}" height="{H}" fill="#fff"/>',
       '<text x="60" y="22" font-size="14" font-weight="bold">Wall ns per walk step, one Apple arm64 core (L0, busy host)</text>']
for t in range(0, int(ymax), 200):
    y = H - 90 - (H - 130) * t / ymax
    out.append(f'<line x1="{x0}" x2="{W-10}" y1="{y:.1f}" y2="{y:.1f}" stroke="#ddd"/><text x="{x0-6}" y="{y+4:.1f}" text-anchor="end">{t}</text>')
for i, (lab, v, c) in enumerate(bars):
    x = x0 + 10 + i * (bw + gap)
    h = (H - 130) * v / ymax
    y = H - 90 - h
    out.append(f'<rect x="{x}" y="{y:.1f}" width="{bw}" height="{h:.1f}" fill="{colors[c]}"/>')
    out.append(f'<text x="{x+bw/2}" y="{y-4:.1f}" text-anchor="middle">{v:.0f}</text>')
    out.append(f'<text transform="translate({x+bw/2+4},{H-82}) rotate(50)" text-anchor="start">{lab}</text>')
out.append('<text x="60" y="' + str(H - 8) + '">blue/red: ecbench rho.signed_frobenius_strong / rho.signed_frobenius (u64 field). green/brown: N83 worker fold / wide backend (u128 field).</text>')
out.append('</svg>')
open('../../benchmarks/20261026-r12-rho-ns-per-step/ns_per_step.svg', 'w').write('\n'.join(out))
