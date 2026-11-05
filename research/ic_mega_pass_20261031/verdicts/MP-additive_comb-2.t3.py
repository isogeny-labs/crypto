import random,math,json,sys
from sympy import nextprime, primitive_root, isprime
def sqrtmod(a,p):
    from sympy.ntheory import sqrt_mod
    return sqrt_mod(a,p)
def run(p,Hs,a,b):
    g=primitive_root(p)
    # subgroup of size Hs dividing p-1
    h=pow(g,(p-1)//Hs,p); H=set(); x=1
    for _ in range(Hs): H.add(x); x=x*h%p
    def F(S):
        X=[x for x in S if pow((x**3+a*x+b)%p,(p-1)//2,p)==1]
        Xs=set(X); T=0
        for i,x1 in enumerate(X):
            for x2 in X[i+1:]:
                # S3 quadratic in x3: A x3^2 + Bc x3 + C
                A=(x1-x2)**2%p; Bc=(-2*((x1+x2)*(x1*x2+a)+2*b))%p; C=((x1*x2-a)**2-4*b*(x1+x2))%p
                D=(Bc*Bc-4*A*C)%p
                if D==0: r=[(-Bc*pow(2*A,-1,p))%p]
                elif pow(D,(p-1)//2,p)!=1: r=[]
                else:
                    s=sqrtmod(D,p); inv=pow(2*A,-1,p); r=[(-Bc+s)*inv%p,(-Bc-s)*inv%p]
                for x3 in r:
                    if x3 in Xs and x3!=x1 and x3!=x2: T+=1
        return len(X),T
    BH,TH=F(H)
    R=random.Random(1).sample(range(1,p),Hs); BR,TR=F(R)
    BI,TI=F(range(1,Hs+1))
    return dict(p=p,H=Hs,a=a,b=b,BH=BH,TH=TH,BR=BR,TR=TR,BI=BI,TI=TI,mu=round(BH**2*Hs/p,3)) # x-triples ~ Bx^2*(|S|/p)
out=[]
random.seed(0)
for p0,dl in [(2**20,0.3),(2**20,0.4),(2**22,0.35),(2**22,0.45)]:
    # find prime p with divisor near p^dl
    p=nextprime(p0)
    while True:
        t=int(p**dl); ok=[d for d in range(t-40,t+40) if d>2 and (p-1)%d==0]
        if ok: break
        p=nextprime(p)
    for c in range(3):
        a,b=random.randrange(p),random.randrange(p)
        out.append(run(p,ok[0],a,b))
    out.append(run(p,ok[0] if ok[0]%3==0 else ok[0],0,random.randrange(p)))
print(json.dumps(out))
