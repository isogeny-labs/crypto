"""Structural (scope-only) re-implementation of the hashed-ecdl-tensor encoder.

DERIVED from reading cryptanalysis/bindings/rust/hashed-ecdl-tensor/src/encoding.rs
(cargo build was refused by the permission system, so the Rust crate could not
be run to dump its graph).  Only the *graph structure* is reproduced (which
signals share which vertex); predicates are not evaluated.  Validated by
reproducing the crate's own published numbers for ECCp-131, m=2: 1,564,506 edges
and maximum degree 1,032 (docs/hashed-ecdl.md l.33).

mul = "fourier" (default, Theorem 3 gadget) or "school" (schoolbook, the crate's
own optional comparison encoder `schoolbook_multiply`).
"""

class Enc:
    def __init__(self, mul="fourier"):
        self.mul = mul
        self.uses = []          # uses[signal] = number of ports using it
        self.verts = []         # (label, [signal ids, with multiplicity])
        self.iface = []
        self.free = []          # free input signals, creation order
        self.twords = []        # forward-transform output words (list of signal lists)
        self.cwords = []        # column-stage middle words
        self.slopes = []
        self.sums = []
        self.label = "x"
        self.nmul = 0
        self.qbit = lambda i: i % 2 == 0

    def fresh(self, w, free=False):
        out = []
        for _ in range(w):
            self.uses.append(0)
            s = len(self.uses) - 1
            if free: self.free.append(s)
            out.append(s)
        return out
    def input_word(self, w): return self.fresh(w, True)
    @staticmethod
    def const(w): return [None] * w
    @staticmethod
    def slice(word, start, w):
        return [word[start + i] if start + i < len(word) else None for i in range(w)]
    @staticmethod
    def pad(word, w):
        return list(word) + [None] * (w - len(word))

    def constraint(self, words):
        act = [b for w in words for b in w if b is not None]
        self.verts.append((self.label, act))
        for s in act: self.uses[s] += 1

    def define(self, inputs, widths, checked=True):
        outs = [self.fresh(w) for w in widths]
        if checked:
            self.constraint(list(inputs) + outs)
        return outs

    def mark_interface(self, word):
        for b in word:
            if b is None:
                b = self.define([], [1])[0][0]
            self.iface.append(b)
            self.uses[b] += 1

    def assert_equal(self, l, r):
        for i in range(len(l)):
            self.constraint([[l[i]], [r[i]]])
    def gated_equal(self, sel, l, r):
        for i in range(len(l)):
            self.constraint([sel, [l[i]], [r[i]]])
    def not_(self, b): return self.define([b], [1])[0]
    def and_(self, a, b): return self.define([a, b], [1])[0]
    def or_(self, a, b): return self.define([a, b], [1])[0]
    def zero_test(self, word):
        seen = self.const(1)
        for i in range(len(word)):
            seen = self.or_(seen, self.slice(word, i, 1))
        return self.not_(seen)
    def less_than(self, a, b):
        less = self.const(1)
        for i in range(len(a)):
            less = self.define([self.slice(a, i, 1), self.slice(b, i, 1), less], [1])[0]
        return less
    def canonical(self, word):
        lt = self.less_than(word, self.const(len(word)))
        self.assert_equal(lt, self.const(1))
    def integer_add(self, a, b):
        carry = self.const(1); res = []
        for i in range(len(a)):
            o = self.define([self.slice(a, i, 1), self.slice(b, i, 1), carry], [1, 1])
            res.append(o[0][0]); carry = o[1]
        res.append(carry[0])
        return res

    @staticmethod
    def params(d):
        bound = (d + 1) * 64
        j = 16
        while j * j < bound: j *= 2
        k = j // 4
        n = -(-d // k)
        length = 1 << (2 * n - 1).bit_length()
        mu = length.bit_length() - 1
        first = 1 << (mu // 2)
        second = length // first
        assert length <= j // 2 and (2 * j) % length == 0
        return dict(d=d, j=j, k=k, n=n, length=length, first=first, second=second)

    def transform(self, inp, p, inverse, record):
        w = p["j"] + 1
        tg = "inv" if inverse else "fwd"
        middle = []
        self.label = tg + "_col"
        for j2 in range(p["second"]):
            ins = [inp[p["second"] * j1 + j2] for j1 in range(p["first"])]
            outs = self.define(ins, [w] * p["first"])
            middle.append(outs)
            if record: self.cwords.extend(outs)
        self.label = tg + "_row"
        out = [None] * p["length"]
        for t1 in range(p["first"]):
            ins = [middle[j2][t1] for j2 in range(p["second"])]
            res = self.define(ins, [w] * p["second"])
            for t2 in range(p["second"]):
                out[t1 + p["first"] * t2] = res[t2]
        if record: self.twords.extend(out)
        return out

    def integer_multiply(self, a, b):
        self.nmul += 1
        if self.mul == "school":
            return self.schoolbook(a, b)
        p = self.params(len(a))
        padded = lambda wd: [self.slice(wd, i * p["k"], p["k"]) if i < p["n"] else self.const(p["k"])
                             for i in range(p["length"])]
        rec = any(x is not None for x in a) and any(x is not None for x in b)
        fa = self.transform(padded(a), p, False, rec)
        fb = self.transform(padded(b), p, False, rec)
        self.label = "mul_pw"
        prods = [self.define([fa[i], fb[i]], [p["j"] + 1])[0] for i in range(p["length"])]
        co = self.transform(prods, p, True, False)
        self.label = "mul_rest"
        for c in co[2 * p["n"] - 1:]:
            self.assert_equal(c, self.const(p["j"] + 1))
        carry = self.const(p["j"] + 1); out = []
        for i in range(2 * p["n"]):
            coef = co[i] if i < 2 * p["n"] - 1 else self.const(p["j"] + 1)
            res = self.define([coef, carry], [p["k"], p["j"] + 1])
            out.extend(res[0]); carry = res[1]
        self.assert_equal(carry, self.const(p["j"] + 1))
        for i in range(2 * p["d"], len(out)):
            self.assert_equal(self.slice(out, i, 1), self.const(1))
        self.label = "x"
        return self.slice(out, 0, 2 * p["d"])

    def schoolbook(self, a, b):
        width = len(a) + len(b)
        s = self.const(width)
        for j in range(len(b)):
            row = [None] * width
            for i in range(len(a)):
                row[i + j] = self.and_(self.slice(a, i, 1), self.slice(b, j, 1))[0]
            nxt = self.integer_add(s, row)
            self.assert_equal(self.slice(nxt, width, 1), self.const(1))
            s = self.slice(nxt, 0, width)
        return s

    def enforce_modular_add(self, x, y, z):
        self.canonical(x); self.canonical(y); self.canonical(z)
        beta = self.define([x, y], [1], checked=False)[0]
        scaled = [beta[0] if self.qbit(i) else None for i in range(len(x))]
        s = self.integer_add(x, y)
        o = self.integer_add(z, scaled)
        self.assert_equal(s, o)
    def modular_add(self, x, y):
        z = self.define([x, y], [len(x)], checked=False)[0]
        self.enforce_modular_add(x, y, z); return z
    def modular_sub(self, x, y):
        self.canonical(x); self.canonical(y)
        z = self.define([x, y], [len(x)], checked=False)[0]
        self.enforce_modular_add(y, z, x); return z
    def modular_mul(self, x, y):
        self.canonical(x); self.canonical(y)
        z, theta = self.define([x, y], [len(x), len(x)], checked=False)
        self.canonical(z)
        prod = self.integer_multiply(x, y)
        qp = self.integer_multiply(theta, self.const(len(x)))
        sm = self.integer_add(qp, self.pad(z, 2 * len(x)))
        self.assert_equal(self.pad(prod, len(sm)), sm)
        return z

    def valid_point(self, pt):
        inf, x, y = pt
        w = len(x)
        self.canonical(x); self.canonical(y)
        zero = self.const(w)
        self.gated_equal(inf, x, zero); self.gated_equal(inf, y, zero)
        xx = self.modular_mul(x, x); xxx = self.modular_mul(xx, x)
        ax = self.modular_mul(x, self.const(w))
        rhs = self.modular_add(xxx, ax); rhs = self.modular_add(rhs, self.const(w))
        yy = self.modular_mul(y, y)
        fin = self.not_(inf)
        self.gated_equal(fin, yy, rhs)

    def input_point(self, w):
        inf = self.const(1); x = self.input_word(w); y = self.input_word(w)
        pt = (inf, x, y); self.valid_point(pt); return pt
    def const_point(self, w): return (self.const(1), self.const(w), self.const(w))
    def gated_point_equal(self, sel, a, b):
        for i in range(3): self.gated_equal(sel, a[i], b[i])

    def point_add(self, a, b):
        w = len(a[1])
        self.valid_point(a); self.valid_point(b)
        inputs = [a[0], a[1], a[2], b[0], b[1], b[2]]
        o = self.define(inputs, [1, w, w], checked=False)
        res = (o[0], o[1], o[2]); self.valid_point(res)
        slope = self.define(inputs, [w], checked=False)[0]
        self.slopes.append(slope); self.sums.append(res[1] + res[2])
        self.canonical(slope)
        dx = self.modular_sub(a[1], b[1]); sy = self.modular_add(a[2], b[2])
        same_x = self.zero_test(dx); inv_y = self.zero_test(sy)
        fa_ = self.not_(a[0]); fb_ = self.not_(b[0])
        both = self.and_(fa_, fb_); sig2 = self.and_(fa_, b[0])
        sxf = self.and_(both, same_x); sig3 = self.and_(sxf, inv_y)
        dif = self.not_(same_x); sig4 = self.and_(both, dif)
        ninv = self.not_(inv_y); sig5 = self.and_(sxf, ninv)
        active = self.or_(sig4, sig5)
        self.gated_point_equal(a[0], res, b)
        self.gated_point_equal(sig2, res, a)
        self.gated_point_equal(sig3, res, (self.const(1), self.const(w), self.const(w)))
        x2x1 = self.modular_sub(b[1], a[1]); y2y1 = self.modular_sub(b[2], a[2])
        gs = self.modular_mul(slope, x2x1)
        self.gated_equal(sig4, gs, y2y1)
        two_y = self.modular_add(a[2], a[2])
        sq = self.modular_mul(a[1], a[1])
        tsq = self.modular_mul(sq, self.const(w))
        tan = self.modular_add(tsq, self.const(w))
        ds = self.modular_mul(slope, two_y)
        self.gated_equal(sig5, ds, tan)
        self.gated_equal(active, res[0], self.const(1))
        ss = self.modular_mul(slope, slope)
        ex = self.modular_sub(ss, a[1]); ex = self.modular_sub(ex, b[1])
        self.gated_equal(active, res[1], ex)
        xd = self.modular_sub(a[1], res[1])
        ey = self.modular_mul(slope, xd); ey = self.modular_sub(ey, a[2])
        self.gated_equal(active, res[2], ey)
        inact = self.not_(active)
        self.gated_equal(inact, slope, self.const(w))
        return res

    def hash_member(self, pt, sbits):
        inf, x, y = pt
        self.assert_equal(inf, self.const(1))
        bits = [y[0]] + list(x)
        enc = self.pad(bits, sbits)
        mult = self.modular_mul(enc, self.const(sbits))
        hashed = self.modular_add(mult, self.const(sbits))
        acc = self.less_than(hashed, self.const(sbits))
        self.assert_equal(acc, self.const(1))

    def edge_count(self):
        tot = 0
        for u in self.uses:
            u = max(u, 2)
            tot += 2 * u - 3
        return tot

def build(b, m, mul="fourier", sbits=None):
    """A_1+..+A_m = R, cofactor 1, hashed factor bases with s of sbits bits (default b+1)."""
    e = Enc(mul)
    sbits = sbits or b + 1
    sum_ = (e.const(1), e.const(b), e.const(b))
    for _ in range(m):
        pt = e.input_point(b)
        e.hash_member(pt, sbits)
        e.mark_interface(pt[0]); e.mark_interface(pt[1]); e.mark_interface(pt[2])
        sum_ = e.point_add(sum_, pt)
    tgt = e.const_point(b)
    for i in range(3): e.assert_equal(sum_[i], tgt[i])
    return e

def stats(e):
    mx = max(len(v[1]) for v in e.verts)
    return dict(signals=len(e.uses), vertices=len(e.verts), edges=e.edge_count(),
                max_vertex_arity=mx, multiplies=e.nmul, free_inputs=len(e.free))

if __name__ == "__main__":
    import sys
    b = int(sys.argv[1]); m = int(sys.argv[2]); mul = sys.argv[3] if len(sys.argv) > 3 else "fourier"
    e = build(b, m, mul)
    print(dict(b=b, m=m, mul=mul, **stats(e)))
