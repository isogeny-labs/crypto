#!/usr/bin/env python3
"""CT115: independent finite-field point arithmetic for pinned 11a1 binary target."""
import json

G = 0x130027
MASK = (1 << 20) - 1

def check(p, msg):
    if not p: raise ArithmeticError(msg)

def f20_mul(a, b):
    out = 0
    for _ in range(20):
        if b & 1: out ^= a
        b >>= 1
        a <<= 1
        if a & (1 << 20): a ^= G
    check(b == 0, 'bounded field multiplication')
    return out & MASK

def f20_pow(a, n):
    z = 1
    while n:
        if n & 1: z = f20_mul(z, a)
        a = f20_mul(a, a)
        n >>= 1
    return z

T = 2
RHS = f20_mul(f20_mul(T,T),T) ^ f20_mul(T,T)
# F_(2^40)=F_(2^20)[u] with u^2+u=RHS.
# Field elements are pairs of F_(2^20) integers.
def add(a,b): return a[0]^b[0], a[1]^b[1]
def mul(a,b):
    x,y=a;z,w=b
    v=f20_mul(y,w)
    return f20_mul(x,z)^f20_mul(v,RHS),f20_mul(x,w)^f20_mul(y,z)^v
ONE=(1,0); ZERO=(0,0)
def power(a,n):
    z=ONE
    while n:
        if n&1:z=mul(z,a)
        a=mul(a,a)
        n>>=1
    return z

def inverse(a):
    check(a!=ZERO,'invert nonzero')
    v=power(a,(1<<40)-2)
    check(mul(a,v)==ONE,'field inversion')
    return v

def neg(P):
    if P is None:return None
    x,y=P
    return x,add(y,ONE)

def point_add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if y1!=y2:return None
        # a1=0,a3=1,a2=1; slope f'(x)/(2y+1)=x^2
        lam=mul(x1,x1)
    else:
        lam=mul(add(y1,y2),inverse(add(x1,x2)))
    x3=add(add(mul(lam,lam),ONE),add(x1,x2))
    y3=add(add(y1,mul(lam,add(x1,x3))),ONE)
    return x3,y3

def scalar(n,P):
    r=None
    while n:
        if n&1:r=point_add(r,P)
        P=point_add(P,P);n>>=1
    return r

def frob(P,n):
    if P is None:return None
    x,y=P
    for _ in range(n):
        x=mul(x,x);y=mul(y,y)
    return x,y

def on_curve(P):
    if P is None:return True
    x,y=P
    return add(mul(y,y),y)==add(mul(mul(x,x),x),mul(x,x))

# Count the five base-field points instead of trusting the recorded trace.
base_count=1+sum((y*y+y-x*x*x-x*x)%2==0 for x in range(2) for y in range(2))
check(base_count==5,'base curve F2 cardinality')
# Field polynomial and quadratic irreducibility pinned by CT103.
check(f20_pow(T,1<<20)==T,'degree20 field test')
for d in (4,10):
    # Compute binary-polynomial gcd directly, independently of the elliptic arithmetic.
    def remainder(a,b):
        while a and a.bit_length()>=b.bit_length():a^=b<<(a.bit_length()-b.bit_length())
        return a
    def gcd(a,b):
        while b:a,b=b,remainder(a,b)
        return a
    check(gcd(f20_pow(T,1<<d)^T,G)==1,'irreducibility test')
trace=0;v=RHS
for _ in range(20):trace ^= v;v=f20_mul(v,v)
check(trace==1 and v==RHS,'quadratic extension irreducible')
P=((T,0),(0,1))
check(on_curve(P),'pinned T on curve')
check(scalar(11,P) is None and all(scalar(i,P) is not None for i in range(1,11)),'order 11')

points=[scalar(i,P) for i in range(1,11)]
check(len(set(points))==10,'ten unique oriented points')
check(len({Q[0] for Q in points})==5,'five Kummer x classes')
for i,V in enumerate(points,1):
    check(on_curve(V),'multiple on curve')
    check(frob(V,4)==scalar(7,V),'Frob4=[7]')
    check(frob(V,20)==neg(V),'Frob20=-1')
    check(frob(V,40)==V,'Frob40=1')
    check(frob(V,20)[0]==V[0] and frob(V,20)[1]!=V[1],'x in 2^20, y outside')
    check(all(frob(V,d)[0]!=V[0] for d in range(1,20)),'x exact degree20')
    check(len({frob(V,4*j)[0] for j in range(5)})==5,'x Frobenius-four cycle')
    check(all(frob(V,d)!=V for d in range(1,40)),'no smaller oriented field')
for i in range(1,11):
    for j in range(1,11):
        check(point_add(points[i-1],points[j-1])==scalar((i+j)%11,P),'complete group-law table')

# Separate mod-11 Frobenius representation computes full 121-point torsion action.
A=((0,9),(1,9))
def matmul(A,B):return tuple(tuple(sum(A[i][k]*B[k][j] for k in range(2))%11 for j in range(2)) for i in range(2))
I=((1,0),(0,1))
M=I
fixed=[]
for d in range(1,41):
    M=matmul(M,A)
    count=sum(1 for a in range(11) for b in range(11) if
              ((M[0][0]-1)*a+M[0][1]*b)%11==0 and
              (M[1][0]*a+(M[1][1]-1)*b)%11==0)
    fixed.append(count)
check(fixed==[1]*39+[121],'all torsion points require F2^40')
check(matmul(matmul(matmul(A,A),A),A)==((7,0),(0,7)),'Frob4 matrix')
print(json.dumps({
    'case':'ct115-frobenius-descent-obstruction',
    'base_curve_f2_points':base_count,
    'binary_x_field_degree':20,'binary_oriented_field_degree':40,
    'fixed_vector_counts_first_39':[1]*39,'fixed_vector_count_at_40':121,
    'frobenius_4_scalar_mod_11':7,'frobenius_20_scalar_mod_11':10,
    'nonzero_target_subgroup_checked':10,'group_law_pairs_checked':100,
    'kummer_x_classes':5,'kummer_x_orbit_length_under_frob4':5,
    'kummer_x_residue_degree':20,
    'hom_constant_z11_into_11_torsion_over_f2_20':1,
    'hom_constant_z11_into_11_torsion_over_f2_40':121,
    'hom_constant_z11_into_fixed_cyclic_target_over_f2_40':11,
    'nonzero_hom_into_11_torsion_over_f2_40':120,
    'global_point_lift_computed':False,'complete_dlp_cost_compared':False
},sort_keys=True,separators=(',',':')))
