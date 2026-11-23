"""Exact supported-norm search on retained affine Gold branches."""
from pathlib import Path
import gzip,hashlib,json,resource,sys,time
from research.gold5_cells_20261115_v22 import support as s
b=s.b;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def controls():
 families=points=positive=repeats=0
 for n,mod in[(2,7),(3,11),(4,19)]:
  f=b.BinaryField(n,mod);d=min(2,n)
  for h in[0,1,2]:
   for k in range(f.order):
    for r in range(f.order):
     H0=b.norm.formula(f,0,0,1,h,k,r,0);cols=[[x^y for x,y in zip(b.norm.formula(f,0,0,1,h,k,r,1<<i),H0)]for i in range(n)]
     result=s.solve_fiber(f,H0,cols,d,1<<n);b.check(result['complete'],'tiny fiber complete');found=sorted(x['parameter']for x in result['supported']);expected=[]
     for t in range(f.order):
      H=b.norm.literal(f,0,1,0,1,h,k,r,t);truth=s.brute_support(f,H,d);b.check(s.supported(f,H,d)==truth,'independent repeated-root factor oracle versus nilpotent support')
      if truth:expected.append(t)
      points+=1
     b.check(found==expected,'complete independent supported parameter sets');families+=1;positive+=len(expected)
 return dict(fields=[4,8,16],families=families,literal_norm_parameters=points,supported_parameters=positive,complete_support_sets_equal=True,repeated_root_safe=True)
def actual():
 D=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=b.BinaryField(83,int(D['extension_field']['modulus_hex'],16));A=int(D['curve']['a_hex'],16);z=int(D['target']['x_hex'],16);y=int(D['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;mask=(1<<83)-1
 data=json.loads((R/'research/whole_gold_cells_20261115_v21/raw.json').read_text())['out']['actual'];cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate']
 raw_direction=lambda t:s.xor(v for i,v in enumerate(cert['raw_basis'])if t>>i&1)
 lift=lambda B,t:s.xor(v for i,v in enumerate(B)if t>>i&1)
 cells=[];subcells=[];answers=[];enumerated=0;unresolved=0;ranks={}
 for cell in data['cell_records']:
  sol=cell['Gold_solution']
  if not sol['consistent']:continue
  u=cell['u'];frame=cell['cell_parameter_basis'];base=cell['cell_parameter_particular']^lift(frame,sol['particular']);directions=[lift(frame,v)for v in sol['basis']]
  raw0=cert['raw_particular']^raw_direction(base);rawcols=[raw_direction(v)for v in directions];outer,inner=s.decompose([v&mask for v in rawcols]);rr=len(outer);ranks[str(rr)]=ranks.get(str(rr),0)+1
  record=dict(u=u,parameters=len(directions),r_rank=rr,r_inner_bits=len(inner),complete=True)
  if rr>4 or len(subcells)+(1<<rr)>128:record.update(complete=False,reason='outerprojection cap');unresolved+=1;cells.append(record);continue
  for t in range(1<<rr):
   offset=lift(outer,t);parameter0=base^lift(directions,offset);B=[lift(directions,v)for v in inner];raw=cert['raw_particular']^raw_direction(parameter0);r=raw&mask;sv=raw>>83
   colsraw=[raw_direction(v)for v in B];b.check(all(v&mask==0 for v in colsraw),'exact fixedr kernel')
   H0=b.norm.formula(f,A,z,y,h,u,r,sv);Hcols=[[a^c for a,c in zip(b.norm.formula(f,A,z,y,h,u,r,sv^(v>>83)),H0)]for v in colsraw]
   result=s.solve_fiber(f,H0,Hcols,18,4096-enumerated);enumerated+=result['enumerated']
   for point in result['supported']:
    parameter=parameter0^lift(B,point['parameter']);raw_value=cert['raw_particular']^raw_direction(parameter);literal=b.norm.literal(f,A,1,z,y,h,u,raw_value&mask,raw_value>>83)
    b.check(literal==point['H']and s.supported(f,literal,18),'supported actual literal norm')
    answers.append(dict(parameter=parameter,k=u,r=raw_value&mask,sv=raw_value>>83,H=point['H']))
   subcells.append(dict(u=u,r=r,parameter0=parameter0,parameter_basis=B,**result))
   if not result['complete']:record['complete']=False;unresolved+=1
  cells.append(record)
 keys=[(a['k'],a['r'],a['sv'])for a in answers];b.check(len(keys)==len(set(keys)),'all norm parameter aliases retained without duplicate parameter tuples')
 complete=unresolved==0
 return dict(scope='Only selected retained k=u affine branch families; exact supported-norm parameter tuples if complete, not original curve/source/ghost roots',branch_cells=len(cells),r_projection_ranks=ranks,subcells=len(subcells),inner_dimensions=sorted({len(c['parameter_basis'])for c in subcells}),linear_inconsistent=sum(not c['linear']['consistent']for c in subcells),linear_nullities=[c['linear']['nullity']for c in subcells if c['linear']['consistent']],enumerated=enumerated,Gold9passed=sum(c['Gold9passed']for c in subcells),support_calls=sum(c['support_calls']for c in subcells),complete_selected_norm_families=complete,supported_norm_parameter_count_exact=len(answers)if complete else None,verified_supported_lower_bound=len(answers),supported_parameters=answers,cell_records=cells,subcell_records=subcells)
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1024*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 compact={k:v for k,v in out.get('actual',{}).items()if k not in['cell_records','subcell_records','supported_parameters']}
 ev=dict(error=error,controls=out.get('controls'),actual=compact,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None and compact.get('complete_selected_norm_families')else'inconclusive',summary='Exact rprojection, linearGold5, residualGold9 and repeated-safe norm-support search on selected whole affine branches.',evidence=ev,next_actions=['Derive reusable whole-cell contradictions for any closed branches and pursue outerr/w/h coverage; exact selected norm-family completion isnot original chain completion or a genuineGroebner speed claim.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
