def points(f,a,b,binary):
 out=[None]
 for x in range(f.q):
  rhs=f.add(f.mul(f.sq(x),x),f.add(f.mul(a,f.sq(x)),b))if binary else f.add(f.mul(f.sq(x),x),f.add(f.mul(a,f.sq(x)),f.mul(b,x)))
  for y in range(f.q):
   if f.add(f.sq(y),f.mul(x,y)if binary else 0)==rhs:out.append((x,y))
 return out

def add(P,Q,f,a,b,binary):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if binary:
  if x==u:
   if f.add(y,v)==x:return None
   slope=f.add(x,f.mul(y,f.inv(x)))
  else:slope=f.mul(f.add(y,v),f.inv(f.add(x,u)))
  w=f.add(f.add(f.sq(slope),slope),f.add(f.add(x,u),a));return(w,f.add(f.mul(slope,f.add(x,w)),f.add(w,y)))
 if x==u:
  if f.add(y,v)==0:return None
  num=f.add(f.mul(3%f.p,f.sq(x)),f.add(f.mul(2%f.p,f.mul(a,x)),b));slope=f.mul(num,f.inv(f.mul(2%f.p,y)))
 else:slope=f.mul(f.sub(v,y),f.inv(f.sub(u,x)))
 w=f.sub(f.sub(f.sub(f.sq(slope),a),x),u);return(w,f.sub(f.mul(slope,f.sub(x,w)),y))

def target(a,b,f,binary):
 if binary:return a,f.power(b,1<<(f.n-1))
 return f.neg(f.mul(2%f.p,a)),f.sub(f.sq(a),f.mul(4%f.p,b))

def phi(P,f,a,b,binary):
 if P is None or P[0]==0:return None
 x,y=P;ix=f.inv(x);ix2=f.sq(ix)
 if binary:
  beta=f.power(b,1<<(f.n-1));u=f.add(x,f.mul(beta,ix));v=f.add(y,f.add(f.mul(beta,f.mul(f.add(y,beta),ix2)),f.add(f.mul(beta,ix),beta)));return(u,v)
 return(f.add(x,f.add(a,f.mul(b,ix))),f.mul(y,f.sub(1,f.mul(b,ix2))))
