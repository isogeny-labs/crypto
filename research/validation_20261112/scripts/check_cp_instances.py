#!/usr/bin/env python3
"""Independent check of cover_pushforward instances (dumped from the committed
selection code) and of every recorded log in pushforward.json / pushforward_15seeds.json."""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
dump = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
docs = [json.load(open(a)) for a in sys.argv[2:4]]
M = (1 << 64) - 1
inst = {(d["p"], d["k"]): d for d in dump}
errs = []; n_inst = 0; n_logs = 0
for d in dump:
    p, r, x = d["p"], d["r"], d["x"]
    c = d["coeffs"]
    x0, g5, E1, E2 = bielliptic(p, c)
    G = tuple(d["g_e"]); T = tuple(d["t_e"]) if d["t_e"] else None
    J = Jac2(p, d["jac_f"])
    chk = {
        "p_prime": is_prime(p), "r_prime": is_prime(r),
        "e1_model": (E1.a2, E1.a4, E1.a6) == tuple(v % p for v in d["e1"]),
        "jac_quintic": [v % p for v in g5] == [v % p for v in d["jac_f"]],
        "x_formula": x == ((d["seed"] * 2246822519 + 7) & M) % r,
        "G_on_E1": E1.on(G) and G is not None, "rG_zero": E1.mul(r, G) is None,
        "T_eq_xG": E1.mul(x, G) == T,
        "pull_G": J.norm(pullback_e1_any(p, c, x0, G, E1, J, aux=G)) == J.norm((d["g"]["u"], d["g"]["v"])),
        "pull_T": J.norm(pullback_e1_any(p, c, x0, T, E1, J, aux=G)) == J.norm((d["t"]["u"], d["t"]["v"])),
        "jac_rG_zero": J.is_id(J.mul(r, (d["g"]["u"], d["g"]["v"]))),
        "jac_xG_eq_T": J.norm(J.mul(x, (d["g"]["u"], d["g"]["v"]))) == J.norm((d["t"]["u"], d["t"]["v"])),
    }
    if E1.p < 20000:
        n1 = E1.count(); chk["r_divides_E1"] = n1 % r == 0
    bad = [k for k, v in chk.items() if v is not True]
    if bad: errs.append((p, d["k"], bad))
    n_inst += 1
for name, doc in zip(("primary", "15seeds"), docs):
    for row in doc["rows"]:
        p = row["p"]
        for k, s in enumerate(row["seeds"]):
            d = inst.get((p, k))
            if d is None: errs.append((name, p, k, "no dump")); continue
            if s["seed"] != d["seed"] or s["x"] != d["x"] or row["r"] != d["r"] or row["coeffs"] != d["coeffs"]:
                errs.append((name, p, k, "instance mismatch")); continue
            E1 = EC(p, *d["e1"]); G = tuple(d["g_e"]); T = tuple(d["t_e"])
            J = Jac2(p, d["jac_f"]); DG = (d["g"]["u"], d["g"]["v"]); DT = (d["t"]["u"], d["t"]["v"])
            logs = {"ic_jc": s["ic_jc"]["recovered"], "ic_jc_first_collision": s["ic_jc"]["first_collision"]["log"],
                    "ic_e1_fibre": s["ic_e1_fibre"]["run"]["recovered"], "ic_e1_fibre_first_collision": s["ic_e1_fibre"]["first_collision"]["log"],
                    "ic_e1_two_term": s["ic_e1_two_term"]["run"]["recovered"], "birthday_e1": s["birthday_e1"]["log"]}
            for arm, kk in logs.items():
                n_logs += 1
                if kk is None or E1.mul(kk, G) != T: errs.append((name, p, k, arm, "EC cert fails"))
            n_logs += 1
            if J.norm(J.mul(logs["ic_jc"], DG)) != J.norm(DT): errs.append((name, p, k, "jac cert fails"))
print(f"instances checked {n_inst}; recorded logs certified {n_logs}; errors {len(errs)}")
for e in errs[:20]: print("  ", e)
