#!/usr/bin/env python3
"""Interleaved A/B of two ecbench binaries on the same exec jobs.
usage: ab_bin.py <binA> <binB> <rounds>
Per job and round the order of A and B alternates.  Records report.solve_wall_ns; checks that every
deterministic field of the report (everything except timing, allocator and online clocks) is identical
across the two binaries and across repeats."""
import json, os, subprocess, sys, glob, statistics as st
A, B, rounds = sys.argv[1], sys.argv[2], int(sys.argv[3])
HERE = os.path.dirname(os.path.abspath(__file__))
jobs = {arm: sorted(glob.glob(f'{HERE}/{arm}/*.json'), key=lambda p: int(os.path.basename(p)[:-5])) for arm in ('sel', 'inc')}
TIMING = {'solve_wall_ns', 'online', 'online_error', 'solver'}
def run(binary, path):
    o = json.loads(subprocess.run([binary, 'exec'], input=open(path).read(), capture_output=True, text=True, check=True).stdout)
    r = o['report']
    det = {k: v for k, v in r.items() if k not in TIMING}
    # phases carry per-phase wall clocks in some builds: drop any *_ns key
    def strip(x):
        if isinstance(x, dict): return {k: strip(v) for k, v in x.items() if not k.endswith('_ns') and k != 'wall_ns'}
        if isinstance(x, list): return [strip(v) for v in x]
        return x
    return r['solve_wall_ns'], json.dumps(strip(det), sort_keys=True)
data = {(v, a): [] for v in 'AB' for a in ('sel', 'inc')}; seen = {}
n = min(len(jobs['sel']), len(jobs['inc']))
for rd in range(rounds):
    for i in range(n):
        order = [('A', 'sel'), ('A', 'inc'), ('B', 'sel'), ('B', 'inc')]
        if (rd + i) % 2: order = order[::-1]
        for v, a in order:
            w, d = run(A if v == 'A' else B, jobs[a][i])
            data[(v, a)].append(w); seen.setdefault((a, i), set()).add(d)
bad = [k for k, s in seen.items() if len(s) > 1]
print(f'deterministic report fields identical across both binaries and all repeats: {not bad}  ({len(seen)} jobs, {len(bad)} differing)')
for a in ('sel', 'inc'):
    ma, mb = st.mean(data[('A', a)]), st.mean(data[('B', a)])
    print(f'{a}: mean solve_wall_ns A {ma:.0f}  B {mb:.0f}  B/A {mb/ma:.4f}   median A {st.median(data[("A",a)]):.0f} B {st.median(data[("B",a)]):.0f}')
for v in 'AB':
    ms, mi = st.mean(data[(v, 'sel')]), st.mean(data[(v, 'inc')])
    print(f'binary {v}: sel/inc solve wall ratio {ms/mi:.4f}')
