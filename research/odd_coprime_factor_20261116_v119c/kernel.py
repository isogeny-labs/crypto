import itertools

def trim(a):
    a=list(a)
    while a and a[-1]==0:a.pop()
    return a

def psub(a,b,f):return trim([f.sub(a[i]if i<len(a)else 0,b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))])
def pmul(a,b,f):
    c=[0]*(len(a)+len(b)-1)if a and b else[]
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]=f.add(c[i+j],f.mul(x,y))
    return trim(c)
def multiply(A,B,f):
    out=[[]for _ in range(len(A)+len(B)-1)]
    for i,a in enumerate(A):
        for j,b in enumerate(B):out[i+j]=psub(out[i+j],[f.neg(v)for v in pmul(a,b,f)],f)
    return out

def initial(f):return [f.neg(1),3%f.p,f.neg(3%f.p),1]
def inverse_block(f):
    B=initial(f);S=[[0]*5 for _ in range(5)]
    for col in range(2):
        for j,b in enumerate(B):S[col+j][col]=b
    for col in range(3):S[col+2][col+2]=1
    M=[row+[int(i==j)for j in range(5)]for i,row in enumerate(S)]
    for j in range(5):
        pivot=next(i for i in range(j,5)if M[i][j]);M[j],M[pivot]=M[pivot],M[j];iv=f.inv(M[j][j]);M[j]=[f.mul(v,iv)for v in M[j]]
        for i in range(5):
            if i==j:continue
            scale=M[i][j];M[i]=[f.sub(a,f.mul(scale,b))for a,b in zip(M[i],M[j])]
    assert all(M[i][j]==int(i==j)for i in range(5)for j in range(5));return S,[row[5:]for row in M]

def matvec(M,v,f):
    out=[]
    for row in M:
        value=0
        for a,b in zip(row,v):value=f.add(value,f.mul(a,b))
        out.append(value)
    return out

def bounded(A,B,d):return all(not any(v[(degree-j)*(d-1)+1:])for factor,degree in [(A,2),(B,3)]for j,v in enumerate(factor))
def lift(F,d,f):
    B0=initial(f);assert [(v[0]if v else 0)for v in F]==[0,0]+B0;S,I=inverse_block(f);A=[[0],[0],[1]];B=[[b]for b in B0]
    for n in range(1,3*(d-1)+1):
        old=multiply(A,B,f);e=[f.sub(F[j][n]if n<len(F[j])else 0,old[j][n]if n<len(old[j])else 0)for j in range(5)];correction=matvec(I,e,f);assert matvec(S,correction,f)==e
        for j in range(2):A[j].append(correction[j])
        for j in range(3):B[j].append(correction[j+2])
    return {'A':A,'B':B,'bounded_factorization':bounded(A,B,d)and [trim(v)for v in multiply(A,B,f)]==[trim(v)for v in F],'support_complete':False}

def reference_d2(F,f):
    out=[]
    for a01,a02,a11 in itertools.product(range(f.q),repeat=3):
        A=[[0,a01,a02],[0,a11],[1]];work=[list(v)for v in F];B=[[]for _ in range(4)]
        for j in range(5,1,-1):
            B[j-2]=list(work[j])
            for i,a in enumerate(A):work[j-2+i]=psub(work[j-2+i],pmul(a,B[j-2],f),f)
        if not any(any(v)for v in work)and [(b[0]if b else 0)for b in B]==initial(f)and bounded(A,B,2):out.append((A,B))
    assert len(out)<=1;return out
