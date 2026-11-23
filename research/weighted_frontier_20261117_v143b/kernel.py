import itertools,math

def frontier_count(widths,v):
 assert len(widths)==len(v)and all(x>=0 for x in v)and any(v)
 return math.prod(widths)-math.prod(max(a-b,0)for a,b in zip(widths,v))

def frontier(widths,v):
 for k,w in enumerate(v):
  if not w:continue
  ranges=[range(min(w,widths[k]))if j==k else range(v[j],widths[j])if j<k and v[j]else range(widths[j])for j in range(len(v))]
  yield from itertools.product(*ranges)

def refusal(reason):return {'complete':False,'roots':None,'root_count':None,'reason':reason}

def solve(p,upper,v,slope,correction,tests=(),cap=8192,output_cap=8192):
 assert len(upper)==len(v)>0 and all(-1<=m<p for m in upper)
 if any(m<0 for m in upper):return {'complete':True,'roots':[],'root_count':0,'candidates':0}
 widths=[m+1 for m in upper];weights=[abs(x)for x in v];sign=[-1 if x<0 else 1 for x in v];k=slope%p
 def original(y):return tuple(m-a if s<0 else a for m,a,s in zip(upper,y,sign))
 if not any(weights):
  assert not k,'invalid_zero_direction_slope'
  N=math.prod(widths)
  if N>cap:return refusal('zero_direction_fallback_cap')
  roots=[z for z in itertools.product(*map(range,widths))if correction(z)%p==0 and all(t(z)for t in tests)]
  if len(roots)>output_cap:return refusal('output_cap')
  return {'complete':True,'roots':sorted(roots),'root_count':len(roots),'candidates':N,'zero_direction_fallback':True}
 N=frontier_count(widths,weights)
 if N>cap:return refusal('frontier_cap')
 ik=pow(k,-1,p)if k else None;roots=[];visited=0
 for u in frontier(widths,weights):
  visited+=1;limit=min((m-a)//w for m,a,w in zip(upper,u,weights)if w);c=correction(original(u))%p
  if k:parameters=[(-c*ik)%p]
  elif c:continue
  else:parameters=range(limit+1)
  for b in parameters:
   if b>limit:continue
   theta=original([a+b*w for a,w in zip(u,weights)]);assert correction(theta)%p==0,'uncertified_slope'
   if not all(t(theta)for t in tests):continue
   if len(roots)>=output_cap:return refusal('output_cap')
   roots.append(theta)
 assert visited==N and len(roots)==len(set(roots));return {'complete':True,'roots':sorted(roots),'root_count':len(roots),'candidates':N,'zero_slope':not k}

def signed_normalizations(p,v):
 out=[]
 for j,a in enumerate(v):
  if not a%p:continue
  scale=pow(a,-1,p);w=[x*scale%p for x in v];signed=[x if 2*x<=p else x-p for x in w];out.append({'pivot':j,'scale':scale,'unsigned':w,'signed':signed})
 return out
