#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 94 (20261013-ecc-arithmetic). Apple M4 Pro run: paired_mac_final.csv
# (the report names it as the final paired run; paired_mac.csv is the first run and is also reported).
import csv
import json
import os
import re

REPO = "/home/user/crypto"
STUDY = "research/ecc_arithmetic_20261013"
BASE = os.path.join(REPO, STUDY)
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/94.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/94.py"
FINAL = STUDY + "/paired_mac_final.csv"
FIRST = STUDY + "/paired_mac.csv"
TESTS = STUDY + "/candidate_tests_final.stdout"


def load(rel):
    return list(csv.DictReader(open(os.path.join(REPO, rel))))


def rng(rows, field, arm):
    xs = [int(r["nanoseconds"]) for r in rows if r["field"] == field and r["arm"] == arm]
    return min(xs), max(xs), len(xs)


rows = load(FINAL)
print("final rows", len(rows), "fields", sorted({r["field"] for r in rows}), "rounds", len({r["round"] for r in rows}),
      "iterations", sorted({r["iterations"] for r in rows}))
bmin, bmax, bn = rng(rows, "f2m131", "baseline")
cmin, cmax, cn = rng(rows, "f2m131", "candidate")
print("final f2m131 baseline %d..%d (n=%d)  candidate %d..%d (n=%d)" % (bmin, bmax, bn, cmin, cmax, cn))
first = load(FIRST)
fbmin, fbmax, _ = rng(first, "f2m131", "baseline")
fcmin, fcmax, _ = rng(first, "f2m131", "candidate")
print("first-run f2m131 baseline %d..%d candidate %d..%d (rows %d)" % (fbmin, fbmax, fcmin, fcmax, len(first)))

txt = open(os.path.join(REPO, TESTS)).read()
res = re.findall(r"running (\d+) tests\n.*?test result: \w+\. (\d+) passed; (\d+) failed", txt, flags=re.S)
print("candidate_tests_final result lines", res)
focused = [r for r in res if r[0] == "17"]

# checksum pairs (baseline vs candidate) for the final run
keys = {}
for r in rows:
    keys.setdefault((r["field"], r["round"], r["order"]), {})[r["arm"]] = r["checksum"]
n_cmp = sum(1 for v in keys.values() if "baseline" in v and "candidate" in v)
n_eq = sum(1 for v in keys.values() if "baseline" in v and "candidate" in v and v["baseline"] == v["candidate"])
print("checksum pairs", n_cmp, "equal", n_eq)

def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}

claims = [
    mk("GF(2^131) baseline 27,826,500 to 932,235,083 ns per 500,000",
       "%d to %d" % (bmin, bmax), "min and max of nanoseconds, f2m131 baseline arm, paired_mac_final.csv",
       [FINAL], "match" if (bmin, bmax) == (27826500, 932235083) else "mismatch"),
    mk("candidate 16,801,958 to 694,301,375 ns per 500,000",
       "%d to %d" % (cmin, cmax), "min and max of nanoseconds, f2m131 candidate arm, paired_mac_final.csv",
       [FINAL], "match" if (cmin, cmax) == (16801958, 694301375) else "mismatch"),
    mk("24 AB/BA rounds per field, 500,000 products per arm per round",
       "rounds %d, iterations %s, rows %d" % (len({r['round'] for r in rows}), sorted({r['iterations'] for r in rows}), len(rows)),
       "distinct round values, iterations column, row count in paired_mac_final.csv",
       [FINAL], "match" if len({r['round'] for r in rows}) == 24 and sorted({r['iterations'] for r in rows}) == ['500000'] else "mismatch"),
    mk("17 focused tests passed",
       "; ".join("%s tests: %s passed, %s failed" % (r[0], r[1], r[2]) for r in res),
       "'running N tests' and 'test result' counts in candidate_tests_final.stdout",
       [TESTS], "match" if focused and focused[0][1] == "17" and focused[0][2] == "0" else "mismatch"),
]
for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

row = {
    "row_no": 94,
    "slug": "20261013-ecc-arithmetic",
    "claims": claims,
    "identical": {"claimed": False, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
                  "files": [FINAL], "status": "not_claimed"},
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
