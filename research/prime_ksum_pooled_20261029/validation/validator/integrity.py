#!/usr/bin/env python3
"""Validator integrity checks on recovered sessions (stdlib only)."""
import json, sys, datetime
from collections import Counter, defaultdict
from pathlib import Path
SESS = Path(sys.argv[1]); STUDY = Path(sys.argv[2]); OUT = Path(sys.argv[3])
def iso(ms): return datetime.datetime.fromtimestamp(ms / 1000, datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
reg = (STUDY / "registered-at.txt").read_text().strip()
frz = (STUDY / "select/frozen-at.txt").read_text().strip()
BIN = (STUDY / "BINARY.sha256").read_text().split()[0]
res = {"registered_at": reg, "frozen_at": frz, "binary_registered": BIN, "sessions": {}}
problems = []
for s in ["train-P20", "train-P24", "train-P28", "train-P32", "holdout-P20", "holdout-P24", "holdout-P28", "holdout-P32"]:
    sj = json.loads((SESS / s / "session.json").read_text())
    plan = json.loads((SESS / s / "plan.json").read_text())
    ex = plan["executions"]; arms = plan["arms"]; wls = plan["workloads"]
    c = Counter(); seqs = []
    aa = defaultdict(dict)
    for line in open(SESS / s / "records.jsonl"):
        r = json.loads(line)
        seqs.append(r["seq"])
        e = ex[r["seq"]]
        if not (arms[e["arm"]]["name"] == r["arm"] and wls[e["workload"]]["workload_id"] == r["workload"]["workload_id"]
                and e["round"] == r["round"] and e["warmup"] == r["warmup"] and str(e["algorithm_seed"]) == str(r["algorithm_seed"])):
            c["plan_mismatch"] += 1
        if r["session_id"] != sj["session_id"]: c["session_id_mismatch"] += 1
        if r["binary_sha256"] != BIN or sj["binary_sha256"] != BIN: c["binary_mismatch"] += 1
        if r["method"]["method_id"] != arms[e["arm"]]["method"]["method_id"]: c["method_mismatch"] += 1
        tg = r["cost"]["total_gae"]; ps = sum(p["gae"] for p in r["phases"])
        if abs(tg - ps) > 1e-9: c["phase_gae_sum_ne_total"] += 1
        if r["cost"]["deterministic"] is not True: c["nondeterministic"] += 1
        if r["isolation"]["level"] != "L0": c["isolation_not_L0"] += 1
        if r["arm"] in ("rho-neg", "rho-neg-aa") and not r["warmup"]:
            aa[(r["workload"]["workload_id"], r["round"])][r["arm"]] = (r["cost"]["total_gae"], json.dumps(r["counters"], sort_keys=True), r["algorithm_seed"])
        c["records"] += 1
    aa_identical = sum(1 for v in aa.values() if len(v) == 2 and v["rho-neg"] == v["rho-neg-aa"])
    info = {"session_id": sj["session_id"], "status": sj["status"], "started": iso(sj["started_unix_ms"]),
            "finished": iso(sj["finished_unix_ms"]), "executions_planned": sj["executions_planned"],
            "records_written": sj["records_written"], "plan_executions": len(ex),
            "seq_contiguous": sorted(seqs) == list(range(len(ex))), "git_commit": sj["git_commit"], "git_dirty": sj["git_dirty"],
            "counts": dict(c), "aa_pairs": len(aa), "aa_counted_identical": aa_identical}
    if s.startswith("train") and not (reg < info["started"] and info["finished"] < frz):
        problems.append(f"{s}: not between registration and freeze")
    if s.startswith("holdout") and not (frz < info["started"]):
        problems.append(f"{s}: holdout started before freeze")
    for k in ("plan_mismatch", "session_id_mismatch", "binary_mismatch", "method_mismatch", "phase_gae_sum_ne_total"):
        if c[k]: problems.append(f"{s}: {k} {c[k]}")
    if not info["seq_contiguous"] or sj["records_written"] != len(ex) or c["records"] != len(ex):
        problems.append(f"{s}: record count/seq")
    res["sessions"][s] = info
    print(s, info["started"], info["finished"], dict(c), "aa identical", aa_identical, "/", len(aa))
res["problems"] = problems
OUT.write_text(json.dumps(res, indent=1))
print("registered", reg, "frozen", frz, "problems", problems)
