from research.residue_quotient_20261116_v99b.kernel import pmul

def reverse(v,degree):return sum(((v>>i)&1)<<(degree-i)for i in range(degree+1))
def normalize(H,d):
    out=[]
    for j,v in enumerate(H):
        degree=(5-j)*(d-1);assert v.bit_length()<=degree+1;out.append(reverse(v,degree))
    return out

def evaluate(coefficients,x):
    out=0
    for a in reversed(coefficients):out=pmul(out,x)^a
    return out

def high_root(H,d):
    G=normalize(H,d);assert H[5]==1 and(G[4]&1)==1 and not any(v&1 for v in G[:4]);r=1
    for n in range(1,d):r^=((evaluate(G,r)>>n)&1)<<n
    residual=evaluate(G,r);assert not(residual&((1<<d)-1));root=reverse(r,d-1)
    assert residual==reverse(evaluate(H,root),5*(d-1))
    return root,residual>>d,r

def xor_poly(a,b,C,limit):return [C.xor(a[i]if i<len(a)else 0,b[i]if i<len(b)else 0)for i in range(min(limit,max(len(a),len(b))))]
def mul_poly(a,b,C,limit):
    out=[0]*min(limit,max(1,len(a)+len(b)-1))
    for i,u in enumerate(a):
        if u==0:continue
        for j,v in enumerate(b):
            if i+j>=limit:break
            if v:out[i+j]=C.xor(out[i+j],C.op('and',u,v))
    return out
def square_poly(a,limit):
    out=[0]*min(limit,max(1,2*len(a)-1))
    for i,v in enumerate(a):
        if 2*i<len(out):out[2*i]=v
    return out
def evaluate_circuit(G,r,C,limit):
    r2=square_poly(r,limit);r4=square_poly(r2,limit);r3=mul_poly(r2,r,C,limit);out=mul_poly(r4,r,C,limit)
    for term in[mul_poly(G[4],r4,C,limit),mul_poly(G[3],r3,C,limit),mul_poly(G[2],r2,C,limit),mul_poly(G[1],r,C,limit),G[0][:limit]]:out=xor_poly(out,term,C,limit)
    return out
def lift_circuit(G,d,C):
    assert G[5]==[1]and G[4][0]==1 and all(g[0]==0 for g in G[:4]);r=[1]
    for n in range(1,d):
        value=evaluate_circuit(G,r,C,n+1);r.append(value[n]if n<len(value)else 0)
    F=evaluate_circuit(G,r,C,5*(d-1)+1)
    return list(reversed(r)),F[d:],F[:d]
