#!/usr/bin/env python3
"""CT107: exact prime-index test; integer-first polynomials, no external CAS."""
import json

def ck(c, m):
    if not c: raise ArithmeticError(m)
def trim(v):
    v=list(v)
    while len(v)>1 and not v[-1]: v.pop()
    return v
def add(a,b):
    return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0)
                 for i in range(max(len(a),len(b)))])
def neg(a): return [-z for z in a]
def sub(a,b): return add(a,neg(b))
def mul(a,b):
    c=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b): c[i+j]+=x*y
    return trim(c)
def ppow(a,n):
    z=[1]
    while n:
        if n&1: z=mul(z,a)
        a=mul(a,a); n>>=1
    return z

D=[-79,-40,-4,4]
f3=[-21,-237,-60,-4,3]
f4=[-5821,-1496,-210,-790,-100,-4,2]
f5=sub(mul(ppow(D,2),f4),ppow(f3,3))
f6=mul(f3,sub(f5,ppow(f4,2)))
f7=sub(mul(f5,ppow(f3,3)),mul(ppow(D,2),ppow(f4,3)))
f11=sub(mul(f7,ppow(f5,3)),mul(mul(ppow(D,2),f4),ppow(f6,3)))
ck(len(f11)==61 and f11[-1]==11,"division polynomial")
X=[0,1]
N=[X,sub(mul(X,D),f3),sub(mul(X,ppow(f3,2)),mul(D,f4)),
   sub(mul(X,mul(D,ppow(f4,2))),mul(f5,f3)),
   sub(mul(X,ppow(f5,2)),mul(D,mul(f6,f4)))]
B=[[1],D,ppow(f3,2),mul(D,ppow(f4,2)),ppow(f5,2)]

def divrem(a,b,p):
    a=trim([z%p for z in a]);b=trim([z%p for z in b])
    lead=pow(b[-1],-1,p)
    q=[0]*max(1,len(a)-len(b)+1)
    while len(a)>=len(b) and a!=[0]:
        c=a[-1]*lead%p; k=len(a)-len(b); q[k]=c
        for i,v in enumerate(b): a[k+i]=(a[k+i]-c*v)%p
        a=trim(a)
    return trim(q),a
def rem(a,b,p): return divrem(a,b,p)[1]
def inverse(a,mod,p):
    r0=trim([z%p for z in mod]);r1=rem(a,mod,p)
    s0=[0];s1=[1]
    while r1!=[0]:
        q,r=divrem(r0,r1,p)
        r0,r1=r1,r;s0,s1=s1,rem(sub(s0,mul(q,s1)),mod,p)
    ck(len(r0)==1,"nonunit denominator")
    return rem([z*pow(r0[0],-1,p) for z in s0],mod,p)
def gcdp(a,b,p):
    while trim([x%p for x in b])!=[0]:
        a,b=b,rem(a,b,p)
    a=trim([x%p for x in a])
    return [x*pow(a[-1],-1,p)%p for x in a]
def ring(p,mod):
    mod=trim([z%p for z in mod])
    def red(a): return rem(a,mod,p)
    def plus(a,b): return red(add(a,b))
    def times(a,b): return red(mul(a,b))
    def negp(a): return red(neg(a))
    def power(a,n):
        z=[1]
        while n:
            if n&1: z=times(z,a)
            a=times(a,a); n>>=1
        return z
    s=[0]
    for n,d in zip(N,B):
        a=red(n); b=red(d); bi=inverse(b,mod,p)
        ck(times(b,bi)==[1],"nonunit division polynomial")
        s=plus(s,times(a,bi))
    return s,red,plus,times,negp,power
def evaluate(c,x,plus,times):
    y=[0]
    for a in reversed(c): y=plus(times(y,x),[a])
    return y
def minpoly_in_algebra(p,mod,limit=12):
    s,red,plus,times,negp,power=ring(p,mod)
    d=len(mod)-1; powers=[];z=[1]
    for k in range(limit+1):
        powers.append((z+[0]*(d-len(z)))[:d])
        mat=[row[:] for row in powers]
        transform=[[int(i==j) for j in range(len(powers))] for i in range(len(powers))]
        rank=0
        for col in range(d):
            row=next((i for i in range(rank,len(mat)) if mat[i][col]%p),None)
            if row is None: continue
            mat[rank],mat[row]=mat[row],mat[rank]
            transform[rank],transform[row]=transform[row],transform[rank]
            factor=pow(mat[rank][col],-1,p)
            mat[rank]=[x*factor%p for x in mat[rank]]
            transform[rank]=[x*factor%p for x in transform[rank]]
            for i in range(len(mat)):
                if i==rank: continue
                c=mat[i][col]
                if c:
                    mat[i]=[(x-c*y)%p for x,y in zip(mat[i],mat[rank])]
                    transform[i]=[(x-c*y)%p for x,y in zip(transform[i],transform[rank])]
            rank+=1
            if rank==len(mat): break
        if rank<len(mat):
            for i in range(rank,len(mat)):
                c=transform[i]
                if c[-1]%p:
                    factor=pow(c[-1],-1,p)
                    c=[v*factor%p for v in c]
                    ck(evaluate(c,s,plus,times)==[0],"relation failed")
                    return c
        z=times(z,s)
    raise ArithmeticError("no dependence within limit")
def orbit_minpoly(p,mod):
    s,red,plus,times,negp,power=ring(p,mod)
    roots=[s]; z=power(s,p)
    while z!=s and len(roots)<len(mod):
        roots.append(z);z=power(z,p)
    ck(z==s,"nonfinite Frobenius orbit")
    c=[[1]]
    for x in roots:
        out=[[0] for _ in range(len(c)+1)]
        for i,a in enumerate(c):
            out[i]=plus(out[i],negp(times(a,x)))
            out[i+1]=plus(out[i+1],a)
        c=out
    ck(all(len(a)==1 for a in c),"norm not in base field")
    return [a[0] for a in c]
def irreducible5(f):
    _,_,_,_,_,power=ring(59,f)
    ck(power(X,59**5)==X and gcdp(f,sub(power(X,59),X),59)==[1],
       "quintic irreducibility")
def irreducible20(f):
    _,_,_,_,_,power=ring(2,f)
    ck(power(X,2**20)==X,"binary Frobenius return")
    for d in (4,10):
        ck(gcdp(f,sub(power(X,2**d),X),2)==[1],"binary irreducibility")
q59=minpoly_in_algebra(59,f11)
q13=minpoly_in_algebra(13,f11)
q4=minpoly_in_algebra(2,f11)
expected59=[19,26,16,12,35,43,33,58,29,29,46,39,1]
expected13=[2,11,0,1,0,3,4,8,6,2,12,6,1]
quartic=[1,1,0,0,1]
ck(q59==expected59 and q13==expected13 and q4==quartic,
   "unexpected reduced minimal polynomial")
ck(gcdp(f11,[i*f11[i] for i in range(1,len(f11))],59)==[1],
   "source point polynomial not squarefree")
g1=[41,22,35,11,14,1];g2=[16,2,57,45,21,1]
for f in (g1,g2): irreducible5(f)
ck([z%59 for z in mul(mul(mul([27,1],[36,1]),g1),g2)]==q59,
   "source line factorization")
ck(gcdp(q59,[i*q59[i] for i in range(1,len(q59))],59)==[1],
   "source line polynomial not squarefree")
def bits(w): return [(w>>i)&1 for i in range(w.bit_length())]
factors=[bits(v) for v in (0x130027,0x131203,0x131211)]
for f in factors: irreducible20(f)
ck([z%2 for z in mul(mul(factors[0],factors[1]),factors[2])]
   ==[z%2 for z in f11],"binary x factorization")
for f in factors:
    ck(orbit_minpoly(2,f)==quartic,"binary residue quartic mismatch")
q2=[1]
for i in range(3):q2=[z%2 for z in mul(q2,quartic)]
ck(q2==[1,1,1,1,1,0,1,0,1,1,0,0,1],"binary norm fifth-root")
print(json.dumps({
"case":"ct107-line-field-prime-index",
"global_line_degree":12,
"source_prime":59,
"source_minpoly_asc":q59,
"source_factor_degrees":[1,1,5,5],
"source_rational_line_invariants":[23,32],
"source_index_59_adic_valuation":0,
"binary_prime":2,
"binary_x_factor_degrees":[20,20,20],
"binary_invariant_minpoly_asc":quartic,
"binary_global_minpoly_reduction_asc":q2,
"binary_line_factor_degrees":[4,4,4],
"binary_residue_subalgebra_dimension":4,
"binary_index_2_adic_lower_bound":8,
"global_char0_minpoly_expanded":False,
"complete_transfer_evaluated":False,
},sort_keys=True,separators=(",",":")))
