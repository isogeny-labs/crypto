"""Frozen independent controls and cold paid target-family support comparison."""
from pathlib import Path
import hashlib,itertools,json,resource,statistics,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.certified_support_20261115_v44 import solver as s
from research.norm_block_reuse_20261115_v36 import backend as blocks
from research.inversion_free_norm_20261115_v34 import compiler
from research.k0_reciprocal_cells_20261115_v38.solver import coefficients
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic()

def controls():
 cases=positives=repeated_counterexamples=0;digest=hashlib.sha256()
 for n,m,d,degree in[(2,7,2,5),(3,11,2,3),(4,19,2,2)]:
  f=BinaryField(n,m);lam=s.compile_basis(f,[1<<i for i in range(d)])[0];D=range(1<<d)
  products={tuple(coefficients(f,rs))for rs in itertools.combinations_with_replacement(D,degree)}
  for cs in itertools.product(range(f.order),repeat=degree):
   H=tuple(cs)+(1,);want=H in products;got=s.generic(f,H,lam);assert got==want
   if want and s.remainder(f,H,lam):repeated_counterexamples+=1
   digest.update(bytes([got]));cases+=1;positives+=got
 # Every small-family parameter retains generic semantics when a certificate is requested.
 family_states=0
 for n,m in[(2,7),(3,11)]:
  f=BinaryField(n,m);A=0;z=next(x for x in range(1,f.order)if any(f.square(c)^c==x for c in range(f.order)))
  c=next(c for c in range(f.order)if f.square(c)^c==z);y=0;B=f.mul(f.square(z),z)
  family=s.Family(f,A,B,z,y,c,2,True);assert not family.certified
  products={tuple(coefficients(f,rs))for rs in itertools.combinations_with_replacement(range(4),5)}
  for h,u,v in itertools.product(range(f.order),repeat=3):
   assert family.support(h,u,v)==(family.polynomial(h,u,v)in products);family_states+=1
 assert repeated_counterexamples>0
 return dict(polynomial_cases=cases,supported=positives,repeated_shortcut_counterexamples=repeated_counterexamples,fallback_family_states=family_states,transcript_sha256=digest.hexdigest())

def candidates(f,ctx,rule):
 old=json.loads((ROOT/rule['centroid_receipt']).read_text())['out']['actual']['domain'];hs=[old['particular']]
 for i in range(1,rule['outer_h']):hs.append(hs[-1]^old['basis'][(i&-i).bit_length()-1])
 C=blocks.Context(f,s.Z,ctx['d'],hs,[0],(35,52,69));C.prepare_blocks();seed=rule['seed'];rows=[]
 for h in hs:
  fiber=C.block(h,0);assert fiber is not None and len(fiber['basis'])==73
  for _ in range(rule['per_h']):
   seed=(seed*6364136223846793005+1442695040888963407)&((1<<128)-1);bits=seed&((1<<73)-1)
   uv=fiber['particular']^blocks.apply(fiber['basis'],bits);u=uv&((1<<83)-1);v=uv>>83
   H=compiler.norm_coefficients(f,ctx,h,u,v);assert H[4]<1<<18 and H[3]<1<<35 and H[2]<1<<52 and H[1]<1<<69
   rows.append((h,u,v))
 return rows

def child(method):
 begin=time.monotonic();rule=json.loads((P/'DATA.json').read_text());f=BinaryField(83,s.MOD);ctx=compiler.compile_context(f,0,s.Z,s.QY^s.Z)
 family=s.Family(f,0,1,s.Z,s.QY^s.Z,ctx['c'],18,method=='certified');rows=candidates(f,ctx,rule);dig=hashlib.sha256();accepted=0;core=time.monotonic()
 for h,u,v in rows:
  good=family.support(h,u,v);accepted+=good;dig.update(json.dumps([h,u,v,good],separators=(',',':')).encode())
 out=dict(method=method,candidates=len(rows),accepted=accepted,fingerprint=dig.hexdigest(),filter_seconds=time.monotonic()-core,total_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
 print(json.dumps(out,separators=(',',':')))

def main():
 out=dict(records=[]);error=None
 try:
  out['controls']=controls()
  for rep,order in enumerate([['generic','certified'],['certified','generic'],['generic','certified']]):
   for method in order:
    assert time.monotonic()-START<65
    t=time.monotonic();run=subprocess.run([sys.executable,'-m','research.certified_support_20261115_v44.worker',method],capture_output=True,text=True,timeout=12)
    assert run.returncode==0,run.stderr;row=json.loads(run.stdout);row.update(cold_seconds=time.monotonic()-t,repetition=rep,stderr=run.stderr);out['records'].append(row)
  assert len({r['fingerprint']for r in out['records']})==1
  out['median_cold_seconds']={m:statistics.median(r['cold_seconds']for r in out['records']if r['method']==m)for m in ['generic','certified']}
  out['AA_spread']={m:max(r['cold_seconds']for r in out['records']if r['method']==m)/min(r['cold_seconds']for r in out['records']if r['method']==m)-1 for m in ['generic','certified']}
  out['component_ratio']=out['median_cold_seconds']['generic']/out['median_cold_seconds']['certified']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result';out['component_ratio']=None;out['complete_controls']=False
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),median_cold_seconds=out.get('median_cold_seconds'),AA_spread=out.get('AA_spread'),component_ratio=out.get('component_ratio'),peak_child_rss_kib=[r['peak_rss_kib']for r in out['records']],parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,worker_seconds=time.monotonic()-START,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_isolation=False,qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Guarded exact k0 support predicate; paid component timing only.',evidence=ev,next_actions=['If savings small, use remainder equations to attack whole coefficient cells; do not repeat unchanged timings. Preserve generic multiplicity behavior and all original-chain open branches.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':
 if sys.argv[1]in ['generic','certified']:child(sys.argv[1])
 else:main()
