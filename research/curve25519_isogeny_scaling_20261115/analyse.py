#!/usr/bin/env python3
"""Analysis for PROTOCOL.md (Parts A-D). Run: python3 -I analyse.py > summary.txt"""
import json, math, os, glob
here = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(here, "..")
S1 = os.path.join(R, "curve25519_isogeny_seeds_20261114", "runs"); S2 = os.path.join(R, "curve25519_isogeny_seed_screen_20261115")
S3 = os.path.join(R, "curve25519_isogeny_seed_screen_lifts_20261115", "runs")
NE, T = 1049624, 20000
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
J = lambda f: json.load(open(f))
pairs = lambda b: b*(b-1)/2/NE * T
g = J(os.path.join(S2, "groups.json"))
def inside(name, arms, tag):
    H = sum(a["hits_verified"] for a in arms); X = sum(pairs(a["base_points"]) for a in arms); lo, hi = pois_int(X)
    ok = lo <= H <= hi
    print("%-34s arms %2d  observed %4d  expected %7.1f (95%% interval %d..%d) obs/exp %.3f -> %s" % (name, len(arms), H, X, lo, hi, H/X, ("%s SUPPORTED" % tag) if ok else ("%s NOT SUPPORTED" % tag)))
    return ok
print("== Part A: pairs law, blind arms, expected = 20000 x b(b-1)/2/1049624")
inside("seeds 0-20 blind", [J(os.path.join(S1, "iso_L0.json" if s == 0 else "s%d_L0.json" % s)) for s in range(21)], "H22")
inside("screen top 8 blind", [J(os.path.join(S2, "runs", "s%d_L0.json" % s)) for s in g["top"]], "H22")
inside("screen bottom 8 blind", [J(os.path.join(S2, "runs", "s%d_L0.json" % s)) for s in g["bottom"]], "H22")
print("descriptive lifts=1 arms (expected x mean lift targets tried per target):")
for name, arms in (("seeds 0-20 lifts", [J(os.path.join(S1, "iso_L1.json" if s == 0 else "s%d_L1.json" % s)) for s in range(21)]),
                   ("screen top 8 lifts", [J(os.path.join(S3, "s%d_L1.json" % s)) for s in g["top"]]), ("screen bottom 8 lifts", [J(os.path.join(S3, "s%d_L1.json" % s)) for s in g["bottom"]])):
    H = sum(a["hits_verified"] for a in arms)
    X = sum(pairs(a["base_points"]) * a["native"]["pkm_lift_targets_tried"] / a["targets_done"] for a in arms)
    f = sum(a["native"]["pkm_lift_targets_tried"] / a["targets_done"] for a in arms) / len(arms)
    print("%-34s observed %4d expected %8.1f obs/exp %.3f (mean lift targets tried per target %.2f)" % (name, H, X, H/X, f))
print("\n== Part B: t = 6, seeds 101-108 blind, 20000 targets")
t6 = [J(f) for f in sorted(glob.glob(os.path.join(here, "runs_t6", "t6_s*.json")))]
if t6:
    tw = {}
    for l in open(os.path.join(here, "deep", "t6_candidates.jsonl")): j = json.loads(l); tw[j["seed"]] = j["search_gae"] + j["count_gae"] + j["build_gae"]
    H = sum(a["hits_verified"] for a in t6); E = sum(a["f4_muladds"] for a in t6)/1e9; lo, hi = garwood(H)
    print("arms %d  hits %d / %.1f = %.3f [%.3f, %.3f]" % (len(t6), H, E, H/E, lo/E, hi/E))
    print("H23 lower bound %.3f vs 3.334 -> %s" % (lo/E, "SUPPORTED" if lo/E > 3.334 else "NOT SUPPORTED"))
    inside("t=6 pairs law", t6, "H24")
    ct = [a["muladds_per_target"] for a in t6]
    print("H25 blind mul-adds/target t=6: %.4e .. %.4e (band 0.885e5 .. 3.541e5) -> %s; ratio to t=5 1.7705e5: %.1fx" % (min(ct), max(ct), "SUPPORTED" if min(ct) >= 0.885e5 and max(ct) <= 3.541e5 else "NOT SUPPORTED", sum(ct)/len(ct)/1.7705e5))
    print("descriptive: base_points %s; tower ops %s; rate t6/t5 unscreened = %.2f; hits per target t6/t5 = %.2f" % (sorted(a["base_points"] for a in t6), sorted(tw.values()), (H/E)/2.909, (H/(20000*len(t6)))/(206/(20000*20))))
print("\n== Part C: deeper screen")
dg = J(os.path.join(here, "deep_groups.json"))
print("H26 chosen seed %d has c = %d (need >= 25) -> %s" % (dg["chosen"], dg["c"], "SUPPORTED" if dg["c"] >= 25 else "NOT SUPPORTED"))
f = os.path.join(here, "runs_deep", "s%d_L0.json" % dg["chosen"])
if os.path.exists(f):
    a = J(f); h = a["hits_verified"]; e = a["f4_muladds"]/1e9; lo, hi = garwood(h)
    print("chosen seed blind: %d hits / %.2f = %.3f [%.3f, %.3f] per 1e9 (unscreened 2.909; top-8 5.155)" % (h, e, h/e, lo/e, hi/e))
    inside("H28 chosen seed pairs law", [a], "H28")
    mph = a["f4_muladds"]/h if h else float("inf")
    lim = 1e-2 * 18 * mph
    print("H27 total tower ops %d vs 1e-2 x 18 x muladds per hit (%.3g) = %.3g -> %s" % (dg["total_tower_ops"], mph, lim, "SUPPORTED" if dg["total_tower_ops"] < lim else "NOT SUPPORTED"))
    rel = a["columns"] + 1; need = rel * mph; rho = math.sqrt(math.pi * 131203 / 4)
    print("\n== Part D (descriptive, indicative only; unit mismatch not resolved; not priced in the 0-1 GAE band)")
    print("chosen seed: columns %d -> %d relations x %.3g mul-adds per hit = %.3g F4 mul-adds; expected rho group ops sqrt(pi*131203/4) = %.0f; ratio %.2e" % (a["columns"], rel, mph, need, rho, need/rho))
