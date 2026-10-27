#!/usr/bin/env python3
"""Independent check of recovered discrete logarithms on Koblitz fixtures.

Written for the 2026-10-22 strong-rho batch-inversion validation. Standard
library only; no repository code is imported. For every JSONL record given
on the command line it:

  * builds GF(2^n) as GF(2)[x] / (x^n + sum_{t in field_modulus_low_terms} x^t)
    and checks that modulus is irreducible (Rabin's test);
  * takes the Koblitz curve E_a: y^2 + xy = x^3 + a*x^2 + 1, with field
    elements read as polynomial-basis bit vectors (bit i = coefficient of x^i);
  * checks G = `generator` and Q = `published_q` lie on E_a;
  * checks `subgroup_order` divides #E_a(GF(2^n)) (from the Frobenius trace
    recurrence t_{k+1} = mu*t_k - 2*t_{k-1}) and that G has exactly that order
    (r*G = O and (r/p)*G != O for each prime p | r);
  * checks d*G == Q for d = `recovered_fixture_scalar`, 1 <= d < r, and
    d == `published_fixture_scalar`;
  * checks the Frobenius eigenvalue: (x_G^2, y_G^2) == lambda*G.

Usage: python3 -I verify_scalars.py FILE.jsonl [FILE.jsonl ...]
Prints a JSON report; exits 0 only if every check on every record passes.
"""
import json
import sys

# ---------------------------------------------------------------- GF(2)[x]


def pmul(a, b):
    """Carry-less product of two GF(2)[x] polynomials."""
    r = 0
    while b:
        if b & 1:
            r ^= a
        a <<= 1
        b >>= 1
    return r


def pmod(a, f):
    df = f.bit_length() - 1
    while a and a.bit_length() - 1 >= df:
        a ^= f << (a.bit_length() - 1 - df)
    return a


def pgcd(a, b):
    while b:
        a, b = b, pmod(a, b)
    return a


class Field:
    def __init__(self, n, low_terms):
        self.n = n
        f = 1 << n
        for t in low_terms:
            if not 0 <= t < n:
                raise ValueError(f"low term {t} outside [0, {n})")
            f ^= 1 << t
        self.f = f

    def mul(self, a, b):
        return pmod(pmul(a, b), self.f)

    def sqr(self, a):
        return self.mul(a, a)

    def inv(self, a):
        if a == 0:
            raise ZeroDivisionError("inverse of 0 in GF(2^n)")
        # extended Euclid in GF(2)[x]
        r0, r1, s0, s1 = self.f, a, 0, 1
        while r1:
            q = 0
            r = r0
            d1 = r1.bit_length()
            while r and r.bit_length() >= d1:
                sh = r.bit_length() - d1
                q ^= 1 << sh
                r ^= r1 << sh
            r0, r1 = r1, r
            s0, s1 = s1, s0 ^ pmul(q, s1)
        if r0 != 1:
            raise ValueError("element not invertible: modulus is reducible")
        return pmod(s0, self.f)

    def frob_power(self, a, k):
        for _ in range(k):
            a = self.sqr(a)
        return a

    def irreducible(self):
        """Rabin: f of degree n is irreducible iff x^(2^n) = x mod f and
        gcd(x^(2^(n/p)) - x, f) = 1 for every prime p | n."""
        n = self.n
        x = 2
        if self.frob_power(x, n) != x:
            return False
        for p in prime_factors(n):
            h = self.frob_power(x, n // p) ^ x
            if pgcd(self.f, h) != 1:
                return False
        return True


# ------------------------------------------------------------ integers


def is_probable_prime(m):
    if m < 2:
        return False
    small = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in small:
        if m % p == 0:
            return m == p
    d, s = m - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    # deterministic for m < 3.3e24 with these bases
    for a in small:
        x = pow(a, d, m)
        if x in (1, m - 1):
            continue
        for _ in range(s - 1):
            x = x * x % m
            if x == m - 1:
                break
        else:
            return False
    return True


def pollard_rho(m):
    import math
    if m % 2 == 0:
        return 2
    c = 1
    while True:
        x = y = 2
        d = 1
        while d == 1:
            x = (x * x + c) % m
            y = (y * y + c) % m
            y = (y * y + c) % m
            d = math.gcd(abs(x - y), m)
        if d != m:
            return d
        c += 1


def prime_factors(m):
    out = set()
    stack = [m]
    while stack:
        v = stack.pop()
        if v == 1:
            continue
        if is_probable_prime(v):
            out.add(v)
            continue
        for p in (2, 3, 5, 7, 11, 13):
            if v % p == 0:
                stack += [p, v // p]
                break
        else:
            d = pollard_rho(v)
            stack += [d, v // d]
    return sorted(out)


# ------------------------------------------------------------ the curve

O = None  # point at infinity


class Koblitz:
    """y^2 + xy = x^3 + a x^2 + 1 over GF(2^n)."""

    def __init__(self, field, a):
        if a not in (0, 1):
            raise ValueError("Koblitz a must be 0 or 1")
        self.F = field
        self.a = a

    def on_curve(self, P):
        if P is O:
            return True
        F = self.F
        x, y = P
        lhs = F.sqr(y) ^ F.mul(x, y)
        x2 = F.sqr(x)
        rhs = F.mul(x2, x) ^ (x2 if self.a else 0) ^ 1
        return lhs == rhs

    def neg(self, P):
        return O if P is O else (P[0], P[0] ^ P[1])

    def add(self, P, Q):
        F = self.F
        if P is O:
            return Q
        if Q is O:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y2 == x1 ^ y1:
                return O  # Q = -P (covers 2-torsion x = 0 too)
            # P == Q: doubling, x1 != 0 here
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sqr(lam) ^ lam ^ self.a_elem()
            y3 = F.sqr(x1) ^ F.mul(lam ^ 1, x3)
            return (x3, y3)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sqr(lam) ^ lam ^ x1 ^ x2 ^ self.a_elem()
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)

    def a_elem(self):
        return 1 if self.a else 0

    def mul(self, k, P):
        if k < 0:
            return self.mul(-k, self.neg(P))
        R = O
        A = P
        while k:
            if k & 1:
                R = self.add(R, A)
            A = self.add(A, A)
            k >>= 1
        return R

    def order(self):
        """#E_a(GF(2^n)) from the trace recurrence over GF(2)."""
        n = self.F.n
        mu = -1 if self.a == 0 else 1  # #E_a(GF(2)) = 3 - mu: 4 for a=0, 2 for a=1
        t_prev, t = 2, mu
        for _ in range(n - 1):
            t_prev, t = t, mu * t - 2 * t_prev
        return 2 ** n + 1 - t


# ------------------------------------------------------------ driver


def check_record(rec, cache):
    n = rec["n"]
    a = rec["a"]
    low = tuple(rec["field_modulus_low_terms"])
    G = tuple(rec["generator"])
    Q = tuple(rec["published_q"])
    r = rec["subgroup_order"]
    lam = rec["lambda"]
    d = rec["recovered_fixture_scalar"]
    key = (n, a, low, G, r, lam)
    if key not in cache:
        F = Field(n, low)
        E = Koblitz(F, a)
        group = E.order()
        pf = prime_factors(r)
        g_on = E.on_curve(G)
        exact = g_on and G is not O and E.mul(r, G) is O and all(E.mul(r // p, G) is not O for p in pf)
        frob = (F.sqr(G[0]), F.sqr(G[1]))
        cache[key] = {
            "n": n,
            "a": a,
            "modulus_low_terms": list(low),
            "modulus_irreducible": F.irreducible(),
            "curve_order": group,
            "subgroup_order": r,
            "subgroup_order_prime": is_probable_prime(r),
            "subgroup_order_factors": pf,
            "subgroup_order_divides_curve_order": group % r == 0,
            "cofactor": group // r if group % r == 0 else None,
            "generator_on_curve": g_on,
            "generator_has_order_subgroup_order": exact,
            "frobenius_eigenvalue_lambda_ok": E.mul(lam, G) == frob,
            "_E": E,
        }
    c = cache[key]
    E = c["_E"]
    checks = {
        "key_fields_consistent": list(G) == rec.get("generator_point_key") and list(Q) == rec.get("published_q_point_key"),
        "q_on_curve": E.on_curve(Q),
        "scalar_in_range": 1 <= d < r,
        "recovered_equals_published": d == rec["published_fixture_scalar"],
        "dG_equals_Q": E.mul(d, G) == Q,
    }
    return c, checks


def main(paths):
    cache = {}
    files = []
    ok_all = True
    for path in paths:
        recs = []
        with open(path) as fh:
            for line in fh:
                if line.strip():
                    recs.append(json.loads(line))
        entries = []
        for rec in recs:
            try:
                c, checks = check_record(rec, cache)
            except (ValueError, ZeroDivisionError, KeyError, TypeError) as exc:
                ok_all = False
                entries.append({"fixture_index": rec.get("fixture_index"), "n": rec.get("n"),
                                "d": rec.get("recovered_fixture_scalar"), "ok": False,
                                "error": f"{type(exc).__name__}: {exc}"})
                continue
            curve_ok = all(c[k] for k in (
                "modulus_irreducible", "subgroup_order_divides_curve_order",
                "generator_on_curve", "generator_has_order_subgroup_order",
                "frobenius_eigenvalue_lambda_ok"))
            ok = curve_ok and all(checks.values())
            ok_all &= ok
            entries.append({
                "fixture_index": rec.get("fixture_index"),
                "n": rec["n"],
                "d": rec["recovered_fixture_scalar"],
                "ok": ok,
                **checks,
            })
        files.append({"path": path, "records": len(recs),
                      "all_ok": bool(recs) and all(e["ok"] for e in entries),
                      "scalars": entries})
        ok_all &= bool(recs)
    curves = [{k: v for k, v in c.items() if k != "_E"} for c in cache.values()]
    print(json.dumps({"all_ok": ok_all, "curves": curves, "files": files}, indent=1))
    return 0 if ok_all else 1


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    sys.exit(main(sys.argv[1:]))
