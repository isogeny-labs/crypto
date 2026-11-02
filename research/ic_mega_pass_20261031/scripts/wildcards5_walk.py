import random,sys,json
POLY={17:(1<<17)|(1<<3)|1,19:(1<<19)|(1<<5)|(1<<2)|(1<<1)|1,21:(1<<21)|(1<<2)|1,23:(1<<23)|(1<<5)|1}
def mul(a,b,n,P):
    r=0
    while b:
        if b&1:r^=a
        b>>=1;a<<=1
        if a>>n&1:a^=P
    return r
def inv(a,n,P):
    r=1;e=(1<<n)-2
    while e:
        if e&1:r=mul(r,a,n,P)
        a=mul(a,a,n,P);e>>=1
    return r
def run(n,steps,seed):
    random.seed(seed);P=POLY[n];a2=1;a6=random.randrange(1,1<<n)
    def add(p,q):
        if p is None:return q
        if q is None:return p
        x1,y1=p;x2,y2=q
        if x1==x2:
            if y1^y2==x1 or x1==0:return None if (y1^y2)==x2 or x1==0 else None
            l=x1^mul(y1,inv(x1,n,P),n,P)
            x3=mul(l,l,n,P)^l^a2
            return (x3,mul(x1,x1,n,P)^mul(l^1,x3,n,P))
        l=mul(y1^y2,inv(x1^x2,n,P),n,P)
        x3=mul(l,l,n,P)^l^x1^x2^a2
        return (x3,mul(l,x1^x3,n,P)^x3^y1)
    def rp():
        while True:
            x=random.randrange(1,1<<n)
            # solve y^2+xy=x^3+a2x^2+a6 by brute trace? use half-trace: z^2+z=c, c=(x^3+a2x^2+a6)/x^2
            c=mul(x^1^0,1,n,P)^a2^mul(a6,inv(mul(x,x,n,P),n,P),n,P)
            t=c;s=c
            for _ in range(n-1):s=mul(s,s,n,P);t^=s
            if t&1==0 and t in (0,):pass
            # trace = t (0 or 1 in field)
            if t!=0:continue
            h=c;s=c
            for _ in range((n-1)//2):s=mul(mul(s,s,n,P),mul(s,s,n,P),n,P) if False else mul(mul(s,s,n,P),1,n,P);s=mul(s,s,n,P);h^=s
            z=h
            if mul(z,z,n,P)^z!=c:continue
            return (x,mul(x,z,n,P))
    l=n//2;R=[rp() for _ in range(20)]
    Vs=[]
    for _ in range(8):
        # random subspace dim l: kernel test via random (n-l) linear functionals
        Vs.append([random.randrange(1,1<<n) for _ in range(n-l)])
    def inV(x,F):return all(bin(x&f).count('1')%2==0 for f in F)
    p=rp();hits=[0]*8;cnt=0
    for i in range(steps):
        p=add(p,R[p[0]%20]) if p else rp()
        if p is None:p=rp();continue
        cnt+=1
        for j,F in enumerate(Vs):hits[j]+=inV(p[0],F)
    exp=cnt*2.0**(l-n)
    return dict(n=n,l=l,steps=cnt,expected=exp,hits=hits,ratio=[round(h/exp,4) for h in hits],sigma_rel=round(exp**-0.5,4))
out=[run(n,int(sys.argv[1]),n) for n in (17,19,21,23)]
print(json.dumps(out))
