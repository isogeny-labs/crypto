import random,sys,itertools
from sympy import symbols,Poly,div,rem,ZZ
random.seed(1)
MODP={1:0b11,2:0b111,3:0b1011,4:0b10011,5:0b100101,6:0b1000011,7:0b10000011,8:0b100011011}
def mul(a,b,k,m):
    r=0
    while b:
        if b&1:r^=a
        b>>=1;a<<=1
        if a>>k&1:a^=m
    return r
def tables(k):
    m=MODP[k];q=1<<k
    # log/exp via generator search
    for g in range(2,q):
        e=[1];x=1
        for i in range(q-2):
            x=mul(x,g,k,m);e.append(x)
        if len(set(e))==q-1:break
    if q==2:e=[1]
    lg={v:i for i,v in enumerate(e)}
    return e,lg
T={k:tables(k) for k in range(1,9)}
def pmul2(a,b):
    r=0
    while b:
        if b&1:r^=a
        b>>=1;a<<=1
    return r
def pmod2(a,b):
    db=b.bit_length()
    while a and a.bit_length()>=db: a^=b<<(a.bit_length()-db)
    return a
def pgcd(a,b):
    while b:a,b=b,pmod2(a,b)
    return a
def deriv(a): return sum(((a>>i)&1)<<(i-1) for i in range(1,a.bit_length()) if i%2==1)
def count(f,h,k):
    e,lg=T[k];q=1<<k;n1=q-1
    def ev(p,x):
        r=0;i=p.bit_length()-1
        while i>=0:
            r=0 if r==0 or x==0 else e[(lg[r]+lg[x])%n1] if x else 0
            if x==0:r=0
            if (p>>i)&1:r^=1
            i-=1
        return r
    # trace
    def tr(a):
        t=0;x=a
        for i in range(k):
            t^=x; x=0 if x==0 else e[(2*lg[x])%n1]
        return t
    N=1
    for x in range(q):
        hv=ev(h,x);fv=ev(f,x)
        if hv==0:N+=1
        else:
            inv2=e[(-2*lg[hv])%n1]
            z=0 if fv==0 else e[(lg[fv]+lg[inv2])%n1]
            N+= 2 if tr(z)==0 else 0
    return N
def Lpoly(Ns,g,q=2):
    # Newton: S_k = q^k+1-N_k ; L = prod(1-a_i T); reverse to charpoly
    S=[q**k+1-Ns[k-1] for k in range(1,g+1)]
    c=[1]
    for k in range(1,g+1):
        c.append(-sum(c[k-i]*S[i-1] for i in range(1,k+1))//k)
    a=c+[c[g-i]*q**(i) for i in range(g-1,-1,-1)] # a_{g+i}=q^i a_{g-i}
    a=c[:g+1]+[c[g-i]*q**i for i in range(1,g+1)]
    return a  # L(T)=sum a_i T^i ; charpoly x^{2g} L(1/x)
x=symbols('x')
from sympy import resultant
y,Tt=symbols('y Tt')
Ps={}
for t in (1,-1):
    Q=resultant(Tt**2-t*Tt+2,y-Tt**5,Tt).subs(y,x**5)
    Ps['K%+d'%t]=Poly(div(Poly(Q,x),Poly(x**2-t*x+2,x))[0],x)
for a in (3,7): Ps['a%d'%a]=Poly(x**10-a*x**5+32,x)
gmax=int(sys.argv[1]); 
for g in range(2,gmax+1):
    N=int(sys.argv[2]) if g<6 else int(sys.argv[3])
    hits={k:0 for k in Ps};ok=0;Ls=set();bad=0
    for s in range(N):
        h=random.getrandbits(g+1) or 1
        f=(1<<(2*g+1))|random.getrandbits(2*g+1)
        if pgcd(h, pmul2(deriv(f),deriv(f))^pmul2(pmul2(deriv(h),deriv(h)),f))!=1: continue
        ok+=1
        Ns=[count(f,h,k) for k in range(1,g+1)]
        a=Lpoly(Ns,g)
        cp=Poly(list(a),x) # charpoly coefficients: x^{2g}L(1/x) has leading a0
        Ls.add(tuple(a))
        for kname,P in Ps.items():
            if P.degree()<=2*g and rem(cp,P).is_zero: hits[kname]+=1
    print(g,N,ok,len(Ls),hits,flush=True)
