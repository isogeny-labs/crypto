#!/usr/bin/env python3
"""CT65 independently checks quotient Frobenius orbits and 31-torsion trace projection."""
import json
Q=1024
POLY=0x409
ell=31
def ck(b,msg):
    if not b: raise RuntimeError(msg)
def mul(a,b):
    z=0
    while b:
        if b&1:z^=a
        b>>=1;a<<=1
        if a&Q:a^=POLY
    return z&(Q-1)
def pw(a,n):
    z=1
    while n:
        if n&1:z=mul(z,a)
        a=mul(a,a);n>>=1
    return z
omega=next(x for x in range(2,Q) if mul(x,x)^x^1==0)
roots={}
for y in range(Q):roots.setdefault(mul(y,y)^y,[]).append(y)
def lift(r):
    r2=mul(r,r);r3=mul(r2,r);r4=mul(r2,r2)
    a=pw(r,Q//2);b=r4^mul(omega,r2)
    if (mul(b,b)^a)!=mul(omega,r):return None
    cc=roots.get(mul(r4,r)^mul(omega,r3))
    if cc is None:return None
    ck(len(cc)==2,"Artin-Schreier roots")
    return a,b,cc,mul(a,r2)^mul(b,r)
all_lifts={r:v for r in range(1,Q) if (v:=lift(r)) is not None}
involutions={r:v for r,v in all_lifts.items() if v[-1]==0}
ck(len(all_lifts)==15 and len(involutions)==5,"translation census")
ck(sorted(involutions)==[20,534,619,809,832],"involutive parameters")
cycle=[20];nxt=pw(20,4)
while nxt!=20:
    ck(nxt not in cycle,"Galois orbit does not close")
    cycle.append(nxt);nxt=pw(nxt,4)
ck(cycle==[20,832,619,809,534],"one five-cycle")
for r,(a,b,cs,flag) in involutions.items():
    for c in cs:
        start=(r,a,b,c);cur=start;seen=set()
        while cur not in seen:
            seen.add(cur)
            cur=tuple(pw(x,4) for x in cur)
        ck(len(seen)==5 and cur==start,"lift orbit")
        for rr,aa,bb,cc in seen:
            ck(rr in involutions and aa==involutions[rr][0] and
               bb==involutions[rr][1] and cc in involutions[rr][2],"Galois closure")
ck(tuple(pw(x,4**4) for x in (20,6,120,432))==(534,794,636,668),
   "selected CT64 quotients are conjugate")
def matmul(A,B):
    return [[sum(A[i][k]*B[k][j] for k in range(4))%ell
             for j in range(4)] for i in range(4)]
def add(A,B):
    return [[(A[i][j]+B[i][j])%ell for j in range(4)] for i in range(4)]
def rank(A):
    a=[row[:] for row in A];r=0
    for j in range(4):
        s=next((i for i in range(r,4) if a[i][j]),None)
        if s is None:continue
        a[s],a[r]=a[r],a[s]
        inv=pow(a[r][j],-1,ell)
        a[r]=[x*inv%ell for x in a[r]]
        for i in range(4):
            if i!=r:
                f=a[i][j]
                a[i]=[(a[i][k]-f*a[r][k])%ell for k in range(4)]
        r+=1
    return r
# Companion of Frobenius Weil polynomial X^4+2X^3+4X^2+8X+16.
F=[[0,0,0,-16],[1,0,0,-8],[0,1,0,-4],[0,0,1,-2]]
I=[[int(i==j) for j in range(4)] for i in range(4)]
S=[[0]*4 for _ in range(4)];H=I
for _ in range(5):
    S=add(S,H);H=matmul(H,F)
ck(H==I,"F^5=1 on J[31]")
P=[[25*x%ell for x in row] for row in S]
ck(matmul(P,P)==P and matmul(P,F)==P and matmul(F,P)==P,
   "Frobenius trace projector")
ck(rank(P)==1,"projector rank")
ck(rank(add(I,[[-x%ell for x in row] for row in P]))==3,"kernel rank")
eigen=sorted(x for x in range(ell) if
             (x**4+2*x**3+4*x**2+8*x+16)%ell==0)
ck(eigen==[1,4,8,16],"distinct Frobenius eigenvalues")
print(json.dumps({"omega":omega,"involutive_r":sorted(involutions),
"orbit_r20":cycle,"lift_orbits":2,"lift_orbit_length":5,
"ct64_quotients_conjugate":True,"frobenius_roots_mod31":eigen,
"frobenius_order_on_31torsion":5,"trace_projector_rank":rank(P),
"trace_projector_kernel_rank":3,"trace_projector_idempotent":True},
sort_keys=True,separators=(",",":")))
