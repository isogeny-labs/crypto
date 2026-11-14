#!/usr/bin/env python3
"""CT109: CRT proposal; exact integer polynomial identity and discriminant proof."""
import json
from ct109_modular_probe import f11,nums,dens,mul,add,sub,trim,panel,ck
P=[13,17,19,23,29,31,37,41,43,47,59]
OUT=[61,79,101]
def exactdiv(a,b):
    a=trim(a);b=trim(b);ck(b!=[0],"zero divisor")
    quotient=[0]*max(1,len(a)-len(b)+1)
    while len(a)>=len(b) and a!=[0]:
        k=len(a)-len(b);ck(a[-1]%b[-1]==0,"nonintegral quotient")
        v=a[-1]//b[-1];quotient[k]=v
        a=trim(sub(a,[0]*k+[v*z for z in b]))
    ck(a==[0],"nonzero remainder")
    return trim(quotient)
def crt():
    residues=panel(P);coeff=[];modulus=1
    for p in P:modulus*=p
    for j in range(13):
        r=0,m=1
        for p in P:
            val=residues[str(p)]['coefficients_low_to_high'][j]
            r+=m*((val-r)*pow(m,-1,p)%p);m*=p
        ck(m==modulus,"CRT modulus")
        coeff.append(r if r<=modulus//2 else r-modulus)
    return coeff,modulus
def certificate(coeff):
    B=[1]
    for b in dens:B=mul(B,b)
    A=[0]
    for a,b in zip(nums,dens):A=add(A,mul(a,exactdiv(B,b)))
    ck(len(A)==52 and len(B)==51,"rational degree")
    H=[coeff[-1]];powerB=[1]
    for c in reversed(coeff[:-1]):
        powerB=mul(powerB,B)
        H=add(mul(H,A),[c*z for z in powerB])
    ck(len(H)==613,"homogenized degree")
    Q=exactdiv(H,f11);ck(len(Q)==553,"quotient degree")
    return len(H)-1,len(Q)-1,max(abs(x).bit_length() for x in H)
def determinant(rows):
    A=[row[:] for row in rows];n=len(A);prev=1;sign=1
    for k in range(n-1):
        if A[k][k]==0:
            r=next((j for j in range(k+1,n) if A[j][k]),None)
            ck(r is not None,"singular")
            A[k],A[r]=A[r],A[k];sign=-sign
        pivot=A[k][k]
        for i in range(k+1,n):
            for j in range(k+1,n):
                t=A[i][j]*pivot-A[i][k]*A[k][j]
                ck(t%prev==0,"Bareiss inexact")
                A[i][j]=t//prev
            A[i][k]=0
        prev=pivot
    return sign*A[-1][-1]
def disc(coeff):
    n=len(coeff)-1;size=2*n-1
    deriv=[i*coeff[i] for i in range(1,n+1)]
    rows=[]
    for k in range(n-1):
        r=[0]*size;r[k:k+n+1]=list(reversed(coeff));rows.append(r)
    for k in range(n):
        r=[0]*size;r[k:k+n]=list(reversed(deriv));rows.append(r)
    return determinant(rows)
def main():
    q,M=crt()
    expected=[-5490195752091,-9133681615853,-4153362124197,-501858246395,
              -179918683945,8753368250,-4064649325,-308550000,
              -2165625,-543125,-5500,-20,1]
    ck(q==expected,"wrong proposed polynomial")
    dH,dQ,bits=certificate(q)
    held=panel(OUT)
    for p in OUT:ck([v%p for v in q]==held[str(p)]['coefficients_low_to_high'],"held-out residue mismatch")
    discr=disc(q);t=discr;v2=0
    while t%2==0:v2+=1;t//=2
    ck(v2==24 and discr<0 and len(str(abs(discr)))==195,"discriminant check")
    h=[1,1,0,0,1];z=[0]*9
    for i,a in enumerate(h):
        for j,b in enumerate(h):z[i+j]^=(a&b)
    cubic=[0]*13
    for i,a in enumerate(z):
        for j,b in enumerate(h):cubic[i+j]^=(a&b)
    ck([v%2 for v in q]==cubic,"binary reduction")
    ck(all(sum(c*pow(x,i,59) for i,c in enumerate(q))%59==0 for x in (23,32)),"source roots")
    print(json.dumps({'case':'ct109-characteristic-zero-line-minpoly',
      'coefficients_low_to_high':q,'degree':12,'crt_modulus':str(M),
      'discriminant_v2':v2,'index_at_2_valuation':v2//2,
      'crt_primes':P,'independent_held_out_primes':OUT,
      'integer_identity_certificate':True,'homogenized_degree':dH,
      'integer_quotient_degree':dQ,'homogenized_max_coefficient_bits':bits,
      'binary_reduction_h_cubed':True,'source_59_roots':[23,32],
      'global_public_lift_constructed':False,'end_to_end_advantage_measured':False},
      sort_keys=True,separators=(',',':')))
if __name__=='__main__':main()
