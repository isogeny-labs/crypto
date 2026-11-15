#!/usr/bin/env python3
"""Target 2: from val_tower_solves output (committed build64 instances and the
committed solvers' returned logs and counts), check with the validator's own
arithmetic (vlib.py): the E instance and both rung instances ([x]G = T, [r]D = 0,
[x]D = T, #J kills random divisors, Hasse-Weil window), certify every returned
log ([k]G = T on E for rho, [k]D = T on the rung for IC, k = planted x), and
recompute the per-instance means that tower_large.json reports.
Usage: check_tower_solves.py solves.jsonl tower_large.json OUT.json"""
import json, sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
random.seed(20261113)
dump = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
tl = json.load(open(sys.argv[2]))
byinst = {(i["p"], i["instance"]): i for i in tl["instances"]}
def rand_div(J, p, g):
    D = Jac2.ID
    for _ in range(g):
        while True:
            x = random.randrange(p); fx = peval(J.f, x, p)
            if fx and pow(fx, (p - 1) // 2, p) == 1: break
        D = J.add(D, ([(-x) % p, 1], [tonelli(fx, p)]))
    return D
out = []; errs = []; ncert = 0; nsolve = 0; ncmp = 0
for d in dump:
    if d.get("none"): errs.append(("no instance", d["p"], d["instance"])); continue
    p, r, x = d["p"], d["r"], d["x"]
    E = EC(p, *d["e"]); G = tuple(d["gen"]); T = tuple(d["target"]) if d["target"] else None
    chk = {"p_prime": is_prime(p), "r_prime": is_prime(r), "G_on_E": E.on(G), "rG_zero": E.mul(r, G) is None,
           "xG_eq_T": E.mul(x, G) == T, "r_ge_p_div_4": r >= p // 4, "r_divides_n_e": d["n_e"] % r == 0}
    if p < 20000: chk["n_e_count"] = E.count() == d["n_e"]
    rungs = []
    for rg in d["rungs"]:
        g = rg["genus"]; J = JacG(p, rg["f"], g); N = rg["jac_order"]
        D = J.norm((rg["d"]["u"], rg["d"]["v"])); Tt = J.norm((rg["t"]["u"], rg["t"]["v"]))
        chk[f"g{g}_valid"] = J.valid(D) and J.valid(Tt)
        chk[f"g{g}_D_nonzero"] = not J.is_id(D)
        chk[f"g{g}_rD_zero"] = J.is_id(J.mul(r, D))
        chk[f"g{g}_xD_eq_T"] = J.norm(J.mul(x, D)) == Tt
        chk[f"g{g}_r_divides_order"] = N % r == 0
        chk[f"g{g}_order_in_HW_window"] = (p ** 0.5 - 1) ** (2 * g) <= N <= (p ** 0.5 + 1) ** (2 * g)
        chk[f"g{g}_order_kills_random"] = all(J.is_id(J.mul(N, rand_div(J, p, g))) for _ in range(3))
        for s in rg["ic"]:
            nsolve += 1
            k = s.get("k")
            ok = k is not None and k == x and J.norm(J.mul(k, D)) == Tt
            ncert += ok
            if not ok: errs.append(("IC log not certified", p, d["instance"], g, s.get("seed"), k))
        rungs.append(rg)
    for s in d["rho"]:
        nsolve += 1
        k = s.get("k")
        ok = k is not None and k == x and E.mul(k, G) == T
        ncert += ok
        if not ok: errs.append(("rho log not certified", p, d["instance"], s["seed"], k))
    bad = [k for k, v in chk.items() if v is not True]
    if bad: errs.append(("instance check", p, d["instance"], bad))
    # per-instance means vs committed tower_large.json
    rec = byinst.get((p, d["instance"]))
    if not d["rho"] or not all(rg["ic"] for rg in d["rungs"]):
        rec_means = False
    else:
        rec_means = True
    if rec is None: errs.append(("instance not in tower_large.json", p, d["instance"]))
    else:
        mean = lambda L, f: sum(f(z) for z in L) / len(L)
        pairs = [("r", r, rec["r"]), ("n_e", d["n_e"], rec["n_e"]), ("seed", d["seed"], rec["seed"]),
                 ("jac_order", [rg["jac_order"] for rg in d["rungs"]], rec["jac_order"])]
        if rec_means: pairs += [
                 ("rho_ops", mean(d["rho"], lambda z: z["ops"]), rec["rho_ops"]),
                 ("rho_steps", mean(d["rho"], lambda z: z["steps"]), rec["rho_steps"]),
                 ("rho_dps", mean(d["rho"], lambda z: z["dps"]), rec["rho_dps"])]
        for gi, rg in enumerate(d["rungs"] if rec_means else []):
            for f, jf in (("ops", "ops"), ("la_mulmods", "la_mulmods"), ("trials", "trials"), ("relations", "relations"), ("columns", "columns")):
                pairs.append((f"g{rg['genus']}_{f}", mean(rg["ic"], lambda z: z[f]), rec["ic"][gi][jf]))
        for name, a, b in pairs:
            ncmp += 1
            if isinstance(a, float) or isinstance(b, float):
                if abs(a - b) > 1e-9 * max(1, abs(b)): errs.append(("mean differs", p, d["instance"], name, a, b))
            elif a != b: errs.append(("value differs", p, d["instance"], name, a, b))
    out.append({"p": p, "instance": d["instance"], "r": r, "checks": chk})
res = {"instances": len(out), "solves": nsolve, "independently_certified": ncert, "fields_compared_with_tower_large_json": ncmp,
       "errors": errs, "per_instance": out}
json.dump(res, open(sys.argv[3], "w"), indent=1)
print(f"instances {len(out)}; solver logs {nsolve}, independently certified {ncert}; fields compared {ncmp}; errors {len(errs)}")
for e in errs[:20]: print("  ", e)
