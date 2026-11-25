import sympy as s
q,v,a,t,y=s.symbols('q v a t y')
f=v**2+(3*q*q+a)*t+3*q*t*t+t**3
A=-2*v/t
C=-(3*q*q+a)/t-3*q-t
P=[s.Integer(2),A]
for j in range(2,13):
    P.append(s.expand(A*P[-1]-C*P[-2]))
for j in range(13):
    direct=2*sum(s.binomial(j,2*h)*(-v)**(j-2*h)*f**h for h in range(j//2+1))/t**j
    assert s.expand(P[j]-direct)==0
    assert all(-j<=term.as_powers_dict().get(t,0)<=j//2 for term in s.Add.make_args(P[j]))
print('sympy',s.__version__,'13 recurrence identities and 13 Laurent support checks passed')
p=2147483647
D=961
m=max(j for j in range(1,D+1) if (p-1)%j==0)
g=5
h=pow(g,(p-1)//m,p)
assert m==906 and pow(h,m,p)==1
assert all(pow(h,m//ell,p)!=1 for ell in (2,3,151))
assert pow(g,m,p)!=1
print('integer arithmetic',m,(p-1)//m,h,[pow(h,m//ell,p) for ell in (2,3,151)],pow(g,m,p))
