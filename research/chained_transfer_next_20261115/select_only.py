"""Selection-only check for O4 Amendment 1 (R2). No arm, no rho, no timing.
Implements the frozen selection rule with the amendment's changes:
p = 1 mod 4 ladder, hop to next prime = 1 mod 4, a != 0, b != 0 (R15), r != p, embedding degree > 6,
list index i over stage 1 then stage 2 (0..9)."""
import math
M=(1<<64)-1
def mix64(z):
    z=(z+0x9E3779B97F4A7C15)&M
    z=((z^(z>>30))*0xBF58476D1CE4E5B9)&M
    z=((z^(z>>27))*0x94D049BB133111EB)&M
    return z^(z>>31)
def isprime(n):
    if n<2: return False
    if n%2==0: return n==2
    i=3
    while i*i<=n:
        if n%i==0: return False
        i+=2
    return True
def nextp1(q):
    q+=1
    while not (isprime(q) and q%4==1): q+=1
    return q
ladder=[229,421,821,1621,3209,6421,12841,25601,51217,102409]
out=[]
for i,p0 in enumerate(ladder):
    p=p0; hops=0; found=None; passed=[]
    while hops<=20:
        assert p%4==1
        sq=bytearray(p); 
        for v in range(1,p): sq[v*v%p]=1
        q4=bytearray(p)
        for v in range(1,p): q4[pow(v,4,p)]=1
        for s in range(4000):
            a=mix64(0xA4B1+101*i+s)%p; b=mix64(0xB7C3+101*i+s)%p
            if a==0 or b==0: continue
            if (4*a**3+27*b*b)%p==0: continue
            cnt=1; n=0; nroot=0
            for x in range(p):
                f=(x*x*x+a*x+b)%p
                if f==0: cnt+=1; nroot+=1
                elif sq[f]:
                    cnt+=2
                    if q4[f]: n+=1
            r=cnt
            if not isprime(r): continue
            if r < math.ceil(0.85*p): continue
            if r==p: continue  # anomalous (Smart)
            if any((pow(p,k,r)-1)%r==0 for k in range(1,7)): continue  # MOV degree <= 6
            ptsC=1+4*n+nroot
            if ptsC<8: continue
            found=(s,a,b,r,n,ptsC); break
        if found: break
        passed.append(p); p=nextp1(p); hops+=1
    s,a,b,r,n,ptsC=found
    print(f"{i}\t{p0}\t{p}\t{passed}\t{s}\t{a}\t{b}\t{r}\t{r/p:.3f}\t{ptsC}\t{n}\t{n/p:.4f}\t{(math.log(n))/4:.2f}", flush=True)
