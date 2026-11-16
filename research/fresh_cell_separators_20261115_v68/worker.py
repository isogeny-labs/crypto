from pathlib import Path
import json,hashlib,resource,sys,time
from research.quadratic_cell_separator_20261115_v68.separator import rank_two,cover,zeta,compose_or
from research.image_cells_20261115_v67.algebra import BField,binary_tower,tower_mod,tower_value,roots_poly
from research.image_cells_actual_20261115_v67.celllib import echelon,quotient,second_image
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'cells':[],'field_evaluations':0,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());ns=Path(d['matrix']).read_text().split();count=int(ns[0]);nums=[int(x,16)for x in ns[1:]];base=nums[:6];cols=[nums[i:i+6]for i in range(6,len(nums),6)];assert len(cols)==count==73;f=BField(83,0x800000000200000000007);scalars=binary_tower(f,18)[:13];W=[tower_value(1<<i,scalars,f)for i in range(13,18)];prod=echelon(f.mul(a,b)for i,a in enumerate(W)for b in W[i:]);anchor=int(d['anchor_hex'],16);positive=d['positive_H'];assert positive==roots_poly([x for x,m in d['positive_roots']for _ in range(m)],f);assert all(0<=x<1<<18 for x,m in d['positive_roots'])
  for name,width in [('negative8',8),('negative9',9),('positive4',4)]:
   fixed=anchor&~((1<<width)-1);H0=positive.copy()if name=='positive4'else base.copy()
   if name!='positive4':
    for i,c in enumerate(cols):
     if fixed>>i&1:H0=[a^b for a,b in zip(H0,c)]
   values=[];begin=time.monotonic()
   for mask in range(1<<width):
    H=H0.copy()
    for i in range(width):
     if mask>>i&1:H=[a^b for a,b in zip(H,cols[i])]
    q=tower_mod(H,scalars,f);values.append(quotient(second_image(H,q,f),prod));out['field_evaluations']+=1
   coefficients=zeta(values);forms=cover(values,83);quadratic,first=rank_two(values,83)
   if forms is not None:assert compose_or(coefficients,forms)==1;assert all(any((v&a).bit_count()%2 for a in forms)for v in values)
   if name=='positive4':assert values[0]==0 and forms is None and quadratic is None
   out['cells'].append({'name':name,'width':width,'points':len(values),'zero_outputs':values.count(0),'forms_hex':[hex(a)for a in forms]if forms is not None else None,'degree_upper_bound':len(forms)if forms is not None else None,'quadratic_found':[hex(a)for a in quadratic]if quadratic is not None else None,'rank_one_impossible':first['solution']is None,'composed_anf_hex':'0x1'if forms is not None else None,'actual_anf_degree':max((i.bit_count()for i,v in enumerate(coefficients)if v),default=0),'coefficient_hex':[hex(a)for a in coefficients],'construction_and_verification_seconds':time.monotonic()-begin});assert time.monotonic()-START<65
  out['construction_enumerates_cells']=True;out['performance_claim']=False
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact multi-projection certificates on fresh larger image-guard cells with a supported-root positive control.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Seek sound circuit-level coefficient orcertificate synthesis without enumerating2^bassignments;all root andfield obligations persist.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
