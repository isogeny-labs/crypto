#!/usr/bin/env python3
"""Analysis for PROTOCOL.md. Run: python3 -I summarise_seeds.py > summary.txt"""
import json, math, os, sys
here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(here, "..", "curve25519_engine_other_bases_20261114"))
def pois_cdf(k, mu): return sum(math.exp(-mu + i * math.log(mu) - math.lgamma(i + 1)) for i in range(k + 1)) if mu > 0 else 1.0
def garwood(k, a=0.05):
    lo = 0.0
    if k > 0:
        x, y = 0.0, 10.0 * k + 10
        for _ in range(100):
            m = (x + y) / 2
            if 1 - pois_cdf(k - 1, m) < a / 2: x = m
            else: y = m
        lo = (x + y) / 2
    x, y = 0.0, 10.0 * k + 40
    for _ in range(100):
        m = (x + y) / 2
        if pois_cdf(k, m) > a / 2: x = m
        else: y = m
    return lo, (x + y) / 2
def binom2(k, n, q):
    pm = lambda i: math.exp(math.lgamma(n+1)-math.lgamma(i+1)-math.lgamma(n-i+1)+i*math.log(q)+(n-i)*math.log(1-q))
    pk = pm(k); return sum(pm(i) for i in range(n+1) if pm(i) <= pk*(1+1e-9))
def chi2_sf(x, k):  # Wilson-Hilferty approx (descriptive only)
    z = ((x/k)**(1/3) - (1 - 2/(9*k))) / math.sqrt(2/(9*k)); return 0.5*math.erfc(z/math.sqrt(2))
R = {}
for s in range(0, 21):
    for L in (0, 1):
        f = os.path.join(here, "runs", ("iso_L%d.json" % L) if s == 0 else "s%d_L%d.json" % (s, L))
        x = json.load(open(f)); R[(s, L)] = (x["hits_verified"], x["f4_muladds"]/1e9, x["muladds_per_target"], x["targets_done"])
tc = {}
for l in open(os.path.join(here, "tower_cost.jsonl")):
    j = json.loads(l); tc[j["seed"]] = j["search_gae"]+j["count_gae"]+j["build_gae"]
print("seed  blind hits  rate/1e9 [95%]        lifts hits  rate/1e9 [95%]       blind mul-adds/target  lifts/target  tower ops")
for s in range(21):
    row = "%4d" % s
    for L in (0, 1):
        h, e, mt, n = R[(s, L)]; lo, hi = garwood(h)
        row += "  %5d  %.3f [%.3f,%.3f]" % (h, h/e, lo/e, hi/e) + ("    " if L == 0 else "")
    row += "  %.4e  %.4e  %6d" % (R[(s, 0)][2], R[(s, 1)][2], tc[s])
    print(row)
S = range(1, 21)
H0 = sum(R[(s, 0)][0] for s in S); E0 = sum(R[(s, 0)][1] for s in S)
H1 = sum(R[(s, 1)][0] for s in S); E1 = sum(R[(s, 1)][1] for s in S)
lo, hi = garwood(H0)
print("\nH6 pooled blind seeds 1-20: %d hits / %.1f = %.3f [%.3f, %.3f]; threshold 1.515 -> %s" % (H0, E0, H0/E0, lo/E0, hi/E0, "SUPPORTED" if lo/E0 > 1.515 else "NOT SUPPORTED"))
lo1, hi1 = garwood(H1)
print("   pooled lifts seeds 1-20: %d hits / %.1f = %.3f [%.3f, %.3f]" % (H1, E1, H1/E1, lo1/E1, hi1/E1))
p = binom2(H1, H0+H1, E1/(E0+E1))
print("H7 rate ratio lifts/blind = %.3f, exact conditional two-sided p = %.4f -> %s" % ((H1/E1)/(H0/E0), p, "SUPPORTED" if p >= 0.05 else "NOT SUPPORTED"))
mt = [R[(s, 0)][2] for s in S]
print("H8 blind mul-adds/target range %.4e..%.4e, spread %.2f%% -> %s" % (min(mt), max(mt), 100*(max(mt)/min(mt)-1), "SUPPORTED" if max(mt)/min(mt) < 1.10 else "NOT SUPPORTED"))
worst = max((tc[s]/(R[(s, 0)][1]*1e9), s) for s in range(1, 21))
print("H9 max tower ops / blind mul-adds = %.2e (seed %d) -> %s" % (worst[0], worst[1], "SUPPORTED" if worst[0] < 1e-3 else "NOT SUPPORTED"))
for name, L in (("blind", 0), ("lifts", 1)):
    hs = [R[(s, L)][0] for s in S]; es = [R[(s, L)][1] for s in S]
    tot = sum(hs)/sum(es); chi = sum((h - tot*e)**2/(tot*e) for h, e in zip(hs, es))
    rates = sorted(h/e for h, e in zip(hs, es))
    print("dispersion %s: chi2=%.1f on 19 df, p~%.3f; per-seed rate range %.3f..%.3f; seed0 %.3f" % (name, chi, chi2_sf(chi, 19), rates[0], rates[-1], R[(0, L)][0]/R[(0, L)][1]))
