import json, re, subprocess
ROOT = "/home/user/crypto/"
S = "research/koblitz-isogeny/"
K = "experiments/koblitz_m3/"
rows = [json.loads(l) for l in open(ROOT + K + "solver_compare_n17.jsonl") if l.strip()]
e3 = {r["l"]: r for r in rows if r["m3_solver"] == "SymmetricE3"}
vals = [e3[l]["us_per_call"] / 1e6 for l in (3, 4, 5)]
sym5 = [r for r in rows if r["l"] == 5 and r["m3_solver"] not in ("SymmetricE3", "Chain")]
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8", errors="replace"), 1):
            if re.search(pat, line, re.I): out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "reports/run-20261011T1955Z/README.md", S + "reports/run-20261011T1955Z/report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep(study, r"A/A|null|control|identical")
iso = grep(study, r"isolat|shared|indicative|wall")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
claims = [
 {"claimed": "n=17 per-call e3 0.000116 / 0.00141 / 0.0216 s at l=3/4/5",
  "computed": " / ".join("%.6f" % v for v in vals) + " s (SymmetricE3 us_per_call/1e6, l=3,4,5)",
  "definition": "solver_compare_n17.jsonl rows with m3_solver SymmetricE3 at l=3,4,5, us_per_call/1e6",
  "files": [K + "solver_compare_n17.jsonl"],
  "status": "match" if ["%.3g" % v for v in vals] == ["0.000116", "0.00141", "0.0216"] else "mismatch"},
 {"claimed": "symmetric l=5 timed out (>300 s for four probes)",
  "computed": "row with m3_solver 'sym' at l=5: timed_out_after_s = %s, probes_requested = %s, all timing fields null" % (sym5[0].get("timed_out_after_s"), sym5[0].get("probes_requested")),
  "definition": "the l=5 symmetric row in solver_compare_n17.jsonl records timed_out_after_s and probes_requested; matches the claim (more than 300 s for four probes)",
  "files": [K + "solver_compare_n17.jsonl"], "status": "match" if (sym5 and sym5[0].get("timed_out_after_s") == 300 and sym5[0].get("probes_requested") == 4) else "mismatch"},
]
out = {"row_no": 1020, "slug": "20261011-koblitz-m3-1955z", "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed_in_headline_or_measured"},
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "empty (no protocol file cited in study)",
 "first_raw_git_date": gitdate(K + "solver_compare_n17.jsonl") + " (solver_compare_n17.jsonl add commit)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1020.py",
 "notes": "The study README table labels the e3 column; the SymmetricE3 rows are the e3 enumeration arm. The symmetric Groebner l=5 arm has no completed record; its row is m3_solver 'sym' with timed_out_after_s 300 and null timing fields."}
print(json.dumps(out))
