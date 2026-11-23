#!/usr/bin/env python3
"""Interleaved wall-time A/B of ecbench exec jobs.  Reads each job once per round, runs the arms
in alternating order, records report.solve_wall_ns and report.total_gae (must be identical across
variants).  usage: ab_wall.py <rounds> [ENV=VAL ...]   (env applies to variant 'B'; variant 'A' is the plain env)"""
import json, os, subprocess, sys, glob, statistics as st, time
ECB = '/home/user/crypto/target/release/ecbench'
HERE = os.path.dirname(os.path.abspath(__file__))
rounds = int(sys.argv[1]); extra = dict(a.split('=', 1) for a in sys.argv[2:])
jobs = {arm: sorted(glob.glob(f'{HERE}/{arm}/*.json'), key=lambda p: int(os.path.basename(p)[:-5])) for arm in ('sel', 'inc')}
def run(path, env):
    t = open(path).read()
    o = subprocess.run([ECB, 'exec'], input=t, capture_output=True, text=True, env=env, check=True).stdout
    r = json.loads(o)['report']
    return r['solve_wall_ns'], r['total_gae']
envA = dict(os.environ); envB = dict(os.environ); envB.update(extra)
data = {(v, a): [] for v in 'AB' for a in ('sel', 'inc')}; gae = {}
n = min(len(jobs['sel']), len(jobs['inc']))
for rd in range(rounds):
    for i in range(n):
        order = [('A', 'sel'), ('A', 'inc'), ('B', 'sel'), ('B', 'inc')]
        if (rd + i) % 2: order = order[::-1]
        for v, a in order:
            w, g = run(jobs[a][i], envA if v == 'A' else envB)
            data[(v, a)].append(w); gae.setdefault((a, i), set()).add(g)
bad = [k for k, s in gae.items() if len(s) > 1]
print('gae identical across variants and repeats:', not bad, f'({len(gae)} jobs)')
for v in 'AB':
    ms, mi = st.mean(data[(v, 'sel')]), st.mean(data[(v, 'inc')])
    print(f'variant {v}: mean solve_wall_ns sel {ms:.0f} inc {mi:.0f} ratio sel/inc {ms/mi:.4f}   medians {st.median(data[(v,"sel")]):.0f} {st.median(data[(v,"inc")]):.0f}')
for a in ('sel', 'inc'):
    ma, mb = st.mean(data[('A', a)]), st.mean(data[('B', a)])
    print(f'{a}: B/A {mb/ma:.4f}')
