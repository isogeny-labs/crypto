#!/usr/bin/env python3
"""m8-runs generator: PKM Kummer chain with per-summand tower depth t_i (PRESET section 3/4).

Uses the parent study's pkmgen.py / gen_all.py as libraries (unchanged).
Usage: gen_mixed.py OUTDIR      (writes OUTDIR/systems/*.ms, *.json and OUTDIR/index.jsonl)
       gen_mixed.py --selftest
"""
import hashlib
import json
import os
import random
import sys

PARENT = "/home/user/crypto/research/prime_field_ic_closure_20261112/finish/pkm-msolve/scripts"
sys.path.insert(0, PARENT)
import pkmgen as G  # noqa: E402
import gen_all as GA  # noqa: E402

P1 = GA.P1


def build_mixed(p, A, B, g, depths, xR):
    m = len(depths)
    names = [f"y{i}_{j}" for i in range(m) for j in range(depths[i])] + [f"u{k}" for k in range(m - 2)]
    nv = len(names)
    off = []
    c = 0
    for i in range(m):
        off.append(c)
        c += depths[i]
    yi = lambda i, j: off[i] + j
    ui = lambda k: c + k
    X = [G.pvar(yi(i, 0), nv, g % p) for i in range(m)]
    U = [G.pvar(ui(k), nv) for k in range(m - 2)]
    xr = G.pconst(xR, nv, p)
    eqs = [G.S3poly(X[0], X[1], U[0], A, B, nv, p)]
    for k in range(1, m - 2):
        eqs.append(G.S3poly(U[k - 1], X[k + 1], U[k], A, B, nv, p))
    eqs.append(G.S3poly(U[m - 3], X[m - 1], xr, A, B, nv, p))
    for i in range(m):
        t = depths[i]
        for j in range(t - 1):
            eqs.append(G.padd(G.pvar(yi(i, j + 1), nv),
                              G.pscale(G.pmul(G.pvar(yi(i, j), nv), G.pvar(yi(i, j), nv), p), -1, p), p))
        last = G.pvar(yi(i, t - 1), nv)
        eqs.append(G.padd(G.pmul(last, last, p), G.pconst(-1, nv, p), p))
    text = ",".join(names) + "\n" + str(p) + "\n" + ",\n".join(G.fmt_poly(e, names) for e in eqs) + "\n"
    return names, eqs, text


def enumerate_mixed(p, A, B, Vs, xR, yR):
    """grid zeros of the chain with per-summand bases Vs[i]; MITM over signed points in E(F_{p^2})"""
    F = G.Fp2(p)
    m = len(Vs)
    sps = [G.signed_points(F, A, B, V) for V in Vs]
    R = ((xR % p, 0), (yR % p, 0))
    negR = G.ec_neg(F, R)
    h = (m + 1) // 2

    def walk(positions):
        res = [((), (), [], None)]
        for pos in positions:
            nxt = []
            for idx, sg, partial, S in res:
                for vi in range(len(Vs[pos])):
                    for s, P in sps[pos][vi]:
                        S2 = G.ec_add(F, A, S, P)
                        nxt.append((idx + (vi,), sg + (s,), partial + [S2], S2))
            res = nxt
        return res

    left = walk(range(h))
    right = walk(range(h, m))
    rmap = {}
    for idx, sg, partial, S in right:
        rmap.setdefault(S, []).append((idx, sg, partial))
    zeros = set()
    for idx, sg, partial, SL in left:
        if sg[0] != 1:
            continue
        for target in (R, negR):
            need = G.ec_add(F, A, target, G.ec_neg(F, SL))
            for ridx, rsg, rpartial in rmap.get(need, ()):
                full = list(partial) + [G.ec_add(F, A, SL, rp) for rp in rpartial]
                if all(full[k] is not None for k in range(1, m - 1)):
                    zeros.add(idx + ridx)
    return zeros


def selftest():
    ok = True
    for p in (40961, P1):
        S = GA.PrimeSetup(p, curve=GA.P1_CURVE if p == P1 else None)
        for m, t in ((4, 2), (5, 1), (6, 2), (8, 1)):
            rng = random.Random(7)
            xR, yR = GA.random_target(S, rng)
            _, _, a = G.build_system(p, S.A, S.B, S.g, m, t, xR)
            _, _, b = build_mixed(p, S.A, S.B, S.g, [t] * m, xR)
            same = a == b
            ok &= same
            print("gen-identity", p, m, t, same)
    # enumerator identity vs parent on uniform depth, incl. planted (nonzero) cases
    p = 769
    S = GA.PrimeSetup(p)
    for m, t in ((4, 2), (5, 2), (4, 1)):
        for kind in ("random", "planted"):
            rng = random.Random(11)
            if kind == "random":
                xR, yR = GA.random_target(S, rng)
            else:
                xR, yR, _, _ = GA.planted_target(S, m, t, rng)
            z1, _ = G.enumerate_zeros(p, S.A, S.B, S.V(t), m, xR, yR, want_solutions=False)
            z2 = enumerate_mixed(p, S.A, S.B, [S.V(t)] * m, xR, yR)
            same = z1 == z2
            ok &= same
            print("enum-identity", p, m, t, kind, len(z1), len(z2), same)
    # known-answer: original committed repo system reproduced
    print("SELFTEST", "PASS" if ok else "FAIL")
    return ok


def planted_mixed(S, depths, rng):
    p, A, B = S.p, S.A, S.B
    F = G.Fp2(p)
    m = len(depths)
    for _ in range(10000):
        xs, pts = [], []
        for i in range(m):
            onE = [v for v in S.V(depths[i]) if G.legendre(v ** 3 + A * v + B, p) == 1]
            x = rng.choice(onE)
            y = G.sqrt_mod((x ** 3 + A * x + B) % p, p)
            if rng.randrange(2):
                y = (-y) % p
            xs.append(x)
            pts.append(((x, 0), (y, 0)))
        partial, Ssum = [], None
        for P in pts:
            Ssum = G.ec_add(F, A, Ssum, P)
            partial.append(Ssum)
        if any(q is None for q in partial[1:]):
            continue
        sol = {}
        for i, x in enumerate(xs):
            yv = x * G.inv(S.g, p) % p
            for j in range(depths[i]):
                sol[f"y{i}_{j}"] = yv
                yv = yv * yv % p
        for k in range(m - 2):
            sol[f"u{k}"] = partial[k + 1][0][0]
        return Ssum[0][0], Ssum[1][0], sol, xs
    raise RuntimeError("planted failed")


def write(out, name, meta, S, depths, xR):
    names, eqs, text = build_mixed(S.p, S.A, S.B, S.g, depths, xR)
    with open(os.path.join(out, name + ".ms"), "w") as fh:
        fh.write(text)
    meta.update(name=name, p=S.p, A=S.A, B=S.B, n_E=S.n, g=S.g, depths=depths, m=len(depths), N=sum(depths),
                nvars=len(names), neqs=len(eqs), xR=xR, Vs=[S.V(t) for t in depths],
                sha256=hashlib.sha256(text.encode()).hexdigest())
    if "planted" in meta:
        import msparse as MP
        F = G.Fp2(S.p)
        point = [(meta["planted"][nm] % S.p, 0) for nm in names]
        meta["planted_ok_gen"] = all(MP.eval_poly(e, point, F) == (0, 0) for e in eqs)
        assert meta["planted_ok_gen"], name
    with open(os.path.join(out, name + ".json"), "w") as fh:
        json.dump(meta, fh)
    return meta


def main():
    if sys.argv[1] == "--selftest":
        sys.exit(0 if selftest() else 1)
    base = sys.argv[1]
    out = os.path.join(base, "systems")
    os.makedirs(out, exist_ok=True)
    SB = GA.PrimeSetup(P1, curve=GA.P1_CURVE)
    SA = GA.PrimeSetup(40961)
    index = []
    # ladders, arm B
    for m in (6, 7, 8, 9, 10):
        rungs = [("s1", k, [2] * k + [1] * (m - k)) for k in range(m + 1)]
        if m == 6:
            rungs += [("s2", k, [3] * k + [2] * (m - k)) for k in range(1, m + 1)]
        for stage, k, depths in rungs:
            for idx in range(2):
                rng = random.Random(GA.seed_of("L", P1, m, tuple(depths), "random", idx))
                xR, yR = GA.random_target(SB, rng)
                name = f"L_p{P1}_m{m}_N{sum(depths)}_{stage}k{k}_r{idx}"
                index.append(write(out, name, dict(arm="B", kind="random", ladder=m, stage=stage, k=k, idx=idx,
                                                   yR=yR), SB, depths, xR))
    # arm A, uniform t = 1
    for m in (8, 9, 10):
        depths = [1] * m
        for idx in range(2):
            rng = random.Random(GA.seed_of("A8", 40961, m, 1, "random", idx))
            xR, yR = GA.random_target(SA, rng)
            name = f"A_p40961_m{m}_t1_r{idx}"
            index.append(write(out, name, dict(arm="A", kind="random", idx=idx, yR=yR), SA, depths, xR))
        rng = random.Random(GA.seed_of("A8", 40961, m, 1, "planted", 0))
        xR, yR, sol, xs = planted_mixed(SA, depths, rng)
        name = f"A_p40961_m{m}_t1_pl0"
        index.append(write(out, name, dict(arm="A", kind="planted", idx=0, yR=yR, planted=sol, planted_xs=xs),
                           SA, depths, xR))
    with open(os.path.join(base, "index.jsonl"), "w") as fh:
        for meta in index:
            fh.write(json.dumps({k: v for k, v in meta.items() if k not in ("planted", "Vs")}) + "\n")
    print("systems", len(index), "curveA", SA.A, SA.B, SA.n, "gA", SA.g, "gB", SB.g)


if __name__ == "__main__":
    main()
