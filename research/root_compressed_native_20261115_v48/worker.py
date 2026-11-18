from pathlib import Path
from types import SimpleNamespace
import json,hashlib,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.root_conditioned_fibers_20261115_v45 import solver as s
from research.certified_support_20261115_v44 import solver as prior
from research.compiled_subspace_20261115_v31 import factor
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def base_actual():
 rule=json.loads((P/'DATA.json').read_text());prior.verify_certificates();f=BinaryField(83,prior.MOD);ctx=s.c.compile_context(f,0,prior.Z,prior.QY^prior.Z);cover=json.loads((R/rule['centroid_receipt']).read_text())['out']['actual']['domain'];h=cover['particular'];assert h
 C=s.b.Context(f,prior.Z,ctx['d'],[h],[0],(35,52,69));C.prepare_blocks();fiber=C.block(h,0);assert len(fiber['basis'])==73;base,cols=s.model(f,ctx,h,fiber)
 text=str(len(cols))+'\n'+'\n'.join(' '.join(format(c,'x')for c in H)for H in [base]+cols)+'\n';text+=' '.join(format(v,'x') for v in [h,ctx['c'],ctx['d'],prior.Z])+'\n';(P/'matrix.txt').write_text(text)
 cmd=['g++','-O3','-std=c++17',str(P/'scan.cpp'),'-o',str(P/'scan')];t=time.monotonic();built=subprocess.run(cmd,capture_output=True,text=True,timeout=15);assert built.returncode==0,built.stderr;compile_seconds=time.monotonic()-t
 t=time.monotonic();run=subprocess.run([str(P/'scan')],input=text,capture_output=True,text=True,timeout=60);assert run.returncode==0,run.stderr;native=json.loads(run.stdout);native_wall=time.monotonic()-t
 checks=0
 for row in native['checkpoints']:
  x=row['x'];values=[s.evaluate(f,H,x)for H in cols];hom=s.b.solve(s.b.rows_from(values,0,83),73);sol=s.condition(f,base,cols,x)
  assert hom['rank']==row['rank']and (sol is not None)==row['consistent']
  if sol is not None:assert s.b.same(sol,dict(particular=int(row['particular'],16),basis=[int(v,16)for v in row['basis']]))
  checks+=1
 supported=[];seen=set();lam=prior.compile_basis(f,[1<<i for i in range(18)])[0];mask=(1<<83)-1
 for x,tstr in native['candidates']:
  t=int(tstr,16);assert t<1<<73;uv=fiber['particular']^s.b.apply(fiber['basis'],t);H=tuple(s.c.norm_coefficients(f,ctx,h,uv&mask,uv>>83));assert s.evaluate(f,H,x)==0
  if t in seen:continue
  seen.add(t)
  if not prior.remainder(f,H,lam):
   result=factor.supported_roots(SimpleNamespace(p=2,field=f),H,[1<<i for i in range(18)]);assert result['supported']and len(result['roots'])==5 and all(m==1 and root for root,m in result['roots']);supported.append(dict(parameter=t,u=uv&mask,v=uv>>83,H=H,roots=result['roots']))
  assert time.monotonic()-START<75
 complete=native['complete']and native['visited']==(1<<18)-1
 return dict(h=h,k=0,native=native,native_stderr=run.stderr,compile_stderr=built.stderr,compile_seconds=compile_seconds,native_wall_seconds=native_wall,independent_checkpoint_systems=checks,distinct_candidates=len(seen),supported=supported,norm_family_complete=complete,supported_norm_count_exact=len(supported)if complete else None,scope='One fixed h k0 norm family, certified x0 excluded; all other h/k and original chain open')

def actual():
 result=base_actual();other=json.loads((R/'research/root_conditioned_native_20261115_v45/raw.json').read_text());assert other['error']is None
 old=other['out'];key=lambda a:{(x,int(t,16))for x,t in a['native']['candidates']}
 assert result['norm_family_complete']and old['norm_family_complete'];assert key(result)==key(old)and result['supported']==old['supported'];result['complete_reference_candidate_match']=True
 return result

def main():
 out={};error=None
 try:out=actual();assert out['norm_family_complete'],'native scan incomplete'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:
  error='late serialized result';out['norm_family_complete']=False;out['supported_norm_count_exact']=None
  if 'native'in out:out['native']['complete']=False
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 n=out.get('native',{});ev=dict(error=error,h=out.get('h'),norm_family_complete=out.get('norm_family_complete',False),supported_norm_count_exact=out.get('supported_norm_count_exact'),visited=n.get('visited'),rank_histogram=n.get('rank_histogram'),compatible_cells=n.get('compatible_cells'),distinct_candidates=out.get('distinct_candidates'),independent_checkpoint_systems=out.get('independent_checkpoint_systems'),compile_seconds=out.get('compile_seconds'),native_wall_seconds=out.get('native_wall_seconds'),native_peak_rss_kib=n.get('peak_rss_kib'),parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,worker_seconds=time.monotonic()-START,raw_sha256=hashlib.sha256(raw).hexdigest(),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Complete root-conditioned source-root scan for one actual h k0 norm chart iff all caps pass.',evidence=ev,next_actions=['Use actual rank/output profile to design a batched all-h root-conditioned search; never extrapolate one chart to full target or quote timeout speed ratios.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
