"""Iterate exact shared-dual filtering on an affine outer domain."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.kernel_class_20261115_v23 import worker as k
b=k.b;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def xor(vs):
 v=0
 for x in vs:v^=x
 return v
def affine_points(p,B):return sorted(p^xor(v for i,v in enumerate(B)if t>>i&1)for t in range(1<<len(B)))
def closure(tables,n,p,B,details=False):
 B=list(B);path=[len(B)];logs=[]
 for iteration in range(len(B)+2):
  matrices=[k.assemble(tables,p)]+[k.assemble(tables,v)for v in B];du=k.duals(matrices,n)
  rows=[(v['syndrome']>>1,v['syndrome']&1)for v in du['vectors']];sol=b.norm.solve(rows,len(B))
  if details:logs.append(dict(parameter0=p,parameter_basis=B,duals=du,rows=rows,solution=sol))
  if not sol['consistent']:return dict(empty=True,parameter0=None,parameter_basis=[],points=[],dimension_path=path,logs=logs,stop='contradiction')
  newp=p^xor(v for i,v in enumerate(B)if sol['particular']>>i&1);newB=[xor(v for i,v in enumerate(B)if t>>i&1)for t in sol['basis']]
  if len(newB)==len(B):return dict(empty=False,parameter0=newp,parameter_basis=newB,points=affine_points(newp,newB),dimension_path=path,logs=logs,stop='fixedpoint')
  p,B=newp,newB;path.append(len(B))
 raise AssertionError('dimension descent bound')
def controls():
 cases=0
 for code in range(4096):
  vs=[(code>>(3*i))&7 for i in range(4)];tables=[[(v&3,v>>2)for v in vs[:2]],[(v&3,v>>2)for v in vs[2:]]];domains=[(0,[1,2])]
  if code<256:domains +=[(1,[]),(2,[1])]
  for p,B in domains:
   result=closure(tables,2,p,B);kept=set(result['points'])
   for u in affine_points(p,B):
    rows=k.assemble(tables,u);feasible=any(all((m&x).bit_count()%2==r for m,r in rows)for x in range(4))
    b.check(not feasible or u in kept,'iterated closure preserves every tiny feasible direction')
   cases+=1
 return dict(tiny_pencils=4096,affine_domains=cases,all_feasible_outer_directions_retained=True)
def actual():
 raw=json.loads((R/'research/kernel_class_20261115_v23/raw.json').read_text())['out']['actual'];out=[];summary={}
 for family in raw['families']:
  tables=family['basis_tables'];n=family['parameter_bits'];good={r['u']for r in family['records']if r['consistent']}|{0};prefix=closure(tables,n,0,[1<<i for i in range(6)],True);truth={u for u in good if u<64}
  b.check(truth<=set(prefix['points']),'all actual low6feasible directions retained');guards=b.norm.solve([(v,0)for v in prefix['parameter_basis']],6)['basis']if not prefix['empty']else[]
  strata=[]
  for d in range(9):
   result=closure(tables,n,1<<d,[1<<i for i in range(d)],True);actualgood={u for u in good if u.bit_length()-1==d};b.check(actualgood<=set(result['points']),'degree closure preserves all actualfeasible directions');result.update(degree=d,actual_consistent=sorted(actualgood),exact=set(result['points'])==actualgood);strata.append(result)
  summary[family['family']]=dict(low6_dimension_path=prefix['dimension_path'],low6_candidate_count=len(prefix['points']),low6_actual_count=len(truth),low6_exact=set(prefix['points'])==truth,low6_parity_masks=guards,degree_strata=[dict(degree=c['degree'],dimension_path=c['dimension_path'],candidate_count=len(c['points']),actual_count=len(c['actual_consistent']),exact=c['exact'])for c in strata])
  out.append(dict(family=family['family'],prefix=prefix,parity_masks=guards,strata=strata))
 return dict(summary=summary,families=out,scope='Sound affine domain refinement on fixed coefficient-span pencils; fixedpoint not generic feasibility or source-root completeness')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<512*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Changed iterative common-dual closure after one-pass failure, preserving complete feasible-direction sets.',evidence=ev,next_actions=['Use only verified exact finite strata as certificates; where fixedpoints overapproximate, derive a guarded split or higher-order certificate while retaining all candidate fibers. Otherh/k/fields andoriginalroots remainopen.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
