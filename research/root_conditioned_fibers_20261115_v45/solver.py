"""Exact root-conditioned affine fibers; no generic-rank assumption."""
from research.norm_block_reuse_20261115_v36 import backend as b
from research.inversion_free_norm_20261115_v34 import compiler as c

def evaluate(f,H,x):
 value=0
 for a in reversed(H):value=f._raw_mul(value,x)^a
 return value

def model(f,ctx,h,fiber):
 n=f.degree;mask=(1<<n)-1;uv=fiber['particular'];base=c.norm_coefficients(f,ctx,h,uv&mask,uv>>n);cols=[]
 for v in fiber['basis']:
  H=c.norm_coefficients(f,ctx,h,(uv^v)&mask,(uv^v)>>n);cols.append(tuple(a^bb for a,bb in zip(H,base)))
 return tuple(base),cols

def condition(f,base,cols,x):
 values=[evaluate(f,H,x)for H in cols];rhs=evaluate(f,base,x)
 return b.solve(b.rows_from(values,rhs,f.degree),len(cols))

def parameters(sol):
 if sol is None:return []
 assert len(sol['basis'])<=16
 return [sol['particular']^b.apply(sol['basis'],i)for i in range(1<<len(sol['basis']))]
