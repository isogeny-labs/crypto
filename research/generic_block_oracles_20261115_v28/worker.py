"""Independent all-fiber oracles for the prepared signed generic block backend."""
from pathlib import Path
import itertools,json,hashlib,resource,sys,time
from research.generic_block_20261115_v27 import backend as g
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CURRENT={};STATES=0
def oracle(F,a,b):
 p=F.p;A=[a//(p**i)%p for i in range(F.degree)];B=[b//(p**i)%p for i in range(F.degree)];C=[0]*(2*F.degree-1)
 for i,x in enumerate(A):
  for j,y in enumerate(B):C[i+j]+=x*y
 if F.degree==2:C[0]+=F.nu*C[2]
 return sum((C[i]%p)*(p**i)for i in range(F.degree))
def add(F,a,b):
 p=F.p
 return sum((((a//(p**i))%p+(b//(p**i))%p)%p)*(p**i)for i in range(F.degree))
def odot(F,row,x):
 v=0
 for a,b in zip(row,x):v=add(F,v,oracle(F,a,b))
 return v
def models(F,result,n):
 if result is None:return set()
 out=set()
 for coefficients in itertools.product(range(F.order),repeat=len(result['basis'])):
  v=list(result['particular'])
  for a,B in zip(coefficients,result['basis']):
   for i,b in enumerate(B):v[i]=add(F,v[i],oracle(F,a,b))
  out.add(tuple(v))
 return out
def frames(n,k):return [[int(i==j)for i in range(n)]for j in range(n-k,n)],[[int(i==j)for i in range(n)]for j in range(n-k)]
def fingerprint(points):return hashlib.sha256(json.dumps(sorted(points),separators=(',',':')).encode()).hexdigest()
def matrices(F,n,index):
 q=F.order
 if index==0:return [[0]*n for _ in range(n)],[0]*n
 if index==1 and n==3:return [[1,1,1],[0,1,2],[1,2,3%q]],[1,2,3%q]
 if index==1:return [[0]*n for _ in range(n)],[1]+[0]*(n-1)
 if index==2:return [[int(i==j)for j in range(n)]for i in range(n)],[(i+1)%q for i in range(n)]
 if index in[3,4]:return [[1]+[0]*(n-1)for _ in range(n)],[0 if index==4 or i==0 else 1 for i in range(n)]
 state=0x28a11+index*97+n*65537+q
 def word():
  nonlocal state
  state=(state*6364136223846793005+1442695040888963407)&((1<<64)-1)
  return state%q
 return [[word()for _ in range(n)]for _ in range(n)],[word()for _ in range(n)]
def check_case(F,label,n,index):
 global CURRENT,STATES
 M,rhs=matrices(F,n,index);K,U=frames(n,index%(n+1));CURRENT=dict(field=label,n=n,index=index,matrix=M,rhs=rhs,K=K,U=U)
 result=g.block(F,M,rhs,K,U);truth=set()
 for x in itertools.product(range(F.order),repeat=n):
  STATES+=1;assert STATES<=100000
  if all(odot(F,row,x)==c for row,c in zip(M,rhs)):truth.add(x)
 found=models(F,result,n);assert found==truth
 return dict(n=n,index=index,matrix=M,rhs=rhs,K=K,U=U,result=result,exact_count=len(truth),complete_set_sha256=fingerprint(truth))
def small(out):
 global CURRENT,STATES
 for p,degree,label,three in[(3,1,'F3',16),(5,1,'F5',16),(3,2,'GF9',16),(5,2,'GF25',2)]:
  F=g.Field(p,degree,2);scalar=0
  for a in range(F.order):
   for b in range(F.order):
    CURRENT=dict(field=label,phase="scalar_arithmetic",a=a,b=b)
    assert F.mul(a,b)==oracle(F,a,b) and F.add(a,b)==add(F,a,b)
    scalar+=1
   if a:
    expected=next(b for b in range(1,F.order)if oracle(F,a,b)==1)
    assert F.inv(a)==expected
   assert add(F,a,F.neg(a))==0
  records=[check_case(F,label,2,i)for i in range(64)]+[check_case(F,label,3,i)for i in range(three)]
  # Empty-row and zero-variable cases are separate semantic checks.
  for M,rhs,K,U,n in[([],[],[],[[1,0],[0,1]],2),([[]],[0],[],[],0),([[]],[1],[],[],0)]:
   CURRENT=dict(field=label,matrix=M,rhs=rhs,K=K,U=U,n=n,semantic=True);result=g.block(F,M,rhs,K,U);truth=set()
   for x in itertools.product(range(F.order),repeat=n):
    STATES+=1;assert STATES<=100000
    if all(odot(F,row,x)==c for row,c in zip(M,rhs)):truth.add(x)
   assert models(F,result,n)==truth
  out.append(dict(field=label,p=p,degree=degree,nu=2,scalar_pairs=scalar,cases=records,semantic_cases=3,all_complete_sets_equal=True))
def wide(out):
 presets=json.loads((R/'lab/research/nist_presets.json').read_text())['curves']
 for record in presets:
  p=int(record['p_hex'],16);F=g.Field(p);cases=[]
  for index in range(16):
   b=(p-1-index*97)%p;c=((1<<(p.bit_length()-1))+index*65537)%p;f=(p-4-index*31)%p;alpha=(p-5-index)%p;beta=((1<<(p.bit_length()-2))+index)%p;bad=index%2==1
   M=[[1,b,c],[0,1,f],[1,(b+1)%p,(c+f)%p]];rhs=[alpha,beta,(alpha+beta+int(bad))%p];K,U=frames(3,1)
   global CURRENT
   CURRENT=dict(field=record['name'],matrix=M,rhs=rhs,K=K,U=U,index=index);result=g.block(F,M,rhs,K,U)
   if bad:assert result is None
   else:
    assert result is not None and len(result['basis'])==1
    part=result['particular'];v=result['basis'][0];assert v[2]
    inv=pow(v[2],-1,p);assert tuple(x*inv%p for x in v)==((b*f-c)%p,(-f)%p,1)
    t=part[2];assert part==[(alpha-b*((beta-f*t)%p)-c*t)%p,(beta-f*t)%p,t]
    assert all(sum(a*x for a,x in zip(row,part))%p==r for row,r in zip(M,rhs))
   for x in[1,2,3,p-1,p-2,(1<<(p.bit_length()-1))+index]:
    a=x%p
    if a:assert a*F.inv(a)%p==1
   cases.append(dict(index=index,matrix=M,rhs=rhs,expected_consistent=not bad,result=result))
  out.append(dict(field=record['name'],prime=p,prime_bits=p.bit_length(),cases=cases,all_exact_planted_line_or_contradiction_checks=True,scope='backend only; no hard nonlinear roots or new primality proof'))
def main():
 out=dict(small=[],wide=[]);error=None
 try:small(out['small']);wide(out['wide'])
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['failed_case']=CURRENT
 out['literal_assignments']=STATES;raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1048576
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,small=[dict(field=v['field'],cases=len(v['cases']),semantic_cases=v['semantic_cases'],scalar_pairs=v['scalar_pairs'])for v in out['small']],wide=[dict(field=v['field'],cases=len(v['cases']),bits=v['prime_bits'])for v in out['wide']],literal_assignments=STATES,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='First independent complete small-field oracles and separate NIST backend checks for signed block lifting.',evidence=ev,next_actions=['If passed, use this exact field interface in a separately proved nonlinear/rootfinding route; oddextensions beyondquadratic and matched hard originalroot benchmarks remainopen. Preserve failurecounterexamples and freeze freshrepair ifneeded.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
