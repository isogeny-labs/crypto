from research.three_point_line_20261115_v46 import solver as norm
from research.root_constant_image_20261115_v47 import solver as c

def coefficients(f,A,B,z,y,h,k,r,s):
 t=f.mul(f.square(z),z)^f.mul(r,f.square(z))^f.mul(s,z)^f.mul(f.mul(h,z)^k,y)
 return norm.norm(f,A,B,z,[r,s,t,h,k])

def model(f,A,B,z,y,h,k,d):
 base=coefficients(f,A,B,z,y,h,k,0,0);columns=[]
 for i in range(2*f.degree):
  r=(1<<i)if i<f.degree else 0;s=(1<<(i-f.degree))if i>=f.degree else 0;H=coefficients(f,A,B,z,y,h,k,r,s);columns.append(tuple(a^b for a,b in zip(base,H)))
 rhs,width=norm.pack(f,base,d);values=[norm.pack(f,H,d)[0]for H in columns];sol=norm.linear.solve(norm.linear.rows_from(values,rhs,width),2*f.degree)
 return base,columns,sol

def point(f,z,yT,h,k,S,x,y):
 G=f.mul(h,x)^k;target=f.mul(f.mul(h,z)^k,yT);term=0 if G==0 else f.mul(G,y)
 rhs=f.square(x)^f.mul(x,z)^f.square(z)^f.mul(target^term,f.inv(x^z))^f.mul(x^z,S['u0'])^S['v0']
 cols=[c.reduce(S,f.mul(x^z,u)^v)[0]for u,v in zip(S['U'],S['V'])];sol=norm.linear.solve(norm.linear.rows_from(cols,c.reduce(S,rhs)[0],f.degree),len(cols))
 if sol is None:return None
 def lift(bits):
  rem,tag=c.reduce(S,rhs^f.mul(x^z,norm.linear.apply(S['U'],bits))^norm.linear.apply(S['V'],bits));assert not rem
  return norm.linear.apply(S['outer'],bits)^norm.linear.apply(S['kernel'],tag)
 p=lift(sol['particular']);return dict(particular=p,basis=[lift(sol['particular']^b)^p for b in sol['basis']],rank=S['dimension']-len(sol['basis']))
