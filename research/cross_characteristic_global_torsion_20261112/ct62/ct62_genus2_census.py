#!/usr/bin/env python3
"""CT62 binary genus-two Jacobian order census over F2/F4.

Models: y^2+h(x)y=f(x), deg h<=3, deg f<=6, normalized leading
terms and infinity constant; infinity may be split, ramified, or nonsplit.
Counts C(Fq), C(Fq²) and independently enumerates reduced Mumford pairs
for odd-degree models. This is NOT a DLP transfer.
"""
from collections import Counter
from itertools import product
import json

def require(condition, context=None):
    if not condition: raise AssertionError(context)

def field(poly, deg):
    n=1<<deg
    mul=[[0]*n for _ in range(n)]
    for a in range(n):
        for b in range(n):
            aa,bb=a,b; z=0
            while bb:
                if bb&1:z^=aa
                bb>>=1;aa<<=1
                if aa&n:aa^=poly
            mul[a][b]=z
    inv=[0]*n
    for a in range(1,n):
        inv[a]=next(b for b in range(1,n) if mul[a][b]==1)
    return mul,inv

M2,I2=field(0b11,1)
M4,I4=field(0b111,2)
M16,I16=field(0b10011,4)
W=next(a for a in range(2,16) if M16[a][a]^a^1==0)
EMBED=[0,1,W,W^1]
require(len(set(EMBED))==4)
require(all(EMBED[M4[a][b]]==M16[EMBED[a]][EMBED[b]] for a in range(4) for b in range(4)))

def trim(p):
    while p and p[-1]==0:p.pop()
    return p

def rem_poly(p,q,mul,inv):
    p=trim(list(p));q=trim(list(q))
    if not q:raise ValueError('zero divisor')
    d=len(q)-1; qc=inv[q[-1]]
    while len(p)>=len(q):
        factor=mul[p[-1]][qc]
        shift=len(p)-len(q)
        for j in range(d+1):p[shift+j]^=mul[factor][q[j]]
        trim(p)
    return p

def gcd_degree(p,q,mul,inv):
    p=trim(list(p));q=trim(list(q))
    while q:p,q=q,rem_poly(p,q,mul,inv)
    return len(p)-1

def smooth(f,h,mul,inv):
    if not any(h):return False
    # Affine singularity criterion gcd(h, f'^2 + h'^2*f) = 1.
    fp=[f[1],0,f[3],0,f[5]]
    hp=[h[1],0,h[3]]
    a=[0]*(2*len(fp)-1)
    for j,v in enumerate(fp):a[2*j]=mul[v][v]
    b=[0]*(2*len(hp)-1)
    for j,v in enumerate(hp):b[2*j]=mul[v][v]
    B=[0]*max(len(a),len(b)+len(f)-1)
    for j,v in enumerate(a):B[j]^=v
    for i,v in enumerate(b):
        for j,w in enumerate(f):B[i+j]^=mul[v][w]
    return gcd_degree(h,B,mul,inv)==0

def tables(mul):
    n=len(mul)
    powers=[]
    for x in range(n):
        r=[1]
        for _ in range(6):r.append(mul[r[-1]][x])
        powers.append(r)
    sol=[[0]*n for _ in range(n)]
    for h in range(n):
        for y in range(n):
            y2=mul[y][y]
            for rhs in range(n):
                if y2^mul[h][y]==rhs:sol[h][rhs]+=1
    return powers,sol

POWERS={2:tables(M2),4:tables(M4),16:tables(M16)}

def npoints(f,h,mul,embed):
    n=len(mul);powers,sol=POWERS[n]
    # h3!=0: infinity satisfies Y²+h3Y=f6 (0 or 2 rational points).
    # h3==0 and f6=0,f5=1: one ramified rational infinity.
    ff=[embed[a] for a in f];hh=[embed[a] for a in h]
    if hh[3]:
        ans=sum(1 for y in range(n) if mul[y][y]^mul[hh[3]][y]==ff[6])
    else:
        ans=1
    for x in range(n):
        p=powers[x]
        rhs=0
        for j in range(7):rhs^=mul[ff[j]][p[j]]
        hx=0
        for j in range(4):hx^=mul[hh[j]][p[j]]
        ans+=sol[hx][rhs]
    return ans

def jac_order(q,n1,n2):
    s1=q+1-n1;s2=q*q+1-n2
    require((s1*s1-s2)%2==0)
    a1=-s1;a2=(s1*s1-s2)//2
    return q*q+1+(q+1)*a1+a2

def mumford_count(f,h,mul,inv):
    require(len(h)==4 and h[3]==0 and f[5]==1 and f[6]==0)
    n=len(mul);count=1
    by_type=Counter()
    for d in (1,2):
        for coeff in product(range(n),repeat=d):
            u=list(coeff)+[1]
            # For u=(x-r)^2 exclude doubled branch points h(r)=0.
            repeated=(d==2 and u[1]==0)
            if repeated:
                root=next(r for r in range(n) if mul[r][r]==u[0])
                H=mul[h[2]][mul[root][root]]^mul[h[1]][root]^h[0]
                if H==0:continue
            for v_coeff in product(range(n),repeat=d):
                v=list(v_coeff)
                vv=[0]*(2*d-1)
                for i,a in enumerate(v):
                    for j,b in enumerate(v):vv[i+j]^=mul[a][b]
                hv=[0]*(d+3)
                for i,a in enumerate(h):
                    for j,b in enumerate(v):hv[i+j]^=mul[a][b]
                expr=[0]*max(len(f),len(vv),len(hv))
                for i,a in enumerate(vv):expr[i]^=a
                for i,a in enumerate(hv):expr[i]^=a
                for i,a in enumerate(f):expr[i]^=a
                if rem_poly(expr,u,mul,inv):continue
                count+=1
                if d==1:tp='degree1'
                elif repeated:tp='repeated_unramified'
                else:
                    roots=sum(1 for r in range(n) if mul[r][r]^mul[u[1]][r]^u[0]==0)
                    tp='nonsplit_degree2' if roots==0 else 'split_degree2'
                by_type[tp]+=1
    return count,dict(sorted(by_type.items()))

def prime_divisors(n):
    out=[];d=2
    while d*d<=n:
        if n%d==0:
            out.append(d)
            while n%d==0:n//=d
        d+=1
    if n>1:out.append(n)
    return out

def sweep(q):
    m,iv=(M2,I2) if q==2 else (M4,I4)
    extmul=(M4 if q==2 else M16)
    embed=list(range(2)) if q==2 else EMBED
    stats=Counter();orders=Counter();prime_count=Counter();rep={};repprime={};by_n1n2=Counter();odd_reps={}
    # f6=0 / h3²*delta select split/nonsplit infinity classes.
    # F4 is deliberately restricted to odd-degree h3=0, f5=1.
    delta=1 if q==2 else 2
    for hs in product(range(q),range(q),range(q),range(q) if q==2 else (0,)):
        h=list(hs)
        for f6 in ([0] if h[3]==0 else [0,m[m[h[3]][h[3]]][delta]]):
          for cs in product(range(q),repeat=5 if h[3]==0 else 6):
            f=list(cs)+([1,0] if h[3]==0 else [f6])
            if not smooth(f,h,m,iv):continue
            n1=npoints(f,h,m,list(range(q)))
            n2=npoints(f,h,extmul,embed)
            J=jac_order(q,n1,n2)
            require((q**.5-1)**4 - 1e-9 <= J <= (q**.5+1)**4 + 1e-9,(q,hs,cs,J))
            stats['smooth']+=1
            stats['one_infinity' if h[3]==0 else ('two_rational_infinities' if f6==0 else 'nonsplit_infinities')]+=1
            if h[3]==0:odd_reps.setdefault(J,(f,h,n1,n2))
            orders[J]+=1
            by_n1n2[(n1,n2)]+=1
            rep.setdefault(J,(f,h,n1,n2))
            for ell in prime_divisors(J):
                prime_count[ell]+=1
                repprime.setdefault(ell,(f,h,n1,n2,J))
    checks=[]
    for J in sorted(odd_reps):
        f,h,n1,n2=odd_reps[J]
        direct,cats=mumford_count(f,h,m,iv)
        require(J==direct,(q,J,direct,f,h,cats))
        checks.append({'J':J,'N1':n1,'N2':n2,'f':f[:-1],'h':h,'mumford':direct,'classes':cats})
    for J,(f,h,n1,n2) in odd_reps.items():
        direct,_=mumford_count(f,h,m,iv)
        require(direct==J,(q,J,direct,f,h))
    return {
      'q':q,'models_total':q**8+(2*(q-1)*q**9 if q==2 else 0),
      'smooth':stats['smooth'],'infinity_types':{k:v for k,v in stats.items() if k!='smooth'},
      'distinct_orders':len(orders),'min_J':min(orders),'max_J':max(orders),
      'orders':dict(sorted(orders.items())),
      'prime_divisors':dict(sorted(prime_count.items())),
      'first_witness_by_prime':{str(p):{'f':v[0], 'h':v[1],'N1':v[2],'N2':v[3],'J':v[4]} for p,v in sorted(repprime.items())},
      'direct_crosschecks':checks,'distinct_zeta_signatures':len(by_n1n2),
      'odd_degree_J_representatives_directly_recounted':len(odd_reps),
    }

def main():
    a=sweep(2);b=sweep(4)
    print(json.dumps({'test':'ct62_binary_genus2_complete_model_census',
      'field':{'F4_irreducible':0b111,'F16_irreducible':0b10011,'F4_in_F16_witness':W},
      'results':[a,b]},sort_keys=True,separators=(',',':')))

if __name__=='__main__':main()
