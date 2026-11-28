from research.residue_quotient_20261116_v99b.kernel import pmul
from research.simple_prefix_lift_20261116_v117b.kernel import normalize

def multiply(A,B):
    out=[0]*(len(A)+len(B)-1)
    for i,a in enumerate(A):
        for j,b in enumerate(B):out[i+j]^=pmul(a,b)
    return out

def lift_k3(H,d):
    F=normalize(H,d);assert [v&1 for v in F]==[0,0,1,1,1,1];A=[0,0,1];B=[1,1,1,1];max_degree=3*(d-1)
    for n in range(1,max_degree+1):
        old=multiply(A,B);e=[((F[j]^old[j])>>n)&1 for j in range(5)];a=[e[0],e[0]^e[1]];b=[e[1]^e[2],e[1]^e[3],e[0]^e[1]^e[4]]
        for j,value in enumerate(a):A[j]^=value<<n
        for j,value in enumerate(b):B[j]^=value<<n
    bounds_A=[2*(d-1),d-1,0];bounds_B=[3*(d-1),2*(d-1),d-1,0];bounded=all(v.bit_length()<=cap+1 for v,cap in zip(A,bounds_A))and all(v.bit_length()<=cap+1 for v,cap in zip(B,bounds_B));product=multiply(A,B);equal=product==F
    # Exact bounded normalized factorization contract only; no root-support claim.
    return {'A':A,'B':B,'bounded_factorization':bounded and equal,'support_complete':False}
