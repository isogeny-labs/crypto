"""Check the weak-success characterisation (Lemma L4-weak) against the literal Macaulay matrix, and the
coefficient-domain routine fastdelta.delta_weak against both.  [pcc:unverified]

weak success at degree D:  Mac_D contains a polynomial in the block-1 variables (x_1, z^(1)) that is not
in the base ideal J_1, i.e.  dim(Mac_D cap k[block1]_{<=D}) > |block1 monomials <= D| - H_W(D).
Prediction:  D_weak = dS + least t such that (v, u) with deg v <= t, u in k[block1]_{<= dS+t},
             v*S = u on all of W^m and u != 0 on W   (rank test rank A < rank(S E_t) + rank(U_t)).
"""
import sys, json, random, time
import numpy as np
import flint
import icmodel as M
import fastdelta as F
from check_L4_xl_degree import pick_targets, reduce_mod_tower_x

def block1_monos(N, nloc, D):
    return [e for e in M.monomials_upto(N, D) if all(k == 0 for k in e[nloc:])]

def weak_XL(G, N, nloc, D, p, base):
    cols = M.monomials_upto(N, D)
    idx = {e: j for j, e in enumerate(cols)}
    M0, nr, nc = M.mac_matrix(G, N, D, p)
    r0 = M0.rank() if nr else 0
    B1 = block1_monos(N, nloc, D)
    # stacked [Mac rows ; unit rows of block-1 monomials]
    rows = []
    for g in G:
        dg = M.tdeg(g)
        if dg > D: continue
        for u in M.monomials_upto(N, D - dg):
            rows.append([(idx[tuple(a + b for a, b in zip(e, u))], c) for e, c in g.items()])
    A = flint.nmod_mat(len(rows) + len(B1), len(cols), p)
    for i, r in enumerate(rows):
        for j, c in r: A[i, j] = c
    for i, e in enumerate(B1):
        A[len(rows) + i, idx[e]] = 1
    rA = A.rank()
    inter = r0 + len(B1) - rA
    # H_W(D) restricted to block 1 (functions on W of degree <= D)
    HW = M.eval_monomials(base.pts, [e[:nloc] for e in B1], p).rank()
    return inter > len(B1) - HW, inter, len(B1) - HW

def weak_pred(N, nloc, W, Sg, dS, p, base, tmax):
    Svals = [M.eval_poly(Sg, w, p) for w in W]
    for t in range(0, tmax + 1):
        mons = M.monomials_upto(N, t)
        B1 = block1_monos(N, nloc, dS + t)
        A = flint.nmod_mat(len(W), len(mons) + len(B1), p)
        SE = flint.nmod_mat(len(W), len(mons), p)
        U = flint.nmod_mat(len(W), len(B1), p)
        for i, w in enumerate(W):
            for j, e in enumerate(mons):
                v = 1
                for x, k in zip(w, e):
                    if k: v = v * pow(x, k, p) % p
                v = v * Svals[i] % p
                A[i, j] = v; SE[i, j] = v
            for j, e in enumerate(B1):
                v = 1
                for x, k in zip(w, e):
                    if k: v = v * pow(x, k, p) % p
                A[i, len(mons) + j] = (-v) % p; U[i, j] = v
        if A.rank() < SE.rank() + U.rank():
            return t
    return None

def run(tag, base, m, p, a, b, pts, targets, S_mode="x", kind="uni"):
    N = m * base.nloc; nloc = base.nloc
    W = M.grid(base, m)
    roots = [w[0] for w in base.pts]
    d = len(roots)
    Vm = F.vandermonde(roots, p); Vinv = F.mat_inv_mod(Vm, p)
    Wt = F.weight_grid(d, m, "uni" if kind == "uni" else "tower")
    for tk, xR in targets:
        t0 = time.time()
        SR = M.semaev_special(m, p, a, b, xR)
        Sg = M.S_global(SR, m, nloc) if S_mode == "x" else reduce_mod_tower_x(SR, base, m, p)
        dS = M.tdeg(Sg)
        G = [Sg] + [M.lift_block(gg, i, nloc, N) for i in range(m) for gg in base.gens]
        tmax_ = m * (len(roots) - 1) if kind == "uni" else m * base.info.get("k", 1)
        tw = weak_pred(N, nloc, W, Sg, dS, p, base, tmax=tmax_ + 1)
        pred = dS + tw if tw is not None else None
        # literal XL scan
        DX = None
        if pred is None:
            # degenerate target (pi_1(V_R) = whole base): no weak success is predicted at any degree;
            # check only up to the strong-refutation scale, then record
            Dcap_ = dS + m * (len(roots) - 1) + 1 if kind == "uni" else dS + m * base.info.get("k", 1) + 1
        for D in range(1, (pred if pred is not None else Dcap_) + 2):
            ok, inter, base_dim = weak_XL(G, N, nloc, D, p, base)
            if ok: DX = D; break
        # fast coefficient-domain routine (grid indexed by base point order)
        Gv = F.semaev_grid(SR, m, roots, p)
        Vidx = [tuple(int(i) for i in ix) for ix in zip(*np.nonzero(Gv == 0))]
        invS = F.inv_mod_array(Gv, p)
        fw = F.delta_weak(invS, roots, Vinv, Vidx, Wt, p, dS, "uni" if kind == "uni" else "tower")
        rec = {"tag": tag, "target": tk, "xR": xR, "N": N, "dS": dS, "nV": len(Vidx),
               "t_weak_eval": tw, "D_weak_pred": pred, "D_weak_XL": DX,
               "fast_delta_weak": fw, "fast_D": (dS + fw) if fw is not None else None,
               "agree_XL_pred": DX == pred, "agree_fast": (fw is not None and dS + fw == pred),
               "sec": round(time.time() - t0, 2)}
        print(json.dumps(rec), flush=True)

if __name__ == "__main__":
    which = sys.argv[1]
    rng = random.Random(f"L4w-{which}")
    p = 12289
    a, b, pts, n = M.rand_curve(p, rng)
    absc = sorted({P[0] for P in pts})
    if which == "uni2":
        for d in (3, 4, 5, 6, 8):
            base = M.uni_base(p, rng.sample(absc, d))
            run(f"uni2-d{d}", base, 2, p, a, b, pts, pick_targets(base, 2, p, a, b, pts, rng, 3, 2))
    elif which == "uni3":
        for d in (3, 4, 5):
            base = M.uni_base(p, rng.sample(absc, d))
            run(f"uni3-d{d}", base, 3, p, a, b, pts, pick_targets(base, 3, p, a, b, pts, rng, 2, 2))
    elif which == "tower2":
        for k in (2, 3):
            base = M.tower_base(p, k, rng.randrange(2, p))
            run(f"tower2-k{k}-Sred", base, 2, p, a, b, pts, pick_targets(base, 2, p, a, b, pts, rng, 2, 2),
                S_mode="red", kind="tower")
