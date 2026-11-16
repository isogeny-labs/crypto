from pathlib import Path
import json,sys,time,resource,subprocess,hashlib
from .trace import small,BField,bit,trace_mask,batch,stripped
from rootfinder.binaryfield import BinaryField
from research.certified_support_20261115_v44 import solver as support
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());out['small']=small();assert time.monotonic()-START<35;f=BField(83,0x800000000200000000007);mask=trace_mask(f);out['trace_mask_hex']=hex(mask)
  rows=json.loads(Path(d['actual_inputs']).read_text());truth={r['name']:r for r in json.loads(Path(d['actual_expected']).read_text())};fixture=json.loads(Path(d['actual_point_fixtures']).read_text())['out']['cases'];curves={r['curve']+'_'+r['kind']+'_0':r for r in fixture};cases=[]
  for r in rows:
   A,B=(curves[r['name']]['A'],curves[r['name']]['B'])if r['name']in curves else(0,1)
   cases.append({'name':r['name'],'H':r['H'],'A':A,'B':B,'support_truth':truth[r['name']]['supported'],'scope':'constructed point fixture'if r['name']in curves else'exposed support/relaxation control'})
  nums=[int(v,16)for v in Path(d['matrix']).read_text().split()[1:]];base=nums[:6];cols=[nums[i:i+6]for i in range(6,len(nums),6)];assert len(cols)==73;state=d['seed'];seen=set()
  for i in range(d['fresh_samples']):
   state=(state*6364136223846793005+1442695040888963407)&((1<<128)-1);key=state&((1<<73)-1);assert key not in seen;seen.add(key);H=base.copy()
   for j,col in enumerate(cols):
    if key>>j&1:H=[a^b for a,b in zip(H,col)]
   cases.append({'name':'fresh_'+str(i),'H':H,'A':0,'B':1,'parameter_hex':hex(key),'scope':'fresh exposed affine relaxation sample'})
  den=[stripped(r['H'])[0][0]for r in cases];begin=time.monotonic();ivs=batch(den,f);out['batch_inverse_seconds_development']=time.monotonic()-begin;begin=time.monotonic();bits=[bit(r['H'],r['A'],r['B'],f,mask,iv)for r,iv in zip(cases,ivs)];out['batch_trace_seconds_development']=time.monotonic()-begin;begin=time.monotonic();assert bits==[bit(r['H'],r['A'],r['B'],f,mask)for r in cases];out['individual_trace_seconds_development']=time.monotonic()-begin
  g=BinaryField(83,f.m);lam=support.compile_basis(g,[1<<i for i in range(18)])[0]
  for r,v in zip(cases,bits):
   r['trace_bit']=v
   if r['name']in curves:assert v==0
   if r['name'].startswith('fresh_'):r['support_truth']=support.generic(g,tuple(r['H']),lam)
  out['actual_records']=cases;out['fresh_rejected']=sum(r['trace_bit']for r in cases if r['name'].startswith('fresh_'));out['old_relaxed_rejected']=sum(r['trace_bit']for r in cases if r['name'].startswith('relaxed_'));out['supported_controls_rejected']=sum(r['trace_bit']and r['support_truth']for r in cases if r['name'].startswith('known_factor_'));assert time.monotonic()-START<60
  b=subprocess.run(['g++','-O3','-std=c++17',str(P/'profile.cpp'),'-o',str(P/'profile')],capture_output=True,text=True,timeout=12);out['build']={'exit_code':b.returncode,'stdout':b.stdout,'stderr':b.stderr};assert b.returncode==0,b.stderr
  r=subprocess.run([str(P/'profile')],capture_output=True,text=True,timeout=10);out['profile_stdout']=r.stdout;out['profile_stderr']=r.stderr;assert r.returncode==0;out['join_preflight']=json.loads(r.stdout);assert int(out['join_preflight']['trace_mask_hex'],16)==mask
 except Exception as e:error=type(e).__name__+': '+str(e)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Vieta rationality trace parity and norm-identity redundancy against independent root controls; exact GF83 join-format refusal before allocation.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Retain parity counterexamples; if cut redundant, seek higher image-Vieta constraints; replace refused explicit pair table with a charged compact representation.']};text=json.dumps(result);assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
