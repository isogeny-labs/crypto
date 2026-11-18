import time
p=1048589; F=GF(p); E=EllipticCurve(F,[0,487554,0,1,0]).short_weierstrass_model()
for l in [2,5,13,17,101,151,197]:
    t=time.time(); I=E.isogenies_prime_degree(l); print(l,len(I),[phi.codomain().j_invariant() for phi in I], round(time.time()-t,1), flush=True)
