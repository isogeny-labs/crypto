#!/usr/bin/env python3
"""VALIDATOR SENSITIVITY PANEL (not pre-registered): SENS_ARM (default rho2xks) at SENS_KAPPAS (default 0.75, 1.0: above the frozen grid) on all 198
holdout configurations, same binary semantics (pristine build of the frozen harness), same argv shape as run.py.
usage: run_sens.py <binary> <frozen study dir> <out.json>"""
import json, sys, os, subprocess, itertools, hashlib
from concurrent.futures import ThreadPoolExecutor
BIN, FZ, OUT = sys.argv[1:4]
KAPPAS = [float(x) for x in os.environ.get("SENS_KAPPAS", "0.75,1.0").split(",")]
jobs = []
for (pb, T), idx, seed in itertools.product([(pb, T) for pb in (24, 28, 32) for T in (1024, 4096, 16384)], range(11), (41, 42)):
    for kp in KAPPAS:
        jobs.append(dict(arm=os.environ.get("SENS_ARM", "rho2xks"), pb=pb, T=T, idx=idx, seed=seed, param=kp))
def run(j):
    argv = [BIN, j["arm"], os.path.join(FZ, "instances", f"W{j['pb']}-{j['idx']}.json"), str(j["T"]), str(j["seed"]), str(j["param"])]
    p = subprocess.run(argv, capture_output=True, text=True, timeout=7200)
    if p.returncode != 0: return dict(j, error=p.stderr[-300:])
    o = json.loads(p.stdout.strip().splitlines()[-1]); o.update(j); return o
with ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
    res = list(ex.map(run, sorted(jobs, key=lambda j: -j["pb"] * j["T"])))
binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
json.dump(dict(binary_sha256=binsha, kappas=KAPPAS, rows=res), open(OUT, "w"))
print(len(res), "runs;", sum("error" in r or r["ok"] != r["targets"] for r in res), "errors/unverified")
