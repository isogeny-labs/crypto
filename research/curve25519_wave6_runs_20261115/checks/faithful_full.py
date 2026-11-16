import sys, numpy as np
def is_prime(n):
    if n<2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n%q==0: return n==q
    d,s=n-1,0
    while d%2==0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x=pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x=x*x%n
            if x==n-1: break
        else: return False
    return True
def shape_ok(p,k):
    if p%8!=5 or (p-1)%3!=0 or (p+1)%4==0: return None
    m=(p-1)//12
    if m%2==0 or m%3==0: return None
    q=5; lim=2**(k/8)
    while q<lim:
        if m%q==0:
            C=m//q
            if is_prime(q) and is_prime(C) and C>q: return (q,C)
        q+=2
    return None
def curve(p,k,amax=40000):
    x=np.arange(p,dtype=np.int64); sq=np.zeros(p,dtype=bool); y=np.arange(1,(p+1)//2,dtype=np.int64); sq[(y*y)%p]=True
    x2=(x*x)%p; x3=(x2*x)%p; A=6
    while A<amax:
        if pow((A*A-4)%p,(p-1)//2,p)!=1:
            v=(x3+(A*x2)%p+x)%p; n=1+int(np.count_nonzero(v==0))+2*int(np.count_nonzero(sq[v])); nt=2*p+2-n
            if n%8==0 and nt%4==0 and is_prime(n//8) and is_prime(nt//4) and n//8>(1<<(k-3)): return (A,n//8,nt//4)
        A+=4
    return None
k=int(sys.argv[1]); want=int(sys.argv[2]); c=1; hits=0
while c<200000 and hits<want:
    p=2**k-c
    if is_prime(p):
        s=shape_ok(p,k)
        if s:
            cv=curve(p,k)
            print(f"k={k} p=2^{k}-{c}={p} p-1=4*3*{s[0]}*{s[1]} curve={cv}", flush=True)
            if cv: hits+=1
    c+=2
