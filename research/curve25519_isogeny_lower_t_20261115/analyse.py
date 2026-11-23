#!/usr/bin/env python3
"""Analysis for PROTOCOL.md with amendments A1, A2. Run: python3 -I analyse.py > summary.txt"""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__))
NE, SQR = 1049624, math.sqrt(131203)
RHO = {0: 1.544, 0.1: 1.654, 1: 2.651}  # T12 rho-neg S at w = 0, 0.1, 1 (atlas section 6)
ATLAS_BEST_GAP_W01 = 1.83e4
def pois_cdf(k, mu): return sum(math.exp(-mu + i*math.log(mu) - math.lgamma(i+1)) for i in range(k+1)) if mu > 0 else 1.0
def garwood(k, a=0.05):
    lo = 0.0
    if k > 0:
        x, y = 0.0, 10.0*k+10
        for _ in range(100):
            m = (x+y)/2
            if 1 - pois_cdf(k-1, m) < a/2: x = m
            else: y = m
        lo = (x+y)/2
    x, y = 0.0, 10.0*k+40
    for _ in range(100):
        m = (x+y)/2
        if pois_cdf(k, m) > a/2: x = m
        else: y = m
    return lo, (x+y)/2
def pois_int(mu, a=0.05):
    lo = 0
    while pois_cdf(lo, mu) < a/2: lo += 1
    hi = lo
    while pois_cdf(hi, mu) < 1 - a/2: hi += 1
    return lo, hi
def pm(i, n, q): return math.exp(math.lgamma(n+1)-math.lgamma(i+1)-math.lgamma(n-i+1)+i*math.log(q)+(n-i)*math.log(1-q))
def one_sided(k, n, q): return sum(pm(i, n, q) for i in range(k, n+1))
g = json.load(open(os.path.join(here, "groups.json")))
def arm(t, G, s): return json.load(open(os.path.join(here, "runs", "t%d_g%d_s%d.json" % (t, G, s))))
cells = {(4, 3): 100000, (4, 2): 100000, (3, 2): 200000}
D = {}
print("t G group seed c b hits mul-adds(1e9) mul-adds/target columns expected(pairs law)")
for (t, G), T in cells.items():
    for grp in ("top", "bottom"):
        rows = []
        for s in g[str(t)][grp]:
            a = arm(t, G, s); b = a["base_points"]; ex = T*b*(b-1)/2/NE
            rows.append(dict(seed=s, c=g[str(t)]["on_curve"][str(s)], b=b, h=a["hits_verified"], e=a["f4_muladds"]/1e9, mt=a["muladds_per_target"], col=a["columns"], ex=ex, ops=g[str(t)]["tower_ops"][str(s)], T=a["targets_done"]))
            r = rows[-1]; print("%d %d %-6s %3d %2d %2d %4d %7.3f %9.1f %3d %6.2f" % (t, G, grp, s, r["c"], b, r["h"], r["e"], r["mt"], r["col"], ex))
        D[(t, G, grp)] = rows
def pool(t, G, grps=("top", "bottom")):
    rows = [r for gr in grps for r in D[(t, G, gr)]]
    return dict(H=sum(r["h"] for r in rows), E=sum(r["e"] for r in rows), X=sum(r["ex"] for r in rows), rows=rows)
print()
P = {}
for (t, G) in cells:
    p = pool(t, G); P[(t, G)] = p; lo, hi = garwood(p["H"]); il, ih = pois_int(p["X"])
    print("cell t=%d G=%d: %d hits / %.2f = %.3f [%.3f, %.3f] per 1e9; mul-adds/target %.4g; pairs law expected %.1f (95%% %d..%d) obs/exp %.3f" % (t, G, p["H"], p["E"], p["H"]/p["E"], lo/p["E"], hi/p["E"], sum(r["mt"] for r in p["rows"])/len(p["rows"]), p["X"], il, ih, p["H"]/p["X"]))
for (t, G) in ((4, 3), (3, 2)):
    p = P[(t, G)]; il, ih = pois_int(p["X"])
    print("H29 t=%d G=%d pairs law: %d in %d..%d -> %s" % (t, G, p["H"], il, ih, "SUPPORTED" if il <= p["H"] <= ih else "NOT SUPPORTED"))
lo, _ = garwood(P[(4, 3)]["H"]); print("H30 t=4 G=3 lower bound %.3f vs 3.334 -> %s" % (lo/P[(4, 3)]["E"], "SUPPORTED" if lo/P[(4, 3)]["E"] > 3.334 else "NOT SUPPORTED"))
a, b = P[(3, 2)], P[(4, 3)]; p = one_sided(a["H"], a["H"]+b["H"], a["E"]/(a["E"]+b["E"]))
print("H31 t=3 G=2 rate %.3f vs t=4 G=3 %.3f: one-sided p = %.4g -> %s" % (a["H"]/a["E"], b["H"]/b["E"], p, "SUPPORTED" if p < 0.05 else "NOT SUPPORTED"))
a, b = P[(4, 3)], P[(4, 2)]; p = one_sided(a["H"], a["H"]+b["H"], a["E"]/(a["E"]+b["E"]))
print("H32 t=4 G=3 rate %.3f vs G=2 %.3f: one-sided p = %.4g -> %s" % (a["H"]/a["E"], b["H"]/b["E"], p, "SUPPORTED" if p < 0.05 else "NOT SUPPORTED"))
tp, bt = pool(4, 3, ("top",)), pool(4, 3, ("bottom",)); p = one_sided(tp["H"], tp["H"]+bt["H"], tp["E"]/(tp["E"]+bt["E"]))
print("H33 t=4 G=3 top %d/%.2f = %.3f vs bottom %d/%.2f = %.3f: one-sided p = %.4g -> %s" % (tp["H"], tp["E"], tp["H"]/tp["E"], bt["H"], bt["E"], bt["H"]/bt["E"] if bt["E"] else 0, p, "SUPPORTED" if p < 0.05 else "NOT SUPPORTED"))
print("\nH34 / indicative S (counted tower GAE + w x (columns+1) x mul-adds per hit, over 362.2; NOT an ecbench S)")
best = max(P, key=lambda k: P[k]["H"]/P[k]["E"])
for (t, G), p in sorted(P.items()):
    mph = p["E"]*1e9/p["H"] if p["H"] else float("inf"); cols = sum(r["col"] for r in p["rows"])/len(p["rows"]); ops = sum(r["ops"] for r in p["rows"])/len(p["rows"])
    line = "t=%d G=%d columns(mean) %.1f mul-adds/hit %.3g tower ops(mean) %.0f:" % (t, G, cols, mph, ops)
    for w in (0, 0.1, 1):
        S = (ops + w*(cols+1)*mph)/SQR; line += "  w=%g S=%.4g gap=%.3g" % (w, S, S/RHO[w])
    print(line + ("   <- best cell" if (t, G) == best else ""))
p = P[best]; mph = p["E"]*1e9/p["H"]; cols = sum(r["col"] for r in p["rows"])/len(p["rows"]); ops = sum(r["ops"] for r in p["rows"])/len(p["rows"])
gap = ((ops + 0.1*(cols+1)*mph)/SQR)/RHO[0.1]
print("H34 best cell t=%d G=%d indicative gap at w=0.1: %.3g vs atlas best %.3g -> %s" % (best[0], best[1], gap, ATLAS_BEST_GAP_W01, "SUPPORTED" if gap < ATLAS_BEST_GAP_W01 else "NOT SUPPORTED"))
