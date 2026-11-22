#!/usr/bin/env python3
"""Positive controls PC1 and PC2 (PROTOCOL.md).

PC1 (exact replay): run the verbatim frame-search instrument (scripts/frame_census_orig.py, sha256 afcf2cc3...)
with the frame-search k = 20 command and seed (20261103 + 20). --t 4 5 6 is needed so that the RNG state at
t = 5, 6 equals the original run (t = 4 consumes draws first). Apply the frame-search selection rule
(analyse.py: argmax J, then cols, then lowest L over translation frames) and compare selected_L,
selected_covered, gain_vs_mean and gain_vs_bits_lo0 at t = 5, 6 with the published summary
(scripts/frame_search_summary_ref.json, a copy of analysis/summary.json, sha256 753be4c0...).
PC2 (instrument equivalence): recompute cols, J and covered of every translation frame and of bits_lo0
at t = 4, 5, 6 with this lead's fast instrument (l5common) and require exact equality.
Usage: pc_replay.py <workdir>   (writes <workdir>/pc_replay.json; the raw census file is deleted after use)
"""
import json, os, subprocess, sys, time
here = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, here)
import numpy as np
from l5common import CURVES, build, frame_logs_translate, class_counts, J_from_counts, J_exact, covered_exact

def main():
    wd = sys.argv[1]; os.makedirs(wd, exist_ok=True)
    k, p, A, n = CURVES['k20']
    raw = os.path.join(wd, 'pc1_raw_k20.json')
    t0 = time.time()
    subprocess.run([sys.executable, '-I', os.path.join(here, 'frame_census_orig.py'), '--p', str(p), '--A', str(A),
                    '--n', str(n), '--t', '4', '5', '6', '--K', '2000', '--seed', str(20261103 + 20), '--out', raw],
                   check=True, stderr=subprocess.DEVNULL)
    t_orig = time.time() - t0
    d = json.load(open(raw))
    ref = {(c['k'], c['t']): c for c in json.load(open(os.path.join(here, 'frame_search_summary_ref.json')))['cells']}
    out = dict(pc1=[], pc2=[], seconds_orig=round(t_orig, 1))
    logu, r, *_ = build(p, A, n); A3 = A * pow(3, -1, p) % p
    for t in (4, 5, 6):
        R = [x for x in d['rows'] if x['t'] == t]
        tr = [x for x in R if x['family'] == 'translate']
        b0 = next(x for x in R if x['family'] == 'bits_lo0')
        sel = sorted(tr, key=lambda x: (-x['J'], -x['cols'], x['frame'][1]))[0]
        mean = float(np.mean([x['covered'] for x in tr]))
        got = dict(t=t, selected_L=sel['frame'][1], selected_covered=sel['covered'], gain_vs_mean=sel['covered'] / mean,
                   gain_vs_bits_lo0=sel['covered'] / b0['covered'], trans_mean=mean)
        rc = ref[(20, t)]
        got['matches_published'] = (got['selected_L'] == rc['selected_L'] and got['selected_covered'] == rc['selected_covered']
                                    and abs(got['gain_vs_mean'] - rc['gain_vs_mean']) < 1e-12
                                    and abs(got['gain_vs_bits_lo0'] - rc['gain_vs_bits_lo0']) < 1e-12)
        got['published'] = dict(selected_L=rc['selected_L'], selected_covered=rc['selected_covered'],
                                gain_vs_mean=rc['gain_vs_mean'], gain_vs_bits_lo0=rc['gain_vs_bits_lo0'])
        out['pc1'].append(got)
        mism = 0; nfr = 0
        for x in tr + [b0]:
            L = x['frame'][1]
            logs = frame_logs_translate(p, A3, logu, L, t)
            J = J_exact(logs, n) if np.any(logs == n // 2) else J_from_counts(class_counts(logs))
            nfr += 1
            mism += int(not (len(logs) == x['cols'] and J == x['J'] and covered_exact(logs, n) == x['covered']))
        out['pc2'].append(dict(t=t, frames=nfr, mismatches=mism))
    os.remove(raw)
    out['PC1_pass'] = all(c['matches_published'] for c in out['pc1'] if c['t'] in (5, 6))
    out['PC2_pass'] = all(c['mismatches'] == 0 for c in out['pc2'])
    json.dump(out, open(os.path.join(wd, 'pc_replay.json'), 'w'), indent=1)
    print(json.dumps({k_: v for k_, v in out.items() if k_ in ('PC1_pass', 'PC2_pass', 'seconds_orig')}))

if __name__ == '__main__':
    main()
