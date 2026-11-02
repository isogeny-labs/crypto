import sys, random, itertools, time
sys.dont_write_bytecode=True
sys.path.insert(0, "/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/N3-capacity-lp-converse")
from n3lib import fpoly, evalf
exec(open('/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/L3-audit-items/diag_n3_p13.py').read().split("M=list")[0].split("from n3lib import fpoly, evalf")[1])
M=list(itertools.product(range(5),repeat=3))
t0=time.time()
for p in (7,11,13,17,19,23):
    row=[]
    for s in range(4):
        rng=random.Random(p) if s==0 else random.Random(5000*s+p)
        f,crv=fpoly(3,p,rng)
        pts=[pt for pt in itertools.product(range(p),repeat=3) if evalf(f,list(pt),p)==0]
        A=[[pow(x,e[0],p)*pow(y,e[1],p)*pow(z,e[2],p)%p for e in M] for x,y,z in pts]
        row.append((len(pts),len(nullspace(A,p))))
    print(p,row,round(time.time()-t0),flush=True)
