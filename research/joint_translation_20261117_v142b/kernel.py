import itertools

def anchors(n,M):
 for k in range(n):
  ranges=[range(1,M+1)if j<k else range(M+1)for j in range(n)if j!=k]
  for free in itertools.product(*ranges):
   u=list(free);u.insert(k,0);yield tuple(u)

def anchor_count(n,M):
 assert n>=1 and M>=0
 return (M+1)**n-M**n

def refused(reason):return {'complete':False,'roots':None,'root_count':None,'reason':reason}

def solve_interval(p,n,M,slope,correction,constraints=(),candidate_cap=8192,output_cap=8192):
 assert n>=1 and 0<=M<p
 if p<=2*M:return refused('nonwrapping_chart_not_admitted')
 N=anchor_count(n,M)
 if N>candidate_cap:return refused('joint_chart_cap')
 k=slope%p;ik=pow(k,-1,p)if k else None;roots=[];visited=0;tested=0
 for u in anchors(n,M):
  visited+=1;c=correction(u)%p
  if k:bs=[(-c*ik)%p]
  elif c:continue
  else:bs=range(M-max(u)+1)
  for b in bs:
   if b>M-max(u):continue
   theta=tuple(x+b for x in u);tested+=1
   assert correction(theta)%p==0,'uncertified_line_slope'
   if not all(t(theta)for t in constraints):continue
   if len(roots)>=output_cap:return refused('output_cap')
   roots.append(theta)
 assert visited==N and len(roots)==len(set(roots));return {'complete':True,'roots':sorted(roots),'root_count':len(roots),'joint_candidates':N,'visited':visited,'original_checks':tested,'slope_zero':not k}

def solve_subspace(f,n,domain,slope,correction,constraints=(),candidate_cap=8192,output_cap=8192):
 D=tuple(sorted(set(domain)));S=set(D);assert n>=1 and D and 0 in S
 # Admission verifies the supplied finite set really is an additive subgroup.
 if len(D)**2>candidate_cap:return refused('subspace_validation_cap')
 if any(f.add(a,b)not in S or f.sub(a,b)not in S for a in D for b in D):return refused('domain_not_additive_subgroup')
 N=len(D)**(n-1)
 if N>candidate_cap:return refused('joint_chart_cap')
 k=slope;ik=f.inv(k)if k else None;roots=[];visited=0
 for free in itertools.product(D,repeat=n-1):
  x=(0,)+free;visited+=1;c=correction(x)
  if k:bs=[f.neg(f.mul(c,ik))]
  elif c:continue
  else:bs=D
  for b in bs:
   if b not in S:continue
   theta=tuple(f.add(a,b)for a in x);assert correction(theta)==0,'uncertified_line_slope'
   if not all(t(theta)for t in constraints):continue
   if len(roots)>=output_cap:return refused('output_cap')
   roots.append(theta)
 assert visited==N and len(roots)==len(set(roots));return {'complete':True,'roots':sorted(roots),'root_count':len(roots),'joint_candidates':N,'visited':visited,'slope_zero':not k}
