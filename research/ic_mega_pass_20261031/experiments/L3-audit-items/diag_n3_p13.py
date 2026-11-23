import sys, random, json, itertools
sys.path.insert(0, "/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/N3-capacity-lp-converse")
from n3lib import fpoly, evalf
def nullspace(A, p):
    A=[r[:] for r in A]; n=len(A[0]); piv=[]; r=0
    for c in range(n):
        k=next((i for i in range(r,len(A)) if A[i][c]%p),None)
        if k is None: continue
        A[r],A[k]=A[k],A[r]; iv=pow(A[r][c],-1,p); A[r]=[x*iv%p for x in A[r]]
        for i in range(len(A)):
            if i!=r and A[i][c]%p:
                t=A[i][c]; A[i]=[(x-t*y)%p for x,y in zip(A[i],A[r])]
        piv.append(c); r+=1
    free=[c for c in range(n) if c not in piv]; B=[]
    for fc in free:
        v=[0]*n; v[fc]=1
        for i,pc in enumerate(piv): v[pc]=(-A[i][fc])%p
        B.append(v)
    return B
M=list(itertools.product(range(5),repeat=3)); res={}
for p in (11,13,17):
    rng=random.Random(p); f,crv=fpoly(3,p,rng)
    pts=[pt for pt in itertools.product(range(p),repeat=3) if evalf(f,list(pt),p)==0]
    A=[[pow(x,s[0],p)*pow(y,s[1],p)*pow(z,s[2],p)%p for s in M] for x,y,z in pts]
    B=nullspace(A,p)
    gp=lambda v:"+".join(f"{c}*x^{e[0]}*y^{e[1]}*z^{e[2]}" for c,e in zip(v,M) if c) or "0"
    res[p]=dict(curve=crv,npts=len(pts),kernel_dim=len(B),
        f="+".join(f"{c%p}*x^{e[0]}*y^{e[1]}*z^{e[2]}" for e,c in f.items() if c%p),kernel=[gp(v) for v in B])
    print(p,len(pts),len(B),flush=True)
json.dump(res,open("raw_n3_p13_diag.json","w"),indent=1)
