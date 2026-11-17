"""Rank-deficit detection degree delta_det(R) for decomposable targets (PRESET_G_amend3).  [pcc:unverified]"""
import sys, json, random, time
import numpy as np
import icmodel as M, fastdelta as F
from check_G_hypothesis import semaev_dict, roots_in_X, rand_curve_big
kind, m, size, p, ndec, seed = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), int(sys.argv[6])
rng = random.Random(f"DET-{kind}-{m}-{size}-{p}-{seed}")
a, b, _, _ = rand_curve_big(p, random.Random(f"curve-{p}-{seed}"))
if kind == "uni":
    d = size; roots = set()
    while len(roots) < d:
        x = rng.randrange(p); r = (x ** 3 + a * x + b) % p
        if r == 0 or pow(r, (p - 1) // 2, p) == 1: roots.add(x)
    roots = sorted(roots); Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
elif kind == "tower":
    d = 2 ** size; base = M.tower_base(p, size, rng.randrange(2, p)); roots = [w[0] for w in base.pts]
    Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
else:
    d = 2 ** size; roots = list(range(d)); Vinv, _ = F.mobius_mats(size, p)
Wt = F.weight_grid(d, m, "uni" if kind == "uni" else "tower")
tmax = int(Wt.max())
rows = []; tries = 0
while len(rows) < ndec and tries < 50 * ndec:
    tries += 1
    xs = [rng.choice(roots) for _ in range(m)]
    Xs = roots_in_X(m, p, a, b, xs)
    if not Xs: continue
    xR = rng.choice(Xs)
    G = F.semaev_grid(semaev_dict(m, p, a, b, xR), m, roots, p)
    Vidx = [tuple(int(i) for i in ix) for ix in zip(*np.nonzero(G == 0))]
    cols = []
    for v in Vidx:
        T = np.ones((1,) * m, dtype=np.int64)
        for ax in range(m):
            sh = [1] * m; sh[ax] = d
            T = (T * Vinv[:, v[ax]].reshape(sh)) % p
        cols.append(T)
    det = tmax
    for t in range(tmax - 1, -2, -1):
        mask = Wt > t
        A = np.stack([c[mask] for c in cols], axis=1)
        if F._rank(A, p) < len(Vidx): det = t
        else: break
    nV = len(Vidx)
    gen = next(t for t in range(-1, tmax + 1) if int((Wt > t).sum()) < nV)
    degenerate = len({v[0] for v in Vidx}) == d
    rows.append({"xR": xR, "nV": nV, "delta_det": det, "generic_det": gen, "max": tmax, "degenerate": degenerate,
                 "bits_floor": (m * size - int(np.floor(np.log2(nV)))) if kind == "bits" else None})
dev = [r["delta_det"] - r["generic_det"] for r in rows]
nd = [r["delta_det"] - r["generic_det"] for r in rows if not r["degenerate"]]
summ = {"kind": kind, "m": m, "size": size, "max": tmax, "n": len(rows), "dev_min": min(dev), "dev_max": max(dev),
        "n_degenerate": sum(r["degenerate"] for r in rows), "nondeg_dev_min": min(nd) if nd else None,
        "nondeg_det_min_over_max": round(min(r["delta_det"] for r in rows if not r["degenerate"]) / tmax, 3) if nd else None,
        "det_min_over_max": round(min(r["delta_det"] for r in rows) / tmax, 3),
        "nV_max": max(r["nV"] for r in rows),
        "bits_floor_ok": all(r["delta_det"] >= r["bits_floor"] for r in rows) if kind == "bits" else None}
print(json.dumps({"summary": summ, "rows": rows}))
