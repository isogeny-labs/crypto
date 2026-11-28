from research.separator_images_20261116_v91.kernel import Solver
from research.symmetric_pair_20261116_v93b.kernel import solve as pair

def solve(spec,method):
 n,mod,d,a,b,B=[spec[k]for k in['n','mod','d','a','b','B']];assert 1<=d<=n and 0<B<1<<n
 F=Solver(n,mod,n);S=Solver(n,mod,d);f=F.f;stats={'full_quadratics':0,'pair_calls':0,'source_iterations':0};branches={'zero':set(),'finite':set(),'infinity':set()};complete=True
 def quadratic(y,u):
  stats['full_quadratics']+=1;return F.solve(*F.coefficients(y,u,B),'stream')
 def restore(x,y,branch):
  for z in quadratic(a,x):
   aa,bb,cc=F.coefficients(y,b,B)
   if f.mul(aa,f.sq(z))^f.mul(bb,z)^cc==0:branches[branch].add((x,y,z))
 if method=='source':
  if d>spec['source_cap_bits']:return{'complete':False,'roots':[],'exact_root_count':None,'stats':stats,'reason':'declared source scan cap'}
  for x in range(1<<d):
   stats['source_iterations']+=1
   for z in quadratic(a,x):
    for y in S.solve(*S.coefficients(z,b,B),'stream'):branches['finite'].add((x,y,z))
 elif method=='rebracket':
  # Every bridge through the unique geometric nonzero two-torsion point.
  if a and b:
   rootB=f.power(B,1<<(n-1));x=f.mul(rootB,f.inv(a));y=f.mul(rootB,f.inv(b))
   if x<1<<d and y<1<<d:branches['zero'].add((x,y,0))
  for w in quadratic(a,b):
   stats['pair_calls']+=1;r=pair({'n':n,'mod':mod,'d':d,'u':w,'B':B,'fiber_cap_bits':spec['pair_cap_bits']})
   if not r['complete']:complete=False
   for x,y in r['roots']:restore(x,y,'finite')
  # Endpoint sum O is a projective branch absent from finite S3(a,b,w).
  if a==b:
   if d>spec['diagonal_cap_bits']:complete=False
   else:
    for x in range(1<<d):stats['source_iterations']+=1;restore(x,x,'infinity')
 else:raise ValueError('method')
 out=set().union(*branches.values());stats.update({'branch_counts':{k:len(v)for k,v in branches.items()},'zero_unique':len(branches['zero']-branches['finite']-branches['infinity']),'infinity_unique':len(branches['infinity']-branches['finite']-branches['zero'])})
 return{'complete':complete,'roots':sorted(out),'exact_root_count':len(out)if complete else None,'stats':stats}
