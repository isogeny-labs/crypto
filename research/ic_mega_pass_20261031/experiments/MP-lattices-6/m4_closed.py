# exact closed-form count for box and symmetric-orbit lattices, m=4 (d=8), and m=2,3 checks
import json, sys
from math import comb
from fractions import Fraction as F
def box(m,d,l):
    M=d*l; n=M+1
    num=sum((M+1-d*j)**m for j in range(1,l+1))         # sum_s k(s)
    den=m*n**(m-1)*M*n//2                                # sum deg
    return F(num,den)
def sym(m,d,l):
    M=d*l
    num=sum(comb(M-d*j+m,m) for j in range(1,l+1))     # multisets with all coords >= dj
    # sum of degrees over multisets of size m from {0..M}: N*m*M/2 by symmetry s->M-s
    N=comb(M+m,m); den=F(N*m*M,2)
    return F(num)/den
out={}
for m in (2,3,4):
    d=2**(m-1)
    for kind,fn in (("box",box),("sym",sym)):
        ls=[100,200,400]; v=[float(fn(m,d,l)) for l in ls]
        # Richardson in 1/l
        r=[2*v[i+1]-v[i] for i in range(2)]; out[f"m{m}_{kind}"]={"l":ls,"vals":v,"richardson":r}
json.dump(out,open(sys.argv[1],'w'),indent=1); print(json.dumps(out))
