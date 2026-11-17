import json, random, sys, statistics
def is_prime(n):
    if n<2: return False
    i=2
    while i*i<=n:
        if n%i==0: return False
        i+=1
    return True
def curve_points(p,a,b):
    sq={}
    for y in range(p): sq.setdefault(y*y%p,[]).append(y)
    pts=[]
    for x in range(p):
        for y in sq.get((x**3+a*x+b)%p,[]): pts.append((x,y))
    return pts
def add(P,Q,p,a):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P;x2,y2=Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else: l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p
    return (x3,(l*(x1-x3)-y1)%p)
def coverage(fb,p,a,N):
    s=set()
    for i,P in enumerate(fb):
        for Q in fb[i:]:
            R=add(P,Q,p,a)
            if R is not None: s.add(R)
    return len(s)/N
def run(p,ncurves,K,nnull,seed,fbfrac,nrand):
    rng=random.Random(seed)
    out=[];curves=[]
    while len(curves)<ncurves:
        a=rng.randrange(1,p);b=rng.randrange(1,p)
        if (4*a**3+27*b*b)%p==0: continue
        pts=curve_points(p,a,b);N=len(pts)+1
        if not is_prime(N) or N<p//2: continue  # prime group order only
        curves.append((a,b,pts,N))
        if len(curves)>=ncurves: break
        if rng.random()<2: pass
    # tolerate small count when primes are rare: relax
    return curves
def stat_for(curves,p,K,B,rng,mode,nrand):
    # mode: 'obs' (isomorphic models, small-x set), 'null' (random same-size subset), 'pos' (multiples of G)
    gains=[]
    for (a,b,pts,N) in curves:
        cov=[];size=[]
        for u in range(1,K+1):
            if mode=='obs':
                m=u*u%p; fb=[P for P in pts if (P[0]*m)%p<B]
            elif mode=='null':
                n=sum(1 for P in pts if (P[0]*(u*u%p))%p<B); fb=rng.sample(pts,n)
            else:
                n=sum(1 for P in pts if (P[0]*(u*u%p))%p<B)
                G=pts[0];fb=[];Q=None
                for _ in range(n):
                    Q=add(Q,G,p,a)
                    if Q: fb.append(Q)
            cov.append(coverage(fb,p,a,N));size.append(len(fb))
        # size-normalised yield: coverage per pair, divided by mean over models
        y=[c/max(1,s*(s+1)/2) for c,s in zip(cov,size)]
        gains.append(max(y)/statistics.mean(y)-1)
    return statistics.mean(gains)
def main():
    smoke='--smoke' in sys.argv
    if smoke: p,nc,K,nn=211,2,4,3
    else: p,nc,K,nn=2027,6,12,20
    B=p//16
    rng=random.Random(20261110)
    curves=[]
    while len(curves)<nc:
        a=rng.randrange(1,p);b=rng.randrange(1,p)
        if (4*a**3+27*b*b)%p==0: continue
        pts=curve_points(p,a,b);N=len(pts)+1
        if smoke or is_prime(N) or N%2==1 and not curves and False: pass
        if not smoke and not is_prime(N): continue
        curves.append((a,b,pts,N))
    obs=stat_for(curves,p,K,B,random.Random(1),'obs',0)
    pos=stat_for(curves,p,K,B,random.Random(2),'pos',0)
    null=[stat_for(curves,p,K,B,random.Random(100+i),'null',0) for i in range(nn)]
    aa=[stat_for(curves,p,K,B,random.Random(900+i),'null',0) for i in range(max(3,nn//3))]
    null.sort(); q95=null[int(0.95*(len(null)-1))]; q99=null[int(0.99*(len(null)-1))]
    # positive control: structured FB (multiples of G) must shift mean yield far below random
    def mean_yield(mode):
        r=random.Random(7);ys=[]
        for (a,b,pts,N) in curves:
            n=sum(1 for P in pts if P[0]<B)
            fb=r.sample(pts,n) if mode=='r' else None
            if mode=='g':
                G=pts[0];Q=None;fb=[]
                for _ in range(n):
                    Q=add(Q,G,p,a)
                    if Q: fb.append(Q)
            ys.append(coverage(fb,p,a,N)/max(1,len(fb)*(len(fb)+1)/2))
        return statistics.mean(ys)
    yr,yg=mean_yield('r'),mean_yield('g')
    posok=yg<0.5*yr
    verdict="inconclusive" if not posok else ("falsified" if obs>q99 else "consistent_with_prediction")
    print(json.dumps({"lead":"LEAD-STR-class-representative-ic-selection","smoke":smoke,"p":p,"curve_orders":[c[3] for c in curves],
      "K_models":K,"B":B,"seeds":{"curves":20261110,"obs":1,"null_base":100,"aa_base":900},
      "statistic_obs_best_of_K_gain":obs,"null_q95":q95,"null_q99":q99,"null_mean":statistics.mean(null),
      "aa_mean":statistics.mean(aa),"aa_range":[min(aa),max(aa)],"positive_control_yield_random":yr,"positive_control_yield_multiples":yg,
      "positive_control_detected":posok,"verdict":verdict}))
main()
