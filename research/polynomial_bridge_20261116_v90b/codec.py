from research.twist_auxiliary_20261116_v86.algebra import Field

def field(c):return Field(c['p'],c['mod'])
def encode(c):
 f=field(c);m=c['arity'];n=2*m-2;D=sorted(set(c['D']));assert len(D)==len(c['D'])and all(0<=x<f.q for x in D);eq=[];kinds=[]
 def polynomial(terms,kind):
  result={}
  for scalar,indices in terms:
   powers=[0]*n;co=scalar
   for i,e in indices:
    if i is None:co=f.mul(co,f.power(c['z'],e))
    else:powers[i]+=e
   t=tuple(powers);result[t]=f.add(result.get(t,0),co)
  eq.append([{'coefficient':v,'powers':list(k)}for k,v in sorted(result.items())if v]);kinds.append(kind)
 def S(i,j,k):
  pairs=[(i,j),(i,k),(j,k)];ts=[(1,[(u,2),(v,2)])for u,v in pairs]
  if f.p==2:ts.extend([(1,[(i,1),(j,1),(k,1)]),(c['B'],[])])
  else:
   ts.extend(((-2)%f.p,[(i,1),(j,1),(k,1),(u,1)])for u in[i,j,k]);ts.extend((f.mul((-2)%f.p,c['A']),[(u,1),(v,1)])for u,v in pairs);ts.extend((f.mul((-4)%f.p,c['B']),[(u,1)])for u in[i,j,k]);ts.append((f.sq(c['A']),[]))
  polynomial(ts,'S3')
 S(0,1,m)
 for j in range(2,m-1):S(m+j-2,j,m+j-1)
 S(2*m-3,m-1,None)
 # Ascending coefficients of product(X-d),including the empty-domain constant1.
 dp=[1]
 for d in D:
  new=[0]*(len(dp)+1)
  for i,a in enumerate(dp):new[i]=f.add(new[i],f.mul(f.neg(d),a));new[i+1]=f.add(new[i+1],a)
  dp=new
 for j in range(m):polynomial([(a,[(j,i)])for i,a in enumerate(dp)],'source_domain')
 for j in range(n):polynomial([(1,[(j,f.q)]),(f.neg(1),[(j,1)])],'field')
 return {'schema':'finite_field_sparse_all_roots_v1','instance_id':c['id'],'field':{'kind':'prime'if f.n==1 else'extension','characteristic':f.p,'degree':f.n,'modulus':c['mod'],'basis':'ascending polynomial;integer base-p encoding'},'variables':[f'x{i+1}'for i in range(m)]+[f'u{i}'for i in range(2,m)],'equations':eq,'equation_kinds':kinds,'source_domain':D,'arity':m,'A':c['A'],'B':c['B'],'target_x':c['z']}

def evaluate(poly,row,f):
 out=0
 for t in poly:
  v=t['coefficient']
  for x,e in zip(row,t['powers']):
   if e:v=f.mul(v,f.power(x,e))
  out=f.add(out,v)
 return out
