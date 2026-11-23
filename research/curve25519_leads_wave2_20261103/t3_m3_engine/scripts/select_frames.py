#!/usr/bin/env python3
"""T3 frame selection (PROTOCOL.md, step S). Reads the regenerated, hash-checked
census data/census/k20.json and applies the torsion study's frozen selection rules
(research/curve25519_torsion_leads_20261103/scripts/analyse.py) verbatim:

  L3*(t): over the first 300 frames (rows carrying m3_cov), argmax J3,
          ties -> more columns, then lowest L.
  L2*(t): over all 2000 frames, argmax J[0], ties -> more columns, then lowest L.

Then checks the selections against the published census summary. Any mismatch
exits non-zero, and run.sh stops before any engine cell.
usage: select_frames.py <census k20.json> <torsion analysis/summary.json> <out selection.json>
"""
import json, sys

census, summary, out = sys.argv[1:4]
d = json.load(open(census))
s = {(c['k'], c['t']): c for c in json.load(open(summary))['cells']}
assert d['k'] == 20 and d['p'] == 1048573 and d['A'] == 926 and d['seed'] == 20261124, 'wrong census'
sel = dict(k=20, p=d['p'], A=d['A'], r=d['r'], census_seed=d['seed'], cells={}, checks={})
EXPECT_L2 = {4: (797820, 78), 5: (233902, 184)}   # torsion README, Addendum A (engine-confirmed)
ok = True
for c in d['cells']:
    t, R = c['t'], c['rows']
    if t > 6:
        continue
    R3 = [x for x in R if 'm3_cov' in x]
    s3 = sorted(R3, key=lambda x: (-x['J3'], -x['cols'], x['L']))[0]
    s2 = sorted(R, key=lambda x: (-x['J'][0], -x['cols'], x['L']))[0]
    m3_mean = sum(x['m3_cov'] for x in R3) / len(R3)
    sel['cells'][str(t)] = dict(
        L3=s3['L'], L3_m3_cov=s3['m3_cov'], L3_J3=s3['J3'], L3_cols=s3['cols'], L3_m2_cov=s3['N'][0],
        L2=s2['L'], L2_m2_cov=s2['N'][0], L2_cols=s2['cols'], L2_in_first300=any(x['L'] == s2['L'] for x in R3),
        m3_mean_cov_300=m3_mean, n_frames_m3=len(R3), n_frames_m2=len(R))
    pub = s[(20, t)]
    chk = dict(L3_cov_equals_summary=s3['m3_cov'] == pub['m3_sel_cov'],
               m3_mean_equals_summary=abs(m3_mean - pub['m3_mean']) < 1e-9,
               L2_cov_equals_summary=s2['N'][0] == pub['cov_frame_only'])
    if t in EXPECT_L2:
        chk['L2_equals_published'] = (s2['L'], s2['N'][0]) == EXPECT_L2[t]
    sel['checks'][str(t)] = chk
    ok &= all(chk.values())
sel['all_checks_pass'] = bool(ok)
json.dump(sel, open(out, 'w'), indent=1)
print(json.dumps(sel['cells'], indent=1))
if not ok:
    print('SELECTION CHECK FAILED', json.dumps(sel['checks']), file=sys.stderr)
    sys.exit(4)
