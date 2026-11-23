"""Exact last-border elimination and carry-free inverse Pascal transforms.

No elliptic points, target labels, random retries, or dense C accepted by solve.
The caller supplies LC-CR=UV^T, L=J^-1, R=gamma J^T.
"""
class Meter:
    def __init__(self, r):
        self.r=r; self.counts={}; self.blocks=[]
    def add(self,k,n=1): self.counts[k]=self.counts.get(k,0)+n
    def mul(self,a,b): self.add('field_products'); return a*b%self.r
    def plus(self,a,b): self.add('field_additions'); return (a+b)%self.r
    def sub(self,a,b): self.add('field_additions'); return (a-b)%self.r
    def inv(self,a):
        if not a%self.r: raise ZeroDivisionError('zero field pivot')
        self.add('field_inversions'); return pow(a,-1,self.r)
    def dot(self,a,b,reverse=False):
        assert len(a)==len(b)
        out=0
        for j in (range(len(a)-1,-1,-1) if reverse else range(len(a))):
            out=self.plus(out,self.mul(a[j],b[j]))
        return out

def packed_product(a,b,m):
    """Nonnegative canonical digits, bound excludes all integer carries."""
    r=m.r
    assert a and b and all(0<=x<r for x in a+b)
    bound=min(len(a),len(b))*(r-1)**2
    width=max(1,(bound.bit_length()+7)//8); radix=1<<(8*width)
    assert radix>bound
    ab=b''.join(x.to_bytes(width,'little') for x in a)
    bb=b''.join(x.to_bytes(width,'little') for x in b)
    ai=int.from_bytes(ab,'little'); bi=int.from_bytes(bb,'little')
    ci=ai*bi; length=len(a)+len(b)-1
    cb=ci.to_bytes(width*length,'little')
    out=[int.from_bytes(cb[i*width:(i+1)*width],'little')%r for i in range(length)]
    m.add('integer_multiplications');m.add('packing_bytes',len(ab)+len(bb))
    m.add('product_bytes',len(cb));m.add('coefficient_reductions',length)
    m.add('operand_bitlength_product_sum',ai.bit_length()*bi.bit_length())
    m.blocks.append(dict(a_length=len(a),b_length=len(b),digit_bytes=width,
                         a_bits=ai.bit_length(),b_bits=bi.bit_length(),product_bits=ci.bit_length()))
    return out

class Pascal:
    def __init__(self,r,n,m):
        assert m.r==r and 1<=n<=r
        self.r=r;self.n=n;self.m=m
        self.fact=[1]*n
        for i in range(1,n):self.fact[i]=m.mul(self.fact[i-1],i)
        self.ifac=[1]*n;self.ifac[-1]=m.inv(self.fact[-1])
        for i in range(n-1,0,-1):self.ifac[i-1]=m.mul(self.ifac[i],i)
        self.g=[m.sub(0,self.ifac[i]) if i%2 else self.ifac[i] for i in range(n)]
    def apply(self,v,transpose=False):
        assert len(v)==self.n and all(0<=x<self.r for x in v)
        m=self.m;n=self.n
        if transpose:
            a=[m.mul(self.fact[i],v[i]) for i in range(n)][::-1]
            w=packed_product(a,self.g,m)
            return [m.mul(self.ifac[j],w[n-1-j]) for j in range(n)]
        a=[m.mul(v[j],self.ifac[j]) for j in range(n)]
        w=packed_product(a,self.g,m)
        return [m.mul(self.fact[i],w[i]) for i in range(n)]

def j_inverse(v,m):
    out=[0]*len(v);last=0
    for i in range(len(v)-1,-1,-1):
        last=m.sub(v[i],last);out[i]=last
    return out

def solve(U,V,h,gamma,m,reverse=False,trace=None):
    """Return solution or first zero-pivot receipt. Never asserts singularity.

    Arithmetic on copies; trace callback is validation-only. Stored c rows
    and inverse pivots require quadratic slots for full back substitution.
    """
    r=m.r;n0=len(h);assert n0>0 and gamma%r not in (0,1)
    assert len(U)==len(V)==n0 and U and U[0]
    s=len(U[0]);assert all(len(row)==s for row in U+V)
    assert all(0<=x<r for row in U+V for x in row)
    assert all(0<=x<r for x in h)
    U=[row[:] for row in U];V=[row[:] for row in V];h=h[:]
    delta=m.sub(1,gamma);invdelta=m.inv(delta);saved=[];pivots=[]
    profile=dict(initial_generator_slots=2*n0*s,
                 backsub_scalar_slots=0,max_selected_scalar_slots=2*n0*s+n0)
    while h:
        n=len(h)-1;ub=U[-1];vb=V[-1]
        g22=m.dot(ub,vb,reverse);d=m.mul(g22,invdelta)
        pivots.append(d)
        if d==0:
            if trace is not None:trace(dict(size=n+1,pivot=d,status='zero_pivot',U=U,V=V,h=h))
            return dict(status='unresolved_zero_pivot',solution=None,pivots=pivots,
                        failed_size=n+1,profile=profile)
        g12=[m.dot(row,vb,reverse) for row in U[:-1]]
        g21=[m.dot(row,ub,reverse) for row in V[:-1]]
        rb=[m.plus(g12[i],g12[i+1]) if i+1<n else m.plus(g12[i],d) for i in range(n)]
        rc=g21[:]
        if n:rc[-1]=m.plus(rc[-1],m.mul(gamma,d))
        def bidiagonal(rhs):
            z=[0]*n;last=0
            for i in range(n-1,-1,-1):
                last=m.mul(invdelta,m.plus(rhs[i],m.mul(gamma,last)));z[i]=last
            return z
        b=bidiagonal(rb);c=bidiagonal(rc);di=m.inv(d)
        bu=[m.mul(x,di) for x in b];cv=[m.mul(x,di) for x in c]
        Un=[[m.sub(row[j],m.mul(bu[i],ub[j])) for j in range(s)] for i,row in enumerate(U[:-1])]
        Vn=[[m.sub(row[j],m.mul(cv[i],vb[j])) for j in range(s)] for i,row in enumerate(V[:-1])]
        hn=[m.sub(h[i],m.mul(bu[i],h[-1])) for i in range(n)]
        if trace is not None:trace(dict(size=n+1,pivot=d,b=b,c=c,Unew=Un,Vnew=Vn,hnew=hn,status='eliminated'))
        saved.append((c,di,h[-1]));profile['backsub_scalar_slots']+=n+2
        # Defined selected arrays only, not RSS or interpreter object overhead.
        selected=2*(n+1)*s+2*n*s+(n+1)+n+8*n+profile['backsub_scalar_slots']
        profile['max_selected_scalar_slots']=max(profile['max_selected_scalar_slots'],selected)
        U,V,h=Un,Vn,hn
    y=[]
    for c,di,last in reversed(saved):y.append(m.mul(di,m.sub(last,m.dot(c,y,reverse))))
    return dict(status='solved',solution=y,pivots=pivots,failed_size=None,profile=profile)
