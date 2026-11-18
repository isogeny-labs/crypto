"""All-k trace-zero binary norm chart; target validity is a precondition."""
from research.inversion_free_norm_20261115_v34.compiler import compile_context

def to_original(f,ctx,h,k,u,v):
 assert ctx['status']=='ready'
 z=ctx['z'];r=z^f.mul(ctx['c'],h)^u
 w=f.square(z)^f.mul(ctx['y'],h)^f.mul(ctx['c'],k)^v
 return r,w^f.mul(z,r)

def norm_coefficients(f,ctx,h,k,u,v):
 assert ctx['status']=='ready'
 z=ctx['z'];d=ctx['d'];h2=f.square(h);k2=f.square(k);u2=f.square(u);v2=f.square(v)
 return [f.mul(z,v2)^f.mul(d,k2),v2^f.mul(k,v),f.mul(z,u2)^f.mul(h,v)^f.mul(d,h2)^f.mul(k,u)^k2,u2^f.mul(h,u)^k,h2^h^z,1]
