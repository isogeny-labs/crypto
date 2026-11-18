from pathlib import Path
from itertools import product,combinations_with_replacement
import json,hashlib,resource,sys,time
from .algebra import translate,j2,orbit_polynomial,literal_s3
from research.image_cells_20261115_v67.algebra import BField,roots_poly,rem,image,evaluate,binary_tower,tower_value,tower_mod,supported
from research.image_cells_actual_20261115_v67.celllib import echelon,quotient
from research.odd_quadratic_lift_20261115_v63.worker import F as Odd
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'small_translation_checks':0,'orbit_profiles':[],'actual':[],'qualified_speedup':None};error=None
 try:
  for f,m,corpus in [(BField(3,11),3,[list(c)+[1]for c in product(range(8),repeat=3)]),(BField(2,7),5,[roots_poly(xs,BField(2,7))for xs in combinations_with_replacement(range(4),5)])]:
   L=roots_poly([0,1],f)
   for H in corpus:
    B0=image(H,rem(L,H,f),f)
    for u in range(f.q):
     H1=translate(H,u,f);B1=image(H1,rem(L,H1,f),f);assert B1==translate(B0,evaluate(L,u,f),f);assert j2(B0,f)==j2(B1,f);out['small_translation_checks']+=1
  for name,f,domain,shifts,z,A,B in [('binary8',BField(3,11),range(4),range(4),4,1,3),('binary16',BField(4,19),range(4),range(4),6,8,7),('zero-control',BField(3,11),range(3),range(4),0,0,0),('odd3',Odd(3),range(3),range(3),1,1,1),('odd9',Odd(3,2),range(3),range(3),4,1,1)]:
   tuples=polys=extraneous=0
   for beta in product(domain,repeat=3):
    R=orbit_polynomial(beta,z,A,B,f);assert len(R)==13 and R[-1]==1
    for t in shifts:
     a,b,c=[f.add(x,t)for x in beta];actual=[u for u in range(f.q)if literal_s3(a,b,u,A,B,f)==0 and literal_s3(u,c,z,A,B,f)==0];selected=evaluate(R,t,f)==0;assert not actual or selected;recovered=actual if selected else[];assert recovered==actual;tuples+=len(actual);extraneous+=selected and not actual
    polys+=1
   out['orbit_profiles'].append({'name':name,'characteristic':f.p,'field_size':f.q,'orbit_polynomials':polys,'monic_degree':12,'complete_original_tuples':tuples,'extraneous_shift_candidates':extraneous})
  data=json.loads((P/'DATA.json').read_text());f=BField(83,0x800000000200000000007);scalars=binary_tower(f,18)[:13];W=[tower_value(1<<i,scalars,f)for i in range(13,18)];prod_basis=echelon(f.mul(a,b)for i,a in enumerate(W)for b in W[i:]);ns=Path(data['matrix']).read_text().split();values=[int(x,16)for x in ns[1:]];base=values[:6];cols=[values[i:i+6]for i in range(6,len(values),6)];out['trace_varying_columns']=sum(bool(c[4])for c in cols);assert out['trace_varying_columns']==0;anchor=int(data['anchor_hex'],16)&~511;negative=[]
  for corner in [0,511]:
   H=base.copy();bits=anchor|corner
   for i,c in enumerate(cols):
    if bits>>i&1:H=[a^b for a,b in zip(H,c)]
   negative.append(H)
  for name,H in [('negative0',negative[0]),('negative511',negative[1]),('positive',data['positive_H'])]:
   B0=image(H,tower_mod(H,scalars,f),f);J=j2(B0,f);g=quotient(J,prod_basis)
   for u in data['translations']:
    H1=translate(H,u,f);B1=image(H1,tower_mod(H1,scalars,f),f);assert B1==translate(B0,tower_value(u,scalars,f),f);assert j2(B1,f)==J
    if name.startswith('negative'):assert g;assert(H1[4]==base[4])==(u==0)
    out['actual'].append({'name':name,'translation_hex':hex(u),'in_kernel':u<8192,'image_j2_hex':hex(J),'quotient_hex':hex(g),'same_fixed_trace_chart':H1[4]==base[4]})
  f=BField(3,11);L=roots_poly([0,1],f);H=[0,0,0,0,0,1];shifted=translate(H,4,f);assert supported(H,range(4),f)['supported']and not supported(shifted,range(4),f)['supported'];assert j2(image(H,rem(L,H,f),f),f)==j2(image(shifted,rem(L,shifted,f),f),f);out['support_noninvariance_control']={'H':H,'shifted':shifted,'translation':4,'original_supported':True,'shifted_supported':False}
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Exacttranslation-invariant imageguards,actualfixed-chart exclusion,and monicdegree12orbit resultants with independent all-root small-field oracles.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Use degree12orbitpolynomial with exactdomainGCD andauxiliary recovery;benchmark against strongGray scanning andretain conditional/global scope distinction.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
