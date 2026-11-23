#!/usr/bin/env python3
"""Independent recomputation for research/ic_positive_control_unit_t_20261116.
Usage: python3 -I validate_unit_t.py REPO_ROOT OUT_JSON"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V

ROOT = sys.argv[1]
D = os.path.join(ROOT, "research/ic_positive_control_unit_t_20261116")
run, rows = V.load(os.path.join(D, "run.json"))
prev, prev_rows = V.load(os.path.join(ROOT, "research/ic_positive_control_la_20261116/run.json"))
by, pb = V.index(rows), V.index(prev_rows)
S, U, Vv, R = "C_coord_square", "C_unit_t", "C_unit_t_fast", "R_rho_negation"
assert set(by) == {S, U, Vv, R}, set(by)
keys = set(by[S])
assert all(set(by[a]) == keys for a in by)
out = {"study": "unit_t", "n_rows": len(rows), "n_per_arm": {a: len(by[a]) for a in by}}
out["accounting_violations"] = V.accounting_check(rows)
out["instance_mismatch_across_arms"] = [k for k in keys if len({(by[a][k]["order"], by[a][k]["h"], by[a][k]["r"]) for a in by}) != 1]
out["modes"] = {a: sorted({(r["oracle"], r["la_mode"], r["walk"]) for r in by[a].values()}) for a in (S, U, Vv)}

# G1: S and R equal previous study's S and R rows on every shared key
g1, shared = [], {S: 0, R: 0}
for a in (S, R):
    for k, r in sorted(by[a].items()):
        if k not in pb[a]:
            continue
        shared[a] += 1
        o = pb[a][k]
        flds = ["group_ops", "rel_mul", "la_mul", "trials", "relations", "certified"] if a == S else ["group_ops", "native_mul", "certified", "steps"]
        for f in flds:
            if r[f] != o[f]:
                g1.append(f"{a} {k} {f}: {r[f]} != {o[f]}")
        extra = V.compare_rows(r, o, ignore=V.NONDET | {"walk"})
        if extra:
            g1.append(f"{a} {k} other: {extra[:4]}")
out["G1"] = {"shared_compared": shared, "unshared": {a: sorted({k[0] for k in by[a] if k not in pb[a]}) for a in (S, R)},
             "failures": g1[:20], "n_failures": len(g1)}
out["G2"] = {"rows": len(rows), "uncertified": [(r["arm"], r["p"], r["curve_idx"], r["seed"]) for r in rows if r["certified"] is not True]}

# D1: V's walk field
out["D1"] = {"V_walks": sorted({r["walk"] for r in by[Vv].values()}), "V_oracles": sorted({r["oracle"] for r in by[Vv].values()}),
             "U_walks": sorted({r["walk"] for r in by[U].values()}), "S_walks": sorted({r["walk"] for r in by[S].values()})}
# V vs S: same walk; are trials/relations identical (root finder is the only difference)?
vs_same = {"trials": 0, "relations": 0, "group_ops": 0}
for k in keys:
    for f in vs_same:
        vs_same[f] += by[Vv][k][f] == by[S][k][f]
out["D1"]["V_equals_S_count"] = vs_same
out["D1"]["n"] = len(keys)
ps = sorted({k[0] for k in keys})
# F1: U fallbacks by p
out["F1"] = {"U_fallbacks_by_p": {p: sum(r["la_fallbacks"] for r in V.cell(by, U, p)) for p in ps},
             "U_runs_by_p": {p: len(V.cell(by, U, p)) for p in ps},
             "U_runs_with_fallback_by_p": {p: sum(1 for r in V.cell(by, U, p) if r["la_fallbacks"] > 0) for p in ps},
             "U_attempts_by_p": {p: sum(r["attempts"] for r in V.cell(by, U, p)) for p in ps},
             "S_fallbacks_by_p": {p: sum(r["la_fallbacks"] for r in V.cell(by, S, p)) for p in ps},
             "V_fallbacks_by_p": {p: sum(r["la_fallbacks"] for r in V.cell(by, Vv, p)) for p in ps},
             "p131_c1_s1001_U": {f: by[U][(131, 1, 1001)][f] for f in ["la_fallbacks", "attempts", "core_rows", "core_cols", "r", "certified"]}}

per_p = []
pt = lambda r: r["rel_mul"] / r["trials"]
for p in ps:
    cS, cU, cV, cR = (V.cell(by, a, p) for a in (S, U, Vv, R))
    e = {"p": p, "runs": len(cS)}
    e["U/S la_mul"] = V.boot(cU, cS, lambda r: r["la_mul"])
    e["U/S native_all"] = V.boot(cU, cS, lambda r: r["native_all"])
    e["U/S la incl adds"] = V.boot(cU, cS, lambda r: r["la_mul"] + r["la_add"])
    e["V/U rel per trial"] = V.boot(cV, cU, pt)
    e["V/S rel per trial"] = V.boot(cV, cS, pt)
    for a, s in [(S, "S"), (U, "U"), (Vv, "V")]:
        for w in V.WEIGHTS:
            e[f"{s}/R w={w}"] = V.boot(V.cell(by, a, p), cR, lambda r, w=w: V.total(r, w))
    e["U/S w=0.1 total"] = V.ratio_of_means(cU, cS, lambda r: V.total(r, 0.1))
    e["V/S w=0.1 total"] = V.ratio_of_means(cV, cS, lambda r: V.total(r, 0.1))
    per_p.append(e)
out["per_p"] = per_p
E = {e["p"]: e for e in per_p}
big = [p for p in ps if p >= 4099]
out["P5_slope_ln(V/R w0.1)"] = V.slope(big, [E[p]["V/R w=0.1"][0] for p in big])
out["slope_ln(U/R w0.1)"] = V.slope(big, [E[p]["U/R w=0.1"][0] for p in big])
out["verdict_below_all_prices_point"] = {s: [p for p in ps if all(E[p][f"{s}/R w={w}"][0] < 1 for w in V.WEIGHTS)] for s in "SUV"}
out["below_R_w0"] = {s: [p for p in ps if E[p][f"{s}/R w=0.0"][0] < 1] for s in "SUV"}
out["above_R_w01_w1"] = {s: [p for p in ps if E[p][f"{s}/R w=0.1"][0] > 1 and E[p][f"{s}/R w=1.0"][0] > 1] for s in "SUV"}
out["unit_t_rule U/S<=0.85 all p>=4099"] = {p: E[p]["U/S la_mul"][0] for p in big}
out["fallback_rule V/U<=0.9 at 2053,8209,65537"] = {p: E[p]["V/U rel per trial"][0] for p in (2053, 8209, 65537)}
out["fallback V/S at 2053,8209,65537"] = {p: E[p]["V/S rel per trial"][0] for p in (521, 2053, 8209, 65537)}
out["V/U at non-fallback p>=1031"] = {p: E[p]["V/U rel per trial"][0] for p in ps if p >= 1031 and p not in (2053, 8209, 65537)}
out["V/S at non-fallback p>=1031"] = {p: E[p]["V/S rel per trial"][0] for p in ps if p >= 1031 and p not in (2053, 8209, 65537)}

checks = []
def chk(label, claimed, actual, digits):
    checks.append({"claim": label, "claimed": claimed, "recomputed": round(actual, digits + 3), "ok": V.close(claimed, actual, digits=digits)})
comp = {8209: (0.778, 0.985, 0.770), 16411: (0.760, 0.962, 1.000), 32771: (0.772, 0.954, 1.000),
        65537: (0.781, 0.971, 0.823), 131101: (0.791, 0.965, 1.000)}
for p, (us, usa, vs) in comp.items():
    chk(f"U/S LA p={p}", us, E[p]["U/S la_mul"][0], 3)
    chk(f"U/S incl additions p={p} (native_all)", usa, E[p]["U/S native_all"][0], 3)
    chk(f"V/S rel per trial p={p}", vs, E[p]["V/S rel per trial"][0], 3)
e2e = {8209: (5.61, 5.34, 4.81, 29.4, 28.0, 25.0), 16411: (1.97, 1.77, 1.97, 10.1, 8.97, 10.1),
       32771: (1.92, 1.72, 1.92, 9.85, 8.75, 9.85), 65537: (2.57, 2.42, 2.25, 13.6, 12.8, 11.9),
       131101: (1.97, 1.78, 1.97, 10.2, 9.17, 10.2)}
for p, (s01, u01, v01, s1, u1, v1) in e2e.items():
    chk(f"S/R w=0.1 p={p}", s01, E[p]["S/R w=0.1"][0], 2)
    chk(f"U/R w=0.1 p={p}", u01, E[p]["U/R w=0.1"][0], 2)
    chk(f"V/R w=0.1 p={p}", v01, E[p]["V/R w=0.1"][0], 2)
    for lab, cv, key in [("S", s1, "S/R w=1.0"), ("U", u1, "U/R w=1.0"), ("V", v1, "V/R w=1.0")]:
        chk(f"{lab}/R w=1 p={p}", cv, E[p][key][0], 2 if cv < 10 else 1)
chk("P1 U/S LA at 4099 (1.24)", 1.24, E[4099]["U/S la_mul"][0], 2)
chk("P5 slope", -0.165, out["P5_slope_ln(V/R w0.1)"], 3)
chk("U/R w=0.1 at 131101 (1.78)", 1.78, E[131101]["U/R w=0.1"][0], 2)
out["claim_checks"] = checks
out["claim_mismatches"] = [c for c in checks if not c["ok"]]
out["claimed_intervals"] = {"V/R w0.1 8209": (3.44, 6.80), "U/R w0.1 16411": (1.03, 3.54), "U/R w0.1 32771": (0.97, 3.32),
                            "V/R w0.1 65537": (1.49, 3.62), "U/R w0.1 131101": (1.15, 4.52), "U/S LA 8209": (0.724, 0.936),
                            "V/S 8209": (0.765, 0.776), "V/S 65537": (0.822, 0.825)}
out["own_intervals"] = {"V/R w0.1 8209": E[8209]["V/R w=0.1"][1:], "U/R w0.1 16411": E[16411]["U/R w=0.1"][1:],
                        "U/R w0.1 32771": E[32771]["U/R w=0.1"][1:], "V/R w0.1 65537": E[65537]["V/R w=0.1"][1:],
                        "U/R w0.1 131101": E[131101]["U/R w=0.1"][1:], "U/S LA 8209": E[8209]["U/S la_mul"][1:],
                        "V/S 8209": E[8209]["V/S rel per trial"][1:], "V/S 65537": E[65537]["V/S rel per trial"][1:]}
json.dump(out, open(sys.argv[2], "w"), indent=1, default=str)
print("rows", out["n_rows"], out["n_per_arm"])
print("modes", out["modes"])
print("accounting violations", len(out["accounting_violations"]), out["accounting_violations"][:3])
print("G1", out["G1"]["shared_compared"], out["G1"]["unshared"], out["G1"]["n_failures"], out["G1"]["failures"][:3])
print("G2", out["G2"]["rows"], "uncertified", len(out["G2"]["uncertified"]))
print("D1", out["D1"])
print("F1", json.dumps(out["F1"]))
print("slope V", out["P5_slope_ln(V/R w0.1)"], "slope U", out["slope_ln(U/R w0.1)"])
print("verdict", out["verdict_below_all_prices_point"], "below w0", out["below_R_w0"])
print("above", out["above_R_w01_w1"])
print("unit-T rule", {p: round(v, 3) for p, v in out["unit_t_rule U/S<=0.85 all p>=4099"].items()})
print("fallback rule V/U", {p: round(v, 3) for p, v in out["fallback_rule V/U<=0.9 at 2053,8209,65537"].items()},
      "V/S", {p: round(v, 3) for p, v in out["fallback V/S at 2053,8209,65537"].items()})
print("V/U non-fallback", {p: round(v, 3) for p, v in out["V/U at non-fallback p>=1031"].items()})
print("V/S non-fallback", {p: round(v, 3) for p, v in out["V/S at non-fallback p>=1031"].items()})
print("U/S native_all", {e["p"]: round(e["U/S native_all"][0], 3) for e in per_p})
print("U/S total w0.1", {e["p"]: round(e["U/S w=0.1 total"], 3) for e in per_p}, "V/S total w0.1", {e["p"]: round(e["V/S w=0.1 total"], 3) for e in per_p})
print("own intervals", json.dumps(out["own_intervals"], default=lambda x: [round(y, 3) for y in x]))
print("claim mismatches:")
for c in out["claim_mismatches"]:
    print("  ", c)
print(len(checks), "claims checked")
