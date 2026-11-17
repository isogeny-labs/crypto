import json, re, subprocess, os
ROOT = "/home/user/crypto/"
S = "research/lit2026_w5_lane_batched_f4_20261011/"
RUN = S + "reports/run-20261011T2225Z/"
summ = json.load(open(ROOT + RUN + "summary.json"))
cells = {(c["cell"], c["engine"]): c["arms"] for c in summ["cells"]}
def wx(c, e, a): return cells[(c, e)][a]["word_xors"]
f6 = sorted(k[0] for k in cells if k[1] == "f6p")
g = {c: wx(c, "f6p", "orig") / wx(c, "f6p", "gate") for c in f6}
claim1 = "%.4f-%.4f" % (min(g.values()), max(g.values()))
batch = [("K1_17", "f4"), ("K1_17", "f6p"), ("K1_23", "f4"), ("K1_23", "f6p"), ("K0_23", "f4"), ("K0_23", "f6p")]
lr = {(c, e): wx(c, e, "lanes") / wx(c, e, "rref") for c, e in batch if (c, e) in cells}
f6_only = [v for (c, e), v in lr.items() if e == "f6p"]
claim2_f6 = "%.1f-%.1f" % (min(f6_only), max(f6_only))
claim2_all = "%.2f-%.2f" % (min(lr.values()), max(lr.values()))
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8", errors="replace"), 1):
            if re.search(pat, line, re.I): out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "README.md", RUN + "report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep(study, r"A/A|null|control|identical")
iso = grep(study, r"isolat|L0|unpinned|load")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
rd = gitdate(S + "README.md")
raw_dates = sorted(set(gitdate(RUN + "summary.json") and [gitdate(RUN + "summary.json")]))
claims = [
 {"claimed": "Gate-first original/candidate word-XOR ratios 1.5675-1.7788",
  "computed": claim1 + " (orig/gate word_xors, f6p cells: " + "; ".join("%s: %.4f" % (c, g[c]) for c in f6) + "; f4 cells unchanged at 1.0 are not in this range)",
  "definition": "min..max over f6p cells of summary.json word_xors(orig)/word_xors(gate), 4 decimals; summary word_xors are deterministic (word_xors_deterministic true)",
  "files": [RUN + "summary.json"],
  "status": "match" if claim1 == "1.5675-1.7788" else "mismatch"},
 {"claimed": "lane roots cost 2.6-4.0x sequential control in the three batch cells",
  "computed": "f6p only: " + claim2_f6 + "; all six batch-cell rows (f4 and f6p): " + claim2_all + " (lanes/rref word_xors: " + "; ".join("%s %s: %.3f" % (c, e, v) for (c, e), v in lr.items()) + ")",
  "definition": "lanes word_xors divided by rref word_xors (the sequential control) for the three batch cells K1_17 k=8, K1_23 k=4, K0_23 k=4, per engine; the headline range matches only the f6p rows",
  "files": [RUN + "summary.json"],
  "status": "partial" if claim2_f6 == "2.6-4.0" and claim2_all != claim2_f6 else ("match" if claim2_f6 == "2.6-4.0" else "mismatch")},
]
out = {
 "row_no": 1016, "slug": "20261011-lit2026-w5-gate-first-lane-roots",
 "claims": claims,
 "identical": {"claimed": False, "n_compared": sum(1 for c in summ["cells"] if c.get("relations_identical_across_candidate_arms") is not None), "n_equal": sum(1 for c in summ["cells"] if c.get("verdicts_and_counters_identical") and c.get("relations_identical_across_candidate_arms")), "n_differ": None, "files": [RUN + "summary.json"],
   "status": "not_claimed_in_headline_or_measured (summary.json carries per-cell identity flags; reported here only as recorded)"},
 "offrepo_refs": off,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": "empty (no PROTOCOL file in study; README add commit: " + (rd or "empty") + ")",
 "first_raw_git_date": (raw_dates[0] if raw_dates else "empty") + " (summary.json add commit)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1016.py",
 "notes": "The README 'Gate-first' table ratios equal summary.json orig/gate word_xors for f6p. The lane-root ratios are word_xors (lanes/rref); the README root-phase table of lane word-equivalents is a separate diagnostic (diag-lanes-echelon.jsonl) and was not used for the headline. The off-repo grep hits the /Volumes SSD path in README section 'Commands'. Summary 'n_compared' is the count of cells carrying identity flags, not a digest count."
}
print(json.dumps(out))
