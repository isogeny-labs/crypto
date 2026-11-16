"""Compress guarded Gold3 rationality into four additional all-k rows."""
from pathlib import Path
import hashlib,json,resource,sys,time
from research.k_linear_20261115_v19 import worker as b
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def image(f,d):
 piv={};gens=[]
 for i in range(d):
  for j in range(i+1,d):
   v=f.mul(f.square(1<<i),1<<j)^f.mul(1<<i,f.square(1<<j));gens.append(v)
   while v:
    p=v.bit_length()-1
    if p in piv:v^=piv[p]
    else:piv[p]=v;break
 return piv,gens
def residue(v,piv):
 for p in sorted(piv,reverse=True):
  if v>>p&1:v^=piv[p]
 return v
def gold_systems(f,A,z,y,h,r,w,d,piv):
 base=b.system(f,A,z,y,h,r,w,d);a=f.square(h)^h^z;m=base['unknown_bits'];n=f.degree;j=min(3*d-2,n)
 H=b.norm.formula(f,A,z,y,h,base['k0'],r,w^f.mul(z,r));g0=f.mul(a,H[3])^H[2]
 delta=[f.square(1<<i)^f.mul(a^z^r,1<<i)for i in range(m)]
 fullfree=[i for i in range(n)if i not in piv];inside=[i for i in range(j)if i not in piv]
 def rows(free,truncate):
  mask=(1<<j)-1 if truncate else(1<<n)-1
  rhs=residue(g0&mask,piv);cols=[residue(v&mask,piv)for v in delta]
  out=base['rows']+[(sum(((v>>bit)&1)<<i for i,v in enumerate(cols)),(rhs>>bit)&1)for bit in free]
  if a>=(1<<d):out.append((0,1))
  return out
 full=rows(fullfree,False);compact=rows(inside,True)
 fs=b.norm.solve(full,m);cs=b.norm.solve(compact,m)
 b.check(fs['consistent']==cs['consistent'],'guarded Gold consistency equivalence')
 if cs['consistent']:b.check(fs['particular']==cs['particular']and fs['basis']==cs['basis'],'complete guarded Gold affine fibers equivalent')
 if a<(1<<d):b.check(fs['rank']==cs['rank'],'guarded Gold rank equivalence')
 return dict(base=base,full_rows=full,compact_rows=compact,full_solution=fs,compact_solution=cs,additional_rows=len(inside),Gold_image_rank=len(piv))
def oracle(f,A,z,y,h,r,w,d,k,span):
 H=b.norm.literal(f,A,1,z,y,h,k,r,w^f.mul(z,r));g=f.mul(H[4],H[3])^H[2]
 return H[4]<(1<<d)and H[3]<(1<<min(2*d-1,f.degree))and b.pack(H,f.degree,d)==0 and g in span
def controls():
 cases=0;calls=0
 for n,mod,d in[(2,7,1),(3,11,1),(4,19,1),(5,37,2)]:
  f=b.BinaryField(n,mod);q=f.order;piv,gens=image(f,d)
  span={0}
  for v in gens:span|={x^v for x in list(span)}
  hs=range(q)if n<4 else[0,1];As=[0,1]if n<5 else[0];ws=range(q)if n<5 else range(16)
  for A in As:
   for h in hs:
    for r in range(q):
     for w in ws:
      sy=gold_systems(f,A,0,1,h,r,w,d,piv);sol=sy['compact_solution'];actual=[]
      if sol['consistent']:
       for t in range(1<<len(sol['basis'])):
        u=sol['particular']
        for i,v in enumerate(sol['basis']):
         if t>>i&1:u^=v
        actual.append(sy['base']['k0']^u)
      expected=[k for k in range(q)if oracle(f,A,0,1,h,r,w,d,k,span)];calls+=q
      b.check(sorted(actual)==expected,'complete independent literal Gold all-k sets');cases+=1
 return dict(systems=cases,literal_norm_k_calls=calls,fields=[4,8,16,32],full_k_sets_equal=True,centroid_rejection_included=True)
def actual_profile():
 data=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=b.BinaryField(83,int(data['extension_field']['modulus_hex'],16));A=int(data['curve']['a_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;piv,gens=image(f,18)
 b.check(len(piv)==48,'actual Gold3 image rank48')
 atlas=json.loads((R/'research/kernel_incidence_20261115_v20/raw.json').read_text())['out']['profile'];points=[(r['u'],p)for r in atlas['records']if r['family']=='Gold69'for p in r['verified_points']];records=[];before={};after={};removed=kept=0
 for u,p in points:
  r,w=p['r'],p['w'];sy=gold_systems(f,A,z,y,h,r,w,18,piv);bs=sy['base']['solution'];cs=sy['compact_solution'];b.check(sy['additional_rows']==4 and len(sy['compact_rows'])==49 and len(sy['full_rows'])==80,'exact guarded row compression80to49')
  v0=sy['base']['k0'];b.check(cs['consistent']and v0<(1<<35)and all((m&v0).bit_count()%2==v for m,v in sy['compact_rows']),'known Gold k0 retained')
  b.check(0<=cs['rank']-bs['rank']<=4,'atmost4new Gold independent equations')
  literal=b.norm.literal(f,A,1,z,y,h,u,r,w^f.mul(z,r));g=f.mul(literal[4],literal[3])^literal[2];expected=residue(g,piv)==0;test=u^sy['base']['k0'];pred=all((m&test).bit_count()%2==v for m,v in sy['compact_rows'])
  b.check(pred==expected,'actual literal positive-or-rejected k=u Gold predicate')
  if pred:kept+=1
  else:removed+=1
  before[str(bs['nullity'])]=before.get(str(bs['nullity']),0)+1;after[str(cs['nullity'])]=after.get(str(cs['nullity']),0)+1
  record=dict(u=u,r=r,w=w,before_nullity=bs['nullity'],after_nullity=cs['nullity'],k0_retained=True,k_u_retained=pred)
  if len(records)<4:record['full_systems']=sy
  records.append(record)
 return dict(points=len(points),image_rank=48,Gold_codimension_in_W3=4,uncompressed_rows=80,compressed_rows=49,before_nullities=before,after_nullities=after,k_u_kept=kept,k_u_rejected=removed,records=records)
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual_profile()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<512*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual={k:v for k,v in out.get('actual',{}).items()if k!='records'},worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact guarded Gold3 compression80to49all-k equations; complete affine fiber equivalence and positive singular controls.',evidence=ev,next_actions=['Integrate four guarded Gold equations with every conditional kfiber; preserve any remaining kernel and source-support/outer166bit/globalfield obligations. Then test full support on retained candidates, not merely linear membership.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
