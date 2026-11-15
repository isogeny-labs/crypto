#!/usr/bin/env python3
"""Paired H1 runs for the Montgomery-residual question (unvalidated, pre-gate).
Adapted from ../run_h1default.py: same solver command and arms (off / --project-cofactor), no env flags;
differences: 4 seeds, a 4-worker pool (trial counts are deterministic given the seed, so contention does
not matter; wall time is not used), per-worker temp input files.
Usage: run_pairs.py INST_DIR[,INST_DIR...] OUT.jsonl [IC_BINARY]"""
import json, os, subprocess, sys, glob, tempfile
from multiprocessing import Pool
IC = sys.argv[3] if len(sys.argv) > 3 else "/home/user/crypto/target/release/ic"
SEEDS = tuple(int(x) for x in os.environ.get("RP_SEEDS", "1,2,3,4").split(","))   # RP_SEEDS overrides

def work(job):
    f, s, arm = job
    d = json.load(open(f))
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, dir=os.environ.get("TMPW")) as t:
        json.dump({k: int(v) for k, v in dict(p=d["field"]["modulus"], a=d["a"], b=d["b"], n=d["subgroup_order"],
                   gx=d["generator"]["x"], gy=d["generator"]["y"], qx=d["point"]["x"], qy=d["point"]["y"]).items()}, t)
        path = t.name
    env = dict(os.environ)
    for k in ("IC_SUM_TABLE", "IC_SHAMIR", "IC_HOIST_SWEEP", "IC_FAST_SWEEP", "IC_WALK"):
        env.pop(k, None)
    cmd = [IC, "--json", "solve-prime-file", "--input", path, "--seed", str(s), "--rho-steps", "0",
           "--max-trials-per-relation", "2000000", "--max-relation-attempts", "512"]
    if arm == "on":
        cmd.append("--project-cofactor")
    r = subprocess.run(cmd, capture_output=True, text=True, env=env)
    os.unlink(path)
    try:
        o = json.loads(r.stdout)
    except Exception:
        o = {"status": "crash"}
    st = (o.get("ic") or {}).get("stages", {})
    return dict(name=d["name"], family=d["family"], r_bits=d["r_bits"], p=d["field"]["modulus"], cofactor=d["cofactor"], seed=s, arm=arm,
                status=o.get("status"), ok=(o.get("ic") or {}).get("scalar") == d["fixture"]["known_log"],
                trials=st.get("trials_total"), rels=st.get("relations_collected"), fb=st.get("factor_base_size"))

if __name__ == "__main__":
    files = []
    for dd in sys.argv[1].split(","):
        files += [f for f in sorted(glob.glob(dd + "/*.json")) if not f.endswith("MANIFEST.json")]
    jobs = [(f, s, a) for f in files for s in SEEDS for a in ("off", "on")]
    with Pool(4) as pool:
        rows = pool.map(work, jobs, chunksize=16)
    with open(sys.argv[2], "w") as fh:
        for r in rows:
            fh.write(json.dumps(r) + "\n")
    print(len(rows), "rows;", sum(r["ok"] for r in rows), "recovered")
