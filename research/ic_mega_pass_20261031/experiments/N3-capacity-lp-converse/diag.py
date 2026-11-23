# diagnose seed-1 l=3 box8 excess: per-level gains c(M,j), j=1..3, same curve as armA m2hp seed 1
import random, json, sys, itertools
from n3lib import *
p = int(sys.argv[1]); seed = int(sys.argv[2])
rng = random.Random(5000*seed+p); f, crv = fpoly(2, p, rng)
a,b,xR = crv
M = list(itertools.product(range(9), repeat=2))
# singularity check: affine singular points of f mod p, and 2-torsion check of xR (x^3+ax+b at xR)
sing = [(x,y) for x in range(p) for y in range(p) if evalf(f,[x,y],p)==0 and
        evalf({(e[0]-1,e[1]):c*e[0] for e,c in f.items() if e[0]},[x,y],p)==0 and
        evalf({(e[0],e[1]-1):c*e[1] for e,c in f.items() if e[1]},[x,y],p)==0]
print("curve", crv, "rhs(xR)=", (xR**3+a*xR+b)%p, "singular affine pts", sing)
for j in (1,2,3):
    pts = sample_U(f, 2, p, j, 4000, random.Random(j))
    g = gain(eval_matrix(M, pts, p**j), p, j); print("j", j, "gain", g, "ksum", ksum(M,2,j), flush=True)
