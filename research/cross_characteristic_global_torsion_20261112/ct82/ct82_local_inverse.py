#!/usr/bin/env python3
"""Independent 11a1/37/7: p-adic root uniqueness and bounded torsion group controls."""
import json
from math import gcd
P=37
N=5
M=P**N

def check(ok,why):
    if not ok: raise ArithmeticError(why)
def poly_add(a,b):return [((a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0)) for i in range(max(len(a),len(b)))]
def poly_sub(a,b):return [((a[i] if i<len(a) else 0)-(b[i] if i<len(b) else 0)) for i in range(max(len(a),len(b)))]
def poly_mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]+=x*y
    return c
def poly_pow(a,n):
    y=[1]
    for _ in range(n):y=poly_mul(y,a)
    return y
D=[-79,-40,-4,4]; p3=[-21,-237,-60,-4,3]; f4=[-5821,-1496,-210,-790,-100,-4,2]
psi7=poly_sub(poly_mul(poly_sub(poly_mul(poly_pow(D,2),f4),poly_pow(p3,3)),poly_pow(p3,3)),poly_mul(poly_pow(D,2),poly_pow(f4,3)))
check(len(psi7)==25 and psi7[-1]==7,'division polynomial')
def val(f,x,mod):
    y=0
    for c in reversed(f):y=(y*x+c)%mod
    return y
def hensel(f,x0):
    der=[i*f[i] for i in range(1,len(f))]
    x=x0;step=P
    check(val(f,x,P)==0 and val(der,x,P)!=0,'bad Hensel root')
    for n in range(2,N+1):
        nxt=step*P
        rem=val(f,x,nxt)
        check(rem%step==0,'defect not divisible')
        digit=(-rem//step *pow(val(der,x,P),-1,P))%P
        x+=step*digit;step=nxt
        check(val(f,x,step)==0,'Hensel iteration')
    return x
roots=[hensel(psi7,x) for x in (9,27,28)]
check(len(set(roots))==3,'root collision')
def y_lift(x,y0):
    y=y0;step=P
    C=(-x**3+x*x+10*x+20)
    for _ in range(2,N+1):
        nxt=step*P; r=(y*y+y+C)%nxt
        check(r%step==0,'y equation defect')
        y+=step*((-r//step*pow(2*y+1,-1,P))%P)
        step=nxt
        check((y*y+y+C)%step==0,'y Hensel')
    return y
# Exhaustively certify these are ALL simple mod-37 division roots.
residue_roots=[a for a in range(P) if val(psi7,a,P)==0]
check(residue_roots==[9,27,28],'unaccounted rational x-root')
for i,x in enumerate(roots):
    d=val(D,x,M); z=val(p3,x,M)
    check(d%P and z%P,'exceptional division denominator')
    x2=((3*x*x-2*x-10)**2*pow(d,-1,M)+1-2*x)%M
    x3=(x-d*val(f4,x,M)*pow(z*z,-1,M))%M
    check(x2==roots[(i+1)%3] and x3==roots[(i+2)%3],'division rational-map root cycle')
lifted=[]
for x,x0 in zip(roots,(9,27,28)):
    candidates=[y for y in range(P) if (y*y+y-x0**3+x0*x0+10*x0+20)%P==0]
    check(len(candidates)==2,'not 2 y roots')
    for y0 in candidates:lifted.append((x,y_lift(x,y0)))
check(len(set(lifted))==6,'oriented point collision')

def curve(pt):
    if pt is None:return True
    x,y=pt;return (y*y+y-x*x*x+x*x+10*x+20)%M==0
def add(A,B):
    if A is None:return B
    if B is None:return A
    x,y=A; u,v=B
    if x==u and (y+v+1)%M==0:return None
    if A==B:
        d=2*y+1
        check(gcd(d,M)==1,'nonunit tangent denominator')
        slope=((3*x*x-2*x-10)*pow(d,-1,M))%M
    else:
        d=u-x
        check(gcd(d,M)==1,'nonunit chord denominator')
        slope=((v-y)*pow(d,-1,M))%M
    nu=(y-slope*x)%M
    xx=(slope*slope+1-x-u)%M
    yy=(-slope*xx-nu-1)%M
    R=(xx,yy)
    check(curve(R),'bad sum')
    return R

G=next(pt for pt in lifted if pt[0]%P==9 and pt[1]%P==4)
seen=[None]
for j in range(1,8):seen.append(add(seen[-1],G))
check(seen[-1] is None,'not order seven')
check(set(seen[1:7])==set(lifted),'Hensel/group-law discrepancy')
check(len([pt for pt in lifted if pt[0]%P in (9,27,28)])==6,'wrong count')
curve_count=1+sum(1 for x in range(P) for y in range(P) if (y*y+y-x**3+x*x+10*x+20)%P==0)
check(curve_count==35 and curve_count%7==0 and curve_count%(7*7)!=0 and (P-1)%7!=0,'source group and field order')
print(json.dumps({'experiment':'ct82-fixed-place-prime-to-p-reduction','source':37,'ell':7,'precision':N,'modulus':M,'x_roots':roots,'oriented_points':len(lifted),'torsion_group_size':7,'rational_full_rank_excluded':True,'group_law_verified':True,'fully_split_roots_found':3},sort_keys=True,separators=(',',':')))
