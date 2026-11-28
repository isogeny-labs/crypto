import itertools,math

def refused(reason):return {'status':'budget_refused','complete':False,'roots':None,'root_count':None,'reason':reason}

def solve_domains(f,domains,v,slope,pivot,correction,constraints=(),max_domain_elements=1024,max_pair_work=100000,max_projected_candidates=4096):
 n=len(domains);assert len(v)==n and 0<=pivot<n and v[pivot]and slope,'nonzeroslopecertificateandpivotrequired';DD=[]
 for domain in domains:
  try:size=len(domain)
  except(OverflowError,TypeError):return refused('input_domain_length_unavailable_or_overflow')
  if size>max_domain_elements:return refused('input_domain_cap')
  vals=tuple(sorted(set(domain)));assert all(type(a)is int and 0<=a<f.q for a in vals);DD.append(vals)
 if any(not x for x in DD):return {'status':'complete','complete':True,'roots':[],'root_count':0,'input_candidates':0,'projected_candidates':0,'visited':0,'pair_work':0}
 pair_work=sum(len(DD[j])*len(DD[pivot])for j in range(n)if j!=pivot)
 if pair_work>max_pair_work:return refused('projected_pair_work_cap')
 iv=f.inv(v[pivot]);ik=f.inv(slope);indices=[j for j in range(n)if j!=pivot];projected=[]
 for j in indices:
  ratio=f.mul(v[j],iv);projected.append(tuple(sorted({f.sub(a,f.mul(ratio,b))for a in DD[j]for b in DD[pivot]})))
 count=math.prod(map(len,projected))
 if count>max_projected_candidates:return refused('projected_product_cap')
 DDsets=[set(x)for x in DD];roots=[];seen=set();visited=0
 for free in itertools.product(*projected):
  x=[0]*n
  for j,a in zip(indices,free):x[j]=a
  c=correction(tuple(x));theta=tuple(f.sub(a,f.mul(b,f.mul(c,ik)))for a,b in zip(x,v));visited+=1
  if any(a not in domain for a,domain in zip(theta,DDsets)):continue
  assert correction(theta)==0,'invalidlinecertificate';assert [f.sub(a,f.mul(b,f.mul(theta[pivot],iv)))for a,b in zip(theta,v)]==x,'gaugeinverse'
  if not all(test(theta)for test in constraints):continue
  assert theta not in seen,'duplicatecanonicalroot';seen.add(theta);roots.append(theta)
 roots.sort();return {'status':'complete','complete':True,'roots':roots,'root_count':len(roots),'input_candidates':math.prod(map(len,DD)),'projected_candidates':count,'projected_sizes':list(map(len,projected)),'visited':visited,'pair_work':pair_work}
