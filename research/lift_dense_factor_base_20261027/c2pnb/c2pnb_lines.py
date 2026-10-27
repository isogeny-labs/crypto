#!/usr/bin/env python3
"""All-lift F_Q-lines u*F_Q for the composite-degree c2pnb curves.

For E: y^2 + xy = x^3 + a x^2 + b over F_{2^N}, N = 16k, a, b in F_{2^16},
Q = 2^{4k}: a nonzero x lifts to E iff Tr_N(x + a + b/x^2) = 0. With
x = u*y, y in F_Q, u in F_{2^16}:
  Tr_N(b/x^2) = Tr_N(c/x)          (c = sqrt b)
              = Tr_Q(T(c/u) / y),   T = Tr_{F_{2^N}/F_Q}
  Tr_N(x)     = Tr_Q(T(u) * y).
So T(u) = T(c/u) = 0 and Tr_N(a) = 0 make every nonzero x in u*F_Q lift,
and u in F_{2^16} keeps the line pi_16-stable. This script checks the
hypotheses on the real parameters, counts such u, and verifies lifting on
sampled line elements by direct computation (no identity assumed).

Usage: python3 c2pnb_lines.py <binary-curve-params.txt> <out.json>
"""
import json, random, re, sys

CURVES = ["c2pnb176v1", "c2pnb208w1", "c2pnb272w1", "c2pnb304w1", "c2pnb368w1"]


def parse(path):
    txt = open(path).read()
    out = {}
    for name in CURVES:
        blk = txt.split("===== %s =====" % name)[1].split("=====")[0]
        def field(key):
            m = re.search(r"\n%s:\s*(.*?)\n(?=[A-Z][a-zA-Z ()]*:)" % key, blk, re.S)
            s = m.group(1).strip()
            s = re.sub(r"[\s:]", "", s)
            return int(s, 16) if s not in ("0",) else 0
        out[name] = dict(poly=field("Polynomial"), a=field("A"), b=field("B"))
    return out


class F:
    def __init__(self, poly):
        self.poly = poly
        self.N = poly.bit_length() - 1

    def mul(self, a, b):
        r = 0
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if (a >> self.N) & 1:
                a ^= self.poly
        return r

    def sq(self, a):
        return self.mul(a, a)

    def frob(self, a, k):  # a^(2^k)
        for _ in range(k):
            a = self.sq(a)
        return a

    def pow(self, a, e):
        r = 1
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.sq(a)
            e >>= 1
        return r

    def inv(self, a):
        return self.pow(a, (1 << self.N) - 2)

    def tr_rel(self, a, d):  # Tr_{F_{2^N}/F_{2^d}}(a)
        t, z = 0, a
        for _ in range(self.N // d):
            t ^= z
            z = self.frob(z, d)
        return t

    def tr(self, a):
        return self.tr_rel(a, 1)


def span_basis(vs):
    piv = {}
    basis = []
    for v in vs:
        x = v
        while x:
            h = x.bit_length() - 1
            if h in piv:
                x ^= piv[h]
            else:
                piv[h] = x
                basis.append(v)
                break
    return basis


def main():
    params = parse(sys.argv[1])
    rng = random.Random(0x11F720261027)
    rows = []
    for name, p in params.items():
        f = F(p["poly"])
        N = f.N
        k = N // 16
        in16 = lambda z: f.frob(z, 16) == z
        a, b = p["a"], p["b"]
        row = dict(curve=name, N=N, k=k, a_in_F16=in16(a), b_in_F16=in16(b))
        tra = f.tr(a)
        row["Tr_a"] = tra
        # F_{2^16}: span of 16 independent Tr_{N/16} images
        cands = []
        while len(span_basis(cands)) < 16:
            cands.append(f.tr_rel(rng.getrandbits(N), 16))
        B16 = span_basis(cands)
        assert len(B16) == 16 and all(in16(z) for z in B16)
        # T restricted to F_{2^16}: u^(2^{4k j}) = u^(2^{(4kj) mod 16})
        def T16(u):
            t = 0
            for j in range(4):
                t ^= f.frob(u, (4 * k * j) % 16)
            return t
        # kernel of the F_2-linear map u -> T16(u) on F_{2^16}
        imgs = [T16(z) for z in B16]
        # Gaussian elimination over combos: brute force 2^16 via Gray code
        ker = []
        u, tu = 0, 0
        for g in range(1, 1 << 16):
            i = (g & -g).bit_length() - 1
            u ^= B16[i]
            tu ^= imgs[i]
            if tu == 0:
                ker.append(u)
        c = f.frob(b, 15) if in16(b) else None  # sqrt(b) in F_{2^16}
        assert c is not None and f.sq(c) == b
        good = []
        for u in ker:
            uinv = f.pow(u, (1 << 16) - 2)  # inverse inside F_{2^16}
            assert f.mul(u, uinv) == 1
            if T16(f.mul(c, uinv)) == 0:
                good.append(u)
        row["u_kernel_T"] = len(ker)
        row["u_all_lift"] = len(good)
        # direct verification on sampled line elements (and the standard line u = 1)
        def lifts(x):
            return f.tr(x ^ a ^ f.mul(b, f.inv(f.sq(x)))) == 0
        def sample_FQ():
            return f.tr_rel(rng.getrandbits(N), 4 * k)
        S = 40
        if good:
            u = good[0]
            hits = 0
            for _ in range(S):
                y = sample_FQ()
                if y == 0:
                    continue
                hits += lifts(f.mul(u, y))
            row["sampled_lift_twisted"] = [hits, S]
            row["u_example_hex"] = hex(u)
        std = 0
        for _ in range(S):
            y = sample_FQ()
            if y:
                std += lifts(y)
        row["sampled_lift_standard"] = [std, S]
        print(row, flush=True)
        rows.append(row)
    json.dump(rows, open(sys.argv[2], "w"), indent=1)


if __name__ == "__main__":
    main()
