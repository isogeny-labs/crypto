#!/usr/bin/env python3
"""Study 1, p = 8209, w = 1: which convention gives the '1.057 [0.765, 1.499]' that
RESULTS.md attributes to 'multiplications and additions on both arms as emitted',
and the relation share of w's native work with and without additions.  Bootstrap
as analyze.py's design (curves, then seeds, unpaired) with the producer's declared
seed 20261119 and B = 2000, implemented by the validator.  [] indexing."""
import json, random, sys
rows = [json.loads(l) for l in open(sys.argv[1])]
mean = lambda v: sum(v) / len(v)
B = 2000
def grp(arm, p, fn):
    g = {}
    for r in rows:
        if r["kind"] == "run" and r["arm"] == arm and r["p"] == p:
            g.setdefault(r["curve_seed"], []).append(fn(r))
    return g
def boot(p, fn_ic, fn_rho, seed=20261119):
    rnd = random.Random(seed)
    gn, gd = grp("C_double_lp+pre+coord+unitT", p, fn_ic), grp("R_rho_neg", p, fn_rho)
    curves = sorted(gn)
    est = mean(sum(gn.values(), [])) / mean(sum(gd.values(), []))
    bs = []
    for _ in range(B):
        n, d = [], []
        for c in [rnd.choice(curves) for _ in curves]:
            n += [rnd.choice(gn[c]) for _ in gn[c]]
            d += [rnd.choice(gd[c]) for _ in gd[c]]
        bs.append(mean(n) / mean(d))
    bs.sort()
    return f"{est:.4f} [{bs[int(0.025 * B)]:.3f}, {bs[int(0.975 * B) - 1]:.3f}]"
M = lambda r: r["rel_mul"] + r["la_mul"]
A = lambda r: r["rel_add"] + r["la_add"]
print("p = 8209, w = 1, arm w over negation rho:")
print("  IC G+M        / rho G+C (protocol)              ", boot(8209, lambda r: r["group_ops"] + M(r), lambda r: r["group_ops"] + r["native"]))
print("  IC G+M        / rho G   (mults only, rho C = 0) ", boot(8209, lambda r: r["group_ops"] + M(r), lambda r: r["group_ops"]))
print("  IC G+M+A      / rho G+C (adds on IC only)       ", boot(8209, lambda r: r["group_ops"] + M(r) + A(r), lambda r: r["group_ops"] + r["native"]))
print("  IC G+M+A      / rho G+C+H+2D est. (3 per step)  ", boot(8209, lambda r: r["group_ops"] + M(r) + A(r), lambda r: r["group_ops"] + r["native"] + 3 * r["steps"]))
for p in (2053, 4099, 8209):
    W = [r for r in rows if r["kind"] == "run" and r["p"] == p and r["arm"] == "C_double_lp+pre+coord+unitT"]
    rm, lm, ra, la = (sum(r[k] for r in W) for k in ("rel_mul", "la_mul", "rel_add", "la_add"))
    print(f"p = {p}: relation share of w's multiplications {rm / (rm + lm):.3f}; of multiplications + additions {(rm + ra) / (rm + lm + ra + la):.3f}")
