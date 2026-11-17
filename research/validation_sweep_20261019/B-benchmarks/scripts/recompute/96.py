#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 96 (20261015-gaudry-k4-d4-char2).
# C4 values from raw/summary.json (mean solve muls per ablation config); S values from the
# ecbench session records (cost.s); trend and paired ratios from raw/summary.json.
import json
import os
import statistics

REPO = "/home/user/crypto"
STUDY = "research/gaudry_quartic_d4_char2_20261015"
BASE = os.path.join(REPO, STUDY)
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/96.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/96.py"
SUMM = STUDY + "/raw/summary.json"
M6 = STUDY + "/sessions/char2-forms-m6/records.jsonl"
M14 = STUDY + "/sessions/char2-m14/records.jsonl"
ABL = [STUDY + "/raw/abl_plain_sr.jsonl", STUDY + "/raw/abl_weighted_sr.jsonl", STUDY + "/raw/abl_frob_sr.jsonl"]
RHO_THEORY_CONST = 0.886   # constant used in REPORT.typ ("log2(S_IC / 0.886)"; "IC / rho theory")

d = json.load(open(os.path.join(REPO, SUMM)))
cfg = {c["name"]: c for c in d["configs"]}
plain = cfg["abl_plain_sr"]["mean_muls"]
weighted = cfg["abl_weighted_sr"]["mean_muls"]
frob = cfg["abl_frob_sr"]["mean_muls"]
print("C4 m=8: plain %.4g weighted %.4g frobenius %.4g" % (plain, weighted, frob))
uni = [c["mean_muls"] for c in d["configs"] if c["form"]["frobenius"] and not c["planted"]]
print("frobenius uniform C4 over m=5..15:", sorted(round(x) for x in uni))
pr = {(p["a"], p["b"]): p for p in d["paired_ratios"]}
pf = pr[("abl_plain_sr", "abl_frob_sr")]
print("paired plain/frob", pf["ratio"], pf["ci95"], "pairs", pf["pairs"])
pw = pr[("abl_plain_sr", "abl_weighted_sr")]
pwf = pr[("abl_weighted_sr", "abl_frob_sr")]
print("paired plain/weighted", pw["ratio"], pw["ci95"], "weighted/frob", pwf["ratio"], pwf["ci95"])
tr = d["trend"]
print("trend keys", sorted(tr.keys()))
print("trend slope", tr.get("slope"), "log2_n_star", tr.get("log2_n_star"))

def S_of(path, arm):
    out = []
    for line in open(os.path.join(REPO, path)):
        if not line.strip():
            continue
        r = json.loads(line)
        if r["arm"] == arm and not r["warmup"]:
            out.append(r["cost"]["s"])
    return out

ic6_frob = S_of(M6, "ic-frobenius")
ic6_plain = S_of(M6, "ic-plain")
rho6 = S_of(M6, "rho-neg")
ic14 = S_of(M14, "ic-frobenius")
print("m6 frob S", ic6_frob, "plain S", ic6_plain, "rho-neg S", rho6)
print("m14 frob S", ic14)
pairs = [f / p for f, p in zip(ic6_frob, ic6_plain)]
print("m6 frob/plain per pair", pairs, "mean", statistics.mean(pairs))
ic6_mean = statistics.mean(ic6_frob)
print("m6 IC/rho-theory per target", [s / RHO_THEORY_CONST for s in ic6_frob])
print("m14 IC/rho-theory", [s / RHO_THEORY_CONST for s in ic14])

def mk(claimed, computed, definition, files, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files, "status": status}

def ok(x, y, rel=0.01):
    return abs(x - y) <= rel * abs(y)

claims = [
    mk("C4 Frobenius 1.98e5 at m=8",
       "%.4g" % frob, "mean_muls of abl_frob_sr (C4 per decomposition, m=8)",
       [SUMM], "match" if ok(frob, 1.98e5, 0.01) else "mismatch"),
    mk("plain/Frobenius 481x (479 to 483) at m=8",
       "%.2f (CI %.2f to %.2f); mean-ratio %.2f" % (pf["ratio"], pf["ci95"][0], pf["ci95"][1], plain / frob),
       "paired_ratios entry abl_plain_sr/abl_frob_sr (ratio and CI as recorded); mean_muls ratio alongside",
       [SUMM], "match" if round(pf["ratio"]) == 481 and round(pf["ci95"][0]) == 479 and round(pf["ci95"][1]) == 483 else "mismatch"),
    mk("C4 Frobenius 1.13 to 2.50e5 for m=5..15",
       "%.4g to %.4g" % (min(uni), max(uni)), "min and max over uniform frobenius configs in raw/summary.json",
       [SUMM], "match" if ok(min(uni), 1.13e5, 0.01) and ok(max(uni), 2.50e5, 0.01) else "mismatch"),
    mk("S Frobenius/plain 0.0057 (0.0051 to 0.0063) at m=6",
       "mean of per-pair ratios %.5f; pair min %.5f max %.5f (2 pairs)" % (statistics.mean(pairs), min(pairs), max(pairs)),
       "cost.s of ic-frobenius over ic-plain per paired target (sessions/char2-forms-m6); interval shown is pair min and max, not a bootstrap",
       [M6], "partial"),
    mk("IC / rho theory 5,080 and 4,124 at m=6 (one per target)",
       "; ".join("%.1f" % (s / RHO_THEORY_CONST) for s in ic6_frob),
       "cost.s of each ic-frobenius target at m=6 divided by 0.886 (constant as written in REPORT.typ)",
       [M6], "match" if ok(ic6_frob[0] / RHO_THEORY_CONST, 5080, 0.01) and ok(ic6_frob[1] / RHO_THEORY_CONST, 4124, 0.01) else "mismatch"),
    mk("IC / rho theory 6.06 at m=14",
       "%.4g" % (ic14[0] / RHO_THEORY_CONST) if ic14 else "none", "cost.s of ic-frobenius at m=14 (one target) divided by 0.886",
       [M14], "match" if ic14 and ok(ic14[0] / RHO_THEORY_CONST, 6.06, 0.005) else "mismatch"),
    mk("fitted slope -0.248 and n* 2^68.1",
       "slope %s ; log2_n_star %.3f" % (tr.get("slope", "absent"), tr["log2_n_star"]),
       "trend block of raw/summary.json (fit as recorded; the fit itself is not recomputed here)",
       [SUMM], "partial" if "slope" not in tr else ("match" if ok(tr["slope"], -0.248, 0.01) and ok(tr["log2_n_star"], 68.1, 0.01) else "mismatch")),
]
for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

row = {
    "row_no": 96,
    "slug": "20261015-gaudry-k4-d4-char2",
    "claims": claims,
    "identical": {"claimed": False, "n_compared": None, "n_equal": None, "n_differ": None,
                  "files": [], "status": "not_claimed"},
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
