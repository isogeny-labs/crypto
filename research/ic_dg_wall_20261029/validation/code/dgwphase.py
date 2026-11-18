#!/usr/bin/env python3
"""Per-phase solve-wall ratios dg/srenew from recompute.json mechanism sums (within-run null). Usage: dgwphase.py <recompute.json> <out.json>"""
import json, sys
d = json.load(open(sys.argv[1]))
out = {}
for b, R in d['sizes'].items():
    m = R['mechanism']; s = m['srenew']; g = m['dg']
    row = {}
    for ph in ('factor_base', 'oracle_setup', 'relations', 'linear_algebra', 'verify'):
        k = 'phase_wall_ns.' + ph
        row[ph] = {'srenew_ms': s[k] / 1e6, 'dg_ms': g[k] / 1e6, 'dg_over_srenew': g[k] / s[k], 'share_of_srenew_solve': s[k] / s['solve_wall_ns'], 'phase_gae_equal': s['phase_gae.' + ph] == g['phase_gae.' + ph]}
    row['solve'] = {'dg_over_srenew': g['solve_wall_ns'] / s['solve_wall_ns']}
    out[b] = row
json.dump(out, open(sys.argv[2], 'w'), indent=1)
