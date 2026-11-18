#!/usr/bin/env python3
"""CT79: fixed 11a1[3] global-point recognition. Toy, not scalable. [pcc:unverified]"""
import json
from math import comb
P=29; L=3; PREC=6; M=P**PREC
H=(-101306819,89655,267096,349907,162543,-14817,-4813,108,27)
A=(15208005,352381,332454,-39746,-19603,324,108)
D=4992581

def check(ok,message):
    if not ok: raise ArithmeticError(message)
def val(poly,z,mod=None):
    r=0
    for c in reversed(poly): r=r*z+c; r=r%mod if mod else r
    return r
def deriv(poly): return tuple(i*poly[i] for i in range(1,len(poly)))
def hensel(root):
    check(val(H,root,P)==0 and val(deriv(H),root,P)!=0,'unramified root')
    z=root
    for k in range(2,PREC+1):
        n=P**k
        z=(z-val(H,z)*pow(val(deriv(H),z,n),-1,n))%n
        check(val(H,z,n)==0 and z%P==root,'bad Newton lift')
    return z
# Polynomial representations of two globally rational K-torsion points.
# Neither tuple is a log label: (a,b) means y=a+b*beta.
REP=((0,1),(-1,-1))
def x_of(z,mod):return (-val(A,z,mod)*pow(D,-1,mod))%mod
def affine_on(z,mod):return (x_of(z,mod),z%mod)
def curve(q,mod):
    if q is None:return True
    x,y=q;return (y*y+y-x**3+x*x+10*x+20)%mod==0

def recognize(q):
    if q is None:return None,0
    checks=0; found=[]
    for a,b in REP:
        checks+=1
        if affine_on((a+5*b)%P,P)==q:found.append((a,b))
    check(len(found)==1,'unrecognized or ambiguous source point')
    return found[0],checks

def src_add(U,V):
    if U is None:return V
    if V is None:return U
    x,y=U;z,t=V
    if x==z and (y+t+1)%P==0:return None
    if U!=V: lam=((t-y)*pow(z-x,-1,P))%P; nu=((y*z-t*x)*pow(z-x,-1,P))%P
    else:
        denom=pow(2*y+1,-1,P)
        lam=((3*x*x-2*x-10)*denom)%P
        nu=((-x*x*x-10*x-40-y)*denom)%P
    xx=(lam*lam+1-x-z)%P
    yy=(-lam*xx-nu-1)%P
    check(curve((xx,yy),P),'source addition off curve')
    return xx,yy

def fm(a,b):
    # F_256 with irreducible M(y)=0x17b
    v=0
    while b:
        if b&1:v^=a
        a<<=1
        if a&256:a^=0x17b
        b>>=1
    return v
def fpow(a,n):
    z=1
    while n:
        if n&1:z=fm(z,a)
        a=fm(a,a);n//=2
    return z
def fval(poly,z):
    v=0
    for c in reversed(poly):v=fm(v,z)^(c&1)
    return v

def dst_curve(q):
    if q is None:return True
    x,y=q;return fm(y,y)^y==fm(fm(x,x),x)^fm(x,x)

def dst_add(U,V):
    if U is None:return V
    if V is None:return U
    x,y=U;z,t=V
    if x==z and (y^t)==1:return None
    if U!=V:
        lam=fm(t^y,fpow(z^x,254))
        nu=fm(y,z)^fm(t,x)
        nu=fm(nu,fpow(z^x,254))
    else:
        lam=fm(x,x);nu=fm(fm(x,x),x)^y
    xx=fm(lam,lam)^1^x^z
    yy=fm(lam,xx)^nu^1
    check(dst_curve((xx,yy)),'target addition off curve')
    return xx,yy

def scan(base,goal,add):
    point=None
    for k in range(L):
        if point==goal:return k,k
        point=add(point,base)
    raise ArithmeticError('target not in subgroup')

def main():
    # Global identities and integral good-reduction checks.
    neg=[sum(c*comb(i,j)*(-1)**i for i,c in enumerate(A) if i>=j) for j in range(len(A))]
    check(tuple(neg)==A,'X is not invariant under global negation')
    check(D%2 and D%P and val(H,5,P)==0 and val(H,23,P)==0,'bad prime')
    b5=hensel(5);b23=hensel(23)
    check(b5==425642517 and b23==(-b5-1)%M,'two source primes mismatch')
    check(affine_on(b5,M)==(25104680,425642517),'CT74 precision-six mismatch')
    check(affine_on(b23,M)==(25104680,169180803),'wrong inverse lift')
    beta=2
    check(fval(H,beta)==0,'bad binary residue field')
    x2=fval(tuple(-c for c in A),beta)  # denominator D=1 modulo 2
    check(x2==19,'wrong global binary X')
    source=[None,(18,5),(18,23)]
    tgt_base=(x2,beta);src_base=(18,5)
    check(src_add(src_base,src_add(src_base,src_base)) is None,'source order')
    check(dst_add(tgt_base,dst_add(tgt_base,tgt_base)) is None,'binary order')
    allrows=[];checks=0
    for q in source:
        rep,n=recognize(q);checks+=n
        if rep is None: image=None; local=None
        else:
            a,b=rep
            # One fixed source place beta -> b5; never switch to beta -> b23.
            local=affine_on((a+b*b5)%M,M)
            check(local[0]%P==q[0] and local[1]%P==q[1],'wrong shared source place')
            check(local[0]==25104680,'not the unique torsion lift')
            image=(x2,(a&1)^fm((b&1),beta))
            check(dst_curve(image),'wrong binary specialization')
        ks,ops=scan(src_base,q,src_add)
        kt,tops=scan(tgt_base,image,dst_add)
        check(ks==kt,'discrete log changed')
        allrows.append({'input':list(q) if q else None,'local_y':local[1] if local else None,'target':list(image) if image else None,'source_dlog':ks,'binary_dlog':kt,'source_scan_additions':ops,'binary_scan_additions':tops,'candidate_checks':n})
    # Falsifier: change only Q's source prime from beta->5 to beta->23,
    # retaining beta->2 at the binary place; Q maps to U instead of -U.
    check(affine_on(b23,P)==(18,23),'wrong-prime source setup')
    wrong=(x2,beta)
    check(wrong!=allrows[2]['target'] and scan(tgt_base,wrong,dst_add)[0]==1,'wrong-place falsifier')
    for bad in ((5,5),(18,6),(0,0)):
        try: recognize(bad)
        except ArithmeticError: pass
        else: raise ArithmeticError('accepted an invalid/outside-subgroup point')
    print(json.dumps({'experiment':'ct79-global-recognition','field_degree':8,'source_prime':29,'target_field':256,'precision':PREC,'source_root5':b5,'alternate_root23':b23,'shared_source_place':5,'target_beta_byte':beta,'candidate_checks_total':checks,'rows':allrows,'wrong_source_place_log':1,'log_lookup_used':False,'candidate_enumeration_used':True,'global_field_precomputed':True,'scalable':False},sort_keys=True,separators=(',',':')))

if __name__=='__main__':main()
