import json, re, subprocess, statistics, os
ROOT = "/home/user/crypto/"
S = "research/koblitz-isogeny/"
P = "experiments/koblitz_presentation/"
rows = [json.loads(l) for l in open(ROOT + P + "pres_31_0_l10_geom_m3pin.jsonl") if l.strip()]
icc = json.load(open(ROOT + P + "icc_n31_m3_geom_diag.json"))["n=31 a2=0 l=10 m=3"]["work_per_call"]
n_cells = len(rows)
bsgs_ok = sum(1 for r in rows if r.get("bsgs_ok") is True)
inconsistent = sum(r.get("inconsistent", 0) for r in rows)
rel = sum(r["relations"] for r in rows); probes = sum(r["probes"] for r in rows)
per_call_ms = [r["groebner_ns"] / r["probes"] / 1e6 for r in rows]
med_ms = statistics.median(per_call_ms)
med_rel = statistics.median(r["relations"] for r in rows)
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8", errors="replace"), 1):
            if re.search(pat, line, re.I): out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "reports/run-20261011T2357Z/README.md", S + "reports/run-20261011T2357Z/report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep(study, r"A/A|null|control|identical")
iso = grep(study, r"isolat|shared|indicative|wall")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
claims = [
 {"claimed": "128/128 verified cells", "computed": "%d/%d cells with bsgs_ok true; sum inconsistent = %d" % (bsgs_ok, n_cells, inconsistent),
  "definition": "count of records in pres_31_0_l10_geom_m3pin.jsonl with bsgs_ok == true, out of record count; 'verified' is taken as bsgs_ok (study README says 'all BSGS-verified')",
  "files": [P + "pres_31_0_l10_geom_m3pin.jsonl"], "status": "match" if (bsgs_ok == 128 and n_cells == 128 and inconsistent == 0) else "mismatch"},
 {"claimed": "20,661 relations/256,000 probes", "computed": "%d relations / %d probes" % (rel, probes),
  "definition": "sum of relations and of probes over the 128 records", "files": [P + "pres_31_0_l10_geom_m3pin.jsonl"],
  "status": "match" if (rel == 20661 and probes == 256000) else "mismatch"},
 {"claimed": "median 16.6 ms/call", "computed": "%.3f ms (median over 128 cells of groebner_ns/probes; median relations per cell = %s)" % (med_ms, med_rel),
  "definition": "median over cells of groebner_ns / probes / 1e6. The record has no us_per_call field; this per-call definition is the closest raw quantity and is an assumption, so the study does not define the aggregation and the status is cannot_define (the value 16.626 rounds to 16.6 under this assumed definition, reported only as computed)",
  "files": [P + "pres_31_0_l10_geom_m3pin.jsonl"],
  "status": "cannot_define"},
 {"claimed": "work ICC 0.228 [0.032,0.403]", "computed": "stored: icc_curve %.4f, ci95 [%.4f, %.4f]; not recomputed (estimator and bootstrap code not in repo)" % (icc["icc_curve"], icc["ci95"][0], icc["ci95"][1]),
  "definition": "value read from icc_n31_m3_geom_diag.json key work_per_call; the stored value matches the printed one at 3 decimals, but the estimator is not re-derived from raw records",
  "files": [P + "icc_n31_m3_geom_diag.json"], "status": "cannot_define"},
]
out = {"row_no": 1018, "slug": "20261011-koblitz-m3-2357z", "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed_in_headline_or_measured"},
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "empty (design file m3-presentation-vs-curve-n31-design-20261012.md cited in report.typ is not in repo)" if not os.path.exists(ROOT + "m3-presentation-vs-curve-n31-design-20261012.md") else gitdate("m3-presentation-vs-curve-n31-design-20261012.md"),
 "first_raw_git_date": gitdate(P + "pres_31_0_l10_geom_m3pin.jsonl") + " (pres_31_0_l10_geom_m3pin.jsonl add commit)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1018.py",
 "notes": "Raw records are committed in experiments/koblitz_presentation/. Study PDF and README text are the source of the printed numbers. The design file cited in report.typ line 4 was not found in the repo root (checked path only). The 16.6 ms definition is an assumption (see claim 3)."}
print(json.dumps(out))
