"""Structured supports for the tower Reed-Muller analogue (delta_det >= mk - log2|S|).  [pcc:unverified]
m = 2: graphs of x -> zeta^j x, x -> c'/x, diagonal restricted to subgroup cosets, products of cosets."""
import json, math, random, sys
import numpy as np
import icmodel as M, fastdelta as F
from exhaustive_G import det_degree
p = 12289
rng = random.Random(11)
for k in range(2, 6):
    d = 2 ** k
    g = rng.randrange(2, p)
    roots = [w[0] for w in M.tower_base(p, k, g).pts]
    idx = {r: i for i, r in enumerate(roots)}
    Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
    Wt = F.weight_grid(d, 2, "tower")
    # roots[j] = g * z^j
    fams = {}
    for j in range(d):
        fams[f"graph x->z^{j} x"] = [(i, (i + j) % d) for i in range(d)]
    fams["graph x->g^2/x"] = [(i, (-i) % d) for i in range(d)]
    for e in range(0, k + 1):
        H = [i for i in range(d) if i % (d >> e) == 0]          # subgroup coset of size 2^e (indices)
        fams[f"diag on coset 2^{e}"] = [(i, i) for i in H]
        fams[f"antidiag on coset 2^{e}"] = [(i, (-i) % d) for i in H]
        for e2 in range(0, k + 1):
            H2 = [i for i in range(d) if i % (d >> e2) == 0]
            fams[f"coset 2^{e} x coset 2^{e2}"] = [(i, j) for i in H for j in H2]
        fams[f"diag + shifted diag on 2^{e}"] = [(i, i) for i in H] + [(i, (i + d // 2) % d) for i in H]
    worst = None; viol = []
    for name, Sx in fams.items():
        Sx = sorted(set(Sx))
        dd = det_degree(Sx, Vinv, Wt, p, d, 2)
        gap = dd - (2 * k - math.log2(len(Sx)))
        if worst is None or gap < worst[0]: worst = (round(gap, 3), name, len(Sx), dd)
        if gap < -1e-9: viol.append((name, len(Sx), dd, round(2 * k - math.log2(len(Sx)), 3)))
    print(json.dumps({"m": 2, "k": k, "p": p, "families": len(fams), "violations": viol[:12], "n_viol": len(viol), "worst_gap": worst}))
