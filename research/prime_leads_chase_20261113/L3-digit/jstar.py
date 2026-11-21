# Exact j*_blk: first degree j where multiplication by l^e on gr(A_blk) = ⊗F[d]/(d^{b_i}) is not injective,
# l = sum w_i d_i with random nonzero weights mod p (rescaling-invariant). Exact rank mod p.
import itertools, random, sys, json
P=1000003
def rank(rows):
    rows=[dict(r) for r in rows if r]; piv={}; rk=0
    for r in rows:
        r={k:v%P for k,v in r.items() if v%P}
        while r:
            lead=max(r)
            if lead in piv:
                pr=piv[lead]; c=r[lead]
                for k,v in pr.items(): r[k]=(r.get(k,0)-c*v)%P
                r={k:v for k,v in r.items() if v}
            else:
                inv=pow(r[lead],P-2,P); piv[lead]={k:v*inv%P for k,v in r.items()}; rk+=1; break
    return rk
def jstar(radices,e,seed=1):
    random.seed(seed); w=[random.randrange(1,P) for _ in radices]
    monos=list(itertools.product(*[range(b) for b in radices]))
    top=sum(b-1 for b in radices)
    # l^e as dict over exponent tuples (in gr: d_i^{b_i}=0)
    le={tuple([0]*len(radices)):1}
    for _ in range(e):
        nl={}
        for mon,c in le.items():
            for i,b in enumerate(radices):
                if mon[i]+1<b:
                    m2=list(mon); m2[i]+=1; m2=tuple(m2); nl[m2]=(nl.get(m2,0)+c*w[i])%P
        le=nl
    idx={m:i for i,m in enumerate(monos)}
    for j in range(0,top+1):
        src=[m for m in monos if sum(m)==j]
        rows=[]
        for s in src:
            r={}
            for mon,c in le.items():
                t=tuple(a+b for a,b in zip(s,mon))
                if all(t[i]<radices[i] for i in range(len(t))): r[idx[t]]=(r.get(idx[t],0)+c)%P
            rows.append(r)
        if rank(rows)<len(src): return j
    return top+1
out=[]
for m in (2,3,4):
    e=2**(m-1)
    fams={'bool':lambda k:[2]*k,'b3':lambda k:[3]*k,'b4':lambda k:[4]*k,'mix':lambda k:[2 if i%2==0 else 3 for i in range(k)]}
    for f,rad in fams.items():
        for k in range(1,10):
            r=rad(k)
            n=1
            for b in r: n*=b
            if n>3000: break
            js=jstar(r,e)
            out.append(dict(m=m,fam=f,k=k,B=n,e=e,jstar=js,ffd=m*e+js))
    for B in (3,4,5,6,8,10,12,16,20,24,32,48,64,96):
        out.append(dict(m=m,fam='uni',k=1,B=B,e=e,jstar=jstar([B],e),ffd=m*e+jstar([B],e)))
for o in out: print(json.dumps(o))
