import json, re, subprocess, os
ROOT = "/home/user/crypto/"
S = "research/lit2026_w4_early_abort_20261011/"
RUN = S + "reports/run-20261011T2025Z/"
summ = json.load(open(ROOT + RUN + "summary.json"))
T = summ["timing"]
def wx(cell, eng, arm): return T[cell][eng][arm]["counters"]["word_xors"]
rows = {}
for cell in ["k1n17", "k0n23", "k0n31"]:
    for eng in ["f4", "f6p"]:
        o = wx(cell, eng, "orig")
        rows[(cell, eng)] = (o / wx(cell, eng, "cand1"), o / wx(cell, eng, "cand2"))
f4 = [v for (c, e), v in rows.items() if e == "f4"]
al = list(rows.values())
m1_f4 = "%.5f-%.5f" % (min(v[0] for v in f4), max(v[0] for v in f4))
m1_all = "%.5f-%.5f" % (min(v[0] for v in al), max(v[0] for v in al))
m2_f4 = "%.5f-%.5f" % (min(v[1] for v in f4), max(v[1] for v in f4))
m2_all = "%.5f-%.5f" % (min(v[1] for v in al), max(v[1] for v in al))
detail = "; ".join("%s %s: mode1 %.5f, mode2 %.5f" % (c, e, v[0], v[1]) for (c, e), v in rows.items())
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
claims = [
 {"claimed": "Mode 1 original/candidate word-XOR ratios 1.00083-1.00378 (no-change)",
  "computed": "f4 rows only: " + m1_f4 + "; all six rows: " + m1_all + " (" + detail + ")",
  "definition": "original/cand1 counters.word_xors in summary.json timing, per cell and engine; the headline range equals the f4 rows only, the f6p rows (0.99993-1.00000) are outside it",
  "files": [RUN + "summary.json"],
  "status": "partial" if m1_f4 == "1.00083-1.00378" and m1_all != m1_f4 else ("match" if m1_all == "1.00083-1.00378" else "mismatch")},
 {"claimed": "mode 2 0.99714-0.99928 (negative)",
  "computed": "f4 rows only: " + m2_f4 + "; all six rows: " + m2_all,
  "definition": "original/cand2 counters.word_xors in summary.json timing; f6p rows add 0.99938 as the maximum, which the headline range omits",
  "files": [RUN + "summary.json"],
  "status": "partial" if m2_f4 == "0.99714-0.99928" and m2_all != m2_f4 else ("match" if m2_all == "0.99714-0.99928" else "mismatch")},
]
out = {
 "row_no": 1017, "slug": "20261011-lit2026-w4-early-abort",
 "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [RUN + "summary.json"],
   "status": "not_claimed_in_headline_or_measured (README states word_xors identical across rounds; not in HEADLINE/MEASURED)"},
 "offrepo_refs": off,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": "empty (no PROTOCOL file in study; README add commit: " + (gitdate(S + "README.md") or "empty") + ")",
 "first_raw_git_date": (gitdate(RUN + "summary.json") or "empty") + " (summary.json add commit)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1017.py",
 "notes": "Trace JSONL files in raw/ carry no word_xors field; the word_xors counters come from summary.json. Headline claims cover the f4 rows only. Off-repo grep hits README SSD path and similar."
}
print(json.dumps(out))
