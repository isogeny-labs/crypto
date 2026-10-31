# E2 driver: selection grid (seed 1, 32 targets) then eval of selected recipes (seeds 2..4, 64 targets).
import subprocess, random, os, sys, json
E='/tmp/claude-501/e2' if not os.environ.get('TMPDIR') else os.environ['TMPDIR']+'/e2'
R={41:549756390943,51:17192986687,53:21044858204113,57:275295876199,59:10063074221,61:162888033982417}
LADDER=[2,3,4,6,8,12,16,24,32,48,64,96,128,192]
CAP=3e7
def B(n,K,sym):
    W=10*K
    return W*(W+1)*n/2 if sym else W*W*n
def targets(n,seed,T):
    p=f'{E}/targets/n{n}_s{seed}_{T}.txt'
    if not os.path.exists(p):
        rng=random.Random(f'e2-{n}-{seed}')
        open(p,'w').write('\n'.join(str(rng.randrange(1,R[n])) for _ in range(T))+'\n')
    return p
def run(n,K,norm,sym,seed,T):
    tag=f'n{n}_K{K}_N{norm}_T{T}_s{sym}_r{seed}'
    out=f'{E}/runs/{tag}'
    if os.path.exists(out+'.stdout') and os.path.getsize(out+'.stdout')>0 and open(out+'.stdout').read().strip().endswith('}'): return tag
    env=dict(os.environ,KIC_NORM=str(norm),KIC_PAIR_SYMMETRY=str(sym))
    with open(out+'.stdout','w') as so, open(out+'.stderr','w') as se:
        subprocess.run([f'{E}/kodf',f'construct:{n}:0:{K}',targets(n,seed,T),str(seed),out+'.jsonl'],env=env,stdout=so,stderr=se)
    print('done',tag,flush=True); return tag
sizes=[int(x) for x in sys.argv[2].split(',')]
phase=sys.argv[1]
for n in sizes:
    if phase=='sel':
        for sym in (0,1):
            for K in LADDER:
                if B(n,K,sym)<=CAP: run(n,K,16,sym,1,32)
if phase=='eval':
    sys.path.insert(0,E); import analyse as A
    sel={}
    for n in sizes:
        best={}
        for sym in (0,1):
            c=[]
            for K in LADDER:
                tag=f'n{n}_K{K}_N16_T32_s{sym}_r1'
                try: s,recs=A.load(E+'/runs',tag)
                except Exception: continue
                s0,d,_=A.costs(n,s,recs); c.append((s0+d if sym==0 else s0*d,K))
            best[sym]=min(c)[1]
        sel[n]=best
        print('SEL',n,best,flush=True)
        recipes=[(best[0],16,0),(best[1],16,1)]
        orig={41:[(12,16,0),(96,16,1)],53:[(20,16,0),(80,16,1)],61:[(18,32,0),(112,16,1)]}
        recipes+=orig.get(n,[])
        for seed in (2,3,4):
            for (K,N,sym) in dict.fromkeys(recipes): run(n,K,N,sym,seed,64)
    json.dump(sel,open(E+'/selection.json','w'))
if phase=='evalb':
    import math
    for n in sizes:
        W=math.sqrt(0.6*math.sqrt(R[n])/n); K=max(1,round(W/10)); print('RULEK',n,K,flush=True)
        for seed in (1,2,3,4):
            run(n,K,16,0,seed,32 if seed==1 else 64)
