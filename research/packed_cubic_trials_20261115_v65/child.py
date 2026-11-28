from pathlib import Path
from types import SimpleNamespace
import json,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.general_binary_cubic_20261115_v60.oracles import original_triples
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients
from research.compiled_subspace_20261115_v31 import factor
from research.original_chain_lift_20261115_v55.worker import eq
from research.positive_binary_charts_20261115_v53 import math as curve
START=time.monotonic()
def main():
 v=json.loads(Path(sys.argv[1]).read_text());binary=sys.argv[2];backend=sys.argv[3];f=BinaryField(v['field_degree'],v['modulus']);A,B,z,y,d=[v[j]for j in ['A','B','z','source_ordinate','d']];assert z>=1<<d and f.mul(curve.sqrt(f,B),f.inv(z))>=1<<d;assert curve.on(f,A,B,(z,y));cmd=[binary]+({'reference':[],'gray':['--constant-gray'],'packed':['--constant-gray','--packed'],'verified':['--constant-gray','--packed','--verify-packed'],'profile':['--constant-gray','--packed','--profile']}[backend])+['--A',hex(A),'--B',hex(B),'--z',hex(z),'--y',hex(v['cli_ordinate']),'--dimension',str(d),'--max-outer','262144','--max-candidates','16777216','--max-nullity','12','--max-accepted','1024','--record-families','0','--max-seconds','8'];run=subprocess.run(cmd,capture_output=True,text=True,timeout=10);native=json.loads(run.stdout);roots=set()
 for rec in native['accepted_norms']:
  h=int(rec['b'],16);r=int(rec['c'],16);C=tuple(int(t,16)for t in rec['cubic']);assert C==coefficients(f,A,B,z,y,h,r);support=factor.supported_roots(SimpleNamespace(p=2,field=f),C,[1<<i for i in range(d)]);assert support['supported']and sum(m for x,m in support['roots'])==3 and h;s=f.square(z)^f.mul(r,z)^f.mul(h,y);points=[]
  for x,m in support['roots']:
   yp=f.mul(f.square(x)^f.mul(r,x)^s,f.inv(h));assert curve.on(f,A,B,(x,yp));points +=[(x,yp)]*m
  assert curve.total(f,A,B,points+[(z,y)])is None
  for a,b,c,u in original_triples(f,B,support['roots'],z):assert not eq(a,b,u,B,f.modulus)and not eq(u,c,z,B,f.modulus);roots.add((a,b,c,u))
 complete=run.returncode==0 and native['complete']is True and native['stats']['outer_visited']==native['stats']['outer_cardinality_exact']and native['stats']['candidates_visited']==native['stats']['candidates_planned'];out={'backend':backend,'complete_original_three_source':complete,'exact_root_count':len(roots)if complete else None,'verified_root_lower_bound':len(roots),'original_tuples':sorted(roots),'native_stats':native['stats'],'native_stdout':run.stdout,'stderr':run.stderr,'native_peak_rss_kib':resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss};trial=json.dumps(out,separators=(',',':'))
 if time.monotonic()-START>=10:out.update(complete_original_three_source=False,exact_root_count=None,reason='late child serialization')
 text=json.dumps(out,separators=(',',':'));print(text)
if __name__=='__main__':main()
