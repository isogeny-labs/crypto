#!/usr/bin/env python3
"""Stage 4 (aut=3): ecbench compare (paired, w = 0, 0.1, 1, --save) per spec and ecbench trend per shape
over the three sizes; writes results.json and trend.json. Computes nothing itself."""
import json, subprocess, glob, os
D = 'research/curve25519_autolab_20261104/siblings/stage4-aut3'
EB = './target/release/ecbench'
P = [('tw2cj-lazy', 'tw2cj-lazy-aut3'), ('sib8-p512', 'sib8-p512-aut3'), ('rho-cubic', 'sib8-p512-aut3'),
     ('rho-cubic', 'tw2cj-lazy-aut3'), ('rho-cubic', 'sib8-p512'), ('rho-neg', 'rho-cubic'), ('rho-neg', 'sib8-p512-aut3'),
     ('rho-neg', 'rho-neg-aa')]
PJ = []
res, trend = {}, {}
for t in ('j0b7',):
    for b in (34, 37, 40):
        d = f'{D}/sessions/{t}-{b}'
        for a, c in P + (PJ if t == 'j0b7' else []):
            row = {}
            for w in ('0', '0.1', '1'):
                o = json.loads(subprocess.run([EB, 'compare', '--dir', d, '--a', a, '--b', c, '--resamples', '4000',
                                               '--seed', '20261111', '--weight', w, '--save', '--json'],
                                              capture_output=True, text=True, check=True).stdout)['ops']
                row[f'w={w}'] = {'ratio': o['ratio_b_over_a'], 'ci95': o['ci95'], 'pairs': o['pairs']}
                row['band_order'] = o['band']['order']
            res[f'{t}-{b}: {c} / {a}'] = row
json.dump(res, open(f'{D}/results.json', 'w'), indent=1)
for n, r in res.items():
    print(f"{n:38s} " + "  ".join(f"{w}:{r[w]['ratio']:.3f}[{r[w]['ci95'][0]:.3f},{r[w]['ci95'][1]:.3f}]" for w in ('w=0', 'w=0.1', 'w=1')) + f" | {r['band_order']}")
