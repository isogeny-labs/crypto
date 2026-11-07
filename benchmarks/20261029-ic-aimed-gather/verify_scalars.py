#!/usr/bin/env python3
"""Independent check of the logarithm recovered by `ic price` on rule v3 k0n53/T01.

Written by the validator of the 2026-10-29 aimed-gather follow-up
(research/ic_aimed_gather_20261023/aa2_20261029/).  Standard library only;
no repository code is imported.  The approach follows
benchmarks/20261022-strong-rho-batchinv/verify_scalars.py; this is a separate
copy, extended to derive the public target from its hash seed.

Inputs and where each value comes from:

  --registry   docs/curves/registry.json: the entry with --slug gives the
               field modulus (params.modulus and the representation's
               modulus_exponents), #E (order) and its trace, the subgroup
               order r, the cofactor and the generator G (polynomial-basis hex).
  --params     the params file: curve degree n and Koblitz a (curve.degree,
               curve.curve_a) and the target's public_hash_seed.
  --target-record
               an `ic price` record that printed the hashed target (rule v3
               T01-R1.price.json): Q, the generator and the modulus low terms
               it used.  Only compared with; Q is derived here from the seed.
  --recovered  any number of `ic price --json` outputs: each must report
               status complete, all_verified true and recovered [d], and its
               curve slug, n, a and r must agree with the registry.

Checks:
  * GF(2^n) = GF(2)[x]/(f), f from the registry; f irreducible (Rabin);
    the two registry encodings of f and the target record's agree.
  * E: y^2 + xy = x^3 + a x^2 + 1;  #E from the Frobenius trace recurrence
    equals the registry order and trace; r is prime (deterministic
    Miller-Rabin for r < 3.3e24), r * cofactor = #E.
  * G on E, G != O, r*G = O (so ord G = r, r prime);
    the Frobenius eigenvalue: one root lambda of l^2 - t1*l + 2 = 0 mod r
    (t1 = -1 for a = 0) has (x_G^2, y_G^2) = lambda*G.
  * Q derived from the seed by the public hash-to-curve the `ic` workflow
    documents (BLAKE3 over a fixed 62-byte message, the low n bits of the
    first 8 digest bytes as x, the lift chosen by digest byte 8, times the
    cofactor; counter incremented until a lift exists); BLAKE3 implemented
    here from its specification and self-tested on the published empty-input
    and "abc" digests; Q on E, Q != O, r*Q = O; Q equals the recorded Q.
  * For every recovered d: 1 <= d < r and d*G == Q.
  * Negative controls: (d+1)*G != Q, and seed+1 hashes to a different
    point on E, so neither test passes vacuously.

Usage:
  python3 -I verify_scalars.py --registry R --slug S --params P \
      --target-record T --recovered F [F ...]
Prints a JSON report; exits 0 only if every check passes.
"""
import argparse
import hashlib
import json
import struct
import sys

# ---------------------------------------------------------------- BLAKE3

_IV = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
       0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19]
_PERM = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8]
_M32 = 0xFFFFFFFF
CHUNK_START, CHUNK_END, ROOT = 1, 2, 8


def _rotr(v, k):
    return ((v >> k) | (v << (32 - k))) & _M32


def _g(s, a, b, c, d, mx, my):
    s[a] = (s[a] + s[b] + mx) & _M32
    s[d] = _rotr(s[d] ^ s[a], 16)
    s[c] = (s[c] + s[d]) & _M32
    s[b] = _rotr(s[b] ^ s[c], 12)
    s[a] = (s[a] + s[b] + my) & _M32
    s[d] = _rotr(s[d] ^ s[a], 8)
    s[c] = (s[c] + s[d]) & _M32
    s[b] = _rotr(s[b] ^ s[c], 7)


def _compress(cv, words, counter, block_len, flags):
    s = list(cv) + _IV[:4] + [counter & _M32, (counter >> 32) & _M32, block_len, flags]
    m = list(words)
    for rnd in range(7):
        _g(s, 0, 4, 8, 12, m[0], m[1])
        _g(s, 1, 5, 9, 13, m[2], m[3])
        _g(s, 2, 6, 10, 14, m[4], m[5])
        _g(s, 3, 7, 11, 15, m[6], m[7])
        _g(s, 0, 5, 10, 15, m[8], m[9])
        _g(s, 1, 6, 11, 12, m[10], m[11])
        _g(s, 2, 7, 8, 13, m[12], m[13])
        _g(s, 3, 4, 9, 14, m[14], m[15])
        if rnd < 6:
            m = [m[i] for i in _PERM]
    for i in range(8):
        s[i] ^= s[i + 8]
        s[i + 8] ^= cv[i]
    return s


def blake3_single_chunk(data):
    """BLAKE3 hash (32 bytes) of an input of at most one chunk (1024 bytes)."""
    if len(data) > 1024:
        raise ValueError("this BLAKE3 handles a single chunk only")
    blocks = [data[i:i + 64] for i in range(0, len(data), 64)] or [b""]
    cv = list(_IV)
    for i, blk in enumerate(blocks):
        flags = (CHUNK_START if i == 0 else 0)
        last = i == len(blocks) - 1
        if last:
            flags |= CHUNK_END | ROOT
        words = struct.unpack("<16I", blk.ljust(64, b"\0"))
        out = _compress(cv, words, 0, len(blk), flags)
        cv = out[:8]
    return struct.pack("<8I", *cv)


BLAKE3_SELFTEST = {
    b"": "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262",
    b"abc": "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
}

# ---------------------------------------------------------------- GF(2)[x]


def pmul(a, b):
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
    def __init__(self, f):
        self.f = f
        self.n = f.bit_length() - 1

    def mul(self, a, b):
        return pmod(pmul(a, b), self.f)

    def sqr(self, a):
        return self.mul(a, a)

    def inv(self, a):
        if a == 0:
            raise ZeroDivisionError("inverse of 0")
        r0, r1, s0, s1 = self.f, a, 0, 1
        while r1:
            q, r = 0, r0
            d1 = r1.bit_length()
            while r and r.bit_length() >= d1:
                sh = r.bit_length() - d1
                q ^= 1 << sh
                r ^= r1 << sh
            r0, r1 = r1, r
            s0, s1 = s1, s0 ^ pmul(q, s1)
        if r0 != 1:
            raise ValueError("not invertible: modulus reducible")
        return pmod(s0, self.f)

    def frob(self, a, k):
        for _ in range(k):
            a = self.sqr(a)
        return a

    def trace(self, a):
        t, v = 0, a
        for _ in range(self.n):
            t ^= v
            v = self.sqr(v)
        return t  # 0 or 1

    def half_trace(self, c):
        """For odd n: H(c) = sum_{i=0}^{(n-1)/2} c^(4^i) solves u^2 + u = c when Tr(c) = 0."""
        h, v = 0, c
        for _ in range((self.n - 1) // 2 + 1):
            h ^= v
            v = self.sqr(self.sqr(v))
        return h

    def irreducible(self):
        n, x = self.n, 2
        if self.frob(x, n) != x:
            return False
        for p in prime_factors(n):
            if pgcd(self.f, self.frob(x, n // p) ^ x) != 1:
                return False
        return True


# ---------------------------------------------------------------- integers


def is_prime(m):
    if m < 2:
        return False
    small = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41)
    for p in small:
        if m % p == 0:
            return m == p
    d, s = m - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:  # deterministic below 3.3e24
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


def prime_factors(m):
    out, p = set(), 2
    while p * p <= m:
        while m % p == 0:
            out.add(p)
            m //= p
        p += 1
    if m > 1:
        out.add(m)
    return sorted(out)


def sqrt_mod(a, p):
    """Tonelli-Shanks."""
    a %= p
    if a == 0:
        return 0
    if pow(a, (p - 1) // 2, p) != 1:
        raise ValueError("not a square")
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


# ---------------------------------------------------------------- the curve

O = None


class Koblitz:
    def __init__(self, F, a):
        self.F, self.a = F, a

    def on_curve(self, P):
        if P is O:
            return True
        F, (x, y) = self.F, P
        x2 = F.sqr(x)
        return (F.sqr(y) ^ F.mul(x, y)) == (F.mul(x2, x) ^ (x2 if self.a else 0) ^ 1)

    def neg(self, P):
        return O if P is O else (P[0], P[0] ^ P[1])

    def add(self, P, Q):
        F = self.F
        if P is O:
            return Q
        if Q is O:
            return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if y2 == x1 ^ y1:
                return O
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sqr(lam) ^ lam ^ self.a
            return (x3, F.sqr(x1) ^ F.mul(lam ^ 1, x3))
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sqr(lam) ^ lam ^ x1 ^ x2 ^ self.a
        return (x3, F.mul(lam, x1 ^ x3) ^ x3 ^ y1)

    def mul(self, k, P):
        if k < 0:
            return self.mul(-k, self.neg(P))
        R, A = O, P
        while k:
            if k & 1:
                R = self.add(R, A)
            A = self.add(A, A)
            k >>= 1
        return R

    def order(self):
        n = self.F.n
        t1 = -1 if self.a == 0 else 1
        s_prev, s = 2, t1
        for _ in range(n - 1):
            s_prev, s = s, t1 * s - 2 * s_prev
        return 2 ** n + 1 - s, 2 ** n + 1 - (2 ** n + 1 - s)

    def lifts(self, x):
        """All points with abscissa x (y^2 + xy = x^3 + a x^2 + 1)."""
        F = self.F
        if x == 0:
            return [(0, F.frob(1, F.n - 1))]
        c = x ^ self.a ^ F.sqr(F.inv(x))  # u^2 + u = x + a + 1/x^2, y = x u
        if F.trace(c):
            return []
        u = F.half_trace(c)
        y = F.mul(x, u)
        return [(x, y), (x, y ^ x)]


def public_hash_target(E, n, a, seed, cofactor, r):
    """The public hash-to-curve the ic workflow documents for public_hash_seed
    (domain "ic-workflow-public-target-v1\\0"; n u32, a u8, k u32 = 1,
    b_index u64 = 1, seed u64, counter u64, all little-endian)."""
    domain = b"ic-workflow-public-target-v1\0"
    mask = (1 << n) - 1
    for counter in range(1_000_000):
        msg = domain + struct.pack("<I", n) + bytes([a]) + struct.pack("<I", 1) \
            + struct.pack("<Q", 1) + struct.pack("<Q", seed) + struct.pack("<Q", counter)
        dig = blake3_single_chunk(msg)
        x = struct.unpack("<Q", dig[:8])[0] & mask
        lifts = sorted(E.lifts(x), key=lambda P: (P[0] + 1, P[1]))
        if not lifts:
            continue
        raw = lifts[(dig[8] & 1) % len(lifts)]
        T = E.mul(cofactor, raw)
        if T is O:
            continue
        return {"point": T, "counter": counter, "message_len": len(msg),
                "message_sha256": hashlib.sha256(msg).hexdigest(), "digest": dig.hex(),
                "in_subgroup": E.mul(r, T) is O}
    raise ValueError("hash-to-curve cap exhausted")


# ---------------------------------------------------------------- driver


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--registry", required=True)
    ap.add_argument("--slug", required=True)
    ap.add_argument("--params", required=True)
    ap.add_argument("--target-record", required=True)
    ap.add_argument("--recovered", nargs="+", required=True)
    args = ap.parse_args()

    checks = {}
    rep = {"inputs": {"registry": args.registry, "slug": args.slug, "params": args.params,
                      "target_record": args.target_record}}

    # BLAKE3 self-test
    checks["blake3_selftest"] = all(blake3_single_chunk(k).hex() == v for k, v in BLAKE3_SELFTEST.items())

    reg = json.load(open(args.registry))
    entry = [c for c in reg["curves"] if c.get("slug") == args.slug]
    if len(entry) != 1:
        sys.exit(f"registry has {len(entry)} entries for {args.slug}")
    entry = entry[0]
    params = json.load(open(args.params))
    trec = json.load(open(args.target_record))

    n = params["curve"]["degree"]
    a = params["curve"]["curve_a"]
    seed = params["targets"][0]["public_hash_seed"]
    f = int(entry["params"]["modulus"], 16)
    rp = entry["representations"][0]
    f_exp = sum(1 << e for e in rp["field"]["modulus_exponents"])
    hashed = trec["target"]["hashed"]
    f_rec = (1 << hashed["curve"]["degree"]) | sum(1 << t for t in hashed["curve"]["irreducible_low_terms"])
    r = int(rp["curve"]["subgroup_order"])
    cof = int(rp["curve"]["cofactor"])
    G = (int(rp["curve"]["generator"][0], 16), int(rp["curve"]["generator"][1], 16))
    G_rec = tuple(int(v) for v in hashed["curve"]["generator"])
    Q_rec = tuple(int(v) for v in trec["target"]["point"])

    checks["modulus_encodings_agree"] = f == f_exp == f_rec
    checks["degree_agrees"] = f.bit_length() - 1 == n == entry["params"]["n"] == hashed["curve"]["degree"]
    checks["a_agrees"] = a == entry["params"]["a"] == hashed["curve"]["curve_a"]
    checks["registry_b_is_1"] = rp["curve"]["coefficients"] == [1, a, 0, 0, 1]
    checks["generator_agrees_with_target_record"] = G == G_rec
    checks["subgroup_order_agrees_with_target_record"] = r == int(hashed["curve"]["subgroup_order"])
    checks["target_seed_agrees"] = trec["target"]["record"]["public_hash_seed"] == seed

    F = Field(f)
    E = Koblitz(F, a)
    checks["modulus_irreducible"] = F.irreducible()
    order, trace = E.order()
    checks["order_matches_registry"] = order == int(entry["order"])
    checks["trace_matches_registry"] = trace == entry["trace"]
    checks["r_prime"] = is_prime(r)
    checks["r_times_cofactor_is_order"] = r * cof == order
    checks["G_on_curve"] = E.on_curve(G)
    checks["G_exact_order_r"] = G is not O and E.mul(r, G) is O  # r prime
    # Frobenius eigenvalue: lambda^2 - t1*lambda + 2 = 0 mod r
    t1 = -1 if a == 0 else 1
    disc = (t1 * t1 - 8) % r
    sq = sqrt_mod(disc, r)
    inv2 = pow(2, -1, r)
    roots = sorted({(t1 + sq) * inv2 % r, (t1 - sq) * inv2 % r})
    frobG = (F.sqr(G[0]), F.sqr(G[1]))
    lam_ok = [lam for lam in roots if E.mul(lam, G) == frobG]
    checks["frobenius_eigenvalue_found"] = len(lam_ok) == 1

    tgt = public_hash_target(E, n, a, seed, cof, r)
    Q = tgt["point"]
    checks["Q_derived_equals_recorded"] = Q == Q_rec
    checks["Q_counter_equals_recorded"] = tgt["counter"] == trec["target"]["record"]["public_hash_counter"]
    checks["Q_on_curve"] = E.on_curve(Q)
    checks["Q_in_subgroup"] = tgt["in_subgroup"]

    recs = []
    ds = set()
    for p in args.recovered:
        d = json.load(open(p))
        ok_meta = (d.get("status") == "complete" and d.get("all_verified") is True
                   and d.get("curve") == args.slug and d.get("n") == n and d.get("a") == a
                   and int(d.get("r")) == r and isinstance(d.get("recovered"), list)
                   and len(d["recovered"]) == 1)
        recs.append({"path": p, "meta_ok": ok_meta, "recovered": d.get("recovered")})
        if ok_meta:
            ds.add(int(d["recovered"][0]))
    checks["all_recovered_records_meta_ok"] = all(x["meta_ok"] for x in recs)
    checks["one_distinct_recovered_scalar"] = len(ds) == 1
    scal = []
    for dv in sorted(ds):
        scal.append({"d": dv, "in_range": 1 <= dv < r, "dG_equals_Q": E.mul(dv, G) == Q})
    checks["every_scalar_dG_equals_Q"] = bool(scal) and all(s["in_range"] and s["dG_equals_Q"] for s in scal)
    # Negative controls: the test must be able to fail.
    if scal:
        checks["negative_control_d_plus_1_rejected"] = E.mul(scal[0]["d"] + 1, G) != Q
    other = public_hash_target(E, n, a, seed + 1, cof, r)["point"]
    checks["negative_control_seed_plus_1_gives_other_Q"] = other != Q and E.on_curve(other)

    rep.update({
        "curve": {"n": n, "a": a, "modulus_hex": hex(f),
                  "modulus_exponents": [i for i in range(f.bit_length()) if f >> i & 1][::-1],
                  "order": order, "trace": trace, "subgroup_order": r, "cofactor": cof,
                  "generator": [hex(G[0]), hex(G[1])], "generator_dec": [str(G[0]), str(G[1])],
                  "frobenius_lambda": lam_ok[0] if lam_ok else None},
        "target": {"public_hash_seed": seed, "counter": tgt["counter"],
                   "hash_message_len": tgt["message_len"], "hash_message_sha256": tgt["message_sha256"],
                   "digest": tgt["digest"], "Q_dec": [str(Q[0]), str(Q[1])],
                   "Q_recorded_dec": [str(Q_rec[0]), str(Q_rec[1])]},
        "recovered_records": len(recs), "scalars": scal, "checks": checks,
        "all_ok": all(checks.values()),
    })
    print(json.dumps(rep, indent=1))
    return 0 if rep["all_ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
