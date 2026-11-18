import random
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,trim,add,mul,square,power,modulo,roots_product
SQ=[sum(((i>>j)&1)<<(2*j)for j in range(8)).to_bytes(2,'little')for i in range(256)]

class Field(Arithmetic):
    def sq(self,a):
        if self.p!=2:return self.mul(a,a)
        v=int.from_bytes(b''.join(SQ[b]for b in a.to_bytes(max(1,(a.bit_length()+7)//8),'little')),'little')
        while v.bit_length()>self.n:v^=self.mask<<(v.bit_length()-self.n-1)
        return v
    def inv(self,a):
        if self.p!=2:return super().inv(a)
        if not a:raise ZeroDivisionError
        u,v,g,h=a,self.mask,1,0
        while u!=1:
            j=u.bit_length()-v.bit_length()
            if j<0:u,v=v,u;g,h=h,g;j=-j
            u^=v<<j;g^=h<<j
            if not u:raise ValueError('reducible field modulus')
        while g.bit_length()>self.n:g^=self.mask<<(g.bit_length()-self.n-1)
        return g

def monic(a,f):
    a=trim(a)
    if not a:return a
    iv=f.inv(a[-1]);return [f.mul(x,iv)for x in a]
def divrem(a,b,f):
    a=trim(a);b=trim(b);assert b;q=[0]*max(0,len(a)-len(b)+1);iv=f.inv(b[-1])
    while len(a)>=len(b):
        shift=len(a)-len(b);c=f.mul(a[-1],iv);q[shift]=c
        for i,x in enumerate(b):a[shift+i]=f.sub(a[shift+i],f.mul(c,x))
        a=trim(a)
    return trim(q),a
def gcd(a,b,f):
    while b:a,b=b,divrem(a,b,f)[1]
    return monic(a,f)
def product(a,b,f):
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):out[i+j]=f.add(out[i+j],f.mul(x,y))
    return trim(out)
def derivative(a,f):return trim([f.mul(a[i],i%f.p)for i in range(1,len(a))])
def radical(H,f):
    if len(H)<=2:return monic(H,f)
    D=derivative(H,f)
    if not D:
        assert all(not c for i,c in enumerate(H)if i%f.p)
        return radical([f.power(H[i],f.p**(f.n-1))for i in range(0,len(H),f.p)],f)
    G=gcd(H,D,f);W,rem=divrem(H,G,f);assert not rem
    if len(G)==1:return monic(H,f)
    R=radical(G,f);C=gcd(W,R,f);Q,rem=divrem(R,C,f);assert not rem
    return monic(product(W,Q,f),f)
def linear_part(H,f):
    if len(H)<=1:return [1]
    L=add(power([0,1],f.q,H,f),[0,f.neg(1)],f)
    return gcd(H,L,f)
def extract(R,f,seed,cap=128):
    rng=random.Random(seed);pending=[R]if len(R)>1 else[];roots=[]
    while pending:
        H=pending.pop();degree=len(H)-1
        if degree==1:roots.append(f.neg(H[0]));continue
        for attempt in range(f.n if f.p==2 else cap):
            if f.p==2:
                z=[0,1<<attempt];T=[]
                for _ in range(f.n):T=add(T,z,f);z=square(z,H,f)
                G=gcd(H,T,f)
            else:
                A=[rng.randrange(f.q)for _ in range(degree)];G=gcd(H,A,f)
                if len(G)in[1,len(H)]:G=gcd(H,add(power(A,(f.q-1)//2,H,f),[f.neg(1)],f),f)
            if 1<len(G)<len(H):
                Q,r=divrem(H,G,f);assert not r;pending.extend([G,Q]);break
        else:raise OverflowError('exact factor splitting cap; incomplete')
    return sorted(roots)
def delta_data(H,f):
    d,c,b,a,t,_=H;A=f.add(b,f.mul(t,a));B=f.add(d,f.mul(t,c));D=f.add(f.sq(B),f.add(f.mul(f.mul(A,B),a),f.mul(f.sq(A),c)))
    return A,B,D
def dispatch(H,f,seed):
    A,B,D=delta_data(H,f)
    if D:return extract(linear_part(H,f),f,seed),'squarefree'
    d,c,b,a,t,_=H
    if not A and not B:
        Q=[f.power(c,1<<(f.n-1)),f.power(a,1<<(f.n-1)),1]
        return sorted(set([t]+extract(linear_part(Q,f),f,seed))),'quadratic_squared'
    assert A;u=f.mul(B,f.inv(A));J=[f.add(b,f.mul(t,u)),f.add(a,u),t,1];assert product([u,0,1],J,f)==H
    return sorted(set([f.power(u,1<<(f.n-1))]+extract(linear_part(J,f),f,seed))),'cubic_repeated'
def solve(H,f,method,seed):
    assert H[-1]==1 and len(H)==6
    if method=='dispatch'and f.p==2:roots,branch=dispatch(H,f,seed)
    else:
        base=radical(H,f)if method=='radical'else H;roots=extract(linear_part(base,f),f,seed);branch=method
    result=[]
    for x in roots:
        a=list(H);multiplicity=0
        while len(a)>1:
            q,r=divrem(a,[f.neg(x),1],f)
            if r:break
            multiplicity+=1;a=q
        assert multiplicity>0;result.append([x,multiplicity])
    assert roots==sorted(set(roots))
    return {'roots':result,'complete':True,'branch':branch}

def binary_irreducible(f):
    assert f.p==2
    def rem(a,b):
        while a.bit_length()>=b.bit_length():a^=b<<(a.bit_length()-b.bit_length())
        return a
    def pgcd(a,b):
        while b:a,b=b,rem(a,b)
        return a
    factors=[];n=f.n;j=2
    while j*j<=n:
        if n%j==0:
            factors.append(j)
            while n%j==0:n//=j
        j+=1
    if n>1:factors.append(n)
    checks={f.n//p for p in factors};x=rem(2,f.mask);v=x
    for k in range(1,f.n+1):
        v=f.sq(v)
        if k in checks and pgcd(v^x,f.mask)!=1:return False
    return v==x
