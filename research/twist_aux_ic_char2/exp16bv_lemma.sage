# EXP16bv: guarded check of Lemma 1 exactness (x1 F1 - x0 F0 == 0) for m = 4, 5, 6 on specialisations
# F(x1, c2..cm, x0), c_i random in K = GF(2^N); guards: F0 != 0, F1 != 0, F not a square, degrees 2^(m-1).
# usage: sage exp16bv_lemma.sage N seed
import sys, time
proof.all(False)
CPU = time.process_time
Nn, seed_ = int(sys.argv[1]), int(sys.argv[2]); set_random_seed(seed_)
K = GF(2 ^ Nn, 'g')
def S3(a_, b_, c_, bb): return (a_ * b_ + a_ * c_ + b_ * c_) ^ 2 + a_ * b_ * c_ + bb
b = K.random_element()
while b == 0: b = K.random_element()
print(f"# exp16bv_lemma: K=GF(2^{Nn}), seed {seed_}", flush=True)
E = EllipticCurve(K, [1, 0, 0, 0, b])
for m in (4, 5, 6):
    for trial in range(2):
        Rb = PolynomialRing(K, 'x1,x0,Y,W'); X1, X0, Yb, Wb = Rb.gens()
        # specialise x2..xm at x-coordinates of actual points (so the point check below is meaningful)
        Ps = [E.random_point() for _ in range(m - 1)]; cs = [P[0] for P in Ps]
        c1 = CPU()
        Fm = S3(X1, Rb(cs[0]), Yb, b)
        for c in cs[1:]:
            Fm = Fm.subs({Yb: Wb}).resultant(S3(Wb, Rb(c), Yb, b), Wb)
        Fm = Fm.subs({Yb: X0})
        F1, F0 = Fm.derivative(X1), Fm.derivative(X0)
        Dm = X1 * F1 - X0 * F0
        # point check: P1 random, x0 = x(P1 + ... + Pm) must be a root
        okp = all(Fm(P1[0], sum(Ps, P1)[0], 0, 0) == 0 for P1 in [E.random_point() for _ in range(5)])
        oddx1 = sum(1 for mm in Fm.monomials() if mm.degree(X1) % 2)
        print(f"m={m} t{trial}: deg (x1,x0)=({Fm.degree(X1)},{Fm.degree(X0)}) terms {len(Fm.monomials())}; "
              f"vanishes on points {okp}; F1!=0 {F1 != 0}; F0!=0 {F0 != 0}; #monomials odd in x1 {oddx1}; "
              f"x1F1-x0F0==0: {Dm == 0}; ==0 mod F: {(Dm.quo_rem(Fm)[1] == 0) if Dm != 0 else True} (cpu {CPU()-c1:.1f}s)", flush=True)
print("DONE", flush=True)
