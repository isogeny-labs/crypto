# EXP16d-2: m = n = 3 in char 2. E: y^2+xy = x^3+b over K = F_{2^{3d}}, Q = 2^d, factor base x = u*w, w in F_Q*.
# S4(x1,x2,x3,xR) = Res_Y(S3(x1,x2,Y), S3(x3,xR,Y)), symmetrised in s1..s3, Weil-descended to 3 eqs over F_Q.
# Measures: even-exponent property of s1,s2 in all descended equations; original ideal degree (degrevlex std);
# reduced (t_i = s_i^2, i=1,2) ideal degree (wdegrevlex(2,2,1)); lex shape of reduced; decomposition counts from
# the reduced system (field equations, sqrt, split) vs brute force. Targets: half planted (sum of 3 FB points), half random 2E.
# usage: sage exp16d_m3.sage d ncurves targets seed
import sys, time
d, ncurves, NT, seed = [int(a) for a in sys.argv[1:5]]
CPU = time.process_time
set_random_seed(seed)
n = 3; N = n*d; Q = 2^d
K = GF(2^N, 't'); t = K.gen()
Fq, emb = K.subfield(d, name='z', map=True); z = Fq.gen()
Tr = lambda c: sum(c^(Q^i) for i in range(n))
B = [emb(z^i)*t^j for j in range(n) for i in range(d)]
M = matrix(GF(2), [vector(GF(2), c._vector_()) for c in B]).transpose(); Minv = M.inverse()
def coords(c):
    v = Minv*vector(GF(2), c._vector_())
    return [sum(Fq(int(v[j*d+i]))*z^i for i in range(d)) for j in range(n)]
FQstar = [emb(z)^i for i in range(Q-1)]
Rw = PolynomialRing(K, 'w1,w2,w3,Y'); w1,w2,w3,Y = Rw.gens()
Rs = PolynomialRing(K, 's1,s2,s3'); S = Rs.gens()
Rq = PolynomialRing(Fq, 's1,s2,s3', order='degrevlex')
W = TermOrder('wdegrevlex', (2,2,1))
Rr = PolynomialRing(Fq, 't1,t2,s3', order=W)
Rrl = PolynomialRing(Fq, 't1,t2,s3', order='lex')
def S3(a, b_, c, bb): return (a*b_ + a*c + b_*c)^2 + a*b_*c + bb
E_ = [w1+w2+w3, w1*w2+w1*w3+w2*w3, w1*w2*w3]
def symred(f):
    g = Rs(0)
    while f != 0:
        lm = f.lm(); c = f.lc(); a = lm.exponents()[0][:3]
        k = [a[0]-a[1], a[1]-a[2], a[2]]
        if min(k) < 0: raise ValueError("not symmetric")
        g += c*prod(S[i]^k[i] for i in range(3)); f -= c*prod(E_[i]^k[i] for i in range(3))
    return g
def system(u, b, xR):
    f = S3(u*w1, u*w2, Y, b).resultant(S3(u*w3, Rw(xR), Y, b), Y)
    g = symred(Rw(f))
    eqs = [Rq(0)]*n
    for c, mon in zip(g.coefficients(), g.monomials()):
        cc = coords(c); ex = mon.exponents()[0]; mm = prod(Rq.gens()[i]^ex[i] for i in range(3))
        for j in range(n): eqs[j] += cc[j]*mm
    return eqs
def odd_in(eqs):
    return sum(1 for e in eqs for ex in e.exponents() if ex[0] % 2 or ex[1] % 2)
def reduce(e, RR):
    g = RR.gens()
    return sum(c*g[0]^(m.exponents()[0][0]//2)*g[1]^(m.exponents()[0][1]//2)*g[2]^m.exponents()[0][2] for c, m in zip(e.coefficients(), e.monomials()))
X = PolynomialRing(Fq, 'X').gen()
def ratpts(G, gens):
    sols = [[]]
    for k in range(2, -1, -1):
        new = []
        for part in sols:
            sub = {gens[k+1+i]: part[i] for i in range(len(part))}
            polys = [g.subs(sub) for g in G if min([i for i in range(3) if g.degree(gens[i]) > 0], default=-1) == k]
            polys = [p for p in polys if p != 0]
            if not polys: continue
            h = gcd([p.univariate_polynomial().change_ring(Fq).subs(X) if p.variables() else Fq(p)+0*X for p in polys])
            if h == 0: continue
            for r, _ in h.roots(): new.append([r]+part)
        sols = new
    return [p for p in sols if all(g(*p) == 0 for g in G)]
def lifts(E, u):
    pts = []
    for w in FQstar:
        x = u*w
        if E.is_x_coord(x):
            P = E.lift_x(x); pts.append((w, [P, -P] if P != -P else [P]))
    return pts
def brute(pts, R):
    sums = {}
    for i in range(len(pts)):
        for j in range(i, len(pts)):
            for A in pts[i][1]:
                for Bp in pts[j][1]: sums.setdefault(A+Bp, set()).add((i, j))
    found = set()
    for k in range(len(pts)):
        for C in pts[k][1]:
            for T in (R-C, -R-C):
                for (i, j) in sums.get(T, ()): found.add(tuple(sorted((i, j, k))))
    return len(found)
print("d Q curve type target | odd-s1s2-terms | orig deg gb_cpu | red deg gb_cpu(wdeg) | lex len s3-elim deg sqfree | Fq sols split | brute", flush=True)
done = 0
while done < ncurves:
    b = K.random_element()
    if b == 0: continue
    E = EllipticCurve(K, [1, 0, 0, 0, b]); sb = b.sqrt()
    D = P1 = None
    for _ in range(20000):
        zz = K.random_element(); u = zz^Q + zz
        if u == 0: continue
        if D is None and Tr(sb/u) == 0 and (sb/u^2)^Q != sb/u^2: D = u
        if P1 is None and Tr(sb/u) != 0: P1 = u
        if D is not None and P1 is not None: break
    if D is None: continue
    done += 1
    for name, u in [("D", D), ("P1", P1), ("R", K.random_element())]:
        pts = lifts(E, u)
        for r in range(NT):
            if r % 2 == 0 and len(pts) >= 3:
                ii = [randint(0, len(pts)-1) for _ in range(3)]
                Rt = sum(pts[i][1][0] for i in ii); tag = 'planted'
            else:
                Rt = 2*E.random_point(); tag = 'random'
            if Rt.is_zero(): continue
            eqs = system(u, b, Rt[0])
            nodd = odd_in(eqs)
            c0 = CPU(); gb = Rq.ideal(eqs).groebner_basis('libsingular:std'); tg = CPU()-c0
            dg = len(Rq.ideal(gb).normal_basis())
            red = [reduce(e, Rr) for e in eqs]
            c0 = CPU(); gbr = Rr.ideal(red).groebner_basis('libsingular:std'); tr = CPU()-c0
            dr = len(Rr.ideal(gbr).normal_basis())
            lx = list(Rrl.ideal([Rrl(str(f)) for f in red]).groebner_basis('libsingular:stdfglm'))
            uv = [g for g in lx if g.variables() == (Rrl.gens()[2],)]
            if uv:
                h = uv[0].univariate_polynomial(); ld = h.degree(); sqf = h.gcd(h.derivative()).degree() == 0
            else: ld = -1; sqf = None
            G = list(Rrl.ideal([Rrl(str(f)) for f in red] + [v^Q - v for v in Rrl.gens()]).groebner_basis('libsingular:stdfglm'))
            V = ratpts(G, Rrl.gens())
            split = 0
            for P in V:
                s = [P[0].sqrt(), P[1].sqrt(), P[2]]
                rts = (X^3 + s[0]*X^2 + s[1]*X + s[2]).roots()
                if sum(mu for _, mu in rts) == 3 and all(x != 0 for x, _ in rts) and all(E.is_x_coord(u*emb(x)) for x, _ in rts): split += 1
            bf = brute(pts, Rt)
            print(f"d={d} Q={Q} c{done} {name} r{r}({tag}) | {nodd} | {dg} {tg:.3f} | {dr} {tr:.3f} | {len(lx)} {ld} {sqf} | {len(V)} {split} | {bf} {'OK' if split == bf else 'MISMATCH'}", flush=True)
