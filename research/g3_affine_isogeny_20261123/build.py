#!/usr/bin/env python3
"""Construct the F_p plane quartic for one (curve, affine kernel) row of the twist study
and (optionally) count its points.

construct(p, f, L_subsets, t, seed) -> dict with diagnostics, the quartic G over F_p,
and, if count=True, N1..N3 and the L-polynomial of Jac(G).
"""
import importlib.util
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
TW = os.path.join(HERE, "..", "g3_affine_twist_20261121")
sys.path.insert(0, HERE)
import fpquartic as FQ  # noqa: E402

_spec = importlib.util.spec_from_file_location("fpeval_tw", os.path.join(TW, "fpeval.py"))
FE = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(FE)

PRODS, PAIRS, JAC, RELS = FQ.load_tables()
_AFF = json.load(open(os.path.join(TW, "formulas", "f_001000.json")))["report"]["L_subsets"]
SIGTAB = FE.sigma_table(_AFF)
ARON = FQ.all_aronhold_sets()
assert len(ARON) == 288


def construct(p, f, L_subsets, t, seed=0, count=True, sig_index=0):
    t0 = time.time()
    cx = FQ.Ctx(p, 56, seed)
    K, R = cx.K, cx.R
    out = {"p": p, "f": f, "t": t, "seed": seed}
    fK = R([K(c) for c in f])
    roots = [r for r, m in fK.roots()]
    assert len(roots) == 7
    z = roots[0]
    orbit = [z]
    for _ in range(6):
        orbit.append(orbit[-1] ** p)
    assert set(orbit) == set(roots)
    us = [(e - cx.c(t)).inverse() for e in orbit] + [K.zero()]
    key = FE.lag_key(L_subsets)
    sigs = SIGTAB[key]
    sig = sigs[sig_index % len(sigs)]
    es = [us[sig[i]] for i in range(8)]
    P = FQ.theta_squares(cx, PRODS, PAIRS, es)
    Theta = {c: P[(c[0], c[1], c[1])] for c in FQ.EVEN}
    nz = sum(1 for c in FQ.EVEN if Theta[c].is_zero())
    out["n_zero_even_theta2"] = nz
    if nz:
        out["status"] = "vanishing_even_theta (hyperelliptic codomain)"
        return out
    cands, st = FQ.individual_thetas(cx, P, RELS)
    out["n_row_sign_patterns_ok"] = len(cands) if cands else 0
    if not cands:
        out["status"] = "no_sign_pattern:" + st
        return out
    th = cands[0]
    g, nfound = FQ.frobenius_label_action(cx, Theta)
    out["label_matches"] = nfound
    if g is None:
        out["status"] = "frobenius_label_action_failed"
        return out
    stable = []
    for A in ARON:
        img = [g(m) for m in A]
        if set(img) == set(A):
            stable.append(A)
    out["n_stable_aronhold_sets"] = len(stable)
    if not stable:
        out["status"] = "no_frobenius_stable_aronhold_set"
        return out
    m = stable[0]
    pi = [m.index(g(x)) for x in m]
    out["perm_on_aronhold"] = pi
    A = FQ.aronhold_matrix(JAC, th, m, K)
    F, ok = FQ.riemann_quartic(A, K)
    out["riemann_linear_system_consistent"] = ok
    lines = [[K.one(), K.zero(), K.zero()], [K.zero(), K.one(), K.zero()],
             [K.zero(), K.zero(), K.one()], [K.one()] * 3] + [list(r) for r in A]
    Fs = {e: c ** p for e, c in F.items()}
    H = None
    for perm in (pi, [pi.index(i) for i in range(7)]):
        Hc, Ap = FQ.frame([lines[perm[i]] for i in range(7)], K)
        if Hc is None:
            continue
        sA = [[x ** p for x in row] for row in A]
        if Ap == sA:
            lam = FQ.proportional(F, FQ.subst(Fs, Hc, K))
            if lam is not None:
                H = Hc
                out["frobenius_perm_direction"] = "pi" if perm == pi else "pi^-1"
                break
    if H is None:
        out["status"] = "frobenius_frame_check_failed"
        return out
    G = FQ.descend(cx, F, H)
    out["G"] = [[list(e), c] for e, c in sorted(G.items())]
    out["sec_construct"] = round(time.time() - t0, 2)
    if count:
        t1 = time.time()
        N = [FQ.count_points(G, p, k) for k in (1, 2, 3)]
        Lp, integral = FQ.charpoly_from_counts(p, N)
        out["N"] = N
        out["P_X"] = Lp
        out["integral"] = integral
        out["sec_count"] = round(time.time() - t1, 2)
    out["status"] = "ok"
    return out


if __name__ == "__main__":
    rows = [json.loads(l) for l in open(os.path.join(TW, "data", "main.jsonl"))]
    r = rows[int(sys.argv[1])]
    res = construct(r["p"], r["f"], r["L_subsets"], r["evals"][0]["t"], seed=1)
    P = r["P"]
    tw = [c * (-1) ** (6 - i) for i, c in enumerate(P)]
    res["P_J"] = P
    res["match_J"] = res.get("P_X") == P
    res["match_twist"] = res.get("P_X") == tw
    print(json.dumps({k: v for k, v in res.items() if k != "G"}))
