#!/usr/bin/env python3
"""Independent re-verification of chained.solve.certificate/v1 records (target 1)."""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *

certs_path, results_path = sys.argv[1], sys.argv[2]
res = json.load(open(results_path))
rows = {r["p"]: r for r in res["rows"]}
out = []; nfail = 0
certs = [json.loads(l) for l in open(certs_path) if l.strip()]
by_p = {}
for i, c in enumerate(certs, 1):
    chk = {}
    p, r, k = c["p"], c["r"], c["claimed_k"]
    row = rows[p]
    chk["schema"] = c["schema"] == "chained.solve.certificate/v1"
    chk["p_prime"] = is_prime(p); chk["r_prime"] = is_prime(r)
    chk["r_matches_results"] = r == row["r"]
    chk["k_present_lt_r"] = k is not None and 0 <= k < r
    chk["k_equals_planted_x"] = k == row["x"]
    gd = c["group"]
    if gd["kind"] == "ec":
        E = EC(p, gd["a2"], gd["a4"], gd["a6"])
        G = tuple(c["g"]) if c["g"] else None; T = tuple(c["t"]) if c["t"] else None
        chk["G_on_curve"] = E.on(G); chk["T_on_curve"] = E.on(T)
        chk["G_nonzero"] = G is not None
        chk["rG_zero"] = E.mul(r, G) is None
        chk["kG_eq_T"] = E.mul(k, G) == T
        if c["arm"] == "rho_e1":
            n = E.count()
            chk["E1_order_matches_results"] = n == row["e1_order"]
            chk["r_divides_E1"] = n % r == 0
            chk["r2_not_divides_E1"] = n % (r * r) != 0
    else:
        J = Jac2(p, gd["f"])
        G = (c["g"]["u"], c["g"]["v"]); T = (c["t"]["u"], c["t"]["v"])
        chk["G_valid"] = J.valid(G); chk["T_valid"] = J.valid(T)
        chk["G_nonzero"] = not J.is_id(G)
        chk["rG_zero"] = J.is_id(J.mul(r, G))
        chk["kG_eq_T"] = J.norm(J.mul(k, G)) == J.norm(T)
    tr = c["transfer"]
    if tr is not None and tr["kind"] == "isogeny":
        sd = tr["src"]; Es = EC(p, sd["a2"], sd["a4"], sd["a6"])
        sg, st = tuple(tr["src_g"]), tuple(tr["src_t"])
        K = tuple(tr["kernel"])
        ell = row["isog_degree"]
        chk["src_points_on_curve"] = Es.on(sg) and Es.on(st) and Es.on(K)
        chk["kernel_order_eq_degree"] = Es.mul(ell, K) is None and K is not None and is_prime(ell)
        chk["src_rG_zero"] = Es.mul(r, sg) is None
        chk["src_kG_eq_T"] = Es.mul(k, sg) == st
        E2, phi = velu(Es, K, ell)
        chk["velu_codomain_matches_group"] = (E2.a2, E2.a4, E2.a6) == (E.a2, E.a4, E.a6)
        chk["velu_image_g"] = phi(sg) == G
        chk["velu_image_t"] = phi(st) == T
        chk["src_is_rho_e1_curve"] = None  # filled below
    if tr is not None and tr["kind"] == "pullback":
        x0, g5, E1, E2 = bielliptic(p, tr["c"])
        chk["quintic_model_matches_group_f"] = [v % p for v in g5] == [v % p for v in gd["f"]]
        sg, st = tuple(tr["src_g"]), tuple(tr["src_t"])
        chk["src_points_on_E1"] = E1.on(sg) and E1.on(st)
        chk["src_rG_zero"] = E1.mul(r, sg) is None
        chk["src_kG_eq_T"] = E1.mul(k, sg) == st
        chk["pullback_g"] = J.norm(pullback_e1(p, tr["c"], x0, sg)) == J.norm(G)
        chk["pullback_t"] = J.norm(pullback_e1(p, tr["c"], x0, st)) == J.norm(T)
        chk["E2_order_matches_results"] = E2.count() == row["e2_order"]
        chk["E1_order_matches_results"] = E1.count() == row["e1_order"]
        chk["r_not_divides_E2"] = row["e2_order"] % r != 0
        chk["jac_order_eq_e1e2"] = row["jac_order"] == row["e1_order"] * row["e2_order"]
        chk["E1_is_rho_e1_curve"] = None
        chk["_E1"] = (E1.a2, E1.a4, E1.a6)
    by_p.setdefault(p, []).append((i, c, chk))
# cross-cert consistency per p
for p, lst in by_p.items():
    e1 = [c for (_, c, _) in lst if c["arm"] == "rho_e1"][0]
    e1g = (e1["group"]["a2"] % p, e1["group"]["a4"] % p, e1["group"]["a6"] % p)
    for (i, c, chk) in lst:
        if "src_is_rho_e1_curve" in chk:
            sd = c["transfer"]["src"]
            chk["src_is_rho_e1_curve"] = (sd["a2"] % p, sd["a4"] % p, sd["a6"] % p) == e1g and \
                tuple(c["transfer"]["src_g"]) == tuple(e1["g"]) and tuple(c["transfer"]["src_t"]) == tuple(e1["t"])
        if "E1_is_rho_e1_curve" in chk:
            E1t = chk.pop("_E1")
            chk["E1_is_rho_e1_curve"] = E1t == e1g and tuple(c["transfer"]["src_g"]) == tuple(e1["g"]) and tuple(c["transfer"]["src_t"]) == tuple(e1["t"])
        ok = all(v is True for v in chk.values())
        nfail += (not ok)
        out.append({"line": i, "arm": c["arm"], "p": p, "r": c["r"], "k": c["claimed_k"], "ok": ok,
                    "solver_ops": c["solver_ops"], "checks": chk})
for o in out:
    bad = [k for k, v in o["checks"].items() if v is not True]
    print(f"line {o['line']:2d} {o['arm']:12s} p={o['p']:5d} r={o['r']:4d} k={o['k']} {'OK' if o['ok'] else 'FAIL '+str(bad)} ({len(o['checks'])} checks)")
print(f"certificates {len(out)}, verified {len(out)-nfail}, failed {nfail}")
json.dump(out, open(sys.argv[3], "w"), indent=1, sort_keys=True) if len(sys.argv) > 3 else None
