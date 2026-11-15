#!/usr/bin/env python3
"""CT77: exhaust 32 binary Weierstrass models, CRT lifts and torsion fields."""
import itertools, json

ORIGINAL=(0,-1,1,-10,-20)
FIXTURES=((3,29),(5,29),(7,37),(11,59),(13,139))

def check(ok,where):
    if not ok: raise ArithmeticError(where)

def disc(a):
    a1,a2,a3,a4,a6=a
    b2=a1*a1+4*a2
    b4=2*a4+a1*a3
    b6=a3*a3+4*a6
    b8=a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    return -b2*b2*b8-8*b4**3-27*b6*b6+9*b2*b4*b6

def count_f2(a):
    a1,a2,a3,a4,a6=a
    return 1+sum((y*y+a1*x*y+a3*y-x**3-a2*x*x-a4*x-a6)%2==0
                 for x in range(2) for y in range(2))

def crt(a,p):
    return tuple(o+((bit-o)%2)*p for o,bit in zip(ORIGINAL,a))

def points_from_trace(trace,f):
    t0,t1=2,trace
    for i in range(1,f): t0,t1=t1,trace*t1-2*t0
    return (1<<f)+1-t1

def matmul(A,B,l):
    return ((A[0]*B[0]+A[1]*B[2])%l,(A[0]*B[1]+A[1]*B[3])%l,
            (A[2]*B[0]+A[3]*B[2])%l,(A[2]*B[1]+A[3]*B[3])%l)

def degree_gate(t,ell):
    M=(1,0,0,1);F=(0,(-2)%ell,1,t%ell)
    fs=ff=None
    for f in range(1,512):
        M=matmul(M,F,ell)
        det=((M[0]-1)*(M[3]-1)-M[1]*M[2])%ell
        check(det==points_from_trace(t,f)%ell,'trace matrix mismatch')
        if fs is None and det==0: fs=f
        if M==(1,0,0,1): ff=f;break
    check(fs is not None and ff is not None,'no Frobenius period')
    return fs,ff

def polymul(a,b,poly,n):
    r=0
    while b:
        if b&1:r^=a
        b>>=1;a<<=1
        if a>>n:a^=poly
    return r

def gf_pow(a,k,poly,n):
    z=1
    while k:
        if k&1:z=polymul(z,a,poly,n)
        k>>=1;a=polymul(a,a,poly,n)
    return z

def find_poly(f):
    for poly in range((1<<f)|1,1<<(f+1),2):
        if all(gf_pow(a,(1<<f)-1,poly,f)==1 for a in range(1,1<<f)):
            return poly
    raise ArithmeticError('no GF polynomial')

def direct_count(a,f):
    poly=find_poly(f);q=1<<f
    a1,a2,a3,a4,a6=a
    total=1
    for x in range(q):
        x2=polymul(x,x,poly,f);x3=polymul(x2,x,poly,f)
        rhs=x3 ^ (a2*x2) ^ (a4*x) ^ a6
        for y in range(q):
            y2=polymul(y,y,poly,f)
            lhs=y2 ^ (a1*polymul(x,y,poly,f)) ^ (a3*y)
            if lhs==rhs:total+=1
    return total

def main():
    reps={};hist={};valid=[]
    for a in itertools.product(range(2),repeat=5):
        N=count_f2(a)
        good=(disc(a)%2!=0)
        if good:
            t=3-N
            reps.setdefault(t,a)
            hist[t]=hist.get(t,0)+1
            valid.append(a)
            for ell,p in FIXTURES:
                B=crt(a,p)
                check(all((b-o)%p==0 for b,o in zip(B,ORIGINAL)), 'CRT source mismatch')
                check(all((b-bit)%2==0 for b,bit in zip(B,a)), 'CRT target mismatch')
                check(disc(B)%p!=0 and disc(B)%2!=0,'CRT bad reduction')
    check(len(valid)==16 and sorted(hist.items())==[(-2,2),(-1,4),(0,4),(1,4),(2,2)],'census wrong')
    print(json.dumps({'good':len(valid),'singular':32-len(valid),
                      'traces':[[k,hist[k]] for k in sorted(hist)],'type':'census'},
                     sort_keys=True,separators=(',',':')))
    for ell,p in FIXTURES:
        gates=[(degree_gate(t,ell)[0], t, degree_gate(t,ell)[1]) for t in sorted(hist)]
        first,t,full=min(gates)
        a=reps[t]
        B=crt(a,p)
        check(disc(B)%p and disc(B)%2,'invalid winner')
        print(json.dumps({'ell':ell,'f':first,'full':full,'model':list(a),
                          'p':p,'trace':t,'type':'best'},sort_keys=True,separators=(',',':')))
        if ell in (7,11,13):
            counted=direct_count(a,first)
            check(counted==points_from_trace(t,first) and counted%ell==0,
                  'extension field count mismatch')
            print(json.dumps({'ell':ell,'f':first,'points':counted,'type':'direct'},
                             sort_keys=True,separators=(',',':')))
if __name__=='__main__': main()
