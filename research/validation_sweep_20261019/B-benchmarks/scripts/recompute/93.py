#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 93 (20261013-ecc-arithmetic-modal).
# Reads committed modal_paired.csv (per-round nanoseconds, checksums) and the two Modal result JSONs.
import csv
import json
import os
import re
import statistics

REPO = "/home/user/crypto"
STUDY = "research/ecc_arithmetic_20261013"
BASE = os.path.join(REPO, STUDY)
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/93.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/93.py"
CSV_F = STUDY + "/modal_paired.csv"
RUNS = {1: STUDY + "/modal_result_attempt2_run1.json", 2: STUDY + "/modal_result_attempt2_run2.json"}

rows = list(csv.DictReader(open(os.path.join(REPO, CSV_F))))
print("csv rows", len(rows), "fields", sorted({r["field"] for r in rows}), "runs", sorted({r["run"] for r in rows}))
print("orders", sorted({r["order"] for r in rows}), "rounds", len({r["round"] for r in rows}))

def med_ms(run, field, arm):
    xs = [int(r["nanoseconds"]) / 1e6 for r in rows if r["run"] == str(run) and r["field"] == field and r["arm"] == arm]
    return statistics.median(xs), len(xs)

for run in (1, 2):
    b, nb = med_ms(run, "f2m131", "baseline")
    c, nc = med_ms(run, "f2m131", "candidate")
    print("run", run, "f2m131 baseline median %.3f (n=%d) candidate median %.3f (n=%d) ratio %.4f" % (b, nb, c, nc, c / b))

# identical outputs: checksum per (run, field, round, order) for baseline vs candidate
n_cmp = n_eq = 0
keys = {}
for r in rows:
    keys.setdefault((r["run"], r["field"], r["round"], r["order"]), {})[r["arm"]] = r["checksum"]
for k, v in keys.items():
    if "baseline" in v and "candidate" in v:
        n_cmp += 1
        n_eq += v["baseline"] == v["candidate"]
print("checksum pairs compared", n_cmp, "equal", n_eq)

# test counts from the Modal result JSONs
tests = {}
for run, f in RUNS.items():
    d = json.load(open(os.path.join(REPO, f)))
    m = re.search(r"running (\d+) tests", d["test_stdout"])
    p = re.search(r"test result: \w+\. (\d+) passed; (\d+) failed", d["test_stdout"])
    tests[run] = (m.group(1) if m else None, p.groups() if p else None, d["test_exit"], d["bench_exit"])
    print("run", run, "tests", tests[run])

claims = []
def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}

def close(claimed_val, comp, dec):
    return round(comp, dec) == claimed_val or abs(comp - claimed_val) <= 0.01 * claimed_val

b1, _ = med_ms(1, "f2m131", "baseline"); c1, _ = med_ms(1, "f2m131", "candidate")
b2, _ = med_ms(2, "f2m131", "baseline"); c2, _ = med_ms(2, "f2m131", "candidate")
claims.append(mk("GF(2^131) run 1 median baseline/candidate ms 36.079/33.580",
                 "%.3f/%.3f" % (b1, c1),
                 "median over 24 rounds of nanoseconds/1e6 per arm, field f2m131, run 1",
                 [CSV_F], "match" if close(36.079, b1, 3) and close(33.580, c1, 3) else "mismatch"))
claims.append(mk("GF(2^131) run 2 median baseline/candidate ms 28.825/27.043",
                 "%.3f/%.3f" % (b2, c2),
                 "median over 24 rounds of nanoseconds/1e6 per arm, field f2m131, run 2",
                 [CSV_F], "match" if close(28.825, b2, 3) and close(27.043, c2, 3) else "mismatch"))
claims.append(mk("34 tests per run",
                 "run1 %s, run2 %s" % (tests[1][1], tests[2][1]),
                 "count of 'running N tests' and 'test result' line in test_stdout of each result JSON",
                 [RUNS[1], RUNS[2]],
                 "match" if tests[1][0] == "34" and tests[2][0] == "34" else "mismatch"))
claims.append(mk("all 384 arm rows checked",
                 "%d rows in modal_paired.csv" % len(rows),
                 "count of data rows in modal_paired.csv",
                 [CSV_F], "match" if len(rows) == 384 else "mismatch"))

for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

row = {
    "row_no": 93,
    "slug": "20261013-ecc-arithmetic-modal",
    "claims": claims,
    "identical": {"claimed": False, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq,
                  "files": [CSV_F], "status": "no_raw_in_repo" if False else "not_claimed"},
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
