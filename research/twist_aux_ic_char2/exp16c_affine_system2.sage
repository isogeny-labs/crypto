import sys, time
# EXP16c item 3 (resumed, trimmed): same-session cost comparison A (affine lift-dense line in sqrt(b)F_{Q^2})
# vs D0 (through-0 line u0*F_Q, original and Exp-15-reduced system), plus the proof check g_A = F o phi,
# where F(e1..e4) = S5 in the elementary symmetric functions of x, phi = e(c0 + u0*w) in terms of s = e(w).
# No ratsols / FGLM on A (degree 4096).  Run from twist_lift/.
# usage: sage exp16c_affine_system2.sage seed ntargets nbrute      (e.g. 11 2 40)
# NOT RUN in the exp16c session: the DOT_SAGE/tmp volume hit ENOSPC before launch.
seed = int(sys.argv[1]); NT = int(sys.argv[2]); NB = int(sys.argv[3])
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
def descend(g):
    eqs = [Rq(0)] * 4
    for c, mon in zip(g.coefficients(), g.monomials()):
        cc = coords(c); ex = mon.exponents()[0]
        m = prod(Rq.gens()[i] ^ ex[i] for i in range(4))
        for j in range(4): eqs[j] += cc[j] * m
    return eqs
def kpoly(u, c0, b, xR):
    S4 = S3(c0 + u * w3, c0 + u * w4, Z, b).resultant(S3(Rw(xR), Y, Z, b), Z)
    S5 = S3(c0 + u * w1, c0 + u * w2, Y, b).resultant(S4, Y)
    return symred(Rw(S5))
def parity(eqs):
    odd = [0, 0, 0, 0]; tot = 0
    for e in eqs:
        for m in e.monomials():
            ex = m.exponents()[0]; tot += 1
            for i in range(4):
                if ex[i] % 2: odd[i] += 1
    return odd, tot
def lifts(E, u, c0):
    return [c0 + u * w for w in FQall if c0 + u * w != 0 and E.is_x_coord(c0 + u * w)]
def brute_pts(E, xs, R):
    pts = [[E.lift_x(x), -E.lift_x(x)] for x in xs]
    sums = {}
    for i in range(len(pts)):
        for j in range(i, len(pts)):
            for A in pts[i]:
                for Bp in pts[j]: sums.setdefault(A + Bp, set()).add((i, j))
    found = set()
    for k in range(len(pts)):
        for l in range(k, len(pts)):
            for C in pts[k]:
                for D_ in pts[l]:
                    for T_ in (R - C - D_, -R - C - D_):
                        for (i, j) in sums.get(T_, ()): found.add(tuple(sorted((i, j, k, l))))
    return len(found)

while True:
    b = K.random_element(); sb = b.sqrt(); tau = sb + sb^(Q^2)
    if b != 0 and tau != 0: break
E = EllipticCurve(K, [1, 0, 0, 0, b])
u0 = sb / tau
while True:
    y = K.random_element(); zz = y + y^(Q^2); c0 = sb * zz
    if (zz * tau)^Q != zz * tau and c0.trace() == 0: break
xA = lifts(E, u0, c0); xD = lifts(E, u0, K(0))
print(f"#E={E.order()} Tr(a)=0 | A: c0/u0 in F_Q^2 \\ F_Q: {(c0/u0)^(Q^2)==c0/u0 and (c0/u0)^Q!=c0/u0}, lifting {len(xA)}/{Q}, all Tr(x)=0: {all(x.trace()==0 for x in xA)} | D0: lifting {len(xD)}/{Q-1}", flush=True)
s1, s2, s3, s4 = Rs.gens()
phi = [u0*s1, u0^2*s2 + c0*u0*s1, u0^3*s3 + c0^2*u0*s1, u0^4*s4 + c0*u0^3*s3 + c0^2*u0^2*s2 + c0^3*u0*s1 + c0^4]
for r in range(NT):
    Rt = 2 * E.random_point()
    c1 = CPU(); gD = kpoly(u0, K(0), b, Rt[0]); eD = descend(gD); tD = CPU() - c1
    c1 = CPU(); gA = kpoly(u0, c0, b, Rt[0]); eA = descend(gA); tA = CPU() - c1
    # F(e) = gD(e1/u0, e2/u0^2, e3/u0^3, e4/u0^4); check gA == F(phi)
    F = gD(s1/u0, s2/u0^2, s3/u0^3, s4/u0^4)
    ok = (F(*phi) == gA)
    print(f"r{r}: build D0 {tD:.1f}s A {tA:.1f}s | odd-exponent monomials (s1,s2,s3,s4): F(e) {parity([F])[0]}/{parity([F])[1]}, D0 descended {parity(eD)[0]}/{parity(eD)[1]}, A K-poly {parity([gA])[0]}/{parity([gA])[1]}, A descended {parity(eA)[0]}/{parity(eA)[1]} | gA == F(phi(s)): {ok}", flush=True)
    c1 = CPU(); gb = Rq.ideal(eD).groebner_basis('libsingular:std'); t1 = CPU() - c1; d1 = len(Rq.ideal(gb).normal_basis())
    red = [reduce(e, Rw2) for e in eD]
    c1 = CPU(); gbr = Rw2.ideal(red).groebner_basis('libsingular:std'); t2 = CPU() - c1; d2 = len(Rw2.ideal(gbr).normal_basis())
    print(f"   D0 original degrevlex GB cpu {t1:.2f}s deg {d1} | D0 reduced wdegrevlex GB cpu {t2:.2f}s deg {d2}", flush=True)
    c1 = CPU(); gbA = Rq.ideal(eA).groebner_basis('libsingular:std'); t3 = CPU() - c1; d3 = len(Rq.ideal(gbA).normal_basis())
    print(f"   A degrevlex GB cpu {t3:.2f}s deg {d3} | ratio A/D0orig {t3/t1:.1f}  A/D0red {t3/t2:.1f}", flush=True)
# yield check: brute-force decomposition counts on A (Q points) vs D0 (Q-1 points), same targets
cA = cD = 0; c1 = CPU()
for r in range(NB):
    Rt = 2 * E.random_point()
    cA += brute_pts(E, xA, Rt); cD += brute_pts(E, xD, Rt)
print(f"brute yield over {NB} targets in 2E: A {cA} ({cA/NB:.3f}/target), D0 {cD} ({cD/NB:.3f}/target), ratio {cA/max(cD,1):.3f}, predicted ratio C({Q}+3,4)/C({Q-1}+3,4) = {binomial(Q+3,4)/binomial(Q+2,4):.3f} | {CPU()-c1:.0f}s", flush=True)
