#!/usr/bin/env python3
"""icbest/rhobest without the shared verification phase (I = 10M).  usage: noverify.py rows.jsonl > noverify.txt"""
import json, sys, collections, statistics as st
rows = [json.loads(l) for l in open(sys.argv[1])]
def cost(o, ci, h):
    I, M, S, H, L = o['phases']['targets']
    return ((ci * I + M + S) / (ci + 3) + h * (H + L)) / o['T']
cfg = collections.defaultdict(lambda: collections.defaultdict(list))
for o in rows: cfg[(o['pb'], o['T'], o['idx'], o['seed'])][o['arm']].append(o)
print("Median per-configuration ratio icbest/rhobest without the shared verification phase (I = 10 M), h = 0 / 0.1 / 1")
print("| bits | T | icbest/rhobest |"); print("|---|---|---|")
for pb in (24, 28, 32):
    for T in (1024, 4096, 16384):
        ks = [k for k in cfg if k[:2] == (pb, T)]; v = []
        for h in (0, 0.1, 1):
            r = [min(cost(o, 10, h) for a in ('ic5', 'ic5ks') for o in cfg[k][a]) /
                 min(cost(o, 10, h) for a in ('rho2xks', 'rhodp') for o in cfg[k][a]) for k in ks]
            v.append('%.3f' % st.median(r))
        print(f"| {pb} | {T} | " + " / ".join(v) + " |")
