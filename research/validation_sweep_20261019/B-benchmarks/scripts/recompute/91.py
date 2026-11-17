#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 91 (20261013/20261014 ic-m3-algebraic-vs-mitm).
# Recomputes the PROTOCOL.md section 5 quantities from the committed raw records:
#   floor amortized C = mean decision wall / p + table build wall / fb_columns   (truth-*.json)
#   arm C = (p*c_yes + (1-p)*c_no) / p, c_yes/c_no = mean wall over the arm's yes/no targets (arm-*.jsonl)
#   slope = OLS of log2 C on n. Bootstrap intervals (2000 reps, seed 20261014) are NOT reproduced:
#   the resampler is not specified in the study, so interval claims are cannot_define.
import glob
import json
import math
import os

REPO = "/home/user/crypto"
STUDY = "research/ic_m3_algebraic_vs_mitm_20261014"
BASE = os.path.join(REPO, STUDY)
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/91.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/91.py"


def git_date(rel):
    import subprocess
    r = subprocess.run(["git", "-C", REPO, "log", "--diff-filter=A", "--format=%H %cI", "--", rel],
                       capture_output=True, text=True)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else "EMPTY (no add commit found for %s)" % rel


def load(p):
    with open(p) as fh:
        return json.load(fh)


def read_jsonl(p):
    out = []
    with open(p) as fh:
        for line in fh:
            line = line.strip()
            if line:
                out.append(json.loads(line))
    return out


def ols_slope(xs, ys):
    n = len(xs)
    mx = sum(xs) / n
    my = sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    return sxy / sxx


# ---- floor cells
floor = {}   # (m, n) -> dict
for m, dirp in [(3, "runs/m3"), (4, "runs/m4")]:
    for f in sorted(glob.glob(os.path.join(BASE, dirp, "truth-m%d-n*.json" % m))):
        d = load(f)
        rows = d["rows"]
        mean_ns = sum(r["ns"] for r in rows) / len(rows)
        p = d["decomposable"] / d["pool"]
        amort = mean_ns / p + d["table_build_ns"] / d["fb_columns"]
        floor[(m, d["n"])] = {"p": p, "C": amort, "log2C": math.log2(amort), "file": os.path.relpath(f, REPO), "d": d["d"]}

floor_slope = {}
for m in (3, 4):
    ns = sorted(n for (mm, n) in floor if mm == m)
    floor_slope[m] = ols_slope(ns, [floor[(m, n)]["log2C"] for n in ns])
    print("floor m=%d cells %s slope %.4f" % (m, ns, floor_slope[m]))
    for n in ns:
        print("  n=%d log2 amort %.3f p %.4f" % (n, floor[(m, n)]["log2C"], floor[(m, n)]["p"]))

# ---- arm cells
arm = {}  # (m, arm, n) -> dict
for m, dirp in [(3, "runs/m3"), (4, "runs/m4")]:
    for f in sorted(glob.glob(os.path.join(BASE, dirp, "arm-*-m%d-n*.jsonl" % m))):
        base = os.path.basename(f)
        arm_name = base[len("arm-"):base.index("-m%d-" % m)]
        allrecs = read_jsonl(f)
        recs = [r for r in allrecs if r.get("cmd") == "solve" and "wall_ns" in r]
        if not recs:
            continue
        n = recs[0]["n"]
        cens = sum(1 for r in recs if r.get("censored"))
        yes = [r["wall_ns"] for r in recs if r.get("truth_yes") and not r.get("censored") and not r.get("unsupported")]
        no = [r["wall_ns"] for r in recs if (not r.get("truth_yes")) and not r.get("censored") and not r.get("unsupported")]
        agrees = [r.get("agrees") for r in recs if not r.get("censored") and not r.get("unsupported")]
        arm[(m, arm_name, n)] = {"recs": len(recs), "cens": cens, "yes": yes, "no": no, "agrees": agrees,
                                 "file": os.path.relpath(f, REPO)}

def arm_C(m, name, n):
    a = arm[(m, name, n)]
    if a["cens"] or not a["yes"] or not a["no"]:
        return None
    p = floor[(m, n)]["p"]
    cy = sum(a["yes"]) / len(a["yes"])
    cn = sum(a["no"]) / len(a["no"])
    return (p * cy + (1 - p) * cn) / p


def arm_slope(m, name):
    ns, ys = [], []
    for (mm, nm, n), a in sorted(arm.items()):
        if mm != m or nm != name or a["cens"] or not a["yes"] or not a["no"]:
            continue
        C = arm_C(m, name, n)
        ns.append(n)
        ys.append(math.log2(C))
    if len(ns) < 2:
        return None, ns
    fs = ols_slope(ns, [math.log2(floor[(m, n)]["C"]) for n in ns])
    return ols_slope(ns, ys), ns, fs


for key, a in sorted(arm.items()):
    m, name, n = key
    if a["cens"] or not a["yes"] or not a["no"]:
        print("arm m=%d %-22s n=%d censored=%d (lower bound, not a measurement)" % (m, name, n, a["cens"]))
        continue
    C = arm_C(m, name, n)
    print("arm m=%d %-22s n=%d log2C %.3f  C/floor %.4g" % (m, name, n, math.log2(C), C / floor[(m, n)]["C"]))

res = {}
for m in (3, 4):
    for name in sorted({nm for (mm, nm, n) in arm if mm == m}):
        r = arm_slope(m, name)
        if r[0] is None:
            res[(m, name)] = None
            print("slope m=%d %s: fewer than 2 measured cells" % (m, name))
        else:
            sl, ns, fs = r
            res[(m, name)] = (sl, ns, fs)
            print("slope m=%d %s on cells %s: %.4f ; floor on same cells %.4f ; delta %.4f" % (m, name, ns, sl, fs, sl - fs))

# cheapest measured arm / floor, per cell
cheap = {}
for m in (3,):
    for n in sorted(floor_n for (mm, floor_n) in floor if mm == m):
        cands = []
        for (mm, nm, nn), a in arm.items():
            if mm == m and nn == n and not a["cens"] and a["yes"] and a["no"]:
                cands.append((arm_C(m, nm, n), nm))
        if cands:
            C, nm = min(cands)
            cheap[n] = (nm, C / floor[(m, n)]["C"])
            print("cheapest measured m=3 n=%d: %s, C/floor %.4g" % (n, nm, C / floor[(m, n)]["C"]))

# lower bounds for censored arms: censored target cost = max(wall, 120 s); lower bound C from the same formula
LB = 120e9
lb = {}
for (m, nm, n), a in sorted(arm.items()):
    if not a["cens"]:
        continue
    recs = [r for r in read_jsonl(os.path.join(REPO, a["file"])) if r.get("cmd") == "solve" and "wall_ns" in r]
    p = floor[(m, n)]["p"]
    yes = [max(r["wall_ns"], LB) if r.get("censored") else r["wall_ns"] for r in recs if r.get("truth_yes")]
    no = [max(r["wall_ns"], LB) if r.get("censored") else r["wall_ns"] for r in recs if not r.get("truth_yes")]
    if not yes or not no:
        continue
    C = (p * (sum(yes) / len(yes)) + (1 - p) * (sum(no) / len(no))) / p
    lb[(m, nm, n)] = C / floor[(m, n)]["C"]
    print("lower bound m=%d %s n=%d censored=%d  C/floor >= %.4g" % (m, nm, n, a["cens"], C / floor[(m, n)]["C"]))

# exactness: agreement of every arm record with the truth
n_cmp = n_eq = 0
files = []
for (m, nm, n), a in sorted(arm.items()):
    n_cmp += sum(1 for x in a["agrees"] if x is not None)
    n_eq += sum(1 for x in a["agrees"] if x is True)
    files.append(a["file"])
print("exactness: records", n_cmp, "agree", n_eq)

# ---------------- claims
claims = []
def cl(claimed, computed, definition, files_, status):
    return {"claimed": claimed, "computed": computed, "definition": definition, "files": files_, "status": status}

fl_files = [floor[(3, n)]["file"] for n in (13, 17, 19, 23, 29, 31)]
claims.append(cl(
    "m=3 floor slope 0.207 [0.204, 0.211]",
    "point %.3f; interval not reproduced" % floor_slope[3],
    "OLS of log2 floor amortized C on n over the 6 m=3 cells (truth files); interval = bootstrap, resampler unspecified",
    fl_files,
    "partial"))
f4_3 = res[(3, "f4-chain")]
claims.append(cl(
    "m=3 f4-chain slope 0.534, delta [0.315, 0.338]",
    "point slope %.3f, delta %.3f; interval not reproduced" % (f4_3[0], f4_3[0] - f4_3[2]),
    "OLS of log2 C on n over the arm's measured cells; delta = arm slope minus floor slope on the same cells",
    [arm[(3, "f4-chain", n)]["file"] for n in f4_3[1]] + fl_files,
    "partial"))
f6_3 = res[(3, "f6-ic")]
claims.append(cl(
    "m=3 f6-ic slope 0.583, delta [0.366, 0.384]",
    "point slope %.3f, delta %.3f; interval not reproduced" % (f6_3[0], f6_3[0] - f6_3[2]),
    "as above for f6-ic",
    [arm[(3, "f6-ic", n)]["file"] for n in f6_3[1]] + fl_files,
    "partial"))
m4_fl = floor_slope[4]
f4_4 = res[(4, "f4-chain")]
claims.append(cl(
    "m=4 floor slope 0.363 [0.342, 0.379]; f4-chain slope 0.560",
    "floor point %.3f; f4-chain point %.3f on cells %s (floor on same cells %.3f); interval not reproduced"
    % (m4_fl, f4_4[0], f4_4[1], f4_4[2]),
    "OLS as above, m=4 cells n=11..31 for the floor",
    [floor[(4, n)]["file"] for n in sorted(n for (mm, n) in floor if mm == 4)],
    "partial"))
claims.append(cl(
    "cheapest arm / floor 3.0e2 at n=13 and 3.8e4 at n=31",
    "n=13 %s %.3g ; n=31 %s %.3g" % (cheap[13][0], cheap[13][1], cheap[31][0], cheap[31][1]),
    "min over measured m=3 arms of C, divided by floor C, at n=13 and n=31",
    [arm[(3, nm, 13)]["file"] for nm in ("f4-chain", "f6-ic", "s4-hybrid-f10", "s4-hybrid-f12")] +
    [arm[(3, nm, 31)]["file"] for nm in ("f4-chain", "f6-ic")],
    "match" if (abs(cheap[13][1] / 300 - 1) <= 0.02 and abs(cheap[31][1] / 3.8e4 - 1) <= 0.02) else "mismatch"))
lbs = sorted(v for k, v in lb.items() if k[1] in ("sat", "s4-hybrid-f10", "s4-hybrid-f12"))
claims.append(cl(
    "SAT and split hybrid censored, lower bounds 10^6 to 10^7 times floor",
    "censored m=3 SAT/split-hybrid lower bounds C/floor from %.3g to %.3g (n-cells listed in files)" % (min(lbs), max(lbs)),
    "C from max(wall,120 s) per censored target, same formula; lower bound only",
    [arm[k]["file"] for k in arm if k[1] in ("sat", "s4-hybrid-f10", "s4-hybrid-f12") and arm[k]["cens"]],
    "partial"))
claims.append(cl(
    "exactness against the floor: every arm verdict equals the truth",
    "records %d, agree %d" % (n_cmp, n_eq),
    "agrees field over all arm-*.jsonl records (m=3 and m=4)",
    sorted(set(files)),
    "match" if n_cmp == n_eq and n_cmp > 0 else "mismatch"))

for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

PROTO_DATE = git_date(STUDY + "/PROTOCOL.md")
RAW_DATE = git_date(STUDY + "/runs/m3/truth-m3-n13-d5.json")
NOTES = ("identical/exactness counts exclude censored, unsupported and skipped records (agrees is null there). "
         "Bootstrap intervals of ANALYSIS.md (2000 reps, seed 20261014) are not reproduced; their resampler is not specified in PROTOCOL.md. "
         "The sat m=3 cell at n=13 and the m=3 floor match ANALYSIS.md to 3 decimals. "
         "Off-repo grep over README.md, PROTOCOL.md, reports/run-20261014T1854Z/README.md: no matches. "
         "Only runs/m3 and runs/m4 are used; runs/m4-wide is not in any headline row.")
row = {
    "row_no": 91,
    "slug": "20261014-ic-m3-algebraic-vs-mitm",
    "claims": claims,
    "identical": {
        "claimed": True,
        "n_compared": n_cmp,
        "n_equal": n_eq,
        "n_differ": n_cmp - n_eq,
        "files": sorted(set(files)),
        "status": "match" if n_cmp == n_eq and n_cmp > 0 else "mismatch",
    },
    "offrepo_refs": [],
    "aa_lines": [],
    "isolation_lines": [],
    "protocol_git_date": PROTO_DATE,
    "first_raw_git_date": RAW_DATE,
    "script": SCRIPT,
    "notes": NOTES,
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("wrote", OUT)
