import sys,json
sys.path.insert(0,'.')
from fitcubic import fit
from fractions import Fraction
def shifted(p):
    m,N,pts,R=fit(p); v=N[0]; v=v*pow(int(v[m.index((2,1))]),p-2,p)%p
    co={m[i]:int(v[i]) for i in range(len(m))}
    # substitute alpha=A-3, beta=B+2 : expand polynomial
    from itertools import product
    from math import comb
    out={}
    for (i,j),c in co.items():
        for r in range(i+1):
            for s in range(j+1):
                coef=c*comb(i,r)*pow(-3,i-r,p)*comb(j,s)*pow(2,j-s,p)%p
                out[(r,s)]=(out.get((r,s),0)+coef)%p
    return out
if __name__=='__main__':
    for p in (19,23,29,37,41,47):
        o=shifted(p)
        print(p,{k:v for k,v in sorted(o.items(),key=lambda kv:(sum(kv[0]),kv[0])) if v})
