"""Degree-stratified shared duals for a frozen kernel-direction pencil."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.kernel_class_20261115_v23 import worker as k
b=k.b;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def xor(values):
 v=0
 for x in values:v^=x
 return v
def values(sol):
 if not sol['consistent']:return []
 return sorted(sol['particular']^xor(v for i,v in enumerate(sol['basis'])if t>>i&1)for t in range(1<<sol['nullity']))
def stratum(tables,n,d):
 dual=k.duals(tables[:d+1],n);rows=[(v['syndrome']&((1<<d)-1),(v['syndrome']>>d)&1)for v in dual['vectors']];sol=b.norm.solve(rows,d)
 candidates=[(1<<d)|v for v in values(sol)]
 return dict(degree=d,duals=dual,compatibility_rows=rows,compatibility_solution=sol,candidate_directions=candidates)
def controls():
 cases=0
 for code in range(4096):
  vs=[(code>>(3*i))&7 for i in range(4)];tables=[[(v&3,v>>2)for v in vs[:2]],[(v&3,v>>2)for v in vs[2:]]]
  for d in[0,1]:
   result=stratum(tables,2,d);candidates=set(result['candidate_directions'])
   for u in range(1<<d,1<<(d+1)):
    rows=k.assemble(tables,u);actual=any(all((m&v).bit_count()%2==r for m,r in rows)for v in range(4))
    b.check(not actual or u in candidates,'complete tiny strata never discard feasible directions')
   cases+=1
 return dict(pencils=4096,degree_strata=cases,all_tiny_necessary_guards_sound=True)
def remainder(v,p):
 while v.bit_length()>=p.bit_length():v^=p<<(v.bit_length()-p.bit_length())
 return v
def actual():
 data=json.loads((R/'research/kernel_class_20261115_v23/raw.json').read_text())['out']['actual'];out=[];summary={}
 for family in data['families']:
  good={r['u']for r in family['records']if r['consistent']};rows=[]
  for d in range(9):
   result=stratum(family['basis_tables'],family['parameter_bits'],d);actual_good=sorted(u for u in good if u.bit_length()-1==d);cand=result['candidate_directions']
   b.check(set(actual_good)<=set(cand),'exact original pencil positive directions preserved')
   result.update(actual_consistent=actual_good,exact_stratum=set(actual_good)==set(cand),excluded_by_shared_duals=(1<<d)-len(cand))
   rows.append(result)
  smallest=min(good)if good else None;low=[u for u in range(1,64)if u in good];pattern=smallest is not None and all((u in good)==(remainder(u,smallest)==0)for u in range(1,64))
  prefix=k.duals(family['basis_tables'][:6],family['parameter_bits']);prefix_rows=[(v['syndrome']&63,0)for v in prefix['vectors']];prefix_sol=b.norm.solve(prefix_rows,6);prefix_values=set(values(prefix_sol))
  b.check((set(low)|{0})<=prefix_values,'low6 shared-dual guard is necessary')
  summary[family['family']]=dict(strata=[dict(degree=x['degree'],shared_dual_dimension=x['duals']['dimension'],candidate_count=len(x['candidate_directions']),actual_consistent_count=len(x['actual_consistent']),exact=x['exact_stratum'])for x in rows],smallest_consistent_u=smallest,smallest_polynomial_binary=bin(smallest)if smallest else None,all_low6_compatibility_matches_divisibility_by_smallest=pattern,low6_shared_dual_rank=prefix['syndrome_rank'],low6_shared_guard_exact=prefix_values==set(low)|{0},low6_compatible_nonzero=low)
  out.append(dict(family=family['family'],strata=rows,low6_common_duals=prefix,low6_compatibility_rows=prefix_rows,low6_solution=prefix_sol))
 return dict(summary=summary,families=out,scope='Exact finite degree-strata of exposed low9pencil, no extrapolation to all35bits/otherh/k/source/globalroots')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<256*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,novelty_claim=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Changed stratified shared-dual class after globalconstant-dual failure, with complete finite-pencil checks.',evidence=ev,next_actions=['Turn any exact low6divisibility guard into an algebraic certificate and test its valid parameter range; retainall higherdegree exceptions andallsource/globalfield obligations. No generalization from fitted exposed data without a new proof.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
