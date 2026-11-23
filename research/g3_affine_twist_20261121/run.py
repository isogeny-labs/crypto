#!/usr/bin/env python3
"""Affine-type (2,2,2) codomain twist study: positive control (Smith pair type on
split curves) and main arm (prime-order odd-model curves, both Frobenius-stable
affine Lagrangians). See PROTOCOL.md.

Usage: python -I run.py control P NCURVES OUT.jsonl
       python -I run.py main P NCURVES OUT.jsonl
"""
import importlib.util
import json
import os
import random
import sys
import time

import fpeval as FE
import smith as SM

HERE = os.path.dirname(os.path.abspath(__file__))
FDIR = os.path.join(HERE, "formulas")
_spec = importlib.util.spec_from_file_location(
    "census", os.path.join(HERE, "..", "g3_transfer_gates_20261120", "census.py"))
CEN = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(CEN)

PAIR_F = "f_000000.json"
AFF_F = "f_001000.json"
AFF_F2 = "f_110111.json"


def load(name):
    fm, PAIRS, rp = FE.load_formula(os.path.join(FDIR, name))
    return fm, PAIRS, rp


def control(p, ncurves, out):
    fm, PAIRS, rp = load(PAIR_F)
    tab = FE.sigma_table(rp["L_subsets"])
    assert len(tab) == 105
    rng = random.Random(7919 * p + 1)
    T = FE.Tower(p, [0, 1], rng)
    for cur in range(ncurves):
        u = rng.sample(range(p), 8)
        c = rng.randrange(1, p)
        F = [c]
        for x in u:
            F = SM.polmul(F, [(-x) % p, 1], p)
        es_all = [T.K.c(x) for x in u]
        for key in sorted(tab, key=lambda k: sorted(sorted(s) for s in k)):
            sigs = tab[key]
            pairs = sorted(tuple(sorted(s)) for s in key if len(s) == 2)
            maps = SM.maps_for_pairing([(u[a], u[b]) for a, b in pairs], p)
            row = {"arm": "control", "p": p, "curve": cur, "u": u, "c": c, "pairs": pairs}
            if maps == "degenerate" or maps is None:
                row["smith"] = "degenerate_system"
                out.write(json.dumps(row) + "\n")
                continue
            sy = [SM.prop4_symbols(N, D, F, p) for N, D in maps]
            row["n_maps"] = len(maps)
            row["smith_symbols"] = [sorted(set(s)) for s in sy]
            vals = []
            for sig in (sigs[0], sigs[rng.randrange(len(sigs))]):
                es = [es_all[sig[i]] for i in range(8)]
                v = FE.eval_formula(T, fm, PAIRS, es, rng)
                vals.append(FE.in_Fp(T, v))
            row["vals"] = vals
            out.write(json.dumps(row) + "\n")
        out.flush()


def stable_keys(tab, perm):
    out = []
    for key in tab:
        img = FE.lag_key([[perm[i] for i in s] for s in key])
        if img == key:
            out.append(key)
    return out


def main_arm(p, ncurves, out, nchecks=5):
    fmA, PAIRS, rpA = load(AFF_F)
    fmB, _, rpB = load(AFF_F2)
    tabA = FE.sigma_table(rpA["L_subsets"])
    tabB = FE.sigma_table(rpB["L_subsets"])
    assert len(tabA) == 30 and set(tabA) == set(tabB)
    perm = [1, 2, 3, 4, 5, 6, 0, 7]
    stab = stable_keys(tabA, perm)
    assert len(stab) == 2, len(stab)
    stab = sorted(stab, key=lambda k: sorted(sorted(s) for s in k))
    fields, reps = CEN.setup_fields(p)
    rng = random.Random(104729 * p + 3)
    got = 0
    draw = 0
    while got < ncurves:
        draw += 1
        f = CEN.random_irreducible_septic(rng, p)
        N = CEN.count_points(fields, reps, f)
        P, integral, _ = CEN.charpoly_from_counts(p, N)
        if not integral or not CEN.weil_ok(p, P):
            raise RuntimeError("charpoly gate failed")
        nJ = sum(P)
        if not CEN.is_prime(nJ):
            continue
        got += 1
        T = FE.Tower(p, f, rng)
        K = T.K
        z = (0, 1, 0, 0, 0, 0, 0)
        roots = [z]
        for _ in range(6):
            roots.append(K.pow(roots[-1], p))
        assert K.pow(roots[-1], p) == z
        do_checks = got <= nchecks
        ts = [t for t in range(p)]
        rng.shuffle(ts)
        tlist = ts[:2] if do_checks else ts[:1]
        for li, key in enumerate(stab):
            row = {"arm": "main", "p": p, "f": f, "N1": N[0], "nJ": nJ, "P": P, "L": li,
                   "L_subsets": sorted(sorted(s) for s in key)}
            res = []
            for t in tlist:
                ft = sum(cf * pow(t, i, p) for i, cf in enumerate(f)) % p
                us = [K.inv(K.sub(e, K.c(t))) for e in roots] + [K.zero]
                sigs = tabA[key]
                sig = sigs[0]
                es = [us[sig[i]] for i in range(8)]
                t0 = time.time()
                v = FE.eval_formula(T, fmA, PAIRS, es, rng)
                x = FE.in_Fp(T, v)
                ent = {"t": t, "c": ft, "val": x, "sec": round(time.time() - t0, 2)}
                if do_checks:
                    v2 = FE.eval_formula(T, fmA, PAIRS, es, rng)  # new root choices
                    ent["inv_roots"] = v2 == v
                    v3 = FE.eval_formula(T, fmA, PAIRS, es, rng, z8pow=3)
                    ent["inv_zeta8"] = v3 == v
                    sig2 = sigs[rng.randrange(len(sigs))]
                    v4 = FE.eval_formula(T, fmA, PAIRS, [us[sig2[i]] for i in range(8)], rng)
                    ent["inv_sigma"] = v4 == v
                    sigB = tabB[key][0]
                    v5 = FE.eval_formula(T, fmB, PAIRS, [us[sigB[i]] for i in range(8)], rng)
                    ent["formula2_val"] = FE.in_Fp(T, v5)
                res.append(ent)
            row["evals"] = res
            out.write(json.dumps(row) + "\n")
        out.flush()


if __name__ == "__main__":
    mode, p, n, path = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    with open(path, "a") as out:
        (control if mode == "control" else main_arm)(p, n, out)
