#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 95 (20261014-rootfinding-buffers).
# Reads committed session records, audit files and candidate-vs-baseline-a0-final.json.
import json
import math
import os

REPO = "/home/user/crypto"
STUDY = "research/ecbench_rootfinding_alloc_20261014"
BASE = os.path.join(REPO, STUDY)
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/95.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/95.py"
CMP = STUDY + "/candidate-vs-baseline-a0-final.json"
AUDITS = [STUDY + "/baseline-a0-audit.json", STUDY + "/baseline-a1-audit.json", STUDY + "/candidate-b-audit.json"]
RECS = [STUDY + "/sessions/baseline-a0/records.jsonl", STUDY + "/sessions/baseline-a1/records.jsonl",
        STUDY + "/sessions/candidate-b/records.jsonl"]

cmp_ = json.load(open(os.path.join(REPO, CMP)))
n_recs = sum(sum(1 for line in open(os.path.join(REPO, f)) if line.strip()) for f in RECS)
verified = 0
replays_n = replays_eq = 0
for a in AUDITS:
    d = json.load(open(os.path.join(REPO, a)))
    verified += d.get("verified_records", 0)
    for r in d.get("replays", []):
        replays_n += 1
        replays_eq += 1 if (r.get("reproduced") and r.get("detail") == "identical") else 0
print("records total", n_recs, "verified (audits)", verified, "replays", replays_n, "identical", replays_eq)
print("pairs", cmp_["records_candidate"], cmp_["records_baseline"], "deterministic_mismatches", cmp_["deterministic_mismatches"],
      "all_verified", cmp_["all_verified"], "missing", cmp_["missing_baseline"], cmp_["missing_candidate"])

t = cmp_["l0_timing"]
print("l0_timing curves", list(t.keys()))
for curve in t:
    for m in t[curve]:
        print(curve, m, "geomean %.6f" % t[curve][m]["candidate_over_baseline_geomean"] if False else "", end="")
        print()
sw = t["b48-batch-8"]["solve_wall"]
rw = t["b48-batch-8"]["relations_wall"]
print("batch-8 solve_wall", sw["candidate_over_baseline_geomean"], sw["target_cluster_bootstrap_95_percent"])
print("batch-8 relations_wall", rw["candidate_over_baseline_geomean"], rw["target_cluster_bootstrap_95_percent"])
print("targets per arm", len(sw["target_log_ratios"]))

def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}

def near(x, y, dec):
    return round(x, dec) == y

claims = [
    mk("384/384 verified sealed executions",
       "records %d, audit verified_records %d" % (n_recs, verified),
       "sum of records.jsonl lines over the three sessions; verified_records in the three audit files",
       RECS + AUDITS, "match" if n_recs == 384 and verified == 384 else "mismatch"),
    mk("288/288 identical measured replays",
       "replays %d, identical %d" % (replays_n, replays_eq),
       "replays[] entries in the three audit files with reproduced true and detail identical",
       AUDITS, "match" if replays_n == 288 and replays_eq == 288 else "mismatch"),
    mk("zero deterministic differences across 128 candidate/baseline pairs",
       "pairs %d, deterministic_mismatches %d" % (cmp_["records_candidate"], len(cmp_["deterministic_mismatches"])),
       "records_candidate and length of deterministic_mismatches in the final comparison",
       [CMP], "match" if cmp_["records_candidate"] == 128 and len(cmp_["deterministic_mismatches"]) == 0 else "mismatch"),
    mk("batch-8 relation-phase wall ratio 0.937 (0.786 to 1.122)",
       "geomean %.4f, cluster bootstrap [%.4f, %.4f]" % (rw["candidate_over_baseline_geomean"], rw["target_cluster_bootstrap_95_percent"][0], rw["target_cluster_bootstrap_95_percent"][1]),
       "l0_timing.b48-batch-8.relations_wall candidate_over_baseline_geomean and target_cluster_bootstrap_95_percent (8 targets)",
       [CMP], "match" if near(rw["candidate_over_baseline_geomean"], 0.937, 3) and near(rw["target_cluster_bootstrap_95_percent"][0], 0.786, 3) and near(rw["target_cluster_bootstrap_95_percent"][1], 1.122, 3) else "mismatch"),
    mk("batch-8 whole-solve wall ratio 0.993 (0.803 to 1.251)",
       "geomean %.4f, cluster bootstrap [%.4f, %.4f]" % (sw["candidate_over_baseline_geomean"], sw["target_cluster_bootstrap_95_percent"][0], sw["target_cluster_bootstrap_95_percent"][1]),
       "l0_timing.b48-batch-8.solve_wall candidate_over_baseline_geomean and target_cluster_bootstrap_95_percent",
       [CMP], "match" if near(sw["candidate_over_baseline_geomean"], 0.993, 3) and near(sw["target_cluster_bootstrap_95_percent"][0], 0.803, 3) and near(sw["target_cluster_bootstrap_95_percent"][1], 1.251, 3) else "mismatch"),
]
for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

row = {
    "row_no": 95,
    "slug": "20261014-rootfinding-buffers",
    "claims": claims,
    "identical": {"claimed": True, "n_compared": replays_n, "n_equal": replays_eq, "n_differ": replays_n - replays_eq,
                  "files": AUDITS, "status": "match" if replays_n == 288 and replays_eq == replays_n else "mismatch"},
    "offrepo_refs": [],
    "aa_lines": [],
    "isolation_lines": [],
    "protocol_git_date": "",
    "first_raw_git_date": "",
    "script": SCRIPT,
    "notes": "",
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("wrote", OUT)
