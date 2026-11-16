#!/usr/bin/env python3
"""Recompute lambda_G by exact Fincke-Pohst enumeration for trials where heights.min_form returned
the smallest eigenvalue (flag 'singular', tolerance 3 r eps) instead of the lattice minimum.
Replays xedni_cubic.main's RNG to regenerate the target curve and trial seed.  [pcc:unverified]
Writes a corrected copy of every run file to <out_dir>; replaced records keep the old heights
under 'heights_tol_flagged'."""
import sys, os, json, glob, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, "/home/user/crypto/research/prime_field_ic_closure_20261112/finish/lifting/code")
import heights
_orig = heights.min_form


def exact_min_form(G, tol=0.0, max_nodes=200000):
    return _orig(G, tol=-1.0, max_nodes=5_000_000)


heights.min_form = exact_min_form
import xedni_cubic as XC, xedni_toy as X0

src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)
SEED = 20261115
for f in sorted(glob.glob(src + "/*.jsonl")):
    recs = [json.loads(l) for l in open(f)]
    need = [i for i, d in enumerate(recs) if d["status"] == "ok" and d.get("heights")
            and d["heights"].get("lambda_vec") == "singular"]
    if need:
        d0 = recs[0]
        bits, r = d0["bits"], d0["r"]
        rng = random.Random(SEED * 1000003 + bits * 101 + r)
        ncur = max(d["curve"] for d in recs) + 1
        per = len(recs) // ncur
        seeds = {}
        for ci in range(ncur):
            tg = X0.make_target(bits, rng)
            for j in range(per):
                sd = rng.randrange(1 << 62)
                seeds[sd] = tg
        for i in need:
            d = recs[i]
            tg = seeds[d["seed"]]
            assert tg["p"] == d["p"]
            new = XC.run_trial(tg, r, "X", d["seed"], 5)
            assert new["status"] == "ok" and new.get("W") == d.get("W")
            new["heights_tol_flagged"] = d["heights"]
            new["sec"], new["curve"] = d["sec"], d["curve"]
            recs[i] = new
            h = new["heights"]
            print(os.path.basename(f), "rho_G old %.2f new %.3f lamG %.3f vec %s" %
                  (d["heights"]["rho_G"], h["rho_G"], h["lambda_G"], h["lambda_vec"]))
    with open(os.path.join(dst, os.path.basename(f)), "w") as fo:
        for d in recs:
            fo.write(json.dumps(d, default=str) + "\n")
