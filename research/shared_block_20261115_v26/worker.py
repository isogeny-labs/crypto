"""Exact constant-image/domain-kernel reduction and full affine-fiber lifting."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.dual_syzygy_20261115_v25 import worker as s
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def solve(rows,n):
 piv={}
 for m,r in rows:
  while m:
   bit=m.bit_length()-1
   if bit not in piv:piv[bit]=(m,r);break
   v,c=piv[bit];m^=v;r^=c
  if not m and r:return None
 part=0
 for bit,(m,r)in sorted(piv.items()):
  if ((m&part).bit_count()&1)!=r:part^=1<<bit
 basis=[]
 for bit in range(n):
  if bit in piv:continue
  v=1<<bit
  for j,(m,r)in sorted(piv.items()):
   if (m&v).bit_count()&1:v^=1<<j
  basis.append(v)
 assert all((m&part).bit_count()%2==r for m,r in rows)
 assert all(all((m&v).bit_count()%2==0 for m,r in rows)for v in basis)
 return dict(particular=part,basis=basis,rank=len(piv))
def image(rows,v):return sum(((m&v).bit_count()&1)<<i for i,(m,r)in enumerate(rows))
def lin(B,v):return s.xor(x for i,x in enumerate(B)if v>>i&1)
def reduce_basis(values):
 piv={}
 for value in values:
  v=value
  while v:
   bit=v.bit_length()-1
   if bit not in piv:piv[bit]=v;break
   v^=piv[bit]
 return piv
def make_plan(coeff,n):
 d=len(coeff)-1;m=len(coeff[0]);assert d<=6 and n<=73 and m<=45
 K,_=s.nullspace([row[0]for rows in coeff[1:]for row in rows],n)
 piv=reduce_basis(K);U=[]
 for i in range(n):
  v=1<<i
  while v:
   bit=v.bit_length()-1
   if bit not in piv:piv[bit]=v;U.append(1<<i);break
   v^=piv[bit]
 assert len(K)+len(U)==n
 images=[image(coeff[0],v)for v in K];Q,r=s.nullspace(images,m)
 liftrows=[sum(((v>>i)&1)<<j for j,v in enumerate(images))for i in range(m)]
 return dict(kernel_basis=K,complement_basis=U,quotient_rows=Q,lift_matrix=liftrows,image_rank=r,inner_bits=n,outer_bits=d,original_rows=m)
def reduced(rows,plan):
 U=plan['complement_basis'];out=[]
 for q in plan['quotient_rows']:
  mask=s.xor(v for i,(v,r)in enumerate(rows)if q>>i&1);rhs=s.xor(r for i,(v,r)in enumerate(rows)if q>>i&1)
  out.append((sum(((mask&u).bit_count()&1)<<j for j,u in enumerate(U)),rhs))
 return out
def lift(rows,plan,sol):
 if sol is None:return None
 U=plan['complement_basis'];K=plan['kernel_basis'];v=lin(U,sol['particular']);rhs=sum((r^((m&v).bit_count()&1))<<i for i,(m,r)in enumerate(rows))
 lift=solve([(m,(rhs>>i)&1)for i,m in enumerate(plan['lift_matrix'])],len(K));assert lift is not None
 part=v^lin(K,lift['particular']);basis=[lin(K,x)for x in lift['basis']]
 for z in sol['basis']:
  w=lin(U,z);delta=image(rows,w);correction=solve([(m,(delta>>i)&1)for i,m in enumerate(plan['lift_matrix'])],len(K));assert correction is not None;basis.append(w^lin(K,correction['particular']))
 assert len(reduce_basis(basis))==len(basis)
 assert all((m&part).bit_count()%2==r for m,r in rows)
 assert all(all((m&v).bit_count()%2==0 for m,r in rows)for v in basis)
 return dict(particular=part,basis=basis)
def values(sol):return set()if sol is None else set(s.domain_points(sol['particular'],sol['basis']))
def controls():
 cases=0
 for code in range(4096):
  vs=[(code>>(3*i))&7 for i in range(4)];tables=[[(v&3,v>>2)for v in vs[:2]],[(v&3,v>>2)for v in vs[2:]]]
  for p in [0,2]:
   plan=make_plan(s.pencil(tables,p,[1]),2)
   for u in[p,p^1]:
    rows=s.k.assemble(tables,u);red=solve(reduced(rows,plan),len(plan['complement_basis']));lifted=lift(rows,plan,red);truth={x for x in range(4)if all((m&x).bit_count()%2==r for m,r in rows)}
    assert values(lifted)==truth;cases+=1
 return dict(pencils=4096,complete_original_inner_fibers=cases,all_sets_equal=True)
def actual():
 data=json.loads((R/'research/kernel_class_20261115_v23/raw.json').read_text())['out']['actual'];out=[];summary={}
 for f in data['families']:
  n=f['parameter_bits'];tables=f['basis_tables'];cells=[];same=0
  for h in range(8):
   p=h<<6;plan=make_plan(s.pencil(tables,p,[1<<i for i in range(6)]),n);records=[]
   for u in range(p,p+64):
    rows=s.k.assemble(tables,u);original=solve(rows,n);rs=solve(reduced(rows,plan),len(plan['complement_basis']));full=lift(rows,plan,rs);assert (original is None)==(full is None)
    if full is not None:assert len(full['basis'])==len(original['basis'])
    records.append(dict(u=u,consistent=full is not None,nullity=len(full['basis'])if full else None));same+=1
   cells.append(dict(high3=h,plan=plan,records=records))
  summary[f['family']]=dict(full_affine_fibers_checked=same,common_kernel_dimensions=[len(c['plan']['kernel_basis'])for c in cells],constant_image_ranks=[c['plan']['image_rank']for c in cells],residual_rows=[len(c['plan']['quotient_rows'])for c in cells],residual_columns=[len(c['plan']['complement_basis'])for c in cells],nonzero_cell_prediction=all(c['plan']['image_rank']>=35 for c in cells[1:]))
  out.append(dict(family=f['family'],cells=cells))
 return dict(summary=summary,families=out,scope='Complete linear fibers on exposed512direction families only; no normsupport/original/globalroot search')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1048576
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact common-domain-kernel/constant-image quotient with full lifted affine fibers.',evidence=ev,next_actions=['Use measured rank and complete paid cost to accept or retire constantblock assumption; ifrsmall seek sparse variablepivot or rankupdate structure ratherthanrepeat. Crossfield andoriginalroot gates remainopen.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
