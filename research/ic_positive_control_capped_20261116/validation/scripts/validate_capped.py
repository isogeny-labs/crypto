#!/usr/bin/env python3
"""Independent recomputation for research/ic_positive_control_capped_20261116.
Usage: python3 -I validate_capped.py REPO_ROOT OUT_JSON"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V

ROOT = sys.argv[1]
D = os.path.join(ROOT, "research/ic_positive_control_capped_20261116")
_, w_rows = V.load(os.path.join(D, "run_w.json"))
sr_run, sr_rows = V.load(os.path.join(D, "run_sr.json"))
_, prev_rows = V.load(os.path.join(ROOT, "research/ic_positive_control_unit_t_20261116/run.json"))
_, ctl_rows = V.load(os.path.join(ROOT, "research/ic_positive_control_20261112/control.json"))
wb, sb, pb, cb = V.index(w_rows), V.index(sr_rows), V.index(prev_rows), V.index(ctl_rows)
W, S, R, C = "C_unit_t_capped_fast", "C_coord_square", "R_rho_negation", "C_double_lp"
assert set(wb) == {W} and set(sb) == {S, R}
out = {"study": "capped", "n_w": len(wb[W]), "n_sr": {a: len(sb[a]) for a in sb}}
out["accounting_violations"] = V.accounting_check(w_rows) + V.accounting_check(sr_rows)
out["W_modes"] = sorted({(r["oracle"], r["la_mode"], r["walk"]) for r in wb[W].values()})
# G1: run_sr rows equal unit-T rows (all deterministic fields)
g1 = []
for a in (S, R):
    for k, r in sorted(sb[a].items()):
        o = pb[a].get(k)
        if o is None:
            g1.append(f"{a} {k}: no unit-T row")
            continue
        d = V.compare_rows(r, o)
        if d:
            g1.append(f"{a} {k}: {d[:4]}")
out["G1"] = {"compared": {a: len(sb[a]) for a in sb}, "primes": sorted({k[0] for k in sb[S]}), "failures": g1[:20], "n_failures": len(g1)}
# expected coverage of run_sr: all unit-T keys with p <= 8209
exp = {k for k in pb[S] if k[0] <= 8209}
out["G1"]["coverage_complete"] = set(sb[S]) == exp and set(sb[R]) == exp
# G2
missing = [k for k in wb[W] if k not in pb[S] or k not in pb[R]]
out["G2"] = {"W_rows": len(wb[W]), "uncertified": [k for k, r in wb[W].items() if r["certified"] is not True],
             "missing_partners": missing, "W_keys_equal_unitT_keys": set(wb[W]) == set(pb[S])}
# W instances match S/R instances
out["instance_mismatch"] = [k for k in wb[W] if (wb[W][k]["order"], wb[W][k]["r"], wb[W][k]["h"]) != (pb[S][k]["order"], pb[S][k]["r"], pb[S][k]["h"])]
ps = sorted({k[0] for k in wb[W]})
byS = {S: pb[S], R: pb[R], W: wb[W]}
fb = {p: sum(r["la_fallbacks"] for r in V.cell(byS, W, p)) for p in ps}
out["W_fallbacks_by_p"] = fb
out["W_fallbacks_total"] = sum(fb.values())
out["W_runs_with_fallback_by_p"] = {p: sum(1 for r in V.cell(byS, W, p) if r["la_fallbacks"] > 0) for p in ps}
pt = lambda r: r["rel_mul"] / r["trials"]
per_p = []
for p in ps:
    cW, cS, cR = (V.cell(byS, a, p) for a in (W, S, R))
    e = {"p": p, "runs": len(cW)}
    e["W/S la_mul"] = V.boot(cW, cS, lambda r: r["la_mul"])
    e["W/S native_all"] = V.boot(cW, cS, lambda r: r["native_all"])
    e["W/S la incl adds"] = V.boot(cW, cS, lambda r: r["la_mul"] + r["la_add"])
    e["W/S rel per trial"] = V.boot(cW, cS, pt)
    for w in V.WEIGHTS:
        e[f"W/S w={w}"] = V.boot(cW, cS, lambda r, w=w: V.total(r, w))
        e[f"W/R w={w}"] = V.boot(cW, cR, lambda r, w=w: V.total(r, w))
        e[f"S/R w={w}"] = V.boot(cS, cR, lambda r, w=w: V.total(r, w))
        e[f"W/R w={w} incl adds"] = V.ratio_of_means(cW, cR, lambda r, w=w: V.total_all(r, w))
    if p in {k[0] for k in cb[C]}:
        cC, cRc = V.cell(cb, C, p), V.cell(cb, R, p)
        for w in V.WEIGHTS:
            e[f"C1112/R w={w}"] = V.ratio_of_means(cC, cRc, lambda r, w=w: V.total(r, w))
    per_p.append(e)
out["per_p"] = per_p
E = {e["p"]: e for e in per_p}
big = [p for p in ps if p >= 4099]
out["advances_every_p>=4099 (point)"] = {p: (E[p]["W/S w=0.1"][0], E[p]["W/S w=1.0"][0]) for p in big}
out["advances_upper<1"] = {p: (E[p]["W/S w=0.1"][2] < 1, E[p]["W/S w=1.0"][2] < 1) for p in big}
s01 = V.slope(big, [E[p]["W/R w=0.1"][0] for p in big])
s1 = V.slope(big, [E[p]["W/R w=1.0"][0] for p in big])
def cross(w):
    xs = [math.log(p) for p in big]
    ys = [math.log(E[p][f"W/R w={w}"][0]) for p in big]
    mx, my = V.mean(xs), V.mean(ys)
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    a = my - b * mx
    return math.exp(-a / b) if b < 0 else None
out["slope_W/R_w0.1"] = s01
out["slope_W/R_w1"] = s1
out["crossover_p_w0.1"] = cross(0.1)
out["crossover_p_w1"] = cross(1.0)
out["crossover_log2_w0.1"] = math.log2(out["crossover_p_w0.1"]) if out["crossover_p_w0.1"] else None
out["crossover_log2_w1"] = math.log2(out["crossover_p_w1"]) if out["crossover_p_w1"] else None
out["W_below_R_all_prices"] = [p for p in ps if all(E[p][f"W/R w={w}"][0] < 1 for w in V.WEIGHTS)]
out["W_w01_upper_below_1"] = [p for p in ps if E[p]["W/R w=0.1"][2] < 1]
out["W_below_R_w0"] = [p for p in ps if E[p]["W/R w=0.0"][0] < 1]
out["W/R w0 p>=16411"] = {p: E[p]["W/R w=0.0"][0] for p in ps if p >= 16411}
out["W/S small p w0.1"] = {p: E[p]["W/S w=0.1"] for p in ps if p <= 2053}

checks = []
def chk(label, claimed, actual, digits):
    checks.append({"claim": label, "claimed": claimed, "recomputed": round(actual, digits + 3), "ok": V.close(claimed, actual, digits=digits)})
tab = {4099: (0.742, 0.958, 0.999, 0.898, 0.890), 8209: (0.777, 0.867, 0.771, 0.786, 0.776),
       16411: (0.760, 0.962, 0.999, 0.898, 0.891), 32771: (0.772, 0.954, 1.000, 0.895, 0.889),
       65537: (0.786, 0.892, 0.823, 0.821, 0.815), 131101: (0.791, 0.965, 1.000, 0.902, 0.897)}
for p, (la, laa, rel, t01, t1) in tab.items():
    chk(f"W/S LA p={p}", la, E[p]["W/S la_mul"][0], 3)
    chk(f"W/S 'LA incl additions' p={p} (native_all)", laa, E[p]["W/S native_all"][0], 3)
    chk(f"W/S rel per trial p={p}", rel, E[p]["W/S rel per trial"][0], 3)
    chk(f"W/S total w=0.1 p={p}", t01, E[p]["W/S w=0.1"][0], 3)
    chk(f"W/S total w=1 p={p}", t1, E[p]["W/S w=1.0"][0], 3)
e2e = {16411: (7.54, 1.97, 1.77, 40.7, 10.1, 8.97), 32771: (7.44, 1.92, 1.72, 40.2, 9.85, 8.75),
       65537: (5.82, 2.57, 2.11, 31.5, 13.6, 11.1), 131101: (None, 1.97, 1.78, None, 10.2, 9.17)}
for p, (c01, s01_, w01, c1, s1_, w1) in e2e.items():
    if c01 is not None:
        chk(f"C(11-12)/R w=0.1 p={p}", c01, E[p]["C1112/R w=0.1"], 2)
        chk(f"C(11-12)/R w=1 p={p}", c1, E[p]["C1112/R w=1.0"], 1)
    chk(f"S/R w=0.1 p={p}", s01_, E[p]["S/R w=0.1"][0], 2)
    chk(f"W/R w=0.1 p={p}", w01, E[p]["W/R w=0.1"][0], 2)
    chk(f"S/R w=1 p={p}", s1_, E[p]["S/R w=1.0"][0], 2 if s1_ < 10 else 1)
    chk(f"W/R w=1 p={p}", w1, E[p]["W/R w=1.0"][0], 2 if w1 < 10 else 1)
chk("slope ln(W/R w0.1)", -0.161, s01, 3)
chk("P1 fallbacks total (26)", 26, out["W_fallbacks_total"], 0)
out["claim_checks"] = checks
out["claim_mismatches"] = [c for c in checks if not c["ok"]]
out["claimed_intervals"] = {"W/S w0.1": {4099: (0.878, 0.917), 8209: (0.769, 0.819), 16411: (0.888, 0.907), 32771: (0.890, 0.901), 65537: (0.815, 0.827), 131101: (0.899, 0.906)},
                            "W/R w0.1": {16411: (1.03, 3.55), 32771: (0.95, 3.32), 65537: (1.39, 3.41), 131101: (1.15, 4.53)}}
out["own_intervals"] = {"W/S w0.1": {p: E[p]["W/S w=0.1"][1:] for p in big}, "W/S w1": {p: E[p]["W/S w=1.0"][1:] for p in big},
                        "W/R w0.1": {p: E[p]["W/R w=0.1"][1:] for p in (16411, 32771, 65537, 131101)}}
json.dump(out, open(sys.argv[2], "w"), indent=1, default=str)
print("W rows", out["n_w"], "SR rows", out["n_sr"], "modes", out["W_modes"])
print("accounting violations", len(out["accounting_violations"]), out["accounting_violations"][:3])
print("G1", out["G1"]["compared"], out["G1"]["primes"], "fails", out["G1"]["n_failures"], out["G1"]["failures"][:3], "coverage", out["G1"]["coverage_complete"])
print("G2", out["G2"]["W_rows"], "uncert", len(out["G2"]["uncertified"]), "missing", len(out["G2"]["missing_partners"]), "keys equal", out["G2"]["W_keys_equal_unitT_keys"])
print("instance mismatch", out["instance_mismatch"][:3])
print("W fallbacks", fb, "total", out["W_fallbacks_total"], "runs with fb", out["W_runs_with_fallback_by_p"])
print("advances point", {p: tuple(round(x, 3) for x in v) for p, v in out["advances_every_p>=4099 (point)"].items()})
print("advances upper<1", out["advances_upper<1"])
print("slope w0.1", s01, "slope w1", s1, "cross log2 w0.1", out["crossover_log2_w0.1"], "w1", out["crossover_log2_w1"])
print("W below all prices", out["W_below_R_all_prices"], "w0.1 upper<1", out["W_w01_upper_below_1"], "W below w0", out["W_below_R_w0"])
print("W/R w0 p>=16411", {p: round(v, 3) for p, v in out["W/R w0 p>=16411"].items()})
print("W/S small p", {p: [round(x, 3) for x in v] for p, v in out["W/S small p w0.1"].items()})
print("W/S rel per trial", {e['p']: round(e['W/S rel per trial'][0], 3) for e in per_p})
print("W/S LA-only incl adds", {e['p']: round(e['W/S la incl adds'][0], 3) for e in per_p})
print("own intervals", json.dumps(out["own_intervals"], default=lambda x: [round(y, 3) for y in x]))
print("claim mismatches:")
for c in out["claim_mismatches"]:
    print("  ", c)
print(len(checks), "claims checked")
