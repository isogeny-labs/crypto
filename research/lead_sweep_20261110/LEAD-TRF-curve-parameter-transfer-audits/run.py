import sys, json, random, math
def isprime(n):
    if n<2: return False
    i=2
    while i*i<=n:
        if n%i==0: return False
        i+=1
    return True
def primes_in(lo,hi): return [n for n in range(lo,hi) if isprime(n)]
def factor_max(n):
    m=1;i=2
    while i*i<=n:
        while n%i==0: m=i;n//=i
        i+=1
    return max(m,n) if n>1 else m
def order(p,a,b,sq):
    # #E = p+1+sum chi(x^3+ax+b)
    s=0
    for x in range(p):
        v=(x*x*x+a*x+b)%p
        s+=sq[v]
    return p+1+s
def sqtab(p):
    t=[-1]*p; t[0]=0
    for y in range(1,p): t[y*y%p]=1
    return t
def phi(d): return sum(1 for i in range(1,d+1) if math.gcd(i,d)==1)
KMAX=6
def embdeg(p,r,kmax=KMAX):
    if r<=1: return 1
    v=1
    for k in range(1,kmax+1):
        v=v*p%r
        if v==1: return k
    return None
def exp_rate(r):
    return sum(phi(d) for d in range(1,KMAX+1) if (r-1)%d==0)/(r-1) if r>2 else 1.0
def arm(seed,plist,ncurves):
    rng=random.Random(seed); hits=0; exp=0.0; anom=0; n=0
    for _ in range(ncurves):
        p=rng.choice(plist); sq=sqtab(p)
        while True:
            a=rng.randrange(p); b=rng.randrange(p)
            if (4*a**3+27*b*b)%p: break
        N=order(p,a,b,sq); r=factor_max(N)
        n+=1; exp+=exp_rate(r)
        if N==p: anom+=1
        elif embdeg(p,r) is not None and r!=p: hits+=1
    return dict(n=n,hits=hits,expected=exp,anomalous=anom)
def positive(plist):
    # supersingular y^2=x^3+x, p=3 mod 4: #E=p+1, k<=2 ; must be detected
    ok=tot=0
    for p in plist:
        if p%4!=3: continue
        sq=sqtab(p); N=order(p,1,0,sq); r=factor_max(N); tot+=1
        ok+= (N==p+1 and embdeg(p,r) is not None and embdeg(p,r)<=2)
    return ok,tot
def main():
    smoke='--smoke' in sys.argv
    lo,hi,nc,npos=(200,400,40,10) if smoke else (1500,2500,1200,60)
    plist=primes_in(lo,hi)
    A=arm(20261110,plist,nc); B=arm(20261111,plist,nc)
    pos=positive(plist[:npos] if smoke else plist[:npos*3])
    tot_h=A['hits']+B['hits']; tot_e=A['expected']+B['expected']
    z=(tot_h-tot_e)/math.sqrt(max(tot_e,1.0))
    aa=(A['hits']-B['hits'])/math.sqrt(max(A['hits']+B['hits'],1))
    if pos[0]!=pos[1] or abs(aa)>3: v="inconclusive"
    elif z>3: v="falsified"
    else: v="consistent_with_prediction"
    print(json.dumps(dict(statistic="z=(hits-expected)/sqrt(expected), hits=random curves with 1<k<=6 on largest prime factor r of #E",
      z=z,aa_z=aa,arm_A=A,arm_B=B,positive_control_detected_vs_total=list(pos),
      seeds=[20261110,20261111],params=dict(p_range=[lo,hi],curves_per_arm=nc,kmax=KMAX,smoke=smoke),verdict=v)))
main()
