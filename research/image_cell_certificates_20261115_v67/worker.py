from pathlib import Path
import json,hashlib,resource,sys,time
from research.image_cells_20261115_v67.algebra import BField,binary_tower,tower_mod,tower_value
from research.image_cells_actual_20261115_v67.celllib import echelon,quotient,second_image
from rootfinder.binaryfield import BinaryField
from research.certified_support_20261115_v44 import solver as prior
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'cells':[],'certified_points':0,'build_evaluations':0,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());text=Path(d['matrix']).read_text().split();count=int(text[0]);ns=[int(x,16)for x in text[1:]];base=ns[:6];cols=[ns[i:i+6]for i in range(6,len(ns),6)];assert count==len(cols)==73;out['norm_parameter_rank']=len(echelon(sum(c[j]<<(83*j)for j in range(6))for c in cols));f=BField(83,0x800000000200000000007);g=BinaryField(f.n,f.m);lam=prior.compile_basis(g,[1<<i for i in range(18)])[0];scalars=binary_tower(f,18)[:13];W=[tower_value(1<<i,scalars,f)for i in range(13,18)];prod=echelon(f.mul(a,b)for i,a in enumerate(W)for b in W[i:]);out['product_rank']=len(prod);out['quotient_dimension']=83-len(prod)
  anchor=int(d['anchor_hex'],16)
  for width in d['cell_widths']:
   fixed=anchor&~((1<<width)-1);H0=base.copy()
   for i,c in enumerate(cols):
    if fixed>>i&1:H0=[a^b for a,b in zip(H0,c)]
   values=[];support_count=0
   for mask in range(1<<width):
    H=H0.copy()
    for i in range(width):
     if mask>>i&1:H=[a^b for a,b in zip(H,cols[i])]
    q=tower_mod(H,scalars,f);values.append(quotient(second_image(H,q,f),prod));support_count+=prior.generic(g,tuple(H),lam);out['build_evaluations']+=1
   coefficients=values.copy()
   for i in range(width):
    for mask in range(1<<width):
     if mask>>i&1:coefficients[mask]^=coefficients[mask^(1<<i)]
   piv=echelon(coefficients[1:]);residue=quotient(coefficients[0],piv);dual=None
   if residue:
    bit=(residue&-residue).bit_length()-1;dual=sum(((quotient(1<<i,piv)>>bit)&1)<<i for i in range(83));assert(coefficients[0]&dual).bit_count()%2==1;assert all((v&dual).bit_count()%2==0 for v in coefficients[1:]);assert all((v&dual).bit_count()%2==1 for v in values);assert support_count==0;out['certified_points']+=1<<width
   out['cells'].append({'width':width,'fixed_parameter_hex':hex(fixed),'points':1<<width,'nonconstant_rank':len(piv),'constant_outside_span':bool(residue),'dual_certificate_hex':hex(dual)if dual is not None else None,'all_guard_values_nonzero':all(values),'original_supported_count':support_count,'anf_degree':max((i.bit_count()for i,v in enumerate(coefficients)if v),default=0),'coefficient_hex':[hex(v)for v in coefficients]});assert time.monotonic()-START<65
  out['construction_enumerates_every_cell']=True;out['performance_claim']=False
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Bounded exact Boolean image-guard cell certificates with explicit dual witnesses and paid exhaustive construction.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Replace enumerative coefficient construction with sound symbolic synthesis;high-degree or full-rank cells require a new bound,not anunsupported pruning claim.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
