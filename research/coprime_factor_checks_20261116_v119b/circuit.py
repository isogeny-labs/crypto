from research.simple_prefix_lift_20261116_v117b.kernel import normalize

def coefficient(A,B,j,n,C):
    value=0
    for i,a in enumerate(A):
        k=j-i
        if k<0 or k>=len(B):continue
        b=B[k]
        for p in range(max(0,n-len(b)+1),min(n,len(a)-1)+1):value=C.xor(value,C.op('and',a[p],b[n-p]))
    return value

def lift(F,d,C):
    assert [v[0]for v in F]==[0,0,1,1,1,1];A=[[0],[0],[1]];B=[[1],[1],[1],[1]];D=3*(d-1)
    for n in range(1,D+1):
        e=[C.xor(F[j][n]if n<len(F[j])else 0,coefficient(A,B,j,n,C))for j in range(5)];a1=C.xor(e[0],e[1]);a=[e[0],a1];b=[C.xor(e[1],e[2]),C.xor(e[1],e[3]),C.xor(a1,e[4])]
        for j,value in enumerate(a):A[j].append(value)
        for j,value in enumerate(b):B[j].append(value)
    bounds=[]
    for factor,degree in [(A,2),(B,3)]:
        for j,v in enumerate(factor):bounds.extend(v[(degree-j)*(d-1)+1:])
    tail=[C.xor(F[j][n]if n<len(F[j])else 0,coefficient(A,B,j,n,C))for j in range(5)for n in range(D+1,5*(d-1)+1)]
    return A,B,bounds+tail

def normalized_affine(raw,particular,columns,d,C):
    base=normalize(raw(particular),d);origin=raw(0);directions=[normalize([a^b for a,b in zip(raw(v),origin)],d)for v in columns];F=[]
    for j,a in enumerate(base):
        bits=[]
        for bit in range((5-j)*(d-1)+1):
            value=(a>>bit)&1
            for i,row in enumerate(directions):
                if row[j]>>bit&1:value=C.xor(value,2+i)
            bits.append(value)
        F.append(bits)
    return F
