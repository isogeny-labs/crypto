"""Positive singular k fibers from a complete affine intersection."""
from pathlib import Path
import gzip,hashlib,json,resource,sys,time
from research.k_linear_20261115_v19 import worker as b
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def run():
 controls=b.controls()
 data=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=b.BinaryField(83,int(data['extension_field']['modulus_hex'],16));A=int(data['curve']['a_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;mask=(1<<83)-1
 cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate'];raw0=cert['raw_particular'];directions=cert['raw_basis']
 def rw(raw):
  r=raw&mask;return r,(raw>>83)^f.mul(z,r)
 def condition(r,w):return ((r^z^1)>>52)|(((w^f.square(z)^f.mul(h,y)^A^z)>>69)<<31)
 r0,w0=rw(raw0);rhs=condition(r0,w0);cols=[]
 for raw in directions:
  r,w=rw(raw);cols.append((r>>52)|((w>>69)<<31))
 rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(cols)),(rhs>>j)&1)for j in range(45)]
 intersection=b.norm.solve(rows,69);cases=[]
 if intersection['consistent']:
  params=[intersection['particular']]+[intersection['particular']^v for v in intersection['basis'][:7]]
  for t in params:
   raw=raw0
   for i,v in enumerate(directions):
    if t>>i&1:raw^=v
   r,w=rw(raw);b.check(condition(r,w)==0,'complete singular affine intersection')
   sy=b.system(f,A,z,y,h,r,w,18);sol=sy['solution']
   b.check(all(not(m&1).bit_count()%2 for m,v in sy['rows']),'kernel direction1 retained')
   for k in[0,1]:
    u=k^sy['k0'];b.check(sol['consistent']and u<(1<<35)and all(((m&u).bit_count()&1)==v for m,v in sy['rows']),'positive singular k0/k1 retained')
   b.check(sol['rank']<35,'singular actual chart counterexample')
   cases.append(dict(parameter=t,r=r,w=w,conditional=sy))
 return dict(controls=controls,intersection=intersection,intersection_rows=rows,covered_affine_family_parameters=(1<<intersection['nullity'])if intersection['consistent']else 0,cases=cases)
def main():
 out={};error=None
 try:out=run()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<256*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,intersection_consistent=out.get('intersection',{}).get('consistent'),intersection_rank=out.get('intersection',{}).get('rank'),affine_family_bits=out.get('intersection',{}).get('nullity'),actual_ranks=[c['conditional']['solution']['rank']for c in out.get('cases',[])],positive_k=[0,1],worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,external_reviewer=False)
 text=json.dumps(dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Construct exact positive singular coefficient-span k fibers; every null direction retained.',evidence=ev,next_actions=['Quotient known kernel1 on its certified affine r/wbranch while keeping further rank drops; intersect full support and global guards separately.']),separators=(',',':'))
 assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
