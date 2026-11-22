from pathlib import Path
import json,hashlib,resource,sys,time
from research.image_cells_20261115_v67.algebra import BField,binary_tower,tower_value,tower_mod,image,supported,compose_mod
from .celllib import echelon,quotient,second_image
from rootfinder.binaryfield import BinaryField
from research.certified_support_20261115_v44 import solver as prior
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'records':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());f=BField(d['field_degree'],int(d['modulus_hex'],16));g=BinaryField(f.n,f.m);lam=prior.compile_basis(g,[1<<i for i in range(d['domain_dimension'])])[0];scalars=binary_tower(f,d['domain_dimension'])[:d['prefix']];basis=[tower_value(1<<i,scalars,f)for i in range(d['prefix'],d['domain_dimension'])];W={0}
  for v in basis:W|={x^v for x in list(W)}
  assert len(W)==32;prod=echelon(f.mul(a,b)for i,a in enumerate(basis)for b in basis[i:]);out.update(image_domain=sorted(W),image_basis=basis,product_rank=len(prod),quotient_dimension=f.n-len(prod));expected={r['name']:r for r in json.loads(Path(d['expected']).read_text())}
  for row in json.loads(Path(d['input']).read_text()):
   H=row['H'];begin=time.monotonic();q=tower_mod(H,scalars,f);B=image(H,q,f);assert not compose_mod(B,q,H,f);target=supported(B,sorted(W),f);image_seconds=time.monotonic()-begin;original=prior.generic(g,tuple(H),lam);assert target['supported']==original==expected[row['name']]['supported'];t2=second_image(H,q,f);assert t2==B[-3];out['records'].append({'name':row['name'],'H':H,'B':B,'supported':original,'image_roots':target['roots'],'image_remaining_degree':target['remaining_degree'],'second_quotient_hex':hex(quotient(t2,prod)),'image_seconds':image_seconds});assert time.monotonic()-START<65
  out['cases']=len(out['records']);out['positive']=sum(r['supported']for r in out['records']);out['negative']=out['cases']-out['positive']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exact GF83quintic image support matches original repeated-root-safe oracle on frozen affine-relaxation samples andpositive norms.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Use projected image coefficient for bounded whole-cell certificates;conditional preimage solving remains unresolved.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
