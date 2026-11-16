#!/usr/bin/env python3
"""Runner for the field-op-counted multi-target panel (see PROTOCOL.md).
usage: run.py tune|holdout|smoke   -> results/<phase>/rows.jsonl"""
import json, subprocess, sys, os, hashlib, itertools
from concurrent.futures import ThreadPoolExecutor
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "../../target/release/examples/mt_fieldops")
INST = os.path.join(HERE, "../ic_e222_m221_20261106/results/instances_vsrho")
FROZEN = json.load(open(os.path.join(HERE, "frozen.json"))) if os.path.exists(os.path.join(HERE, "frozen.json")) else None
RHO_K = [0.5, 0.7, 1.0]; BSGS_MF = [0.7, 1.0, 1.4]
def jobs(phase):
    if phase == "smoke":
        cells = [(20, 256)]; idxs = [0]; seeds = [7]
        ic = [(1.0, 1.5)]
    elif phase == "tune":
        cells = [(22, 1024), (24, 4096), (25, 16384)]; idxs = [0]; seeds = [101, 102, 103]
        ic = list(itertools.product([0.6, 0.8, 1.0, 1.2, 1.4], [1.0, 1.5, 2.0]))
    else:
        cells = [(22, 1024), (24, 1024), (24, 4096), (24, 16384), (25, 4096), (25, 16384)]
        idxs = range(1, 12); seeds = [1, 2]
        ic = [(FROZEN["mu"], FROZEN["g"])]
    for (pb, T), idx, seed in itertools.product(cells, idxs, seeds):
        f = f"{INST}/W{pb}-{idx}.json"
        for mu, g in ic: yield dict(arm="ic4", f=f, pb=pb, idx=idx, T=T, seed=seed, param=mu, g=g)
        if phase == "tune": continue
        for k in RHO_K:
            yield dict(arm="rho", f=f, pb=pb, idx=idx, T=T, seed=seed, param=k)
            yield dict(arm="rho2", f=f, pb=pb, idx=idx, T=T, seed=seed, param=k)
        for mf in BSGS_MF: yield dict(arm="bsgs2", f=f, pb=pb, idx=idx, T=T, seed=seed, param=mf)
def run(j):
    argv = [BIN, j["arm"], j["f"], str(j["T"]), str(j["seed"]), str(j["param"])] + ([str(j["g"])] if j["arm"] == "ic4" else [])
    r = subprocess.run(argv, capture_output=True, text=True, timeout=3600)
    if r.returncode != 0:
        return dict(j, error=r.stderr[-400:], argv=argv[1:])
    o = json.loads(r.stdout.strip().splitlines()[-1]); o.update({k: v for k, v in j.items() if k != "f"}); o["argv"] = argv[1:]
    return o
if __name__ == "__main__":
    phase = sys.argv[1]
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
    commit = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, cwd=HERE).stdout.strip()
    js = list(jobs(phase)); n_err = 0
    with open(os.path.join(out, "rows.jsonl"), "w") as fh, ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
        for o in ex.map(run, js):
            o.update(bin_sha256=binsha, commit=commit); fh.write(json.dumps(o) + "\n"); fh.flush()
            n_err += "error" in o or o.get("ok") != o.get("targets")
    print(phase, len(js), "runs;", n_err, "errors or unverified")
