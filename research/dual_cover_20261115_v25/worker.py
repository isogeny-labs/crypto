"""Complete exposed low9 cover with constant and variable-dual guards."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.dual_refinement_20261115_v24 import worker as refine
from research.dual_syzygy_20261115_v25 import worker as syz
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def feasible(rows):
 piv={}
 for m,r in rows:
  while m:
   i=m.bit_length()-1
   if i not in piv:piv[i]=(m,r);break
   v,s=piv[i];m^=v;r^=s
  if not m and r:return False
 return True
def filter_cell(tables,n,p,B):
 first=refine.closure(tables,n,p,B,True)
 if first['empty']:return dict(constant_filter=first,syzygy=None,points=[])
 result=syz.guards(syz.pencil(tables,first['parameter0'],first['parameter_basis']),n)
 ps=syz.domain_points(first['parameter0'],first['parameter_basis']);kept=syz.candidates(result)
 return dict(constant_filter=first,syzygy=result,points=sorted(ps[t]for t in kept))
def controls():
 checked=exact=0
 for code in range(4096):
  vs=[(code>>(3*i))&7 for i in range(4)];tables=[[(v&3,v>>2)for v in vs[:2]],[(v&3,v>>2)for v in vs[2:]]];kept=set()
  for p in [0,2]:kept.update(filter_cell(tables,2,p,[1])['points'])
  truth={u for u in range(4)if any(all((m&x).bit_count()%2==r for m,r in refine.k.assemble(tables,u))for x in range(4))}
  assert truth<=kept;exact+=truth==kept;checked+=1
 return dict(pencils=checked,disjoint_affine_cover_domains=8192,all_feasible_retained=True,exact_covers=exact)
def actual():
 data=json.loads((R/'research/kernel_class_20261115_v23/raw.json').read_text())['out']['actual'];out=[];summary={}
 for f in data['families']:
  tables=f['basis_tables'];n=f['parameter_bits'];truth={u for u in range(512)if feasible(refine.k.assemble(tables,u))};recorded={r['u']for r in f['records']if r['consistent']}|{0};assert truth==recorded
  cover=[];seen=set();keptall=set()
  for high in range(8):
   p=high<<6;B=[1<<i for i in range(6)];domain=set(syz.domain_points(p,B));assert not domain&seen;seen.update(domain);c=filter_cell(tables,n,p,B);kept=set(c['points']);actualgood=truth&domain;assert actualgood<=kept<=domain;keptall|=kept
   before=set(c['constant_filter']['points']);d=len(c['constant_filter']['parameter_basis']);bad=len(before-actualgood);minimum=1<<max(0,d-2);obstruction=not c['constant_filter']['empty'] and bad<minimum
   if obstruction:assert c['syzygy']['guard_rank']==0
   c.update(high3=high,actual_directions=sorted(actualgood),exact=kept==actualgood,constant_candidate_count=len(before),incompatible_after_constants=bad,nonzero_quadratic_minimum_weight=minimum,quadratic_guard_impossible_from_density=obstruction)
   cover.append(c)
  assert seen==set(range(512));summary[f['family']]=dict(cover_cells=8,actual_feasible=len(truth),constant_candidates=sum(c['constant_candidate_count']for c in cover),final_candidates=len(keptall),exact=len(keptall)==len(truth),cells=[dict(high3=c['high3'],constant_candidates=c['constant_candidate_count'],syzygy_guard_rank=c['syzygy']['guard_rank']if c['syzygy']else None,candidates=len(c['points']),actual=len(c['actual_directions']),exact=c['exact'],density_obstruction=c['quadratic_guard_impossible_from_density'])for c in cover]);out.append(dict(family=f['family'],cells=cover))
 return dict(summary=summary,families=out,scope='Complete512direction exposed cover only; certificate-class obstruction not general impossibility; no originalroot completeness')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1048576
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Full exposed cover tests reach of variable-dual pruning and bounded-degree obstruction.',evidence=ev,next_actions=['Keep complete cover and all unresolved cells; test symbolic guarded rank/minor conditions only where lowdegree guards are provably insufficient. Match paid complete-pipeline costs before speed claims; close originalsource/outer/field coverage separately.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
