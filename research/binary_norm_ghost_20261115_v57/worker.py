from pathlib import Path
from itertools import product,permutations
from collections import Counter
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import eq
from research.fused_affine_norm_20261115_v56.worker import independent_norm
from research.positive_binary_charts_20261115_v53 import math as curve
from research.binary_norm_coverage_20261115_v57.oracle import points,hermite_norm
P=Path(__file__).resolve().parent;START=time.monotonic()
def run(n,mod,A,B,d):
 f=BinaryField(n,mod);e,ps=points(f,A,B,d);beta=B
 for _ in range(n-1):beta=f.square(beta)
 domain=range(1,1<<d)
 for a,b,c in product(domain,repeat=3):assert eq(a,b,c,B,mod)!=0 and eq(a,b,f.mul(beta,f.inv(c)),B,mod)!=0
 ghosts=sorted(p for p in ps if not e.real(p[1]));real=sorted(p for p in ps if p[0]and e.real(p[1]));assert ghosts and len(real)>2
 assert not any(f.mul(a[0],b[0])==beta for a,b in product(ghosts,repeat=2))
 choice=None
 for a,b in product(real,repeat=2):
  if a[0]==b[0]:continue
  T=curve.total(e,A,B,[(0,beta),a,b])
  if T is None or T[0]<1<<d or f.mul(beta,f.inv(T[0]))<1<<d:continue
  choice=(a,b,T);break
 assert choice is not None
 a,b,T=choice;g=next(p for p in ghosts if p[0]not in[a[0],b[0]]);bag=(g[0],0,g[0],a[0],b[0]);target=T[0];Qfixed=(target,T[1]^target);orders=sorted(set(permutations(bag)));byx={x:[p for p in ps if p[0]==x]for x in set(bag)};truth=set();witnesses=[];paths=0
 for xs in orders:
  for original in product(*(byx[x]for x in xs)):
   total=None;aux=[];valid=True
   for i,p in enumerate(original):
    total=curve.add(e,A,B,total,p)
    if i in[1,2,3]:
     if total is None or not e.real(total[0]):valid=False;break
     aux.append(total[0])
   if not valid or total is None or total[0]!=target or not e.real(total[1]):continue
   values=xs+tuple(aux);truth.add(values);paths+=1;Q=(target,total[1]^target);sources=list(original)
   if Q!=Qfixed:sources=[(x,y^x)for x,y in sources];Q=Qfixed
   C=Counter(sources+[Q]);assert all(C[(x,y)]==C[(x,e.conjugate(y))]for x,y in C);V=hermite_norm(f,e,A,B,sources,Q);r,s,t,h,k=V;assert t==f.mul(f.square(target),target)^f.mul(r,f.square(target))^f.mul(s,target)^f.mul(f.mul(h,target)^k,Q[1]);H=independent_norm(f,A,B,target,Q[1],h,k,r,s);expected=(1,)
   for x in bag:expected=curve.polynomial.b._mul(expected,(x,1),f)
   assert H==tuple(expected)
   witnesses.append({'values':values,'function':V,'H':H,'ghosts':sum(not e.real(y)for x,y in sources)})
 L=Lift(f,B);got=set()
 for xs in orders:got.update(L.ordered(xs,target))
 assert got==truth and got;assert all(w['ghosts']==2 for w in witnesses)
 return {'n':n,'modulus':mod,'A':A,'B':B,'d':d,'bag':bag,'target':Qfixed,'source_orders_examined':len(orders),'finite_original_tuples':sorted(got),'group_paths':paths,'multiplicity_hermite_witnesses':witnesses,'source_zero_included':True,'repeated_ghost_included':True}
def main():
 out={'cases':[]};error=None
 try:
  for args in json.loads((P/'DATA.json').read_text())['profiles']:
   out['cases'].append(run(*args));assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=75:error='late serialized output'
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Nonvacuous repeated-ghost/zero coverage fixtures use exact finite original roots and Hermite norm witnesses on Koblitz and ordinary tiny curves.','evidence':{'error':error,'cases':[{'n':v['n'],'A':v['A'],'B':v['B'],'orders':v['source_orders_examined'],'original_tuples':len(v['finite_original_tuples']),'group_paths':v['group_paths']}for v in out['cases']],'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['Use explicit guarded theorem only on supported targets; global outer h/k enumeration and genuinely matched hard baselines remain open.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
