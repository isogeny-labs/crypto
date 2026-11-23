# EXP16d-1: parity test for the squaring reduction at general m (char 2).
# Claim tested: the symmetrised g(s1..sm) has only even exponents in s1..s_{m-1}
#   <=> x_i d f/dx_i = x_j d f/dx_j for all i,j  (f = unsymmetrised S_{m+1}(x1..xm, xR), xR fixed)
#   <=> every monomial of f has all its x1..xm exponents of the same parity.
# (char 2: d(s^k)/ds = 0 iff k even; w -> s has Vandermonde Jacobian, so the converse holds generically.)
# m = 2,3,4: full f (xR, b random in K). m = 5: x3,x4,x5 specialised to random constants, test the (1,2) pair.
import sys, time
CPU = time.process_time
set_random_seed(int(sys.argv[1]) if len(sys.argv) > 1 else 1601)
K = GF(2^20, 'g')
def S3(a, b_, c, bb): return (a*b_ + a*c + b_*c)^2 + a*b_*c + bb
def parity_ok(f, idx):
    bad = 0
    for e in f.exponents():
        if len(set(e[i] % 2 for i in idx)) > 1: bad += 1
    return bad
for trial in range(3):
    b = K.random_element(); xR = K.random_element()
    # full S_{m+1} for m = 2,3,4 ; variables x1..x4 and also check parity including xR as a variable
    P = PolynomialRing(K, 'x1,x2,x3,x4,xr,Y,Z'); x1,x2,x3,x4,xr,Y,Z = P.gens()
    S3f = S3(x1, x2, xr, b)
    c0 = CPU(); S4 = S3(x1, x2, Y, b).resultant(S3(x3, xr, Y, b), Y); t4 = CPU()-c0
    c0 = CPU(); S4b = S3(x3, x4, Z, b).resultant(S3(xr, Y, Z, b), Z); S5 = S3(x1, x2, Y, b).resultant(S4b, Y); t5 = CPU()-c0
    for m, f, idx in [(2, S3f, [0,1,4]), (3, S4, [0,1,2,4]), (4, S5, [0,1,2,3,4])]:
        nb = parity_ok(f, idx)
        nbR = parity_ok(f.subs(xr=xR), idx[:-1])
        print(f"trial {trial} m={m}: terms {len(f.monomials())} maxdeg/var {[f.degree(P.gens()[i]) for i in idx]} | monomials with mixed parity (x1..xm,xR variable): {nb} | (xR fixed): {nbR}", flush=True)
    print(f"   time S4 {t4:.2f}s S5 {t5:.2f}s", flush=True)
    # m = 5: S6(x1,x2,c3,c4,c5,xR) = Res_Y(S3(x1,x2,Y), S5(c3,c4,c5,xR? ...)) built as Res_Y(S3(x1,x2,Y), S5'(Y,c3,c4,c5,xR))
    # S5'(Y,c3,c4,c5,xR) = Res_Z(S3(c3,c4,Z), S4(c5,xR,Y,Z)) with S4(c5,xR,Y,Z) = Res_W(S3(c5,xR,W), S3(Y,Z,W))
    P2 = PolynomialRing(K, 'x1,x2,Y,Z,W'); X1,X2,YY,ZZ,WW = P2.gens()
    c3,c4,c5 = [K.random_element() for _ in range(3)]
    c0 = CPU()
    S4c = S3(P2(c5), P2(xR), WW, b).resultant(S3(YY, ZZ, WW, b), WW)
    S5c = S3(P2(c3), P2(c4), ZZ, b).resultant(S4c, ZZ)
    S6c = S3(X1, X2, YY, b).resultant(S5c, YY)
    t6 = CPU()-c0
    nb6 = parity_ok(S6c, [0,1])
    D = X1*S6c.derivative(X1) - X2*S6c.derivative(X2)
    print(f"trial {trial} m=5 specialised: S6(x1,x2,c3,c4,c5,xR) terms {len(S6c.monomials())} deg x1 {S6c.degree(X1)} deg x2 {S6c.degree(X2)} | mixed-parity monomials {nb6} | x1 f1 - x2 f2 == 0: {D == 0} | build {t6:.2f}s", flush=True)
    # control: a random non-summation symmetric poly should fail
    ctl = X1^3*X2 + X1*X2 + X1^2 + X2^2 + X1 + X2
    print(f"   control (x1^3x2+x1x2+x1^2+x2^2+x1+x2): mixed {parity_ok(ctl,[0,1])}", flush=True)
