#!/usr/bin/env python3
"""Validator sensitivity run (scratch; not producer evidence): the Kuhn-Struik retention arms rho2xks and ic5ks
(patch_ks.py) on every holdout configuration of Study A (same instances, T, idx, seeds as the frozen job list).
rho2xks kappa in {0.05, 0.1, 0.25, 0.5}; ic5ks mu in {0.85, 1.0}, k = 4.
usage: run_ks.py <ks binary> <frozen v3 dir> <out.jsonl> [jobs]"""
import json, subprocess, sys, os, itertools
from concurrent.futures import ThreadPoolExecutor
BIN, D, OUT = sys.argv[1:4]; JOBS = int(sys.argv[4]) if len(sys.argv) > 4 else 4
jobs = []
for (pb, T), idx, seed in itertools.product([(pb, T) for pb in (22, 24, 28, 32) for T in (1024, 4096, 16384)], range(1, 12), [21, 22]):
    f = f"{D}/instances/W{pb}-{idx}.json"; base = dict(pb=pb, T=T, idx=idx, seed=seed)
    for kp in (0.05, 0.1, 0.25, 0.5): jobs.append(dict(base, arm="rho2xks", param=kp, argv=["rho2xks", f, str(T), str(seed), str(kp)]))
    for mu in (0.85, 1.0): jobs.append(dict(base, arm="ic5ks", param=mu, argv=["ic5ks", f, str(T), str(seed), str(mu), "4"]))
jobs.sort(key=lambda j: -j["pb"])
def run(j):
    p = subprocess.run([BIN] + j["argv"], capture_output=True, text=True, timeout=7200)
    if p.returncode != 0: return dict(j, error=p.stderr[-300:])
    o = json.loads(p.stdout.strip().splitlines()[-1]); o.update({k: v for k, v in j.items() if k != "argv"}); return o
with ThreadPoolExecutor(JOBS) as ex, open(OUT, "w") as fh:
    for o in ex.map(run, jobs): fh.write(json.dumps(o) + "\n"); fh.flush()
rows = [json.loads(l) for l in open(OUT)]
print(len(rows), "runs;", sum("error" in r or r["ok"] != r["targets"] for r in rows), "errors or unverified")
