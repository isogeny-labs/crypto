#!/usr/bin/env python3
"""Driver for the native 2+1 linear-leaf CDCL pilot. Standard library only.

  python3 run.py                       # measured cells (seed 20261116), writes raw/ and results.json
  python3 run.py --cells 5 --out DIR   # smallest cell only, into DIR (rebuild-and-rerun check)

Each (target, arm) is one process of the standalone binary, at most 2 at a time, killed after
CAP_S seconds (only PIDs started here). Scratch build under $LIN21_SCRATCH.
"""
import argparse, concurrent.futures as cf, hashlib, json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
SCRATCH = os.environ.get(
    "LIN21_SCRATCH",
    "/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/P1")
SEED = 20261116
CAP_S = 600
WORKERS = 2
N_UNIFORM, N_PLANTED = 16, 4

# arm -> rule selecting targets in a cell. "all" = every target; ("uniform", k) = first k
# non-decomposable uniform targets plus all planted targets.
PLAN = {
    5: {"scan": "all", "lin": "all", "lin-nolex": "all", "lin21": "all", "lin21-pu": "all"},
    6: {"scan": "all", "lin": "all", "lin-nolex": "all", "lin21": "all", "lin21-pu": "all"},
    7: {"scan": "all", "lin": "all", "lin-nolex": "all", "lin21": "all", "lin21-pu": "all"},
    8: {"scan": "all", "lin": "all", "lin-nolex": ("uniform", 4), "lin21": "all", "lin21-pu": "all"},
    9: {"scan": "all", "lin": ("uniform", 8), "lin21": "all", "lin21-pu": "all"},
    10: {"scan": "all", "lin21": "all", "lin21-pu": "all"},
    11: {"scan": "all", "lin21": "all", "lin21-pu": "all"},
    12: {"scan": "all", "lin21": "all"},
}


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def build():
    os.makedirs(SCRATCH, exist_ok=True)
    exe = os.path.join(SCRATCH, "lin21")
    subprocess.run(["rustc", "-O", "-o", exe, os.path.join(HERE, "lin21.rs")], check=True,
                   stderr=subprocess.DEVNULL)
    return exe


def run_one(exe, tgt, arm):
    cmd = [exe, "run", "--l", str(tgt["l"]), "--n", str(tgt["n"]), "--xr", str(tgt["x_r"]), "--arm", arm]
    t0 = time.time()
    row = {"l": tgt["l"], "n": tgt["n"], "idx": tgt["idx"], "kind": tgt["kind"], "x_r": tgt["x_r"],
           "seed": tgt["seed"], "arm": arm, "oracle_sat": tgt["oracle_sat"]}
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=CAP_S)
        if p.returncode != 0:
            row.update(status="error", stderr=p.stderr[-400:])
        else:
            row.update(json.loads(p.stdout))
    except subprocess.TimeoutExpired:
        row.update(status="censored", cap_s=CAP_S)
    row["process_s"] = round(time.time() - t0, 3)
    st = row["status"]
    row["agree"] = (st == "sat") == tgt["oracle_sat"] if st in ("sat", "unsat") else None
    if st == "sat":
        row["verified"] = bool(row["in_base"] and row["liftable"] and row["sums_to_target"] and row["chain_ok"])
    else:
        row["verified"] = None
    return row


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cells", default=",".join(str(l) for l in PLAN))
    ap.add_argument("--out", default=HERE)
    ap.add_argument("--seed", type=int, default=SEED)
    ap.add_argument("--skip-selftest", action="store_true")
    a = ap.parse_args()
    cells = [int(x) for x in a.cells.split(",")]
    raw = os.path.join(a.out, "raw")
    os.makedirs(raw, exist_ok=True)
    exe = build()
    meta = {"seed": a.seed, "cap_s": CAP_S, "workers": WORKERS, "cells": cells,
            "sha256": {"lin21.rs": sha(os.path.join(HERE, "lin21.rs")),
                       "pdp_lin_cdcl.rs": sha(os.path.join(HERE, "../../examples/pdp_lin_cdcl.rs")),
                       "run.py": sha(os.path.abspath(__file__))},
            "rustc": subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip(),
            "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    selftest = None
    if not a.skip_selftest:
        p = subprocess.run([exe, "selftest"], capture_output=True, text=True, timeout=CAP_S)
        selftest = {"returncode": p.returncode, "cells": [json.loads(x) for x in p.stdout.splitlines()]}
        json.dump(selftest, open(os.path.join(raw, "selftest.json"), "w"), indent=1)
        if p.returncode != 0:
            sys.exit("selftest failed")
    rows, targets_all = [], []
    rows_path = os.path.join(raw, "rows.jsonl")
    open(rows_path, "w").close()
    for l in cells:
        p = subprocess.run([exe, "targets", "--l", str(l), "--seed", str(a.seed), "--uniform", str(N_UNIFORM),
                            "--planted", str(N_PLANTED)], capture_output=True, text=True, check=True, timeout=CAP_S)
        tg = [json.loads(x) for x in p.stdout.splitlines()]
        open(os.path.join(raw, f"targets_l{l}.jsonl"), "w").write(p.stdout)
        targets_all += tg
        jobs = []
        for arm, rule in PLAN[l].items():
            if rule == "all":
                sel = tg
            else:
                uni = [t for t in tg if t["kind"] == "uniform"][: rule[1]]
                sel = uni + [t for t in tg if t["kind"] == "planted"]
            jobs += [(t, arm) for t in sel]
        # interleave arms target by target so no arm owns a time window
        jobs.sort(key=lambda j: (j[0]["idx"], j[1]))
        t0 = time.time()
        with cf.ThreadPoolExecutor(max_workers=WORKERS) as ex:
            for row in ex.map(lambda j: run_one(exe, j[0], j[1]), jobs):
                rows.append(row)
                with open(rows_path, "a") as f:
                    f.write(json.dumps(row) + "\n")
        bad = [r for r in rows if r["l"] == l and (r["agree"] is False or r["verified"] is False or r["status"] == "error")]
        cens = [r for r in rows if r["l"] == l and r["status"] == "censored"]
        print(f"l={l}: {len(jobs)} jobs in {time.time()-t0:.0f}s, disagreements/unverified/errors={len(bad)}, censored={len(cens)}",
              flush=True)
    meta["finished_utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    json.dump({"schema": "sat_native_lin21.pilot/v1", "meta": meta, "selftest": selftest,
               "targets": targets_all, "rows": rows},
              open(os.path.join(a.out, "results.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
