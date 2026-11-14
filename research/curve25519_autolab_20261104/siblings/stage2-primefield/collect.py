#!/usr/bin/env python3
"""Collect ecbench compare outputs (paired, w = 0, 0.1, 1) per shape into results.json. Computes nothing itself."""
import json, subprocess
D = 'research/curve25519_autolab_20261104/siblings/stage2-primefield'
P = [('tw2cj-lazy', 'sib8-p64'), ('tw2cj-lazy', 'sib8-p256'), ('tw2cj-lazy', 'sib8-p512'), ('sib8-p256', 'sib8-p512'),
     ('rho-neg', 'tw2cj-lazy'), ('rho-neg', 'sib8-p256'), ('rho-neg', 'sib8-p512'), ('rho-neg', 'rho-neg-aa')]
PJ = [('rho-neg', 'rho-cubic'), ('rho-cubic', 'tw2cj-lazy'), ('rho-cubic', 'sib8-p256'), ('rho-cubic', 'sib8-p512')]
out = {}
for t in ('a3', 'gen', 'j0b7'):
    for a, b in P + (PJ if t == 'j0b7' else []):
        row = {}
        for w in ('0', '0.1', '1'):
            r = subprocess.run(['./target/release/ecbench', 'compare', '--dir', f'{D}/sessions/{t}', '--a', a, '--b', b,
                                '--resamples', '4000', '--seed', '20261111', '--weight', w, '--save', '--json'], capture_output=True, text=True, check=True)
            c = json.loads(r.stdout); o = c['ops']
            row[f'w={w}'] = {'ratio': o['ratio_b_over_a'], 'ci95': o['ci95'], 'pairs': o['pairs']}
            row['band_order'] = o['band']['order']
        out[f'{t}: {b} / {a}'] = row
json.dump(out, open(f'{D}/results.json', 'w'), indent=1)
for n, r in out.items():
    print(f"{n:34s} " + "  ".join(f"{w}:{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('w=0', 'w=0.1', 'w=1')) + f" | {r['band_order']}")
