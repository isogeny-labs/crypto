#!/usr/bin/env python3
"""Collect ecbench compare outputs (paired, w = 0, 0.1, 1) into results.json. Computes nothing itself."""
import json, subprocess
D = 'research/curve25519_autolab_20261104/siblings/stage1-eks'
PAIRS = [('tw2cj-lazy', 'tw2cj-lazy-sib8-p64'), ('tw2cj-lazy', 'tw2cj-lazy-sib8-p256'), ('tw2cj-lazy', 'tw2cj-cv0'),
         ('rho-neg', 'tw2cj-lazy'), ('rho-neg', 'tw2cj-lazy-sib8-p64'), ('rho-neg', 'tw2cj-lazy-sib8-p256'),
         ('rho-neg', 'tw2cj-cv0'), ('rho-neg', 'rho-neg-aa')]
out = {}
for k in (28, 32):
    for a, b in PAIRS:
        row = {}
        for w in ('0', '0.1', '1'):
            c = json.loads(subprocess.run(['./target/release/ecbench', 'compare', '--dir', f'{D}/sessions/k{k}', '--a', a, '--b', b,
                                           '--resamples', '4000', '--seed', '20261110', '--weight', w, '--save', '--json'],
                                          capture_output=True, text=True, check=True).stdout)
            o = c['ops']
            row[f'w={w}'] = {'ratio': o['ratio_b_over_a'], 'ci95': o['ci95'], 'pairs': o['pairs']}
            row['band_order'] = o['band']['order']; row['wall_end_to_end'] = c.get('wall', {}).get('end_to_end_ratio', None)
        out[f'k{k}: {b} / {a}'] = row
json.dump(out, open(f'{D}/results.json', 'w'), indent=1)
for name, r in out.items():
    print(f"{name:46s} " + "  ".join(f"{w}:{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('w=0', 'w=0.1', 'w=1')) + f"  | {r['band_order']}")
