# EXP16b (symbolic part): parity of the elementary symmetric variables in the char-2 symmetrised
# summation polynomial, and the gradient identity w(x_j) dF/dx_j = w(x_0) dF/dx_0 behind it
# (invariant differential dx/w(x)).
# Ordinary curve y^2 + xy = x^3 + a x^2 + b (S3 = e2^2 + e3 + b, independent of a; w(x) = x), and a
# supersingular control y^2 + y = x^3 + a4 x + a6 (a1 = 0, a3 = 1; w = 1): there the prediction is the
# mirror image: e_2..e_m even, e_1 odd.
# usage: sage exp16b_symbolic.sage mmax_symbolic mmax_numeric
import sys, time
proof.polynomial(False)
CPU = time.process_time
mmax_sym, mmax_num = int(sys.argv[1]), int(sys.argv[2])

def build(base, m, S3):
    """S_{m+1}(x1..xm, x0) over base, by S_{k+1}(x1..xk,x0) = Res_Y(S_k(x1..x_{k-1},Y), S3(x_k,x0,Y))."""
    names = ['x%d' % i for i in range(1, m + 1)] + ['x0', 'Y']
    R = PolynomialRing(base, names, order='lex')
    xs = R.gens()[:m]; x0 = R.gens()[m]; Y = R.gens()[m + 1]
    if m == 2: return R, xs, x0, S3(xs[0], xs[1], x0)
    F = S3(xs[0], xs[1], Y)                 # S_3(x1, x2, Y)
    for k in range(3, m + 1):
        F = F.resultant(S3(xs[k - 1], x0, Y), Y)   # S_k(x1..x_{k-1}, x0)
        if k < m: F = F.subs({x0: Y})              # x0 becomes the free slot Y for the next round
    return R, xs, x0, F

def symred(F, R, xs, m):
    """F symmetric in xs -> dict {(k1..km, other exps): coeff} with F = sum coeff * e1^k1..em^km * other."""
    e = [sum(prod(c) for c in Combinations(xs, k)) for k in range(1, m + 1)]
    out = {}
    f = F
    while f != 0:
        lm = f.lm(); c = f.lc(); a = lm.exponents()[0]
        k = [a[i] - a[i + 1] for i in range(m - 1)] + [a[m - 1]]
        if min(k) < 0: raise ValueError("not symmetric")
        rest = R.monomial(*([0] * m + list(a[m:])))
        key = tuple(k) + tuple(a[m:])
        out[key] = out.get(key, 0) + c
        f -= c * prod(e[i] ^ k[i] for i in range(m)) * rest
    return out

def parity_report(G, m, tag):
    odd = [0] * m
    for key in G:
        for i in range(m):
            if key[i] % 2: odd[i] += 1
    print(f"  [{tag}] m={m}: {len(G)} monomials in (e1..e{m}, x0); monomials with odd exponent of e_k: "
          + ", ".join(f"e{i+1}:{odd[i]}" for i in range(m)), flush=True)

def gradient_report(F, xs, x0, m, tag, w):
    ref = w(x0) * F.derivative(x0)
    res = []
    for j in range(m):
        d = w(xs[j]) * F.derivative(xs[j]) - ref
        res.append('0' if d == 0 else 'NONZERO')
    print(f"  [{tag}] m={m}: w(x_j)dF/dx_j - w(x0)dF/dx0 for j=1..{m}: {res}  (x0 term zero? {ref == 0})", flush=True)

# ordinary curve, b symbolic
Bs = PolynomialRing(GF(2), 'b'); bsym = Bs.gen()
S3o = lambda a, b_, c: (a * b_ + a * c + b_ * c) ^ 2 + a * b_ * c + bsym
for m in range(2, mmax_sym + 1):
    c0 = CPU(); R, xs, x0, F = build(Bs, m, S3o)
    G = symred(F, R, xs, m)
    print(f"ordinary, b symbolic, m={m}: S_{m+1} degrees in (x1..xm,x0) {[F.degree(v) for v in list(xs)+[x0]]}, terms {len(F.monomials())} (cpu {CPU()-c0:.1f}s)", flush=True)
    parity_report(G, m, 'ordinary b symbolic')
    gradient_report(F, xs, x0, m, 'ordinary b symbolic', lambda t: t)

# ordinary curve, numeric b in GF(2^13), larger m
Kn = GF(2 ^ 13, 'g'); set_random_seed(1601); bn = Kn.random_element()
S3n = lambda a, b_, c: (a * b_ + a * c + b_ * c) ^ 2 + a * b_ * c + bn
for m in range(mmax_sym + 1, mmax_num + 1):
    c0 = CPU(); R, xs, x0, F = build(Kn, m, S3n)
    G = symred(F, R, xs, m)
    print(f"ordinary, b = random in GF(2^13), m={m}: S_{m+1} terms {len(F.monomials())}, degrees {[F.degree(v) for v in list(xs)+[x0]]} (cpu {CPU()-c0:.1f}s)", flush=True)
    parity_report(G, m, 'ordinary b numeric')
    gradient_report(F, xs, x0, m, 'ordinary b numeric', lambda t: t)

# supersingular control: y^2 + y = x^3 + a4 x + a6. S3 by elimination of y1, y2.
A = PolynomialRing(GF(2), 'a4,a6'); a4, a6 = A.gens()
Re = PolynomialRing(GF(2), 'a4f,a6f,y1,y2,x1,x2,x3', order='lex'); a4f, a6f, y1, y2, x1, x2, x3 = Re.gens()
C = lambda x, y: y ^ 2 + y + x ^ 3 + a4f * x + a6f
N = (x3 + x1 + x2) * (x1 + x2) ^ 2 + (y1 + y2) ^ 2          # x3 = lambda^2 + x1 + x2, lambda = (y1+y2)/(x1+x2)
r = N.resultant(C(x1, y1), y1).resultant(C(x2, y2), y2)
fl = r.factor()
print("supersingular: eliminant factors ((deg x1, x2, x3), multiplicity):", [((f.degree(x1), f.degree(x2), f.degree(x3)), e) for f, e in fl], flush=True)
S3s_flat = [f for f, e in fl if f.degree(x3) == 2][0]
print("supersingular S3 =", S3s_flat, flush=True)
for m in range(2, mmax_sym + 1):
    def S3sub(p, q, s, _P=S3s_flat):
        Rt_ = p.parent()
        return _P(Rt_(a4), Rt_(a6), 0, 0, p, q, s)
    c0 = CPU(); R, xs, x0, F = build(A, m, S3sub)
    G = symred(F, R, xs, m)
    print(f"supersingular, a4,a6 symbolic, m={m}: S_{m+1} terms {len(F.monomials())} (cpu {CPU()-c0:.1f}s)", flush=True)
    parity_report(G, m, 'supersingular')
    gradient_report(F, xs, x0, m, 'supersingular', lambda t: 1)
print("DONE", flush=True)
