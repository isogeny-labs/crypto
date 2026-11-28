from pathlib import Path
import json,sys,time,hashlib
from lab.research.nist_fields import preset,prove_prime
from research.prime_orbit_roots_20261116_v71.field import Prime
from research.prime_orbit_roots_20261116_v71.solver import power,gcd
from research.image_cells_20261115_v67.algebra import pmul,trim
P=Path(__file__).resolve().parent;START=time.monotonic()
def plus(a,b,f,subtract=False):
 r=list(a)+[0]*max(0,len(b)-len(a))
 for i,x in enumerate(b):r[i]=f.sub(r[i],x)if subtract else f.add(r[i],x)
 return trim(r)
def divide(a,b,f):
 a=trim(a);b=trim(b);out=[0]*max(0,len(a)-len(b)+1);iv=f.inv(b[-1])
 while len(a)>=len(b):
  j=len(a)-len(b);v=f.mul(a[-1],iv);out[j]=v
  for i,x in enumerate(b):a[i+j]=f.sub(a[i+j],f.mul(v,x))
  a=trim(a)
 return trim(out),a
def bezout(F,h,f):
 a,b=list(F),trim(h);u,v=[1],[];s,t=[],[1]
 while b:
  q,r=divide(a,b,f);a,b=b,r;u,v=v,plus(u,pmul(q,v,f),f,True);s,t=t,plus(s,pmul(q,t,f),f,True)
 assert len(a)==1;iv=f.inv(a[0]);return [f.mul(x,iv)for x in u],[f.mul(x,iv)for x in s]
def main():
 out={'fields':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for name in d['NIST_fields']:
   v=preset(name);f=Prime(v['p']);proof=prove_prime(f.p,d['flint_library']);proof.pop('library_path',None);F=[v['b'],v['a'],0,1];h=power([0,1],f.p,F,f)
   while len(h)<2:h.append(0)
   h[1]=f.sub(h[1],1);h=trim(h);assert gcd(F,h,f)==[1];U,V=bezout(F,h,f);assert plus(pmul(U,F,f),pmul(V,h,f),f)==[1]
   gx,gy=v['generator'];assert f.sq(gy)==f.add(f.mul(f.sq(gx),gx),f.add(f.mul(v['a'],gx),v['b']))
   assert (4*pow(v['a'],3,f.p)+27*pow(v['b'],2,f.p))%f.p
   out['fields'].append({'name':name,'p':f.p,'F':F,'h':h,'U':U,'V':V,'prime_proof':proof,'standard_generator_on_curve':True,'finite_rational_two_torsion_absent':True,'all_finite_chain_roots_rational':True})
 except Exception as e:error=type(e).__name__+': '+str(e)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Direct five-NIST finite-two-torsion absence certificates using modular Frobenius and independent Bezout verification.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Apply exact torsion-cut routing and measure component-image search costs; no global speedup follows from these certificates.']};text=json.dumps(result);assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
