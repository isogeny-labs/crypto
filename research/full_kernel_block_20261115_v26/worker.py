"""Extend exact active-column compression to all35kernel-direction coordinates."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.shared_block_20261115_v26 import worker as b
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def actual():
 profile=json.loads((R/'research/kernel_incidence_20261115_v20/raw.json').read_text())['out']['profile'];families=[];summary={}
 for name,n in [('pre-Gold73',73),('Gold69',69)]:
  records=[v for v in profile['records']if v['family']==name];lookup={v['u']:v['intersection']['rows']for v in records};tables=[lookup[1<<i]for i in range(35)];m=len(tables[0]);zero=[(0,0)]*m;levels=[]
  for width in [6,9,18,35]:
   plan=b.make_plan([zero]+tables[:width],n)if width<=6 else plan_large(tables[:width],n)
   for v in plan['kernel_basis']:assert all(b.image(rows,v)==0 for rows in tables[:width])
   levels.append(dict(direction_bits=width,kernel_dimension=len(plan['kernel_basis']),active_columns=len(plan['complement_basis']),plan=plan))
  plan=levels[-1]['plan'];probes=[]
  for record in records:
   u=record['u'];rows=b.s.k.assemble(tables,u);assert rows==[tuple(v)for v in record['intersection']['rows']]
   original=b.solve(rows,n);rs=b.solve(b.reduced(rows,plan),len(plan['complement_basis']));lifted=b.lift(rows,plan,rs);assert (original is None)==(lifted is None)
   if original is not None:assert len(lifted['basis'])==len(original['basis'])
   probes.append(dict(u=u,consistent=original is not None,nullity=len(original['basis'])if original else None))
  summary[name]=dict(levels=[{k:v for k,v in level.items()if k!='plan'}for level in levels],fullwidth_prediction=len(plan['complement_basis'])<=64,independent_direct_field_systems_replayed=len(probes))
  families.append(dict(family=name,parameter_bits=n,basis_tables=tables,levels=levels,probes=probes))
 return dict(summary=summary,families=families,scope='Exact shared-kernel identities span all35u coordinates; feasibility probes64only, not all2^35 or originalroot search')
def plan_large(tables,n):
 K,_=b.s.nullspace([row[0]for rows in tables for row in rows],n);piv=b.reduce_basis(K);U=[]
 for i in range(n):
  v=1<<i
  while v:
   bit=v.bit_length()-1
   if bit not in piv:piv[bit]=v;U.append(1<<i);break
   v^=piv[bit]
 m=len(tables[0]);assert len(K)+len(U)==n
 return dict(kernel_basis=K,complement_basis=U,quotient_rows=[1<<i for i in range(m)],lift_matrix=[0]*m,image_rank=0,inner_bits=n,outer_bits=len(tables),original_rows=m)
def main():
 out={};error=None
 try:out['controls']=b.controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1048576
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Full35direction-coordinate shared-kernel identities and exact lifted probe fibers.',evidence=ev,next_actions=['Use fullwidth rank to accept or refute compactcore claim beforetiming; ifwidthgrows, seek adaptivecellswithchargedcover ratherthanextrapolateprefix. Odd-field backendandoriginalrootbaseline streamsremainopen.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
