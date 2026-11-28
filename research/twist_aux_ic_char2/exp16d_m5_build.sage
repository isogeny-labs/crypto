# EXP16d-3: m = n = 5 in char 2: build the symmetrised S6 and the Weil-descended system at small Q, staged and timed.
# E: y^2+xy = x^3+b over K = F_{2^{5d}}, factor base x = u*w (u a D line), target R = sum of 5 FB points (planted).
# S6(x1..x5,xR) = Res_Y(S4(x1,x2,x3,Y), S4(x4,x5,xR,Y)).
# Stage A: S4(uw1,uw2,uw3,Y) symmetrised in w1..w3 -> A(e1,e2,e3,Y); S4(uw4,uw5,xR,Y) in w4,w5 -> B(f1,f2,Y).
# Stage B: h(e,f) = Res_Y(A,B).
# Stage C: h -> g(s1..s5) by subalgebra reduction, s = elementary symmetric of the 5 w's in terms of (e,f):
#   s1=e1+f1, s2=e2+e1f1+f2, s3=e3+e2f1+e1f2, s4=e3f1+e2f2, s5=e3f2 ; lex e1>e2>e3>f1>f2 makes LM injective.
# Stage D: checks (g(s(w_planted)) = 0; g(s(w)) vs direct numeric S6 at random w), parity, descent to 5 eqs over F_Q,
#   reduction t_i = s_i^2 (i=1..4). Saves the reduced system.
# usage: sage exp16d_m5_build.sage d seed
import sys, time
d, seed = int(sys.argv[1]), int(sys.argv[2])
CPU = time.process_time
set_random_seed(seed)
n = 5; N = n*d; Q = 2^d
K = GF(2^N, 't'); t = K.gen()
Fq, emb = K.subfield(d, name='z', map=True); z = Fq.gen()
Tr = lambda c: sum(c^(Q^i) for i in range(n))
Bas = [emb(z^i)*t^j for j in range(n) for i in range(d)]
M = matrix(GF(2), [vector(GF(2), c._vector_()) for c in Bas]).transpose(); Minv = M.inverse()
def coords(c):
    v = Minv*vector(GF(2), c._vector_())
    return [sum(Fq(int(v[j*d+i]))*z^i for i in range(d)) for j in range(n)]
def S3(a, b_, c, bb): return (a*b_ + a*c + b_*c)^2 + a*b_*c + bb
# curve and D line
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
log = []
# ---- Stage A
c0 = CPU()
RA = PolynomialRing(K, 'w1,w2,w3,Y,Z'); w1,w2,w3,Y,Z = RA.gens()
RAs = PolynomialRing(K, 'e1,e2,e3,Y'); 
def symred(f, W, Ssyms, nv):
    # f symmetric in W (first nv gens), extra gens kept; returns poly in Ssyms ring
    from itertools import combinations
    el = [sum(prod(c) for c in combinations(W, k)) for k in range(1, nv+1)]
    Rt_ = Ssyms[0].parent(); g = Rt_(0); extra = f.parent().gens()[nv:]
    while f != 0:
        c = f.lc(); a = f.lm().exponents()[0]
        k = [a[i]-a[i+1] for i in range(nv-1)] + [a[nv-1]]
        if min(k) < 0: raise ValueError("not symmetric")
        rest = a[nv:]
        g += c*prod(Ssyms[i]^k[i] for i in range(nv))*prod(Ssyms[nv+j]^rest[j] for j in range(len(rest)) if nv+j < len(Ssyms))
        f -= c*prod(el[i]^k[i] for i in range(nv))*prod(extra[j]^rest[j] for j in range(len(rest)))
    return g
S4A = S3(u*w1, u*w2, Z, b).resultant(S3(u*w3, Y, Z, b), Z)   # in w1,w2,w3,Y
RA2 = PolynomialRing(K, 'w1,w2,w3,Y', order='lex'); S4A = RA2(S4A)
A = symred(S4A, RA2.gens()[:3], RAs.gens(), 3)
RB = PolynomialRing(K, 'w4,w5,Y,Z'); w4,w5,YB,ZB = RB.gens()
S4B = S3(u*w4, u*w5, ZB, b).resultant(S3(RB(xR), YB, ZB, b), ZB)
RB2 = PolynomialRing(K, 'w4,w5,Y', order='lex'); S4B = RB2(S4B)
RBs = PolynomialRing(K, 'f1,f2,Y')
Bp = symred(S4B, RB2.gens()[:2], RBs.gens(), 2)
tA = CPU()-c0
print(f"Stage A (partial symmetrisation of two S4's): {tA:.2f}s cpu; A terms {len(A.monomials())} degY {A.degree(RAs.gens()[3])}; B terms {len(Bp.monomials())} degY {Bp.degree(RBs.gens()[2])}", flush=True)
# ---- Stage B
c0 = CPU()
RH = PolynomialRing(K, 'e1,e2,e3,f1,f2,Y', order='lex')
e1,e2,e3,f1,f2,YH = RH.gens()
Ah = RH(A.subs({}).change_ring(K)(*[e1,e2,e3,YH])) if False else A(e1,e2,e3,YH)
Bh = Bp(f1,f2,YH)
h = Ah.resultant(Bh, YH)
tB = CPU()-c0
RE = PolynomialRing(K, 'e1,e2,e3,f1,f2', order='lex'); h = RE(h)
print(f"Stage B (Res_Y, 8x8 Sylvester over K[e,f]): {tB:.2f}s cpu; h terms {len(h.monomials())}, total degree {h.total_degree()}", flush=True)
# ---- Stage C
c0 = CPU()
E1,E2,E3,F1,F2 = RE.gens()
sEF = [E1+F1, E2+E1*F1+F2, E3+E2*F1+E1*F2, E3*F1+E2*F2, E3*F2]
RS = PolynomialRing(K, 's1,s2,s3,s4,s5'); SS = RS.gens()
powc = [[RE(1)] for _ in range(5)]
def spow(i, k):
    while len(powc[i]) <= k: powc[i].append(powc[i][-1]*sEF[i])
    return powc[i][k]
g = RS(0); steps = 0; last = c0; hh = h; lim = None
while hh != 0:
    c = hh.lc(); a = hh.lm().exponents()[0]   # (A,B,C,D,E) for e1,e2,e3,f1,f2
    k4 = a[1]; k5 = a[2]; k2 = a[3]; k3 = a[4]-k4-k5; k1 = a[0]-k2-k3
    k = [k1,k2,k3,k4,k5]
    if min(k) < 0: raise ValueError(f"LM not in image: {a}")
    g += c*prod(SS[i]^k[i] for i in range(5))
    hh -= c*prod(spow(i, k[i]) for i in range(5))
    steps += 1
    if steps % 2000 == 0:
        print(f"   stage C: {steps} terms of g, remaining h terms {len(hh.monomials())}, {CPU()-c0:.1f}s", flush=True)
tC = CPU()-c0
print(f"Stage C (h(e,f) -> g(s)): {tC:.2f}s cpu; g terms {len(g.monomials())}, total degree {g.total_degree()}, deg per s_i {[g.degree(v) for v in SS]}", flush=True)
# ---- Stage D: checks
Xw = PolynomialRing(K, 'X').gen()
def sym5(ws): 
    p = prod(Xw - w for w in ws); cf = p.list()  # X^5 + s1 X^4 + ... (char 2: signs irrelevant)
    return [cf[4], cf[3], cf[2], cf[1], cf[0]]
print(f"check: g(s(w_planted)) == 0 : {g(*sym5(wpl)) == 0}", flush=True)
RY = PolynomialRing(K, 'YY'); YY = RY.gen()
def S4num(x1, x2, x3, y):
    Rz = PolynomialRing(K, 'ZZ,YY2'); ZZ, Y2 = Rz.gens()
    r = S3(Rz(x1), Rz(x2), ZZ, b).resultant(S3(Rz(x3), Y2, ZZ, b), ZZ)
    return r
ok = 0; ratios = set()
for _ in range(3):
    ws = [K.random_element() for _ in range(5)]
    Rz = PolynomialRing(K, 'ZZ,Y2'); ZZ, Y2 = Rz.gens()
    a1 = S3(Rz(u*ws[0]), Rz(u*ws[1]), ZZ, b).resultant(S3(Rz(u*ws[2]), Y2, ZZ, b), ZZ)
    a2 = S3(Rz(u*ws[3]), Rz(u*ws[4]), ZZ, b).resultant(S3(Rz(xR), Y2, ZZ, b), ZZ)
    direct = a1.resultant(a2, Y2)
    val = g(*sym5(ws)); ratios.add(K(direct)/val if val != 0 else None)
print(f"check: direct S6(uw, xR) / g(s(w)) at 3 random w: {len(ratios)} distinct ratio(s) (1 expected)", flush=True)
odd = sum(1 for ex in g.exponents() if any(ex[i] % 2 for i in range(4)))
print(f"parity: monomials of g with an odd exponent in s1..s4: {odd} of {len(g.monomials())}", flush=True)
c0 = CPU()
Rq = PolynomialRing(Fq, 's1,s2,s3,s4,s5', order='degrevlex')
eqs = [Rq(0)]*n
for cc, mon in zip(g.coefficients(), g.monomials()):
    co = coords(cc); ex = mon.exponents()[0]; mm = prod(Rq.gens()[i]^ex[i] for i in range(5))
    for j in range(n): eqs[j] += co[j]*mm
Rr = PolynomialRing(Fq, 't1,t2,t3,t4,s5', order=TermOrder('wdegrevlex', (2,2,2,2,1)))
gr = Rr.gens()
red = [sum(c*prod(gr[i]^(m.exponents()[0][i]//2) for i in range(4))*gr[4]^m.exponents()[0][4] for c, m in zip(e.coefficients(), e.monomials())) for e in eqs]
tD = CPU()-c0
print(f"Stage D (descent + reduction): {tD:.2f}s cpu; descended eq terms {[len(e.monomials()) for e in eqs]}; reduced terms {[len(e.monomials()) for e in red]}, weighted degrees {[max(2*sum(m.exponents()[0][:4])+m.exponents()[0][4] for m in e.monomials()) for e in red]}", flush=True)
# planted solution in reduced coords
sp = [Fq(0)]*5
spK = sym5(wpl)
print(f"planted s in F_Q: {all(c^Q == c for c in spK)}", flush=True)
save({'d': d, 'Q': Q, 'modulus': str(Fq.modulus()), 'red': [str(f) for f in red], 'eqs_orig': [str(f) for f in eqs],
      'planted_s': [str(c) for c in spK]}, f"results/exp16d_m5_sys_d{d}_s{seed}")
print("saved", flush=True)
