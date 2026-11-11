#!/usr/bin/env python3
"""Trace-zero factor-base toy test (see PROTOCOL.md and Amendment A1). Stdlib only.

Run: python3 -I tracezero.py --out DIR [--curves 200 --vsets 20 --ps 11,13,...]
"""
import argparse
import hashlib
import json
import os
import random
from multiprocessing import Pool


class Field:
    """F_{p^k}: elements are ints 0..q-1 (base-p digits, digit 0 constant)."""

    def __init__(self, p, k):
        self.p, self.k, self.q = p, k, p ** k
        rnd = random.Random(1000 * p + k)
        q = self.q
        while True:
            c = [rnd.randrange(p) for _ in range(k)]
            if c[0] == 0:
                continue
            exp = []
            cur = [1] + [0] * (k - 1)
            ok = True
            for i in range(q - 1):
                e = self._enc(cur)
                if i > 0 and e == 1:
                    ok = False
                    break
                exp.append(e)
                top = cur[-1]
                cur = [0] + cur[:-1]
                for j in range(k):
                    cur[j] = (cur[j] + top * c[j]) % p
            if ok and self._enc(cur) == 1:
                break
        self.exp = exp
        self.log = [0] * q
        for i, e in enumerate(exp):
            self.log[e] = i
        self.sq = {0: [0]}
        for y in range(1, q):
            self.sq.setdefault(self.mul(y, y), []).append(y)

    def _enc(self, d):
        r = 0
        for x in reversed(d):
            r = r * self.p + x
        return r

    def add(self, a, b):
        p, r, m = self.p, 0, 1
        for _ in range(self.k):
            r += ((a % p + b % p) % p) * m
            a //= p
            b //= p
            m *= p
        return r

    def neg(self, a):
        p, r, m = self.p, 0, 1
        for _ in range(self.k):
            r += ((-(a % p)) % p) * m
            a //= p
            m *= p
        return r

    def mul(self, a, b):
        if a == 0 or b == 0:
            return 0
        return self.exp[(self.log[a] + self.log[b]) % (self.q - 1)]

    def inv(self, a):
        return self.exp[(-self.log[a]) % (self.q - 1)]

    def frob(self, a):
        if a == 0:
            return 0
        return self.exp[(self.log[a] * self.p) % (self.q - 1)]


def padd(F, a, P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if F.add(y1, y2) == 0:
            return None
        num = F.add(F.mul(3, F.mul(x1, x1)), a)
        lam = F.mul(num, F.inv(F.mul(2, y1)))
    else:
        lam = F.mul(F.add(y2, F.neg(y1)), F.inv(F.add(x2, F.neg(x1))))
    x3 = F.add(F.add(F.mul(lam, lam), F.neg(x1)), F.neg(x2))
    y3 = F.add(F.mul(lam, F.add(x1, F.neg(x3))), F.neg(y1))
    return (x3, y3)


SALT = os.environ.get("TZ_SALT", "")  # A5: independent seeds for the replication


def seed(*parts):
    h = hashlib.sha256("|".join(map(str, (SALT,) + parts if SALT else parts)).encode()).digest()
    return int.from_bytes(h[:8], "big")


class Curve:
    """E(F_p) with an addition table over point indices (0 = identity)."""

    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b
        pts = [None]
        for x in range(p):
            f = (x * x * x + a * x + b) % p
            for y in range(p):
                if (y * y) % p == f:
                    pts.append((x, y))
        self.pts = pts
        self.n = len(pts)
        self.idx = {P: i for i, P in enumerate(pts)}
        n = self.n
        self.neg = [self.idx[None if P is None else (P[0], (-P[1]) % p)] for P in pts]
        self.T = [[self._add(pts[i], pts[j]) for j in range(n)] for i in range(n)]

    def _add(self, P, Q):
        p = self.p
        if P is None:
            return self.idx[Q]
        if Q is None:
            return self.idx[P]
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0:
                return 0
            lam = (3 * P[0] * P[0] + self.a) * pow(2 * P[1], -1, p) % p
        else:
            lam = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x3 = (lam * lam - P[0] - Q[0]) % p
        y3 = (lam * (P[0] - x3) - P[1]) % p
        return self.idx[(x3, y3)]

    def subgroup_image(self):
        for mult in (2, 3):
            H = set()
            for i in range(self.n):
                r = 0
                for _ in range(mult):
                    r = self.T[r][i]
                H.add(r)
            if len(H) < self.n:
                return sorted(H)
        return None

    def sub_stats(self, draws, s, rnd):
        """Coverage using s random distinct non-identity draws (unsaturated regime)."""
        pool = sorted(set(d for d in draws if d != 0))
        if len(pool) < s:
            return None
        return self.stats(rnd.sample(pool, s))

    def stats(self, draws):
        """draws: list of point indices (any). Returns (|S|, cov2, cov3)."""
        S = set(d for d in draws if d != 0)
        S |= {self.neg[s] for s in S}
        if not S:
            return 0, 0.0, 0.0
        T = self.T
        S2 = {T[i][j] for i in S for j in S}
        S3 = {T[s][j] for s in S2 for j in S}
        t = self.n - 1
        return len(S), len(S2 - {0}) / t, len(S3 - {0}) / t


def lifts_trace(F, curve, V, a_int, b_int):
    """Return (number of lifts, list of E(F_p) indices of Tr(Q)) for x in V."""
    k = F.k
    out = []
    nl = 0
    for x in V:
        x3 = F.mul(F.mul(x, x), x)
        f = F.add(F.add(x3, F.mul(a_int, x)), b_int)
        if f not in F.sq:
            continue
        for y in F.sq[f]:
            nl += 1
            Q = (x, y)
            S = Q
            cur = Q
            for _ in range(k - 1):
                cur = (F.frob(cur[0]), F.frob(cur[1]))
                S = padd(F, a_int, S, cur)
            if S is None:
                out.append(0)
            else:
                assert S[0] < F.p and S[1] < F.p
                out.append(curve.idx[S])
    return nl, out


def random_subspace(F, d, rnd):
    p = F.p
    while True:
        gens = [rnd.randrange(1, F.q) for _ in range(d)]
        span = {0}
        for g in gens:
            new = set()
            for s in span:
                m = 0
                for c in range(p):
                    new.add(F.add(s, m))
                    m = F.add(m, g)
            span = new
        if len(span) == p ** d:
            if EXCL and any(g and F.frob(g) in (g, F.neg(g)) for g in span):
                continue  # A6: drop degenerate V (contains g with g^p = +-g)
            return sorted(span)


SYM = bool(int(os.environ.get("TZ_SYM", "0")))
EXCL = bool(int(os.environ.get("TZ_EXCL", "0")))
SUB = 3  # A2: small factor-base size for the unsaturated arm
_F = {}


def work(job):
    p, k, d, a, b, nv = job
    if (p, k) not in _F:
        _F.clear()
        _F[(p, k)] = Field(p, k)
    F = _F[(p, k)]
    curve = Curve(p, a, b)
    H = curve.subgroup_image()
    rnd = random.Random(seed("rows", p, k, d, a, b))
    acc = {"trace": [], "aa": [], "rand": [], "pos": []}
    accs = {"trace": [], "aa": [], "rand": [], "pos": []}
    for _ in range(nv):
        V0 = random_subspace(F, d, rnd)
        V1 = random_subspace(F, d, rnd)
        n0, t0 = lifts_trace(F, curve, V0, a, b)
        n1, t1 = lifts_trace(F, curve, V1, a, b)
        if SYM:  # A4: null draws come in +-pairs, like lifts (Q, -Q)
            r0 = [rnd.randrange(curve.n) for _ in range(n0 // 2)]
            r0 += [curve.neg[i] for i in r0]
            h0 = None
            if H:
                h0 = [rnd.choice(H) for _ in range(n0 // 2)]
                h0 += [curve.neg[i] for i in h0]
        else:
            r0 = [rnd.randrange(curve.n) for _ in range(n0)]
            h0 = [rnd.choice(H) for _ in range(n0)] if H else None
        for name, dr in (("trace", t0), ("aa", t1), ("rand", r0), ("pos", h0)):
            if dr is None:
                continue
            acc[name].append(curve.stats(dr))
            sb = curve.sub_stats(dr, SUB, rnd)
            if sb is not None:
                accs[name].append(sb)
    row = {"p": p, "k": k, "d": d, "a": a, "b": b, "N": curve.n,
           "pos_ok": bool(H), "nv": nv}
    for name, v in acc.items():
        row[name] = [sum(x[i] for x in v) / len(v) for i in range(3)] if v else None
    for name, v in accs.items():
        row["sub_" + name] = ([sum(x[i] for x in v) / len(v) for i in range(3)], len(v)) if v else None
    return row


def curves_for(p, k, n):
    rnd = random.Random(seed("curves", p, k))
    seen, out = set(), []
    nonsing = sum(1 for a in range(p) for b in range(p) if (4 * a ** 3 + 27 * b * b) % p)
    n = min(n, nonsing)  # p = 11, 13 have fewer than 200 non-singular (a, b)
    while len(out) < n:
        a, b = rnd.randrange(p), rnd.randrange(p)
        if (4 * a ** 3 + 27 * b * b) % p == 0 or (a, b) in seen:
            continue
        seen.add((a, b))
        out.append((a, b))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--curves", type=int, default=200)
    ap.add_argument("--vsets", type=int, default=20)
    ap.add_argument("--ps", default="11,13,17,19,23,29,31")
    ap.add_argument("--ks", default="2,3")
    ap.add_argument("--ds", default="1,2")
    ap.add_argument("--procs", type=int, default=4)
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)
    jobs = []
    for p in map(int, args.ps.split(",")):
        for k in map(int, args.ks.split(",")):
            for d in map(int, args.ds.split(",")):
                for a, b in curves_for(p, k, args.curves):
                    jobs.append((p, k, d, a, b, args.vsets))
    path = os.path.join(args.out, "rows.jsonl")
    with Pool(args.procs) as pool, open(path, "w") as fh:
        for r in pool.imap(work, jobs, chunksize=4):
            fh.write(json.dumps(r, sort_keys=True) + "\n")
    print(path, len(jobs))


if __name__ == "__main__":
    main()
