from pathlib import Path
import itertools,json,hashlib,resource,statistics,subprocess,sys,time
from rootfinder import binaryfield as b
from research.certified_support_20261115_v44 import solver as old
from research.certified_support_20261115_v44 import worker as prior
from research.quintic_discriminant_20261115_v44 import solver as s
P=Path(__file__).resolve().parent;START=time.monotonic()

def controls():
 count=repeats=positive=0;digest=hashlib.sha256()
 for n,m in [(2,7),(3,11)]:
  f=b.BinaryField(n,m);lam=old.compile_basis(f,[1,2])[0]
  products={tuple(prior.coefficients(f,rs))for rs in itertools.combinations_with_replacement(range(4),5)}
  for cs in itertools.product(range(f.order),repeat=5):
   H=tuple(cs)+(1,);derivative=(H[1],0,H[3],0,1);repeat=len(b._gcd(H,derivative,f))>1
   assert s.repeated(f,H)==repeat
   got=s.supported(f,H,lam);assert got==(H in products)
   count+=1;repeats+=repeat;positive+=got;digest.update(bytes([repeat,got]))
   if count%1024==0:assert time.monotonic()-START<30
 return dict(complete_quintics=count,repeated=repeats,supported=positive,independent_gcd_and_root_product_oracles=True,transcript_sha256=digest.hexdigest())

def child(method):
 begin=time.monotonic();rule=json.loads((P/'DATA.json').read_text());f=b.BinaryField(83,old.MOD);ctx=prior.compiler.compile_context(f,0,old.Z,old.QY^old.Z)
 family=old.Family(f,0,1,old.Z,old.QY^old.Z,ctx['c'],18,False);rows=prior.candidates(f,ctx,rule);digest=hashlib.sha256();accepted=0;core=time.monotonic()
 for h,u,v in rows:
  H=family.polynomial(h,u,v);got=s.supported(f,H,family.lam)if method=='adaptive'else old.generic(f,H,family.lam)
  accepted+=got;digest.update(json.dumps([h,u,v,got],separators=(',',':')).encode())
 print(json.dumps(dict(method=method,candidates=len(rows),accepted=accepted,fingerprint=digest.hexdigest(),filter_seconds=time.monotonic()-core,total_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))

def main():
 out=dict(records=[]);error=None
 try:
  out['controls']=controls()
  for rep,order in enumerate([['generic','adaptive'],['adaptive','generic'],['generic','adaptive']]):
   for method in order:
    assert time.monotonic()-START<65
    t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.quintic_discriminant_20261115_v44.worker',method],capture_output=True,text=True,timeout=12)
    assert run.returncode==0,run.stderr;row=json.loads(run.stdout);row.update(cold_seconds=time.monotonic()-t,repetition=rep,stderr=run.stderr);out['records'].append(row)
  assert len({r['fingerprint']for r in out['records']})==1
  out['median_cold_seconds']={m:statistics.median(r['cold_seconds']for r in out['records']if r['method']==m)for m in ['generic','adaptive']}
  out['AA_spread']={m:max(r['cold_seconds']for r in out['records']if r['method']==m)/min(r['cold_seconds']for r in out['records']if r['method']==m)-1 for m in ['generic','adaptive']}
  out['component_ratio']=out['median_cold_seconds']['generic']/out['median_cold_seconds']['adaptive']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result';out['component_ratio']=None;out['complete_controls']=False
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),median_cold_seconds=out.get('median_cold_seconds'),AA_spread=out.get('AA_spread'),component_ratio=out.get('component_ratio'),peak_child_rss_kib=[r['peak_rss_kib']for r in out['records']],parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,worker_seconds=time.monotonic()-START,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_isolation=False,qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Inversion-free discriminant adaptive support predicate; paid component timing only.',evidence=ev,next_actions=['If savings small, use remainder equations to attack whole coefficient cells; do not repeat unchanged timings. Preserve generic multiplicity behavior and all original-chain open branches.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':
 if sys.argv[1]in ['generic','adaptive']:child(sys.argv[1])
 else:main()
