import time
START=time.monotonic()
import itertools,json,random,math
from pathlib import Path
from .common import P,finish
from .kernel import anchors,anchor_count,solve_interval,solve_subspace
from research.implicit_projection_20261117_v141b.kernel import solve_prime_boxes
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
out={'small_root_checks':0,'anchor_orbit_checks':0,'contexts':[],'cohort':[],'refusals':0,'wrap_counterexample':None,'all_registered_contexts_complete':False,'original_complete':False};error=None

def brute(D,n,c,tests=()):return sorted(z for z in itertools.product(D,repeat=n)if c(z)==0 and all(t(z)for t in tests))
def family(p,n,k,weights,plant):
 def raw(z):
  d=[(z[j]-z[0])%p for j in range(1,n)];return (k*z[0]+sum(a*x+b*x*x for x,(a,b)in zip(d,weights[:n-1]))+sum(weights[n-1+j][0]*d[j]*d[j+1]for j in range(n-2)))%p
 shift=raw(plant)
 return lambda z:(raw(z)-shift)%p

def timed(call,c,tests):
 start=time.perf_counter();r=call();assert r['complete'];assert len(r['roots'])==len(set(r['roots']))
 for z in r['roots']:assert c(z)==0 and all(t(z)for t in tests)
 payload=json.dumps(r['roots'],separators=(',',':'));return time.perf_counter()-start,r['roots'],len(payload)
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);cap=data['candidate_cap']
 for p in data['small_primes']:
  for M in range((p-1)//2+1):
   for n in [1,2,3]:
    A=list(anchors(n,M));expected={tuple(z[j]-min(z)for j in range(n))for z in itertools.product(range(M+1),repeat=n)};assert set(A)==expected and len(A)==len(set(A))==anchor_count(n,M);out['anchor_orbit_checks']+=1
    for k in range(p):
     c=lambda z,p=p,k=k:(k*z[0]+sum((z[j]-z[0])**2 for j in range(1,len(z))))%p
     for tests in [(),(lambda z:sum(x*x for x in z)%p==0,)]:
      got=solve_interval(p,n,M,k,c,tests);assert got['complete']and got['roots']==brute(range(M+1),n,c,tests);out['small_root_checks']+=1
 p=5;M=3;c=lambda z:(sum(z)-3)%5;bad=[]
 for u in anchors(2,M):
  b=(-c(u)*pow(2,-1,p))%p
  if b<=M-max(u):bad.append(tuple(x+b for x in u))
 assert len(bad)>len(set(bad))and bad.count((0,3))==2;out['wrap_counterexample']={'p':5,'M':3,'duplicate_root':[0,3],'multiplicity_from_unsafe_charts':2}
 assert not solve_interval(5,2,3,2,c)['complete'];out['refusals']+=1
 assert not solve_interval(31,5,5,1,lambda z:z[0],candidate_cap=1)['complete'];out['refusals']+=1
 assert not solve_interval(7,3,2,0,lambda z:0,output_cap=0)['complete'];out['refusals']+=1
 specs=json.loads(Path(data['field_data']).read_bytes())['fields']
 for spec in specs:
  f=Arithmetic(spec['p'],spec['mod']);name=spec.get('name',str(f.p)+'^'+str(f.n));checks=0
  if f.p>100:
   D=[0,1];c=lambda z,p=f.p:(z[0]+(z[1]-z[0])**2-(z[2]-z[0]))%p;got=solve_interval(f.p,3,1,1,c);assert got['complete']and got['roots']==brute(D,3,c);checks+=1
   bits={'P-192':64,'P-224':74,'P-256':85,'P-384':128,'P-521':173}[name];M=(1<<bits)-1;N=anchor_count(3,M);large=solve_interval(f.p,3,M,1,c);assert not large['complete']and large['roots']is None;out['refusals']+=1
   out['contexts'].append({'name':name,'complete_small_cases':checks,'full_width_anchor_count':N,'large_enumeration':False});continue
  D=list(range(f.p))
  if f.p==2 and f.n>1:D=[0,1,2,3]
  for k in [0,1]:
   c=lambda z,k=k,f=f:f.add(f.mul(k,z[0]),f.add(f.sq(f.sub(z[1],z[0])),f.sub(z[2],z[0])))
   tests=(lambda z,f=f:f.add(f.mul(z[0],z[1]),z[2])==0,)
   got=solve_subspace(f,3,D,k,c,tests);assert got['complete']and got['roots']==brute(D,3,c,tests);checks+=1
  out['contexts'].append({'name':name,'complete_small_cases':checks,'joint_candidates':len(D)**2})
 f=Arithmetic(5,[0,1]);assert not solve_subspace(f,2,[0,1],1,lambda z:z[0])['complete'];out['refusals']+=1
 for spec in data['cohort']:
  p,n,M,k=[spec[key]for key in ['p','n','M','slope']];weights=[(rng.randrange(p),rng.randrange(p))for _ in range(2*n)];plant=tuple(rng.randrange(M+1)for _ in range(n));c=family(p,n,k,weights,plant);target=sum((j+1)*x*x for j,x in enumerate(plant))%p;tests=(lambda z,p=p,target=target:sum((j+1)*x*x for j,x in enumerate(z))%p==target,)
  expected=brute(range(M+1),n,c,tests);candidate=lambda:solve_interval(p,n,M,k,c,tests,candidate_cap=cap,output_cap=data['output_cap']);baseline=lambda:solve_prime_boxes(p,[(0,M)]*n,[1]*n,k,0,c,tests,cap=cap)
  a=candidate();b=baseline();assert a['complete']and b['complete']and a['roots']==b['roots']==expected
  timings={'joint':[],'projected':[]};payload_bytes=None
  for rep in range(data['cold_repetitions']):
   for label,call in ([('joint',candidate),('projected',baseline)]if rep%2==0 else[('projected',baseline),('joint',candidate)]):
    elapsed,roots,size=timed(call,c,tests);assert roots==expected;timings[label].append(elapsed);payload_bytes=size
  joint=sorted(timings['joint'])[1];old=sorted(timings['projected'])[1];out['cohort'].append({'id':spec['id'],'n':n,'M':M,'root_count':len(expected),'joint_candidates':a['joint_candidates'],'projected_candidates':b['projected_candidates'],'paired_component_seconds':timings,'projected_over_joint':old/joint,'serialized_root_bytes':payload_bytes})
 ratios=[r['projected_over_joint']for r in out['cohort']];out['development_geomean']=math.exp(sum(math.log(x)for x in ratios)/len(ratios));out['development_worst_slowdown']=max(1/x for x in ratios);out['development_target_met']=out['development_geomean']>=data['development_geomean_target']and out['development_worst_slowdown']<=data['development_worst_slowdown_limit'];out['all_registered_contexts_complete']=len(out['contexts'])==17;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False;out['development_target_met']=False
finish(out,error,START,'Exact joint translation charts with complete independent bounded root sets and retained paired component costs','If cost target fails,derive exact rank/select or specialized direct prime arithmetic;otherwise integrate only under a certified global line slope and supported original-domain map. Keep hard all-roots and matched baseline gate separate.')
