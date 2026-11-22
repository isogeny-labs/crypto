#!/usr/bin/env python3
"""Validator: compare my recomputation (vdata.json) with analysis/summary.json and README.md numbers."""
import json, math, re, sys
L1 = sys.argv[1]
V = json.load(open('vdata.json')); S = json.load(open(f'{L1}/analysis/summary.json'))
R = open(f'{L1}/README.md').read()
out = dict(summary_vs_mine={}, readme_checks=[], mismatches=[])
NAMEMAP = {'T0/blind': 'T0/blind'}
# --- summary.json vs mine
maxpt = 0; maxci = 0
for cm, cs in zip(V['cells'], S['cells']):
    assert (cm['k'], cm['N']) == (cs['k'], cs['N'])
    for name, v in cm['ratios'].items():
        s = cs['ratios'][name]
        maxpt = max(maxpt, abs(v['mc'] - s['mc']), abs(v['exact'] - s['exact']))
        maxci = max(maxci, abs(v['mc_ci'][0] - s['mc_ci'][0]), abs(v['mc_ci'][1] - s['mc_ci'][1]))
    for a in ['bits_rand', 'rand', 'rand2', 'hash', 'hash2', 'rand_proj', 'hash_proj', 'plant3', 'plant5']:
        sa = cs['arms'][a]; ma = cm['arms'][a]
        maxpt = max(maxpt, abs(sa['mc_rate'] - ma['mc_rate']), abs(sa['exact_rate'] - ma['exact_rate']))
        if sa['hits'] != ma['hits']:
            out['mismatches'].append(f"hits {cs['k']} {cs['N']} {a}")
for name, v in V['pooled'].items():
    s = S['pooled'][name]
    maxpt = max(maxpt, abs(v['mc'] - s['mc']), abs(v['exact'] - s['exact']))
    maxci = max(maxci, abs(v['mc_ci'][0] - s['mc_ci'][0]), abs(v['mc_ci'][1] - s['mc_ci'][1]))
out['summary_vs_mine'] = dict(max_abs_point_diff=maxpt, max_abs_ci_endpoint_diff=maxci)
for k in S['predictions']:
    pass
pz = S['predictions']; keys = list(pz)
out['summary_pred_holds'] = [pz[k]['holds'] for k in keys]
out['mine_pred_holds'] = [V['predictions'][k]['holds'] for k in ['P1', 'P1p', 'P2', 'P2b', 'P3', 'P4', 'P5', 'P6', 'P7', 'P8']]


def chk(desc, readme_val, mine, tol):
    ok = abs(readme_val - mine) <= tol
    out['readme_checks'].append([desc, readme_val, mine, ok])
    if not ok:
        out['mismatches'].append(f'{desc}: README {readme_val} vs mine {mine}')


# --- README pooled table
for line in R.split('\n'):
    m = re.match(r'\| (hash/rand|hash/bits_rand|bits_rand/rand|hash_proj/rand_proj|rand_proj/rand|rand/rand2 \(A/A\)|hash/hash2 \(A/A\)|plant3/rand|plant5/rand|T0/blind) \| ([\d.]+) \| \[([\d.]+), ([\d.]+)\] \| ([\d.]+) \|$', line)
    if m:
        nm = m.group(1); v = V['pooled'][nm]
        chk(f'pooled {nm} mc', float(m.group(2)), v['mc'], 0.00005)
        chk(f'pooled {nm} ci_lo (own bootstrap)', float(m.group(3)), v['mc_ci'][0], 0.0015)
        chk(f'pooled {nm} ci_hi (own bootstrap)', float(m.group(4)), v['mc_ci'][1], 0.0015)
        chk(f'pooled {nm} exact', float(m.group(5)), v['exact'], 0.00005)
# --- README per-cell table 1
cols1 = ['hash/rand', 'hash/bits_rand', 'bits_rand/rand', 'hash_proj/rand_proj', 'hash/hash2 (A/A)', 'rand/rand2 (A/A)']
cols2 = ['rand_proj/rand', 'T0/blind', 'plant3/rand', 'plant5/rand']
cellidx = {(c['k'], c['N']): c for c in V['cells']}
for line in R.split('\n'):
    m = re.match(r'\| (\d+) \| (\d+) \| (.*) \|$', line)
    if not m:
        continue
    k, N = int(m.group(1)), int(m.group(2)); rest = [x.strip() for x in m.group(3).split('|')]
    if (k, N) not in cellidx:
        continue
    c = cellidx[(k, N)]
    if len(rest) == 6 and '[' in rest[0]:
        for nm, cell in zip(cols1, rest):
            mm = re.match(r'([\d.]+) \[([\d.]+), ([\d.]+)\]', cell)
            v = c['ratios'][nm]
            chk(f'cell {k}/{N} {nm} mc', float(mm.group(1)), v['mc'], 0.00005)
            chk(f'cell {k}/{N} {nm} ci_lo', float(mm.group(2)), v['mc_ci'][0], 0.003)
            chk(f'cell {k}/{N} {nm} ci_hi', float(mm.group(3)), v['mc_ci'][1], 0.003)
    elif len(rest) == 4 and '[' not in rest[0]:
        for nm, cell in zip(cols2, rest):
            dec = len(cell.split('.')[1])
            chk(f'cell {k}/{N} {nm} mc', float(cell), c['ratios'][nm]['mc'], 0.5 * 10 ** -dec + 1e-12)
    elif len(rest) == 7:   # cost table
        G, p, pc, w0, w01, w1 = (float(x.replace(',', '')) for x in rest[:6]) if False else (None,) * 6
        vals = [float(x) for x in rest]
        cr = [x for x in V['cost'] if x['k'] == k and x['N'] == N][0]
        chk(f'cost {k}/{N} N^2', vals[0], cr['G'], 0)
        chk(f'cost {k}/{N} p', vals[1], cr['p'], 0)
        chk(f'cost {k}/{N} p/N', vals[2], cr['per_col'], 0.5)
        chk(f'cost {k}/{N} w0', vals[3], cr['ratio_w0.0'], 0.005)
        chk(f'cost {k}/{N} w0.1', vals[4], cr['ratio_w0.1'], 0.005)
        chk(f'cost {k}/{N} w1', vals[5], cr['ratio_w1.0'], 0.005)
json.dump(out, open('vcompare.json', 'w'), indent=1)
print(json.dumps(out['summary_vs_mine']), out['summary_pred_holds'], out['mine_pred_holds'])
print('readme numeric checks', len(out['readme_checks']), 'failed', sum(not c[3] for c in out['readme_checks']))
for m in out['mismatches']:
    print('MISMATCH', m)
