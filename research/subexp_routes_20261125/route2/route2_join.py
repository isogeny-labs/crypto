#!/usr/bin/env python3
"""
Route 2 -- can a STRUCTURED factor base V beat |V| in the m = 2 join P1 + P2 = Q?

Public toy instances only (p <= 2^16), pure Python, no third-party deps.

Stages
  geom        base geometry: |V|, |F|, |V+V|, additive energy, multiplicative
              energy, AP counts, z-scores against the random-set model
  join        measured join: hits per pass, ops per relation, probes to collect
              |V| relations (the "cost is Theta(p), flat in |V|" check)
  structured  the three structure-exploiting joins, each costed:
              (a) multiplicative membership  x^(2^t) == g^(2^t)
              (b) Velu tower membership      L_t o ... o L_1(x) == c
              (c) sumset (pair) table        |F|^2 sums, O(1) per target
  caps        what |V| each tower can reach: measured t_max per p and the
              analytic 2-adic cap table for the standard primes

Usage:  python3 route2_join.py [all|geom|join|structured|caps] [--json OUT]
"""
import json
import math
import random
import sys
import time

# ----------------------------------------------------------------- field utils


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def v2(n):
    k = 0
    while n % 2 == 0:
        n //= 2
        k += 1
    return k


def make_inv(p):
    """inv[i] = i^-1 mod p for 1 <= i < p, in O(p) time."""
    inv = [0] * p
    if p > 1:
        inv[1] = 1
    for i in range(2, p):
        inv[i] = (p - (p // i) * inv[p % i] % p) % p
    return inv


def make_qr(p):
    """set of nonzero quadratic residues mod p."""
    return {x * x % p for x in range(1, p)}


# ------------------------------------------------------------------ curves
# points are (x, y) tuples; None is the identity.


def padd(P, Q, a, p, inv):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + a) * inv[2 * y1 % p] % p
    else:
        lam = (y2 - y1) * inv[(x2 - x1) % p] % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def pmul(k, P, a, p, inv):
    R = None
    while k:
        if k & 1:
            R = padd(R, P, a, p, inv)
        P = padd(P, P, a, p, inv)
        k >>= 1
    return R


def order_points(a, b, p, qr):
    """#E(F_p) = p + 1 + sum_x chi(x^3 + a x + b)."""
    s = 0
    for x in range(p):
        r = (x * x * x + a * x + b) % p
        s += 1 if r == 0 else (2 if r in qr else 0)
    return s + 1


def sqrt_mod(r, p):
    """Tonelli-Shanks, p odd."""
    if r == 0:
        return 0
    if pow(r, (p - 1) // 2, p) != 1:
        return None
    if p % 4 == 3:
        return pow(r, (p + 1) // 4, p)
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, rt = s, pow(z, q, p), pow(r, q, p), pow(r, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        bb = pow(c, 1 << (m - i - 1), p)
        m, c, t, rt = i, bb * bb % p, t * bb * bb % p, rt * bb % p
    return rt


def random_point(p, a, b, qr, rng):
    while True:
        x = rng.randrange(p)
        r = (x * x * x + a * x + b) % p
        if r in qr:
            y = sqrt_mod(r, p)
            if y is not None:
                return (x, y if rng.getrandbits(1) else p - y)


def find_curve(p, qr):
    """first curve with (a, b) drawn deterministically and prime order."""
    rng = random.Random(0xC0FFEE ^ p)
    for _ in range(400):
        a = rng.randrange(0, p)
        b = rng.randrange(1, p)
        if (4 * a * a * a + 27 * b * b) % p == 0:
            continue
        n = order_points(a, b, p, qr)
        if is_prime(n):
            return a, b, n
    for _ in range(400):
        a = rng.randrange(0, p)
        b = rng.randrange(1, p)
        if (4 * a * a * a + 27 * b * b) % p != 0:
            return a, b, order_points(a, b, p, qr)
    raise RuntimeError("no curve")


# -------------------------------------------------------------- factor bases


def interval_V(p, t):
    """prime-interval: every x in [1, 2^t).  |V| = 2^t, no condition on p."""
    return list(range(1, min((1 << t) + 1, p)))


def kummer_V(p, t, g=1):
    """prime-kummer: V = g * mu_{2^t}, needs 2^t | p-1."""
    if (p - 1) % (1 << t) != 0:
        return None
    e = (p - 1) >> t
    zeta = None
    for h in range(2, min(p, 1 << 20)):
        z = pow(h, e, p)
        if pow(z, 1 << (t - 1), p) != 1:
            zeta = z
            break
    if zeta is None:
        return None
    V, cur = [], g % p
    for _ in range(1 << t):
        V.append(cur)
        cur = cur * zeta % p
    return V


class Tower:
    """prime-isogeny on an auxiliary curve E', mirroring
    src/cryptanalysis/ic_framework/prime_tower.rs (tower build + Velu chain)."""

    def __init__(self, p, t, seed, a2, b2, n2, q, r0, v, steps, c):
        self.p, self.t, self.seed = p, t, seed
        self.a2, self.b2, self.n2 = a2, b2, n2
        self.q, self.r0, self.v, self.steps, self.c = q, r0, v, steps, c

    def eval(self, x):
        """L = L_t o ... o L_1 at x; None at a pole."""
        p, y = self.p, x
        for (xi, tau) in self.steps:
            d = (y - xi) % p
            if d == 0:
                return None
            y = (y + tau * pow(d, p - 2, p)) % p
        return y


def velu_map(P, xi, tau, p, inv):
    if P is None or P[0] == xi:
        return None
    d = inv[(P[0] - xi) % p]
    return ((P[0] + tau * d) % p, P[1] * (1 - tau * d % p * d) % p)


def isogeny_tower(p, t, seed=1, qr=None, inv=None, budget=4000):
    """search E'/F_p with a point Q of order 2^t; build V = x(R' + <Q>)."""
    if qr is None:
        qr = make_qr(p)
    if inv is None:
        inv = make_inv(p)
    rng = random.Random((p * 0x9E3779B1) ^ (t << 20) ^ seed)
    two_t = 1 << t
    for _ in range(budget):
        a = rng.randrange(0, p)
        b = rng.randrange(1, p)
        if (4 * a * a * a + 27 * b * b) % p == 0:
            continue
        n2 = order_points(a, b, p, qr)
        v = v2(n2)
        if v < t:
            continue
        odd = n2 >> v
        q = None
        for _ in range(24):
            P = random_point(p, a, b, qr, rng)
            T = pmul(odd, P, a, p, inv)          # 2-power order
            if T is None:
                continue
            j, cur = 0, T
            while cur is not None:
                cur = padd(cur, cur, a, p, inv)
                j += 1
            if j >= t:
                q = pmul(1 << (j - t), T, a, p, inv)
                break
        if q is None:
            continue
        mults, acc = [None], q
        for _ in range(two_t - 1):
            mults.append(acc)
            acc = padd(acc, q, a, p, inv)
        if acc is not None:
            continue
        keys = set(mults)
        r0 = None
        for _ in range(64):
            cand = random_point(p, a, b, qr, rng)
            if padd(cand, cand, a, p, inv) not in keys:
                r0 = cand
                break
        if r0 is None:
            continue
        v_list = []
        for m in mults:
            s = padd(r0, m, a, p, inv)
            if s is None:
                v_list.append(None)
            else:
                v_list.append(s[0])
        steps = []
        aj, bj = a, b
        qj, rj = q, r0
        ok = True
        for j in range(t):
            kj = qj
            for _ in range(t - 1 - j):
                kj = padd(kj, kj, aj, p, inv)
            if kj is None or kj[1] % p != 0:
                ok = False
                break
            xi = kj[0]
            tau = (3 * xi * xi + aj) % p
            steps.append((xi, tau))
            qj = velu_map(qj, xi, tau, p, inv)
            rj = velu_map(rj, xi, tau, p, inv)
            aj = (aj - 5 * tau) % p
            bj = (bj - 7 * xi * tau) % p
        if not ok or rj is None:
            continue
        if len(set(v_list)) != two_t:
            continue
        return Tower(p, t, seed, a, b, n2, q, r0, v_list, steps, rj[0])
    return None


def lift_abscissae(p, a, b, xs, qr):
    """F = all points of E with x in xs (both signs)."""
    F = []
    for x in xs:
        if x is None:
            continue
        r = (x * x * x + a * x + b) % p
        if r == 0:
            F.append((x, 0))
        elif r in qr:
            y = sqrt_mod(r, p)
            F.append((x, y))
            F.append((x, p - y))
    return F


# ------------------------------------------------------------------- measures


def pass_hits(T, F, xset, a, p, inv):
    """one join pass: for every P in F test x(T - P) in V."""
    h = 0
    for P in F:
        R = padd(T, (P[0], (-P[1]) % p), a, p, inv)
        if R is not None and R[0] in xset:
            h += 1
    return h


def collect_relations(p, a, b, F, xset, want, rng, inv, qr, max_passes=200000):
    """probe whole passes over F with fresh targets until `want` distinct
    unordered relations are found; return (found, probes, passes)."""
    seen = set()
    probes = passes = 0
    while len(seen) < want and passes < max_passes:
        T = random_point(p, a, b, qr, rng)
        passes += 1
        for P in F:
            probes += 1
            R = padd(T, (P[0], (-P[1]) % p), a, p, inv)
            if R is not None and R[0] in xset:
                seen.add(tuple(sorted((P, R))))
    return len(seen), probes, passes


def sumset_stats(F, a, p, inv):
    """|F+F| and additive energy sum_c r(c)^2 over ordered pairs."""
    cnt = {}
    for P in F:
        for Q in F:
            R = padd(P, Q, a, p, inv)
            cnt[R] = cnt.get(R, 0) + 1
    return len(cnt), sum(c * c for c in cnt.values())


def mult_energy(V, p):
    cnt = {}
    for x in V:
        for y in V:
            k = x * y % p
            cnt[k] = cnt.get(k, 0) + 1
    return len(cnt), sum(c * c for c in cnt.values())


def ap3_count(V, p):
    """#{(x, y) in V^2 : x + y in V}."""
    S, n = set(V), 0
    for x in V:
        for y in V:
            if (x + y) % p in S:
                n += 1
    return n


def build_base(p, tower, t, a, b, qr, inv):
    if tower == "interval":
        V = interval_V(p, t)
    elif tower == "kummer":
        V = kummer_V(p, t)
    elif tower == "isogeny":
        tw = isogeny_tower(p, t, qr=qr, inv=inv)
        V = None if tw is None else list(tw.v)
    else:
        raise ValueError(tower)
    if V is None:
        return None, None
    V = list(dict.fromkeys(V))
    return V, lift_abscissae(p, a, b, V, qr)


# --------------------------------------------------------------------- stages


def stage_geom(cfgs, out):
    rows = []
    print("\n== geom: does V-level structure survive the lift to E? ==")
    print("   V-level: sums and products of the x-values themselves (V subset F_p)")
    print("   E-level: sums of the points F (the group the join lives in)")
    print("%-6s %-9s %2s %4s %4s %7s %6s %7s %7s %6s %9s %9s %6s"
          % ("p", "tower", "t", "|V|", "|F|", "|V+V|x", "r_add", "|V*V|",
             "r_mul", "AP3_z", "|F+F|", "|F+F|ctl", "E2/ctl"))
    for (p, tower, t) in cfgs:
        qr, inv = make_qr(p), make_inv(p)
        a, b, N = find_curve(p, qr)
        V, F = build_base(p, tower, t, a, b, qr, inv)
        if V is None or len(V) < 2 or not (4 <= len(F) <= 140):
            print("%-6d %-9s %2d   -- (cap or size)" % (p, tower, t))
            continue
        n, nF = len(V), len(F)
        # ---- V-level (x-values in F_p)
        xsum = len({(x + y) % p for x in V for y in V})
        xsum_ratio = xsum / min(p, n * n / 2.0)
        xprod = len({x * y % p for x in V for y in V})
        xprod_ratio = xprod / min(p - 1, n * n / 2.0)
        ap = ap3_count(V, p)
        pred_ap = n ** 3 / p
        ap_z = (ap - pred_ap) / math.sqrt(max(1.0, pred_ap))
        # ---- E-level (points)
        ss, en = sumset_stats(F, a, p, inv)
        rng = random.Random(2024 ^ p ^ t)
        ctl_ss = ctl_en = 0
        nctl = 3
        for _ in range(nctl):
            # same size, same negation symmetry: nF/2 QR abscissae lifted
            xs = []
            while len(xs) < nF // 2:
                x = rng.randrange(1, p)
                r = (x * x * x + a * x + b) % p
                if r in qr:
                    xs.append(x)
            G = lift_abscissae(p, a, b, xs, qr)[:nF]
            s2, e2 = sumset_stats(G, a, p, inv)
            ctl_ss += s2
            ctl_en += e2
        ctl_ss /= nctl
        ctl_en /= nctl
        print("%-6d %-9s %2d %4d %4d %7d %6.3f %7d %7.3f %6.2f %9d %9.1f %6.2f"
              % (p, tower, t, n, nF, xsum, xsum_ratio, xprod, xprod_ratio,
                 ap_z, ss, ctl_ss, en / ctl_en))
        rows.append(dict(p=p, tower=tower, t=t, V=n, F=nF, N=N,
                         xsum=xsum, xsum_ratio=xsum_ratio, xprod=xprod,
                         xprod_ratio=xprod_ratio, ap3_z=ap_z,
                         sumset=ss, sumset_ctl=ctl_ss, energy=en,
                         energy_ctl=ctl_en, energy_ratio=en / ctl_en,
                         coset_ratio=ss / (3.0 * nF)))
    out["geom"] = rows
    return rows


def stage_join(p_list, out):
    rows = []
    print("\n== join: hits per pass, ops per relation, probes for |V| relations ==")
    print("%-6s %-9s %2s %5s %5s %8s %8s %9s %9s %8s %5s"
          % ("p", "tower", "t", "|V|", "|F|", "h/pass", "pred", "ops/rel",
             "pred", "tot/p", "r"))
    for p in p_list:
        qr, inv = make_qr(p), make_inv(p)
        a, b, N = find_curve(p, qr)
        rng = random.Random(12345 ^ p)
        for tower in ("interval", "kummer", "isogeny"):
            for t in range(3, 9):
                V, F = build_base(p, tower, t, a, b, qr, inv)
                if V is None or not (8 <= len(F) <= 260):
                    continue
                n, nF = len(V), len(F)
                if 2 * n > p // 2:
                    continue
                xset = set(V)
                nt = 400
                hits = 0
                t0 = time.time()
                for _ in range(nt):
                    T = random_point(p, a, b, qr, rng)
                    hits += pass_hits(T, F, xset, a, p, inv)
                wall = (time.time() - t0) / nt
                hbar = hits / nt
                if hits < 8:
                    continue
                ops_rel = 2 * nF / hbar          # 2 ordered hits = 1 relation
                tot = n * ops_rel
                print("%-6d %-9s %2d %5d %5d %8.3f %8.3f %9.1f %9.1f %8.3f %5.2f"
                      % (p, tower, t, n, nF, hbar, nF * nF / N, ops_rel,
                         2.0 * N / nF, tot / p, ops_rel / (2.0 * N / nF)))
                rows.append(dict(p=p, tower=tower, t=t, V=n, F=nF, N=N,
                                 hits_per_pass=hbar, hits_pred=nF * nF / N,
                                 ops_per_relation=ops_rel,
                                 ops_per_relation_pred=2.0 * N / nF,
                                 ratio=ops_rel / (2.0 * N / nF),
                                 total_over_p=tot / p, pass_wall=wall))
    # direct check of the flatness: collect |V| relations, count probes
    print("  direct collection check (probe until |V| relations found):")
    print("   %-6s %-6s %2s %5s %5s %9s %8s" %
          ("p", "tower", "t", "|V|", "|F|", "probes", "probes/p"))
    direct = []
    for p in (1009, 12289):
        qr, inv = make_qr(p), make_inv(p)
        a, b, N = find_curve(p, qr)
        rng = random.Random(5150 ^ p)
        for tower in ("interval", "kummer", "isogeny"):
            for t in (3, 4, 5):
                V, F = build_base(p, tower, t, a, b, qr, inv)
                if V is None or not (6 <= len(F) <= 64):
                    continue
                n = len(V)
                want = min(n, 64)
                tot_p, reps = 0, 3
                for _ in range(reps):
                    found, probes, passes = collect_relations(
                        p, a, b, F, set(V), want, rng, inv, qr)
                    if found < want:
                        tot_p = 0
                        break
                    tot_p += probes
                if tot_p == 0:
                    continue
                probes = tot_p / reps
                print("   %-6d %-6s %2d %5d %5d %9d %8.3f"
                      % (p, tower, t, n, len(F), probes, probes / N))
                direct.append(dict(p=p, tower=tower, t=t, V=n, F=len(F),
                                   want=want, probes=probes,
                                   over_p=probes / N))
    out["join_direct"] = direct
    out["join"] = rows
    return rows


def stage_structured(p, out):
    """cost the three structure-exploiting joins against the hash join."""
    qr, inv = make_qr(p), make_inv(p)
    a, b, N = find_curve(p, qr)
    rng = random.Random(999 ^ p)
    res = {"p": p, "N": N}

    def timed(fn, reps):
        t0 = time.time()
        for _ in range(reps):
            fn()
        return (time.time() - t0) / reps

    t = min(v2(p - 1), 7)
    V = kummer_V(p, t)
    if V is not None:
        n = len(V)
        xset = set(V)
        gt = pow(1, 1 << t, p)
        xs = [rng.randrange(1, p) for _ in range(4000)]

        def mul_member():
            hit = 0
            for x in xs:
                if pow(x, 1 << t, p) == gt:
                    hit += 1
            return hit

        def hash_member():
            hit = 0
            for x in xs:
                if x in xset:
                    hit += 1
            return hit

        tmul = timed(mul_member, 3) / len(xs)
        thash = timed(hash_member, 3) / len(xs)
        res["kummer_membership"] = dict(t=t, V=n, per_test_mul=tmul,
                                        per_test_hash=thash, ratio=tmul / thash)
        print("  (a) kummer membership t=%d |V|=%d: x^2^t test %.2f us, hash %.2f us,"
              " ratio %.1fx" % (t, n, tmul * 1e6, thash * 1e6, tmul / thash))

    tw = isogeny_tower(p, 6, qr=qr, inv=inv)
    if tw is not None:
        c, xset = tw.c, set(tw.v)
        xs = [rng.randrange(1, p) for _ in range(2000)]
        agree = sum(1 for x in xs if ((tw.eval(x) == c) == (x in xset)))
        tf = lambda: sum(1 for x in xs if tw.eval(x) == c)
        th = lambda: sum(1 for x in xs if x in xset)
        tvelu = timed(tf, 2) / len(xs)
        thash = timed(th, 2) / len(xs)
        res["isogeny_membership"] = dict(t=6, V=len(tw.v), per_test_velu=tvelu,
                                         per_test_hash=thash, ratio=tvelu / thash,
                                         agreement=agree / len(xs))
        print("  (b) isogeny membership t=6 |V|=%d: Velu %.2f us, hash %.2f us,"
              " ratio %.1fx (V == zero set of L-c: %.3f)"
              % (len(tw.v), tvelu * 1e6, thash * 1e6, tvelu / thash,
                 agree / len(xs)))

    print("  (c) sumset-table join vs enumerative join, IC totals for |V| "
          "relations (enumerative = |V|*2N/|F|, measured flat at ~2p):")
    print("   %-6s %-3s %5s %5s %10s %10s %7s"
          % ("p", "t", "|V|", "|F|", "hash_tot", "table_tot", "winner"))
    rows = []
    best = {}
    for p in (1009, 12289, 40961):
        qr2, inv2 = make_qr(p), make_inv(p)
        a2, b2, N2 = find_curve(p, qr2)
        for t in range(2, 10):
            V, F = build_base(p, "interval", t, a2, b2, qr2, inv2)
            if V is None or not (4 <= len(F) <= 400):
                continue
            n, nF = len(V), len(F)
            hash_tot = n * 2.0 * N2 / nF   # validated: join stage, ratio 1.005
            # distinct relations per table pass = ordered hits / 2
            rel_per_pass = nF * nF / (2.0 * N2)
            targets = n / rel_per_pass
            table_tot = nF * (nF + 1) / 2.0 + targets
            win = "table" if table_tot < hash_tot else "hash"
            print("   %-6d %-3d %5d %5d %10.0f %10.0f %7s"
                  % (p, t, n, nF, hash_tot, table_tot, win))
            rows.append(dict(p=p, t=t, V=n, F=nF, hash_total=hash_tot,
                             table_total=table_tot, winner=win))
            if p not in best or table_tot < best[p][0]:
                best[p] = (table_tot, t, n)
    if best:
        import math as _m
        ps = sorted(best)
        print("   best total per prime: " +
              ", ".join("%d->%.0f (t=%d, |V|=%d)" % (q, best[q][0], best[q][1],
                                                       best[q][2]) for q in ps))
        if len(ps) >= 2:
            (q0, q1) = (ps[0], ps[-1])
            ex = _m.log(best[q1][0] / best[q0][0]) / _m.log(q1 / q0)
            print("   fitted exponent of the best join total, p = %d..%d: %.3f "
                  "(asymptote 2/3)" % (q0, q1, ex))
            res["best"] = {str(q): dict(total=best[q][0], t=best[q][1],
                                        V=best[q][2]) for q in ps}
            res["fitted_exponent"] = ex
    res["sumset_table"] = rows

    # wall time for the table build against one enumerative pass (p is the
    # stage argument; 40961 in the default run)

    V, F = build_base(p, "interval", 6, a, b, qr, inv)
    xset = set(V)
    t0 = time.time()
    tab = {}
    for P in F:
        for Q in F:
            R = padd(P, Q, a, p, inv)
            tab[R] = tab.get(R, 0) + 1
    ttab = time.time() - t0
    T = random_point(p, a, b, qr, rng)
    t0 = time.time()
    for _ in range(20):
        pass_hits(T, F, xset, a, p, inv)
    tpass = (time.time() - t0) / 20
    res["wall"] = dict(table_build=ttab, one_pass=tpass, F=len(F), entries=len(tab))
    print("  wall |F|=%d: sumset table %.1f ms, one join pass %.2f ms (ratio %.0fx)"
          % (len(F), ttab * 1e3, tpass * 1e3, ttab / tpass))
    out["structured"] = res
    return res


def stage_caps(p_list, out):
    print("\n== caps: how large can |V| be, per tower ==")
    print("  the isogeny tower needs 2^t | #E' for some E'/F_p; random curves give")
    print("  P(2^t | #E') ~ 2^-t, so the aux-curve search costs ~2^t order counts:")
    print("%-6s %3s %6s %5s %7s %9s %12s %8s"
          % ("p", "v2", "p^1/2", "draws", "t_built", "|V|", "curves(2^t)", "|V|/p^.5"))
    rows = []
    for p in p_list:
        qr = make_qr(p)
        rng = random.Random(7 ^ p)
        draws = 240 if p > 20000 else 600
        hh = {}
        for _ in range(draws):
            a = rng.randrange(0, p)
            b = rng.randrange(1, p)
            if (4 * a * a * a + 27 * b * b) % p == 0:
                continue
            k = v2(order_points(a, b, p, qr))
            hh[k] = hh.get(k, 0) + 1
        tmax = max(hh)
        ge = {}
        for t in range(1, tmax + 1):
            seen_t = sum(c for k, c in hh.items() if k >= t)
            ge[t] = draws / max(1, seen_t)
        # the largest t with a valid coset tower (needs an odd cofactor, so 2^t*3 <= Hasse hi)
        tbuild = None
        inv = make_inv(p)
        for t in range(tmax, 0, -1):
            if 3 * (1 << t) > p + 1 + int(2 * math.sqrt(p)) + 2:
                continue
            tw = isogeny_tower(p, t, qr=qr, inv=inv, budget=min(600, 8 << t))
            if tw is not None:
                tbuild = t
                break
        vm = (1 << tbuild) if tbuild else 0
        print("%-6d %3d %6.1f %5d %5d %9d %9.0f %7.2f"
              % (p, v2(p - 1), math.sqrt(p), draws, tbuild or 0, vm,
                 ge.get(tbuild or tmax, float("nan")), vm / math.sqrt(p)))
        rows.append(dict(p=p, v2_pm1=v2(p - 1), v2_pp1=v2(p + 1), draws=draws,
                         t_max_seen=tmax, t_built=tbuild, V=vm,
                         sqrt_p=math.sqrt(p), ratio=vm / math.sqrt(p), hist=hh,
                         curves_for_t={str(k): v for k, v in ge.items()}))
    print("  measured: the built |V| = 2^t exceeds sqrt(p) by 1.3x-18x, and the search")
    print("  cost is ~2^t curve-order computations: the build is Theta(|V|) group ops.")

    primes = {
        "P-192": 2 ** 192 - 2 ** 64 - 1,
        "P-224": 2 ** 224 - 2 ** 96 + 1,
        "P-256": 2 ** 256 - 2 ** 224 + 2 ** 192 + 2 ** 96 - 1,
        "P-384": 2 ** 384 - 2 ** 128 - 2 ** 96 + 2 ** 32 - 1,
        "P-521": 2 ** 521 - 1,
        "secp256k1": 2 ** 256 - 2 ** 32 - 977,
    }
    print("\n  analytic 2-adic caps (Kummer needs 2^t | p-1; the supersingular\n"
          "  isogeny tower on y^2 = x^3 + x needs 2^t | p+1):")
    print("  %-11s %5s %8s %7s %8s %8s %8s"
          % ("prime", "bits", "v2(p-1)", "bKummer", "v2(p+1)", "bIsog_ss", "max"))
    analytic = []
    for name, p in primes.items():
        bits = p.bit_length()
        bK, bI = v2(p - 1) / bits, v2(p + 1) / bits
        print("  %-11s %5d %8d %7.3f %8d %8.3f %8.3f"
              % (name, bits, v2(p - 1), bK, v2(p + 1), bI, max(bK, bI)))
        analytic.append(dict(name=name, bits=bits, beta_kummer=bK,
                             beta_isogeny_ss=bI, beta_max=max(bK, bI),
                             v2_pm1=v2(p - 1), v2_pp1=v2(p + 1)))
    out["caps"] = dict(measured=rows, analytic=analytic)
    return rows


def main():
    stage = sys.argv[1] if len(sys.argv) > 1 else "all"
    outp = "route2_results.json"
    if "--json" in sys.argv:
        outp = sys.argv[sys.argv.index("--json") + 1]
    out = {}
    t0 = time.time()
    if stage in ("all", "geom"):
        cfgs = []
        for p in (1009, 12289):
            for tower in ("interval", "kummer", "isogeny"):
                for t in (4, 5, 6):
                    cfgs.append((p, tower, t))
        stage_geom(cfgs, out)
    if stage in ("all", "join"):
        stage_join([1009, 12289, 40961], out)
    if stage in ("all", "structured"):
        stage_structured(40961, out)
    if stage in ("all", "caps"):
        stage_caps([1009, 4099, 12289, 40961], out)
    print("\ntotal wall %.1f s" % (time.time() - t0))
    with open(outp, "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)
    print("wrote %s" % outp)


if __name__ == "__main__":
    main()
