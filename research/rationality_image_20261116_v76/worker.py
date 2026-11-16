from pathlib import Path
from itertools import combinations_with_replacement
import json,sys,time,hashlib,resource
from research.image_cells_20261115_v67.algebra import BField,roots_poly,rem,image
from research.vieta_rationality_20261116_v75.trace import inv,power,stripped
P=Path(__file__).resolve().parent;START=time.monotonic()
def label_image(H,A,B,f):
 H,removed=stripped(H);m=len(H)-1
 if not m:return [1],[],removed
 iv=inv(H[0],f);b=power(B,1<<(f.n-1),f);t=[f.mul(b,f.mul(v,iv))for v in H[1:]]
 if len(t)<2:t +=[0]*(2-len(t))
 t[0]^=A;t[1]^=1;t=rem(t,H,f);total=list(t)
 for _ in range(1,f.n):
  sq=[0]*max(0,2*len(t)-1)
  for i,x in enumerate(t):sq[2*i]=f.sq(x)
  t=rem(sq,H,f)
  if len(total)<len(t):total +=[0]*(len(t)-len(total))
  for i,x in enumerate(t):total[i]^=x
 return image(H,total,f),total,removed

def bits(C):
 m=len(C)-1;return [C[m-j]if m>=j else 0 for j in [1,2,4]]
def main():
 out={'small_cases':0,'actual':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA_BOUNDED.json').read_text());witnesses={}
  for n,M in d['small_fields']:
   f=BField(n,M)
   for A in [0,1]:
    rat={x:any(f.sq(y)^f.mul(x,y)==f.mul(f.sq(x),x)^f.mul(A,f.sq(x))^1 for y in range(1<<n))for x in range(1<<n)}
    for degree in d['degrees']:
     for xs in combinations_with_replacement(range(1<<n),degree):
      H=roots_poly(xs,f);C,L,removed=label_image(H,A,1,f);labels=[0 if rat[x]else 1 for x in xs if x];expected=roots_poly(labels,f);assert C==expected;v=bits(C);k=sum(labels);assert all(x in [0,1]for x in v)and sum(v[i]<<i for i in range(3))==k
      if k in [2,4]and k not in witnesses:witnesses[k]={'n':n,'A':A,'roots':xs,'image_polynomial':C,'bits':v}
      out['small_cases']+=1
  out['even_anti_witnesses']=witnesses;assert 2 in witnesses and 4 in witnesses;assert time.monotonic()-START<35
  prior=json.loads(Path(d['input']).read_text());assert prior['error']is None;records=prior['out']['actual_records'];take=[r for r in records if (r['name'].startswith('relaxed_')or r['name'].startswith('fresh_'))and int(r['name'].split('_')[-1])<16]+[r for r in records if r['scope']=='constructed point fixture'];assert len(take)==36
  fixtures=json.loads(Path('research/positive_binary_charts_20261115_v53/raw.json').read_text())['out']['cases'];expected={r['curve']+'_'+r['kind']+'_0':r for r in fixtures};f=BField(83,0x800000000200000000007)
  for r in take:
   begin=time.monotonic();C,L,removed=label_image(r['H'],r['A'],r['B'],f);elapsed=time.monotonic()-begin;v=bits(C);row={'name':r['name'],'H':r['H'],'A':r['A'],'B':r['B'],'removed_zero_roots':removed,'image_polynomial':C,'lambda_remainder':L,'e1_e2_e4':v,'reject':any(v),'seconds_development':elapsed,'scope':r['scope']}
   if r['name']in expected:
    ref=expected[r['name']];tags={t['x']:t['nonrational']for t in ref['point_checks']};k=sum(mult for x,mult in ref['factorization']['roots']if tags[x]);assert C==roots_poly([0]*(len(C)-1-k)+[1]*k,f);row['independent_anti_count']=k
   out['actual'].append(row);assert time.monotonic()-START<68
  out['old_rejected']=sum(r['reject']for r in out['actual']if r['name'].startswith('relaxed_'));out['fresh_rejected']=sum(r['reject']for r in out['actual']if r['name'].startswith('fresh_'));out['construction_enumerates_samples']=True
 except Exception as e:error=type(e).__name__+': '+str(e)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Higher rationality-label image coefficients recover repeated anti-root counts and test exposed actual affine samples with paid construction.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Compare cost and independent pruning against existing support/image filters; require compact whole-cell synthesis before a speed claim.']};text=json.dumps(result);assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
