import itertools,math

class ProjectionBudget(Exception):pass

def floor_sum(n,m,a,b):
 assert n>=0 and m>0
 qa,a=divmod(a,m);qb,b=divmod(b,m);ans=qa*n*(n-1)//2+qb*n;steps=0;limit=4*m.bit_length()+8
 while True:
  steps+=1;assert steps<=limit,'euclideanstepcap'
  if a>=m:ans+=(n-1)*n*(a//m)//2;a%=m
  if b>=m:ans+=n*(b//m);b%=m
  y=a*n+b
  if y<m:return ans,steps
  n,b=divmod(y,m);m,a=a,m

def cyclic_segments(lo,hi,p):
 if hi<lo:return []
 length=hi-lo+1
 if length>=p:return [(0,p-1)]
 start=lo%p;end=start+length-1
 return [(start,end)]if end<p else[(0,end-p),(start,p-1)]

class PrimeProjection:
 def __init__(self,p,jbox,pbox,ratio):
  assert p>=2;self.p=p;self.jbox=tuple(jbox);self.pbox=tuple(pbox);self.r=ratio%p;self.calls=0;self.max_steps=0
  L,U=self.jbox;A,B=self.pbox;self.empty=L>U or A>B
  if not self.empty:assert 0<=L<=U<p and 0<=A<=B<p
  self.segments=[]if self.empty else None
  if not self.empty:
   m=U-L+1;n=B-A+1;s=self.r if 2*self.r<=p else self.r-p
   if s==0:self.segments=[(L,U)]
   elif m>=abs(s):self.segments=cyclic_segments(L-s*B if s>0 else L-s*A,U-s*A if s>0 else U-s*B,p)
  self.cardinality=None if self.segments is None else sum(b-a+1 for a,b in self.segments)
  self.cardinality_upper_bound=0 if self.empty else min(p,(U-L+1)*(B-A+1))
 def witness_count(self,x):
  if self.empty:return 0
  p=self.p;L,U=self.jbox;A,B=self.pbox;n=B-A+1;b=(x%p)+self.r*A;first,s1=floor_sum(n,p,self.r,b+p-L);second,s2=floor_sum(n,p,self.r,b+p-1-U);count=first-second;assert 0<=count<=n;self.calls+=1;self.max_steps=max(self.max_steps,s1,s2);return count
 def contains(self,x):return self.witness_count(x)>0
 def members(self,cap=4096,scan_cap=64):
  if self.cardinality is not None:
   if self.cardinality>cap:raise ProjectionBudget('projectedcardinalitycap')
   return [x for a,b in self.segments for x in range(a,b+1)]
  if self.p>scan_cap:raise ProjectionBudget('membershiponlyprojection_noadmittedenumerator')
  values=[x for x in range(self.p)if self.contains(x)]
  if len(values)>cap:raise ProjectionBudget('projectedcardinalitycap')
  return values
 def describe(self):return {'mode':'segments'if self.segments is not None else'membership_only','segments':self.segments,'cardinality':self.cardinality,'cardinality_upper_bound':self.cardinality_upper_bound,'query_calls':self.calls,'max_floor_steps':self.max_steps}

class SubspaceProjection:
 def __init__(self,f,joffset,jbasis,poffset,pbasis,ratio):
  self.f=f;self.offset=f.sub(joffset,f.mul(ratio,poffset));self.rows={};n=f.n;p=f.p
  for value in list(jbasis)+[f.neg(f.mul(ratio,b))for b in pbasis]:
   assert type(value)is int and 0<=value<f.q
   v=[value//p**i%p for i in range(n)]
   for i in range(n-1,-1,-1):
    if not v[i]:continue
    if i in self.rows:
     t=v[i];v=[(a-t*b)%p for a,b in zip(v,self.rows[i])]
    else:
     iv=pow(v[i],-1,p);self.rows[i]=[a*iv%p for a in v];break
  self.rank=len(self.rows);self.cardinality=p**self.rank;self.basis=[sum(a*p**i for i,a in enumerate(self.rows[k]))for k in sorted(self.rows)]
 def contains(self,x):
  f=self.f;assert 0<=x<f.q;value=f.sub(x,self.offset);v=[value//f.p**i%f.p for i in range(f.n)]
  for i in sorted(self.rows,reverse=True):
   t=v[i]
   if t:v=[(a-t*b)%f.p for a,b in zip(v,self.rows[i])]
  return not any(v)
 def members(self,cap=4096):
  if self.cardinality>cap:raise ProjectionBudget('subspacecardinalitycap')
  if not self.basis:return [self.offset]
  f=self.f;out=[]
  for coefficients in itertools.product(range(f.p),repeat=self.rank):
   value=self.offset
   for c,b in zip(coefficients,self.basis):value=f.add(value,f.mul(c,b))
   out.append(value)
  assert len(set(out))==self.cardinality;return sorted(out)

def solve_prime_boxes(p,boxes,v,slope,pivot,correction,constraints=(),cap=4096):
 assert slope%p and v[pivot]%p
 if any(a>b for a,b in boxes):return {'complete':True,'roots':[],'root_count':0,'projected_candidates':0}
 iv=pow(v[pivot],-1,p);ik=pow(slope,-1,p);indices=[j for j in range(len(boxes))if j!=pivot];projected=[PrimeProjection(p,boxes[j],boxes[pivot],v[j]*iv)for j in indices]
 if all(s.cardinality is not None for s in projected)and math.prod(s.cardinality for s in projected)>cap:return {'complete':False,'roots':None,'root_count':None,'reason':'projectedproductcap'}
 try:members=[s.members(cap)for s in projected]
 except ProjectionBudget as e:return {'complete':False,'roots':None,'root_count':None,'reason':str(e)}
 count=math.prod(map(len,members))
 if count>cap:return {'complete':False,'roots':None,'root_count':None,'reason':'projectedproductcap'}
 roots=[]
 for free in itertools.product(*members):
  x=[0]*len(boxes)
  for j,a in zip(indices,free):x[j]=a
  c=correction(tuple(x))%p;theta=tuple((a-b*c*ik)%p for a,b in zip(x,v))
  if not all(a<=x<=b for x,(a,b)in zip(theta,boxes)):continue
  assert correction(theta)%p==0
  if all(test(theta)for test in constraints):roots.append(theta)
 assert len(roots)==len(set(roots));return {'complete':True,'roots':sorted(roots),'root_count':len(roots),'projected_candidates':count}
