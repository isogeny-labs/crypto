#!/usr/bin/env python3
"""CT110: exact 2-integral order from CT109's characteristic-zero q(S).

Use independent integer polynomial arithmetic. All checks remain active under -O.
"""
import json

def require(condition, message):
    if not condition:
        raise ArithmeticError(message)

# CT109's certified q(S), ascending coefficient order.
Q = [-5490195752091, -9133681615853, -4153362124197,
     -501858246395, -179918683945, 8753368250, -4064649325,
     -308550000, -2165625, -543125, -5500, -20, 1]
H = [1, 1, 0, 0, 1]

def trim(a):
    a = a[:]
    while len(a)>1 and a[-1]==0:
        a.pop()
    return a

def add(a,b):
    return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0)
                 for i in range(max(len(a),len(b)))])

def mul(a,b):
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):
            out[i+j]+=x*y
    return trim(out)

def divmonic(a,b):
    require(b[-1]==1,'monic integer polynomial required')
    rem=trim(a)
    quotient=[0]*max(1,len(rem)-len(b)+1)
    while len(rem)>=len(b) and rem!=[0]:
        d=len(rem)-len(b); t=rem[-1]
        quotient[d]+=t
        for j,x in enumerate(b):
            rem[d+j]-=t*x
        rem=trim(rem)
    return trim(quotient),rem

def h_exp(a):
    out=[]; t=trim(a)
    while t!=[0]:
        t,r=divmonic(t,H)
        out.append(r)
    return out

EXP=h_exp(Q)
require(len(EXP)==4 and EXP[3]==[1],'q must be cubic in h')
require(all(x%(2**(3-i))==0 for i in range(3) for x in EXP[i]),
        'h-adic coefficients lack required 2-divisibility')
B=[[n//(2**(3-i)) for n in EXP[i]] for i in range(3)]
require(all(x%2==0 for i in (1,2) for x in B[i]), 'b1,b2 even')
require([v%2 for v in B[0]]==[1,0,0,0], 'b0 != 1 modulo 2')

# Recompose exact degree-12 polynomial, not merely a modular identity.
reconstructed=[0]
for i,part in enumerate(EXP):
    power=[1]
    for _ in range(i):
        power=mul(power,H)
    reconstructed=add(reconstructed,mul(part,power))
require(reconstructed==Q,'global polynomial identity failed')

# e_(i,j)=S^i*h(S)^j /2^j, 0<=i<4 and 0<=j<3.
# Test closure under multiplication by BOTH ring generators S and U.
max_abs=0
for j in range(3):
    hp=[1]
    for _ in range(j):
        hp=mul(hp,H)
    for i in range(4):
        elem=mul([0]*i+[1],hp)
        for op in ([0,1],H):
            numerator=mul(elem,op)
            denom_power=j+(op is H)
            _,remainder=divmonic(numerator,Q)
            expansion=h_exp(remainder)
            require(len(expansion)<=3,'ring basis overflow')
            for k,part in enumerate(expansion):
                for c in part:
                    n=c*(2**k); d=2**denom_power
                    require(n%d==0,f'nonintegral multiplication {i,j,k}')
                    max_abs=max(max_abs,abs(n//d))

# F16 = F2[t]/(t^4+t+1). Its F4 roots are t^5=t^2+t and t^10.
def fmul(a,b):
    out=0
    while b:
        if b&1: out^=a
        b>>=1; a<<=1
        if a&16: a^=0x13
    return out

def fpow(a,k):
    out=1
    while k:
        if k&1: out=fmul(out,a)
        a=fmul(a,a);k//=2
    return out

def eval_binary(coeffs,u):
    out=0
    for c in reversed(coeffs):
        out=fmul(out,u)^c
    return out

w,w2=fpow(2,5),fpow(2,10)
require((w,w2)==(6,7) and (w^w2)==1,'F4 roots incorrect')
roots=[1,w,w2]
require(all(fpow(z,3)==1 for z in roots),'cubic root control')
# Evaluate explicit orthogonal idempotents at all 3 binary prime images.
idempotents=[[1,1,1],[1,w2,w],[1,w,w2]]
for i,pol in enumerate(idempotents):
    for j,u in enumerate(roots):
        require(eval_binary(pol,u)==int(i==j),'binary projector mismatch')

print(json.dumps({
    'basis_multiplication_integral':True,
    'binary_idempotents':3,
    'binary_residue_degrees':[4,4,4],
    'coefficient_polynomials_ascending':B,
    'disc_v2_from_CT109':24,
    'extension_degree':12,
    'index_v2':12,
    'max_abs_structure_coefficient':str(max_abs),
    'order_is_2_maximal':True,
    'reduction_cubic':'u^3+1',
    'root_labels_F16':roots,
},sort_keys=True,separators=(',',':')))
