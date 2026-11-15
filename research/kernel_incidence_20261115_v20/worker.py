"""Rank-kernel incidence atlas, comparing pre-Gold73 and Gold69 families."""
from pathlib import Path
import gzip,hashlib,json,resource,sys,time
from research.k_linear_20261115_v19 import worker as b
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def condition(f,A,z,y,h,u,d,r,w):
 n=f.degree;j=min(3*d-2,n);q=min(4*d-3,n);u2=f.square(u)
 c=u2^f.mul(z^r,u);e=f.mul(A^z,u2)^f.mul(f.square(z)^f.mul(h,y)^w,u)
 return (c>>j)|((e>>q)<<(n-j))
def intersection(f,A,z,y,h,u,d,raw0,directions):
 n=f.degree;mask=(1<<n)-1
 def rw(raw):
  r=raw&mask;return r,(raw>>n)^f.mul(z,r)
 r0,w0=rw(raw0);rhs=condition(f,A,z,y,h,u,d,r0,w0);constant=condition(f,A,z,y,h,u,d,0,0)
 columns=[condition(f,A,z,y,h,u,d,*rw(raw))^constant for raw in directions]
 width=2*n-min(3*d-2,n)-min(4*d-3,n)
 rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(columns)),(rhs>>j)&1)for j in range(width)]
 return dict(rows=rows,solution=b.norm.solve(rows,len(directions)))
def affine_values(sol):
 if not sol['consistent']:return []
 out=[]
 for t in range(1<<len(sol['basis'])):
  x=sol['particular']
  for i,v in enumerate(sol['basis']):
   if t>>i&1:x^=v
  out.append(x)
 return sorted(out)
def controls():
 base=b.controls();calls=0;systems=0
 for n,mod,d,us in[(3,11,1,[1]),(4,19,1,[1]),(5,37,2,list(range(1,8)))]:
  f=b.BinaryField(n,mod);q=1<<n
  for a in range(q):
   for c in range(q):b.check(f.mul(a,c)==b.naive(a,c,n,mod),'independent GF32-inclusive arithmetic')
  for A in[0,1]:
   for h in[0,1]:
    for u in us:
     sy=intersection(f,A,0,1,h,u,d,0,[1<<i for i in range(2*n)]);expected=[]
     for raw in range(1<<(2*n)):
      r=raw&(q-1);s=raw>>n;H0=b.norm.literal(f,A,1,0,1,h,0,r,s);H1=b.norm.literal(f,A,1,0,1,h,u,r,s)
      diff=[a^c for a,c in zip(H0,H1)]
      if b.pack(diff,n,d)==0:expected.append(raw)
      calls+=2
     b.check(affine_values(sy['solution'])==expected,'complete literal kernel-guard parameter sets')
     systems+=1
 return dict(base_all_k=base,kernel_guard_systems=systems,literal_difference_calls=calls,complete_kernel_guard_sets_equal=True)
def profile():
 data=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=b.BinaryField(83,int(data['extension_field']['modulus_hex'],16));A=int(data['curve']['a_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;mask=(1<<83)-1
 old=json.loads(gzip.decompress((R/'research/full_quintic_block_20261115_v9/raw.json.gz').read_bytes()))['out']['profile']['proofs']['baseline166']
 gold=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate']
 families=[('pre-Gold73',old['particular'],old['basis']),('Gold69',gold['raw_particular'],gold['raw_basis'])]
 us=[1<<i for i in range(35)];state=0x6b65726e656c61746c6173
 while len(us)<64:
  state^=(state<<13)&((1<<64)-1);state^=state>>7;state^=(state<<17)&((1<<64)-1);state &=(1<<64)-1
  u=state&((1<<35)-1)
  if u and u not in us:us.append(u)
 records=[];summary={}
 for name,raw0,basis in families:
  consistent=0;positives=0;ranks={};nullities={};verified=0
  for u in us:
   sy=intersection(f,A,z,y,h,u,18,raw0,basis);sol=sy['solution'];record=dict(family=name,u=u,intersection=sy,verified_points=[])
   ranks[str(sol['rank'])]=ranks.get(str(sol['rank']),0)+1
   if sol['consistent']:
    consistent+=1;nullities[str(sol['nullity'])]=nullities.get(str(sol['nullity']),0)+1
    params=[sol['particular']]+[sol['particular']^v for v in sol['basis'][:3]]
    for t in params:
     raw=raw0
     for i,v in enumerate(basis):
      if t>>i&1:raw^=v
     r=raw&mask;s=raw>>83;w=s^f.mul(z,r)
     b.check(condition(f,A,z,y,h,u,18,r,w)==0,'actual complete kernel-incidence guard')
     ks=b.system(f,A,z,y,h,r,w,18);k_sol=ks['solution']
     for k in[0,u]:
      H=b.norm.literal(f,A,1,z,y,h,k,r,s);b.check(H[3]>>35==0 and b.pack(H,83,18)==0,'actual literal positive singular norm-span pair')
      p=k^ks['k0'];b.check(p<(1<<35)and all((m&p).bit_count()%2==v for m,v in ks['rows']),'all-k solver retains both positive k values')
     b.check(k_sol['consistent']and k_sol['rank']<35,'proved actual singular conditional k system')
     point=dict(parameter=t,r=r,w=w,k_rank=k_sol['rank'],k_nullity=k_sol['nullity'],positive_k=[0,u])
     if verified==0:point['full_k_system']=ks
     record['verified_points'].append(point);verified+=1;positives+=2
   records.append(record)
  summary[name]=dict(directions=len(us),consistent=consistent,inconsistent=len(us)-consistent,intersection_rank_histogram=ranks,consistent_affine_bits_histogram=nullities,verified_points=verified,positive_relaxed_k_witnesses=positives)
 return dict(directions=us,summary=summary,records=records,scope='Exact conditional rank-kernel incidence in coefficient-span families; no supported-source root or global completion.')
def main():
 out={};error=None
 try:out['controls']=controls();out['profile']=profile()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1024*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),profile=out.get('profile',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact kernel-incidence atlas before/after Gold3; complete tiny literal norm-difference oracle and checked actual singular witnesses.',evidence=ev,next_actions=['Use explicit consistency/obstruction row witnesses to prove whole kernel-direction classes. A sampled union is not all2^35nonzerodirections; retain all kernel branches and source-support/globalfield obligations.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';ev['error']='late report';text=json.dumps(result,separators=(',',':'))
 assert len(text.encode())<8192 and time.monotonic()-START<88;print(text)
if __name__=='__main__':main()
