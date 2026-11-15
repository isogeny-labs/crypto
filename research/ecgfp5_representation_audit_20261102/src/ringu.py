"""Pornin-u ring-form quotient of the m-summand decomposition system on a double-odd curve.

H_R(u_1..u_m) = 0  iff  there are P_i with u(P_i) = u_i (u = x/y) and sum P_i = R.
H_R is invariant under permutations and even sign changes (translation of an even number of
summands by N = (0,0), since u(P+N) = -u(P)), so H_R lies in K[s_1..s_{m-1}, pi] with
s_k = e_k(u_i^2), pi = prod u_i (FHJRV Prop. 13), s_m = pi^2.
The coefficients are found by exact interpolation over F_q from random genuine decompositions
(points need not be in the factor base) and checked on fresh decompositions and non-decompositions.
Then u_i = mu * v_i with v_i in F_p and Weil descent in the basis 1, z, ..., z^{n-1}.

usage: ringu.py p m seed mu_mode(one|rand) outprefix
"""
import itertools
import json
import random
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from fq import Fq, Curve  # noqa: E402


def esym(F, vals, k):
    # elementary symmetric e_k of a list of F_q elements
    e = [F.one()] + [F.zero()] * len(vals)
    for v in vals:
        for j in range(len(vals), 0, -1):
            e[j] = F.add(e[j], F.mul(e[j - 1], v))
    return e[k]


def monomials(m):
    D = 2 ** (m - 2)
    mons = []
    # even part: s_1^a1..s_m^am with sum <= D  (s_m = pi^2)  -> exponent of pi is 2*am
    # odd part: pi * s^a with sum <= D-1
    for par, deg in ((0, D), (1, D - 1)):
        for a in itertools.product(range(deg + 1), repeat=m):
            if sum(a) <= deg:
                sexp = list(a[: m - 1])
                piexp = 2 * a[m - 1] + par
                mons.append((tuple(sexp), piexp))
    return mons


def eval_mons(F, mons, us):
    sq = [F.mul(u, u) for u in us]
    s = [esym(F, sq, k) for k in range(1, len(us))]
    pi = F.one()
    for u in us:
        pi = F.mul(pi, u)
    row = []
    for sexp, pe in mons:
        v = F.pow(pi, pe)
        for sk, e in zip(s, sexp):
            if e:
                v = F.mul(v, F.pow(sk, e))
        row.append(v)
    return row


def nullspace(F, rows, ncols):
    # Gaussian elimination over F_q; return a basis of the right kernel
    M = [list(r) for r in rows]
    piv_cols, r = [], 0
    for col in range(ncols):
        pr = None
        for i in range(r, len(M)):
            if not F.is_zero(M[i][col]):
                pr = i
                break
        if pr is None:
            continue
        M[r], M[pr] = M[pr], M[r]
        iv = F.inv(M[r][col])
        M[r] = [F.mul(iv, x) for x in M[r]]
        for i in range(len(M)):
            if i != r and not F.is_zero(M[i][col]):
                f = M[i][col]
                M[i] = [F.sub(x, F.mul(f, y)) for x, y in zip(M[i], M[r])]
        piv_cols.append(col)
        r += 1
        if r == len(M):
            break
    free = [c for c in range(ncols) if c not in piv_cols]
    basis = []
    for fc in free:
        v = [F.zero()] * ncols
        v[fc] = F.one()
        for i, pc in enumerate(piv_cols):
            v[pc] = F.neg(M[i][fc])
        basis.append(v)
    return basis


def setup(p, m):
    n = m
    # c: smallest integer >= 2 not an n-th power mod p (needs p = 1 mod n)
    assert (p - 1) % n == 0
    c = next(cc for cc in range(2, p) if pow(cc, (p - 1) // n, p) != 1)
    if n % 2 == 0:
        # n = 4, p = 1 mod 4: z^4 - c is irreducible iff c is a non-square
        c = next(cc for cc in range(2, p) if pow(cc, (p - 1) // 2, p) != 1)
    F = Fq(p, n, c)
    a = F.const(2)
    z = (0, 1) + (0,) * (n - 2)
    for b0 in range(1, p):
        b = F.mul(F.const(b0), z)
        if not F.is_square(b) and not F.is_square(F.sub(F.mul(a, a), F.mul(F.const(4), b))):
            break
    return F, Curve(F, a, b), c, b0


def build(p, m, seed, mu_mode):
    rng = random.Random(seed)
    F, E, c, b0 = setup(p, m)
    mu = F.one() if mu_mode == "one" else F.rand(rng)
    # planted target: m factor-base points (u(P)/mu in F_p)
    planted_v, Ps = [], []
    while len(Ps) < m:
        v = rng.randrange(1, p)
        pts = E.points_with_u(F.mul(mu, F.const(v)), rng)
        if pts:
            planted_v.append(v)
            Ps.append(pts[rng.randrange(2)])
    R = None
    for P in Ps:
        R = E.add(R, P)
    mons = monomials(m)
    N = len(mons)

    def decomp(target):
        while True:
            Q = [E.rand_point(rng) for _ in range(m - 1)]
            S = None
            for P in Q:
                S = E.add(S, P)
            last = E.add(target, E.neg(S))
            pts = Q + [last]
            us = [E.u(P) for P in pts]
            if all(u is not None for u in us):
                return us

    rows = [eval_mons(F, mons, decomp(R)) for _ in range(N + 8)]
    ker = nullspace(F, rows, N)
    assert len(ker) == 1, f"kernel dim {len(ker)}"
    h = ker[0]

    def val(us):
        r = eval_mons(F, mons, us)
        acc = F.zero()
        for x, y in zip(r, h):
            acc = F.add(acc, F.mul(x, y))
        return acc

    # checks: vanishes on fresh decompositions, of R; not on random tuples / decompositions of R+N
    ok_fresh = all(F.is_zero(val(decomp(R))) for _ in range(24))
    Nn = (F.zero(), F.zero())
    ok_RN = sum(F.is_zero(val(decomp(E.add(R, Nn)))) for _ in range(8))
    ok_rand = sum(F.is_zero(val([F.rand(rng) for _ in range(m)])) for _ in range(8))
    planted_u = [F.mul(mu, F.const(v)) for v in planted_v]
    ok_planted = F.is_zero(val(planted_u))
    # descend: u = mu v -> s_k(u) = mu^{2k} s_k(v), pi(u) = mu^m pi(v)
    eqs = [dict() for _ in range(m)]
    for (sexp, pe), coef in zip(mons, h):
        if F.is_zero(coef):
            continue
        scale = F.pow(mu, m * pe + sum(2 * (k + 1) * e for k, e in enumerate(sexp)))
        cc = F.mul(coef, scale)
        for j in range(m):
            if cc[j]:
                eqs[j][(sexp, pe)] = cc[j]
    # planted point in (s_1..s_{m-1}, pi) over F_p
    sv = [esym(Fq(p, 1, 1), [((v * v) % p,) for v in planted_v], k)[0] for k in range(1, m)]
    piv = 1
    for v in planted_v:
        piv = piv * v % p
    return dict(p=p, m=m, c=c, b0=b0, mu=mu, nterms=sum(not F.is_zero(x) for x in h), nmons=N,
                checks=dict(fresh=ok_fresh, zero_on_R_plus_N=ok_RN, zero_on_random=ok_rand,
                            planted=ok_planted),
                eqs=eqs, planted=dict(v=planted_v, s=sv, pi=piv))


def to_singular(res, path):
    m, p = res["m"], res["p"]
    var = [f"s{k}" for k in range(1, m)] + ["w"]

    def mono(sexp, pe):
        parts = [f"{var[k]}^{e}" for k, e in enumerate(sexp) if e] + ([f"w^{pe}"] if pe else [])
        return "*".join(parts) if parts else "1"
    polys = []
    for eq in res["eqs"]:
        terms = [f"{c}*{mono(*k)}" for k, c in eq.items()]
        polys.append("+".join(terms) if terms else "0")
    with open(path, "w") as f:
        f.write(f'ring r = {p},({",".join(var)}),dp;\n')
        f.write("ideal I = " + ",\n".join(polys) + ";\n")
        f.write("option(redSB); ideal G = std(I);\n")
        f.write('print("vdim"); print(vdim(G)); print("dim"); print(dim(G));\n')
        pl = res["planted"]
        sub = ",".join(str(x) for x in pl["s"] + [pl["pi"]])
        f.write(f"map phi = r,{sub}; print(\"planted_in_V\"); print(phi(I));\n")
        f.write("quit;\n")
    with open(path.replace(".sing", ".ms"), "w") as f:
        f.write(",".join(var) + "\n" + str(p) + "\n")
        f.write(",\n".join(x.replace("*w", "*w") for x in polys) + "\n")


if __name__ == "__main__":
    p, m, seed, mode, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5]
    res = build(p, m, seed, mode)
    to_singular(res, out + ".sing")
    js = {k: v for k, v in res.items() if k != "eqs"}
    js["eq_terms"] = [len(e) for e in res["eqs"]]
    json.dump(js, open(out + ".json", "w"), indent=1, default=str)
    print(json.dumps(js, default=str))
