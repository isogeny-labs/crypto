#!/usr/bin/env python3
"""EXPLORATORY (post-hoc; not in PROTOCOL.md). Is hits ∝ base_points x exposure? Run: python3 -I exploratory.py > exploratory.txt"""
import json, os, math
SD = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "curve25519_isogeny_seeds_20261114", "runs")
def arm(s, L): return json.load(open(os.path.join(SD, ("iso_L%d.json" % L) if s == 0 else "s%d_L%d.json" % (s, L))))
def chi2_sf(x, k):
    z = ((x/k)**(1/3) - (1 - 2/(9*k))) / math.sqrt(2/(9*k)); return 0.5*math.erfc(z/math.sqrt(2))
for L, nm in ((0, "blind"), (1, "lifts")):
    d = [(arm(s, L)["hits_verified"], arm(s, L)["f4_muladds"]/1e9, arm(s, L)["base_points"]) for s in range(21)]
    lam = sum(h for h, e, b in d) / sum(e*b for h, e, b in d)
    chi = sum((h - lam*e*b)**2/(lam*e*b) for h, e, b in d)
    print("%s: pooled hits per (1e9 mul-adds x base point) = %.4f; chi2 vs hits ∝ base_points x exposure = %.1f on 20 df, approx p = %.3f" % (nm, lam, chi, chi2_sf(chi, 20)))
    for s in (13, 17): print("   seed %d: observed %d, expected %.1f" % (s, d[s][0], lam*d[s][1]*d[s][2]))
