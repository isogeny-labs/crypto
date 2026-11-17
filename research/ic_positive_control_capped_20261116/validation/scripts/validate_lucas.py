#!/usr/bin/env python3
"""Independent recomputation for research/ic_positive_control_lucas_20261116.
Usage: python3 -I validate_lucas.py REPO_ROOT OUT_JSON
Reads run.json and the 11-12 control.json; recomputes gates G1-G3 (PROTOCOL §4),
the per-trial table, paired ratios, arm/R ratios at w = 0, 0.1, 1, LA shares, and
compares with numbers transcribed by hand from RESULTS.md."""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V

ROOT = sys.argv[1]
D = os.path.join(ROOT, "research/ic_positive_control_lucas_20261116")
run, rows = V.load(os.path.join(D, "run.json"))
old, old_rows = V.load(os.path.join(ROOT, "research/ic_positive_control_20261112/control.json"))
by, ob = V.index(rows), V.index(old_rows)
C, L, M, R = "C_double_lp", "C_lucas", "C_lucas_roots", "R_rho_negation"
out = {"study": "lucas", "n_rows": len(rows), "n_per_arm": {a: len(by[a]) for a in by}}
ps = sorted({r["p"] for r in rows})

# arms present must be exactly CLMR and every arm must cover the same keys
assert set(by) == {C, L, M, R}, set(by)
keys = set(by[C])
assert all(set(by[a]) == keys for a in by), "arms cover different (p, curve, seed)"
# instance identity across arms: order, h, r equal for the same (p, curve)
inst_bad = [k for k in keys if len({(by[a][k]["order"], by[a][k]["h"], by[a][k]["r"]) for a in by}) != 1]
out["instance_mismatch_across_arms"] = inst_bad

# accounting
out["accounting_violations"] = V.accounting_check(rows)

# G1: C and R rows equal 11-12 control rows
g1 = []
n1 = 0
for arm, fields in [(C, ["group_ops", "rel_mul", "la_mul", "trials", "relations", "certified"]),
                    (R, ["group_ops", "native_mul", "certified", "steps", "walk_ops"])]:
    for k, r in sorted(by[arm].items()):
        if k not in ob[arm]:
            g1.append(f"{arm} {k}: no 11-12 row")
            continue
        n1 += 1
        o = ob[arm][k]
        for f in fields:
            if r[f] != o[f]:
                g1.append(f"{arm} {k} {f}: {r[f]} != {o[f]}")
        # also every other deterministic field
        extra = V.compare_rows(r, o, ignore=V.NONDET | {"oracle"})
        if extra:
            g1.append(f"{arm} {k} other fields differ: {extra[:4]}")
out["G1"] = {"compared_rows": n1, "failures": g1[:30], "n_failures": len(g1)}

# G2: L equals C in trials, splits, relations, group_ops, la_mul, la_add; rel_mul strictly lower.
# (The recovered log is not in run.json; checked separately by the replay dumper.)
g2 = []
for k in sorted(keys):
    c, l = by[C][k], by[L][k]
    for f in ["trials", "splits", "relations", "group_ops", "la_mul", "la_add"]:
        if c[f] != l[f]:
            g2.append(f"{k} {f}: L {l[f]} != C {c[f]}")
    if not l["rel_mul"] < c["rel_mul"]:
        g2.append(f"{k} rel_mul not lower")
out["G2"] = {"compared": len(keys), "failures": g2[:30], "n_failures": len(g2),
             "note": "recovered-log clause not checkable from run.json (no recovered field)"}
# G3
out["G3"] = {"rows": len(rows), "uncertified": [(r["arm"], r["p"], r["curve_idx"], r["seed"]) for r in rows if r["certified"] is not True],
             "all_certified_flag": run["all_certified"]}

per_p = []
for p in ps:
    e = {"p": p, "runs": {a: len(V.cell(by, a, p)) for a in by}}
    cC, cL, cM, cR = (V.cell(by, a, p) for a in (C, L, M, R))
    pt = lambda r: r["rel_mul"] / r["trials"]
    e["per_trial"] = {"C": V.mean([pt(r) for r in cC]), "L": V.mean([pt(r) for r in cL]), "M": V.mean([pt(r) for r in cM])}
    e["L/C rel_mul (ratio of mean totals)"] = V.boot(cL, cC, lambda r: r["rel_mul"])
    e["M/C rel_mul (ratio of mean totals)"] = V.boot(cM, cC, lambda r: r["rel_mul"])
    e["L/C per-trial (ratio of mean per-trial)"] = e["per_trial"]["L"] / e["per_trial"]["C"]
    e["M/C per-trial (ratio of mean per-trial)"] = e["per_trial"]["M"] / e["per_trial"]["C"]
    for a, s in [(C, "C"), (L, "L"), (M, "M")]:
        for w in V.WEIGHTS:
            e[f"{s}/R w={w}"] = V.boot(V.cell(by, a, p), cR, lambda r, w=w: V.total(r, w))
            e[f"{s}/R w={w} incl adds"] = V.ratio_of_means(V.cell(by, a, p), cR, lambda r, w=w: V.total_all(r, w))
        e[f"la_share_{s}"] = V.mean([r["la_mul"] / (r["la_mul"] + r["rel_mul"]) for r in V.cell(by, a, p)])
        e[f"la_share_{s}_of_means"] = V.mean([r["la_mul"] for r in V.cell(by, a, p)]) / V.mean([r["native_mul"] for r in V.cell(by, a, p)])
    # LA alone vs rho at w = 0.1 (RESULTS: "LA alone is about 2.6x rho at w = 0.1" at 32771)
    e["M LA*0.1 / R total_w01"] = V.mean([0.1 * r["la_mul"] for r in cM]) / V.mean([V.total(r, 0.1) for r in cR])
    # M == L rows at fallback primes?
    e["M_rows_equal_L_rows"] = all(not V.compare_rows(by[M][k], by[L][k], ignore=V.NONDET | {"arm", "oracle"}) for k in keys if k[0] == p)
    per_p.append(e)
out["per_p"] = per_p
out["verdict_below_R_all_prices_point"] = {s: [e["p"] for e in per_p if all(e[f"{s}/R w={w}"][0] < 1 for w in V.WEIGHTS)] for s in "CLM"}
out["verdict_below_R_all_prices_upper"] = {s: [e["p"] for e in per_p if all(e[f"{s}/R w={w}"][2] < 1 for w in V.WEIGHTS)] for s in "CLM"}
out["below_R_w0_from"] = {s: [e["p"] for e in per_p if e[f"{s}/R w=0.0"][0] < 1] for s in "CLM"}
out["above_R_w01_and_w1_point"] = {s: [e["p"] for e in per_p if e[f"{s}/R w=0.1"][0] > 1 and e[f"{s}/R w=1.0"][0] > 1] for s in "CLM"}

# ---- claims transcribed from RESULTS.md ----
E = {e["p"]: e for e in per_p}
checks = []
def chk(label, claimed, actual, digits):
    ok = V.close(claimed, actual, digits=digits)
    checks.append({"claim": label, "claimed": claimed, "recomputed": round(actual, digits + 3), "ok": ok})
tab = {1031: (240.5, 115.2, 53.5, 0.479, 0.223), 4099: (273.5, 127.7, 57.7, 0.467, 0.210),
       16411: (334.6, 154.6, 67.0, 0.462, 0.202), 32771: (335.9, 152.4, 66.2, 0.454, 0.196),
       65537: (362.1, 177.6, 177.6, 0.490, 0.490)}
for p, (c, l, m, lc, mc) in tab.items():
    chk(f"per-trial C p={p}", c, E[p]["per_trial"]["C"], 1)
    chk(f"per-trial L p={p}", l, E[p]["per_trial"]["L"], 1)
    chk(f"per-trial M p={p}", m, E[p]["per_trial"]["M"], 1)
    chk(f"L/C p={p} (ratio of mean rel_mul)", lc, E[p]["L/C rel_mul (ratio of mean totals)"][0], 3)
    chk(f"M/C p={p} (ratio of mean rel_mul)", mc, E[p]["M/C rel_mul (ratio of mean totals)"][0], 3)
e2e = {4099: (10.43, 5.10, 56.24, 26.82), 16411: (7.54, 3.69, 40.73, 19.54),
       32771: (7.44, 3.69, 40.24, 19.57), 65537: (5.82, 3.97, 31.49, 21.30)}
for p, (c01, m01, c1, m1) in e2e.items():
    chk(f"C/R w=0.1 p={p}", c01, E[p]["C/R w=0.1"][0], 2)
    chk(f"M/R w=0.1 p={p}", m01, E[p]["M/R w=0.1"][0], 2)
    chk(f"C/R w=1 p={p}", c1, E[p]["C/R w=1.0"][0], 2)
    chk(f"M/R w=1 p={p}", m1, E[p]["M/R w=1.0"][0], 2)
chk("L/R w=0.1 p=65537 (L = M)", 3.97, E[65537]["L/R w=0.1"][0], 2)
chk("L/R w=1 p=65537", 21.30, E[65537]["L/R w=1.0"][0], 2)
chk("C/R w=0 p=65537 ('0.11x rho')", 0.11, E[65537]["C/R w=0.0"][0], 2)
chk("LA share C p=32771", 0.36, E[32771]["la_share_C"], 2)
chk("LA share L p=32771", 0.54, E[32771]["la_share_L"], 2)
chk("LA share M p=32771", 0.74, E[32771]["la_share_M"], 2)
chk("P5 LA share M p=65537 (54%)", 0.54, E[65537]["la_share_M"], 2)
chk("LA alone vs rho w=0.1 p=32771 (~2.6)", 2.6, E[32771]["M LA*0.1 / R total_w01"], 1)
out["claim_checks"] = checks
out["claim_mismatches"] = [c for c in checks if not c["ok"]]
out["claimed_G3_run_count"] = 696
out["actual_run_count"] = len(rows)
# L/C range claim "0.45-0.52 at every p >= 257"
out["L/C range p>=257"] = [min(E[p]["L/C rel_mul (ratio of mean totals)"][0] for p in ps if p >= 257),
                           max(E[p]["L/C rel_mul (ratio of mean totals)"][0] for p in ps if p >= 257)]
out["M/C range at cube-root primes"] = {p: E[p]["M/C rel_mul (ratio of mean totals)"][0] for p in ps}
out["component_rule L/C<=0.8 at p>=1031"] = all(E[p]["L/C rel_mul (ratio of mean totals)"][0] <= 0.8 for p in ps if p >= 1031)
json.dump(out, open(sys.argv[2], "w"), indent=1, default=str)
print(json.dumps({k: out[k] for k in ["n_rows", "n_per_arm", "instance_mismatch_across_arms"]}, default=str))
print("accounting violations:", len(out["accounting_violations"]), out["accounting_violations"][:5])
print("G1:", out["G1"]["compared_rows"], "compared,", out["G1"]["n_failures"], "failures", out["G1"]["failures"][:5])
print("G2:", out["G2"]["compared"], "compared,", out["G2"]["n_failures"], "failures", out["G2"]["failures"][:5])
print("G3: rows", out["G3"]["rows"], "uncertified", len(out["G3"]["uncertified"]))
print("verdict below at all prices (point):", out["verdict_below_R_all_prices_point"])
print("below R at w=0:", out["below_R_w0_from"])
print("above at w=0.1 and 1:", out["above_R_w01_and_w1_point"])
print("L/C range p>=257:", out["L/C range p>=257"], "component rule:", out["component_rule L/C<=0.8 at p>=1031"])
for e in per_p:
    print(e["p"], {k: (round(v[0], 3) if isinstance(v, tuple) else (round(v, 3) if isinstance(v, float) else v))
                   for k, v in e.items() if k in ("L/C rel_mul (ratio of mean totals)", "M/C rel_mul (ratio of mean totals)",
                                                  "L/C per-trial (ratio of mean per-trial)", "M/C per-trial (ratio of mean per-trial)", "M_rows_equal_L_rows")},
          "M/R w0.1", [round(x, 2) for x in e["M/R w=0.1"]], "M/R w1", [round(x, 2) for x in e["M/R w=1.0"]])
print("claim mismatches:")
for c in out["claim_mismatches"]:
    print("  ", c)
print(len(checks), "claims checked")
