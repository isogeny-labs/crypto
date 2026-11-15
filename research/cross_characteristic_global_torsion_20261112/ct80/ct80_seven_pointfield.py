#!/usr/bin/env python3
"""CT80 exact seven-torsion global point-field and two-prime toy (unsigned).

Not a large-order log-free lifting algorithm. [pcc:unverified]
"""
import json

def check(ok, reason):
    if not ok: raise ArithmeticError(reason)

def clean(a):
    a=list(a)
    while len(a)>1 and a[-1]==0: a.pop()
    return tuple(a)

def add(a,b):
    return clean([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])

def sub(a,b):
    return clean([(a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])

def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,v in enumerate(a):
        for j,w in enumerate(b): c[i+j]+=v*w
    return clean(c)

def power(a,n):
    v=(1,)
    while n:
        if n&1:v=mul(v,a)
        a=mul(a,a);n//=2
    return v

# Standard division-polynomial recurrences for E: y^2+y=x^3-x^2-10x-20.
D=(-79,-40,-4,4)  # psi_2^2
psi3=(-21,-237,-60,-4,3)
f4=(-5821,-1496,-210,-790,-100,-4,2)  # psi_4/psi_2
psi5=sub(mul(power(D,2),f4),power(psi3,3))
psi7=sub(mul(psi5,power(psi3,3)),mul(power(D,2),power(f4,3)))
check(len(psi7)==25 and psi7[-1]==7,'wrong division polynomial')
check(max(abs(c) for c in psi7).bit_length()==51,'coefficient-height mismatch')

def norm(a,p):return clean([x%p for x in a])

def divrem(a,b,p):
    a=list(norm(a,p));b=norm(b,p)
    check(b!=(0,),'division by zero polynomial')
    q=[0]*max(1,len(a)-len(b)+1)
    inv=pow(b[-1],-1,p)
    while len(a)>=len(b) and a!=[0]:
        d=len(a)-len(b);v=a[-1]*inv%p;q[d]=v
        for j,u in enumerate(b):a[d+j]=(a[d+j]-v*u)%p
        a=list(clean(a))
    return norm(q,p),norm(a,p)

def gcdpoly(a,b,p):
    a=norm(a,p);b=norm(b,p)
    while b!=(0,):a,b=b,divrem(a,b,p)[1]
    return norm(tuple(v*pow(a[-1],-1,p)%p for v in a),p)

def monic(a,p):
    a=norm(a,p)
    return norm(tuple(v*pow(a[-1],-1,p) for v in a),p)

def powmod(a,n,f,p):
    v=(1,)
    while n:
        if n&1:v=divrem(mul(v,a),f,p)[1]
        a=divrem(mul(a,a),f,p)[1];n//=2
    return v

def frob_x(k,f,p):
    x=(0,1)
    for _ in range(k):x=powmod(x,p,f,p)
    return x

def irred(f,p):
    f=monic(f,p);n=len(f)-1
    if n<1 or gcdpoly(f,tuple(i*v for i,v in enumerate(f))[1:],p)!=(1,):return False
    if frob_x(n,f,p)!=(0,1):return False
    divisors=[d for d in range(2,n+1) if n%d==0 and all(d%q for q in range(2,int(d**0.5)+1))]
    for q in divisors:
        if len(gcdpoly(f,sub(frob_x(n//q,f,p),(0,1)),p))>1:return False
    return True

F1=(1,0,1,0,0,1,0,0,0,1,0,0,1)
F2=(1,1,1,0,1,1,0,0,0,1,0,0,1)
check(norm(mul(F1,F2),2)==norm(psi7,2),'incorrect mod2 factorization')
check(irred(F1,2) and irred(F2,2),'mod2 factors not irreducible')
check(irred(psi7,3),'mod3 irreducibility witness fails')
check(gcdpoly(psi7,tuple(i*v for i,v in enumerate(psi7))[1:],37)==(1,),'ramified source x')
g1=gcdpoly(psi7,sub(frob_x(1,monic(psi7,37),37),(0,1)),37)
g3=gcdpoly(psi7,sub(frob_x(3,monic(psi7,37),37),(0,1)),37)
check(len(g1)-1==3 and len(g3)-1==24,'mod37 factor degrees')
check(all(sum(c*pow(z,i,37) for i,c in enumerate(psi7))%37==0 for z in (9,27,28)),'bad local roots')

MOD=0x1225;MASK=(1<<12)-1
def cmul(a,b):
    o=0
    while b:
        if b&1:o^=a
        b>>=1;a<<=1
        if a&(1<<12):a^=MOD
    return o&MASK

def cpow(a,n):
    o=1
    while n:
        if n&1:o=cmul(o,a)
        a=cmul(a,a);n//=2
    return o

G=cmul(cmul(2,2),2)^cmul(2,2)
tr=0;t=G
for _ in range(12):tr^=t;t=cmul(t,t)
check(tr==1,'Artin-Schreier quadratic splits over F4096')

# F2^24 = F2[x]/F1 [Y]/(Y^2+Y+x^3+x^2).
ONE=(1,0);XX=(2,0);YY=(0,1)
def fadd(a,b):return (a[0]^b[0],a[1]^b[1])
def fmul(a,b):
    aa=cmul(a[0],b[0]);bd=cmul(a[1],b[1])
    return (aa^cmul(bd,G),cmul(a[0],b[1])^cmul(a[1],b[0])^bd)

def finv(a):
    aa,bb=a;n=cmul(aa,aa)^cmul(aa,bb)^cmul(cmul(bb,bb),G)
    check(n!=0,'zero inverse')
    z=cpow(n,(1<<12)-2)
    return (cmul(aa^bb,z),cmul(bb,z))

def fdiv(a,b):return fmul(a,finv(b))

def fpow(a,n):
    z=ONE
    while n:
        if n&1:z=fmul(z,a)
        a=fmul(a,a);n//=2
    return z

def dst_curve(P):
    if P is None:return True
    x,y=P
    return fadd(fmul(y,y),y)==fadd(fmul(fmul(x,x),x),fmul(x,x))

def dst_add(U,V):
    if U is None:return V
    if V is None:return U
    x,y=U;z,t=V
    if x==z and fadd(y,t)==ONE:return None
    if U==V:lam=fmul(x,x);nu=fadd(y,fmul(lam,x))
    else:lam=fdiv(fadd(t,y),fadd(z,x));nu=fadd(y,fmul(lam,x))
    xx=fadd(fadd(fadd(fmul(lam,lam),ONE),x),z)
    yy=fadd(fadd(fmul(lam,xx),nu),ONE)
    check(dst_curve((xx,yy)),'off-curve target sum')
    return xx,yy

def src_curve(P):
    if P is None:return True
    x,y=P
    return (y*y+y-x*x*x+x*x+10*x+20)%37==0

def src_add(U,V):
    p=37
    if U is None:return V
    if V is None:return U
    x,y=U;z,t=V
    if x==z and (y+t+1)%p==0:return None
    if U==V:
        lam=(3*x*x-2*x-10)*pow(2*y+1,-1,p)%p
        nu=(y-lam*x)%p
    else:lam=(t-y)*pow(z-x,-1,p)%p;nu=(y-lam*x)%p
    xx=(lam*lam+1-x-z)%p
    yy=(-lam*xx-nu-1)%p
    check(src_curve((xx,yy)),'off-curve source sum')
    return xx,yy

def multiples(P,add):
    out=[None]
    for _ in range(1,8):out.append(add(out[-1],P))
    check(out[-1] is None and len(set(map(str,out[:-1])))==7,'wrong point order')
    return out[:-1]

def main():
    P=(9,4);U=(XX,YY)
    check(src_curve(P) and dst_curve(U),'bad curve models')
    S=multiples(P,src_add);B=multiples(U,dst_add)
    check(fpow(XX,1<<12)==XX and fpow(YY,1<<12)==fadd(YY,ONE),'wrong degree24 target')
    check(fpow(YY,1<<24)==YY,'wrong field size')
    check(all(src_curve(a) and dst_curve(b) for a,b in zip(S,B)),'invalid image')
    check(len({str(q[0]) for q in S if q})==3,'wrong source x orbits')
    rows=[];recognition_steps=0;target_steps=0
    for q in S:
        j=next((i for i,v in enumerate(S) if v==q),None)
        check(j is not None,'source recognition failed')
        recognition_steps+=j;image=B[j]
        k=next((i for i,v in enumerate(B) if v==image),None)
        check(k==j,'target log mismatch')
        target_steps+=k
        rows.append({'source':list(q) if q else None,'target':[[x[0],x[1]] for x in image] if image else None,'recovered':k,'candidates':j+1})
    wrong=(fpow(B[1][0],1<<12),fpow(B[1][1],1<<12))
    check(wrong==B[6] and wrong!=B[1],'wrong-place Frobenius control failed')
    print(json.dumps({'experiment':'ct80-seven-point-field','ell':7,'global_x_degree':24,'global_point_degree':48,'source_prime':37,'source_linear_x_roots':[9,27,28],'source_factor_degrees':{'1':3,'3':7},'target_field_degree':24,'target_x_factor_degrees':[12,12],'source_generator':[9,4],'target_generator':[[2,0],[0,1]],'coefficient_height_bits':51,'source_scan_additions_if_repeated':recognition_steps,'target_scan_additions_if_repeated':target_steps,'candidate_enumeration_used':True,'global_field_precomputed':True,'rows':rows},sort_keys=True,separators=(',',':')))

if __name__=='__main__':main()
