# EXP16d-3d: copy of exp16d_m5_interp.sage with (i) samples rejected when deg_Y of A(e,Y) or B(f,Y) drops
# (numeric resultant then differs from the specialised one; matters at small K, e.g. d=1), (ii) msolve export over F_2 when d=1.
# EXP16d-3b: m = n = 5, char 2: symmetrised S6 by evaluation/interpolation (the direct multivariate Res_Y of
# exp16d_m5_build.sage stalls). E: y^2+xy = x^3+b over K = F_{2^{5d}}, D line u, planted target R = sum of 5 FB points.
# g(s1..s5) with g(s(w)) = S6(uw1..uw5, xR). For any (e,f) in K^5 (cubic e, quadratic f),
#   g(s(e,f)) = h(e,f) = Res_Y(A(e,Y), B(f,Y)), A = S4(uw1,uw2,uw3,Y) symmetrised, B = S4(uw4,uw5,xR,Y) symmetrised,
# so g can be sampled cheaply. Ansatz support: s1..s4 even exponents (proved by a Z/2-grading argument, see log of
# exp16d_parity), total degree <= 16. Overdetermined solve (extra rows) checks the ansatz.
# usage: sage exp16d_m5_interp.sage d seed
import sys, time
from itertools import combinations
d, seed = int(sys.argv[1]), int(sys.argv[2])
CPU = time.process_time
set_random_seed(seed)
n = 5; N = n*d; Q = 2^d
K = GF(2^N, 't'); t = K.gen()
Fq, emb = K.subfield(d, name='z', map=True); z = Fq.gen()
Tr = lambda c: sum(c^(Q^i) for i in range(n))
Bas = [emb(z^i)*t^j for j in range(n) for i in range(d)]
Mb = matrix(GF(2), [vector(GF(2), c._vector_()) for c in Bas]).transpose(); Minv = Mb.inverse()
def coords(c):
    v = Minv*vector(GF(2), c._vector_())
    return [sum(Fq(int(v[j*d+i]))*z^i for i in range(d)) for j in range(n)]
def S3(a, b_, c, bb): return (a*b_ + a*c + b_*c)^2 + a*b_*c + bb
while True:
    b = K.random_element()
    if b == 0: continue
    E = EllipticCurve(K, [1,0,0,0,b]); sb = b.sqrt(); D = None
    for _ in range(20000):
        zz = K.random_element(); u = zz^Q + zz
        if u != 0 and Tr(sb/u) == 0 and (sb/u^2)^Q != sb/u^2: D = u; break
    if D is not None: break
u = D
FQstar = [emb(z)^i for i in range(Q-1)]
fb = [w for w in FQstar if E.is_x_coord(u*w)]
print(f"d={d} Q={Q} K=F_2^{N}; D line, {len(fb)} of {Q-1} FB x-values lift", flush=True)
wpl = [fb[randint(0, len(fb)-1)] for _ in range(5)]
Rt = sum(E.lift_x(u*w) for w in wpl); xR = Rt[0]
def symred(f, nv, Rt_):
    W = f.parent().gens()[:nv]; extra = f.parent().gens()[nv:]; Sy = Rt_.gens()
    el = [sum(prod(c) for c in combinations(W, k)) for k in range(1, nv+1)]
    g = Rt_(0)
    while f != 0:
        c = f.lc(); a = f.lm().exponents()[0]
        k = [a[i]-a[i+1] for i in range(nv-1)] + [a[nv-1]]
        if min(k) < 0: raise ValueError("not symmetric")
        g += c*prod(Sy[i]^k[i] for i in range(nv))*prod(Sy[nv+j]^a[nv+j] for j in range(len(extra)))
        f -= c*prod(el[i]^k[i] for i in range(nv))*prod(extra[j]^a[nv+j] for j in range(len(extra)))
    return g
c0 = CPU()
RA = PolynomialRing(K, 'w1,w2,w3,Y,Z'); w1,w2,w3,Y,Z = RA.gens()
A = symred(PolynomialRing(K, 'w1,w2,w3,Y', order='lex')(S3(u*w1, u*w2, Z, b).resultant(S3(u*w3, Y, Z, b), Z)), 3, PolynomialRing(K, 'e1,e2,e3,Y'))
RB = PolynomialRing(K, 'w4,w5,Y,Z'); w4,w5,YB,ZB = RB.gens()
B = symred(PolynomialRing(K, 'w4,w5,Y', order='lex')(S3(u*w4, u*w5, ZB, b).resultant(S3(RB(xR), YB, ZB, b), ZB)), 2, PolynomialRing(K, 'f1,f2,Y'))
print(f"Stage A: {CPU()-c0:.2f}s cpu; A terms {len(A.monomials())}, B terms {len(B.monomials())}", flush=True)
mons = []
for k5 in range(17):
    for a1 in range(9):
        for a2 in range(9):
            for a3 in range(9):
                for a4 in range(9):
                    if 2*(a1+a2+a3+a4) + k5 <= 16: mons.append((2*a1, 2*a2, 2*a3, 2*a4, k5))
nm = len(mons); npts = nm + 60
print(f"ansatz monomials: {nm}; sample points {npts}", flush=True)
Ry = PolynomialRing(K, 'y'); y = Ry.gen()
c0 = CPU()
rows = []; rhs = []
def s_of(e, f):
    return [e[0]+f[0], e[1]+e[0]*f[0]+f[1], e[2]+e[1]*f[0]+e[0]*f[1], e[2]*f[0]+e[1]*f[1], e[2]*f[1]]
for _ in range(npts):
    e = [K.random_element() for _ in range(3)]; f = [K.random_element() for _ in range(2)]
    Ay = A(e[0], e[1], e[2], y); By = B(f[0], f[1], y)
    while Ay.degree() < 4 or By.degree() < 4:
        e = [K.random_element() for _ in range(3)]; f = [K.random_element() for _ in range(2)]
        Ay = A(e[0], e[1], e[2], y); By = B(f[0], f[1], y)
    rhs.append(Ay.resultant(By))
    s = s_of(e, f)
    pw = [[K(1)] for _ in range(5)]
    for i in range(5):
        for j in range(16): pw[i].append(pw[i][-1]*s[i])
    rows.append([pw[0][m[0]]*pw[1][m[1]]*pw[2][m[2]]*pw[3][m[3]]*pw[4][m[4]] for m in mons])
tS = CPU()-c0
print(f"Stage B' (sampling {npts} points: numeric Res_Y of two quartics + monomial rows): {tS:.2f}s cpu", flush=True)
c0 = CPU()
Mm = matrix(K, rows); vv = vector(K, rhs)
print(f"   matrix type {type(Mm).__name__}", flush=True)
try:
    sol = Mm.solve_right(vv); consistent = True
except ValueError:
    consistent = False
tL = CPU()-c0
print(f"Stage C' (linear solve {npts}x{nm} over K): {tL:.2f}s cpu; consistent (ansatz holds on {npts-nm} extra rows): {consistent}; rank {Mm.rank() if not consistent else 'n/a'}", flush=True)
if not consistent: sys.exit(1)
RS = PolynomialRing(K, 's1,s2,s3,s4,s5'); SS = RS.gens()
g = sum(sol[i]*prod(SS[j]^mons[i][j] for j in range(5)) for i in range(nm))
print(f"g: {len(g.monomials())} nonzero terms, total degree {g.total_degree()}, deg per s_i {[g.degree(v) for v in SS]}, max weighted deg sum(i*k_i) {max(sum((j+1)*ex[j] for j in range(5)) for ex in g.exponents())}", flush=True)
Xw = PolynomialRing(K, 'X').gen()
def sym5(ws):
    cf = prod(Xw - w for w in ws).list(); return [cf[4], cf[3], cf[2], cf[1], cf[0]]
print(f"check: g(s(w_planted)) == 0 : {g(*sym5(wpl)) == 0}", flush=True)
# independent check: direct S6 at random w (full resultant chain, numeric)
Rz = PolynomialRing(K, 'ZZ,Y2'); ZZ, Y2 = Rz.gens()
okc = 0
for _ in range(3):
    ws = [K.random_element() for _ in range(5)]
    a1 = S3(Rz(u*ws[0]), Rz(u*ws[1]), ZZ, b).resultant(S3(Rz(u*ws[2]), Y2, ZZ, b), ZZ)
    a2 = S3(Rz(u*ws[3]), Rz(u*ws[4]), ZZ, b).resultant(S3(Rz(xR), Y2, ZZ, b), ZZ)
    okc += (K(a1.resultant(a2, Y2)) == g(*sym5(ws)))
print(f"check: direct S6(uw,xR) == g(s(w)) at {okc}/3 random w", flush=True)
# a random non-solution should not vanish
print(f"check: g at random FB 5-tuple != 0 : {g(*sym5([fb[randint(0,len(fb)-1)] for _ in range(5)])) != 0}", flush=True)
c0 = CPU()
Rq = PolynomialRing(Fq, 's1,s2,s3,s4,s5', order='degrevlex')
eqs = [Rq(0)]*n
for cc, mon in zip(g.coefficients(), g.monomials()):
    co = coords(cc); ex = mon.exponents()[0]; mm = prod(Rq.gens()[i]^ex[i] for i in range(5))
    for j in range(n): eqs[j] += co[j]*mm
Rr = PolynomialRing(Fq, 't1,t2,t3,t4,s5', order=TermOrder('wdegrevlex', (2,2,2,2,1)))
gr = Rr.gens()
red = [sum(c*prod(gr[i]^(m.exponents()[0][i]//2) for i in range(4))*gr[4]^m.exponents()[0][4] for c, m in zip(e.coefficients(), e.monomials())) for e in eqs]
spK = sym5(wpl)
print(f"Stage D (descent + reduction): {CPU()-c0:.2f}s cpu; descended eq terms {[len(e.monomials()) for e in eqs]}; weighted degrees {[max(2*sum(m.exponents()[0][:4])+m.exponents()[0][4] for m in e.monomials()) for e in red]}; planted s in F_Q: {all(c^Q == c for c in spK)}", flush=True)
odd = sum(1 for e in eqs for ex in e.exponents() if any(ex[i] % 2 for i in range(4)))
print(f"parity in descended eqs: odd-exponent terms in s1..s4: {odd} (by construction of the ansatz; consistency above is the test)", flush=True)
spq = [Fq(c) if False else None for c in spK]
# planted point in reduced coords (F_Q values): t_i = s_i^2
pl = [coords(c)[0] for c in spK]
plr = [pl[0]^2, pl[1]^2, pl[2]^2, pl[3]^2, pl[4]]
print(f"check: reduced system vanishes at planted (t,s5): {all(f(*plr) == 0 for f in red)}", flush=True)
if d == 1:
    import os; os.makedirs('results/exp16d_msolve', exist_ok=True)
    fn = f'results/exp16d_msolve/m5_q2_s{seed}_red.ms'
    with open(fn, 'w') as F:
        F.write('t1,t2,t3,t4,s5\n2\n' + ',\n'.join(str(f).replace(' ', '') for f in red) + '\n')
    print('wrote', fn, flush=True)
save({'d': d, 'Q': Q, 'modulus': str(Fq.modulus()), 'red': [str(f) for f in red], 'planted_red': [str(c) for c in plr]}, f"results/exp16d_m5_sys_d{d}_s{seed}")
print("saved", flush=True)
