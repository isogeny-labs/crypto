# EXP16bv: adversarial re-check of exp16b (squaring reduction), built independently of exp14/exp16b code.
#  (1) S3 checked against actual curve points (y^2+xy=x^3+a x^2+b, a != 0), S5 checked on points.
#  (2) symmetrisation by normal form modulo (E_k - e_k(x)) (block order), not leading-term reduction;
#      odd-exponent counts of G(E1..E4) for x0 = x_R numeric.
#  (3) descent at a different Q with e_k(c0+u w) = sum_l C(m-l,k-l) c0^(k-l) u^l s_l (formula itself tested),
#      ideal degree before/after the adapted squaring, dim U vs prediction.
#  (4) Lemma 1 exactness for m = 5, 6 on specialisations (x2..xm random constants, x1, x0 symbolic).
# usage: sage exp16bv_check.sage d seed  (Q = 2^d, K = F_{2^{4d}})
import sys, time
proof.all(False)
CPU = time.process_time
d, seed_ = int(sys.argv[1]), int(sys.argv[2])
set_random_seed(seed_)
Q = 2 ^ d; N = 4 * d
K = GF(2 ^ N, 'g'); g = K.gen()
Fq, emb = K.subfield(d, name='z', map=True); z = Fq.gen()
# F_Q-coordinates in basis 1, g, g^2, g^3 via F_2-linear algebra
Bas = [emb(z ^ i) * g ^ j for j in range(4) for i in range(d)]
Mb = matrix(GF(2), [vector(GF(2), c._vector_()) for c in Bas]).transpose(); Mbi = Mb.inverse()
def coords(c):
    v = Mbi * vector(GF(2), c._vector_())
    return [sum(Fq(int(v[j * d + i])) * z ^ i for i in range(d)) for j in range(4)]
for _ in range(5):                                   # sanity of coords
    c = K.random_element(); cc = coords(c)
    assert sum(emb(cc[j]) * g ^ j for j in range(4)) == c

def S3(a_, b_, c_, bb): return (a_ * b_ + a_ * c_ + b_ * c_) ^ 2 + a_ * b_ * c_ + bb

print(f"# exp16bv_check: d={d} (Q={Q}), K=GF(2^{N}), seed {seed_}", flush=True)
while True:
    a = K.random_element(); b = K.random_element()
    if b != 0 and a != 0: break
E = EllipticCurve(K, [1, a, 0, 0, b])
# (1) S3 on points
ok3 = all(S3(P[0], Pp[0], (P + Pp)[0], b) == 0 for P, Pp in [(E.random_point(), E.random_point()) for _ in range(20)])
print(f"(1) S3 vanishes at (x(P1),x(P2),x(P1+P2)) for 20 random pairs on y^2+xy=x^3+a x^2+b (a!=0): {ok3}", flush=True)

Rx = PolynomialRing(K, 'x1,x2,x3,x4,x0,Y,Z'); x1, x2, x3, x4, x0, Y, Z = Rx.gens()
c1 = CPU()
S4_ = S3(x3, x4, Z, b).resultant(S3(x0, Y, Z, b), Z)       # S4(x3,x4,x0,Y)
F = S3(x1, x2, Y, b).resultant(S4_, Y)                      # S5(x1..x4,x0)
print(f"    S5 built: {len(F.monomials())} terms, degrees {[F.degree(v) for v in (x1,x2,x3,x4,x0)]} (cpu {CPU()-c1:.1f}s)", flush=True)
okp = True
for _ in range(10):
    Ps = [E.random_point() for _ in range(4)]; P0 = sum(Ps)
    if F(*([P[0] for P in Ps] + [P0[0], 0, 0])) != 0: okp = False
print(f"    S5 vanishes on 10 random (P1..P4, P1+..+P4): {okp}", flush=True)
D = [v * F.derivative(v) - x0 * F.derivative(x0) for v in (x1, x2, x3, x4)]
print(f"    Lemma 1, m=4 (b in GF(2^{N})): x_jF_j - x0F0 == 0 exactly for j=1..4: {[Dj == 0 for Dj in D]}", flush=True)

# (2) symmetrisation by normal form, with x0 = x_R numeric
R0 = E.random_point() * 2; xR = R0[0]
Rsym = PolynomialRing(K, 'x1,x2,x3,x4,E1,E2,E3,E4', order=TermOrder('degrevlex', 4) + TermOrder('degrevlex', 4))
X = Rsym.gens()[:4]; EE = Rsym.gens()[4:]
el = [sum(prod(c) for c in Combinations(X, k)) for k in range(1, 5)]
Isym = Rsym.ideal([EE[k] - el[k] for k in range(4)])
c1 = CPU()
# direct coercion: substitute variables
Fsub = F.subs({x0: xR})
Fx = Rsym(0)
for cf, mon in zip(Fsub.coefficients(), Fsub.monomials()):
    ex = mon.exponents()[0]
    Fx += cf * X[0] ^ ex[0] * X[1] ^ ex[1] * X[2] ^ ex[2] * X[3] ^ ex[3]
Gx = Isym.reduce(Fx)
assert all(m_.degree(X[i]) == 0 for m_ in Gx.monomials() for i in range(4)), "NF not in K[E]"
oddE = [sum(1 for m_ in Gx.monomials() if m_.degree(EE[k]) % 2) for k in range(4)]
print(f"(2) G(E1..E4) at x0=x_R by normal form: {len(Gx.monomials())} terms; odd-exponent counts E1..E4: {oddE} (cpu {CPU()-c1:.1f}s)", flush=True)

# (3) descent
RsK = PolynomialRing(K, 's1,s2,s3,s4'); sK = RsK.gens()
Rq = PolynomialRing(Fq, 's1,s2,s3,s4', order='degrevlex'); sq = Rq.gens()
Gfun = lambda e_: sum(cf * prod(e_[k] ^ mon.degree(EE[k]) for k in range(4)) for cf, mon in zip(Gx.coefficients(), Gx.monomials()))
def ek_formula(c0, u, ss):
    s_ = [1] + list(ss)
    return [sum(K(binomial(4 - l, k - l)) * c0 ^ (k - l) * u ^ l * s_[l] for l in range(k + 1)) for k in range(1, 5)]
# test the e_k formula on random points
okf = True
for _ in range(5):
    c0t, ut = K.random_element(), K.random_element(); w = [emb(Fq.random_element()) for _ in range(4)]
    xs = [c0t + ut * wi for wi in w]
    ed = [sum(prod(c) for c in Combinations(xs, k)) for k in range(1, 5)]
    sw = [sum(prod(c) for c in Combinations(w, k)) for k in range(1, 5)]
    if ed != ek_formula(c0t, ut, sw): okf = False
print(f"(3) e_k(c0+u w) formula check on 5 random cases: {okf}", flush=True)

def descend(c0, u):
    P = Gfun(ek_formula(c0, u, sK))
    eqs = [Rq(0)] * 4
    for cf, mon in zip(P.coefficients(), P.monomials()):
        cc = coords(cf); ex = mon.exponents()[0]
        mm = prod(sq[i] ^ ex[i] for i in range(4))
        for j in range(4): eqs[j] += cc[j] * mm
    return eqs
def Ubasis(eqs):
    rows = []
    for e in eqs:
        dv = [e.derivative(v) for v in sq]
        mons = set(m_ for dd in dv for m_ in dd.monomials())
        for m_ in mons: rows.append([dd.monomial_coefficient(m_) for dd in dv])
    return matrix(Fq, rows).right_kernel().basis() if rows else []
def spandim(r): return matrix(Fq, [coords(r ^ k) for k in range(4)]).rank()
def gdeg(R_, eqs):
    c1 = CPU(); gb = R_.ideal(eqs).groebner_basis('libsingular:std')
    return len(R_.ideal(gb).normal_basis()), CPU() - c1
def reduced(eqs, Ub):
    k = len(Ub); cols = list(Ub)
    for v in identity_matrix(Fq, 4).columns():
        if len(cols) < 4 and matrix(Fq, cols + [v]).rank() > len(cols): cols.append(v)
    Mm = matrix(Fq, cols).transpose()
    Ry = PolynomialRing(Fq, 'y1,y2,y3,y4'); yy = Mm * vector(Ry.gens())
    ye = [e(*yy) for e in eqs]
    odd = [sum(1 for e in ye for m_ in e.monomials() if m_.degree(Ry.gen(i)) % 2) for i in range(4)]
    Rt = PolynomialRing(Fq, 'y1,y2,y3,y4', order=TermOrder('wdegrevlex', tuple([2] * k + [1] * (4 - k)))); tg = Rt.gens()
    if any(odd[i] for i in range(k)): return odd, None, None
    te = [sum(cf * prod(tg[i] ^ (m_.degree(Ry.gen(i)) // (2 if i < k else 1)) for i in range(4)) for cf, m_ in zip(e.coefficients(), e.monomials())) for e in ye]
    rd, tr = gdeg(Rt, te)
    return odd, rd, tr

u = K.random_element()
while True:
    yv = K.random_element(); r2 = yv + yv ^ (Q ^ 2)
    if r2 ^ Q != r2: break
zz = emb(Fq.random_element())
while zz == 0: zz = emb(Fq.random_element())
kinds = [("through0", K(0), True), ("c0=u*z", u * zz, True), ("r in F_Q^2\\F_Q", u * r2, True)] + \
        [("r generic #%d" % i, u * K.random_element(), False) for i in range(2)]
print("# kind | span dim | odd counts s1..s4 | dim U (pred) | ideal deg (cpu) | odd counts adapted y1..y4 | reduced deg (cpu) | ratio", flush=True)
for name, c0, dogb in kinds:
    c1 = CPU(); eqs = descend(c0, u)
    oc = [sum(1 for e in eqs for m_ in e.monomials() if m_.degree(sq[i]) % 2) for i in range(4)]
    Ub = Ubasis(eqs); sd = spandim(c0 / u) if c0 != 0 else 1
    line = f"{name} | {sd} | {oc} | {len(Ub)} (pred {4 - sd}) | build {CPU()-c1:.1f}s"
    if dogb:
        dg, tg_ = gdeg(Rq, eqs); line += f" | {dg} ({tg_:.1f}s)"
        if len(Ub) > 0:
            odd, rd, tr = reduced(eqs, Ub)
            line += f" | {odd} | {rd} ({tr:.1f}s) | {dg / rd if rd else '-'}"
    print(line, flush=True)

# (4) Lemma 1 exactness for m = 5, 6 on specialisations
for m in (5, 6):
    for trial in range(2):
        Rb = PolynomialRing(K, 'x1,x0,Y,W'); X1, X0, Yb, Wb = Rb.gens()
        cs = [K.random_element() for _ in range(m - 1)]          # x2..xm
        c1 = CPU()
        # S_{k+1}(x1, c2..ck, Y) built left to right, then close with x0
        Fm = S3(X1, Rb(cs[0]), Yb, b)                              # S3(x1, x2, Y): Y = x(P1+P2)
        for c in cs[1:]:
            Fm = Fm.subs({Yb: Wb}).resultant(S3(Wb, Rb(c), Yb, b), Wb)   # Y := x(prev + P_k)
        Fm = Fm.subs({Yb: X0})                                     # sum P1..Pm has x = x0
        Dm = X1 * Fm.derivative(X1) - X0 * Fm.derivative(X0)
        print(f"(4) m={m} trial {trial}: F(x1, c2..c{m}, x0) degrees ({Fm.degree(X1)},{Fm.degree(X0)}), "
              f"x1F1 - x0F0 == 0: {Dm == 0}; == 0 mod F: {Dm.quo_rem(Fm)[1] == 0 if Dm != 0 else True} (cpu {CPU()-c1:.1f}s)", flush=True)
print("DONE", flush=True)
