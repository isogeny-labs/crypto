# (1) Wilson's rank formula for inclusion matrices W_{t,K}(v) over F_p, t <= min(K, v-K):
#     rank_p = sum_{i=0..t, p does not divide C(K-i, t-i)} (C(v,i) - C(v,i-1)).
# (2) Boolean j*: first j where multiplication by l^e (l = sum 2^i a_i) on the squarefree graded ring is
#     not injective, over F_p for small p, versus the closed form floor((k-e)/2)+1 claimed for p > k.
import itertools
from math import comb
def rank_mod(rows, p, ncols):
    rows=[r[:] for r in rows]; rk=0; col=0
    piv_rows=[]
    m=len(rows)
    for c in range(ncols):
        pr=None
        for i in range(rk,m):
            if rows[i][c]%p: pr=i; break
        if pr is None: continue
        rows[rk],rows[pr]=rows[pr],rows[rk]
        inv=pow(rows[rk][c],p-2,p)
        rows[rk]=[x*inv%p for x in rows[rk]]
        for i in range(m):
            if i!=rk and rows[i][c]%p:
                f=rows[i][c]; rows[i]=[(x-f*y)%p for x,y in zip(rows[i],rows[rk])]
        rk+=1
    return rk
def W(t,K,v):
    T=list(itertools.combinations(range(v),t)); KK=list(itertools.combinations(range(v),K))
    idx={s:i for i,s in enumerate(KK)}
    rows=[]
    for s in T:
        r=[0]*len(KK); ss=set(s)
        for j,u in enumerate(KK):
            if ss<=set(u): r[j]=1
        rows.append(r)
    return rows,len(KK)
bad=0; tot=0
for v in range(2,10):
    for t in range(0,v+1):
        for K in range(t,v+1):
            if t>min(K,v-K): continue
            rows,nc=W(t,K,v)
            for p in (2,3,5,7):
                r=rank_mod(rows,p,nc)
                f=sum(comb(v,i)-(comb(v,i-1) if i>0 else 0) for i in range(t+1) if comb(K-i,t-i)%p)
                tot+=1
                if r!=f: bad+=1; print('MISMATCH',v,t,K,p,r,f)
print(f'Wilson formula: {tot-bad}/{tot} (v<=9, t<=min(K,v-K), p in 2,3,5,7) match')
# (2) j* for Boolean with weights 2^i mod p
def jstar_bool(k,e,p):
    w=[pow(2,i,p) for i in range(k)]
    # l^e in squarefree ring = e! * sum_{|U|=e} w_U a_U
    fact=1
    for i in range(1,e+1): fact=fact*i%p
    for j in range(0,k+1):
        src=list(itertools.combinations(range(k),j)); dst=list(itertools.combinations(range(k),j+e)) if j+e<=k else []
        if not dst: return j
        idx={s:i for i,s in enumerate(dst)}
        rows=[]
        for s in src:
            r=[0]*len(dst); ss=set(s)
            for U in itertools.combinations([i for i in range(k) if i not in ss],e):
                wu=fact
                for i in U: wu=wu*w[i]%p
                r[idx[tuple(sorted(ss|set(U)))]]=(r[idx[tuple(sorted(ss|set(U)))]]+wu)%p
            rows.append(r)
        if rank_mod(rows,p,len(dst))<len(src): return j
    return k+1
print('k e p jstar closed_form(p>k) agree')
for e in (2,4):
    for k in range(e,11):
        cf=(k-e)//2+1
        for p in (3,5,7,11,13,1000003):
            js=jstar_bool(k,e,p)
            if p>k or js!=cf: print(k,e,p,js,cf,js==cf, 'p>k' if p>k else 'p<=k')
