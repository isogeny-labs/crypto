#!/usr/bin/env python3
"""CT39: public F101 elliptic factors, two fixed integer lifts, residue field floors."""
import json
CASES=((1,46,109,112,27),(44,21,113,99,4256))
def require(x,msg):
    if not x: raise RuntimeError(msg)
def count(a,b,p):
    squares={y*y%p for y in range(p)}
    direct=1+sum((y*y-x**3-a*x-b)%p==0 for x in range(p) for y in range(p))
    via_char=1+sum(1 if f==0 else 2 if f in squares else 0
                   for x in range(p) for f in ((x**3+a*x+b)%p,))
    require(direct==via_char,"point count disagreement")
    require((-16*(4*a*a*a+27*b*b))%p!=0,"bad reduction")
    return direct
def recurrence(r,q,t,stop):
    s0,s1=2%r,t%r
    qpower=1
    hits=[]
    for f in range(1,stop+1):
        qpower=qpower*q%r
        if f==1: s=s1
        else:
            s=(t*s1-q*s0)%r
            s0,s1=s1,s
        if (qpower+1-s)%r==0: hits.append(f)
    return hits
def mm(A,B,r):
    return tuple(tuple(sum(A[i][k]*B[k][j] for k in range(2))%r
                       for j in range(2)) for i in range(2))
def matrix(r,q,t,stop):
    F=((0,(-q)%r),(1,t%r))
    P=((1,0),(0,1))
    hits=[]
    for f in range(1,stop+1):
        P=mm(P,F,r)
        d=((1-P[0][0])*(1-P[1][1])-P[0][1]*P[1][0])%r
        if d==0: hits.append(f)
    return hits
def main():
    out=[]
    for a,b,r,n103,floor in CASES:
        n101,n=count(a,b,101),count(a,b,103)
        require(n101==r and n==n103,"cardinality witness")
        t=104-n
        hits=recurrence(r,103,t,2*floor)
        require(hits==matrix(r,103,t,2*floor)==[floor,2*floor],
                "two Frobenius algorithms disagree")
        require(n%r!=0 and recurrence(r,103,t+2,2*floor)!=hits,
                "mutated-trace negative control")
        out.append(dict(a=a,b=b,r=r,n101=n101,n103=n,t103=t,
          min_residue_degree=floor,number_field_degree_floor=floor,
          discriminant103=(-16*(4*a*a*a+27*b*b))%103))
    print(json.dumps({"cases":out,"status":"[pcc:unverified]; fixed Q-models only"},sort_keys=True))
if __name__=="__main__": main()
