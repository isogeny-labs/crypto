import sys, time
from sympy import symbols, Poly, expand, GF
p=int(sys.argv[1]) if len(sys.argv)>1 else 1009
x1,x2,x3,y1,y2,y3=symbols('x1 x2 x3 y1 y2 y3')
# E1: y^2=x^3+a x+b with 2-torsion x0; E = Velu codomain
a=5; x0=3; b=(-x0**3-a*x0)%p; t=(3*x0*x0+a)%p; w=x0*t%p
A=(a-5*t)%p; B=(b-7*w)%p
assert t and (4*a**3+27*b*b)%p and (4*A**3+27*B*B)%p
dom=lambda e: Poly(e,x1,x2,x3,modulus=p)
def S3(X1,X2,X3,a_,b_):
    return (X1-X2)**2*X3**2-2*((X1+X2)*(X1*X2+a_)+2*b_)*X3+(X1*X2-a_)**2-4*b_*(X1+X2)
S_E=dom(S3(x1,x2,x3,A,B))
# pulled back: X_i = (x_i^2 - x0 x_i + t)/(x_i - x0); clear (x_i-x0)^2 per variable
num=lambda x:(x*x-x0*x+t); den=lambda x:(x-x0)
X=[num(v) for v in (x1,x2,x3)]; D=[den(v) for v in (x1,x2,x3)]
Sp=dom(expand((X[0]*D[1]-X[1]*D[0])**2*X[2]**2 - 2*((X[0]*D[1]+X[1]*D[0])*(X[0]*X[1]+A*D[0]*D[1])+2*B*D[0]**2*D[1]**2)*X[2]*D[2] + ((X[0]*X[1]-A*D[0]*D[1])**2-4*B*D[0]*D[1]*(X[0]*D[1]+X[1]*D[0]))*D[2]**2))
print("deg per var  plain S3 on E:", [S_E.degree(v) for v in (x1,x2,x3)], " total", S_E.total_degree())
print("deg per var  pulled-back S3:", [Sp.degree(v) for v in (x1,x2,x3)], " total", Sp.total_degree(), " terms", len(Sp.terms()), "vs", len(S_E.terms()))
# Net polynomial Psi_{1,1,1}: zero iff P+Q+R=O; numerator is the collinearity determinant
# Det = y1(x2-x3)+y2(x3-x1)+y3(x1-x2). Norm over y2->±y2, y3->±y3, y1->±y1, then y_i^2 -> f(x_i).
def norm_det(f):
    import itertools
    prod=1
    for s2,s3 in itertools.product((1,-1),repeat=2):
        prod*= y1*(x2-x3)+s2*y2*(x3-x1)+s3*y3*(x1-x2)
    prod=expand(prod)
    # remaining odd powers of y1 cancel under y1->-y1 by symmetry; substitute squares
    q=Poly(prod,y1,y2,y3)
    out=0
    for (e1,e2,e3),c in q.terms():
        assert e1%2==0 and e2%2==0 and e3%2==0
        out+=c*f(x1)**(e1//2)*f(x2)**(e2//2)*f(x3)**(e3//2)
    return dom(expand(out))
fE=lambda x:x**3+A*x+B
N=norm_det(fE)
V=dom((x1-x2)**2*(x2-x3)**2*(x3-x1)**2)
q,r=N.div(S_E*V)
assert r.is_zero and q.as_expr()==1
print("Norm(Psi_111) on E  ==  c * Vandermonde^2 * S3 ? remainder zero:", r.is_zero, " quotient:", q.as_expr())
# Pulled-back net: evaluate Det at phi(P_i): X_i, Y_i = y_i*(1 - t/(x_i-x0)^2) ; y_i^2 = f1(x_i)
f1=lambda x:x**3+a*x+b
t0=time.time()
import itertools
prod=1
for s2,s3 in itertools.product((1,-1),repeat=2):
    Ys=[y1*(D[0]**2-t), s2*y2*(D[1]**2-t), s3*y3*(D[2]**2-t)]  # times D_i^2
    # Det with X_i=num/den, Y_i=Ys/D^2 ; multiply through by D1^2 D2^2 D3^2
    Xc=[X[i]*D[i] for i in range(3)]  # X_i * D_i^2
    prod*= Ys[0]*(Xc[1]*D[2]**2-Xc[2]*D[1]**2)*1 + Ys[1]*(Xc[2]*D[0]**2-Xc[0]*D[2]**2) + Ys[2]*(Xc[0]*D[1]**2-Xc[1]*D[0]**2)
q4=Poly(expand(prod),y1,y2,y3); Np=0
for (e1,e2,e3),c in q4.terms():
    assert e1%2==0 and e2%2==0 and e3%2==0
    Np+=c*f1(x1)**(e1//2)*f1(x2)**(e2//2)*f1(x3)**(e3//2)
Np=dom(expand(Np))
q,r=Np.div(Sp)
print("Norm(pulled-back Psi_111) divisible by pulled-back S3:", r.is_zero)
# identify cofactor: should be pulled-back Vandermonde^2 times powers of (x_i-x0) and (x_i^2-... ) kernel/2-torsion factors
from sympy import factor_list
for v,o in ((x1,(x2,x3)),):
    pass
print("deg per var norm(pulled Psi):", [Np.degree(v) for v in (x1,x2,x3)], " cofactor:", [q.degree(v) for v in (x1,x2,x3)], f"({time.time()-t0:.1f}s)")
# check pulled-back S3 against direct composition at random points
import random
R=random.Random(0)
for _ in range(5):
    v=[R.randrange(p) for _ in range(3)]
    if any(c==x0 for c in v): continue
    Xv=[(c*c-x0*c+t)*pow(c-x0,-1,p)%p for c in v]
    lhs=Sp.eval(dict(zip((x1,x2,x3),v)))%p
    rhs=S3(*Xv,A,B)*pow((v[0]-x0)*(v[1]-x0)*(v[2]-x0),2,p)%p
    assert lhs==rhs, (lhs,rhs)
print("pulled-back S3 == (prod D_i^2) * S3(phi_x(x_i)) at random points: ok")
Vp=dom(expand(((X[0]*D[1]-X[1]*D[0])*(X[1]*D[2]-X[2]*D[1])*(X[2]*D[0]-X[0]*D[2]))**2))
q2,r2=q.div(Vp)
K=dom(expand(((x1-x0)*(x2-x0)*(x3-x0))**2))
assert r2.is_zero and q2==K
print("cofactor == prod (x_i-x0)^2 * pulled-back Vandermonde^2: exact")
