"""Brute-force check of the conjectured tower Reed-Muller analogue (PROOF.md section 3, (T)):
   a nonzero f on (g mu_{2^k})^m of weight-degree <= t has >= 2^{mk-t} nonzeros.  [pcc:unverified]
For every support S with |S| < 2^{mk-t} (all of them, or a random sample where too many), test whether
L_t cap F_S != 0 (rank of the weight->t coefficient rows of the point functions e_v, v in S)."""
import itertools, random, json, sys, math
import numpy as np
import fastdelta as F
from exhaustive_G import det_degree
import icmodel as M
p = int(sys.argv[1]) if len(sys.argv) > 1 else 769
rng = random.Random(7)
res = []
CELLS = [(1, 2, None), (1, 3, None), (1, 4, None), (1, 5, 200000), (2, 2, None), (2, 3, 3000), (3, 2, 3000)]
if len(sys.argv) > 2: CELLS = CELLS[int(sys.argv[2]):]
for (m, k, cap) in CELLS:
    d = 2 ** k
    roots = [w[0] for w in M.tower_base(p, k, rng.randrange(2, p)).pts]
    Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
    Wt = F.weight_grid(d, m, "tower")
    pts = list(itertools.product(range(d), repeat=m))
    viol = 0; tested = 0; worst = None
    for s in range(1, len(pts)):
        if s > 2 ** (m * k) // 2: break
        combos = math.comb(len(pts), s)
        it = itertools.combinations(pts, s) if (cap is None or combos <= cap) else (tuple(rng.sample(pts, s)) for _ in range(cap))
        n_here = 0
        for S_ in it:
            dd = det_degree(list(S_), Vinv, Wt, p, d, m)
            tested += 1; n_here += 1
            gap = dd - (m * k - math.log2(s))
            if worst is None or gap < worst[0]: worst = (round(gap, 3), s, dd)
            if gap < -1e-9: viol += 1
            if cap is not None and n_here >= cap: break
    r = {"m": m, "k": k, "p": p, "exhaustive": cap is None, "supports_tested": tested, "violations": viol,
         "min(delta_det - (mk - log2 s))": worst}
    res.append(r); print(json.dumps(r), flush=True)
