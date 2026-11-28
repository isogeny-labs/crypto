#!/usr/bin/env python3
"""Lead #5 census: Frobenius-stable Lagrangians of J[l] for prime-order genus-3
hyperelliptic Jacobians, decided from the Frobenius characteristic polynomial.

Protocol: PROTOCOL.md (frozen before data). Pure Python, standard library, one thread.

Usage:
  python3 -I census.py gates   OUTDIR       # instrument gates G1, G2 (separate seeds)
  python3 -I census.py census  OUTDIR       # arm A census (writes census_out.jsonl)
  python3 -I census.py armb    L06RAWDIR OUTDIR   # arm B: L06 recorded chi
  python3 -I census.py summary OUTDIR       # census_summary.json from the jsonl files
"""
import itertools
import json
import math
import os
import random
import sys
import time

PRIMES_A = [29, 31, 37, 41, 43, 47]
TARGET_PRIME = 100
LS = [2, 3, 5, 7]
KS = [4, 6, 8, 13]

# ---------------------------------------------------------------- F_p polynomials
# Polynomials are lists of coefficients, low degree first, entries in [0, p).


def ptrim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def pmod(a, m, p):
    a = a[:]
    inv = pow(m[-1], p - 2, p)
    dm = len(m) - 1
    while len(ptrim(a)) - 1 >= dm:
        c = a[-1] * inv % p
        sh = len(a) - 1 - dm
        for i, mi in enumerate(m):
            a[sh + i] = (a[sh + i] - c * mi) % p
    return a


def pmul(a, b, p):
    if not a or not b:
        return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                r[i + j] = (r[i + j] + x * y) % p
    return ptrim(r)


def pmulmod(a, b, m, p):
    return pmod(pmul(a, b, p), m, p)


def ppowmod(a, e, m, p):
    r = [1]
    a = pmod(a, m, p)
    while e:
        if e & 1:
            r = pmulmod(r, a, m, p)
        a = pmulmod(a, a, m, p)
        e >>= 1
    return r


def psub(a, b, p):
    n = max(len(a), len(b))
    r = [((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)]
    return ptrim(r)


def pgcd(a, b, p):
    a, b = ptrim(a[:]), ptrim(b[:])
    while b:
        a, b = b, ptrim(pmod(a, b, p))
    if a:
        inv = pow(a[-1], p - 2, p)
        a = [x * inv % p for x in a]
    return a


def is_irreducible(f, p):
    """Rabin test for f monic over F_p."""
    n = len(f) - 1
    x = [0, 1]
    xp = x
    for i in range(1, n // 2 + 1):
        xp = ppowmod(xp, p, f, p)
        if len(pgcd(f, psub(xp, x, p), p)) > 1:
            return False
    return True


# ---------------------------------------------------------------- F_{p^r} log tables


class Field:
    """F_{p^r} with elements encoded as integers sum c_i p^i, and log tables."""

    def __init__(self, p, r):
        self.p, self.r = p, r
        self.q = p ** r
        n = self.q - 1
        for coeffs in itertools.product(range(p), repeat=r):
            m = list(coeffs) + [1]
            if m[0] == 0 or not is_irreducible(m, p):
                continue
            exp = self._try_primitive(m, n)
            if exp is not None:
                break
        self.mod = m
        self.exp = exp
        log = [None] * self.q
        for i, v in enumerate(exp):
            log[v] = i
        self.log = log
        self.n = n

    def _try_primitive(self, m, n):
        p, r = self.p, self.r
        # multiply by t: vector shift then reduce by monic m
        exp = [0] * n
        v = [1] + [0] * (r - 1)
        seen1 = False
        for i in range(n):
            code = 0
            for c in reversed(v):
                code = code * p + c
            if i > 0 and code == 1:
                return None
            exp[i] = code
            top = v[-1]
            v = [0] + v[:-1]
            if top:
                for j in range(r):
                    v[j] = (v[j] - top * m[j]) % p
        code = 0
        for c in reversed(v):
            code = code * p + c
        return exp if code == 1 else None

    def orbit_reps(self):
        """Representatives (as logs) of Frobenius orbits on F_q \\ (proper subfields),
        for r prime, with orbit size r. Also returns logs of F_p^* elements."""
        n, p, r = self.n, self.p, self.r
        sub = (self.q - 1) // (p - 1)  # F_p^* = multiples of sub
        seen = bytearray(n)
        reps = []
        for i in range(n):
            if seen[i]:
                continue
            orb = []
            j = i
            while not seen[j]:
                seen[j] = 1
                orb.append(j)
                j = j * p % n
            if len(orb) == r:
                reps.append(i)
            else:
                assert len(orb) == 1 and i % sub == 0
        return reps


def char_sum(field, f, reps):
    """sum over x in F_q of chi_q(f(x)), f monic over F_p (low first)."""
    p, n, exp, log = field.p, field.n, field.exp, field.log
    coeffs = f[:-1][::-1]  # f_{d-1}, ..., f_0 for Horner
    # x in F_p: chi_q(a) = chi_p(a)^r = chi_p(a) for r odd, = 1 (nonzero) for r even
    total = 0
    for x in range(p):
        v = 0
        for c in f[::-1]:
            v = (v * x + c) % p
        if v:
            if field.r % 2 == 1:
                total += 1 if pow(v, (p - 1) // 2, p) == 1 else -1
            else:
                total += 1
    s = 0
    for lx in reps:
        acc = exp[lx]  # monic: acc = x after first step (1*x + f_{d-1}) below
        acc = acc - acc % p + (acc % p + coeffs[0]) % p
        for c in coeffs[1:]:
            if acc:
                acc = exp[(log[acc] + lx) % n]
            acc = acc - acc % p + (acc % p + c) % p
        if acc:
            s += 1 if log[acc] % 2 == 0 else -1
    return total + field.r * s


def count_points(fields, reps, f):
    p = fields[1].p
    N = []
    for r in (1, 2, 3):
        S = char_sum(fields[r], f, reps[r])
        N.append(p ** r + 1 + S)  # one point at infinity (odd degree)
    return N


def charpoly_from_counts(p, N):
    S1 = p + 1 - N[0]
    S2 = p ** 2 + 1 - N[1]
    S3 = p ** 3 + 1 - N[2]
    e1 = S1
    e2n = S1 * S1 - S2
    e3n = S1 ** 3 - 3 * S1 * S2 + 2 * S3
    integral = (e2n % 2 == 0) and (e3n % 6 == 0)
    e2, e3 = e2n // 2, e3n // 6
    # P(T) low-first
    P = [p ** 3, -p * p * e1, p * e2, -e3, e2, -e1, 1]
    return P, integral, (S1, S2, S3)


def weil_ok(p, P):
    # |a1| <= 6 sqrt p, |a2| <= 15 p, |a3| <= 20 p^1.5 (coarse binomial bounds)
    e1, e2, e3 = -P[5], P[4], -P[3]
    sp = math.sqrt(p)
    return abs(e1) <= 6 * sp + 1e-9 and abs(e2) <= 15 * p + 1e-9 and abs(e3) <= 20 * p * sp + 1e-9


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


# ---------------------------------------------------------------- integer #J(F_{p^k})


def det_int(M):
    """Bareiss fraction-free determinant."""
    M = [row[:] for row in M]
    n = len(M)
    sign, prev = 1, 1
    for k in range(n - 1):
        if M[k][k] == 0:
            for i in range(k + 1, n):
                if M[i][k] != 0:
                    M[k], M[i] = M[i], M[k]
                    sign = -sign
                    break
            else:
                return 0
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                M[i][j] = (M[i][j] * M[k][k] - M[i][k] * M[k][j]) // prev
        prev = M[k][k]
    return sign * M[n - 1][n - 1]


def matmul(A, B):
    n = len(A)
    return [[sum(A[i][t] * B[t][j] for t in range(n)) for j in range(n)] for i in range(n)]


def order_JFk(P, k, cache):
    """#J(F_{p^k}) = prod (1 - alpha^k) = det(C^k - I) for even degree 6."""
    if k in cache:
        return cache[k]
    n = 6
    C = [[0] * n for _ in range(n)]
    for i in range(1, n):
        C[i][i - 1] = 1
    for i in range(n):
        C[i][n - 1] = -P[i]
    R = [[int(i == j) for j in range(n)] for i in range(n)]
    B, e = C, k
    while e:
        if e & 1:
            R = matmul(R, B)
        B = matmul(B, B)
        e >>= 1
    for i in range(n):
        R[i][i] -= 1
    v = det_int(R)
    cache[k] = v
    return v


def vl(n, l):
    if n == 0:
        return 99
    n = abs(n)
    c = 0
    while n % l == 0:
        n //= l
        c += 1
    return c


# ---------------------------------------------------------------- F_l[T] structure


_IRR = {}


def irreducibles(l, dmax=6):
    if l in _IRR:
        return _IRR[l]
    out = []
    for d in range(1, dmax + 1):
        for coeffs in itertools.product(range(l), repeat=d):
            f = list(coeffs) + [1]
            if d == 1 or (f[0] != 0 and is_irreducible(f, l)):
                out.append(tuple(f))
    _IRR[l] = out
    return out


def factor_mod(P, l):
    a = ptrim([c % l for c in P])
    fac = {}
    for g in irreducibles(l):
        if len(g) - 1 > len(a) - 1:
            break
        g = list(g)
        while True:
            q, r = pdivmod(a, g, l)
            if r:
                break
            a = q
            fac[tuple(g)] = fac.get(tuple(g), 0) + 1
        if len(a) == 1:
            break
    assert a == [1], (P, l, a)
    return fac


def pdivmod(a, b, p):
    a = a[:]
    inv = pow(b[-1], p - 2, p)
    db = len(b) - 1
    q = [0] * max(len(a) - db, 1)
    while len(ptrim(a)) - 1 >= db and a:
        c = a[-1] * inv % p
        sh = len(a) - 1 - db
        q[sh] = c
        for i, bi in enumerate(b):
            a[sh + i] = (a[sh + i] - c * bi) % p
    return ptrim(q), ptrim(a)


def dual(f, p, l):
    """f*(T) = T^d f(p/T) / f(0), monic, over F_l."""
    d = len(f) - 1
    g = [0] * (d + 1)
    for i, a in enumerate(f):
        g[d - i] = a * pow(p, i, l) % l
    inv = pow(g[-1], l - 2, l)
    return tuple(x * inv % l for x in g)


_ORD = {}


def root_order(f, l):
    key = (f, l)
    if key in _ORD:
        return _ORD[key]
    d = len(f) - 1
    N = l ** d - 1
    f = list(f)
    best = None
    for k in sorted(divisors(N)):
        if ppowmod([0, 1], k, f, l) == [1]:
            best = k
            break
    _ORD[key] = best
    return best


def divisors(n):
    ds = set()
    i = 1
    while i * i <= n:
        if n % i == 0:
            ds.add(i)
            ds.add(n // i)
        i += 1
    return ds


def lcm(*xs):
    r = 1
    for x in xs:
        r = r * x // math.gcd(r, x)
    return r


def fmt(f):
    return "".join(str(c) for c in f)


def classify(P, p, l, Ks, cache):
    """Status of stable Lagrangians of J[l] from the charpoly P (integers, low first)."""
    fac = factor_mod(P, l)
    pl = p % l
    duals = {f: dual(f, pl, l) for f in fac}
    for f, m in fac.items():
        assert fac.get(duals[f]) == m, ("dual multiplicity mismatch", P, l)
    squarefree = all(m == 1 for m in fac.values())
    selfdual = {f: m for f, m in fac.items() if duals[f] == f}
    pairs = []
    done = set()
    for f, m in fac.items():
        if duals[f] != f and f not in done:
            pairs.append((f, duals[f], m))
            done.add(f)
            done.add(duals[f])
    sd_odd1 = any(m == 1 for m in selfdual.values())
    guaranteed = all(m == 1 for (_, _, m) in pairs) and all(m == 2 for m in selfdual.values())
    # choices for guaranteed structures: each pair -> ord f or ord f*, self-dual m=2 -> ord f
    choice_ks = None
    if guaranteed and not sd_odd1:
        base = [root_order(f, l) for f in selfdual]
        opts = [(root_order(f, l), root_order(g, l)) for (f, g, _) in pairs]
        choice_ks = sorted(lcm(*base, *c) for c in itertools.product(*opts)) if opts else [lcm(*base)]
    # fact 1 enumeration: h | Pbar, deg 3, h h* = Pbar
    items = list(fac.items())
    hs = []
    for mult in itertools.product(*[range(m + 1) for _, m in items]):
        deg = sum((len(f) - 1) * e for (f, _), e in zip(items, mult))
        if deg != 3:
            continue
        h = {f: e for (f, _), e in zip(items, mult) if e}
        hstar = {}
        for f, e in h.items():
            hstar[duals[f]] = hstar.get(duals[f], 0) + e
        tot = dict(h)
        for f, e in hstar.items():
            tot[f] = tot.get(f, 0) + e
        if tot != fac:
            continue
        kss = lcm(*[root_order(f, l) for f in h])
        maxm = max(h.values())
        jmax = 0
        while l ** jmax < maxm:
            jmax += 1
        hs.append({"h": {fmt(f): e for f, e in h.items()}, "k_ss": kss, "jmax": jmax})

    def status(K):
        if sd_odd1:
            return "NO", None
        if choice_ks is not None:
            # structure-independent: the set of possible k is exactly choice_ks (fact 5)
            kmin = choice_ks[0]
            if K is None or kmin <= K:
                return "YES", kmin
            return "NO", None
        possible = []
        for H in hs:
            for j in range(H["jmax"] + 1):
                k = H["k_ss"] * l ** j
                if K is not None and k > K:
                    continue
                if K is not None and vl(order_JFk(P, k, cache), l) < 3:
                    continue
                possible.append(k)
        if not possible:
            return "NO", None
        if choice_ks is not None and K is None:
            return "YES", choice_ks[0]
        return "AMBIG", min(possible)

    out = {
        "Pbar": fmt([c % l for c in P]),
        "factors": {fmt(f): m for f, m in fac.items()},
        "squarefree": squarefree,
        "selfdual": {fmt(f): m for f, m in selfdual.items()},
        "n_pairs": len(pairs),
        "hs": hs,
    }
    ex, kmin = status(None)
    out["exists"] = ex
    out["kmin"] = kmin
    if squarefree:
        out["n_lagr"] = 0 if selfdual else 2 ** len(pairs)
        out["k_list"] = [] if selfdual else choice_ks
    for K in Ks:
        st, km = status(K)
        out["k<=%d" % K] = st
    return out


# ---------------------------------------------------------------- drivers


def setup_fields(p):
    fields = {r: Field(p, r) for r in (1, 2, 3)}
    reps = {r: (fields[r].orbit_reps() if r > 1 else []) for r in (1, 2, 3)}
    return fields, reps


def random_irreducible_septic(rng, p):
    while True:
        f = [rng.randrange(p) for _ in range(7)] + [1]
        if f[0] != 0 and is_irreducible(f, p):
            return f


def analyse_curve(p, f, P, extra):
    cache = {}
    row = dict(extra)
    row.update({"p": p, "f": f, "P": P, "nJ": sum(P), "nJtw": sum(c * (-1) ** i for i, c in enumerate(P))})
    row["l"] = {str(l): classify(P, p, l, KS, cache) for l in LS}
    return row


def run_census(outdir):
    out = open(os.path.join(outdir, "census_out.jsonl"), "w")
    log = {}
    for p in PRIMES_A:
        t0 = time.time()
        fields, reps = setup_fields(p)
        n_prime = n_irred = 0
        i = 0
        gate_fail = 0
        while n_prime < TARGET_PRIME:
            rng = random.Random(1000003 * p + i)
            f = [rng.randrange(p) for _ in range(7)] + [1]
            i += 1
            if f[0] == 0 or not is_irreducible(f, p):
                continue
            n_irred += 1
            N = count_points(fields, reps, f)
            P, integral, S = charpoly_from_counts(p, N)
            if not integral or not weil_ok(p, P):
                gate_fail += 1
                continue
            nJ = sum(P)
            if not is_prime(nJ):
                continue
            n_prime += 1
            row = analyse_curve(p, f, P, {"seed_i": i - 1, "N": N})
            out.write(json.dumps(row, sort_keys=True) + "\n")
        log[p] = {"draws": i, "irreducible": n_irred, "prime": n_prime, "gate_fail_G1": gate_fail,
                  "seconds": round(time.time() - t0, 1)}
        print(p, log[p], flush=True)
    out.close()
    json.dump(log, open(os.path.join(outdir, "census_log.json"), "w"), indent=1, sort_keys=True)


def run_gates(outdir):
    res = {"G1": {}, "G2": {}}
    # G1 on separate seeds at the census primes: integrality and Weil on 30 irreducible f
    for p in PRIMES_A:
        fields, reps = setup_fields(p)
        ok = 0
        for i in range(30):
            rng = random.Random(7 + i)
            f = random_irreducible_septic(rng, p)
            N = count_points(fields, reps, f)
            P, integral, S = charpoly_from_counts(p, N)
            ok += int(integral and weil_ok(p, P))
        res["G1"][p] = "%d/30" % ok
    # G2: N4 direct vs predicted, p = 13, 17, 10 curves each (any squarefree monic septic)
    for p in (13, 17):
        fields, reps = setup_fields(p)
        F4 = Field(p, 4)
        ok = 0
        detail = []
        for i in range(10):
            rng = random.Random(7 + i)
            while True:
                f = [rng.randrange(p) for _ in range(7)] + [1]
                df = ptrim([(j * f[j]) % p for j in range(1, 8)])
                if len(pgcd(f, df, p)) == 1:
                    break
            N = count_points(fields, reps, f)
            P, integral, S = charpoly_from_counts(p, N)
            # direct N4: sum over all x in F_{p^4}
            n4, exp, log = F4.n, F4.exp, F4.log
            tot = 0
            for x in range(F4.q):
                acc = 1
                for c in f[-2::-1]:
                    if acc:
                        acc = exp[(log[acc] + log[x]) % n4] if x else 0
                    acc = acc - acc % p + (acc % p + c) % p
                if acc:
                    tot += 1 if log[acc] % 2 == 0 else -1
            N4 = p ** 4 + 1 + tot
            # predicted: S4 from Newton with e4 = p e2 (low-first P[2]), e1..e3
            e1, e2, e3, e4 = -P[5], P[4], -P[3], P[2]
            S1, S2, S3 = S
            S4 = e1 * S3 - e2 * S2 + e3 * S1 - 4 * e4
            pred = p ** 4 + 1 - S4
            ok += int(pred == N4)
            detail.append({"f": f, "N4_direct": N4, "N4_pred": pred})
        res["G2"][p] = {"ok": "%d/10" % ok, "detail": detail}
    json.dump(res, open(os.path.join(outdir, "gates.json"), "w"), indent=1, sort_keys=True)
    print(json.dumps({k: (v if k == "G1" else {p: d["ok"] for p, d in v.items()}) for k, v in res.items()}))


def run_armb(rawdir, outdir):
    out = open(os.path.join(outdir, "armb_out.jsonl"), "w")
    conv_fail = 0
    n = 0
    for name in sorted(os.listdir(rawdir)):
        if not (name.startswith("census_p") and name.endswith(".jsonl")):
            continue
        for line in open(os.path.join(rawdir, name)):
            r = json.loads(line)
            if r.get("status") != "ok" or not r.get("prime"):
                continue
            p, s1, s2, s3 = r["p"], r["s1"], r["s2"], r["s3"]
            P = [p ** 3, -p * p * s1, p * s2, -s3, s2, -s1, 1]  # L06 SPEC.md line 196 (deviation D1)
            if sum(P) != r["nJ"] or [c % 3 for c in P] != r["crit"]["3"]["chi_mod_l"]:
                conv_fail += 1
                continue
            n += 1
            row = analyse_curve(p, r["f"], P, {"src": name, "i": r["i"]})
            out.write(json.dumps(row, sort_keys=True) + "\n")
    out.close()
    print("arm B rows", n, "convention failures", conv_fail)


def wilson(x, n, z=1.96):
    if n == 0:
        return [0.0, 1.0]
    ph = x / n
    d = 1 + z * z / n
    c = (ph + z * z / (2 * n)) / d
    h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n)) / d
    return [round(max(0, c - h), 4), round(min(1, c + h), 4)]


def summarise(rows):
    S = {"N": len(rows)}
    byp = {}
    for r in rows:
        byp.setdefault(r["p"], []).append(r)
    S["per_p"] = {p: len(v) for p, v in sorted(byp.items())}
    for l in LS:
        L = str(l)
        d = {}
        for key in ["exists"] + ["k<=%d" % K for K in KS]:
            c = {"YES": 0, "NO": 0, "AMBIG": 0}
            for r in rows:
                c[r["l"][L][key]] += 1
            n = len(rows)
            c["R_low"] = round(c["YES"] / n, 4) if n else None
            c["R_high"] = round((c["YES"] + c["AMBIG"]) / n, 4) if n else None
            c["wilson_low_R_low"] = wilson(c["YES"], n)[0]
            c["wilson_up_R_high"] = wilson(c["YES"] + c["AMBIG"], n)[1]
            d[key] = c
        kd = {}
        for r in rows:
            x = r["l"][L]
            if x["exists"] == "YES":
                kd[x["kmin"]] = kd.get(x["kmin"], 0) + 1
        d["kmin_dist_decided_yes"] = dict(sorted(kd.items()))
        d["squarefree"] = sum(r["l"][L]["squarefree"] for r in rows)
        # per p, k<=4
        d["k<=4_per_p"] = {}
        for p, v in sorted(byp.items()):
            c = {"YES": 0, "NO": 0, "AMBIG": 0}
            for r in v:
                c[r["l"][L]["k<=4"]] += 1
            d["k<=4_per_p"][p] = c
        pats = {}
        for r in rows:
            x = r["l"][L]
            if x["k<=4"] != "NO":
                key = "%s:%s" % (x["k<=4"], x["Pbar"])
                pats[key] = pats.get(key, 0) + 1
        d["k<=4_nonNO_patterns"] = pats
        S["l=%d" % l] = d
    # l = 2 control (P3)
    phi7 = {"1101": 1, "1011": 1}
    S["P3_l2_control"] = sum(
        1 for r in rows if r["l"]["2"]["factors"] == phi7 and r["l"]["2"].get("n_lagr") == 2
        and r["l"]["2"].get("k_list") == [7, 7])
    # verdict (arm A only)
    c = S["l=3"]["k<=4"]
    if c["wilson_up_R_high"] < 0.05:
        v = "KILL"
    elif c["wilson_low_R_low"] >= 0.05:
        v = "SURVIVE"
    else:
        v = "INCONCLUSIVE"
    S["verdict_rule_output"] = v
    return S


def run_summary(outdir):
    res = {}
    for name, arm in (("census_out.jsonl", "armA"), ("armb_out.jsonl", "armB")):
        path = os.path.join(outdir, name)
        if os.path.exists(path):
            rows = [json.loads(x) for x in open(path)]
            res[arm] = summarise(rows)
    json.dump(res, open(os.path.join(outdir, "census_summary.json"), "w"), indent=1, sort_keys=True)
    print(json.dumps({a: {"N": s["N"], "l3_k<=4": s["l=3"]["k<=4"], "verdict": s["verdict_rule_output"]}
                      for a, s in res.items()}, indent=1))


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "gates":
        run_gates(sys.argv[2])
    elif cmd == "census":
        run_census(sys.argv[2])
    elif cmd == "armb":
        run_armb(sys.argv[2], sys.argv[3])
    elif cmd == "summary":
        run_summary(sys.argv[2])
