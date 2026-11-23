#!/usr/bin/env python3
"""Compare the producer's analysis.json (blob at the validated commit) and the numbers printed in
RESULTS.md with the validator's recompute.json.  Point estimates must agree to 1e-9 relative;
bootstrap bounds (different RNG and resample count) must agree within 0.002 absolute on a ratio
or 0.003 on a slope / exponent; RESULTS.md's rounded numbers must equal the recomputed value
rounded to the printed digits.  Writes validation/out/compare_claims.json.
"""
import json, math, os, subprocess

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
OUT = os.path.join(V, "out")
COMMIT = "469f5bf73199cfb69e01426df46170bd976bdb88"
REL = "research/genus_ladder_scaling_20261119"
A = json.loads(subprocess.check_output(["git", "-C", S, "show", f"{COMMIT}:{REL}/analysis.json"]))
RES = subprocess.check_output(["git", "-C", S, "show", f"{COMMIT}:{REL}/RESULTS.md"], text=True)
M = json.load(open(os.path.join(OUT, "recompute.json")))
WMAP = {"0": "0.0", "0.1": "0.1", "1": "1.0"}
point, bounds, flags = [], [], []


def P(name, claimed, mine):
    rel = abs(claimed - mine) / max(abs(mine), 1e-300)
    point.append({"name": name, "claimed": claimed, "recomputed": mine, "rel_diff": rel, "ok": rel <= 1e-9})


def B(name, claimed, mine, tol):
    bounds.append({"name": name, "claimed": claimed, "recomputed_independent_bootstrap": mine,
                   "abs_diff": abs(claimed - mine), "tol": tol, "ok": abs(claimed - mine) <= tol})


def F(name, claimed, mine):
    flags.append({"name": name, "claimed": claimed, "recomputed": mine, "ok": claimed == mine})


for k, c in A["cells"].items():
    m = M["cells"][k]
    F(f"{k}/keys", c["keys"], m["keys"])
    F(f"{k}/curves", c["curves"], m["curves"])
    P(f"{k}/mean_r", c["mean_r"], m["mean_r"])
    P(f"{k}/mean_cost_w1", c["mean_cost_w1"], m["mean_cost_w1"])
    P(f"{k}/la_share_w1", c["la_share_w1"], m["la_share_w1"])
    for cv, v in c["per_curve_w1"].items():
        P(f"{k}/per_curve_w1/{cv}", v, m["per_curve_w1"][cv])
    for w, mw in WMAP.items():
        P(f"{k}/neg/{w}/ratio", c["neg"][w]["ratio"], m["neg"][mw]["ratio"])
        B(f"{k}/neg/{w}/lo", c["neg"][w]["lo"], m["neg"][mw]["lo"], 0.002)
        B(f"{k}/neg/{w}/hi", c["neg"][w]["hi"], m["neg"][mw]["hi"], 0.002)
    F(f"{k}/below_all_w", c["below_all_w"], m["below_all_w"])
for arm, c in A["C1"].items():
    m = M["C1"][arm]
    for x in ("slope_ratio_vs_neg_w1", "cost_exponent", "middle_residual"):
        P(f"C1/{arm}/{x}", c[x], m[x])
    for x in ("slope_ci", "cost_exponent_ci"):
        for i in (0, 1):
            B(f"C1/{arm}/{x}/{i}", c[x][i], m[x][i], 0.003)
    F(f"C1/{arm}/consistent_with_1p5", c["consistent_with_1p5"], m["consistent_with_1p5_rule"])
for k, c in A["C2"].items():
    if k.startswith("N_"):
        m = M["C2"][k]
        P(f"C2/{k}/x5star_over_xstar_w1", c["x5star_over_xstar_w1"], m["x5star_over_xstar_w1"])
        P(f"C2/{k}/N_g5", c["N_g5"], m["N_g5"])
        P(f"C2/{k}/N_g4", c["N_g4"], m["N_g4"])
        for i in (0, 1):
            B(f"C2/{k}/ci/{i}", c["ci"][i], m["ci"][i], 0.003)
        F(f"C2/{k}/verdict", c["verdict"], m["verdict"])
    else:
        mk = {"slope_in_lnN_g5": "slope_in_lnN_g5_x5*", "slope_in_lnN_g4": "slope_in_lnN_g4_x*"}[k]
        P(f"C2/{k}/slope", c["slope"], M["C2"][mk]["slope"])
        for i in (0, 1):
            B(f"C2/{k}/ci/{i}", c["ci"][i], M["C2"][mk]["ci"][i], 0.003)
P("tune/choice/mean_cost_w1", A["tune"]["mean_cost_w1"], M["tune"]["choice"]["mean_cost_w1"])
F("tune/choice", [A["tune"]["cc"], A["tune"]["w_max"], A["tune"]["fill_max"]],
  [M["tune"]["choice"]["cc"], M["tune"]["choice"]["w_max"], M["tune"]["choice"]["fill_max"]])
P("rho_check/mean_over_sqrt_pi_r_2", A["rho_check"]["mean_over_sqrt_pi_r_2"], M["G3"]["mean_over_sqrt_pi_r_2"])
F("rho_check/runs", A["rho_check"]["runs"], M["G3"]["runs"])
F("gates", A["gates"], {"G1": M["G1"], "G2": True, "G3": M["G3"]["in_0p5_1p8"] and M["G3"]["certified_ok"]})

# RESULTS.md printed numbers (rounded) against the recomputation
def r3(x):
    return f"{x:.3f}"


cells = M["cells"]
expect = []
for key, lbl in (("g4_p54293_x", "x (cc 2.0)"), ("g4_p54293_x*", "**x\\* (cc 1.4)**"), ("g5_p2357_x5", "x5 (cc 2.0)"),
                 ("g5_p2357_x5*", "**x5\\* (cc 2.8, SGE 8,2)**"), ("g5_p4099_x5", "x5"), ("g5_p4099_x5*", "**x5\\***")):
    n = cells[key]["neg"]
    w1 = f"{r3(n['1.0']['ratio'])} [{r3(n['1.0']['lo'])}, {r3(n['1.0']['hi'])}]"
    expect.append((f"RESULTS headline row {key}", f"{r3(n['0.0']['ratio'])} | {r3(n['0.1']['ratio'])}", key))
    expect.append((f"RESULTS headline row {key} w=1 point", r3(n["1.0"]["ratio"]), key))
txt_checks = []
for name, s, key in expect:
    txt_checks.append({"name": name, "string": s, "found_in_RESULTS": s in RES})
for s in ["−0.431 [−0.435, −0.426]", "1.568 [1.564, 1.573]", "−0.366", "1.633", "−0.011", "−0.005",
          "0.479 [0.465, 0.491]", "0.315 [0.311, 0.319]", "−0.264", "−0.114", "0.82 ×", "0.524", "0.876", "0.640",
          "0.56–0.62", "0.11–0.19", "costs 0.20", "costs 0.64"]:
    txt_checks.append({"name": f"RESULTS string {s}", "string": s, "found_in_RESULTS": s in RES})
# what the validator's numbers round to, for the strings above
mine_strings = {
    "x* slope": f"{M['C1']['x*']['slope_ratio_vs_neg_w1']:.3f}", "x* exponent": f"{M['C1']['x*']['cost_exponent']:.3f}",
    "x slope": f"{M['C1']['x']['slope_ratio_vs_neg_w1']:.3f}", "x exponent": f"{M['C1']['x']['cost_exponent']:.3f}",
    "x* residual": f"{M['C1']['x*']['middle_residual']:.3f}", "x residual": f"{M['C1']['x']['middle_residual']:.3f}",
    "C2 2^56": f"{M['C2']['N_g5p2357_vs_g4p16411']['x5star_over_xstar_w1']:.3f}",
    "C2 2^60": f"{M['C2']['N_g5p4099_vs_g4p32771']['x5star_over_xstar_w1']:.3f}",
    "g5 lnN slope": f"{M['C2']['slope_in_lnN_g5_x5*']['slope']:.3f}", "g4 lnN slope": f"{M['C2']['slope_in_lnN_g4_x*']['slope']:.3f}",
    "rho G3": f"{M['G3']['mean_over_sqrt_pi_r_2']:.2f}",
    "LA share g4 x*": [round(cells[k]["la_share_w1"], 2) for k in ("g4_p16411_x*", "g4_p32771_x*", "g4_p54293_x*")],
    "LA share g5": [round(cells[k]["la_share_w1"], 2) for k in ("g5_p2357_x5", "g5_p2357_x5*", "g5_p4099_x5", "g5_p4099_x5*")],
}
res = {"point": point, "bounds": bounds, "flags": flags, "results_md": txt_checks, "validator_rounded": mine_strings,
       "summary": {"point_n": len(point), "point_ok": all(x["ok"] for x in point),
                   "point_max_rel_diff": max(x["rel_diff"] for x in point),
                   "bounds_n": len(bounds), "bounds_ok": all(x["ok"] for x in bounds),
                   "bounds_max_abs_diff": max(x["abs_diff"] for x in bounds),
                   "flags_ok": all(x["ok"] for x in flags), "results_md_strings_found": all(x["found_in_RESULTS"] for x in txt_checks)}}
json.dump(res, open(os.path.join(OUT, "compare_claims.json"), "w"), indent=1)
print(json.dumps(res["summary"], indent=1))
print(json.dumps(mine_strings, indent=1))
for x in point + bounds + flags:
    if not x["ok"]:
        print("MISMATCH", x)
for x in txt_checks:
    if not x["found_in_RESULTS"]:
        print("NOT FOUND", x)
