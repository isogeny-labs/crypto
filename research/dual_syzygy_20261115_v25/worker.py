"""Boolean degree-one left-syzygy guards on affine binary matrix pencils."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.kernel_class_20261115_v23 import worker as k
b=k.b;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def xor(vs):
 out=0
 for v in vs:out^=v
 return out
def nullspace(rows,n):
 piv={}
 for row in rows:
  while row:
   bit=row.bit_length()-1
   if bit not in piv:piv[bit]=row;break
   row^=piv[bit]
 free=[i for i in range(n)if i not in piv];out=[]
 for i in free:
  v=1<<i
  for j,row in sorted(piv.items()):
   if (row&v).bit_count()&1:v^=1<<j
  assert all((row&v).bit_count()%2==0 for row in rows);out.append(v)
 return out,len(piv)
def domain_points(p,B):
 return [p^xor(v for i,v in enumerate(B)if t>>i&1)for t in range(1<<len(B))]
def pencil(tables,p,B):return [k.assemble(tables,p)]+[k.assemble(tables,v)for v in B]
def guards(coeff,n):
 d=len(coeff)-1;m=len(coeff[0]);assert d<=6 and m<=45 and n<=73
 terms=[0]+[1<<i for i in range(d)];mon=sorted({a|c for a in terms for c in terms});position={v:i for i,v in enumerate(mon)}
 eq=[[0]*n for _ in mon]
 for i,a in enumerate(terms):
  for j,c in enumerate(terms):
   rows=eq[position[a|c]]
   for r,(mask,rhs)in enumerate(coeff[j]):
    while mask:
     bit=mask&-mask;rows[bit.bit_length()-1]^=1<<(i*m+r);mask^=bit
 equations=[v for rows in eq for v in rows];basis,rank=nullspace(equations,m*len(terms));ind={};out=[]
 for packed in basis:
  lambdas=[(packed>>(i*m))&((1<<m)-1)for i in range(len(terms))];identity=[0]*len(mon);rhs=[0]*len(mon)
  for i,a in enumerate(terms):
   for j,c in enumerate(terms):
    q=position[a|c]
    identity[q]^=xor(mask for r,(mask,v)in enumerate(coeff[j])if lambdas[i]>>r&1)
    rhs[q]^=xor(v for r,(mask,v)in enumerate(coeff[j])if lambdas[i]>>r&1)
  assert not any(identity)
  poly=sum(v<<i for i,v in enumerate(rhs));rem=poly
  while rem:
   bit=rem.bit_length()-1
   if bit not in ind:ind[bit]=rem;out.append(dict(polynomial=poly,multipliers=lambdas));break
   rem^=ind[bit]
 return dict(outer_bits=d,rows=m,inner_bits=n,unknowns=m*len(terms),equations=len(equations),coefficient_rank=rank,syzygy_dimension=len(basis),monomials=mon,guards=out,guard_rank=len(out))
def evaluate(poly,mon,t):return xor(1 for i,v in enumerate(mon)if poly>>i&1 and t&v==v)
def candidates(result):return [t for t in range(1<<result['outer_bits'])if all(evaluate(g['polynomial'],result['monomials'],t)==0 for g in result['guards'])]
def controls():
 cases=exact=0
 for code in range(4096):
  vs=[(code>>(3*i))&7 for i in range(4)];tables=[[(v&3,v>>2)for v in vs[:2]],[(v&3,v>>2)for v in vs[2:]]];domains=[(0,[1,2])]
  if code<256:domains.append((2,[1]))
  for p,B in domains:
   c=pencil(tables,p,B);result=guards(c,2);kept=set(candidates(result));actual=set()
   for t,u in enumerate(domain_points(p,B)):
    rows=k.assemble(tables,u)
    if any(all((m&x).bit_count()%2==r for m,r in rows)for x in range(4)):actual.add(t)
   assert actual<=kept;exact+=actual==kept;cases+=1
 return dict(pencils=4096,affine_domains=cases,all_feasible_retained=True,exact_domains=exact)
def actual():
 data=json.loads((R/'research/kernel_class_20261115_v23/raw.json').read_text())['out']['actual']
 previous=json.loads((R/'research/dual_refinement_20261115_v24/raw.json').read_text())['out']['actual'];out=[];summary={}
 for f,old in zip(data['families'],previous['families']):
  assert f['family']==old['family'];p=old['prefix']['parameter0'];B=old['prefix']['parameter_basis'];assert len(B)==4
  result=guards(pencil(f['basis_tables'],p,B),f['parameter_bits']);kept=candidates(result);us=domain_points(p,B);good={r['u']for r in f['records']if r['consistent']}|{0};truth={i for i,u in enumerate(us)if u in good};assert truth<=set(kept)
  result.update(parameter0=p,parameter_basis=B,candidate_parameters=kept,candidate_directions=sorted(us[t]for t in kept),actual_parameters=sorted(truth),exact=set(kept)==truth)
  summary[f['family']]=dict(syzygy_dimension=result['syzygy_dimension'],guard_rank=result['guard_rank'],candidate_count=len(kept),actual_count=len(truth),exact=result['exact'],polynomials=[g['polynomial']for g in result['guards']])
  out.append(dict(family=f['family'],result=result))
 return dict(summary=summary,families=out,scope='Only exposed low6 common-dual fixedpoint; not all kernels, outercharts, norm support, original roots or other fields')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<524288
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Degree-one variable-dependent dual syzygies after constant-dual fixedpoint failure.',evidence=ev,next_actions=['If exact, retain full syzygy identities and test paid cost versus guarded splitting; if incomplete, record certificate-degree obstruction and test rank-conditioned minors rather than repeat the same class. No full root or speed claim.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
