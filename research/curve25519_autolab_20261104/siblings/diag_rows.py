import random
r=131203;C=1
def run(m,seed):
    rnd=random.Random(seed)
    d=rnd.randrange(r); s=rnd.randrange(r)
    vec=lambda:[rnd.randrange(r) for _ in range(3)]
    J=[vec() for _ in range(m)];P=vec()
    sc=lambda v:(v[0]+v[1]*d+v[2]*s)%r
    seen={};rows=set();coll=0;prev=None;n=0
    def emit(v):
        nonlocal coll,n
        n+=1
        c=sc(v)
        if c==0:return
        k=min(c,r-c)
        if k in seen:
            u=seen[k]
            if u!=v:
                coll+=1
                same=sc(u)==c
                row=tuple((v[i]-u[i])%r if same else (v[i]+u[i])%r for i in range(3))
                neg=tuple((-x)%r for x in row)
                rows.add(min(row,neg))
        else: seen[k]=v
    emit(P)
    while n<12*int(r**.5):
        h=rnd.randrange(m)
        for a in range(m):
            for pl in (1,-1):
                if pl==-1 and prev==a:continue
                emit([(P[i]+pl*J[a][i])%r for i in range(3)])
        P=[(P[i]+J[h][i])%r for i in range(3)];prev=h
    return coll,len(rows)
for m in (4,16):
    tot=[0,0]
    for sd in range(20):
        c,u=run(m,sd);tot[0]+=c;tot[1]+=u
    print(m,tot,tot[0]/tot[1])
