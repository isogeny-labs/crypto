# Symbolic integer polynomial identities only; not a finite-field experiment.
import sympy as s
x,q,v,m,a,b,u,y=s.symbols('x q v m a b u y')
f=lambda x:x**3+a*x+b
R=x*x+(q-m*m)*x+q*q+a+q*m*m-2*v*m
Delta=m**4-6*q*m*m+8*v*m-3*q*q-4*a
checks=[s.expand(f(x)-(v+m*(x-q))**2-(x-q)*R-(f(q)-v*v)),s.expand(s.discriminant(R,x)-Delta),s.expand((2*x+q-m*m)**2-4*R-Delta),s.expand(((u-q)*m+v)**2-y*y-((u-q)*m+v-y)*((u-q)*m+v+y))]
assert checks==[0]*4,checks
print('SymPy',s.__version__,'four symbolic integer-ring residuals:',checks)
print('No finite-field points, random draws or timing.')
