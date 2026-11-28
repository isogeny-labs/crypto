"""Resultant-tree S3, with projective/infinity branch qualification."""
from .core import s3

def affine_chain_equations(xs,X,ys,a,b):
    """S3(x1,x2,T1),...,S3(T_(m-2),x_m,X), total degree <=4."""
    if len(xs)<3: return [s3(xs[0],xs[1],X,a,b)]
    if len(ys)!=len(xs)-2:raise ValueError('one intermediate per extra summand')
    triples=[(xs[0],xs[1],ys[0])]
    triples.extend((ys[i-1],xs[i+1],ys[i]) for i in range(1,len(ys)))
    triples.append((ys[-1],xs[-1],X))
    return [s3(*t,a,b) for t in triples]

def projective_pair_s3(x,y,U,V,a,b):
    """Homogeneous degree 2 in auxiliary U:V; S3(x,y,U/V)*V^2."""
    c0=(x*y-a)**2-4*b*(x+y)
    c1=-2*((x+y)*(x*y+a)+2*b)
    c2=(x-y)**2
    return c2*U**2+c1*U*V+c0*V**2
