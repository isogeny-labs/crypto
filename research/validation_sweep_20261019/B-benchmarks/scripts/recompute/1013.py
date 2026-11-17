import json, re, subprocess, statistics, glob, os
ROOT = "/home/user/crypto/"
S = "research/pdp_oracle_speedup_20261011/"
B = S + "baseline_v1/"
rows = [json.loads(l) for l in open(ROOT + B + "raw/times.jsonl") if l.strip()]
if not rows or "ns_per_pair" not in rows[0]:
    raise SystemExit("unexpected times.jsonl schema: %r" % (list(rows[0].keys()) if rows else None))
def tag(r): return r["label"].split("-l")[0]
meds = {}; spreads = {}
for l in [8, 9, 10, 11, 12]:
    a = [r["ns_per_pair"] for r in rows if tag(r) == "A" and r["l"] == l]
    m = statistics.median(a)
    meds[l] = m
    spreads[l] = (max(a) - min(a)) / m
claim1 = " / ".join("%.1f" % meds[l] for l in [8, 9, 10, 11, 12])
mx = max(spreads.values()); lmx = max(spreads, key=spreads.get)
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8"), 1):
            if re.search(pat, line, re.I):
                out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [B + "README.md", B + "aa_summary.md", S + "reports/run-20261011T2107Z/report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch|/tmp")
aa = grep(study, r"A/A|spread")
iso = grep(study, r"isolat|reserved core|contended|pinned")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
pd = gitdate(S + "protocols/PS-0.md")
rawd = sorted(set(gitdate(p) for p in glob.glob(ROOT + B + "raw/*") and [os.path.relpath(p, ROOT) for p in glob.glob(ROOT + B + "raw/*")]))
claims = [
 {"claimed": "Baseline medians 337.7 / 375.4 / 393.3 / 421.4 / 452.3 ns per pair",
  "computed": claim1 + " (A arm median over 5 rounds, l=8..12; A ns_per_pair from times.jsonl)",
  "definition": "median over the five A-tagged rounds of times.jsonl ns_per_pair, per l, rounded to 0.1",
  "files": [B + "raw/times.jsonl"],
  "status": "match" if claim1 == "337.7 / 375.4 / 393.3 / 421.4 / 452.3" else "mismatch"},
 {"claimed": "largest A/A spread 8.81%",
  "computed": "%.2f%% at l=%d (" % (100 * mx, lmx) + ", ".join("l=%d:%.2f%%" % (l, 100 * spreads[l]) for l in [8, 9, 10, 11, 12]) + ")",
  "definition": "max over l of (max - min) / median of the five A-arm ns_per_pair values (study's stated spread definition, README section 5)",
  "files": [B + "raw/times.jsonl"],
  "status": "match" if abs(100 * mx - 8.81) <= 0.0881 else "mismatch"},
]
out = {
 "row_no": 1013, "slug": "20261011-pdp-ps0-baseline",
 "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
   "status": "not_claimed_in_headline_or_measured (digest replays are in study README section 4 and digests.txt only)"},
 "offrepo_refs": off,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": pd or "empty (protocols/PS-0.md has no add commit)",
 "first_raw_git_date": (rawd[0] if rawd else "empty") + " (earliest add commit among baseline_v1/raw/* files)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1013.py",
 "notes": "Study reports 'A/A spread' as (max-min)/median per arm; the largest is at l=9. Protocol and raw date strings are the git add commit dates as printed by git log. Off-repo grep includes the '/tmp/ps0_bin' phrase found in README section 1."
}
print(json.dumps(out))
