import json, re, subprocess, os
ROOT = "/home/user/crypto/"
S = "research/lit2026_w8_summation_models_20261011/"
RUN = S + "reports/run-20261011T2131Z/"
CELLS = ["K1/F2^15 m=2", "K0/F2^15 m=2", "K1/F2^17 m=2", "K0/F2^23 m=2", "K1/F2^23 m=2"]
def load(f): return json.load(open(ROOT + RUN + "raw/" + f))["result"]["cells"]
def arm(cells, name):
    return {c["cell"]: [a for a in c["arms"] if a["arm"] == name][0] for c in cells}
rounds = {}
for r in range(1, 6):
    rounds[r] = (load("r%d-orig.json" % r), load("r%d-cand.json" % r))
def wops(a): return sum(x["word_ops"] for t in a["samples"] for x in (t if isinstance(t, list) else [t]))
ratios = {}; digest_cmp = 0; digest_eq = 0; wop_eq_all = True
per_round_ratio = {}
for cell in CELLS:
    rs = []
    for r in range(1, 6):
        o, c = rounds[r]
        so = arm(o, "sym")[cell]; ec = arm(c, "eg")[cell]
        ro, rc = wops(so), wops(ec)
        rs.append(ro / rc)
        if r == 1: ratios[cell] = (ro, rc)
        for dO, dC in zip(so["key_set_digests"], ec["key_set_digests"]):
            digest_cmp += 1; digest_eq += (dO == dC)
        if ro != ratios[cell][0] or rc != ratios[cell][1]: wop_eq_all = False
    per_round_ratio[cell] = rs
rat = {c: ratios[c][0] / ratios[c][1] for c in CELLS}
claim1 = "%.3f-%.3f" % (min(rat.values()), max(rat.values()))
claim_rounds = len([f for f in os.listdir(ROOT + RUN + "raw") if re.match(r"r\d-orig\.json$", f)])
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8", errors="replace"), 1):
            if re.search(pat, line, re.I): out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "README.md", RUN + "report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep(study, r"A/A|null|control|original = previous")
iso = grep(study, r"isolat|L0|load average|shared")
raw_dates = sorted(set(gitdate(RUN + "raw/" + f) for f in os.listdir(ROOT + RUN + "raw") if f.endswith(".json")))
claims = [
 {"claimed": "Original/candidate F4 word-operation ratios 1.340-1.642",
  "computed": claim1 + " (total word_ops over 8 targets, round 1: " + "; ".join("%s: %d/%d=%.3f" % (c, ratios[c][0], ratios[c][1], rat[c]) for c in CELLS) + "; per-round ratios identical across rounds: " + str(all(len(set(round(x, 6) for x in per_round_ratio[c])) == 1 for c in CELLS)) + ")",
  "definition": "min..max over the five cells of sum(word_ops) of arm sym in r*-orig.json divided by sum(word_ops) of arm eg in r*-cand.json, round 1 (all rounds equal per cell; see computed field)",
  "files": [RUN + "raw/r1-orig.json", RUN + "raw/r1-cand.json"],
  "status": "match" if claim1 == "1.340-1.642" else "mismatch"},
 {"claimed": "five interleaved rounds", "computed": str(claim_rounds) + " original-round files (r1..r5-orig.json) and " + str(len([f for f in os.listdir(ROOT + RUN + 'raw') if re.match(r'r\d-cand\.json$', f)])) + " candidate-round files",
  "definition": "count of r<k>-orig.json files in raw/", "files": [RUN + "raw"],
  "status": "match" if claim_rounds == 5 else "mismatch"},
]
out = {
 "row_no": 1015, "slug": "20261011-lit2026-w8-quotient-s3",
 "claims": claims,
 "identical": {"claimed": True, "n_compared": digest_cmp, "n_equal": digest_eq, "n_differ": digest_cmp - digest_eq,
   "files": [RUN + "raw/r%d-orig.json" % r for r in range(1, 6)] + [RUN + "raw/r%d-cand.json" % r for r in range(1, 6)],
   "status": "match" if digest_cmp == digest_eq else "mismatch"},
 "offrepo_refs": off,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": "empty (no PROTOCOL file in this study; study README is part of commit ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 when present)",
 "first_raw_git_date": (raw_dates[0] if raw_dates else "empty") + " (earliest add commit among run-20261011T2131Z/raw/*.json)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1015.py",
 "notes": "identical compares sym (original) against eg (candidate) per target per round over key_set_digests; the previous arm is not in the comparison. word_ops compared are the per-target samples summed per round. Off-repo grep includes the README /Volumes path and the report's 'scratch crate' phrase (a non-data reference). The README also calls its root equalities 'identical' across original, previous and candidate; that was not recomputed for previous."
}
print(json.dumps(out))
