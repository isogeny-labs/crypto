exec(open('filt.py').read().split("Qm=(1<<12)-1")[0])
Qm=(1<<12)-1
# Frobenius eigenvalue lambda on <G>: sigma(G)=lambda*G
xG,yG=pts[1]; sG=(sq(xG),sq(yG))
lam=[i for i in range(1,r) if pts[i]==sG][0]
print("lambda=",lam," order:", [e for e in range(1,r) if pow(lam,e,r)==1][0])
# orbits of k in 1..r-1 under k->lam*k
orb_id=np.zeros(r,dtype=int)-1; orbits=[]
for k in range(1,r):
    if orb_id[k]<0:
        o=[]; x=k
        while orb_id[x]<0:
            orb_id[x]=len(orbits); o.append(x); x=(x*lam)%r
        orbits.append(o)
print("orbits:",len(orbits),"of size",len(orbits[0]))
rng=np.random.default_rng(1)
k=np.arange(1,r); s=(k[:,None]+k[None,:])%r; mask=s!=0
def modal_rate(b,m):
    bi=b[:,None]; bj=b[None,:]
    bs=np.full(s.shape,-1); bs[mask]=b[s[mask]-1]
    key=((bi*m+bj)*m+bs)[mask]
    cnt=np.bincount(key,minlength=m*m*m).reshape(m,m,m)
    return cnt.max(axis=2).sum()/cnt.sum()
for m in [3,5,7,9,13,15,21,35,45,63,105]:
    e=Qm//m
    vals=[ppow(u,e) for u in us]
    codes={v:i for i,v in enumerate(sorted(set(vals)))}
    b=np.array([codes[v] for v in vals])   # index k-1 -> bucket of kG
    bfull=np.concatenate([[0],b])          # index k
    real=modal_rate(b,m)
    nulls=[]
    for t in range(5):
        # equivariant null: permute orbits, rotate within orbit
        perm=rng.permutation(len(orbits)); bn=np.zeros(r,dtype=int)
        for oi,o in enumerate(orbits):
            src=orbits[perm[oi]]; rot=rng.integers(len(o))
            for j,kk in enumerate(o):
                bn[kk]=bfull[src[(j+rot)%len(src)]]
        nulls.append(modal_rate(bn[1:],m))
    print(f"m={m:3d} used={len(codes):3d} real={real:.4f} equivariant-null={np.mean(nulls):.4f}+-{np.std(nulls):.4f} chance={1/m:.4f}")
