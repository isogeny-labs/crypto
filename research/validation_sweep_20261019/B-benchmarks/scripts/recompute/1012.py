import json, re, subprocess
ROOT = "/home/user/crypto/"
S = "research/pdp_oracle_speedup_20261011/"
rows = [json.loads(l) for l in open(ROOT + S + "ps1_algebra/raw/gates.jsonl") if l.strip()]
task = [r["task_fused"] for r in rows]
der = [r["f1_gcd_derived"]["gates"] for r in rows]
rat = [d / t for d, t in zip(der, task)]
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8"), 1):
            if re.search(pat, line, re.I):
                out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "ps1_algebra/README.md", S + "reports/run-20261011T2217Z/report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep(study, r"A/A|null|control")
iso = grep(study, r"timing|isolat|wall|host")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
pd = gitdate(S + "protocols/PS-1.md"); rd = gitdate(S + "ps1_algebra/raw/gates.jsonl")
def f(x): return "%d" % x
claims = [
 {"claimed": "7410-14608 fused ternary-logic issues per group-law task",
  "computed": "%d-%d" % (min(task), max(task)) + " (task_fused by l=8..12: " + ", ".join(str(t) for t in task) + ")",
  "definition": "min..max of task_fused over the five gates.jsonl records (l=8..12)",
  "files": [S + "ps1_algebra/raw/gates.jsonl"],
  "status": "match" if (min(task), max(task)) == (7410, 14608) else "mismatch"},
 {"claimed": "derived baseline 70695-190275", 
  "computed": "%d-%d" % (min(der), max(der)) + " (f1_gcd_derived.gates by l=8..12: " + ", ".join(str(d) for d in der) + ")",
  "definition": "min..max of f1_gcd_derived.gates as written in gates.jsonl; the derivation from baseline_v1/opcount/counts.jsonl was NOT recomputed here, so status is partial",
  "files": [S + "ps1_algebra/raw/gates.jsonl", S + "baseline_v1/opcount/counts.jsonl"],
  "status": "partial"},
 {"claimed": "9.5-13.0x",
  "computed": "%.2f-%.2f" % (min(rat), max(rat)) + " (derived/task_fused by l=8..12: " + ", ".join("%.2f" % r for r in rat) + ")",
  "definition": "min..max over l of f1_gcd_derived.gates / task_fused, rounded to 0.1 for the match test",
  "files": [S + "ps1_algebra/raw/gates.jsonl"],
  "status": "match" if ("%.1f" % min(rat), "%.1f" % max(rat)) == ("9.5", "13.0") else "mismatch"},
]
out = {
 "row_no": 1012, "slug": "20261011-pdp-ps1-algebra-gates",
 "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
   "status": "not_claimed_in_headline_or_measured (study README has agreement checks in section 3.2; not in HEADLINE/MEASURED)"},
 "offrepo_refs": off,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": pd or "empty (protocols/PS-1.md has no add commit)",
 "first_raw_git_date": rd or "empty",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1012.py",
 "notes": "Program-model gate counts; no timing. Claim 2 is a derived value read from the committed record, not re-derived. No A/A or null-control exists in this study (aa_lines shows only any matching text). Identity-certificate claim blocked per study section 5 is not a numeric claim here."
}
print(json.dumps(out))
