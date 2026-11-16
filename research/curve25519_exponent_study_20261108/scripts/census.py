#!/usr/bin/env python3
"""Exact census (PROTOCOL.md section 4) for every attempted cell: |V|, columns,
base points, m-sum coverage of the order-r subgroup. Output data/census.json."""
import glob, json, os, subprocess
BIN = os.environ.get("BIN", "/home/user/ctarget-f1/release/curve25519_family")
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
keys = set()
for f in glob.glob(os.path.join(D, "cells", "*.json")):
    j = json.load(open(f))
    keys.add((j["k"], j["d_or_t"], j["gidx"]))
out = []
for k, d, g in sorted(keys):
    mmax = 4 if d <= 63 else 3
    if d > 513:
        mmax = 2
    r = subprocess.run([BIN, "subcensus", str(k), "sub", str(d), str(g), "mont", str(mmax)],
                       capture_output=True, text=True, check=True)
    j = json.loads(r.stdout.strip().splitlines()[-1])
    out.append(j)
    print(k, d, g, j["v_size"], j["columns"], j["base_points"], j["covered_points"], flush=True)
json.dump(out, open(os.path.join(D, "data", "census.json"), "w"), indent=1)
