import math,random
exec(open('/tmp/m4mitm2.py').read().split('def run(')[0])
def run3(nbits,seed=1):
    p=prev_prime(1<<nbits); E=EC(p,0,7); rng=random.Random(seed+nbits)
    while p%4!=3: p=prev_prime(p-2); E=EC(p,0,7)
    B=max(4,int(round(p**0.25)))
    base=[]
    while len(base)<B:
        P=rand_point(E,rng)
        if P is not None: base.append(P)
    table={}; adds=0
    for i in range(B):
        for j in range(i,B):
            S=E.add(base[i],base[j]); adds+=1; table.setdefault(S,(i,j))
    trials=80; totq=0
    for _ in range(trials):
        idx=rng.sample(range(B),3); R=None
        for t in idx: R=E.add(R,base[t])
        order=list(range(B)); rng.shuffle(order); q=0
        for i in order:
            q+=1
            D=E.add(R,E.neg(base[i]))
            if D in table: break
        totq+=q
    avgq=totq/trials
    solve=adds+avgq
    hitprob=min(1.0,(B**3)/p); uniform=adds+(1.0/hitprob)*B
    return p,B,adds,avgq,solve,uniform
rows=[(nb,)+run3(nb) for nb in (13,15,17,19,21)]
for r in rows: print(r[0],'p=',r[1],'B=',r[2],'table=',r[3],'avgq=%.1f'%r[4],'solve=%.0f'%r[5],'uniform=%.2e'%r[6])
def sl(xs,ys):
    mx=sum(xs)/len(xs); my=sum(ys)/len(ys)
    return sum((x-mx)*(y-my) for x,y in zip(xs,ys))/sum((x-mx)**2 for x in xs)
xs=[math.log2(r[1]) for r in rows]
print('m=3 solve exponent =',round(sl(xs,[math.log2(r[5]) for r in rows]),3))
print('m=3 uniform exponent =',round(sl(xs,[math.log2(r[6]) for r in rows]),3))
