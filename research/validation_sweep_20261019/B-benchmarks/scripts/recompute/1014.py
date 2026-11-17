import json, re, subprocess, glob, os
ROOT = "/home/user/crypto/"
S = "research/lit2026_w8_summation_models_20261011/"
RAW = S + "reports/run-20261011T2017Z/raw/"
def load(f): return json.load(open(ROOT + f))["result"]
def cellmap(f):
    out = {}
    for c in load(f)["cells"]:
        for a in c["arms"]:
            out[(c["cell"], a["arm"])] = a["median_word_ops"]
    return out
# convolution splitting: repo vs convolution, all cells present in lucas.json (full run) and lucas-l34.json (final build)
full = cellmap(RAW + "lucas.json")
l34 = cellmap(RAW + "lucas-l34.json")
conv_cells = sorted({k[0] for k in full if k[1] == "e-split-convolution"})
conv_ratio = {c: full[(c, "e-split-convolution")] / full[(c, "e-split-repo")] for c in conv_cells}
conv_ratio34 = {c: l34[(c, "e-split-convolution")] / l34[(c, "e-split-repo")] for c in conv_cells if (c, "e-split-convolution") in l34}
claim1 = "%.2f-%.2f" % (min(conv_ratio.values()), max(conv_ratio.values()))
# frobenius auxiliaries: MatrixF4 cells in frob.json
frob = {}
for c in load(RAW + "frob.json")["cells"]:
    frob[c["cell"]] = (c["arms"][1]["median_word_ops"] / c["arms"][0]["median_word_ops"], c["engine"], c["arms"][0]["arm"], c["arms"][1]["arm"])
mat = {k: v for k, v in frob.items() if "MatrixF4" in str(v[1])}
allr = {k: round(v[0], 1) for k, v in mat.items()}
claim2_all = "%.1f-%.1f" % (min(v[0] for v in mat.values()), max(v[0] for v in mat.values()))
table_cells = ["K1/F2^15 m=2", "K0/F2^15 m=2", "K0/F2^9 m=3"]  # README section 3 MatrixF4 rows
t = [mat[c][0] for c in table_cells if c in mat]
claim2_table = "%.1f-%.1f" % (min(t), max(t))
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8"), 1):
            if re.search(pat, line, re.I):
                out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "README.md", S + "reports/run-20261011T2017Z/report.typ", S + "reports/run-20261011T2131Z/report.typ"]
off = grep([f for f in study if os.path.exists(ROOT + f)], r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep([S + "README.md"], r"A/A|null|control|brute")
iso = grep([S + "README.md"], r"isolat|L0|load average|shared")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
readme_d = gitdate(S + "README.md")
rawfiles = [os.path.relpath(p, ROOT) for p in glob.glob(ROOT + RAW + "*.json")]
raw_dates = sorted(set(gitdate(p) for p in rawfiles))
claims = [
 {"claimed": "Convolution splitting: 1.21-2.07x more word operations",
  "computed": claim1 + " (convolution/repo median_word_ops over cells " + "; ".join("%s: %.3f" % (c, conv_ratio[c]) for c in conv_cells) + "; from lucas.json (3 cells); lucas-l34.json (2 cells) gives " + "; ".join("%s: %.3f" % (c, conv_ratio34[c]) for c in conv_ratio34) + ")",
  "definition": "min..max over the convolution cells of median_word_ops(e-split-convolution)/median_word_ops(e-split-repo), arm medians over 8 targets x 5 rounds as recorded in lucas.json",
  "files": [RAW + "lucas.json", RAW + "lucas-l34.json"],
  "status": "match" if claim1 == "1.21-2.07" else "mismatch"},
 {"claimed": "Frobenius auxiliaries: 57-701x more MatrixF4 word operations",
  "computed": "over the three README table MatrixF4 cells: " + claim2_table + "; over ALL MatrixF4 cells in frob.json: " + claim2_all + " (per cell: " + "; ".join("%s: %.2f" % (k, v[0]) for k, v in mat.items()) + ")",
  "definition": "median_word_ops(x-frob-aux)/median_word_ops(x-direct) over MatrixF4 cells of frob.json. The README table's three MatrixF4 cells give 57.2-701.2, which agrees with the headline 57-701 at integer precision; frob.json also holds a fourth MatrixF4 cell (K0/F2^9 m=2, 30.1x) that the headline range omits, so the claim is partial",
  "files": [RAW + "frob.json"],
  "status": "partial"},
]
out = {
 "row_no": 1014, "slug": "20261011-lit2026-w8-summation-models",
 "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [],
   "status": "not_claimed_in_headline_or_measured (root-set equality is stated in README sections 1-4 only)"},
 "offrepo_refs": off,
 "aa_lines": aa,
 "isolation_lines": iso,
 "protocol_git_date": "empty (no PROTOCOL file in study; README add commit: " + (readme_d or "empty") + ")",
 "first_raw_git_date": (raw_dates[0] if raw_dates else "empty") + " (earliest add commit among reports/run-20261011T2017Z/raw/*.json)",
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1014.py",
 "notes": "lucas.json holds the three convolution cells (n=9,l=3; n=12,l=4; n=15,l=5); lucas-l34.json, named in RAW CANDIDATES, holds only the first two. frob.json holds a fourth MatrixF4 cell (K0/F2^9 m=2, ratio shown above) absent from the README frob table. Study statements about degree and 'identical root sets' are not in HEADLINE/MEASURED and were not recomputed. Off-repo: README line with /Volumes/SSD990-2 path is in offrepo_refs."
}
print(json.dumps(out))
