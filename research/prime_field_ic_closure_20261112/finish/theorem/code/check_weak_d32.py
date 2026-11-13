"""Literal-Macaulay validation of the weak degree where it deviates from the random-function prediction
(uni, m = 2, d = 16 and 32).  [pcc:unverified]"""
import sys, json, random, time
import numpy as np
import icmodel as M, fastdelta as F
from check_L4_weak import weak_XL
from check_G_hypothesis import generic_weak
p = 786433
rng = random.Random("weak-d32")
while True:
    a = rng.randrange(p); b = rng.randrange(1, p)
    if (4 * a ** 3 + 27 * b ** 2) % p: break
for d in (16, 32):
    roots = set()
    while len(roots) < d:
        x = rng.randrange(p); r = (x ** 3 + a * x + b) % p
        if r == 0 or pow(r, (p - 1) // 2, p) == 1: roots.add(x)
    roots = sorted(roots)
    base = M.uni_base(p, roots)
    Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
    Wt = F.weight_grid(d, 2, "uni")
    for trial in range(2):
        while True:
            xR = rng.randrange(p)
            SR = M.semaev_special(2, p, a, b, xR)
            G = F.semaev_grid(SR, 2, roots, p)
            if not (G == 0).any(): break
        invS = F.inv_mod_array(G, p)
        fw = F.delta_weak(invS, roots, Vinv, [], Wt, p, 4, "uni")
        gen = generic_weak(Wt, d, 0, 0)
        Sg = M.S_global(SR, 2, 1)
        Gs = [Sg] + [M.lift_block(gg, i, 1, 2) for i in range(2) for gg in base.gens]
        t0 = time.time()
        res = {}
        for D in (4 + fw - 1, 4 + fw):
            ok, inter, base_dim = weak_XL(Gs, 2, 1, D, p, base)
            res[D] = ok
        print(json.dumps({"d": d, "xR": xR, "fast_delta_weak": fw, "generic_weak": gen,
                          "D_pred": 4 + fw, "XL_weak_at_Dpred_minus_1": res[4 + fw - 1],
                          "XL_weak_at_Dpred": res[4 + fw], "max": 2 * (d - 1),
                          "sec": round(time.time() - t0, 1)}), flush=True)
