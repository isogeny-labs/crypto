# Row 45 | 20261015-ic-improve4-01-genus3-relations: recompute ratios and exponents from per-trial records.
import json, glob, math, os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "research/ic_improve4_20261015/01-genus3-relations"
RUN = f"{STUDY}/results/run"
cost = collections.defaultdict(lambda: collections.defaultdict(dict))  # arm -> bits -> curve -> cost
qbits = {}
used_files = set()
for f in sorted(glob.glob(os.path.join(C.REPO, RUN, "b*_*.jsonl"))):
    used_files.add(os.path.relpath(f, C.REPO))
    for line in open(f):
        if not line.strip():
            continue
        r = json.loads(line)
        qbits[r["bits"]] = r["q"]
        for x in r["results"]:
            cost[x["arm"]][r["bits"]][r["curve"]] = x["cost"]

def mean_cost(arm, b):
    cs = sorted(cost[arm][b].keys())
    common = [0, 1, 2]
    return sum(cost[arm][b][k] for k in common) / len(common)

def arm_series(prefix_name_by_bits, bits):
    return {b: mean_cost(prefix_name_by_bits(b), b) for b in bits}

def ols(xs, ys):
    mx, my = sum(xs)/len(xs), sum(ys)/len(ys)
    return sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)

def slope(series, lo, hi):
    bs = [b for b in range(lo, hi+1)]
    xs = [math.log2(qbits[b]) for b in bs]
    ys = [math.log2(series[b]) for b in bs]
    return ols(xs, ys)

INC = lambda b: "r2:lpsgebw2@16" if b <= 17 else "r2:lpsgebw2@8"
C1 = lambda b: "r2:lpsgebw2@16+C1" if b <= 17 else "r2:lpsgebw2@8+C1"
AA = lambda b: "aa:lpsgebw2@16" if b <= 17 else "aa:lpsgebw2@8"
C1C2W = lambda b: "w:lpsgebw2@16+C1C2" if b <= 17 else "w:lpsgebw2@8+C1C2"
C2W = lambda b: "w:lpsgebw2@16+C2" if b <= 17 else "w:lpsgebw2@8+C2"
C3 = lambda b: "r2:lpsgebw2o70c@32"
C4C1C2 = lambda b: "c4:lpsgebw2o100c@16+C1C2"
C1C3 = lambda b: "r2:lpsgebw2o70c@32+C1"
WCTRL = lambda b: "w:lpsgebw2@16" if b <= 17 else "w:lpsgebw2@8"


# Gate 0: round-4 incumbent records vs round-3 R2 records (r2:lpsgebw2 family), per (bits, curve)
R3 = "research/ic_improve3_20261015/01-genus3-relations/results/run"
r3 = {}
for f in sorted(glob.glob(os.path.join(C.REPO, R3, "b*_*.jsonl"))):
    for line in open(f):
        if not line.strip():
            continue
        rr = json.loads(line)
        for x in rr["results"]:
            if x["arm"].startswith("r2:lpsgebw2@") and x["arm"].endswith("+R1"):
                r3[(rr["bits"], rr["curve"])] = x
g_cmp = g_eq = g_diff = 0
g_missing = []
for b in range(12, 23):
    for k in sorted(cost[INC(b)][b].keys()):
        key = (b, k)
        g_cmp += 1
        if key not in r3:
            g_missing.append(key)
            g_diff += 1
            continue
        # round-4 record cost field vs round-3 cost field
        r4cost = cost[INC(b)][b][k]
        if r4cost == r3[key]["cost"]:
            g_eq += 1
        else:
            g_diff += 1
GATE0 = {"n_compared": g_cmp, "n_equal": g_eq, "n_differ": g_diff, "missing_round3": g_missing}

B = list(range(12, 23))
S = {}
for nm, fn in [("inc", INC), ("C1", C1), ("C1C2", C1C2W), ("C2", C2W), ("C3", C3), ("C4C1C2", C4C1C2), ("C1C3", C1C3), ("Wctrl", WCTRL)]:
    S[nm] = {b: mean_cost(fn(b), b) for b in B}
S["AA"] = {b: mean_cost(AA(b), b) for b in range(12, 22)}  # aa arms exist for 12..21 only

def ratio22(nm):
    return S[nm][22] / S["inc"][22]

sl = {nm: {"12_22": slope(S[nm], 12, 22), "16_22": slope(S[nm], 16, 22)} for nm in S if nm != "AA"}
claims = []
claims.append({"claimed": "C1 whole ratio 0.756 at 2^22", "computed": f"{ratio22('C1'):.4f}",
               "definition": "mean over curves 0,1,2 of cost(C1 arm) / cost(incumbent arm) at 22 bits (arms r2:lpsgebw2@8 family)",
               "files": [f"{RUN}/b22_c0_r2.jsonl", f"{RUN}/b22_c1_r2.jsonl", f"{RUN}/b22_c2_r2.jsonl"],
               "status": "match" if abs(ratio22("C1")-0.756)/0.756 <= 0.01 else "mismatch"})
claims.append({"claimed": "C3 whole ratio 0.979 at 2^22", "computed": f"{ratio22('C3'):.4f}",
               "definition": "as above, arm r2:lpsgebw2o70c@32", "files": [f"{RUN}/b22_c0_r2.jsonl"],
               "status": "match" if abs(ratio22("C3")-0.979)/0.979 <= 0.01 else "mismatch"})
claims.append({"claimed": "C1+C3 ratio 0.719 at 2^22 (README table)", "computed": f"{ratio22('C1C3'):.4f}",
               "definition": "as above, arm r2:lpsgebw2o70c@32+C1", "files": [f"{RUN}/b22_c0_r2.jsonl"],
               "status": "match" if abs(ratio22("C1C3")-0.719)/0.719 <= 0.01 else "mismatch"})
claims.append({"claimed": "C2 constant ratio 0.959 at 2^22 (against incumbent)", "computed": f"{ratio22('C2'):.4f}",
               "definition": "as above, arm w:lpsgebw2@8+C2 (W = 32 walks) vs incumbent", "files": [f"{RUN}/b22_c0_w.jsonl"],
               "status": "match" if abs(ratio22("C2")-0.959)/0.959 <= 0.01 else "mismatch"})
claims.append({"claimed": "C1+C2 ratio 0.715 at 2^22", "computed": f"{ratio22('C1C2'):.4f}",
               "definition": "as above, arm w:lpsgebw2@8+C1C2", "files": [f"{RUN}/b22_c0_w.jsonl"],
               "status": "match" if abs(ratio22("C1C2")-0.715)/0.715 <= 0.01 else "mismatch"})
claims.append({"claimed": "C4 ratio 0.690 at 2^22", "computed": f"{ratio22('C4C1C2'):.4f}",
               "definition": "as above, arm c4:lpsgebw2o100c@16+C1C2", "files": [f"{RUN}/b22_c0_c4.jsonl"],
               "status": "match" if abs(ratio22("C4C1C2")-0.690)/0.690 <= 0.01 else "mismatch"})
claims.append({"claimed": "W-walk control 0.999 at 2^22", "computed": f"{ratio22('Wctrl'):.4f}",
               "definition": "as above, arm w:lpsgebw2@8 vs incumbent", "files": [f"{RUN}/b22_c0_w.jsonl"],
               "status": "match" if abs(ratio22("Wctrl")-0.999)/0.999 <= 0.01 else "mismatch"})
d12 = sl["C1"]["12_22"] - sl["inc"]["12_22"]; d16 = sl["C1"]["16_22"] - sl["inc"]["16_22"]
claims.append({"claimed": "C1 exponent Delta -0.015 (12..22) and -0.034 (16..22)",
               "computed": f"OLS of log2(mean cost) on log2 q: inc 12..22 {sl['inc']['12_22']:.4f}, C1 {sl['C1']['12_22']:.4f}, Delta {d12:+.4f}; 16..22 inc {sl['inc']['16_22']:.4f}, C1 {sl['C1']['16_22']:.4f}, Delta {d16:+.4f}",
               "definition": "OLS slope of log2 mean cost on log2 q; Delta = arm minus incumbent. Study: paired bootstrap CI, 2000 resamples, RNG not stated in the README (point estimate only)",
               "files": [f"{RUN}/summary_all.json", f"{RUN}/analysis.md"],
               "status": "match" if (abs(d12+0.0152) <= 0.001 and abs(d16+0.0341) <= 0.001) else "mismatch"})
claims.append({"claimed": "incumbent exponent 1.430 (12..22), 1.408 (16..22)",
               "computed": f"{sl['inc']['12_22']:.4f} (12..22), {sl['inc']['16_22']:.4f} (16..22)",
               "definition": "OLS slope as above", "files": [f"{RUN}/analysis.md"],
               "status": "match" if (abs(sl['inc']['12_22']-1.430) <= 0.002 and abs(sl['inc']['16_22']-1.408) <= 0.002) else "mismatch"})
loc = lambda nm: math.log2(S[nm][22]/S[nm][21]) / math.log2(qbits[22]/qbits[21])
claims.append({"claimed": "local exponent 21-22: incumbent 1.408, C1 1.364",
               "computed": f"inc {loc('inc'):.4f}, C1 {loc('C1'):.4f} (log2 cost ratio over log2 q ratio, 21 to 22)",
               "definition": "two-point local slope between 2^21 and 2^22 on mean costs",
               "files": [f"{RUN}/analysis.md"],
               "status": "match" if (abs(loc('inc')-1.408) <= 0.002 and abs(loc('C1')-1.364) <= 0.002) else "mismatch"})
claims.append({"claimed": "A/A exponent Delta -0.0012 [-0.0037, +0.0014] (12..21)",
               "computed": f"point Delta (aa minus incumbent, 12..21) = {slope(S['AA'],12,21)-slope(S['inc'],12,21):+.4f}; CI not recomputed",
               "definition": "OLS slope difference over 12..21; the CI requires the bootstrap RNG which is not stated",
               "files": [f"{RUN}/analysis_aa.md"], "status": "partial"})
claims.append({"claimed": "per-trial log slope 9.75 to 2.71 units per bit; per-trial cost 0.650 at 2^22",
               "computed": "not recomputed: per-trial primitive cost tables (pertrial_*.json) are not re-derived here",
               "definition": "units per trial by primitive; needs the unit model from the study", "files": [f"{RUN}/pertrial_r2.json"],
               "status": "cannot_define"})
claims.append({"claimed": "Gate 0: 41/41 r2:lpsgebw2 records equal round 3 (cost field)",
               "computed": f"{GATE0['n_equal']}/{GATE0['n_compared']} equal on the cost field; missing round-3 pairs {GATE0['missing_round3']}",
               "definition": "per (bits, curve) cost equality between round-4 r2:lpsgebw2 and round-3 r2:lpsgebw2+R1; only the cost field is compared, not k, relation cost, LA counters or steps",
               "files": [f"{RUN}/gate0.md", R3], "status": "partial"})

offrepo = C.grep_lines([STUDY], C.OFFREPO_PAT)
aa_lines = C.grep_lines([f"{RUN}/analysis_aa.md", f"{STUDY}/README.md"], C.AA_PAT, cap=40)
iso_lines = C.grep_lines([f"{STUDY}/README.md", f"{RUN}/env.txt", f"{STUDY}/PROTOCOL.md"], C.ISO_PAT, cap=40)
prot_date = C.git_first_add(f"{STUDY}/PROTOCOL.md")
first_raw = C.git_first_add(f"{RUN}/b22_c0_r2.jsonl")
row = {
    "row_no": 45,
    "slug": "20261015-ic-improve4-01-genus3-relations",
    "claims": claims,
    "identical": {"claimed": True, "n_compared": GATE0["n_compared"], "n_equal": GATE0["n_equal"], "n_differ": GATE0["n_differ"],
                  "files": [f"{RUN}/b*_*_r2.jsonl (round 4, arm r2:lpsgebw2@16/@8)", f"{R3}/b*_*.jsonl (round 3, arm r2:lpsgebw2@16/@8+R1)"],
                  "status": "match" if GATE0["n_differ"] == 0 else "mismatch"},
    "offrepo_refs": offrepo,
    "aa_lines": aa_lines,
    "isolation_lines": iso_lines,
    "protocol_git_date": prot_date,
    "first_raw_git_date": first_raw,
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/45.py",
    "notes": ("Ratios are the mean over curves 0,1,2 at each size (common curve set). Arm names change at 2^18 "
              "(@16 for 12..17, @8 for 18..22) in the incumbent and W-walk families, as in the record files. "
              f"Trial files read: {len(used_files)} b*_*.jsonl files in results/run. Exponent Deltas are point OLS "
              "estimates; the study's CIs (2000 paired bootstrap resamples) are not regenerated. "
              "Identical claim (Gate 0) is a field-equality claim; not recomputed in this row. "
              "C2 and W-walk arms exist only at 12..22 with the same @16/@8 switch, as recorded."),
}
print(json.dumps(row, ensure_ascii=False))
