#!/usr/bin/env python3
"""Independent recomputation for research/ic_positive_control_la_20261116.
Usage: python3 -I validate_la.py REPO_ROOT OUT_JSON"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vlib as V

ROOT = sys.argv[1]
D = os.path.join(ROOT, "research/ic_positive_control_la_20261116")
run, rows = V.load(os.path.join(D, "run.json"))
prev, prev_rows = V.load(os.path.join(ROOT, "research/ic_positive_control_lucas_20261116/run.json"))
by, pb = V.index(rows), V.index(prev_rows)
M, N, S, R = "C_lucas_roots", "C_coord", "C_coord_square", "R_rho_negation"
assert set(by) == {M, N, S, R}, set(by)
keys = set(by[M])
assert all(set(by[a]) == keys for a in by)
out = {"study": "la", "n_rows": len(rows), "n_per_arm": {a: len(by[a]) for a in by}}
out["accounting_violations"] = V.accounting_check(rows)
out["instance_mismatch_across_arms"] = [k for k in keys if len({(by[a][k]["order"], by[a][k]["h"], by[a][k]["r"]) for a in by}) != 1]
# modes recorded per arm
out["modes"] = {a: sorted({(r["oracle"], r["la_mode"]) for r in by[a].values()}) for a in (M, N, S)}

# G1: M reproduces the Lucas study's M rows; R too (not a protocol gate, extra)
g1 = []
for k, r in sorted(by[M].items()):
    o = pb[M][k]
    for f in ["group_ops", "rel_mul", "la_mul", "trials", "relations"]:
        if r[f] != o[f]:
            g1.append(f"M {k} {f}: {r[f]} != {o[f]}")
    extra = V.compare_rows(r, o, ignore=V.NONDET | {"la_mode", "la_fallbacks"})
    if extra:
        g1.append(f"M {k} other fields: {extra[:4]}")
rx = []
for k, r in sorted(by[R].items()):
    d = V.compare_rows(r, pb[R][k])
    if d:
        rx.append(f"R {k}: {d[:3]}")
out["G1"] = {"compared": len(by[M]), "failures": g1[:20], "n_failures": len(g1), "extra_R_identity_failures": rx[:10], "n_R_compared": len(by[R])}
# G2: N and S have M's trials and relations (and, as RESULTS states, rel_mul)
g2 = []
for a in (N, S):
    for k in sorted(keys):
        for f in ["trials", "relations", "rel_mul", "splits", "fb_cols"]:
            if by[a][k][f] != by[M][k][f]:
                g2.append(f"{a} {k} {f}: {by[a][k][f]} != {by[M][k][f]}")
out["G2"] = {"compared": 2 * len(keys), "failures": g2[:20], "n_failures": len(g2)}
out["G3"] = {"rows": len(rows), "uncertified": [(r["arm"], r["p"], r["curve_idx"], r["seed"]) for r in rows if r["certified"] is not True]}
out["fallbacks_total"] = {a: sum(r["la_fallbacks"] for r in by[a].values()) for a in (M, N, S)}
out["N_and_S_runs"] = len(by[N]) + len(by[S])
# group ops: N/S differ from M only by verification?
gdiff = []
for a in (N, S):
    for k in keys:
        x, m = by[a][k], by[M][k]
        if x["setup_ops"] != m["setup_ops"] or x["walk_ops"] != m["walk_ops"]:
            gdiff.append((a, k, "setup/walk differ"))
out["N_S_setup_walk_equal_M"] = not gdiff

ps = sorted({k[0] for k in keys})
per_p = []
for p in ps:
    cM, cN, cS, cR = (V.cell(by, a, p) for a in (M, N, S, R))
    e = {"p": p, "runs": len(cM)}
    e["M_la_mul_mean"] = V.mean([r["la_mul"] for r in cM])
    e["N/M la_mul"] = V.boot(cN, cM, lambda r: r["la_mul"])
    e["S/M la_mul"] = V.boot(cS, cM, lambda r: r["la_mul"])
    e["S/M la incl adds"] = V.boot(cS, cM, lambda r: r["la_mul"] + r["la_add"])
    e["S/M native_all"] = V.boot(cS, cM, lambda r: r["native_all"])
    e["fallbacks N,S"] = (sum(r["la_fallbacks"] for r in cN), sum(r["la_fallbacks"] for r in cS))
    for a, s in [(M, "M"), (N, "N"), (S, "S")]:
        for w in V.WEIGHTS:
            e[f"{s}/R w={w}"] = V.boot(V.cell(by, a, p), cR, lambda r, w=w: V.total(r, w))
            e[f"{s}/R w={w} incl adds"] = V.ratio_of_means(V.cell(by, a, p), cR, lambda r, w=w: V.total_all(r, w))
    e["la_share_S"] = V.mean([r["la_mul"] / r["native_mul"] for r in cS])
    per_p.append(e)
out["per_p"] = per_p
E = {e["p"]: e for e in per_p}
out["verdict_below_all_prices_point"] = {s: [p for p in ps if all(E[p][f"{s}/R w={w}"][0] < 1 for w in V.WEIGHTS)] for s in "MNS"}
out["S_below_R_w0"] = [p for p in ps if E[p]["S/R w=0.0"][0] < 1]
out["S_above_R_w01_w1"] = [p for p in ps if E[p]["S/R w=0.1"][0] > 1 and E[p]["S/R w=1.0"][0] > 1]
out["S/M la ranges all p"] = [min(E[p]["S/M la_mul"][0] for p in ps), max(E[p]["S/M la_mul"][0] for p in ps)]
out["S/M native_all ranges all p"] = [min(E[p]["S/M native_all"][0] for p in ps), max(E[p]["S/M native_all"][0] for p in ps)]
out["S/M la incl adds ranges all p"] = [min(E[p]["S/M la incl adds"][0] for p in ps), max(E[p]["S/M la incl adds"][0] for p in ps)]
out["la_share_S range"] = [min(E[p]["la_share_S"] for p in ps), max(E[p]["la_share_S"] for p in ps)]
out["S/R w0.1 by p"] = {p: E[p]["S/R w=0.1"][0] for p in ps}

checks = []
def chk(label, claimed, actual, digits):
    checks.append({"claim": label, "claimed": claimed, "recomputed": round(actual, digits + 3), "ok": V.close(claimed, actual, digits=digits)})
comp = {4099: (2.67e6, 0.669, 0.323, 0.405), 16411: (1.80e7, 0.667, 0.326, 0.394),
        32771: (4.63e7, 0.667, 0.328, 0.388), 65537: (1.20e8, 0.667, 0.329, 0.453)}
for p, (mla, nm, sm, sma) in comp.items():
    checks.append({"claim": f"M LA mult p={p}", "claimed": mla, "recomputed": E[p]["M_la_mul_mean"],
                   "ok": abs(E[p]["M_la_mul_mean"] - mla) <= 0.005 * 10 ** math.floor(math.log10(mla))})
    chk(f"N/M LA p={p}", nm, E[p]["N/M la_mul"][0], 3)
    chk(f"S/M LA p={p}", sm, E[p]["S/M la_mul"][0], 3)
    chk(f"S/M incl additions p={p} (native_all)", sma, E[p]["S/M native_all"][0], 3)
e2e = {4099: (5.10, 2.77, 26.8, 14.0), 16411: (3.69, 1.97, 19.5, 10.1), 32771: (3.69, 1.92, 19.6, 9.85), 65537: (3.97, 2.57, 21.3, 13.6)}
for p, (m01, s01, m1, s1) in e2e.items():
    chk(f"M/R w=0.1 p={p}", m01, E[p]["M/R w=0.1"][0], 2)
    chk(f"S/R w=0.1 p={p}", s01, E[p]["S/R w=0.1"][0], 2)
    d1 = 1 if m1 < 20 else 1
    chk(f"M/R w=1 p={p}", m1, E[p]["M/R w=1.0"][0], 1)
    chk(f"S/R w=1 p={p}", s1, E[p]["S/R w=1.0"][0], 2 if s1 < 10 else 1)
out["claim_checks"] = checks
out["claim_mismatches"] = [c for c in checks if not c["ok"]]
out["claimed_intervals"] = {"S/R w0.1": {4099: (1.35, 8.20), 16411: (1.15, 3.96), 32771: (1.08, 3.71), 65537: (1.70, 4.14)},
                            "S/R w1": {4099: (6.8, 41.6), 16411: (5.9, 20.3), 32771: (5.55, 19.0), 65537: (9.0, 21.9)}}
out["own_intervals"] = {"S/R w0.1": {p: E[p]["S/R w=0.1"][1:] for p in comp}, "S/R w1": {p: E[p]["S/R w=1.0"][1:] for p in comp}}

# training: recompute the score from train_cc*.json
tr = {}
for cc in ["0.7", "1.0", "1.4", "2.0"]:
    td, trows = V.load(os.path.join(D, f"train_cc{cc}.json"))
    tb = V.index(trows)
    assert set(tb) == {S, R}, set(tb)
    assert td["cfg"]["cc"] == float(cc) and td["cfg"]["curve_seed0"] == 5001
    logs = []
    for k, r in tb[S].items():
        logs.append(math.log(V.total(r, 0.1) / V.total(tb[R][k], 0.1)))
    tr[cc] = {"n": len(logs), "uncertified": sum(1 for r in trows if r["certified"] is not True),
              "score": math.exp(sum(logs) / len(logs)), "primes": sorted({k[0] for k in tb[S]}),
              "fb_cols_by_p": {p: sorted({r["fb_cols"] for k, r in tb[S].items() if k[0] == p}) for p in sorted({k[0] for k in tb[S]})}}
out["training"] = tr
best = min(tr, key=lambda c: (tr[c]["score"], -float(c)))
out["training_cc_star"] = best
claimed_scores = {"0.7": 4.073, "1.0": 3.270, "1.4": 2.876, "2.0": 2.843}
out["training_score_mismatch"] = {c: (claimed_scores[c], tr[c]["score"]) for c in tr if not V.close(claimed_scores[c], tr[c]["score"], digits=3)}
# test curves disjoint from training curves? compare instance orders at shared primes
test_inst = {(r["p"], r["order"]) for r in rows}
train_inst = set()
for cc in ["2.0"]:
    _, trows = V.load(os.path.join(D, f"train_cc{cc}.json"))
    train_inst |= {(r["p"], r["order"]) for r in trows}
out["train_test_instance_overlap"] = sorted(test_inst & train_inst)

json.dump(out, open(sys.argv[2], "w"), indent=1, default=str)
print("rows", out["n_rows"], out["n_per_arm"], "modes", out["modes"])
print("accounting violations:", len(out["accounting_violations"]), out["accounting_violations"][:3])
print("G1:", out["G1"]["compared"], out["G1"]["n_failures"], out["G1"]["failures"][:3], "R identity fails:", len(out["G1"]["extra_R_identity_failures"]))
print("G2:", out["G2"]["compared"], out["G2"]["n_failures"], out["G2"]["failures"][:3])
print("G3:", out["G3"]["rows"], "uncertified", len(out["G3"]["uncertified"]))
print("fallbacks", out["fallbacks_total"], "N+S runs", out["N_and_S_runs"], "N/S setup+walk equal M:", out["N_S_setup_walk_equal_M"])
print("verdict below all prices:", out["verdict_below_all_prices_point"])
print("S below R w0:", out["S_below_R_w0"], "S above w0.1&w1:", out["S_above_R_w01_w1"])
print("S/M la range", out["S/M la ranges all p"], "S/M native_all range", out["S/M native_all ranges all p"], "S/M la incl adds", out["S/M la incl adds ranges all p"])
print("LA share of S range", out["la_share_S range"])
print("S/R w0.1 by p", {p: round(v, 2) for p, v in out["S/R w0.1 by p"].items()})
print("own intervals", json.dumps(out["own_intervals"], default=lambda x: [round(y, 2) for y in x]))
print("training", {c: (round(v["score"], 3), v["n"], v["uncertified"]) for c, v in tr.items()}, "cc*", best, "mismatch", out["training_score_mismatch"])
print("train/test overlap", out["train_test_instance_overlap"])
print("claim mismatches:")
for c in out["claim_mismatches"]:
    print("  ", c)
print(len(checks), "claims checked")
