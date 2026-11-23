from research.image_cells_20261115_v67.algebra import pmul,trim,evaluate

def add(a,b,f):
 out=[0]*max(len(a),len(b))
 for i,x in enumerate(a):out[i]=x
 for i,x in enumerate(b):out[i]=f.add(out[i],x)
 return trim(out)
def neg(a,f):return [f.neg(x)for x in a]
def sub(a,b,f):return add(a,neg(b,f),f)
def scale(a,c,f):return trim([f.mul(x,c)for x in a])
def square(a,f):return pmul(a,a,f)
def translate(H,u,f):
 out=[]
 for c in H[::-1]:out=add(pmul(out,[u,1],f),[c],f)
 return out

def j2(H,f):
 m=len(H)-1
 if f.p!=2 or m%2!=1:raise ValueError('binary odd-degree invariant')
 return H[-3]^ (f.sq(H[-2])if(m*(m-1)//2)%2 else 0)

def quad_coeffs(x,y,A,B,f):
 xy=pmul(x,y,f)
 if f.p==2:return add(square(x,f),square(y,f),f),xy,add(square(xy,f),[B],f)
 total=add(x,y,f);a=square(sub(x,y,f),f);b=scale(add(pmul(total,add(xy,[A],f),f),[f.scale(B,2)],f),f.neg(2%f.p),f);c=sub(square(sub(xy,[A],f),f),scale(total,f.scale(B,4),f),f);return a,b,c

def orbit_polynomial(beta,z,A,B,f):
 x=[[v,1]for v in beta];a,b,c=quad_coeffs(x[0],x[1],A,B,f);d,e,h=quad_coeffs(x[2],[z],A,B,f)
 # Res(aV^2+bV+c,dV^2+eV+h),without divisions.
 afcd=sub(pmul(a,h,f),pmul(c,d,f),f);aebd=sub(pmul(a,e,f),pmul(b,d,f),f);bhce=sub(pmul(b,h,f),pmul(c,e,f),f)
 return sub(square(afcd,f),pmul(aebd,bhce,f),f)

def literal_s3(a,b,u,A,B,f):
 if f.p==2:
  ab=f.mul(a,b);au=f.mul(a,u);bu=f.mul(b,u);return f.sq(ab)^f.sq(au)^f.sq(bu)^f.mul(ab,u)^B
 a2=f.sub(a,b);t=f.mul(f.sq(a2),f.sq(u));v=f.add(f.mul(f.add(a,b),f.add(f.mul(a,b),A)),f.scale(B,2));t=f.sub(t,f.scale(f.mul(v,u),2));t=f.add(t,f.sq(f.sub(f.mul(a,b),A)));return f.sub(t,f.scale(f.mul(B,f.add(a,b)),4))
