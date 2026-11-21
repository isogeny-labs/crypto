# EXP16b (descent part): does the squaring redundancy survive Weil descent on lines through 0 (u*F_Q) and on
# affine lines c0 + u*F_Q?  Prediction (exp16b_proof.md): with r = c0/u, the F_Q-subspace U of directions along
# which every descended equation has zero derivative has dim U = m - dim_{F_Q} span{1, r, .., r^{m-1}}, and in
# coordinates adapted to U the system is a polynomial in the squares of the U-coordinates, so the ideal degree
# shrinks by 2^{dim U} and every point has multiplicity >= 2^{dim U}. m = n = 4 here:
#   r in F_Q (line through 0): dim U = 3 (factor 8);  r in F_{Q^2} \ F_Q: dim U = 2 (factor 4);  r generic: dim U = 0.
# Reuses exp14_char2_solve.sage (read-only, executed up to its main loop, as exp15 does). Run from twist_lift/.
# usage: sage exp16b_descent.sage ncurves seed
import sys, time
nc_, seed_ = int(sys.argv[1]), int(sys.argv[2])
sys.argv = ['x', '5', '1', '1', str(seed_), '0', '0', '0']
src = open('exp14_char2_solve.sage').read(); src = src[:src.index('print("d Q curve')]
exec(preparse(src))
CPU = time.process_time
set_random_seed(seed_)

def system_aff(c0, u, b, xR):
    """exp14 system() with x_i = c0 + u*w_i (c0 = 0 gives exp14's system exactly)."""
    X = [c0 + u * w for w in (w1, w2, w3, w4)]
    S4 = S3(X[2], X[3], Z, b).resultant(S3(Rw(xR), Y, Z, b), Z)
    S5 = S3(X[0], X[1], Y, b).resultant(S4, Y)
    g = symred(Rw(S5))
    eqs = [Rq(0)] * 4
    for c, mon in zip(g.coefficients(), g.monomials()):
        cc = coords(c); ex = mon.exponents()[0]
        m_ = prod(Rq.gens()[i] ^ ex[i] for i in range(4))
        for j in range(4): eqs[j] += cc[j] * m_
    return eqs

def odd_counts(eqs, R_):
    oc = [0] * 4
    for e in eqs:
        for mon in e.monomials():
            ex = mon.exponents()[0]
            for i in range(4):
                if ex[i] % 2: oc[i] += 1
    return oc

def null_directions(eqs):
    """F_Q-basis of U = {delta : sum_l delta_l dP_j/ds_l = 0 for all j}."""
    rows = {}
    for j, e in enumerate(eqs):
        for l in range(4):
            dl = e.derivative(Rq.gens()[l])
            for c, mon in zip(dl.coefficients(), dl.monomials()):
                rows.setdefault((j, mon), [Fq(0)] * 4)[l] += c
    if not rows: return []
    Mx = matrix(Fq, list(rows.values()))
    return Mx.right_kernel().basis()

def span_dim(r):
    vs = [vector(Fq, coords(r ^ k)) for k in range(4)]
    return matrix(Fq, vs).rank()

def adapted(eqs, Ubasis):
    """Substitute s = M y with the first k columns of M spanning U; return the new equations in y."""
    k = len(Ubasis)
    cols = list(Ubasis)
    for v in identity_matrix(Fq, 4).columns():
        if matrix(Fq, cols + [v]).rank() > len(cols): cols.append(v)
        if len(cols) == 4: break
    M = matrix(Fq, cols).transpose()
    Ry = PolynomialRing(Fq, 'y1,y2,y3,y4', order='degrevlex'); y = vector(Ry.gens())
    sub = M * y
    return Ry, [e(*sub) for e in eqs], k

def squared(eqs, Ry, k):
    """y_i -> t_i for i <= k (requires even exponents)."""
    W_ = TermOrder('wdegrevlex', tuple([2] * k + [1] * (4 - k)))
    Rt = PolynomialRing(Fq, 'y1,y2,y3,y4', order=W_); g = Rt.gens()
    out = []
    for e in eqs:
        f = Rt(0)
        for c, mon in zip(e.coefficients(), e.monomials()):
            ex = mon.exponents()[0]
            f += c * prod(g[i] ^ (ex[i] // 2 if i < k else ex[i]) for i in range(4))
        out.append(f)
    return Rt, out

def gbdeg(R_, eqs):
    c0 = CPU(); gb = R_.ideal(eqs).groebner_basis('libsingular:std'); t = CPU() - c0
    return len(R_.ideal(gb).normal_basis()), t, gb

print("# exp16b_descent: d=5 (Q=32), m=n=4, curve y^2+xy=x^3+b, target 2*random point", flush=True)
print("# kind | dim span{1,r,r^2,r^3} | odd-exponent monomial counts (s1..s4) over the 4 eqs | dim U (measured; predicted 4-span)"
      " | ideal deg (cpu) | adapted: odd counts in y1..y4 | reduced deg (cpu) | ratio", flush=True)
done = 0
while done < nc_:
    b = K.random_element()
    if b == 0: continue
    E = EllipticCurve(K, [1, 0, 0, 0, b]); done += 1
    u = K.random_element()
    while True:
        yv = K.random_element(); r2 = yv + yv ^ (Q ^ 2)           # in F_{Q^2}
        if r2 ^ Q != r2: break                                    # not in F_Q
    rg = K.random_element()
    kinds = [("through0 (c0=0)", K(0)), ("c0=u*z, z in F_Q*", u * emb(z) ^ 3), ("c0=u*r, r in F_Q^2\\F_Q", u * r2), ("c0 generic", u * rg)]
    Rt_ = 2 * E.random_point()
    for name, c0 in kinds:
        r = c0 / u
        eqs = system_aff(c0, u, b, Rt_[0])
        oc = odd_counts(eqs, Rq)
        Ub = null_directions(eqs)
        sd = span_dim(r) if r != 0 else 1
        dg, tg, gb = gbdeg(Rq, eqs)
        line = f"c{done} {name} | {sd} | {oc} | {len(Ub)} (pred {4 - sd}) | {dg} ({tg:.2f}s)"
        if len(Ub) > 0:
            Ry, ye, k = adapted(eqs, Ub)
            ocy = odd_counts(ye, Ry)
            if all(ocy[i] == 0 for i in range(k)):
                Rt, te = squared(ye, Ry, k)
                rd, tr, _ = gbdeg(Rt, te)
                line += f" | {ocy} | {rd} ({tr:.2f}s) | {dg / rd:.2f}"
            else:
                line += f" | {ocy} | not even in U-coordinates | -"
        else:
            # all points simple?  I + (det J) = (1) <=> every point of V(I) has invertible Jacobian
            c1 = CPU()
            J = matrix(Rq, [[e.derivative(v) for v in Rq.gens()] for e in eqs]); dJ = J.det()
            gbJ = Rq.ideal(list(gb) + [dJ]).groebner_basis('libsingular:std')
            simple = (len(gbJ) == 1 and gbJ[0].is_unit())
            dsing = 0 if simple else len(Rq.ideal(gbJ).normal_basis())
            line += f" | - | radical test: I+(detJ) = (1)? {simple} (deg of I+(detJ) {dsing}; cpu {CPU()-c1:.1f}s) | -"
        print(line, flush=True)
print("DONE", flush=True)
