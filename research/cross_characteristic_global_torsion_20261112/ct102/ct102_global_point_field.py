#!/usr/bin/env python3
"""CT102: prove degree 60 over Q and nonsquare ordinate discriminant, hence 120."""
import contextlib
import io
import json
import runpy
from pathlib import Path

def check(ok,msg):
    if not ok:raise ArithmeticError(msg)

base=Path(__file__).resolve().parent.parent/'ct101'/'ct101_eleven_torsion.py'
with contextlib.redirect_stdout(io.StringIO()):
    data=runpy.run_path(str(base))
F=data['f11']; D=data['D']
gcdpoly,modpow=data['gcdpoly'],data['modpow']
check(len(F)==61 and F[-1]==11,'division polynomial')
# Rabin criterion for degree 60=2^2*3*5, mod 13.
p=13;f=[u%p for u in F];x=[0,1];u=x
for k in range(1,61):
    u=modpow(u,p,f,p)
    if k in (12,20,30):
        z=data['sub'](u,x)
        check(gcdpoly(f,z,p)==[1],f'Rabin gcd at {k}')
    if k==60:check(u==x,'Rabin final Frobenius')

# Exact Sylvester determinant via fraction-free Bareiss elimination.
n,m=len(F)-1,len(D)-1
A=[[0]*i+list(reversed(F))+[0]*(m-1-i) for i in range(m)] + [
    [0]*i+list(reversed(D))+[0]*(n-1-i) for i in range(n)]
size=n+m
check(len(A)==size and all(len(row)==size for row in A),'Sylvester shape')
prev=1;sign=1
for k in range(size-1):
    if A[k][k]==0:
        for j in range(k+1,size):
            if A[j][k]:
                A[k],A[j]=A[j],A[k];sign=-sign
                break
        else:raise ArithmeticError('zero resultant')
    pivot=A[k][k]
    for i in range(k+1,size):
        aik=A[i][k]
        for j in range(k+1,size):
            v=A[i][j]*pivot-aik*A[k][j]
            q,r=divmod(v,prev)
            check(r==0,'Bareiss nonexact division')
            A[i][j]=q
        A[i][k]=0
    prev=pivot
resultant=sign*A[-1][-1]
check(resultant<0,'nonsquare norm certificate must be negative')
expected=-1617717835776189959423474389729474523259174290922026987531771334356336309518989285550030273833071044614659831525201714316224621866288174560213656800883169001
check(resultant==expected,'independent resultant control')
# Norm_{Q(alpha)/Q}(D(alpha))=Res(F,D)/lc(F)^deg(D) < 0.
# If D(alpha) were a square in Q(alpha), its rational norm would be a square.
print(json.dumps({'case':'ct102-global-eleven-point-field','irreducible_mod_prime':13,
 'minimal_x_degree':60,'oriented_point_field_degree':120,
 'rabin_gcd_exponents':[12,20,30],'rabin_final_exponent':60,
 'resultant_f11_D':str(resultant),'norm_denominator':11**3,
 'negative_norm_certifies_nonsquare':True,'global_lift_of_public_source_computed':False},sort_keys=True,separators=(',',':')))
