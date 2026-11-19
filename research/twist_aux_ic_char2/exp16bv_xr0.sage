# EXP16bv: degenerate target x_R = 0 (R = T): is G(e, 0) even in ALL e_k (so Cor. 3 needs G_m(., x_R) != 0)? m = 3, 4.
import sys
proof.all(False)
K = GF(2 ^ 15, 'g'); set_random_seed(5)
b = K.random_element()
def S3(a_, b_, c_, bb): return (a_ * b_ + a_ * c_ + b_ * c_) ^ 2 + a_ * b_ * c_ + bb
R = PolynomialRing(K, 'x1,x2,x3,x4,x0,Y,Z'); x1, x2, x3, x4, x0, Y, Z = R.gens()
F4 = S3(x1, x2, Y, b).resultant(S3(x3, x0, Y, b), Y)
F5 = S3(x1, x2, Y, b).resultant(S3(x3, x4, Z, b).resultant(S3(x0, Y, Z, b), Z), Y)
for m, F in ((3, F4), (4, F5)):
    f = F.subs({x0: 0}); xs = [x1, x2, x3, x4][:m]
    odd = [sum(1 for mm in f.monomials() if mm.degree(v) % 2) for v in xs]
    print(f"m={m}: F(x, x0=0) has {len(f.monomials())} terms, nonzero {f != 0}; monomials with odd exponent of x_i: {odd}", flush=True)
print("DONE", flush=True)
