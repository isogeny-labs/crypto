#!/usr/bin/env python3
"""Runner for the v4 panel (see PROTOCOL.md).  usage: run.py holdout  -> results/holdout/rows.jsonl"""
import json, subprocess, sys, os, hashlib, itertools
from concurrent.futures import ThreadPoolExecutor
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "../../target/release/examples/mt_fieldops")
INST = os.path.join(HERE, "instances")
BSGS_MF = [0.7, 1.0, 1.4]
DP_K = [0.0, 0.25, 0.5]; DP_OFF = [-1, 0, 1, 2]; KS_K = [0.5, 0.7, 1.0]
IC5 = (1.0, 4)     # frozen in ../mt_fieldops_v3_20261111/frozen_ic5.json, unchanged
def jobs(phase):
    cells = [(pb, T) for pb in (24, 28, 32) for T in (1024, 4096, 16384)]
    idxs = range(0, 11); seeds = [31, 32]
    for (pb, T), idx, seed in itertools.product(cells, idxs, seeds):
        f = f"{INST}/W{pb}-{idx}.json"; base = dict(f=f, pb=pb, idx=idx, T=T, seed=seed)
        yield dict(base, arm="ic5", param=IC5[0], k=IC5[1], extra=[str(IC5[1])])
        yield dict(base, arm="ic5ks", param=IC5[0], k=IC5[1], extra=[str(IC5[1])])
        for kp in KS_K: yield dict(base, arm="rho2xks", param=kp, extra=[])
        for kp in DP_K:
            for off in DP_OFF: yield dict(base, arm="rhodp", param=kp, doff=off, extra=[str(off)])
        for mf in BSGS_MF: yield dict(base, arm="bsgs2", param=mf, extra=[])
def run(j):
    argv = [BIN, j["arm"], j["f"], str(j["T"]), str(j["seed"]), str(j["param"])] + j["extra"]
    r = subprocess.run(argv, capture_output=True, text=True, timeout=7200)
    meta = {k: v for k, v in j.items() if k not in ("f", "extra")}
    if r.returncode != 0:
        return dict(meta, error=r.stderr[-400:], argv=argv[1:])
    o = json.loads(r.stdout.strip().splitlines()[-1]); o.update(meta); o["argv"] = argv[1:]
    return o
if __name__ == "__main__":
    phase = sys.argv[1]
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
    commit = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, cwd=HERE).stdout.strip()
    js = sorted(jobs(phase), key=lambda j: -j["pb"])  # big runs first
    n_err = 0
    with open(os.path.join(out, "rows.jsonl"), "w") as fh, ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
        for o in ex.map(run, js):
            o.update(bin_sha256=binsha, commit=commit); fh.write(json.dumps(o) + "\n"); fh.flush()
            n_err += "error" in o or o.get("ok") != o.get("targets")
    print(phase, len(js), "runs;", n_err, "errors or unverified")
