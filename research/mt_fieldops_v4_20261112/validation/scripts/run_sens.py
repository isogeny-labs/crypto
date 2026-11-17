#!/usr/bin/env python3
"""Validator sensitivity panel (not pre-registered; grid fixed before running): on all 198 holdout
configurations (33 curves x seeds 31, 32 x T 1024/4096/16384) run
  ic5rw mu in {1.0, 0.85, 0.7, 0.6, 0.5} (k = 4), ic5w mu = 1.0 (control), ic5ks mu in {0.85, 0.7}
with the binary built from validation/scripts/patch_ic5rw.py.  Every target must verify.
Second panel (SENS_GRID=rho): rho2xks at kappa in {0.05, 0.1, 0.25}, below the frozen grid {0.5, 0.7, 1.0}
(the v3 validator found kappa = 0.05 best for this arm), so the reference gets the same downward tuning
as the IC sensitivity arm.  Third panel (SENS_GRID=dpu): rhodpu (validation/scripts/patch_rhodpu.py:
rhodp with distinct step values) on rhodp's frozen grid kappa {0, 0.25, 0.5} x offset {-1, 0, 1, 2}.
usage: run_sens.py <binary> <frozen study dir> <out.json>"""
import json, sys, os, subprocess, hashlib, itertools
from concurrent.futures import ThreadPoolExecutor
BIN, study, out = sys.argv[1:4]
inst = os.path.join(study, "instances")
GRID = [("ic5rw", mu) for mu in (1.0, 0.85, 0.7, 0.6, 0.5)] + [("ic5w", 1.0), ("ic5ks", 0.85), ("ic5ks", 0.7)]
if os.environ.get("SENS_GRID") == "rho":   # second panel: the rho2xks kappa values below the frozen grid
    GRID = [("rho2xks", kp) for kp in (0.05, 0.1, 0.25)]
if os.environ.get("SENS_GRID") == "dpu":   # third panel: rhodpu (distinct steps) on rhodp's full frozen grid
    GRID = [("rhodpu", (kp, off)) for kp in (0.0, 0.25, 0.5) for off in (-1, 0, 1, 2)]
jobs = []
for pb, T, idx, seed in itertools.product((32, 28, 24), (1024, 4096, 16384), range(11), (31, 32)):
    for arm, mu in GRID:
        doff = None
        if isinstance(mu, tuple):
            mu, doff = mu
        tail = ["4"] if arm.startswith("ic5") else [str(doff)] if doff is not None else []
        jobs.append(dict(arm=arm, pb=pb, T=T, idx=idx, seed=seed, param=mu, k=4 if arm.startswith("ic5") else None, doff=doff,
                         argv=[arm, os.path.join(inst, f"W{pb}-{idx}.json"), str(T), str(seed), str(mu)] + tail))
def run(j):
    p = subprocess.run([BIN] + j["argv"], capture_output=True, text=True)
    if p.returncode != 0:
        return dict(j, error=p.stderr[-300:])
    o = json.loads(p.stdout.strip().splitlines()[-1])
    return dict(j, n=o["n"], targets=o["targets"], ok=o["ok"], I=o["I"], M=o["M"], S=o["S"], H=o["H"], L=o["L"],
                d=o.get("d"), walks=o.get("walks"), abandoned=o.get("abandoned"), pre_steps=o.get("pre_steps"),
                phases=o["phases"], trials=o.get("trials"), m=o.get("m"), table=o.get("table"), w_len=o.get("w_len"))
with ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
    res = list(ex.map(run, jobs))
for r in res: r["argv"] = [r["argv"][0], os.path.basename(r["argv"][1])] + r["argv"][2:]
meta = dict(binary_sha256=hashlib.sha256(open(BIN, "rb").read()).hexdigest(), runs=len(res),
            errors=sum("error" in r for r in res), unverified=sum(r.get("ok") != r.get("targets") for r in res))
json.dump(dict(meta=meta, rows=res), open(out, "w"))
print(json.dumps(meta))
