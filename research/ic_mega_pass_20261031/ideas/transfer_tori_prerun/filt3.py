exec(open('filt2.py').read().split("rng=np.random.default_rng(1)")[0])
# merge orbits of k and -k (negation symmetry: b(-k)=b(k) exactly)
orb2={}; orbits2=[]
for o in orbits:
    key=min(min(o), min((r-x)%r for x in o))
    if key in orb2: orbits2[orb2[key]]+= [x for x in o]
    else: orb2[key]=len(orbits2); orbits2.append(list(o))
print("+- orbits:",len(orbits2),"sizes",sorted(set(len(o) for o in orbits2)))
rng=np.random.default_rng(2)
k=np.arange(1,r); s=(k[:,None]+k[None,:])%r; mask=s!=0
def modal_rate(b,m):
    bi=b[:,None]; bj=b[None,:]
    bs=np.full(s.shape,-1); bs[mask]=b[s[mask]-1]
    key=((bi*m+bj)*m+bs)[mask]
    cnt=np.bincount(key,minlength=m*m*m).reshape(m,m,m)
    return cnt.max(axis=2).sum()/cnt.sum()
for m in [5,7,15,21,35,45,63,105]:
    e=Qm//m
    vals=[ppow(u,e) for u in us]
    codes={v:i for i,v in enumerate(sorted(set(vals)))}
    b=np.array([codes[v] for v in vals]); bfull=np.concatenate([[0],b])
    real=modal_rate(b,m)
    nulls=[]
    for t in range(8):
        perm=rng.permutation(len(orbits2)); bn=np.zeros(r,dtype=int)
        for oi,o in enumerate(orbits2):
            src=orbits2[perm[oi]]
            # orbit o = {+-lam^i k0}: map lam^i k0 -> lam^(i+rot) k0', and -x -> same label as x
            o_pos=[x for x in o[:13]]; src_pos=src[:13]; rot=rng.integers(13)
            for j,kk in enumerate(o_pos):
                lab=bfull[src_pos[(j+rot)%13]]
                bn[kk]=lab; bn[(r-kk)%r]=lab
        nulls.append(modal_rate(bn[1:],m))
    print(f"m={m:3d} used={len(codes):3d} real={real:.4f} +-equivariant-null={np.mean(nulls):.4f}+-{np.std(nulls):.4f} chance={1/m:.4f}")
