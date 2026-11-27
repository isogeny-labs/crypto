#!/usr/bin/env python3
"""Descent-window sweep on the n = 41 v6 rung (rho off), ABAB-rotated, pinned.

usage: sweep.py IC OUTDIR ROUNDS
Every run is a fresh `ic workflow` (cold: its own directory) under
tools/isolated_bench.py on CPU 3 with RAYON_NUM_THREADS=1.  Arms differ only
in KIC_DESCENT_WINDOW (and, for arm m2, descent_summands = 2).
"""
import json, os, subprocess, sys, pathlib
IC, OUT, ROUNDS = sys.argv[1], pathlib.Path(sys.argv[2]), int(sys.argv[3])
HERE = pathlib.Path(__file__).resolve().parent.parent
CRYPTO = pathlib.Path('/home/user/crypto')
P3, P2 = HERE / 'params/icv1-f2m41-tm2308219-7f48b14a-w-norho.json', None
ARMS = [('unset', None, P3), ('w16', '16', P3), ('w32', '32', P3), ('w64', '64', P3), ('w128', '128', P3),
        ('w207', '207', P3), ('w414', '414', P3), ('w828', '828', P3), ('d3', 'd3', P3), ('full', '0', P3),
]  # m2 (descent_summands = 2) dropped after the smoke round: 2 of 32 targets exhausted max_trials
OUT.mkdir(parents=True, exist_ok=True)
rows = open(OUT / 'rows.jsonl', 'a')
for r in range(ROUNDS):
    order = ARMS[r % len(ARMS):] + ARMS[:r % len(ARMS)]
    for name, w, params in order:
        d = OUT / f'r{r}' / name
        d.mkdir(parents=True, exist_ok=True)
        env = dict(os.environ, RAYON_NUM_THREADS='1')
        env.pop('KIC_DESCENT_WINDOW', None)
        if w is not None:
            env['KIC_DESCENT_WINDOW'] = w
        cmd = ['python3', str(CRYPTO / 'tools/isolated_bench.py'), 'run', '--wait', '--cpus', '3',
               '--out', str(OUT / 'isolation.jsonl'), '--label', f'{name}-r{r}', '--settle', '1',
               '--', IC, 'workflow', '--params', str(params), '--dir', str(d / 'run'),
               '--out', str(d / 'report.json'), '--json']
        p = subprocess.run(cmd, env=env, capture_output=True, text=True, cwd=CRYPTO)
        if p.returncode != 0:
            print(name, r, 'FAILED', p.stderr[-800:], file=sys.stderr)
            continue
        rep = json.load(open(d / 'report.json'))
        items = rep['solutions']['items']
        row = {'round': r, 'arm': name, 'window_env': w, 'params': params.name,
               'status': rep['status'], 'verified': sum(1 for i in items if i.get('verified')),
               'targets': len(items),
               'descent_trials': sum(i['descent_trials'] for i in items),
               'descent_seconds_total': sum(i['elapsed_seconds'] for i in items),
               'solve_stage_seconds': rep['state']['solve']['elapsed_seconds'],
               'stages': {k: rep['state'][k]['elapsed_seconds'] for k in ('select', 'collect', 'logs', 'solve')},
               'relations': rep['state'].get('relations_collected'),
               'base_points': rep['factor_base']['points'],
               'per_target_trials': [i['descent_trials'] for i in items],
               'per_target_seconds': [i['elapsed_seconds'] for i in items],
               'recovered_digest': __import__('hashlib').sha256(json.dumps([i['recovered'] for i in items]).encode()).hexdigest()[:16]}
        rows.write(json.dumps(row) + '\n'); rows.flush()
        print(f"r{r} {name:6s} trials {row['descent_trials']:6d} descent {row['descent_seconds_total']*1e3:8.2f} ms  verified {row['verified']}/{row['targets']}", flush=True)
