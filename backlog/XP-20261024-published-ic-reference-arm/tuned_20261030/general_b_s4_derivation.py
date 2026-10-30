import sympy as sp
from sympy.polys.polyfuncs import symmetrize
x1,x2,x3,x4,X,b,e1,e2,e3=sp.symbols('x1 x2 x3 x4 X b e1 e2 e3')
S3=lambda p,q,r: (p*q+p*r+q*r)**2+p*q*r+b
R=sp.resultant(S3(x1,x2,X),S3(x3,x4,X),X)
R=sp.Poly(sp.expand(R),x1,x2,x3,x4,b,modulus=2).as_expr()
sym,rem,defs=None,None,None
res=symmetrize(sp.expand(R),[x1,x2,x3],formal=True)
expr,rest,defs=res[0],res[1],res[2]
assert rest==0, rest
m={s:v for s,v in zip([d[0] for d in defs],[e1,e2,e3])}
f=sp.Poly(sp.expand(expr.subs(m)),e1,e2,e3,x4,b,modulus=2)
print('general:',f.as_expr())
coded=x4**4+e1**4+e3**4+e2**4*x4**4+e3**3*x4+e3*e2**2*x4**3+e3*e1**2*x4+e3*x4**3+e1**2*e3**2*x4**2+e3**2*x4**4+e3**2+e2**2*x4**2
print('b=1 matches coded:', sp.Poly(f.as_expr().subs(b,1)-coded,e1,e2,e3,x4,modulus=2).is_zero)
