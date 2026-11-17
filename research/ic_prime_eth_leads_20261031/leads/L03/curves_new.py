# Find 3 prime-order short-Weierstrass curves per bit size (pure numpy; Legendre point count).
import numpy as np, random
random.seed(20261103)
def isprime(n):
    if n<2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n%q==0: return n==q
    d=n-1;s=0
    while d%2==0: d//=2;s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x=pow(a,d,n)
        if x in(1,n-1): continue
        for _ in range(s-1):
            x=x*x%n
            if x==n-1:break
        else:return False
    return True
out=open('curves_new.txt','w')
for b in (16,20):
    got=0
    while got<3:
        p=random.randint(2**(b-1)+2**(b-2),2**b-1)
        if not isprime(p): continue
        y=np.arange(p,dtype=np.int64)
        sq=np.zeros(p,dtype=np.int8); sq[(y*y)%p]=1
        chi=np.where(sq==1,1,-1).astype(np.int64); chi[0]=0
        x=y
        x3=(x*x%p)*x%p
        for _ in range(400):
            a=random.randint(1,p-1); bb=random.randint(1,p-1)
            if (4*pow(a,3,p)+27*bb*bb)%p==0: continue
            f=(x3+a*x+bb)%p
            r=p+1+int(chi[f].sum())
            if isprime(r) and r!=p:
                xs=np.nonzero((chi[f]==1))[0]
                x0=int(xs[0]); f0=int(f[x0]); y0=int(np.nonzero((y*y)%p==f0)[0][0])
                out.write(f"{b} {p} {a} {bb} {r} {x0} {y0}\n"); got+=1; break
out.close()
