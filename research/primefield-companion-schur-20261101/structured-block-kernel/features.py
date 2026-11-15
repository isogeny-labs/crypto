"""Public character-boundary factors; no dense feature matrix is built here."""
from itertools import combinations
from kernel import Pascal,j_inverse

def power(a,e,m):
    z=1
    while e:
        if e&1:z=m.mul(z,a)
        e//=2
        if e:a=m.mul(a,a)
    return z

def build(r,q,gamma,weights,m):
    assert 1<=q<r and gamma not in (0,1)
    patterns=[(0,)]+[(0,d) for d in range(1,q)]+[(0,d,e) for d in range(1,q) for e in range(d+1,q)]
    assert len(weights)==1+len(patterns) and all(0<w<r for w in weights)
    chi=[power(k,(r-1)//2,m) for k in range(r)]
    pascal=Pascal(r,r,m);columns=[];coefficients=[]
    columns.append(pascal.apply([1]*r));coefficients.append(m.mul(m.sub(1,gamma),weights[0]))
    labels=[()];diagonal=[weights[0]]
    for pattern,w in zip(patterns,weights[1:]):
        ell=q-max(pattern)
        for shift in (0,ell):
            v=[]
            for k in range(r):
                a=1
                for d in pattern:a=m.mul(a,chi[(k+shift+d)%r])
                v.append(a)
            columns.append(pascal.apply(v))
        coefficients.extend([w,m.sub(0,m.mul(w,power(gamma,ell,m)))])
        a=w
        for shift in range(ell):
            labels.append(tuple(shift+d for d in pattern));diagonal.append(a)
            if shift+1<ell:a=m.mul(a,gamma)
    V=[list(row) for row in zip(*columns)];ucols=[]
    for z,a in zip(columns,coefficients):ucols.append(j_inverse([m.mul(a,v) for v in z],m))
    U=[list(row) for row in zip(*ucols)]
    return dict(U=U,V=V,labels=labels,diagonal=diagonal,pascal=pascal,
                patterns=len(patterns),generator_width=len(columns))
