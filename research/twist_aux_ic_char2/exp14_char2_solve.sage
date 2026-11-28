# EXP14: per-trial solver cost in char 2, by line type (lift-dense D, pinned P1/P2, random R).
# E: y^2 + xy = x^3 + b over K = F_{2^N}, N = 4d, Q = 2^d; factor base x = u*w, w in F_Q*.
# Decomposition of R (in 2E): S5(u w1, .., u w4, x_R) = 0, symmetrised in w (s1..s4), Weil-descended over F_Q
# to 4 equations in 4 unknowns over F_Q. Solver = libSingular (in-process, via Sage): degrevlex std; std+FGLM to lex
# (reported as fglm_s = that time minus the std time); then the
# F_Q-rational points. Validated against a brute-force multiset count when dobf = 1.
# usage: sage exp14_char2_solve.sage d ncurves targets_per_line seed dobf [dofglm [dofe]]
import sys, time
d, ncurves, NT, seed, dobf = [int(a) for a in sys.argv[1:6]]
DOFGLM = len(sys.argv) > 6 and sys.argv[6] == '1'   # full FGLM of the degree-4096 ideal (Singular: ~10-15 min each)
DOFE = not (len(sys.argv) > 7 and sys.argv[7] == '0')  # 0 = skip the field-equation rational-point route
CPU = time.process_time
set_random_seed(seed)
N = 4 * d; Q = 2 ^ d
K = GF(2 ^ N, 't'); t = K.gen()
Fq, emb = K.subfield(d, name='z', map=True)
z = Fq.gen()
Tr = lambda c: c + c ^ Q + c ^ (Q ^ 2) + c ^ (Q ^ 3)
# coordinates of K over F_Q in the basis t^0..t^3, via an F_2 basis {emb(z^i) t^j}
B = [emb(z ^ i) * t ^ j for j in range(4) for i in range(d)]
M = matrix(GF(2), [vector(GF(2), c._vector_()) for c in B]).transpose()
Minv = M.inverse()
def coords(c):
    v = Minv * vector(GF(2), c._vector_())
    return [sum(Fq(int(v[j * d + i])) * z ^ i for i in range(d)) for j in range(4)]
FQstar = [emb(z) ^ i for i in range(Q - 1)]

Rw = PolynomialRing(K, 'w1,w2,w3,w4,Y,Z'); w1, w2, w3, w4, Y, Z = Rw.gens()
Rs = PolynomialRing(K, 's1,s2,s3,s4'); S = Rs.gens()
Rq = PolynomialRing(Fq, 's1,s2,s3,s4', order='degrevlex')
Rlex = PolynomialRing(Fq, 's1,s2,s3,s4', order='lex')
def S3(a, b_, c, bb): return (a * b_ + a * c + b_ * c) ^ 2 + a * b_ * c + bb

def symred(f):
    """f in K[w1..w4] symmetric -> g in K[s1..s4] with g(e1..e4) = f."""
    W = [w1, w2, w3, w4]
    e = [W[0] + W[1] + W[2] + W[3], W[0]*W[1] + W[0]*W[2] + W[0]*W[3] + W[1]*W[2] + W[1]*W[3] + W[2]*W[3],
         W[0]*W[1]*W[2] + W[0]*W[1]*W[3] + W[0]*W[2]*W[3] + W[1]*W[2]*W[3], W[0]*W[1]*W[2]*W[3]]
    g = Rs(0); f = f
    while f != 0:
        lm = f.lm(); c = f.lc(); a = lm.exponents()[0][:4]
        k = [a[0] - a[1], a[1] - a[2], a[2] - a[3], a[3]]
        if min(k) < 0: raise ValueError("not symmetric")
        g += c * prod(S[i] ^ k[i] for i in range(4))
        f -= c * prod(e[i] ^ k[i] for i in range(4))
    return g

def system(u, b, xR):
    # S5(x1..x4, xR) = Res_Y(S3(x1,x2,Y), S4(x3,x4,xR,Y)), S4(x3,x4,xR,Y) = Res_Z(S3(x3,x4,Z), S3(xR,Y,Z))
    S4 = S3(u * w3, u * w4, Z, b).resultant(S3(Rw(xR), Y, Z, b), Z)
    S5 = S3(u * w1, u * w2, Y, b).resultant(S4, Y)
    f = Rw(S5)
    g = symred(f)
    eqs = [Rq(0)] * 4
    for c, mon in zip(g.coefficients(), g.monomials()):
        cc = coords(c); ex = mon.exponents()[0]
        m = prod(Rq.gens()[i] ^ ex[i] for i in range(4))
        for j in range(4): eqs[j] += cc[j] * m
    return eqs

def brute(E, u, R):
    pts = []
    for w in FQstar:
        x = u * w
        if E.is_x_coord(x):
            P = E.lift_x(x); pts.append((w, [P, -P] if P != -P else [P]))
    sums = {}
    for i in range(len(pts)):
        for j in range(i, len(pts)):
            for A in pts[i][1]:
                for Bp in pts[j][1]:
                    sums.setdefault(A + Bp, set()).add((i, j))
    found = set()
    for k in range(len(pts)):
        for l in range(k, len(pts)):
            for C in pts[k][1]:
                for D in pts[l][1]:
                    for T in (R - C - D, -R - C - D):
                        for (i, j) in sums.get(T, ()):
                            found.add(tuple(sorted((i, j, k, l))))
    return len(found)

def rat_points(G):
    """F_Q-points of a zero-dim lex GB in s1 > s2 > s3 > s4 (radical, field equations included): back-substitution."""
    gens = Rlex.gens(); sols = [[]]
    for k in range(3, -1, -1):       # solve for s4, then s3, ...
        new = []
        for part in sols:            # part = values of s_{k+1}..s_4
            sub = {gens[k + 1 + i]: part[i] for i in range(len(part))}
            polys = [g.subs(sub) for g in G if min([i for i in range(4) if g.degree(gens[i]) > 0], default=-1) == k]
            polys = [p for p in polys if p != 0]
            if not polys: continue
            X = PolynomialRing(Fq, 'X').gen()
            h = gcd([p.univariate_polynomial().change_ring(Fq).subs(X) if p.variables() else Fq(p) + 0 * X for p in polys])
            if h == 0: continue
            for r, _ in h.roots(): new.append([r] + part)
        sols = new
    # keep only points that satisfy all of G
    return [p for p in sols if all(g(*p) == 0 for g in G)]

def solve(eqs, u, E, R):
    I = Rq.ideal(eqs)
    c0 = CPU(); t0 = time.time(); gb = I.groebner_basis('libsingular:std'); tgb = time.time() - t0; cgb = CPU() - c0
    Ig = Rq.ideal(gb); deg = len(Ig.normal_basis())
    tfg = -1.0; shape = None
    if DOFGLM:
        c0 = CPU(); t0 = time.time(); lexgb = Rlex.ideal([Rlex(g) for g in eqs]).groebner_basis('libsingular:stdfglm'); tfg = time.time() - t0 - tgb; cfg = CPU() - c0 - cgb
        print(f'   cpu: gb {cgb:.2f} fglm {cfg:.2f} lexgb_len {len(lexgb)}', flush=True)
        for gi in lexgb: print(f'      lex elt vars {gi.variables()} deg {gi.degree()}', flush=True)
        uv = [gi for gi in lexgb if gi.variables() == (Rlex.gens()[3],)]
        if uv:
            h = uv[0].univariate_polynomial(); sf = h.gcd(h.derivative())
            print(f'      s4-univariate deg {h.degree()} sqfree-part deg {h.degree() - sf.degree()} (ideal deg {deg})', flush=True)
        s4 = Rlex.gens()[3]
        shape = len(lexgb) == 4 and lexgb[-1].variables() == (s4,) and lexgb[-1].degree() == deg
    # F_Q-rational points: degrevlex GB plus field equations (toy-Q route), then lex via FGLM on a small ideal
    if not DOFE: return tgb, deg, tfg, shape, -1, -1.0, -1
    t0 = time.time()
    G2 = Rlex.ideal([Rlex(g) for g in gb] + [v ^ Q - v for v in Rlex.gens()]).groebner_basis('libsingular:stdfglm')
    V = rat_points(list(G2))
    tsol = time.time() - t0
    split = 0
    X = PolynomialRing(Fq, 'X').gen()
    for sol in V:
        s = sol
        p = X ^ 4 + s[0] * X ^ 3 + s[1] * X ^ 2 + s[2] * X + s[3]
        rts = p.roots()
        if sum(m for _, m in rts) == 4 and all(r != 0 for r, _ in rts):
            ws = [emb(r) for r, m in rts for _ in range(m)]
            if all(E.is_x_coord(u * w) for w in ws): split += 1
    return tgb, deg, tfg, shape, len(V), tsol, split

print("d Q curve type target | gb_s deg fglm_s(-1 = skipped) shape | Fq_sols fe_s split | brute", flush=True)
done = 0
while done < ncurves:
    b = K.random_element()
    if b == 0: continue
    E = EllipticCurve(K, [1, 0, 0, 0, b]); sb = b.sqrt()
    D = P1 = P2 = None
    for _ in range(20000):
        zz = K.random_element(); u = zz ^ Q + zz
        if u == 0: continue
        if D is None and Tr(sb / u) == 0 and (sb / u ^ 2) ^ Q != sb / u ^ 2: D = u
        if P1 is None and Tr(sb / u) != 0: P1 = u
        if D is not None and P1 is not None: break
    if D is None: continue
    for _ in range(20000):
        u = K.random_element()
        if u != 0 and Tr(u) != 0 and Tr(sb / u) == 0: P2 = u; break
    done += 1
    lines = [("D", D), ("P1", P1), ("P2", P2), ("R", K.random_element())]
    for r in range(NT):
        Rt = 2 * E.random_point()
        if Rt.is_zero(): continue
        for name, u in lines:
            if u is None: continue
            t0 = time.time(); eqs = system(u, b, Rt[0]); tsys = time.time() - t0
            res = solve(eqs, u, E, Rt)
            bf = brute(E, u, Rt) if dobf else -1
            print(f"d={d} Q={Q} c{done} {name} r{r} | {res[0]:.2f} {res[1]} {res[2]:.2f} {res[3]} | {res[4]} {res[5]:.2f} {res[6]} | {bf}  (sys {tsys:.1f}s)", flush=True)
