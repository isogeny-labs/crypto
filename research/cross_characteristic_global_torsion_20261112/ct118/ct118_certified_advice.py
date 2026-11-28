#!/usr/bin/env python3
"""CT118: genuine two-prime global-field interpolation with hidden source-DLP advice.
UNVERIFIED research negative control, not an efficient transfer or speedup.
Requires only Python's standard library. Run with -B (and again with -O).
"""
import json
from fractions import Fraction
from math import isqrt

P, N, SOURCE_G, MOD2 = 59, 11, (19, 22), 0x130027

def neg(q):
    return None if q is None else (q[0], (-q[1]-1) % P)

def add(q, r):
    if q is None: return r
    if r is None: return q
    x, y = q; z, w = r
    if x == z and (y+w+1) % P == 0: return None
    if x == z:
        lam = (3*x*x-2*x-10)*pow(2*y+1, -1, P) % P
    else:
        lam = (w-y)*pow(z-x, -1, P) % P
    xx = (lam*lam+1-x-z) % P
    return xx, (-1-y+lam*(x-xx)) % P

def mul(n, q):
    s = None
    while n:
        if n & 1: s = add(s, q)
        q = add(q, q); n //= 2
    return s

def valid(q):
    return q is not None and all(isinstance(v, int) and 0 <= v < P for v in q) and (q[1]**2+q[1]-q[0]**3+q[0]**2+10*q[0]+20) % P == 0 and mul(N,q) is None

def trim(a):
    while len(a)>1 and a[-1] == 0: a.pop()
    return a

def padd(a,b):
    return trim([(a[k] if k<len(a) else 0)+(b[k] if k<len(b) else 0) for k in range(max(len(a),len(b)))])

def pneg(a): return [-c for c in a]
def psub(a,b): return padd(a,pneg(b))
def pmul(a,b):
    c = [0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b): c[i+j] += x*y
    return trim(c)

def ppow(a,n):
    out=[1]
    while n:
        if n&1:out=pmul(out,a)
        a=pmul(a,a);n//=2
    return out

def poly_eval(coeff, x, addop, mulop, canon):
    out=canon(0)
    for c in reversed(coeff): out=addop(mulop(out,x),canon(c))
    return out

def gf_mul(a,b):
    z=0
    while b:
        if b&1:z ^= a
        b >>=1;a <<=1
        if a&(1<<20):a ^= MOD2
    return z & ((1<<20)-1)

def gf_pow(a,n):
    r=1
    while n:
        if n&1:r=gf_mul(r,a)
        a=gf_mul(a,a);n>>=1
    return r

def rational_maps():
    x=[0,1]
    g=[-79,-40,-4,4]                      # psi_2 squared
    p3=[-21,-237,-60,-4,3]               # psi_3
    h=[-5821,-1496,-210,-790,-100,-4,2] # psi_4/psi_2
    p5=psub(pmul(ppow(g,2),h),ppow(p3,3))
    return [
        (x,[1]),
        (psub(pmul(x,g),p3),g),
        (psub(pmul(x,ppow(p3,2)),pmul(g,h)),ppow(p3,2)),
        (psub(pmul(x,pmul(g,ppow(h,2))),pmul(p3,p5)),pmul(g,ppow(h,2))),
        (psub(pmul(x,ppow(p5,2)),pmul(pmul(pmul(g,h),p3),psub(p5,ppow(h,2)))),ppow(p5,2)),
    ]

def l_basis(abscissas):
    polys=[]
    for i,r in enumerate(abscissas):
        poly=[Fraction(1)]; den=1
        for j,s in enumerate(abscissas):
            if i==j:continue
            newer=[Fraction(0)]*(len(poly)+1)
            for k,c in enumerate(poly):newer[k]-=s*c; newer[k+1]+=c
            poly=newer;den*=r-s
        polys.append([a/den for a in poly])
    return polys

def source_rational(n,d):
    a=poly_eval(n,19,lambda u,v:(u+v)%P,lambda u,v:u*v%P,lambda v:v%P)
    b=poly_eval(d,19,lambda u,v:(u+v)%P,lambda u,v:u*v%P,lambda v:v%P)
    if not b:raise ArithmeticError('source specialization pole')
    return a*pow(b,-1,P)%P,b

def binary_rational(n,d):
    a=poly_eval(n,2,lambda u,v:u^v,gf_mul,lambda v:v&1)
    b=poly_eval(d,2,lambda u,v:u^v,gf_mul,lambda v:v&1)
    if not b:raise ArithmeticError('binary specialization pole')
    return gf_mul(a,gf_pow(b,(1<<20)-2)),b

def bsgs(q):
    m=isqrt(N)+1; babies={}; s=None; cost=0
    for j in range(m):babies[s]=j;s=add(s,SOURCE_G);cost+=1
    shift=neg(s);t=q
    for i in range(m+1):
        if t in babies:return (i*m+babies[t])%N,cost
        t=add(t,shift);cost+=1
    raise ArithmeticError('BSGS failed')

def main():
    if not valid(SOURCE_G):raise ArithmeticError('bad source generator')
    maps=rational_maps()
    # CT102 supplies degree 60 for x(T); no nonzero polynomial of smaller degree vanishes.
    if max(len(d)-1 for n,d in maps)>=60:raise ArithmeticError('global denominator degree')
    source=[source_rational(n,d) for n,d in maps]
    binary=[binary_rational(n,d) for n,d in maps]
    if [r for r,_ in source]!=[19,20,43,46,22]:raise ArithmeticError('source coordinate certificates')
    if [r for r,_ in binary]!=[2,17,4956,65536,4697]:raise ArithmeticError('binary field certificates')
    # OFFLINE: construct the complete source projective DLP table, then embed in coefficients.
    points=[mul(k,SOURCE_G) for k in range(1,6)]
    abscissas=[q[0] for q in points]
    if abscissas!=[r for r,_ in source] or len(set(abscissas))!=5:raise ArithmeticError('advice mismatch')
    basis=l_basis(abscissas)
    for k,r in enumerate(abscissas):
        if [poly_eval(b,Fraction(r),lambda a,c:a+c,lambda a,c:a*c,Fraction) for b in basis]!=[Fraction(int(k==j)) for j in range(5)]:raise ArithmeticError('not characteristic-zero Kronecker')
    # ONLINE: A(x_Q)=sum_j L_j(x_Q)*R_j(alpha), alpha=x(T) in fixed degree-60 field.
    public=[];source_cost=[]
    for secret in range(1,11):
        q=mul(secret,SOURCE_G)
        if not valid(q):raise ArithmeticError('invalid public point')
        weights=[poly_eval(b,Fraction(q[0]),lambda a,c:a+c,lambda a,c:a*c,Fraction) for b in basis]
        if sum(weights)!=1 or any(w not in (0,1) for w in weights):raise ArithmeticError('unrecognized x')
        target=0
        for w,(bitmask,_) in zip(weights,binary):
            if w==1:target ^= bitmask
        j=next((j for j, (b,_) in enumerate(binary) if b==target),None)
        if j is None:raise ArithmeticError('target projective recovery')
        c=j+1 if mul(j+1,SOURCE_G)==q else (-j-1)%N
        if c!=secret:raise ArithmeticError('sign/DLP failure')
        scalar,bc=bsgs(q)
        if scalar!=secret:raise ArithmeticError('BSGS mismatch')
        source_cost.append(bc)
        public.append([q[0],q[1],target,c])
    rejects=[]
    for title,bad in [('off_curve',(19,23)),('outside_subgroup',(4,18)),('identity',None)]:
        if valid(bad):raise ArithmeticError('bad input admitted')
        rejects.append(title)
    print(json.dumps({'case':'ct118-genuine-dual-place-interpolation-advice','pcc':'unverified',
      'source_x_classes':abscissas,'binary_x_classes':[b for b,_ in binary],
      'global_field_x_degree_prerequisite':60,
      'rational_map_numerator_degrees':[len(n)-1 for n,d in maps],
      'rational_map_denominator_degrees':[len(d)-1 for n,d in maps],
      'source_denominator_residues':[b for r,b in source],
      'binary_denominator_residues':[b for r,b in binary],
      'interpolation_polynomial_degree':4,'global_coefficients_are_rational_functions_of_alpha':True,
      'charzero_evaluation_gives_exact_global_conjugates':True,
      'source_and_binary_fixed_prime_residues_match':True,
      'offline_projective_dlp_entries':5,'offline_source_additions':4,
      'online_scalar_lookup':False,'online_certificate_has_no_advice':False,
      'public_recoveries':public,'rejected_controls':rejects,
      'matched_source_bsgs_additions':source_cost,
      'target_full_dlp_and_all_costs_measured':False,
      'global_certificate_suffices_for_speedup':False},sort_keys=True,separators=(',',':')))
if __name__=='__main__':main()
