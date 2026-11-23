import random, sys
import os; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ic_fp2 as M
r=10007
def determined(rows,nv):
    # rows: list of dict col->coef, col nv = w ; check e_w in rowspace via elimination
    piv={}
    def reduce(v):
        v=dict(v)
        changed=True
        while True:
            ks=[k for k in v if v[k]%r and k in piv]
            if not ks: return v
            k=ks[0]; pr=piv[k]; c=v[k]
            for kk,x in pr.items(): v[kk]=(v.get(kk,0)-c*x)%r
            v={k2:x for k2,x in v.items() if x%r}
    for row in rows:
        v=reduce(row)
        if v:
            k=min(v); c=pow(v[k],-1,r); v={kk:x*c%r for kk,x in v.items()}
            for kk in list(piv):
                if k in piv[kk]:
                    cc=piv[kk][k]; piv[kk]={a:(piv[kk].get(a,0)-cc*v.get(a,0))%r for a in set(piv[kk])|set(v)}; piv[kk]={a:x for a,x in piv[kk].items() if x}
            piv[k]=v
    return not reduce({nv:1})
bad=0
for trial in range(300):
    rng=random.Random(trial); nv=rng.randrange(3,30)
    f=[rng.randrange(r) for _ in range(nv)]; w=rng.randrange(1,r)
    S=M.Solver(r); rows=[]; first_uf=None; first_la=None
    for e in range(3*nv):
        i=rng.randrange(nv); j=rng.randrange(nv); si=rng.choice((1,-1)); sj=rng.choice((1,-1))
        beta=rng.randrange(1,r); alpha=(si*f[i]+sj*f[j]-beta*w)%r
        row={}
        row[i]=(row.get(i,0)+si)%r; row[j]=(row.get(j,0)+sj)%r; row[nv]=(-beta)%r
        row={k:x for k,x in row.items() if x}
        rows.append(row)
        if first_la is None and determined(rows,nv): first_la=e
        if first_uf is None:
            eq=S.add(i,si,j,sj,alpha,beta)
            if eq is not None and M.solve_eq(eq[0],eq[1],r) is not None:
                assert M.solve_eq(eq[0],eq[1],r)==w; first_uf=e
    if first_uf!=first_la: bad+=1; print(trial,nv,first_uf,first_la)
print('mismatches',bad)
