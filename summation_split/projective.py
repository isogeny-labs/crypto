"""Exact S3 projective lift over Fp2; no factor-base restriction.

Projective infinity is a separate chart; finite chart constraints S3 have
maximum total degree four. This is a finite-field root-propagation oracle,
NOT a Groebner solver, and its work grows with the number of witnesses.
"""
from functools import lru_cache
from .field import Field

def coeffs(F,x,y,a,b):
    mul,add,sub,sq=F.mul,F.add,F.sub,F.sq
    A=(a,0);B=(b,0)
    xy=mul(x,y);xy_a=add(xy,A);sumxy=add(x,y)
    c2=sq(sub(x,y))
    c1=mul((-2,0),add(mul(sumxy,xy_a),mul((2,0),B)))
    c0=sub(sq(sub(xy,A)),mul((4,0),mul(B,sumxy)))
    return (c0,c1,c2)

@lru_cache(maxsize=10)
def sqrt_table(p):
    F=Field(p);t={}
    for u in range(p):
        for v in range(p):
            z=(u,v);t.setdefault(F.sq(z),[]).append(z)
    return t

def roots_quadratic_projective(F,poly):
    """Zero set of c0*V^2+c1*U*V+c2*U^2 in P1(Fp2), with None=∞."""
    c0,c1,c2=poly
    if c2==F.zero:
        if c1==F.zero:
            if c0==F.zero:
                raise ValueError('identically zero quadratic needs chart-wide witness')
            return {None}
        return {None,F.div(F.neg(c0),c1)}
    D=F.sub(F.sq(c1),F.mul((4,0),F.mul(c2,c0)))
    ds=sqrt_table(F.p).get(D,[])
    den=F.mul((2,0),c2)
    return {F.div(F.sub(d,F.add(c1,F.zero)),den) for d in ds}

def projective_chain_roots(p,a,b,xs):
    """Exact projected Fp X roots of a chart-complete chained S3 system.

    The first pair S3 admits P1(Fp2) intermediates, and each later S3
    propagates P1 witnesses. For a previous infinity and finite next leaf,
    the next partial abscissa is that leaf (O+P=P). Final infinity is zero.
    """
    F=Field(p)
    states=roots_quadratic_projective(F,coeffs(F,(xs[0],0),(xs[1],0),a,b))
    for x in xs[2:]:
        nxt=set()
        for t in states:
            if t is None:nxt.add((x,0))
            else:nxt.update(roots_quadratic_projective(F,coeffs(F,t,(x,0),a,b)))
        states=nxt
    out=set()
    for X in range(p):
        if (X,0) in states:out.add(X)
    return out

def affine_fp_chain_roots(p,a,b,xs):
    """Intentionally lossy naive finite-Fp-only affine S3 chain."""
    F=Field(p)
    states={t for t in range(p) if eval_s3_field(F,((xs[0],0),(xs[1],0),(t,0)),a,b)==F.zero}
    for x in xs[2:]:
        states={z for t in states for z in range(p) if eval_s3_field(F,((t,0),(x,0),(z,0)),a,b)==F.zero}
    return states

def eval_s3_field(F,xyz,a,b):
    x,y,z=xyz;c0,c1,c2=coeffs(F,x,y,a,b)
    return F.add(c0,F.mul(z,F.add(c1,F.mul(c2,z))))
