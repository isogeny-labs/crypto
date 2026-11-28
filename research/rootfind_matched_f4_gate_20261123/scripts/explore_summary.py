#!/usr/bin/env python3
"""Summarise data/explore_posthoc.jsonl (POST-HOC, not gate-scored) into data/explore_posthoc.csv."""
import json, statistics as st
from pathlib import Path
D = Path(__file__).resolve().parents[1] / 'data'
rows = [json.loads(l) for l in open(D / 'explore_posthoc.jsonl')]
out = ['M,chart,B_s_per_instance,F4visit_s_per_instance,ratio,B_complete,B_nodes,F4_complete,F4_n_roots,B_n_roots,root_sets_equal']
for M in sorted({r['M'] for r in rows}):
    for cid in ('s1', 's2', 's11'):
        b = [r for r in rows if r['M'] == M and r['id'] == cid and r['arm'] == 'B']
        f = [r for r in rows if r['M'] == M and r['id'] == cid and r['arm'] == 'F4-visit']
        tb = st.median(r['wall'] for r in b) / 5; tf = st.median(r['wall'] for r in f) / 5
        bc = all(r['complete'] for r in b); fc = all(r['complete'] for r in f)
        eq = bc and fc and len({r['roots_sha'] for r in b + f}) == 1
        out.append(f"{M},{cid},{tb:.5f},{tf:.5f},{tf/tb:.2f},{bc},{b[0].get('nodes')},{fc},{f[0]['n_roots']},{b[0]['n_roots']},{eq}")
(D / 'explore_posthoc.csv').write_text('\n'.join(out) + '\n')
print('\n'.join(out))
