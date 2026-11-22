"""Cross-binary-field complete pair-query oracle, not curve coverage."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.binary_wide_20261115_v41.backend import WideBinaryField as BinaryField
from research.rational_fraction_oracles_20261115_v39 import worker as ref
from research.rational_pair_query_20261115_v39 import solver as s
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def naive(f,a,b):
 v=0
 while b:
  if b&1:v^=a
  b>>=1;a<<=1
 while v.bit_length()>f.degree:v^=f.modulus<<(v.bit_length()-f.degree-1)
 return v
def controls():
 data=json.loads((R/'research/rational_pair_fields_20261115_v39/DATA.json').read_text());products=inverses=0
 for row in data['fields']:
  f=BinaryField(row['n'],row['modulus']);mask=f.order-1
  for i in range(32):
   a=int.from_bytes(hashlib.shake_256(b'wide-a-v41'+bytes([i])).digest((f.degree+7)//8),'little')&mask;b=int.from_bytes(hashlib.shake_256(b'wide-b-v41'+bytes([i])).digest((f.degree+7)//8),'little')&mask
   assert f.mul(a,b)==naive(f,a,b)and f.square(a)==naive(f,a,a);products+=2
   if a:assert naive(f,a,f.inv(a))==1;inverses+=1
  x=2
  for _ in range(f.degree):x=f.square(x)
  assert x==2
  for value in[-1,f.order]:
   try:f.validate(value)
   except ValueError:pass
   else:raise AssertionError('noncanonical admitted')
  try:BinaryField(f.degree,(1<<f.degree)|1)
  except ValueError:pass
  else:raise AssertionError('reducible modulus admitted')
 try:BinaryField(571,(1<<571)|(1<<10)|(1<<5)|(1<<2)|1,max_degree=512)
 except ValueError:budget_refused=True
 else:raise AssertionError('field budget ignored')
 return dict(independent_products=products,verified_inverses=inverses,Frobenius_fields=6,reducible_refused=True,canonical_and_budget_refused=budget_refused)
def main():
 out=dict(fields=[]);error=None;states=0
 try:
  out['controls']=controls();data=json.loads((R/'research/rational_pair_fields_20261115_v39/DATA.json').read_text())
  for spec in data['fields']:
   begin=time.monotonic();f=BinaryField(spec['n'],spec['modulus']);t=f.inv(data['a'])^f.inv(data['b']);records=[]
   for target in[t,0,spec['query']]:
    got=s.query(f,target,18);assert got['complete'];fraction,_=ref.reference(f,target,18);want=[]
    if fraction is not None:
     N,D=fraction
     if not N:want=[(a,a)for a in range(1,1<<18)];states+=(1<<18)-1
     else:
      for a in range(1,1<<18):
       den=ref.p.multiply(N,a)^D
       if den:
        b,r=ref.division(ref.p.multiply(D,a),den)
        if not r and 0<b<1<<18:want.append((a,b))
       states+=1
    assert got['pairs']==want
    for a,b in got['pairs']:
     assert(a==b)if not target else(f.inv(a)^f.inv(b)==target)
    records.append(dict(t=target,count=len(want),pair_sha256=hashlib.sha256(json.dumps(want,separators=(',',':')).encode()).hexdigest(),fraction=got.get('fraction'),divisors=got.get('divisors')))
   assert records[0]['count']>=2 and records[1]['count']==(1<<18)-1
   out['fields'].append(dict(n=f.degree,modulus=f.modulus,queries=records,complete=True,field_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss));assert time.monotonic()-START<78
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 out['exhaustive_a_states']=states;raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(controls=out.get('controls'),error=error,fields=[v['n']for v in out['fields']],complete_pair_queries=sum(len(v['queries'])for v in out['fields']),actual_a_states=states,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False,curve_coverage_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Reviewed widerRabin backend and exactpairquery over6binaryfields, independentwidearithmetic/fullprefix oracles; no curveclaim.',evidence=ev,next_actions=['Freeze paid query panel then compare with strongest completed pairscanner; keep genericprimitive correctness separate from curve/source/rootperformance. Pursue odd-extension/prime-interval rational heights with separate proofs.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
