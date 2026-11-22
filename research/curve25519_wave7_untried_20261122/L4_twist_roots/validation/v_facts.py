#!/usr/bin/env python3
"""(1) Curve facts by two independent methods: Euler-criterion character sum and x-only ladder point
orders (Hasse-interval uniqueness).  Usage: v_facts.py OUT.json"""
import json, math, random, sys, time
import numpy as np
sys.path.insert(0, __import__("os").path.dirname(__file__))
from vcore import CURVES, Tables, ladder, x_order, isprime_td, factor_td

out = {}
for k, (p, A) in CURVES.items():
    t0 = time.process_time()
    T = Tables(p, A)
    nZ, nE, nT = len(T.Zx), len(T.Ex), len(T.Tx)
    NE = 1 + nZ + 2 * nE            # infinity + 2-torsion x (y = 0) + two y per E-class x
    NT = 1 + nZ + 2 * nT
    rng = random.Random(f"validator-L4-{k}")
    lo, hi = p + 1 - 2 * math.isqrt(p) - 2, p + 1 + 2 * math.isqrt(p) + 2
    res = {"p": p, "A": A, "p_prime": isprime_td(p), "p_mod_8": p % 8,
           "n_x_Z": nZ, "n_x_E": nE, "n_x_T": nT, "Z_x": T.Zx.tolist(),
           "#E_charsum": NE, "#twist_charsum": NT, "sum_eq_2p+2": NE + NT == 2 * p + 2,
           "#E_factor": {str(q): e for q, e in factor_td(NE).items()},
           "#twist_factor": {str(q): e for q, e in factor_td(NT).items()},
           "A2-4_square": int(T.qr[(A * A - 4) % p]) == 1,
           "class_x=1": int(T.cls[1]), "class_x=-1": int(T.cls[p - 1])}
    r, rt = NE // 8, NT // 4
    res["r"] = r if NE % 8 == 0 else None
    res["r_twist"] = rt if NT % 4 == 0 else None
    res["r_prime"] = isprime_td(r)
    res["r_twist_prime"] = isprime_td(rt)
    # ladder method: point orders on E and on the twist (ladder is B-independent)
    for name, lst, N in (("E", T.Ex, NE), ("twist", T.Tx, NT)):
        orders = []
        for _ in range(8):
            x = int(lst[rng.randrange(len(lst))])
            orders.append(x_order(x, N, A, p))
        L = 1
        for o in orders:
            L = L * o // math.gcd(L, o)
        mults = [m for m in range(L * ((lo + L - 1) // L), hi + 1, L)]
        res[f"ladder_{name}"] = {"orders": orders, "lcm": L, "multiples_of_lcm_in_hasse": mults,
                                 "unique_and_equal": mults == [N]}
        # cross-check: the wrong group order must NOT kill a random point of the other class
        other = NT if name == "E" else NE
        x = int(lst[rng.randrange(len(lst))])
        res[f"ladder_{name}"]["killed_by_other_order"] = ladder(x, other, A, p)[1] == 0
    res["cpu_s"] = round(time.process_time() - t0, 2)
    out[k] = res
    print(k, json.dumps(res))
    del T
json.dump(out, open(sys.argv[1], "w"), indent=1)
