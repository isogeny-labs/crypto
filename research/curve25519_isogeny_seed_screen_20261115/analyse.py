#!/usr/bin/env python3
"""Analysis for PROTOCOL.md. Run: python3 -I analyse.py > summary.txt"""
import json, math, os
here = os.path.dirname(os.path.abspath(__file__))
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
def pois_int(mu, a=0.05):  # exact central interval of counts for mean mu
    lo = 0
    while pois_cdf(lo, mu) < a/2: lo += 1
    hi = lo
    while pois_cdf(hi, mu) < 1 - a/2: hi += 1
    return lo, hi
def binom_one_sided_greater(k, n, q):
    pm = lambda i: math.exp(math.lgamma(n+1)-math.lgamma(i+1)-math.lgamma(n-i+1)+i*math.log(q)+(n-i)*math.log(1-q))
    return sum(pm(i) for i in range(k, n+1))
g = json.load(open(os.path.join(here, "groups.json")))
LAM, UNSCREENED = 0.0913, 2.909
def arm(s): return json.load(open(os.path.join(here, "runs", "s%d_L0.json" % s)))
res = {}
print("group seed on_curve base_pts hits mul-adds(1e9) rate/1e9 expected(model)")
for name in ("top", "bottom"):
    H = E = X = 0
    for s in g[name]:
        a = arm(s); h = a["hits_verified"]; e = a["f4_muladds"]/1e9; b = a["base_points"]; ex = LAM*e*b
        H += h; E += e; X += ex
        print("%-6s %3d %3d %3d %4d %7.3f %6.3f %6.2f" % (name, s, g["on_curve"][str(s)], b, h, e, h/e, ex))
    res[name] = (H, E, X)
for name in ("top", "bottom"):
    H, E, X = res[name]; lo, hi = garwood(H); il, ih = pois_int(X)
    print("\n%s: %d hits / %.1f = %.3f [%.3f, %.3f]; model expects %.1f (95%% count interval %d..%d) -> H15 %s" % (name, H, E, H/E, lo/E, hi/E, X, il, ih, "SUPPORTED" if il <= H <= ih else "NOT SUPPORTED"))
Ht, Et, _ = res["top"]; Hb, Eb, _ = res["bottom"]
p = binom_one_sided_greater(Ht, Ht+Hb, Et/(Et+Eb))
print("H13 top vs bottom: ratio %.2f, one-sided exact conditional p = %.4g -> %s" % ((Ht/Et)/(Hb/Eb), p, "SUPPORTED" if p < 0.05 else "NOT SUPPORTED"))
lo, hi = garwood(Ht); print("H14 top lower bound %.3f vs %.3f -> %s" % (lo/Et, UNSCREENED, "SUPPORTED" if lo/Et > UNSCREENED else "NOT SUPPORTED"))
print("H16 tower ops all 40 candidates %d vs 1e-3 x top F4 mul-adds %.3g -> %s" % (g["total_tower_ops"], 1e-3*Et*1e9, "SUPPORTED" if g["total_tower_ops"] < 1e-3*Et*1e9 else "NOT SUPPORTED"))
lo2, hi2 = garwood(Hb)
print("descriptive: top rate / unscreened = %.2f; bottom rate / unscreened = %.2f; bottom interval [%.3f, %.3f]" % ((Ht/Et)/UNSCREENED, (Hb/Eb)/UNSCREENED, lo2/Eb, hi2/Eb))
rows = [(g["on_curve"][str(s)], arm(s)["hits_verified"]/(arm(s)["f4_muladds"]/1e9)) for s in g["top"]+g["bottom"]]
def rk(v):
    o = sorted(range(len(v)), key=lambda i: v[i]); r = [0.0]*len(v); i = 0
    while i < len(o):
        j = i
        while j+1 < len(o) and v[o[j+1]] == v[o[i]]: j += 1
        for k in range(i, j+1): r[o[k]] = (i+j)/2
        i = j+1
    return r
x, y = rk([r[0] for r in rows]), rk([r[1] for r in rows]); m = (len(x)-1)/2
print("descriptive: Spearman on-curve vs arm rate over 16 arms = %.3f" % (sum((a-m)*(b-m) for a, b in zip(x, y))/(sum((a-m)**2 for a in x)*sum((b-m)**2 for b in y))**0.5))
