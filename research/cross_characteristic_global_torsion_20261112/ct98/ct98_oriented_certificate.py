#!/usr/bin/env python3
"""CT98: oriented point certificates expose the hidden multiplication scalar.

CT97 supplies exact scalar-blind x-root rational expressions at fixed primes.
This derivative-based ordinate certificate computes their source orientation and
binary specialization, but recovers the multiplier from the rational map first.
Thus it is a NEGATIVE cryptanalytic control, not a fast DLP reduction.
[pcc:unverified]
"""
from pathlib import Path
from contextlib import redirect_stdout
from io import StringIO
from math import isqrt
import json, runpy
import sympy as sp

with redirect_stdout(StringIO()):
    ctx = runpy.run_path(str(Path(__file__).resolve().parent.parent /
        'ct97' / 'ct97_selective_x_root.py'))
x=ctx['x']; roots=ctx['roots']; recon=ctx['rational_reconstruct']; check=ctx['check']
D=sp.Poly(4*x**3-4*x**2-40*x-79,x,domain=sp.ZZ)
f=sp.Poly(x**3-x**2-10*x-20,x,domain=sp.ZZ)
public=((9,4),(9,32),(27,9),(27,27),(28,0),(28,36))

def m12(a,b):
    c=0
    while b:
        if b&1:c^=a
        a<<=1
        if a&4096:a^=0x1225
        b>>=1
    return c&4095

def pow12(a,n):
    z=1
    while n:
        if n&1:z=m12(z,a)
        a=m12(a,a); n>>=1
    return z

def binval(P):
    v=0
    for a in P.all_coeffs():
        check(a.q==1,'non-integral coefficient at target prime')
        v=m12(v,2)^(int(a)&1)
    return v

def ev37(P):return int(P.eval(9))%37

def certified(A,B,source_y):
    A=sp.Poly(A,x,domain=sp.ZZ); B=sp.Poly(B,x,domain=sp.ZZ)
    check(A.degree()==B.degree()+1,'not multiplication-map form')
    square=sp.Rational(B.LC(),A.LC())
    check(square.q==1 and square>0,'leading ratio not positive integer')
    mag=isqrt(int(square))
    check(mag*mag==square,'leading ratio not a square')
    N=A.diff()*B-A*B.diff()
    H=4*A**3-4*A**2*B-40*A*B**2-79*B**3
    check(D*N*N==mag*mag*B*H,'invariant-differential identity')
    check(all(int(z)%mag==0 for z in N.all_coeffs()),'V not integral')
    V=sp.Poly(N.as_expr()/mag,x,domain=sp.ZZ)
    S=V-B**2
    check(all(int(z)%2==0 for z in S.all_coeffs()),'2-integrality missing')
    U=sp.Poly(S.as_expr()/2,x,domain=sp.ZZ)
    h=A**3-A**2*B-10*A*B**2-20*B**3
    check(U*U+U*B**2+V*V*f==B*h,'global ordinate equation')
    den37=ev37(B)**2%37
    check(den37!=0,'source denominator nonunit')
    y0=(ev37(U)+4*ev37(V))*pow(den37,-1,37)%37
    sign=1 if source_y==y0 else -1 if source_y==(-1-y0)%37 else 0
    check(sign!=0,'oriented source mismatch')
    den12=binval(B)
    check(den12!=0,'binary denominator nonunit')
    dinv=pow12(m12(den12,den12),4094)
    ybin=[m12(binval(U),dinv),m12(binval(V),dinv)]
    if sign<0:ybin[0]^=1
    xb=m12(binval(A),pow12(den12,4094))
    def mul24(a,b):
        t=m12(a[1],b[1])
        return [m12(a[0],b[0])^m12(t,12),
                m12(a[0],b[1])^m12(a[1],b[0])^t]
    y2=mul24(ybin,ybin)
    check([y2[0]^ybin[0],y2[1]^ybin[1]]==
          [m12(m12(xb,xb),xb)^m12(xb,xb),0], 'binary curve equation')
    return dict(source=[ev37(A)*pow(ev37(B),-1,37)%37,source_y],
                target_x=xb,target_y=ybin,multiplier_from_x_map=(mag*sign)%7,
                multiplier_magnitude=mag,differential_certificate=True,
                two_integral_numerator=True)

precomputed=[recon(beta) for beta in roots]
rows=[]
for qx,qy in public:
    matches=[]
    for A,B in precomputed:
        b0=ev37(sp.Poly(B,x,domain=sp.ZZ))
        if b0 and ev37(sp.Poly(A,x,domain=sp.ZZ))*pow(b0,-1,37)%37==qx:
            matches.append((A,B))
    check(len(matches)==1,'nonunique CT97 x root')
    row=certified(*matches[0],qy)
    check(row['source']==[qx,qy],'source point mismatch')
    rows.append(row)
check([r['target_x'] for r in rows]==[2,2,17,17,1562,1562], 'binary x')
check([r['target_y'] for r in rows]==
      [[0,1],[1,1],[77,1],[76,1],[2879,1],[2878,1]], 'binary ordinate')
print(json.dumps(dict(experiment='CT98-differential-ordinate-certificate',
   orientation_transport=True,scalar_independent_transfer=False,
   leaked_source_log=True,source_rows=rows),sort_keys=True,separators=(',',':')))
