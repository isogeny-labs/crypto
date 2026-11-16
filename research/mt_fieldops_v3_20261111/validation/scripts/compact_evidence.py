#!/usr/bin/env python3
"""Write compact validator evidence (< 1 MiB .json, no .jsonl) into a study's validation/ directory:
 ks_runs.json             the validator's retention-arm runs (rho2xks, ic5ks): config, counters, verify-phase vector
 rho_instrumentation.json per rho-family row of the producer's rows: restart cost vectors (phase 1, 2), restart counts,
                          phase-2 lookups and repeated lookups (from the instrumented replay; counters unchanged)
usage: compact_evidence.py <rows.jsonl> <dbg replay.jsonl> <ks rows.jsonl> <out dir>"""
import json, sys, hashlib
ROWS, DBG, KS, OUT = sys.argv[1:5]
rows = [json.loads(l) for l in open(ROWS)]; dbg = {r["i"]: r for r in map(json.loads, open(DBG))}
ks = [json.loads(l) for l in open(KS)]
def ver(r):
    k = list(r["phases"]); return [b - a for a, b in zip(r["phases"][k[-2]], r["phases"][k[-1]])]
kf = ["arm", "pb", "T", "idx", "seed", "param", "ok", "targets", "I", "M", "S", "H", "L", "verify_IMSHL", "db", "db_steps", "target_steps"]
kr = [[r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], r["ok"], r["targets"], r["I"], r["M"], r["S"], r["H"], r["L"], ver(r),
       r.get("db"), r.get("db_steps"), r.get("target_steps")] for r in ks]
src_sha = hashlib.sha256(open(KS, "rb").read()).hexdigest()
json.dump(dict(schema="validator.ks-runs/v1", note="validator sensitivity arms (scratch build of patch_ks.py on the frozen source); not producer evidence", source_sha256=src_sha, fields=kf, rows=kr),
          open(f"{OUT}/ks_runs.json", "w"), separators=(",", ":"))
RHO = {"rho", "rho2", "rho2x", "rhoseq"}
f2 = ["row", "arm", "pb", "T", "idx", "seed", "param", "restart_p1_IMSHL", "n_restart_p1", "restart_p2_IMSHL", "n_restart_p2", "lookups_p2", "repeat_lookups_p2"]
ir = []
for i, r in enumerate(rows):
    if r["arm"] not in RHO: continue
    d = dbg[i]["instr"]
    ir.append([i, r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], d["restart_p1"], d["n_restart_p1"], d["restart_p2"], d["n_restart_p2"], d["lookups_p2"], d["repeat_lookups_p2"]])
json.dump(dict(schema="validator.rho-instrumentation/v1", rows_sha256=hashlib.sha256(open(ROWS, "rb").read()).hexdigest(), fields=f2, rows=ir),
          open(f"{OUT}/rho_instrumentation.json", "w"), separators=(",", ":"))
print("ok", len(kr), len(ir))
