from research.separator_images_20261116_v91.kernel import Solver
from research.twist_auxiliary_20261116_v86.algebra import Field

class Prefix(Solver):
 def __init__(self,n,mod,d):
  self.f=Field(2,mod);self.n=n;self.d=d;self.mask=(1<<n)-1;self.reduction=sum(c<<i for i,c in enumerate(mod))^(1<<n);self.basis=[1<<i for i in range(d)];self.squares=[self.f.sq(x)for x in self.basis]

def columns(S,y,u0,k,B):
 a,b,c=S.coefficients(y,u0,B);cols=S.columns(a,b,True)
 for i in range(k):
  h=1<<i;v=S.f.mul(y,h);cols.extend(S.columns(S.f.sq(h),v,True));cols.append(S.f.sq(v))
 return cols,c

def certificate(S,y,u0,k,B):
 assert 0<=k<=S.n and u0&((1<<k)-1)==0;cols,c=columns(S,y,u0,k,B);rows=S.prepare(cols)[0]
 if S.particular(c,rows)is not None:return {'empty':False,'rank':len(rows),'dual':None}
 def reduce(v):
  for p in sorted(rows,reverse=True):
   if v>>p&1:v^=rows[p][0]
  return v
 r=reduce(c);bit=(r&-r).bit_length()-1;w=sum(((reduce(1<<j)>>bit)&1)<<j for j in range(S.n));assert all((w&v).bit_count()%2==0 for v in cols)and(w&c).bit_count()%2==1
 return {'empty':True,'rank':len(rows),'dual':w}

def solve(spec,method):
 n,mod,d,y,B=spec['n'],spec['mod'],spec['d'],spec['y'],spec['B'];S=Prefix(n,mod,d);f=S.f;stats={'nodes':0,'leaves':0,'pruned':[],'fiber_queries':0};out=[]
 if method=='reverse':
  F=Solver(n,mod,n)
  for x in range(1<<d):
   a,b,c=F.coefficients(x,y,B);us=F.solve(a,b,c,'normalized');stats['fiber_queries']+=1
   for u in us:out.append((x,u))
 elif method=='scan':
  for u in range(1<<n):
   a,b,c=S.coefficients(y,u,B);xs=S.solve(a,b,c,'stream');stats['fiber_queries']+=1
   out.extend((x,u)for x in xs)
 elif method=='cells':
  def visit(u0,k):
   stats['nodes']+=1
   if stats['nodes']>spec['node_cap']:raise OverflowError('disjointcovernodecap')
   cert=certificate(S,y,u0,k,B)
   if cert['empty']:stats['pruned'].append({'u0':u0,'k':k,**cert});return
   if k:visit(u0,k-1);visit(u0^(1<<(k-1)),k-1)
   else:
    a,b,c=S.coefficients(y,u0,B);xs=S.solve(a,b,c,'stream');stats['fiber_queries']+=1;stats['leaves']+=1;out.extend((x,u0)for x in xs)
  visit(0,n)
 else:raise ValueError(method)
 assert len(out)==len(set(out));return sorted(out),stats
