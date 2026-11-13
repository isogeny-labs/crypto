#!/usr/bin/env python3
"""Independent check of tower_large instances dumped from the committed build64:
E instance, and the genus-2 / genus-3 rung instances (validator's own Cantor)."""
import json, sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
dump = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
tl = json.load(open(sys.argv[2]))
byinst = {(i["p"], i["instance"]): i for i in tl["instances"]}
random.seed(int(sys.argv[3]) if len(sys.argv) > 3 else 1)
errs = []; n = 0
def rand_div(J, p, g):
    D = Jac2.ID
    for _ in range(g):
        while True:
            x = random.randrange(p); fx = peval(J.f, x, p)
            if fx and pow(fx, (p - 1) // 2, p) == 1: break
        y = next(y for y in range(1, p) if y * y % p == fx) if p < 5000 else tonelli(fx, p)
        D = J.add(D, ([(-x) % p, 1], [y]))
    return D
for d in dump:
    p, r, x = d["p"], d["r"], d["x"]
    E = EC(p, *d["e"]); G = tuple(d["gen"]); T = tuple(d["target"]) if d["target"] else None
    rec = byinst.get((p, d["instance"]))
    chk = {"p_prime": is_prime(p), "r_prime": is_prime(r), "G_on_E": E.on(G), "rG_zero": E.mul(r, G) is None,
           "xG_eq_T": E.mul(x, G) == T, "r_ge_p_over_4": r >= p // 4,
           "n_e_count": E.count() == d["n_e"], "r_divides_n_e": d["n_e"] % r == 0,
           "matches_tower_large_json": rec is not None and rec["r"] == r and rec["n_e"] == d["n_e"] and rec["seed"] == d["seed"]
               and list(rec["jac_order"]) == [rg["jac_order"] for rg in d["rungs"]]}
    for i, rg in enumerate(d["rungs"]):
        g = rg["genus"]; J = JacG(p, rg["f"], g)
        D = (rg["d"]["u"], rg["d"]["v"]); Tt = (rg["t"]["u"], rg["t"]["v"])
        N = rg["jac_order"]
        chk[f"g{g}_D_valid"] = J.valid(D) and J.valid(Tt)
        chk[f"g{g}_D_nonzero"] = not J.is_id(D)
        chk[f"g{g}_rD_zero"] = J.is_id(J.mul(r, D))
        chk[f"g{g}_xD_eq_T"] = J.norm(J.mul(x, D)) == J.norm(Tt)
        chk[f"g{g}_r_divides_order"] = N % r == 0
        # Hasse-Weil window for #J: (sqrt(p)-1)^(2g) <= N <= (sqrt(p)+1)^(2g)
        chk[f"g{g}_order_in_HW_window"] = (p ** 0.5 - 1) ** (2 * g) <= N <= (p ** 0.5 + 1) ** (2 * g)
        chk[f"g{g}_order_kills_random"] = all(J.is_id(J.mul(N, rand_div(J, p, g))) for _ in range(3))
    bad = [k for k, v in chk.items() if v is not True]
    if bad: errs.append((p, d["instance"], bad))
    n += 1
print(f"tower instances checked {n}; errors {len(errs)}")
for e in errs: print("  ", e)
