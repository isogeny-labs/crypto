import sys, random, itertools, subprocess
sys.dont_write_bytecode=True
sys.path.insert(0, "/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/N3-capacity-lp-converse")
from n3lib import fpoly
import cypari2
pari=cypari2.Pari(); pari.allocatemem(10**9)
M=list(itertools.product(range(5),repeat=3))
for p,s in ((13,0),(19,2),(19,3),(19,1)):
    rng=random.Random(p) if s==0 else random.Random(5000*s+p)
    f,crv=fpoly(3,p,rng)
    # independent point enumeration: direct polynomial evaluation using Python ints, no n3lib.evalf
    terms=[(e,c%p) for e,c in f.items() if c%p]
    rows=[]
    for x in range(p):
        for y in range(p):
            for z in range(p):
                v=0
                for (a,b,c),co in terms: v+=co*pow(x,a,p)*pow(y,b,p)*pow(z,c,p)
                if v%p==0: rows.append([pow(x,a,p)*pow(y,b,p)*pow(z,c,p)%p for a,b,c in M])
    A=pari.matrix(len(rows),125,[i for r in rows for i in r])
    A=A*pari("Mod(1,%d)"%p)
    print(p,s,crv,"npts",len(rows),"kernel dim",125-int(pari.matrank(A)),flush=True)
