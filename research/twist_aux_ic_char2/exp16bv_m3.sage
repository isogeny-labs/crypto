# EXP16bv: m = n = 3 analogue of exp16b Cor. 3 (cheap enough to finish the generic affine line):
# K = F_{Q^3}, factor base x = c0 + u*w, w in F_Q, S_4(x1,x2,x3,x_R) symmetrised, descended to 3 eqs in s1..s3.
# Prediction: dim U = 3 - span dim{1,r,r^2}; ratio 2^{dim U}; generic r: dim U = 0, test whether all points are simple
# via I + (det J) == (1).
# usage: sage exp16bv_m3.sage d seed ncurves
import sys, time
proof.all(False)
CPU = time.process_time
d, seed_, nc = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]); set_random_seed(seed_)
Q = 2 ^ d; N = 3 * d; n = 3
K = GF(2 ^ N, 'g'); g = K.gen()
Fq, emb = K.subfield(d, name='z', map=True); z = Fq.gen()
Bas = [emb(z ^ i) * g ^ j for j in range(n) for i in range(d)]
Mbi = matrix(GF(2), [vector(GF(2), c._vector_()) for c in Bas]).transpose().inverse()
def coords(c):
    v = Mbi * vector(GF(2), c._vector_())
    return [sum(Fq(int(v[j * d + i])) * z ^ i for i in range(d)) for j in range(n)]
def S3(a_, b_, c_, bb): return (a_ * b_ + a_ * c_ + b_ * c_) ^ 2 + a_ * b_ * c_ + bb
Rx = PolynomialRing(K, 'x1,x2,x3,x0,Y'); x1, x2, x3, x0, Y = Rx.gens()
Rsym = PolynomialRing(K, 'x1,x2,x3,E1,E2,E3', order=TermOrder('degrevlex', 3) + TermOrder('degrevlex', 3))
X = Rsym.gens()[:3]; EE = Rsym.gens()[3:]
Isym = Rsym.ideal([EE[k] - sum(prod(c) for c in Combinations(X, k + 1)) for k in range(3)])
Rq = PolynomialRing(Fq, 's1,s2,s3', order='degrevlex'); sq = Rq.gens()
RsK = PolynomialRing(K, 's1,s2,s3'); sK = RsK.gens()
def ekf(c0, u, ss):
    s_ = [1] + list(ss)
    return [sum(K(binomial(n - l, k - l)) * c0 ^ (k - l) * u ^ l * s_[l] for l in range(k + 1)) for k in range(1, n + 1)]
print(f"# exp16bv_m3: d={d} (Q={Q}), K=GF(2^{N}), m=n=3, seed {seed_}", flush=True)
print("# curve kind | span dim | odd counts s1..s3 | dim U (pred) | deg (cpu) | adapted odd | reduced deg | ratio | simple-points test", flush=True)
for ci in range(nc):
    b = K.random_element()
    while b == 0: b = K.random_element()
    E = EllipticCurve(K, [1, K.random_element(), 0, 0, b])
    xR = (2 * E.random_point())[0]
    F = S3(x1, x2, Y, b).resultant(S3(x3, x0, Y, b), Y).subs({x0: xR})
    Fx = sum(cf * prod(X[i] ^ mon.degree(Rx.gen(i)) for i in range(3)) for cf, mon in zip(F.coefficients(), F.monomials()))
    G = Isym.reduce(Rsym(Fx))
    assert all(mm.degree(X[i]) == 0 for mm in G.monomials() for i in range(3))
    oddE = [sum(1 for mm in G.monomials() if mm.degree(EE[k]) % 2) for k in range(3)]
    print(f"c{ci} G terms {len(G.monomials())}, odd counts E1..E3 {oddE}", flush=True)
    u = K.random_element()
    rg = [K.random_element() for _ in range(3)]
    kinds = [("through0", K(0)), ("c0=u*z", u * emb(z) ^ 5)] + [("generic #%d" % i, u * r) for i, r in enumerate(rg)]
    for name, c0 in kinds:
        c1 = CPU()
        P = sum(cf * prod(e_ ^ mon.degree(EE[k]) for k, e_ in enumerate(ekf(c0, u, sK))) for cf, mon in zip(G.coefficients(), G.monomials()))
        eqs = [Rq(0)] * n
        for cf, mon in zip(P.coefficients(), P.monomials()):
            cc = coords(cf); mm = prod(sq[i] ^ mon.degree(sK[i]) for i in range(n))
            for j in range(n): eqs[j] += cc[j] * mm
        oc = [sum(1 for e in eqs for mm in e.monomials() if mm.degree(sq[i]) % 2) for i in range(n)]
        rows = []
        for e in eqs:
            dv = [e.derivative(v) for v in sq]
            for mm in set(x for dd in dv for x in dd.monomials()): rows.append([dd.monomial_coefficient(mm) for dd in dv])
        Ub = matrix(Fq, rows).right_kernel().basis()
        r = c0 / u; sd = matrix(Fq, [coords(r ^ k) for k in range(n)]).rank() if r != 0 else 1
        I = Rq.ideal(eqs); gb = I.groebner_basis('libsingular:std'); dg = len(Rq.ideal(gb).normal_basis())
        line = f"c{ci} {name} | {sd} | {oc} | {len(Ub)} (pred {n - sd}) | {dg} ({CPU()-c1:.1f}s)"
        if len(Ub) > 0:
            k = len(Ub); cols = list(Ub)
            for v in identity_matrix(Fq, n).columns():
                if len(cols) < n and matrix(Fq, cols + [v]).rank() > len(cols): cols.append(v)
            Mm = matrix(Fq, cols).transpose()
            Ry = PolynomialRing(Fq, 'y1,y2,y3'); yy = Mm * vector(Ry.gens()); ye = [e(*yy) for e in eqs]
            odd = [sum(1 for e in ye for mm in e.monomials() if mm.degree(Ry.gen(i)) % 2) for i in range(n)]
            Rt = PolynomialRing(Fq, 'y1,y2,y3', order=TermOrder('wdegrevlex', tuple([2] * k + [1] * (n - k)))); tg = Rt.gens()
            te = [sum(cf * prod(tg[i] ^ (mm.degree(Ry.gen(i)) // (2 if i < k else 1)) for i in range(n)) for cf, mm in zip(e.coefficients(), e.monomials())) for e in ye]
            rd = len(Rt.ideal(Rt.ideal(te).groebner_basis('libsingular:std')).normal_basis())
            line += f" | {odd} | {rd} | {dg / rd}"
        else:
            J = matrix(Rq, [[e.derivative(v) for v in sq] for e in eqs]).det()
            gbJ = Rq.ideal(list(gb) + [J]).groebner_basis('libsingular:std')
            simple = (len(gbJ) == 1 and gbJ[0].is_unit())
            line += f" | - | - | - | I+(detJ)=(1): {simple}" + ("" if simple else f" (deg of I+(detJ): {len(Rq.ideal(gbJ).normal_basis())})")
        print(line, flush=True)
print("DONE", flush=True)
