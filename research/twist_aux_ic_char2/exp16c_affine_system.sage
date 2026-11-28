import sys, time
# EXP16c item 3: does the Exp 15 squaring reduction (s1,s2,s3 only with even exponents) survive on an affine line
# x_i = c0 + u*w_i?  Run from twist_lift/.  usage: sage exp16c_affine_system.sage seed ntargets dofglm
# Lines on one curve y^2 + xy = x^3 + b (a = 0) over F_{2^20}, Q = 32:
#   A  = affine lift-dense line in the plane sqrt(b)F_{Q^2}: u0 = sqrt(b)/tau, tau = Tr_{K/F_{Q^2}}(sqrt b),
#        c0 = sqrt(b)*z (z in F_{Q^2}), Tr(c0) = Tr(a) = 0  -> all Q x-values lift, all in 2E
#   G  = generic affine line (random u, random c0)
#   D0 = the through-0 D line u0*F_Q (for the cost comparison with the Exp 15 reduction)
seed = int(sys.argv[1]); NT = int(sys.argv[2]); DOFGLM = sys.argv[3] == '1'
sys.argv = ['x', '5', '1', '1', str(seed), '0', '0', '0']
src = open('exp14_char2_solve.sage').read(); src = src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(seed)
CPU = time.process_time
FQall = [K(0)] + FQstar
W = TermOrder('wdegrevlex', (2, 2, 2, 1))
Rw2 = PolynomialRing(Fq, 't1,t2,t3,s4', order=W)
def reduce(e, RR):
    g = RR.gens(); return sum(c * g[0]^(m.exponents()[0][0]//2) * g[1]^(m.exponents()[0][1]//2) * g[2]^(m.exponents()[0][2]//2) * g[3]^m.exponents()[0][3] for c, m in zip(e.coefficients(), e.monomials()))

def system_aff(u, c0, b, xR):
    S4 = S3(c0 + u * w3, c0 + u * w4, Z, b).resultant(S3(Rw(xR), Y, Z, b), Z)
    S5 = S3(c0 + u * w1, c0 + u * w2, Y, b).resultant(S4, Y)
    g = symred(Rw(S5))
    eqs = [Rq(0)] * 4
    for c, mon in zip(g.coefficients(), g.monomials()):
        cc = coords(c); ex = mon.exponents()[0]
        m = prod(Rq.gens()[i] ^ ex[i] for i in range(4))
        for j in range(4): eqs[j] += cc[j] * m
    return eqs, g

def brute_aff(E, u, c0, R):
    pts = []
    for w in FQall:
        x = c0 + u * w
        if x != 0 and E.is_x_coord(x):
            P = E.lift_x(x); pts.append((w, [P, -P] if P != -P else [P]))
    sums = {}
    for i in range(len(pts)):
        for j in range(i, len(pts)):
            for A in pts[i][1]:
                for Bp in pts[j][1]: sums.setdefault(A + Bp, set()).add((i, j))
    found = set()
    for k in range(len(pts)):
        for l in range(k, len(pts)):
            for C in pts[k][1]:
                for D_ in pts[l][1]:
                    for T_ in (R - C - D_, -R - C - D_):
                        for (i, j) in sums.get(T_, ()): found.add(tuple(sorted((i, j, k, l))))
    return len(found), len(pts)

def parity(eqs):
    odd = [0, 0, 0, 0]; tot = 0
    for e in eqs:
        for m in e.monomials():
            ex = m.exponents()[0]; tot += 1
            for i in range(4):
                if ex[i] % 2: odd[i] += 1
    return odd, tot

X = PolynomialRing(Fq, 'X').gen()
def ratsols(eqs):
    G2 = Rlex.ideal([Rlex(g) for g in eqs] + [v ^ Q - v for v in Rlex.gens()]).groebner_basis('libsingular:stdfglm')
    return rat_points(list(G2))

def split_count(V, E, u, c0):
    n = 0
    for s in V:
        rts = (X^4 + s[0]*X^3 + s[1]*X^2 + s[2]*X + s[3]).roots()
        if sum(m for _, m in rts) == 4 and all(c0 + u*emb(r) != 0 and E.is_x_coord(c0 + u*emb(r)) for r, _ in rts): n += 1
    return n

while True:
    b = K.random_element(); sb = b.sqrt(); tau = sb + sb^(Q^2)
    if b != 0 and tau != 0: break
E = EllipticCurve(K, [1, 0, 0, 0, b])
u0 = sb / tau
assert Tr(u0) == 0
while True:
    y = K.random_element(); zz = y + y^(Q^2)
    c0 = sb * zz
    if (zz * tau)^Q != zz * tau and (c0).trace() == 0: break
nl = sum(1 for w in FQall if E.is_x_coord(c0 + u0 * w))
print(f"curve b, line A: u0 = sqrt(b)/tau, T(u0) = {Tr(u0)}, Tr(c0) = {c0.trace()}, lifting x-values {nl}/{Q}", flush=True)
uG = K.random_element(); cG = K.random_element()
for r in range(NT):
    Rt = 2 * E.random_point()
    for name, u, c in [("A", u0, c0), ("G", uG, cG)]:
        c1 = CPU(); eqs, g = system_aff(u, c, b, Rt[0]); tsys = CPU() - c1
        odd, tot = parity(eqs); oddK, totK = parity([g])
        c1 = CPU(); gb = Rq.ideal(eqs).groebner_basis('libsingular:std'); tgb = CPU() - c1
        dg = len(Rq.ideal(gb).normal_basis())
        # weighted order too (in case it helps the unreduced affine system)
        Rqw = PolynomialRing(Fq, 's1,s2,s3,s4', order=W)
        c1 = CPU(); gbw = Rqw.ideal([Rqw(e) for e in eqs]).groebner_basis('libsingular:std'); tgbw = CPU() - c1
        print(f"r{r} {name}: sys {tsys:.1f}s | monomials with odd exponent in s1,s2,s3,s4: K-poly {oddK}/{totK}, descended {odd}/{tot} | degrevlex GB cpu {tgb:.2f}s deg {dg} | wdegrevlex(2,2,2,1) GB cpu {tgbw:.2f}s", flush=True)
        if name == "A" or r == 0:
            c1 = CPU(); V = ratsols(gb); tfe = CPU() - c1
            sp = split_count(V, E, u, c)
            bf, npts = brute_aff(E, u, c, Rt)
            J = matrix(Rq, [[e.derivative(v) for v in Rq.gens()] for e in eqs])
            jd = [J.subs({Rq.gens()[i]: s[i] for i in range(4)}).change_ring(Fq).det() for s in V]
            print(f"     F_Q-solutions {len(V)} (fe route {tfe:.1f}s), split {sp}, brute {bf} (line has {npts} lifting x) | Jacobian det at F_Q-solutions nonzero: {sum(1 for x in jd if x != 0)}/{len(jd)}", flush=True)
        if DOFGLM and name == "A" and r == 0:
            c1 = CPU(); lx = Rlex.ideal([Rlex(e) for e in eqs]).groebner_basis('libsingular:stdfglm'); tl = CPU() - c1
            uv = [h for h in lx if h.variables() == (Rlex.gens()[3],)]
            if uv:
                h = uv[0].univariate_polynomial(); sq = h.degree() - h.gcd(h.derivative()).degree()
                print(f"     lex (stdfglm cpu {tl:.1f}s): len {len(lx)} s4-eliminant deg {h.degree()} squarefree-part deg {sq}", flush=True)
            else: print(f"     lex (stdfglm cpu {tl:.1f}s): len {len(lx)}, no s4-univariate", flush=True)
    # through-0 D line u0*F_Q on the same target: original and reduced system
    eqs = system(u0, b, Rt[0]); odd, tot = parity(eqs)
    c1 = CPU(); gb = Rq.ideal(eqs).groebner_basis('libsingular:std'); tgb = CPU() - c1
    red = [reduce(e, Rw2) for e in eqs]
    c1 = CPU(); gbr = Rw2.ideal(red).groebner_basis('libsingular:std'); tgr = CPU() - c1
    print(f"r{r} D0 (through 0, u0): odd-exponent monomials {odd}/{tot} | original degrevlex GB cpu {tgb:.2f}s deg {len(Rq.ideal(gb).normal_basis())} | reduced wdegrevlex GB cpu {tgr:.2f}s deg {len(Rw2.ideal(gbr).normal_basis())}", flush=True)
