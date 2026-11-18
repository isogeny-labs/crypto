"""Direct exact original-r/s norm columns; same affine fiber contract."""
from research.general_binary_points_20261115_v52 import solver as old
norm=old.norm
c=old.c
coefficients=old.coefficients
point=old.point

def model(f,A,B,z,y,h,k,d):
 base=coefficients(f,A,B,z,y,h,k,0,0);rcols=[];scols=[]
 z2=f.square(z);z3=f.mul(z,z2);kz=f.mul(k,z);hz=f.mul(h,z)^k
 for i in range(f.degree):
  v=1<<i;q=f.square(v);zq=f.mul(z,q);he=f.mul(h,v)
  rcols.append((f.mul(z3,q),f.mul(z2,q)^f.mul(kz,v),zq^f.mul(hz,v),q^he,0,0))
  scols.append((zq,q^f.mul(k,v),he,0,0,0))
 columns=rcols+scols;rhs,width=norm.pack(f,base,d);values=[norm.pack(f,H,d)[0]for H in columns];sol=norm.linear.solve(norm.linear.rows_from(values,rhs,width),2*f.degree)
 return base,columns,sol
