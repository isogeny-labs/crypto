#!/usr/bin/env python3
"""Independent instance check (no producer code): for every prime of a run file,
walk curve seeds from curve_seed0 with the re-implemented generator and, for
each committed curve (in curve_idx order), find the first later seed whose
squarefree f satisfies [order]D = 0 for 4 random divisors (validator Cantor
arithmetic). Also check r prime, h*r = order, r > h, h <= max_h, h the smallest
such cofactor, order inside the Hasse-Weil interval.
Usage: check_instances.py RUN_JSON [RUN_JSON...] --out OUT"""
import json, math, os, random, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import jac3 as J

args = sys.argv[1:]
out_path = args[args.index("--out") + 1]
files = args[:args.index("--out")]
inst = {}
for f in files:
    d = json.load(open(f))
    s0 = d["cfg"]["curve_seed0"]
    for r in d["rows"]:
        inst.setdefault((r["p"], s0), {})[r["curve_idx"]] = (r["order"], r["h"], r["r"])
rnd = random.Random(7)
res = []
for (p, s0), curves in sorted(inst.items()):
    seed = s0
    for ci in sorted(curves):
        order, h, r = curves[ci]
        ok_arith = (h * r == order and r > h and h <= 64 and J.is_prime(r)
                    and (math.sqrt(p) - 1) ** 6 <= order <= (math.sqrt(p) + 1) ** 6)
        smallest = all(not (order % hh == 0 and order // hh > hh and J.is_prime(order // hh)) for hh in range(1, h))
        found = None
        while seed < s0 + 4000:
            f = J.random_curve_f(p, seed)
            seed += 1
            if not J.squarefree(f, p):
                continue
            jac = J.Jac(p, f)
            if all(jac.mul(jac.random_divisor(rnd), order) == jac.ZERO for _ in range(4)):
                found = seed - 1
                break
        res.append({"p": p, "curve_idx": ci, "curve_seed": found, "order": order, "h": h, "r": r,
                     "h_r_prime_hasse": ok_arith, "h_smallest": smallest})
        print(res[-1], flush=True)
json.dump(res, open(out_path, "w"), indent=1)
bad = [x for x in res if x["curve_seed"] is None or not x["h_r_prime_hasse"] or not x["h_smallest"]]
print(len(res), "instances checked;", len(bad), "failures")
