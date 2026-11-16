from pathlib import Path
import json,hashlib,itertools,resource,sys,time
from .separator import equal_one,rank_two,cover,zeta,compose_or
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'small_sets':0,'controls':[],'actual':[],'qualified_speedup':None};error=None
 try:
  for encoded in range(256):
   values=[v for v in range(8)if encoded>>v&1];brute=any(all((a&v).bit_count()%2 or(b&v).bit_count()%2 for v in values)for a,b in itertools.product(range(8),repeat=2));forms,first=rank_two(values,3,True);assert(forms is not None)==brute;general=cover(values,3);assert(general is None)==(0 in values)
   if general is not None:assert all(any((v&a).bit_count()%2 for a in general)for v in values)
   out['small_sets']+=1
  records=json.loads(Path('research/image_cell_certificates_20261115_v67/raw.json').read_text())['out']['cells']
  for row in records:
   coefficients=[int(x,16)for x in row['coefficient_hex']];values=zeta(coefficients);begin=time.monotonic();forms,first=rank_two(values,83);assert forms is not None;assert compose_or(coefficients,forms)==1
   if first['solution']is None:
    dependency=first['odd_dependency'];assert dependency.bit_count()%2;v=0
    for i,x in enumerate(values):
     if dependency>>i&1:v^=x
    assert v==0
   bad=values.copy();bad[0]=0;assert rank_two(bad,83)[0]is None;assert cover(bad,83)is None
   out['actual'].append({'width':row['width'],'points':len(values),'forms_hex':[hex(x)for x in forms],'output_degree_upper_bound':len(forms),'rank_one_impossible':first['solution']is None,'rank_one_conflict_hex':hex(first['odd_dependency'])if first['odd_dependency']is not None else None,'exact_composed_anf_hex':'0x1','synthesis_and_verification_seconds':time.monotonic()-begin,'coefficient_sha256':hashlib.sha256(json.dumps(row['coefficient_hex'],separators=(',',':')).encode()).hexdigest()})
  out['field_evaluations_this_experiment']=0;out['coefficient_provenance']='C67used exhaustive norm evaluations;reuse doesnot establish nonenumerative searchspeedup'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact quadratic parity separator and Boolean composition on the previously saturated imagecell,with exhaustive small-set andzero-containing controls.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Test fresh larger andpositive-containing cells;keep prior coefficientconstruction cost in every end-to-end assessment.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
