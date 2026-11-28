import time
START=time.monotonic()
from pathlib import Path
import json,random,itertools
from .common import P,finish
from .kernel import solve_domains
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.field_tangent_20261117_v139c.kernel import family,tangent_row,certificate,dot,evaluate,correction
from research.odd_coprime_factor_20261116_v119c.kernel import inverse_block
out={'wrapper_calls':0,'exhaustive_subdomain_cases':0,'counterexample':None,'expansion_case':None,'cap_refusals':0,'zero_slope_rejections':0,'contexts':[],'all_registered_contexts_complete':False,'original_complete':False};error=None

def brute(f,domains,c,constraints=()):return sorted(theta for theta in itertools.product(*[sorted(set(x))for x in domains])if c(theta)==0 and all(t(theta)for t in constraints))
def call(f,domains,v,k,p,c,constraints=(),**caps):
 assert out['wrapper_calls']<2000 and time.monotonic()-START<72,'boundedwrapper';out['wrapper_calls']+=1;return solve_domains(f,domains,v,k,p,c,constraints,**caps)
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);f=Arithmetic(5,[0,1]);c=lambda z:(sum(z)-1)%5;r=call(f,[(0,1)]*2,[1,1],2,0,c);assert r['roots']==[(0,1),(1,0)];naive=[]
 for t in [0,1]:
  x=[0,t];a=c(x);theta=tuple((z-a*3)%5 for z in x)
  if all(z in[0,1]for z in theta):naive.append(theta)
 assert naive==[(0,1)];out['counterexample']={'correct_roots':r['roots'],'naive_roots':naive,'projected_candidates':r['projected_candidates']}
 r=call(f,[(0,1)]*3,[1,1,1],3,0,c);assert r['roots']==brute(f,[(0,1)]*3,c)and r['projected_candidates']>r['input_candidates'];out['expansion_case']={k:v for k,v in r.items()if k!='roots'}
 for p,mod in [(2,[0,1]),(3,[0,1]),(2,[1,1,1])]:
  f=Arithmetic(p,mod);subsets=[tuple(x for x in range(f.q)if mask>>x&1)for mask in range(1<<f.q)]
  for mode in range(3):
   pivot=mode%2;v=[1,f.q-1]if mode else[1,1];k=1 if mode<2 else f.q-1;iv=f.inv(v[pivot]);j=1-pivot
   def c(theta):
    w=f.sub(theta[j],f.mul(f.mul(v[j],iv),theta[pivot]));return f.add(f.mul(f.mul(k,iv),theta[pivot]),f.add(f.sq(w),mode%f.p))
   for a in subsets:
    for b in subsets:
     domains=[a,b];r=call(f,domains,v,k,pivot,c);assert r['complete']and r['roots']==brute(f,domains,c);out['exhaustive_subdomain_cases']+=1
  r=call(f,[(0,1,1),(0,0,1)],[1,1],1,0,lambda z:f.add(z[0],f.sq(f.sub(z[1],z[0]))));assert r['roots']==brute(f,[(0,1),(0,1)],lambda z:f.add(z[0],f.sq(f.sub(z[1],z[0]))))
 f=Arithmetic(257,[0,1]);c=lambda z:sum(z)%257
 for caps,domains in [({'max_projected_candidates':10},[range(20)]*3),({'max_pair_work':10},[range(20)]*3),({'max_domain_elements':10},[range(20)]*3),({},[range(1<<300)]*3)]:
  r=call(f,domains,[1,1,1],3,0,c,**caps);assert not r['complete']and r['roots']is None and r['root_count']is None;out['cap_refusals']+=1
 try:solve_domains(f,[(0,1)]*2,[1,1],0,0,c)
 except AssertionError:out['zero_slope_rejections']+=1
 else:raise AssertionError('acceptedzeroslope')
 specs=json.loads(Path(data['field_data']).read_bytes())['fields']
 for spec in specs:
  begin=time.monotonic();f=Arithmetic(spec['p'],spec['mod']);base,E,D=family(f,rng);R0=tangent_row(base,D,f);var=[]
  for i in range(2):
   y=[0,0];y[i]=1;r=tangent_row(evaluate(base,E,y,f),D,f);var.append([f.sub(a,b)for a,b in zip(r,R0)])
  v,k=certificate(R0,var,f);scale=rng.randrange(1,f.q);v=[f.mul(a,scale)for a in v];k=dot(R0,v,f);pivot=next(i for i,a in enumerate(v)if a);_,I=inverse_block(f);j=next(j for j,a in enumerate(I[1])if a);c0=correction(base,f);base[j][3]=f.sub(base[j][3],f.mul(c0,f.inv(I[1][j])));assert correction(base,f)==0
  def c(theta):return correction(evaluate(base,D,theta,f),f)
  constraint=lambda z:z[1]==z[2];domains=[(0,1)]*3;r=call(f,domains,v,k,pivot,c,[constraint],max_domain_elements=data['max_domain_elements'],max_pair_work=data['max_pair_work'],max_projected_candidates=data['max_projected_candidates']);expected=brute(f,domains,c,[constraint]);assert r['complete']and r['roots']==expected and (0,0,0)in expected;out['contexts'].append({'name':spec.get('name',str(spec['p'])+'^'+str(len(spec['mod'])-1)),'root_count':r['root_count'],'roots':r['roots'],'input_candidates':r['input_candidates'],'projected_candidates':r['projected_candidates'],'projected_sizes':r['projected_sizes'],'pair_work':r['pair_work'],'seconds':time.monotonic()-begin,'pivot':pivot})
 out['all_registered_contexts_complete']=len(out['contexts'])==len(specs);out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Exact bounded-domain field-line pullback with missed-root/expansion/cap controls and17field Hensel component oracles','If passed,derive an implicit interval projection/shared-witness representation with cost admission before large prime domains;otherwise preserve the exact counterexample and repair its contract in a new variant.')
