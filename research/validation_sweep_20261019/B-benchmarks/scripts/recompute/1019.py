import json, re, subprocess, os
ROOT = "/home/user/crypto/"
S = "research/koblitz-isogeny/"
K = "experiments/koblitz_m3/"
def rows(p): return [json.loads(l) for l in open(ROOT + p) if l.strip()]
pv = rows(K + "pin_vs_e3.jsonl")
fp = rows(K + "full_solves_pin.jsonl")
def pick(n, solver): return [r for r in pv if r["n"] == n and r["m3_solver"] == solver and r["geometric"] is True][0]["us_per_call"] / 1000.0
e3_17, e3_19 = pick(17, "SymmetricE3"), pick(19, "SymmetricE3")
px_17, px_19 = pick(17, "PinX3"), pick(19, "PinX3")
full31 = [r["seconds"] for r in fp if r["n"] == 31]
def grep(files, pat):
    out = []
    for f in files:
        for i, line in enumerate(open(ROOT + f, encoding="utf-8", errors="replace"), 1):
            if re.search(pat, line, re.I): out.append("%s:%d: %s" % (f, i, line.strip()[:300]))
    return out
study = [S + "reports/run-20261011T2150Z/README.md", S + "reports/run-20261011T2150Z/report.typ"]
off = grep(study, r"drive\.google|/Volumes|\bSSD\b|/Users|s3://|gs://|store:|Modal|not committed|scratch")
aa = grep(study, r"A/A|null|control|identical")
iso = grep(study, r"isolat|shared|indicative|wall")
def gitdate(path):
    return subprocess.run(["git", "-C", ROOT, "log", "--diff-filter=A", "--format=%H %cI", "--", path], capture_output=True, text=True).stdout.strip()
claims = [
 {"claimed": "Geometric n=17/19 e3 9.702 / 123.595 ms", "computed": "%.3f / %.3f ms (SymmetricE3, geometric, us_per_call/1000)" % (e3_17, e3_19),
  "definition": "pin_vs_e3.jsonl rows with m3_solver SymmetricE3 and geometric true, n=17 and n=19", "files": [K + "pin_vs_e3.jsonl"],
  "status": "match" if ("%.3f" % e3_17, "%.3f" % e3_19) == ("9.702", "123.595") else "mismatch"},
 {"claimed": "PinX3 0.052 / 0.154 ms per call", "computed": "%.3f / %.3f ms" % (px_17, px_19),
  "definition": "pin_vs_e3.jsonl rows with m3_solver PinX3 and geometric true, n=17 and n=19, us_per_call/1000",
  "files": [K + "pin_vs_e3.jsonl"], "status": "match" if ("%.3f" % px_17, "%.3f" % px_19) == ("0.052", "0.154") else "mismatch"},
 {"claimed": "n=31 full solves 33.5 / 45.9 s", "computed": " / ".join("%.3f" % s for s in full31) + " s (full_solves_pin.jsonl, n=31, PinX3)",
  "definition": "seconds field of the n=31 records in full_solves_pin.jsonl, in file order", "files": [K + "full_solves_pin.jsonl"],
  "status": "match" if ["%.1f" % s for s in full31] == ["33.5", "45.9"] else "mismatch"},
]
out = {"row_no": 1019, "slug": "20261011-koblitz-m3-2150z", "claims": claims,
 "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None, "files": [], "status": "not_claimed_in_headline_or_measured (report.typ line 14 says identical relations; not in HEADLINE/MEASURED)"},
 "offrepo_refs": off, "aa_lines": aa, "isolation_lines": iso,
 "protocol_git_date": "empty (no protocol file cited in study)",
 "first_raw_git_date": gitdate(K + "pin_vs_e3.jsonl") + " (pin_vs_e3.jsonl add commit); full_solves_pin.jsonl: " + gitdate(K + "full_solves_pin.jsonl"),
 "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1019.py",
 "notes": "Raw records committed in experiments/koblitz_m3/. The per-call ms figures are taken from pin_vs_e3.jsonl (40 probes each, geometric V). The study's Run 3 table is the printed source. No off-repo reference was found in the study README or report.typ."}
print(json.dumps(out))
