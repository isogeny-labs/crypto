from pathlib import Path
from itertools import product
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.binary_norm_coverage_20261115_v57.oracle import points
from research.binary_norm_coverage_20261115_v57.worker import trace
from research.original_chain_lift_20261115_v55.solver import Lift
from research.original_chain_lift_20261115_v55.worker import literal,eq
from research.positive_binary_charts_20261115_v53 import math as curve
P=Path(__file__).resolve().parent;START=time.monotonic()
def run(n,mod,d):
 f=BinaryField(n,mod);A=B=1;e,ps=points(f,A,B,d);g=next(p for p in ps if p[0]==1 and not e.real(p[1]));assert curve.add(e,A,B,g,g)==(0,1);real=sorted(p for p in ps if p[0]and e.real(p[1]));choice=None
 for a,b,c in product(real,repeat=3):
  aux=[];total=(0,1);valid=True
  for p in[a,b,c]:
   total=curve.add(e,A,B,total,p)
   if total is None:valid=False;break
   aux.append(total[0])
  if not valid or total[0]<1<<d or f.inv(total[0])<1<<d:continue
  choice=(a,b,c,total,aux);break
 assert choice is not None
 a,b,c,T,partial=choice;xs=(1,1,a[0],b[0],c[0]);aux=(0,partial[0],partial[1]);values=xs+aux;z=T[0];assert all(e.real(v)for v in values);assert values in Lift(f,B).ordered(xs,z)
 for x,y,w in[(xs[0],xs[1],aux[0]),(aux[0],xs[2],aux[1]),(aux[1],xs[3],aux[2]),(aux[2],xs[4],z)]:assert eq(x,y,w,B,mod)==0
 for u,v,w in product(range(1,1<<d),repeat=3):assert eq(u,v,w,B,mod)!=0 and eq(u,v,f.inv(w),B,mod)!=0
 H=[1]
 for x in xs:
  out=[0]*(len(H)+1)
  for j,v in enumerate(H):out[j]^=literal(x,v,mod);out[j+1]^=v
  H=out
 obstruction=H[4]^z;assert trace(f,obstruction)==1;assert Lift(f,B).AS(obstruction)is None
 return {'n':n,'modulus':mod,'d':d,'A':A,'B':B,'original_values':values,'target':(z,T[1]^z),'H':H,'trace_H4_plus_z':1,'norm_family_empty_for_source_bag':True,'all_other_coverage_guards_pass':True,'failed_guard':'two nonrational x1 sources have product beta1 and sumP0','proof':'Every monicF3+YG1 norm quotient has H4=z+h^2+h; its trace obstruction must vanish. The explicit original finite root has trace1, excluding every h,k in that untwisted family.'}
def main():
 out=[];error=None
 try:
  for args in json.loads((P/'DATA.json').read_text())['profiles']:out.append(run(*args))
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Finite original roots refute unguarded binary norm-family coverage through an exact trace obstruction.','evidence':{'error':error,'cases':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Add a torsion-twisted chart for failed pair guards: remove the ghost pair and represent the three rational sources against targetQ+P0; verify a complete exceptional-chart union before broad promotion.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<45;print(text)
if __name__=='__main__':main()
