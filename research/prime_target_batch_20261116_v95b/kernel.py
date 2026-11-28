from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic,nonsquare,square_root,coefficients,batch_inverse

def solve(spec,method):
 p,A,B,M=[spec[k]for k in['p','A','B','M']];targets=spec['targets'];assert 0<=M<=1023 and len(targets)<=64 and p%2==1
 f=Arithmetic(p,[0,1]);assert(4*A**3+27*B**2)%p!=0;delta=nonsquare(f);di=pow(delta,-1,p);cache={};pending=[];roots=[set()for _ in targets];stats={'sqrt_calls':0,'class_calls':0,'denominator_inverse_calls':0,'pair_count':len(targets)*(M+1)}
 def normalized(x):
  if x in cache:return cache[x]
  fx=(x*x*x+A*x+B)%p
  if not fx:cache[x]=(0,0);return cache[x]
  cls=1 if pow(fx,(p-1)//2,p)==1 else -1;stats['class_calls']+=1;stats['sqrt_calls']+=1;r=square_root(fx if cls==1 else fx*di%p,f,delta);assert r is not None;cache[x]=(cls,r);return cache[x]
 if method=='cached':
  for x in range(M+1):normalized(x)
  for u in targets:normalized(u)
 elif method!='direct':raise ValueError('method')
 for i,u in enumerate(targets):
  assert 0<=u<p
  for x in range(M+1):
   a,b,c=coefficients(x,u,A,B,f)
   if not a:
    if b:pending.append((i,x,-c%p,None,b))
    else:assert c!=0
    continue
   if method=='direct':
    stats['sqrt_calls']+=1;r=square_root((b*b-4*a*c)%p,f,delta)
    if r is None:continue
   else:
    cx,rx=cache[x];cu,ru=cache[u]
    if cx*cu==-1:continue
    r=4*rx*ru%p
    if cx==cu==-1:r=r*delta%p
   pending.append((i,x,-b%p,r,2*a%p))
 invs=batch_inverse([v[4]for v in pending],f)if pending else[];stats['denominator_inverse_calls']=int(bool(pending))
 for(i,x,num,r,den),iv in zip(pending,invs):
  ys={num*iv%p}if r is None else{(num+r)*iv%p,(num-r)*iv%p}
  for y in ys:
   if y<=M:roots[i].add((x,y))
 return{'complete':True,'roots':[sorted(r)for r in roots],'exact_root_counts':[len(r)for r in roots],'stats':stats}
