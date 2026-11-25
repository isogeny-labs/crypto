#!/usr/bin/env python3
"""Sibling divisors D - M from the shared Cantor step (PROTOCOL.md).

A line-by-line port of genus_ladder.rs `Fk` / `JCurve::cantor` / `reduce` with the
same multiplication and inversion ticks, plus `cantor_pm(D, M) -> (D + M, D - M)`
that shares xgcd(u1, u2), u1*u2 and e1*u1. Every number it prints is
[pcc:unverified] (L0, one shared VM, Python, counts only).

usage: sibwalk.py <cells.json> <out.jsonl>
"""
import hashlib
import json
import sys

M64 = (1 << 64) - 1


def mix64(z):
    z = (z + 0x9E3779B97F4A7C15) & M64
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


class Fk:
    """F_p polynomial arithmetic with genus_ladder.rs's ticks."""

    def __init__(self, p):
        self.p = p
        self.mul_n = 0
        self.inv_n = 0

    @staticmethod
    def trim(a):
        while a and a[-1] == 0:
            a.pop()
        return a

    def inv(self, a):
        self.inv_n += 1
        return pow(a, -1, self.p)

    def add(self, a, b):
        p = self.p
        n = max(len(a), len(b))
        return self.trim([((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)])

    def sub(self, a, b):
        p = self.p
        n = max(len(a), len(b))
        return self.trim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)])

    def neg(self, a):
        return [(-c) % self.p for c in a]

    def scale(self, a, s):
        self.mul_n += len(a)
        return self.trim([c * s % self.p for c in a])

    def mul(self, a, b):
        if not a or not b:
            return []
        p = self.p
        out = [0] * (len(a) + len(b) - 1)
        for i, ai in enumerate(a):
            for j, bj in enumerate(b):
                out[i + j] += ai * bj
        self.mul_n += len(a) * len(b)
        return self.trim([c % p for c in out])

    def divrem(self, a, b):
        assert b, "division by zero polynomial"
        p = self.p
        r = list(a)
        if len(a) < len(b):
            return [], r
        monic = b[-1] == 1
        li = 1 if monic else self.inv(b[-1])
        db = len(b) - 1
        q = [0] * (len(a) - len(b) + 1)
        while len(r) >= len(b) and r:
            shift = len(r) - 1 - db
            if monic:
                coef = r[-1]
            else:
                self.mul_n += 1
                coef = r[-1] * li % p
            q[shift] = coef
            for j in range(db):
                r[shift + j] = (r[shift + j] - coef * b[j]) % p
            self.mul_n += db
            r[shift + db] = 0
            self.trim(r)
        return self.trim(q), r

    def rem(self, a, b):
        return self.divrem(a, b)[1]

    def monic(self, a):
        if not a or a[-1] == 1:
            return a
        return self.scale(a, self.inv(a[-1]))

    def xgcd(self, a, b):
        r0, r1 = list(a), list(b)
        s0, s1 = [1], []
        t0, t1 = [], [1]
        while r1:
            q, r = self.divrem(r0, r1)
            r0, r1 = r1, r
            s = self.sub(s0, self.mul(q, s1))
            s0, s1 = s1, s
            t = self.sub(t0, self.mul(q, t1))
            t0, t1 = t1, t
        if not r0 or r0[-1] == 1:
            return r0, s0, t0
        li = self.inv(r0[-1])
        return self.scale(r0, li), self.scale(s0, li), self.scale(t0, li)


IDENT = ((1,), ())


class JCurve:
    def __init__(self, p, g, f):
        self.k = Fk(p)
        self.p = p
        self.g = g
        self.f = Fk.trim(list(f))

    # divisors are (u tuple monic low->high, v tuple trimmed)
    def reduce(self, u, v):
        k = self.k
        while len(u) - 1 > self.g:
            t = k.sub(self.f, k.mul(v, v))
            q, r = k.divrem(t, u)
            assert not r, "(f - v^2)/u is exact"
            un = k.monic(q)
            v = k.rem(k.neg(v), un)
            u = un
        u = k.monic(u)
        return (tuple(u), tuple(Fk.trim(list(v))))

    def cantor(self, a, b):
        if a == IDENT:
            return b
        if b == IDENT:
            return a
        k = self.k
        u1, v1 = list(a[0]), list(a[1])
        u2, v2 = list(b[0]), list(b[1])
        d1, e1, e2 = k.xgcd(u1, u2)
        if len(d1) == 1:
            u = k.mul(u1, u2)
            t = k.mul(k.mul(e1, u1), k.sub(v2, v1))
            v = k.rem(k.add(v1, t), u)
        else:
            d, c1, c2 = k.xgcd(d1, k.add(v1, v2))
            s1 = k.mul(c1, e1)
            s2 = k.mul(c1, e2)
            u, ur = k.divrem(k.mul(u1, u2), k.mul(d, d))
            assert not ur
            t1 = k.mul(k.mul(s1, u1), v2)
            t2 = k.mul(k.mul(s2, u2), v1)
            t3 = k.mul(c2, k.add(k.mul(v1, v2), self.f))
            vq, vr = k.divrem(k.add(k.add(t1, t2), t3), d)
            assert not vr
            v = k.rem(vq, u)
        return self.reduce(u, v)

    def negate(self, d):
        return (d[0], tuple((-c) % self.p for c in d[1]))

    def cantor_pm(self, a, b):
        """(a + b, a - b), sharing xgcd(u1,u2), u1*u2 and e1*u1 in the coprime case."""
        if a == IDENT or b == IDENT:
            return self.cantor(a, b), self.cantor(a, self.negate(b))
        k = self.k
        u1, v1 = list(a[0]), list(a[1])
        u2, v2 = list(b[0]), list(b[1])
        d1, e1, e2 = k.xgcd(u1, u2)
        if len(d1) != 1:
            return self.cantor(a, b), self.cantor(a, self.negate(b))
        u = k.mul(u1, u2)
        E = k.mul(e1, u1)
        P = k.mul(E, v2)
        Q = k.mul(E, v1)
        vp = k.rem(k.add(v1, k.sub(P, Q)), u)
        vm = k.rem(k.sub(v1, k.add(P, Q)), u)
        return self.reduce(list(u), vp), self.reduce(list(u), vm)

    def mul_ltr(self, d, n):
        acc = IDENT
        for i in range(n.bit_length() - 1, -1, -1):
            acc = self.cantor(acc, acc)
            if (n >> i) & 1:
                acc = self.cantor(acc, d)
        return acc

    def key(self, d):
        h = 0x243F6A8885A308D3 ^ (len(d[0]) - 1)
        u, v = d
        for i in range(len(u) - 1):
            vi = v[i] if i < len(v) else 0
            h = mix64(h ^ ((u[i] << 32) | vi))
        return h

    def evalf(self, x):
        acc = 0
        for c in reversed(self.f):
            acc = (acc * x + c) % self.p
        return acc

    def random_divisor(self, rng):
        p = self.p
        d = IDENT
        placed = 0
        while placed < self.g:
            x = rng.randrange(p)
            fx = self.evalf(x)
            if fx == 0 or pow(fx, (p - 1) // 2, p) != 1:
                continue
            y = sqrt_mod(fx, p)
            if rng.random() < 0.5:
                y = (-y) % p
            d = self.cantor(d, (((-x) % p, 1), (y,)))
            placed += 1
        return d


def sqrt_mod(a, p):
    a %= p
    if a == 0:
        return 0
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1:
            tt = tt * tt % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


# ---- uncounted helpers for the measurement (not part of either arm's cost) ----

def pmulmod(a, b, u, p):
    out = [0] * (len(a) + len(b) - 1) if a and b else []
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            out[i + j] += ai * bj
    out = [c % p for c in out]
    d = len(u) - 1
    for top in range(len(out) - 1, d - 1, -1):
        c = out[top]
        if c:
            for j in range(d):
                out[top - d + j] = (out[top - d + j] - c * u[j]) % p
        out[top] = 0
    return Fk.trim(out[:d] if len(out) > d else out)


def ppowx(u, e, p, shift=0):
    """(x + shift)^e mod u."""
    res = [1]
    base = Fk.trim([shift % p, 1]) if len(u) > 2 else [(shift - u[0]) % p]
    while e:
        if e & 1:
            res = pmulmod(res, base, u, p)
        base = pmulmod(base, base, u, p)
        e >>= 1
    return res


def splits(u, p):
    if len(u) == 2:
        return True
    xp = ppowx(list(u), p, p)
    return Fk.trim(list(xp)) == [0, 1]


def roots(u, p, rng):
    u = list(u)
    if len(u) == 1:
        return []
    if len(u) == 2:
        return [(-u[0]) % p]
    k = Fk(p)
    while True:
        a = rng.randrange(p)
        h = ppowx(u, (p - 1) // 2, p, a)
        h = k.sub(h, [1])
        if not h:
            continue
        g = k.xgcd(u, h)[0]
        if 1 < len(g) < len(u):
            q, r = k.divrem(u, g)
            return roots(g, p, rng) + roots(q, p, rng)


# ---- the walk ----

def seed_rng(tag):
    import random
    return random.Random(int.from_bytes(hashlib.sha256(tag.encode()).digest()[:8], "big"))


def cost(k, m0, i0):
    return (k.mul_n - m0) + 10 * (k.inv_n - i0), k.mul_n - m0, k.inv_n - i0


def run_cell(cell, arm, s, n_steps, restart=4096, verify=200):
    p, g, f, r, h = cell["p"], cell["g"], cell["f"], cell["r"], cell["h"]
    C = JCurve(p, g, f)
    k = C.k
    rng = seed_rng(f"rootfind_genus23|{p}|{cell['curve_seed']}|{s}")
    vrng = seed_rng(f"rootfind_genus23|verify|{p}|{cell['curve_seed']}|{s}")
    # generator of the order-r part and target
    while True:
        G = C.mul_ltr(C.random_divisor(rng), h)
        if G != IDENT:
            break
    klog = rng.randrange(1, r)
    T = C.mul_ltr(G, klog)
    short = min(r, 1 << 12)
    jumps = []
    for _ in range(16):
        a, b = rng.randrange(1, short), rng.randrange(1, short)
        jumps.append((C.cantor(C.mul_ltr(G, a), C.mul_ltr(T, b)), a, b))
    st = dict(arm=arm, s=s, p=p, g=g, curve_seed=cell["curve_seed"], steps=0, restarts=0,
              fwd_mul=0, fwd_inv=0, restart_mul=0, restart_inv=0,
              sib_mul=0, sib_inv=0, pm_mul=0, pm_inv=0, sibs=0, sib_skipped=0,
              fwd_cand=0, fwd_split=0, sib_cand=0, sib_split=0,
              dup_sib=0, n_mismatch=0, verified=0, verify_fail=0)
    seen = set()
    cur, ca, cb = IDENT, 0, 0
    since = restart
    jprev = None
    while st["steps"] < n_steps:
        if since >= restart:
            m0, i0 = k.mul_n, k.inv_n
            a, b = rng.randrange(1, r), rng.randrange(1, r)
            cur = C.cantor(C.mul_ltr(G, a), C.mul_ltr(T, b))
            ca, cb = a, b
            _, dm, di = cost(k, m0, i0)
            st["restart_mul"] += dm
            st["restart_inv"] += di
            st["restarts"] += 1
            since = 0
            jprev = None
            seen.add(cur)
            # the restart point itself is a candidate in both arms (as in the lab)
        j = (mix64(C.key(cur)) >> 24) % 16
        M, aj, bj = jumps[j]
        emit = arm in ("B", "N") and j != jprev
        m0, i0 = k.mul_n, k.inv_n
        if arm == "B" and emit:
            nxt, sib = C.cantor_pm(cur, M)
            _, dm, di = cost(k, m0, i0)
            st["pm_mul"] += dm
            st["pm_inv"] += di
        else:
            nxt = C.cantor(cur, M)
            _, dm, di = cost(k, m0, i0)
            st["fwd_mul"] += dm
            st["fwd_inv"] += di
            sib = None
            if arm == "N" and emit:
                m1, i1 = k.mul_n, k.inv_n
                sib = C.cantor(cur, C.negate(M))
                _, dm, di = cost(k, m1, i1)
                st["sib_mul"] += dm
                st["sib_inv"] += di
        if arm == "B" and emit:
            # exactness check, uncounted (counters restored)
            m1, i1 = k.mul_n, k.inv_n
            ref_n = C.cantor(cur, M)
            ref_s = C.cantor(cur, C.negate(M))
            k.mul_n, k.inv_n = m1, i1
            if ref_n != nxt or ref_s != sib:
                st["n_mismatch"] += 1
        if not emit and arm in ("B", "N"):
            st["sib_skipped"] += 1
        sca, scb = ca - aj, cb - bj
        cur, ca, cb = nxt, ca + aj, cb + bj
        jprev = j
        since += 1
        st["steps"] += 1
        # forward candidate
        if len(cur[0]) - 1 == g:
            st["fwd_cand"] += 1
            if splits(cur[0], p):
                st["fwd_split"] += 1
        seen.add(cur)
        if sib is not None:
            st["sibs"] += 1
            if sib in seen:
                st["dup_sib"] += 1
            seen.add(sib)
            if len(sib[0]) - 1 == g:
                st["sib_cand"] += 1
                if splits(sib[0], p):
                    st["sib_split"] += 1
                    if st["verified"] < verify and arm == "B":
                        m1, i1 = k.mul_n, k.inv_n
                        ok = True
                        xs = roots(sib[0], p, vrng)
                        acc = IDENT
                        for x in xs:
                            fx = C.evalf(x)
                            vy = 0
                            for cf in reversed(sib[1]):
                                vy = (vy * x + cf) % p
                            ok &= vy * vy % p == fx
                            acc = C.cantor(acc, (((-x) % p, 1), (vy,) if vy else ()))
                        ok &= acc == sib
                        e = (sca + klog * scb) % r
                        ok &= C.mul_ltr(G, e) == sib
                        k.mul_n, k.inv_n = m1, i1
                        st["verified"] += 1
                        st["verify_fail"] += 0 if ok else 1
    return st


def main():
    cells = json.load(open(sys.argv[1]))
    out = open(sys.argv[2], "a")
    arms = sys.argv[3].split(",") if len(sys.argv) > 3 else ["A", "B", "N", "AA"]
    for cell in cells:
        n = cell["steps"]
        for arm in arms:
            a, s = ("A", 2) if arm == "AA" else (arm, 1)
            st = run_cell(cell, a, s, n)
            st["arm_label"] = arm
            out.write(json.dumps(st) + "\n")
            out.flush()
            print(arm, cell["p"], cell["g"], cell["curve_seed"],
                  "fwd/step %.1f" % ((st["fwd_mul"] + st["pm_mul"]) / st["steps"]), file=sys.stderr)


if __name__ == "__main__":
    main()
