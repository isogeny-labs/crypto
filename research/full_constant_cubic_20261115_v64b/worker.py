from pathlib import Path
from types import SimpleNamespace
import hashlib,json,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.general_binary_cubic_20261115_v60.oracles import family,original_triples
from research.torsion_shifted_cubic_20261115_v58.solver import coefficients
from research.compiled_subspace_20261115_v31 import factor
from research.norm_block_reuse_20261115_v36 import backend as linear
from research.original_chain_lift_20261115_v55.worker import eq
from research.positive_binary_charts_20261115_v53 import math as curve
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def main():
 out={'complete_original_three_source':False,'verified_norms':[],'original_tuple_count_exact':None};error=None
 try:
  source=R/'research/constant_cubic_gray_20261115_v64b';v=json.loads((source/'full_ordinary_instance.json').read_text());f=BinaryField(v['field_degree'],v['modulus']);A,B,z,y,d=[v[j]for j in['A','B','z','source_ordinate','d']];beta=curve.sqrt(f,B);assert z>=1<<d and f.mul(beta,f.inv(z))>=1<<d;assert curve.on(f,A,B,(z,y));cmd=[str(source/'scan'),'--constant-gray','--A',hex(A),'--B',hex(B),'--z',hex(z),'--y',hex(v['cli_ordinate']),'--dimension',str(d),'--max-outer','262144','--max-candidates','16777216','--max-nullity','12','--max-accepted','1024','--record-families','16','--max-seconds','55'];start=time.monotonic();run=subprocess.run(cmd,capture_output=True,text=True,timeout=60);native=json.loads(run.stdout);out.update(native_wall_seconds=time.monotonic()-start,native_exit_code=run.returncode,native_stdout=run.stdout,native_stderr=run.stderr,native=native)
  cols=[(f.square(1<<i)^(1<<i))>>d for i in range(83)];outer=linear.solve(linear.rows_from(cols,z>>d,83-d),83);assert outer is not None and native['stats']['outer_cardinality_exact']==1<<len(outer['basis'])
  for rec in native['conditional_family_records']:
   ref=family(f,v,int(rec['b'],16));assert rec['consistent']==(ref is not None)
   if ref is not None:assert linear.same(ref,{'particular':int(rec['particular'],16),'basis':[int(t,16)for t in rec['kernel']]})
  original=set()
  for rec in native['accepted_norms']:
   h=int(rec['b'],16);r=int(rec['c'],16);C=tuple(int(t,16)for t in rec['cubic']);assert C==coefficients(f,A,B,z,y,h,r);roots=factor.supported_roots(SimpleNamespace(p=2,field=f),C,[1<<i for i in range(d)]);assert roots['supported']and sum(m for x,m in roots['roots'])==3 and h;s=f.square(z)^f.mul(r,z)^f.mul(h,y);ps=[]
   for x,m in roots['roots']:
    yp=f.mul(f.square(x)^f.mul(r,x)^s,f.inv(h));assert curve.on(f,A,B,(x,yp));ps +=[(x,yp)]*m
   assert curve.total(f,A,B,ps+[(z,y)])is None;values=original_triples(f,B,roots['roots'],z)
   for a,b,c,u in values:assert not eq(a,b,u,B,f.modulus)and not eq(u,c,z,B,f.modulus)
   original.update(values);out['verified_norms'].append({'h':h,'r':r,'C':C,'roots':roots['roots'],'original_tuples':values})
  # Validation-only witnesses are never passed to the native solver.
  witness=json.loads((source/'full_ordinary_witness.json').read_text());assert any(row['h']==witness['h']and row['r']==witness['r']and row['C']==tuple(witness['C'])for row in out['verified_norms']);assert set(map(tuple,witness['original_tuples']))<=original
  complete=run.returncode==0 and native['complete']is True and native['stats']['outer_visited']==native['stats']['outer_cardinality_exact']and native['stats']['candidates_visited']==native['stats']['candidates_planned'];out.update(complete_original_three_source=complete,original_tuples=sorted(original),original_tuple_count_exact=len(original)if complete else None,supported_norm_count_exact=len(out['verified_norms'])if complete else None,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,native_child_peak_rss_kib=resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,original_variable_count=3*d+83,field_bit_equations=166)
  if not complete:error='bounded native scan unresolved'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output'
 if error:
  out['complete_original_three_source']=False;out['original_tuple_count_exact']=None;out['supported_norm_count_exact']=None
  if 'native'in out:
   out['native']['complete']=False
   if 'supported_norm_count_exact'in out['native']:out['native']['supported_norm_count_lower_bound']=out['native'].pop('supported_norm_count_exact')
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Constant-block Gray complete original three-source roots on an ordinary full-widthGF83 curve under exact target guards, with all norm and original tuple replay.','evidence':{'error':error,'complete_original_three_source':out['complete_original_three_source'],'original_tuple_count_exact':out.get('original_tuple_count_exact'),'native_wall_seconds':out.get('native_wall_seconds'),'native_stats':out.get('native',{}).get('stats'),'parent_peak_rss_kib':out.get('parent_peak_rss_kib'),'native_child_peak_rss_kib':out.get('native_child_peak_rss_kib'),'worker_seconds':time.monotonic()-START,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'complete_global_root_search':False,'release_gate_claim':False},'next_actions':['Run preregistered ordinary/Koblitz cubic target ladders and genuine matched three-source GB baselines; separately reduce the five-source main search and broaden fields.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
