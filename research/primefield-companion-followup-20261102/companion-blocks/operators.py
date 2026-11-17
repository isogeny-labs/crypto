"""Prime-field companion operators for arbitrary dimensions and distinct wraps."""
from kernel import packed_product

def convolution(x,y,m):
    out=packed_product(x,y,m)
    d=max(1,((min(len(x),len(y))*(m.r-1)**2).bit_length()+7)//8)
    m.add('input_integer_encodings',2);m.add('decoded_bytes',len(out)*d)
    return out

def extract(p,n,wrap,m):
    return [p[n-1]]+[m.plus(p[n-1+j],m.mul(wrap,p[j-1])) for j in range(1,n)]

def reduce(p,n,wrap,m):
    return [m.plus(p[i],m.mul(wrap,p[n+i])) if i<n-1 else p[i] for i in range(n)]

def apply(G,K,a,b,x,m,transpose=False):
    n=len(x);r=m.r
    assert r>3 and n>0 and 0<=a<r and 0<=b<r and a!=b
    assert len(G)==len(K)==n and G[0]
    s=len(G[0]);assert all(len(row)==s for row in list(G)+list(K))
    assert all(type(v)is int and 0<=v<r for row in list(G)+list(K) for v in row)
    assert all(type(v)is int and 0<=v<r for v in x)
    inv=m.inv(m.sub(a,b));out=[0]*n
    for j in range(s):
        g=[row[j] for row in G];k=[row[j] for row in K]
        if not transpose:
            t=extract(convolution(k,list(x)[::-1],m),n,b,m)
            y=reduce(convolution(g,t[::-1],m),n,a,m)
        else:
            z=extract(convolution(g[::-1],list(x),m),n,a,m)
            y=reduce(convolution(k[::-1],z[::-1],m),n,b,m)[::-1]
        out=[m.plus(u,v) for u,v in zip(out,y)]
    return [m.mul(inv,u) for u in out]

class BoundaryCache:
    def __init__(self,G,K,a,b,m):
        self.G=G;self.K=K;self.a=a;self.b=b;self.m=m;self.saved={}
    def get(self,transpose,index):
        n=len(self.G);assert 0<=index<n
        self.m.add('boundary_requests');key=(transpose,index)
        if key not in self.saved:
            x=[int(i==index) for i in range(n)]
            self.saved[key]=apply(self.G,self.K,self.a,self.b,x,self.m,transpose)
            self.m.add('boundary_queries')
        else:self.m.add('boundary_cache_hits')
        return self.saved[key]
    def profile(self):
        return dict(cached_vectors=len(self.saved),cached_field_slots=sum(map(len,self.saved.values())))

def principal(G,K,a,b,n,c,d,boundary,m):
    """Leading principal block generators; boundary supplies public row/column queries."""
    size=len(G);assert 0<n<size and len(K)==size and 0<=c<m.r and 0<=d<m.r and c!=d
    last=boundary.get(True,size-1)[:n]
    crossing=boundary.get(False,n)[:n]
    row=boundary.get(True,n-1)[:n]
    first=boundary.get(False,0)[:n]
    na=m.sub(0,a)
    U=[];V=[]
    for i in range(n):
        U.append(list(G[i])+[na if i==0 else 0,crossing[i],c if i==0 else 0,m.sub(0,m.mul(d,first[i]))])
        V.append(list(K[i])+[last[i],int(i==n-1),row[i],int(i==n-1)])
    return U,V

def inverse_generators(G,K,solve,solve_transpose,m):
    """Conditional transport through supplied inverse oracles; does not implement a solver."""
    n=len(G);assert n and len(K)==n and G[0]
    s=len(G[0]);assert all(len(row)==s for row in G+K)
    U=[[0]*s for _ in range(n)];V=[[0]*s for _ in range(n)]
    for j in range(s):
        m.add('inverse_oracle_calls');u=solve([row[j] for row in G])
        m.add('inverse_transpose_oracle_calls');v=solve_transpose([row[j] for row in K])
        assert len(u)==len(v)==n
        for i in range(n):U[i][j]=m.sub(0,u[i]);V[i][j]=v[i]
    return U,V
