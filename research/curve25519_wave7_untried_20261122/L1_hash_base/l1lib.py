#!/usr/bin/env python3
"""L1 instrument: factor-base membership rules vs C=2 table-and-probe yield.

Curve: Montgomery B y^2 = x^3 + A x^2 + x over F_p, E(F_p) cyclic of order n = 8 r.
Every point has a full discrete log k in Z/n (g = generator), used ONLY as an accelerator
(table of u for each log) and to draw targets; every counted hit is re-verified by real
affine point arithmetic (verify_sum), independent of the log model.
Weierstrass x of the engine's model is y_W = u + A/3, so "bits lo=L" is y_W in [L, L+W).
"""
import hashlib, math, random, sys, time
from array import array
import numpy as np

# family-rule members (curve25519_family family, frame_search README erratum E1), k = 24 per the
# wave-7 brief: the faithful member p = 2^24 - 75, A = 7842.
CURVES = {16: (65437, 1058), 18: (262133, 78), 20: (1048573, 926),
          22: (4194301, 43186), 24: (16777141, 7842)}
HASH_KEYS = {'key1': b'curve25519-wave7-L1-hashbase-key-1',
             'key2': b'curve25519-wave7-L1-hashbase-key-2'}   # key2 only for the A/A arm
C_SIZES = (0.2, 0.4)          # N = round(c * sqrt(r)); nominal unprojected yield c^2/4 = 1 %, 4 %
HITS_PER_BASE = 200           # MC design: expected verified hits per base (MC noise per arm = 0.31 % at B = 512)
PLANT = {'plant3': 1.0149, 'plant5': 1.0247}   # size factors; yield ratio ~ f^2 = 1.030, 1.050


def seed(*parts):
    return int.from_bytes(hashlib.sha256(('L1|v1|' + '|'.join(map(str, parts))).encode()).digest()[:8], 'big')


def isprime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True


def sqrt_mod(a, p):                      # p = 5 mod 8 for every family member
    a %= p
    if a == 0: return 0
    if pow(a, (p - 1) // 2, p) != 1: return None
    x = pow(a, (p + 3) // 8, p)
    if x * x % p != a: x = x * pow(2, (p - 1) // 4, p) % p
    return x


def count_points(p, A):
    x = np.arange(p, dtype=np.int64); x2 = x * x % p
    f = (x2 * x % p + A * x2 % p + x) % p
    sq = np.zeros(p, bool); sq[x2] = True
    z = int((f == 0).sum()); s = int((sq[f] & (f != 0)).sum())
    return 1 + z + 2 * s, z


class Curve:
    def __init__(self, k, build=True):
        self.k = k; self.p, self.A = CURVES[k]; p, A = self.p, self.A
        assert p % 8 == 5
        self.n, self.nroots2 = count_points(p, A)
        n = self.n; assert n % 8 == 0
        self.r = n // 8; assert isprime(self.r) and self.r > 2 ** (k - 3)
        self.twist = 2 * p + 2 - n; assert self.twist % 4 == 0 and isprime(self.twist // 4)
        self.A3 = A * pow(3, -1, p) % p
        if build: self._build()

    # ---- real affine arithmetic (the verification oracle) ----
    def add(self, P, Q):
        p, A = self.p, self.A
        if P is None: return Q
        if Q is None: return P
        (u1, v1), (u2, v2) = P, Q
        if u1 == u2:
            if (v1 + v2) % p == 0: return None
            lam = (3 * u1 * u1 + 2 * A * u1 + 1) * pow(2 * v1, -1, p) % p
        else:
            lam = (v2 - v1) * pow(u2 - u1, -1, p) % p
        u3 = (lam * lam - A - u1 - u2) % p
        return (u3, (lam * (u1 - u3) - v1) % p)

    def mul(self, k, P):
        R = None
        while k:
            if k & 1: R = self.add(R, P)
            P = self.add(P, P); k >>= 1
        return R

    def point(self, u):
        p, A = self.p, self.A
        v = sqrt_mod(u * u % p * u + A * u * u + u, p)
        return None if v is None else (u, v)

    def _build(self):
        p, A, n, r = self.p, self.A, self.n, self.r
        rng = random.Random(1)
        while True:
            u = rng.randrange(2, p); g = self.point(u)
            if g is None: continue
            if self.mul(n // 2, g) is not None and self.mul(8, g) is not None: break
        self.g = g
        gu, gv = g
        ulog = array('i', [0]) * p; uofk = array('i', [0]) * (n // 2 + 1)
        u1, v1 = gu, gv; ulog[u1] = 1; uofk[1] = u1
        P2 = self.add(g, g); u1, v1 = P2; ulog[u1] = 2; uofk[2] = u1
        for k in range(3, n // 2 + 1):
            lam = (gv - v1) * pow(gu - u1, -1, p) % p
            u3 = (lam * lam - A - u1 - gu) % p
            v1 = (lam * (u1 - u3) - v1) % p; u1 = u3
            ulog[u1] = k; uofk[k] = u1
        assert u1 == 0                                  # k = n/2 is T0 = (0, 0)
        self.ulog = np.frombuffer(ulog, dtype=np.int32); self.uofk = np.frombuffer(uofk, dtype=np.int32)
        self._keep = (ulog, uofk)
        ok = (self.ulog > 0) & (self.ulog % r != 0)     # affine points outside E[8]
        vu = np.nonzero(ok)[0].astype(np.int64); vk = self.ulog[vu].astype(np.int64)
        self.vu0, self.vk0 = vu, vk                     # valid columns, u order
        y = (vu + self.A3) % p; o = np.argsort(y, kind='stable')
        self.vy, self.vk = y[o], vk[o]                  # valid columns, Weierstrass-x order
        self.M = len(vu)

    def fold(self, x):
        return np.minimum(x, self.n - x)

    # ---- hash membership ----
    def hash_order(self, keyname, cap):
        key = HASH_KEYS[keyname]; sha = hashlib.sha256
        raw = b''.join(sha(key + int(u).to_bytes(4, 'little')).digest()[:8] for u in self.vu0.tolist())
        h = np.frombuffer(raw, dtype='>u8').astype(np.uint64)
        cap = min(cap, len(h))
        part = np.argpartition(h, cap - 1)[:cap]
        part = part[np.argsort(h[part])]
        return part, h[part]

    # ---- bases: arrays of folded logs, exactly N columns ----
    def b_bits_lo0(self, N): return self.vk[:N]

    def b_bits_rand(self, N, L):
        s = int(np.searchsorted(self.vy, L)) % self.M
        return self.vk[(s + np.arange(N)) % self.M]

    def b_rand(self, N, rng):
        return self.vk0[rng.choice(self.M, N, replace=False)]

    def _project_first(self, ks, N):
        kp = self.fold(8 * ks % self.n)
        _, first = np.unique(kp, return_index=True); first.sort()
        assert len(first) >= N
        return kp[first[:N]]

    def b_rand_proj(self, N, rng):
        idx = rng.choice(self.M, min(self.M, int(1.3 * N) + 10), replace=False)
        return self._project_first(self.vk0[idx], N)

    def b_hash(self, order, N, b): return self.vk0[order[b * N:(b + 1) * N]]

    def b_hash_proj(self, order, N, b):
        return self._project_first(self.vk0[order[b * N:b * N + int(1.3 * N) + 10]], N)

    # ---- real-arithmetic verification of R = +-P_i +-P_j ----
    def verify_sum(self, ua, ub, uR):
        P = self.point(ua); Q = self.point(ub)
        Qn = (Q[0], (-Q[1]) % self.p)
        for S in (self.add(P, Q), self.add(P, Qn)):
            if S is not None and S[0] == uR: return True
        return False

    def verify_shift(self, ua, ub, uR):
        """R + T0 = +-P_i +-P_j, with R+T0 computed by real addition of T0 = (0,0)."""
        R = self.point(uR); S = self.add(R, (0, 0))
        return S is not None and self.verify_sum(ua, ub, S[0])

    # ---- one base: exact coverage + MC probe with verification ----
    def eval_base(self, ks, j, m, want_t0=False):
        n, r, uofk = self.n, self.r, self.uofk
        L = np.asarray(ks, dtype=np.int64); N = len(L)
        S = np.concatenate([L, n - L]); M = (S[:, None] + S[None, :]) % n
        M8 = M & 7; nz = M != 0
        sub0 = (M8 == 0) & nz
        exact = int(np.unique(M[sub0]).size)            # distinct covered points of E[r]\{O}
        exact_t0 = -1
        if want_t0:
            R4 = (M[M8 == 4] - 4 * r) % n; R4 = R4[R4 != 0]
            exact_t0 = int(np.unique(np.concatenate([M[sub0], R4])).size)
        kf = np.minimum(M, n - M).ravel()
        uq, idx = np.unique(kf, return_index=True)
        if uq[0] == 0: uq, idx = uq[1:], idx[1:]
        uT = uofk[uq]; o = np.argsort(uT); uT, idx = uT[o], idx[o]
        j = j[:m]; kR = 8 * j                                   # R = [8 j] g, folded
        kR = np.minimum(kR, n - kR); uR = uofk[kR].astype(np.int64)
        pos = np.minimum(np.searchsorted(uT, uR), len(uT) - 1); hit = uT[pos] == uR
        hits = int(hit.sum()); ok = fail = 0
        for t in np.flatnonzero(hit):
            a, b = divmod(int(idx[pos[t]]), 2 * N)
            if self.verify_sum(int(uofk[L[a % N]]), int(uofk[L[b % N]]), int(uR[t])): ok += 1
            else: fail += 1
        hits_t0 = -1
        if want_t0:
            k0 = (8 * j + 4 * r) % n; k0 = np.minimum(k0, n - k0); u0 = uofk[k0].astype(np.int64)
            pos0 = np.minimum(np.searchsorted(uT, u0), len(uT) - 1); hit0 = uT[pos0] == u0
            allh = hit | hit0; hits_t0 = int(allh.sum())
            for t in np.flatnonzero(hit0 & ~hit):
                a, b = divmod(int(idx[pos0[t]]), 2 * N)
                if self.verify_shift(int(uofk[L[a % N]]), int(uofk[L[b % N]]), int(uR[t])): ok += 1
                else: fail += 1
        return [N, exact, exact_t0, m, hits, hits_t0, ok, fail]

    # ---- instrument checks against real arithmetic (once per cell) ----
    def instrument_check(self, rng, ns=2000):
        n, r, p, uofk = self.n, self.r, self.p, self.uofk; bad = 0
        for _ in range(ns):
            k1, k2 = rng.randrange(1, n // 2), rng.randrange(1, n // 2)
            P, Q = self.point(int(uofk[k1])), self.point(int(uofk[k2]))
            want = {int(uofk[min(x % n, n - x % n)]) for x in (k1 + k2, k1 - k2) if x % n}
            got = set()
            for S in (self.add(P, Q), self.add(P, (Q[0], -Q[1] % p))):
                if S is not None: got.add(S[0])
            bad += got != want
        for _ in range(300):                                    # projection and T0 shift
            k1 = rng.randrange(1, n // 2); P = self.point(int(uofk[k1]))
            R = self.mul(8, P)
            bad += R is None or R[0] != int(uofk[min(8 * k1 % n, n - 8 * k1 % n)])
            S = self.add(P, (0, 0))
            bad += S is None or S[0] * P[0] % p != 1
            if R is not None: bad += self.mul(r, R) is not None  # [8]P in E[r]
        return int(bad)


def facts(k):
    cv = Curve(k, build=False); p, A = cv.p, cv.A
    return dict(k=k, p=p, A=A, order=cv.n, r=cv.r, r_prime=isprime(cv.r), r_gt_2k3=cv.r > 2 ** (k - 3),
                twist_order=cv.twist, twist_r=cv.twist // 4, twist_r_prime=isprime(cv.twist // 4),
                rational_2torsion_roots_of_f=cv.nroots2,
                p_mod_8=p % 8, A2m4_nonsquare=pow((A * A - 4) % p, (p - 1) // 2, p) == p - 1,
                sqrt_r=math.sqrt(cv.r), N=[max(2, round(c * math.sqrt(cv.r))) for c in C_SIZES])


if __name__ == '__main__':
    import json
    ks = [int(a) for a in sys.argv[1:]] or sorted(CURVES)
    print(json.dumps([facts(k) for k in ks], indent=1))
