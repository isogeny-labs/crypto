#!/usr/bin/env python3
"""VALIDATOR cross-check that the v5 harness leaves I, M, S, H, L (and ok) unchanged for every pre-existing arm and
that ic5w, ic5rw and rhodpu are the v4 validator's patched arms: replay a nonce-drawn sample of v4 rows
(frozen v4 runs.json: ic5, ic5ks, rho2xks, rhodp, bsgs2) and of the v4 validator's sensitivity runs
(sens_ic_runs.json: ic5w/ic5rw at mu = 1.0; sens_dpu_runs.json: rhodpu) with the v5 binary on the v4 instances.
For rhodp/rhodpu also checks N == 2 x the v4 validator's instrumented canonicalisation count where available, and
for rho2xks N == its step-index count ('pidx' in v4 native_work.json).
usage: v4cross.py <v5 binary> <repo> <out.json> <nonce> [per-group]"""
import json, sys, os, subprocess, random, collections, hashlib
from concurrent.futures import ThreadPoolExecutor
BIN, REPO, OUT, NONCE = sys.argv[1:5]; K = int(sys.argv[5]) if len(sys.argv) > 5 else 2
V4 = os.path.join(REPO, "research/mt_fieldops_v4_20261112")
rnd = random.Random(NONCE + ":v4cross")
def load(path):
    d = json.load(open(path)); return [dict(zip(d["fields"], x)) for x in d["rows"]]
nw = {}
d = json.load(open(os.path.join(V4, "validation/native_work.json")))
for x in d["rows"]:
    r = dict(zip(d["fields"], x)); nw[(r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], r["doff"])] = r
groups = collections.defaultdict(list)
for r in load(os.path.join(V4, "results/holdout/runs.json")): groups[("frozen", r["arm"], r["pb"])].append(r)
for r in load(os.path.join(V4, "validation/sens_ic_runs.json")):
    if r["arm"] in ("ic5w", "ic5rw") and r["param"] == 1.0: groups[("sens", r["arm"], r["pb"])].append(r)
for r in load(os.path.join(V4, "validation/sens_dpu_runs.json")): groups[("sens", r["arm"], r["pb"])].append(r)
pick = []
for g in sorted(groups, key=str): pick += rnd.sample(groups[g], K)
def run(r):
    extra = ["4"] if r["arm"].startswith("ic5") else ([str(r["doff"])] if r["arm"] in ("rhodp", "rhodpu") else [])
    argv = [BIN, r["arm"], os.path.join(V4, "instances", f"W{r['pb']}-{r['idx']}.json"), str(r["T"]), str(r["seed"]), str(r["param"])] + extra
    p = subprocess.run(argv, capture_output=True, text=True, timeout=7200)
    o = json.loads(p.stdout.strip().splitlines()[-1])
    bad = [f for f in ("I", "M", "S", "H", "L", "ok", "targets") if o[f] != r[f]]
    n = nw.get((r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], r.get("doff")))
    nchk = None
    if n is not None and r["arm"] == "rhodp": nchk = o["N"] == 2 * n["canon"]
    if n is not None and r["arm"] == "rho2xks": nchk = o["N"] == n["pidx"]
    return dict(arm=r["arm"], pb=r["pb"], T=r["T"], idx=r["idx"], seed=r["seed"], param=r["param"], doff=r.get("doff"),
                match_IMSHL_ok=not bad, bad=bad, N=o["N"], N_check_vs_v4_instrumented=nchk)
with ThreadPoolExecutor(4) as ex: res = list(ex.map(run, pick))
summ = dict(binary_sha256=hashlib.sha256(open(BIN, "rb").read()).hexdigest(), nonce=NONCE, rows=len(res),
            mismatching=sum(not x["match_IMSHL_ok"] for x in res), per_arm=dict(collections.Counter(x["arm"] for x in res)),
            N_checks=dict(collections.Counter(str(x["N_check_vs_v4_instrumented"]) for x in res)))
json.dump(dict(summary=summ, results=res), open(OUT, "w"), indent=1)
print(json.dumps(summ))
