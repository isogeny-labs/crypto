#!/usr/bin/env python3
"""logs/*.log -> results.tsv (via parse.sh) and per-(d',m) summary markdown on stdout."""
import subprocess, re, collections, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
out = subprocess.check_output(["bash", f"{HERE}/../../ic_tree_gluing_20261103/parse.sh", "-H", f"{HERE}/logs"], text=True)
rows = [l.split("\t") for l in out.strip().splitlines()]
hdr = rows[0]; data = {}
for r in rows[1:]:
    m = re.match(r"d(\d+)_m(\d)_s(\d)_(\w+)\.log", r[0])
    d = dict(zip(hdr, r))
    tail = open(f"{HERE}/logs/{r[0]}").read()
    mm = re.search(r"^exit=(-?\d+) wall=([0-9.]+)", tail, re.M)
    if mm: d["exit"], d["wall"] = mm[1], mm[2]
    d.update(dp=int(m[1]), m=int(m[2]), seed=int(m[3]), arm=m[4]); data[(d["dp"], d["m"], d["seed"], d["arm"])] = d
open(f"{HERE}/results.tsv", "w").write("dprime\tm\ttarget\tarm\tD\twidest\tsumcells\tqdim\texit\twall_s\n" +
    "".join(f"{d['dp']}\t{d['m']}\t{d['seed']}\t{d['arm']}\t{d['Dsel'] if d['exit'] == '0' else '>=' + d['Dsel']}\t{d['width'] if d['exit'] == '0' else '>=' + d['width']}\t{d['sumcells']}\t{d['quotient_dim']}\t{d['exit']}\t{d['wall']}\n"
            for k, d in sorted(data.items())))
print(open(f"{HERE}/results.tsv").read())
