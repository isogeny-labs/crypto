#!/usr/bin/env python3
"""Correction rerun (see PROTOCOL.md): every rho-family job of the v2 holdout with the fixed binary.
usage: run.py -> results/rows.jsonl"""
import json, subprocess, os, sys, hashlib
from concurrent.futures import ThreadPoolExecutor
HERE = os.path.dirname(os.path.abspath(__file__))
V2 = os.path.join(HERE, "../mt_fieldops_v2_20261111")
sys.path.insert(0, V2)
import run as v2run  # same job list, instances and argv as the v2 holdout
if __name__ == "__main__":
    js = [j for j in v2run.jobs("holdout") if j["arm"] in v2run.RHO_ARMS]
    out = os.path.join(HERE, "results"); os.makedirs(out, exist_ok=True)
    binsha = hashlib.sha256(open(v2run.BIN, "rb").read()).hexdigest()
    commit = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, cwd=HERE).stdout.strip()
    n_err = 0
    with open(os.path.join(out, "rows.jsonl"), "w") as fh, ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
        for o in ex.map(v2run.run, js):
            o.update(bin_sha256=binsha, commit=commit); fh.write(json.dumps(o) + "\n"); fh.flush()
            n_err += "error" in o or o.get("ok") != o.get("targets")
    print(len(js), "runs;", n_err, "errors or unverified")
