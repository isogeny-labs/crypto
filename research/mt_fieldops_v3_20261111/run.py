#!/usr/bin/env python3
"""Runner for the v3 panel (see PROTOCOL.md).  usage: run.py tune|holdout  -> results/<phase>/rows.jsonl"""
import json, subprocess, sys, os, hashlib, itertools
from concurrent.futures import ThreadPoolExecutor
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "../../target/release/examples/mt_fieldops")
INST = os.path.join(HERE, "instances")
FZ = os.path.join(HERE, "frozen_ic5.json")
RHO_K = [0.5, 0.7, 1.0]; RHO_ARMS = ["rho", "rho2", "rho2x", "rhoseq"]; BSGS_MF = [0.7, 1.0, 1.4]
IC4 = (1.0, 2.0)   # frozen in research/mt_fieldops_20261107/frozen.json, unchanged
def jobs(phase):
    if phase == "tune":
        cells = [(pb, T) for pb in (22, 24, 28, 32) for T in (1024, 16384)]; idxs = [0]; seeds = [101, 102]
        ic5 = list(itertools.product([0.7, 0.85, 1.0, 1.2, 1.4], [3, 4]))
    else:
        cells = [(pb, T) for pb in (22, 24, 28, 32) for T in (1024, 4096, 16384)]; idxs = range(1, 12); seeds = [21, 22]
        fz = json.load(open(FZ)); ic5 = [(fz["mu"], fz["k"])]
    for (pb, T), idx, seed in itertools.product(cells, idxs, seeds):
        f = f"{INST}/W{pb}-{idx}.json"; base = dict(f=f, pb=pb, idx=idx, T=T, seed=seed)
        for mu, k in ic5: yield dict(base, arm="ic5", param=mu, k=k, extra=[str(k)])
        if phase == "tune": continue
        yield dict(base, arm="ic4", param=IC4[0], extra=[str(IC4[1])])
        for kp in RHO_K:
            for arm in RHO_ARMS: yield dict(base, arm=arm, param=kp, extra=[])
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
