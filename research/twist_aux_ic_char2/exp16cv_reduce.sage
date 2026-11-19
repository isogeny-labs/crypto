import sys, time
# EXP16cv (verifier of exp16c): does the dense affine plane line A = c0 + u0*F_Q (gamma = c0/u0 in F_{Q^2} \ F_Q)
# admit a PARTIAL squaring reduction?  Argument: S5 = F(e1^2, e2^2, e3^2, e4) (Exp 15 evenness), and on A
# e4 = c0^4 + u0^4 * l(s) with l(s) = s4 + g s3 + g^2 s2 + g^3 s1, g = c0/u0.  The K-polys of the descended system
# only involve s_i^2 and the Galois conjugates l, l^sigma; since g in F_{Q^2}, these span a 2-dim space W defined
# over F_Q.  New F_Q-coordinates y1,y2 (basis of W), y3,y4 (complement): the system is even in y3,y4 -> 4x reduction.
# usage: sage exp16cv_reduce.sage seed doorig
seed = int(sys.argv[1]); DOORIG = sys.argv[2] == '1'
sys.argv = ['x', '5', '1', '1', str(seed), '0', '0', '0']
src = open('exp14_char2_solve.sage').read(); src = src[:src.index('print("d Q curve')]
exec(preparse(src))
set_random_seed(seed)
CPU = time.process_time
FQall = [K(0)] + FQstar
toFq = {emb(x): x for x in Fq}
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
def parity(eqs, n=4):
    odd = [0] * n; tot = 0
    for e in eqs:
        for m in e.monomials():
            ex = m.exponents()[0]; tot += 1
            for i in range(n):
                if ex[i] % 2: odd[i] += 1
    return odd, tot

while True:
    b = K.random_element(); sb = b.sqrt(); tau = sb + sb^(Q^2)
    if b != 0 and tau != 0: break
E = EllipticCurve(K, [1, 0, 0, 0, b])
u0 = sb / tau
while True:
    yy = K.random_element(); zz = yy + yy^(Q^2); c0 = sb * zz
    if (zz * tau)^Q != zz * tau and c0.trace() == 0: break
g = c0 / u0
xA = [c0 + u0 * w for w in FQall if E.is_x_coord(c0 + u0 * w)]
print(f"seed {seed} #E={E.order()} | A: g=c0/u0 in F_Q^2: {g^(Q^2)==g}, in F_Q: {g^Q==g} | lifting {len(xA)}/{Q}, all Tr(x)=0: {all(x.trace()==0 for x in xA)}", flush=True)
# F_Q basis of W = span(v, v^sigma), v = (g^3, g^2, g, 1) on (s1..s4): m1 = v + v^Q, m2 = th*v + (th*v)^Q
th = zz * tau   # in F_{Q^2} \\ F_Q by construction
v = [g^3, g^2, g, K(1)]
m1 = [toFq[c + c^Q] for c in v]; m2 = [toFq[th*c + (th*c)^Q] for c in v]
for n1 in range(4):
    for n2 in range(n1 + 1, 4):
        Mt = matrix(Fq, [m1, m2, [Fq(int(i == n1)) for i in range(4)], [Fq(int(i == n2)) for i in range(4)]])
        if Mt.is_invertible(): break
    if Mt.is_invertible(): break
print(f"change of variables y = M s, M =\n{Mt}", flush=True)
Ry = PolynomialRing(Fq, 'y1,y2,y3,y4', order='degrevlex')
Mtinv = Mt.inverse()
svals = list(Mtinv * vector(Ry, Ry.gens()))   # s = M^-1 y
Rt2 = PolynomialRing(Fq, 'y1,y2,t3,t4', order='degrevlex')
Rt2w = PolynomialRing(Fq, 'y1,y2,t3,t4', order=TermOrder('wdegrevlex', (1, 1, 2, 2)))
def red(e, RR):
    G = RR.gens(); return sum(c * G[0]^m.exponents()[0][0] * G[1]^m.exponents()[0][1] * G[2]^(m.exponents()[0][2]//2) * G[3]^(m.exponents()[0][3]//2) for c, m in zip(e.coefficients(), e.monomials()))
for r in range(2):
    Rtg = 2 * E.random_point()
    c1 = CPU(); gA = kpoly(u0, c0, b, Rtg[0]); eA = descend(gA); tb = CPU() - c1
    eY = [e(*svals) for e in eA]
    print(f"r{r}: build {tb:.1f}s | A descended odd-exponent counts (s1..s4) {parity(eA)} | in y-coords (y1..y4) {parity(eY)}", flush=True)
    if any(parity(eY)[0][2:]):
        print("   NOT even in y3,y4 -> no 4x reduction", flush=True); continue
    eR = [red(e, Rt2) for e in eY]
    c1 = CPU(); gbR = Rt2.ideal(eR).groebner_basis('libsingular:std'); tR = CPU() - c1; dR = len(Rt2.ideal(gbR).normal_basis())
    eRw = [Rt2w(e) for e in eR]
    c1 = CPU(); gbRw = Rt2w.ideal(eRw).groebner_basis('libsingular:std'); tRw = CPU() - c1
    print(f"   reduced (y1,y2,t3=y3^2,t4=y4^2): degrevlex GB cpu {tR:.2f}s deg {dR} | wdegrevlex(1,1,2,2) GB cpu {tRw:.2f}s", flush=True)
    # F_Q-points: reduced vs original (field equations make the ideal radical; deg = #F_Q points)
    c1 = CPU(); nR = len(Rt2.ideal(list(gbR) + [x^Q - x for x in Rt2.gens()]).normal_basis()); tfR = CPU() - c1
    msg = f"   #F_Q-points reduced {nR} ({tfR:.1f}s)"
    if DOORIG and r == 0:
        c1 = CPU(); gbO = Rq.ideal(eA).groebner_basis('libsingular:std'); tO = CPU() - c1; dO = len(Rq.ideal(gbO).normal_basis())
        c1 = CPU(); nO = len(Rq.ideal(list(gbO) + [x^Q - x for x in Rq.gens()]).normal_basis()); tfO = CPU() - c1
        msg += f" | original degrevlex GB cpu {tO:.2f}s deg {dO}, #F_Q-points {nO} ({tfO:.1f}s)"
    print(msg, flush=True)
