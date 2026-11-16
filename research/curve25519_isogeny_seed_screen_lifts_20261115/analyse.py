#!/usr/bin/env python3
"""Analysis for PROTOCOL.md. Run: python3 -I analyse.py > summary.txt"""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__)); prev = os.path.join(here, "..", "curve25519_isogeny_seed_screen_20261115")
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
def two_sided(k, n, q):
    pk = pm(k, n, q); return sum(pm(i, n, q) for i in range(n+1) if pm(i, n, q) <= pk*(1+1e-9))
g = json.load(open(os.path.join(prev, "groups.json")))
K, C, UNSCR = 2.05, 0.0022857, 2.864
def L1(s): return json.load(open(os.path.join(here, "runs", "s%d_L1.json" % s)))
def L0(s): return json.load(open(os.path.join(prev, "runs", "s%d_L0.json" % s)))
R = {}
print("group seed on_curve base_pts lifts_hits mul-adds(1e9) rate/1e9 expected(model) | blind_hits blind_rate | lifts/target")
for name in ("top", "bottom"):
    H = E = X = H0 = E0 = 0; costs = []
    for s in g[name]:
        a, b0 = L1(s), L0(s); h = a["hits_verified"]; e = a["f4_muladds"]/1e9; bp = a["base_points"]; ex = C*e*bp**K
        H += h; E += e; X += ex; H0 += b0["hits_verified"]; E0 += b0["f4_muladds"]/1e9; costs.append(a["muladds_per_target"])
        print("%-6s %3d %3d %3d %5d %7.2f %6.3f %7.2f | %3d %6.3f | %.4e" % (name, s, g["on_curve"][str(s)], bp, h, e, h/e, ex, b0["hits_verified"], b0["hits_verified"]/(b0["f4_muladds"]/1e9), a["muladds_per_target"]))
    R[name] = dict(H=H, E=E, X=X, H0=H0, E0=E0, costs=costs)
for name in ("top", "bottom"):
    r = R[name]; lo, hi = garwood(r["H"]); il, ih = pois_int(r["X"])
    print("\n%s lifts: %d hits / %.1f = %.3f [%.3f, %.3f]; model expects %.1f (95%% interval %d..%d) -> H20 %s" % (name, r["H"], r["E"], r["H"]/r["E"], lo/r["E"], hi/r["E"], r["X"], il, ih, "SUPPORTED" if il <= r["H"] <= ih else "NOT SUPPORTED"))
t, b = R["top"], R["bottom"]
p = one_sided(t["H"], t["H"]+b["H"], t["E"]/(t["E"]+b["E"]))
print("H17 top/bottom lifts rate ratio %.2f, one-sided p = %.4g -> %s" % ((t["H"]/t["E"])/(b["H"]/b["E"]), p, "SUPPORTED" if p < 0.05 else "NOT SUPPORTED"))
lo, _ = garwood(t["H"]); print("H18 top lower bound %.3f vs %.3f -> %s" % (lo/t["E"], UNSCR, "SUPPORTED" if lo/t["E"] > UNSCR else "NOT SUPPORTED"))
H1 = t["H"]+b["H"]; E1 = t["E"]+b["E"]; H0 = t["H0"]+b["H0"]; E0 = t["E0"]+b["E0"]
p2 = two_sided(H1, H0+H1, E1/(E0+E1))
print("H19 pooled lifts %d/%.1f = %.3f vs blind %d/%.1f = %.3f; ratio %.3f, two-sided p = %.4f -> %s" % (H1, E1, H1/E1, H0, E0, H0/E0, (H1/E1)/(H0/E0), p2, "SUPPORTED" if p2 >= 0.05 else "NOT SUPPORTED"))
for name in ("top", "bottom"):
    r = R[name]; print("   %s lifts/blind rate ratio %.2f" % (name, (r["H"]/r["E"])/(r["H0"]/r["E0"])))
cs = t["costs"] + b["costs"]
print("H21 lifts mul-adds/target %.4e..%.4e, spread %.2f%% -> %s" % (min(cs), max(cs), 100*(max(cs)/min(cs)-1), "SUPPORTED" if max(cs)/min(cs) < 1.10 else "NOT SUPPORTED"))
rows = [(g["on_curve"][str(s)], L1(s)["hits_verified"]/(L1(s)["f4_muladds"]/1e9)) for s in g["top"]+g["bottom"]]
def rk(v):
    o = sorted(range(len(v)), key=lambda i: v[i]); r = [0.0]*len(v); i = 0
    while i < len(o):
        j = i
        while j+1 < len(o) and v[o[j+1]] == v[o[i]]: j += 1
        for k in range(i, j+1): r[o[k]] = (i+j)/2
        i = j+1
    return r
x, y = rk([r[0] for r in rows]), rk([r[1] for r in rows]); m = (len(x)-1)/2
print("descriptive: Spearman on-curve vs lifts arm rate (16 arms) = %.3f" % (sum((a-m)*(c-m) for a, c in zip(x, y))/(sum((a-m)**2 for a in x)*sum((c-m)**2 for c in y))**0.5))
