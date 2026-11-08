# All arithmetic here is ordinary short-Weierstrass group arithmetic over F_p.
def point(P):return (0,0,1) if P is None else (P[0],P[1],0)
def affine(P):return None if P[2] else (P[0],P[1])
def add(P,Q,p,a):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and (y+v)%p==0:return None
 slope=((3*x*x+a)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p
 X=(slope*slope-x-u)%p;return X,(slope*(x-X)-y)%p
def mul(n,P,p,a):
 R=None
 while n:
  if n&1:R=add(R,P,p,a)
  P=add(P,P,p,a);n//=2
 return R

def residuals(P,Q,R,selectors,s,z,p,a,b):
 x,y,o=P;u,v,q=Q;X,Y,O=R;I,J,C,D,F=selectors
 out=[O*(O-1),O*X,O*Y,(1-O)*(Y*Y-X**3-a*X-b)]
 out += [c*(c-1) for c in selectors]+[sum(selectors)-1]
 out += [(1-D-F)*s,(1-D-F)*z]
 cases=[
  [o-1,X-u,Y-v,O-q],
  [o,q-1,X-x,Y-y,O-o],
  [o,q,x-u,y+v,O-1,X,Y],
  [o,q,x-u,y-v,O,2*y*z-1,s-(3*x*x+a)*z,X-s*s+2*x,Y-s*(x-X)+y],
  [o,q,O,(u-x)*z-1,s-(v-y)*z,X-s*s+x+u,Y-s*(x-X)+y]]
 for mask,eqs in zip(selectors,cases):out += [mask*f for f in eqs]
 assert len(out)==44
 return [x%p for x in out]

def witness(P,Q,p,a,b):
 x,y,o=P;u,v,q=Q;s=z=0
 if o:case=0;R=Q
 elif q:case=1;R=P
 elif x==u and (y+v)%p==0:case=2;R=(0,0,1)
 elif P==Q:
  case=3;z=pow(2*y,-1,p);s=(3*x*x+a)*z%p;X=(s*s-2*x)%p;R=(X,(s*(x-X)-y)%p,0)
 else:
  case=4;z=pow(u-x,-1,p);s=(v-y)*z%p;X=(s*s-x-u)%p;R=(X,(s*(x-X)-y)%p,0)
 selectors=[int(i==case) for i in range(5)]
 assert not any(residuals(P,Q,R,selectors,s,z,p,a,b))
 return R,selectors,s,z

def circuit(n,G,Q,p,a,b,k):
 bits=[(n>>i)&1 for i in range(k)];states=[(0,0,1),(0,0,1)];bases=[G,Q];all_values=list(bits);branches=[0]*5;equations=k
 assert all(e*(e-1)%p==0 for e in bits)
 for i,e in enumerate(bits):
  for j in range(2):
   U=(e*bases[j][0]%p,e*bases[j][1]%p,1-e)
   R,ss,s,z=witness(states[j],U,p,a,b);all_values.extend(R);all_values.extend(ss);all_values.extend([s,z]);branches[ss.index(1)]+=1;equations+=44
   states[j]=R;bases[j]=add(bases[j],bases[j],p,a)
 return states,all_values,branches,equations
