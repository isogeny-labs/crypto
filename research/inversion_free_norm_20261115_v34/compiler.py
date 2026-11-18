"""Prepared trace-zero binary k=0norm chart; not executed."""
def compile_context(f,A,z,y):
 assert f.degree%2==1
 value=A^z;trace=0;x=value
 for _ in range(f.degree):trace^=x;x=f.square(x)
 if trace:return dict(status='unsupported_trace_one')
 c=0;x=value
 for _ in range((f.degree+1)//2):c^=x;x=f.square(f.square(x))
 assert f.square(c)^c==value
 return dict(status='ready',c=c,d=y^f.mul(z,c),z=z,y=y)
def to_original(f,ctx,h,u,v):
 assert ctx['status']=='ready'
 z=ctx['z'];r=z^f.mul(ctx['c'],h)^u;w=f.square(z)^f.mul(ctx['y'],h)^v;s=w^f.mul(z,r)
 return r,s
def norm_coefficients(f,ctx,h,u,v):
 assert ctx['status']=='ready'
 h2=f.square(h);u2=f.square(u);v2=f.square(v);z=ctx['z']
 return [f.mul(z,v2),v2,f.mul(z,u2)^f.mul(h,v)^f.mul(ctx['d'],h2),u2^f.mul(h,u),h2^h^z,1]
