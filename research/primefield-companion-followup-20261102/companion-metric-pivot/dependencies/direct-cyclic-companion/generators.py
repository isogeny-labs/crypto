"""Unexecuted public companion-generator constructor; no linear solve."""
def power(a,e,m):
    out=1
    while e:
        if e&1:out=m.mul(out,a)
        e//=2
        if e:a=m.mul(a,a)
    return out

def build(r,q,gamma,weights,m):
    assert m.r==r and r>3 and 1<=q<r and 1<gamma<r
    patterns=[(0,)]+[(0,d) for d in range(1,q)]+[(0,d,e) for d in range(1,q) for e in range(d+1,q)]
    assert len(weights)==len(patterns)+1 and all(0<w<r for w in weights)
    chi=[power(k,(r-1)//2,m) for k in range(r)]
    vectors=[[1]*r];coeff=[m.mul(m.sub(1,gamma),weights[0])]
    labels=[()];diagonal=[weights[0]]
    for P,w in zip(patterns,weights[1:]):
        ell=q-max(P)
        for shift in (0,ell):
            col=[]
            for k in range(r):
                value=1
                for d in P:value=m.mul(value,chi[(k+shift+d)%r])
                col.append(value)
            vectors.append(col)
        coeff.extend([w,m.sub(0,m.mul(w,power(gamma,ell,m)))])
        a=w
        for shift in range(ell):
            labels.append(tuple(shift+d for d in P));diagonal.append(a)
            if shift+1<ell:a=m.mul(a,gamma)
    igamma=m.inv(gamma);scales=[1]
    for i in range(1,r):scales.append(m.mul(scales[-1],igamma))
    G=[];K=[];rhs=[]
    for i in range(r):
        G.append([m.sub(0,m.mul(scales[i],m.mul(c,col[r-1-i]))) for c,col in zip(coeff,vectors)])
        K.append([col[(r-2-i)%r] for col in vectors])
        rhs.append(m.mul(scales[i],r-1-i))
    return dict(left_wrap=gamma,right_wrap=1,G=G,K=K,rhs=rhs,
                inverse_diagonal=scales,labels=labels,diagonal=diagonal,
                generator_width=len(vectors))
