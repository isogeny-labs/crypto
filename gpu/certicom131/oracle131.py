#!/usr/bin/env python3
"""Reference for the two 131-bit Certicom ECC challenges, ECCp-131 and ECC2-131.

    python3 oracle131.py check            verify every published parameter
    python3 oracle131.py params > params131.h
    python3 oracle131.py vectors > vectors131.h

Parameters are Certicom's (https://www.certicom.com/en/curves-list, read
2026-10-17; ECC2-131 also matches src/binary_ecc/curve.rs). `check` proves,
without trusting the transcription, that each field modulus is prime or
irreducible, both curves are nonsingular, the generator P and target Q lie on
the curve and have the published prime order n, and n * h is the group order
bound Hasse allows. `vectors` emits field and group test vectors that
test131.cpp and the device selftest hold the C++ arithmetic to.
"""
import random
import sys

# ---- ECCp-131: y^2 = x^3 + a x + b over F_p -------------------------------
P131 = dict(
    p=0x048E1D43F293469E33194C43186B3ABC0B,
    a=0x041CB121CE2B31F608A76FC8F23D73CB66,
    b=0x02F74F717E8DEC90991E5EA9B2FF03DA58,
    gx=0x03DF84A96B5688EF574FA91A32E197198A,
    gy=0x014721161917A44FB7B4626F36F0942E71,
    n=0x048E1D43F293469E317F7ED728F6B8E6F1, h=1,
    qx=0x03AA6F004FC62E2DA1ED0BFB62C3FFB568,
    qy=0x009C21C284BA8A445BB2701BF55E3A67ED,
)
# ---- ECC2-131: y^2 + x y = x^3 + a x^2 + b over F_2[t]/(t^131+t^13+t^2+t+1)
B131 = dict(
    m=131, terms=(0, 1, 2, 13),
    a=0x07EBCB7EECC296A1C4A1A14F2C9E44352E,
    b=0x00610B0A57C73649AD0093BDD622A61D81,
    gx=0x00439CBC8DC73AA981030D5BC57B331663,
    gy=0x014904C07D4F25A16C2DE036D60B762BD4,
    n=0x0400000000000000026ABB991FE311FE83, h=2,
    qx=0x0602339C5DB0E9C694AC8908528C51C440,
    qy=0x04F7B99169FA1A0F2737813742B1588CB8,
)
FB = sum(1 << t for t in B131["terms"]) | (1 << B131["m"])


def is_prime(m):
    if m < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if m % q == 0:
            return m == q
    d, s = m - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for w in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53):
        x = pow(w, d, m)
        if x in (1, m - 1):
            continue
        for _ in range(s - 1):
            x = x * x % m
            if x == m - 1:
                break
        else:
            return False
    return True


# ---- prime curve ------------------------------------------------------------
def p_add(P, Q, c=P131):
    p, a = c["p"], c["a"]
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if P == Q:
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def p_on(P, c=P131):
    x, y = P
    return (y * y - (x ** 3 + c["a"] * x + c["b"])) % c["p"] == 0


# ---- binary field and curve ---------------------------------------------------
def clmul(x, y):
    r = 0
    while y:
        if y & 1:
            r ^= x
        x <<= 1
        y >>= 1
    return r


def b_red(x):
    m = B131["m"]
    while x.bit_length() > m:
        x ^= FB << (x.bit_length() - 1 - m)
    return x


def b_mul(x, y):
    return b_red(clmul(x, y))


def b_inv(x):
    # x^(2^m - 2)
    r, e, base = 1, (1 << B131["m"]) - 2, x
    while e:
        if e & 1:
            r = b_mul(r, base)
        base = b_mul(base, base)
        e >>= 1
    return r


def b_add(P, Q, c=B131):
    a = c["a"]
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if y1 == y2 and x1 != 0:          # doubling
            lam = x1 ^ b_mul(y1, b_inv(x1))
            x3 = b_mul(lam, lam) ^ lam ^ a
            return (x3, b_mul(x1, x1) ^ b_mul(lam ^ 1, x3))
        return None                       # Q = -P = (x, x + y), or 2P with x = 0
    lam = b_mul(y1 ^ y2, b_inv(x1 ^ x2))
    x3 = b_mul(lam, lam) ^ lam ^ x1 ^ x2 ^ a
    return (x3, b_mul(lam, x1 ^ x3) ^ x3 ^ y1)


def b_on(P, c=B131):
    x, y = P
    return b_mul(y, y) ^ b_mul(x, y) ^ b_mul(b_mul(x, x), x) ^ b_mul(c["a"], b_mul(x, x)) ^ c["b"] == 0


def b_irreducible():
    # f | t^(2^m) - t means every irreducible factor of f has degree dividing
    # m; m = 131 is prime, so each factor has degree 1 or 131, and f has no
    # degree-1 factor when it has no root in F_2 (f(0) = 1 and f(1) = 1).
    m, x = B131["m"], 2
    for _ in range(m):
        x = b_mul(x, x)
    no_root = (FB & 1) == 1 and bin(FB).count("1") % 2 == 1
    return x == 2 and is_prime(m) and no_root


def mul(add, k, P):
    R = None
    while k:
        if k & 1:
            R = add(R, P)
        P = add(P, P)
        k >>= 1
    return R


def check(out=sys.stdout):
    c, d = P131, B131
    p = c["p"]
    rows = [
        ("ECCp-131 p is a 131-bit prime", is_prime(p) and p.bit_length() == 131),
        ("ECCp-131 n is a 131-bit prime", is_prime(c["n"]) and c["n"].bit_length() == 131),
        ("ECCp-131 curve is nonsingular", (4 * c["a"] ** 3 + 27 * c["b"] ** 2) % p != 0),
        ("ECCp-131 P on curve", p_on((c["gx"], c["gy"]))),
        ("ECCp-131 Q on curve", p_on((c["qx"], c["qy"]))),
        ("ECCp-131 n P = O", mul(p_add, c["n"], (c["gx"], c["gy"])) is None),
        ("ECCp-131 n Q = O", mul(p_add, c["n"], (c["qx"], c["qy"])) is None),
        ("ECCp-131 n h within Hasse bound", abs(c["n"] * c["h"] - (p + 1)) <= 2 * int(p ** 0.5) + 2),
        ("ECC2-131 f = t^131+t^13+t^2+t+1 irreducible", b_irreducible()),
        ("ECC2-131 n is a prime", is_prime(d["n"])),
        ("ECC2-131 curve is nonsingular (b != 0)", d["b"] != 0),
        ("ECC2-131 P on curve", b_on((d["gx"], d["gy"]))),
        ("ECC2-131 Q on curve", b_on((d["qx"], d["qy"]))),
        ("ECC2-131 n P = O", mul(b_add, d["n"], (d["gx"], d["gy"])) is None),
        ("ECC2-131 n Q = O", mul(b_add, d["n"], (d["qx"], d["qy"])) is None),
        ("ECC2-131 n h within Hasse bound", abs(d["n"] * d["h"] - (2 ** 131 + 1)) <= 2 * 2 ** 66),
    ]
    ok = True
    for name, good in rows:
        print(("PASS " if good else "FAIL ") + name, file=out)
        ok &= bool(good)
    return ok


def limbs32(x, n):
    return ", ".join("0x%08xu" % ((x >> (32 * i)) & 0xFFFFFFFF) for i in range(n))


def limbs64(x, n):
    return ", ".join("0x%016xull" % ((x >> (64 * i)) & 0xFFFFFFFFFFFFFFFF) for i in range(n))


def vectors(count=256, seed=0x131131):
    rnd = random.Random(seed)
    c, d = P131, B131
    p = c["p"]
    out = ["/* Generated by oracle131.py vectors.  Do not edit. */",
           "#ifndef CERTICOM131_VECTORS_H", "#define CERTICOM131_VECTORS_H",
           "static const int VEC_COUNT = %d;" % count]
    # prime field: a, b, a*b, a^-1 (canonical, little-endian 32-bit limbs)
    pa = [rnd.randrange(p) for _ in range(count)]
    pb = [rnd.randrange(p) for _ in range(count)]
    pa[0], pb[0], pa[1], pb[1] = 1, p - 1, p - 1, p - 1
    out.append("static const uint32_t VP_A[%d][5] = {%s};" % (count, ", ".join("{%s}" % limbs32(x, 5) for x in pa)))
    out.append("static const uint32_t VP_B[%d][5] = {%s};" % (count, ", ".join("{%s}" % limbs32(x, 5) for x in pb)))
    out.append("static const uint32_t VP_MUL[%d][5] = {%s};" % (count, ", ".join("{%s}" % limbs32(x * y % p, 5) for x, y in zip(pa, pb))))
    out.append("static const uint32_t VP_INV[%d][5] = {%s};" % (count, ", ".join("{%s}" % limbs32(pow(x, -1, p), 5) for x in pa)))
    # binary field: a, b, a*b, a^2, a^-1 (polynomial basis, 64-bit words)
    ba = [rnd.getrandbits(131) for _ in range(count)]
    bb = [rnd.getrandbits(131) for _ in range(count)]
    ba[0], bb[0], ba[1], bb[1] = 1, (1 << 131) - 1, (1 << 131) - 1, (1 << 131) - 1
    ba = [x or 1 for x in ba]
    out.append("static const uint64_t VB_A[%d][3] = {%s};" % (count, ", ".join("{%s}" % limbs64(x, 3) for x in ba)))
    out.append("static const uint64_t VB_B[%d][3] = {%s};" % (count, ", ".join("{%s}" % limbs64(x, 3) for x in bb)))
    out.append("static const uint64_t VB_MUL[%d][3] = {%s};" % (count, ", ".join("{%s}" % limbs64(b_mul(x, y), 3) for x, y in zip(ba, bb))))
    out.append("static const uint64_t VB_SQR[%d][3] = {%s};" % (count, ", ".join("{%s}" % limbs64(b_mul(x, x), 3) for x in ba)))
    out.append("static const uint64_t VB_INV[%d][3] = {%s};" % (count, ", ".join("{%s}" % limbs64(b_inv(x), 3) for x in ba)))
    # group vectors: k, kP (both curves), and P+Q, 2P for the generator and target
    ks = [rnd.randrange(1, c["n"]) for _ in range(16)]
    G, Q = (c["gx"], c["gy"]), (c["qx"], c["qy"])
    out.append("static const uint32_t VPK[16][5] = {%s};" % ", ".join("{%s}" % limbs32(k, 5) for k in ks))
    out.append("static const uint32_t VPKG[16][2][5] = {%s};" % ", ".join(
        "{{%s},{%s}}" % (limbs32(R[0], 5), limbs32(R[1], 5)) for R in (mul(p_add, k, G) for k in ks)))
    S, D = p_add(G, Q), p_add(G, G)
    out.append("static const uint32_t VP_GQ[2][5] = {{%s},{%s}};" % (limbs32(S[0], 5), limbs32(S[1], 5)))
    out.append("static const uint32_t VP_2G[2][5] = {{%s},{%s}};" % (limbs32(D[0], 5), limbs32(D[1], 5)))
    ks = [rnd.randrange(1, d["n"]) for _ in range(16)]
    G, Q = (d["gx"], d["gy"]), (d["qx"], d["qy"])
    out.append("static const uint64_t VBK[16][3] = {%s};" % ", ".join("{%s}" % limbs64(k, 3) for k in ks))
    out.append("static const uint64_t VBKG[16][2][3] = {%s};" % ", ".join(
        "{{%s},{%s}}" % (limbs64(R[0], 3), limbs64(R[1], 3)) for R in (mul(b_add, k, G) for k in ks)))
    S, D = b_add(G, Q), b_add(G, G)
    out.append("static const uint64_t VB_GQ[2][3] = {{%s},{%s}};" % (limbs64(S[0], 3), limbs64(S[1], 3)))
    out.append("static const uint64_t VB_2G[2][3] = {{%s},{%s}};" % (limbs64(D[0], 3), limbs64(D[1], 3)))
    out.append("#endif")
    return "\n".join(out) + "\n"


def params():
    c, d = P131, B131
    p = c["p"]
    R = 1 << 160
    ninv = (-pow(p, -1, 1 << 32)) % (1 << 32)
    out = ["/* Generated by oracle131.py params.  Do not edit. */",
           "#ifndef CERTICOM131_PARAMS_H", "#define CERTICOM131_PARAMS_H",
           "/* ECCp-131 over F_p, p < 2^131, Montgomery radix R = 2^160 (five 32-bit limbs). */",
           "#define P131_P {%s}" % limbs32(p, 5),
           "#define P131_NINV 0x%08xu" % ninv,
           "#define P131_R1 {%s}" % limbs32(R % p, 5),
           "#define P131_R2 {%s}" % limbs32(R * R % p, 5),
           "#define P131_PM2 {%s}" % limbs32(p - 2, 5),
           "#define P131_A {%s}" % limbs32(c["a"], 5),
           "#define P131_B {%s}" % limbs32(c["b"], 5),
           "#define P131_GX {%s}" % limbs32(c["gx"], 5),
           "#define P131_GY {%s}" % limbs32(c["gy"], 5),
           "#define P131_QX {%s}" % limbs32(c["qx"], 5),
           "#define P131_QY {%s}" % limbs32(c["qy"], 5),
           "#define P131_N {%s}" % limbs32(c["n"], 5),
           "/* ECC2-131 over F_2[t]/(t^131 + t^13 + t^2 + t + 1), polynomial basis, 64-bit words. */",
           "#define B131_A {%s}" % limbs64(d["a"], 3),
           "#define B131_B {%s}" % limbs64(d["b"], 3),
           "#define B131_GX {%s}" % limbs64(d["gx"], 3),
           "#define B131_GY {%s}" % limbs64(d["gy"], 3),
           "#define B131_QX {%s}" % limbs64(d["qx"], 3),
           "#define B131_QY {%s}" % limbs64(d["qy"], 3),
           "#define B131_N {%s}" % limbs32(d["n"], 5),
           "#endif"]
    return "\n".join(out) + "\n"


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "check"
    if cmd == "check":
        sys.exit(0 if check() else 1)
    elif cmd == "params":
        if not check(out=sys.stderr):
            sys.exit("parameter check failed; no header written")
        sys.stdout.write(params())
    elif cmd == "vectors":
        if not check(out=sys.stderr):
            sys.exit("parameter check failed; no vectors written")
        sys.stdout.write(vectors())
    else:
        sys.exit(__doc__)
