#!/usr/bin/env python3
"""Validator sensitivity run (scratch): rho2xks (Kuhn-Struik retention, patch_ks.py) at kappa 0.05/0.1/0.25/0.5
on every configuration (pb, T, idx, seed) found in a rows file, with the same instance (frozen copy) and seed.
usage: run_ks_cfg.py <ks binary> <rows.jsonl> <frozen instances dir> <out.jsonl> [jobs]"""
import json, subprocess, sys, os
from concurrent.futures import ThreadPoolExecutor
BIN, ROWS, INST, OUT = sys.argv[1:5]; JOBS = int(sys.argv[5]) if len(sys.argv) > 5 else 4
cfg = {}
for l in open(ROWS):
    r = json.loads(l); cfg[(r["pb"], r["T"], r["idx"], r["seed"])] = os.path.join(INST, os.path.basename(r["argv"][1]))
jobs = [dict(pb=k[0], T=k[1], idx=k[2], seed=k[3], arm="rho2xks", param=kp, argv=["rho2xks", f, str(k[1]), str(k[3]), str(kp)])
        for k, f in sorted(cfg.items(), key=lambda kv: -kv[0][0]) for kp in (0.05, 0.1, 0.25, 0.5)]
def run(j):
    p = subprocess.run([BIN] + j["argv"], capture_output=True, text=True, timeout=7200)
    if p.returncode != 0: return dict(j, error=p.stderr[-300:])
    o = json.loads(p.stdout.strip().splitlines()[-1]); o.update({k: v for k, v in j.items() if k != "argv"}); return o
with ThreadPoolExecutor(JOBS) as ex, open(OUT, "w") as fh:
    for o in ex.map(run, jobs): fh.write(json.dumps(o) + "\n"); fh.flush()
rows = [json.loads(l) for l in open(OUT)]
print(len(rows), "runs;", sum("error" in r or r["ok"] != r["targets"] for r in rows), "errors or unverified")
