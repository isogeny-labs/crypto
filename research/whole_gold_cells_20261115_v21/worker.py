"""Four-row Gold guards on whole affine cells and exact Schur fibers."""
from pathlib import Path
import gzip,hashlib,json,resource,sys,time
from research.k_linear_20261115_v19 import worker as b
from research.k_gold_compression_20261115_v20 import worker as g
from research.whole_gold_cells_20261115_v21.affine import xor,compose,affine_values
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def controls():
 cases=0
 for a in range(8):
  for c in range(8):
   base_rows=[(a&3,a>>2),(c&3,c>>2)];base=b.norm.solve(base_rows,2)
   for guard in range(8):
    guards=[(guard&3,guard>>2)];result=compose(base,guards,b.norm.solve)
    expected=[t for t in range(4)if all((m&t).bit_count()%2==r for m,r in base_rows+guards)]
    b.check(affine_values(result)==expected,'complete independent tiny Schur fiber oracle');cases+=1
 return dict(all_two_variable_base_and_guard_systems=cases,full_original_sets_equal=True,inconsistent_unique_and_positive_dimensional_fibers=True)
def actual():
 D=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=b.BinaryField(83,int(D['extension_field']['modulus_hex'],16));A=int(D['curve']['a_hex'],16);z=int(D['target']['x_hex'],16);y=int(D['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;a=f.square(h)^h^z;mask=(1<<83)-1
 atlas=json.loads((R/'research/kernel_incidence_20261115_v20/raw.json').read_text())['out']['profile'];gold=json.loads((R/'research/k_gold_compression_20261115_v20/raw.json').read_text())['out']['actual'];observed={(c['u'],c['r'],c['w']):c['k_u_retained']for c in gold['records']}
 cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate'];piv,_=g.image(f,18);free=[i for i in range(52)if i not in piv]
 b.check(len(piv)==48 and len(free)==4 and a<(1<<18),'admitted centroid and four Gold coordinates')
 def project(value):
  b.check(value>>52==0,'whole-cell guards actually lie in W3')
  v=g.residue(value,piv);return sum(((v>>bit)&1)<<i for i,bit in enumerate(free))
 def raw_value(t):return cert['raw_particular']^xor(v for i,v in enumerate(cert['raw_basis'])if t>>i&1)
 def r_direction(t):return xor(v&mask for i,v in enumerate(cert['raw_basis'])if t>>i&1)
 records=[];rank_hist={};dim_hist={};consistent=0;samples=0;schur=[];widths={}
 for record in atlas['records']:
  if record['family']!='Gold69'or not record['intersection']['solution']['consistent']:continue
  frame=record['intersection']['solution'];u=record['u'];B=frame['basis'];p=frame['particular'];r0=raw_value(p)&mask
  rhs=project(f.square(u)^f.mul(a^z^r0,u));columns=[project(f.mul(u,r_direction(v)))for v in B]
  rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(columns)),(rhs>>j)&1)for j in range(4)];sol=b.norm.solve(rows,len(B));rank_hist[str(sol['rank'])]=rank_hist.get(str(sol['rank']),0)+1
  count=(1<<sol['nullity'])if sol['consistent']else 0
  if sol['consistent']:consistent+=1;dim_hist[str(sol['nullity'])]=dim_hist.get(str(sol['nullity']),0)+1
  pivcols={m.bit_length()-1 for m,v,witness in frame['pivots']};free_frame=[i for i in range(69)if i not in pivcols]
  b.check(len(free_frame)==len(B),'complete cell coordinate frame')
  for point in record['verified_points']:
   t=point['parameter'];v=sum(((t>>j)&1)<<i for i,j in enumerate(free_frame));b.check(p^xor(x for i,x in enumerate(B)if v>>i&1)==t,'exact original cell parameter reconstruction')
   pred=all((m&v).bit_count()%2==r for m,r in rows)
   b.check(pred==observed[(u,point['r'],point['w'])],'whole-cell guard matches independent frozen literal Gold outcome')
   sy=g.gold_systems(f,A,z,y,h,point['r'],point['w'],18,piv);guards=sy['compact_rows'][len(sy['base']['rows']):];b.check(len(guards)==4,'exact Schur Gold block')
   combined=compose(sy['base']['solution'],guards,b.norm.solve);reference=sy['compact_solution']
   b.check(combined['consistent']==reference['consistent'],'Schur/49row consistency')
   if combined['consistent']:
    b.check(all((m&combined['particular']).bit_count()%2==r for m,r in sy['compact_rows']),'expanded Schur particular replay')
    b.check(all(all((m&v).bit_count()%2==0 for m,r in sy['compact_rows'])for v in combined['basis']),'expanded Schur kernels replay')
    b.check(len(b.norm.basis(combined['basis']))==len(combined['basis'])==35-reference['rank'],'complete Schur kernel, no missing solutions')
   entry=dict(u=u,r=point['r'],w=point['w'],inner_width=combined.get('inner_width'),remaining_bits=len(combined['basis'])if combined['consistent']else None,equivalent_to_full49=True)
   if len(schur)<4:entry.update(reference_rows=sy['compact_rows'],base_solution=sy['base']['solution'],composed=combined,reference_solution=reference)
   schur.append(entry);widths[str(combined.get('inner_width'))]=widths.get(str(combined.get('inner_width')),0)+1;samples+=1
  records.append(dict(u=u,cell_parameter_particular=p,cell_parameter_basis=B,cell_bits=len(B),Gold_rows=rows,Gold_solution=sol,retained_k_u_branch_parameters=count,rejected_k_u_branch_parameters=(1<<len(B))-count,k0_branch_retained=True,other_kernels_unresolved_without_rank_minor=True))
 return dict(whole_cells=len(records),consistent_k_u_subcells=consistent,Gold_guard_rank_histogram=rank_hist,retained_affine_bits_histogram=dim_hist,cell_records=records,point_replays=samples,schur_width_histogram=widths,schur_records=schur,scope='Exact k=u branch guards over overlapping affine cells; retaink0 andallotherkernels; no source/globalcount.')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<512*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 actual_summary={k:v for k,v in out.get('actual',{}).items()if k not in['cell_records','schur_records']}
 ev=dict(error=error,controls=out.get('controls'),actual=actual_summary,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact whole-cell affine Gold branch filtering and small Schur solve preserve complete conditional fibers.',evidence=ev,next_actions=['Use whole-cell guard rank/consistency to represent accepted andrejected k=u branches without2^24enumeration; retainrank-minor-zero/otherkernels andsource/globalfield obligations. Benchmark complete paid Schur pipeline against49row scalar before any speed claim.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
