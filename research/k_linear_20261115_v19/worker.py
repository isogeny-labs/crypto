"""Conditional all-k elimination, never a complete source-root search."""
from pathlib import Path
import gzip,hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as norm
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CHECKS=0
def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=300000 and time.monotonic()-START<78,'work cap'
 assert v,why
def pack(H,n,d):
 j=min(3*d-2,n);k=min(4*d-3,n)
 return (H[2]>>j)|((H[1]>>k)<<(n-j))
def system(f,A,z,y,h,r,w,d):
 n=f.degree;m=min(2*d-1,n);j=min(3*d-2,n);q=min(4*d-3,n);s=w^f.mul(z,r)
 a=f.square(h)^h^z;k0=f.square(r)^f.mul(h,r)^f.mul(f.square(h),A)^f.mul(z,a)
 H0=norm.formula(f,A,z,y,h,k0,r,s);check(H0[3]==0,'zero E2 offset')
 c=z^r;e=f.square(z)^f.mul(h,y)^w;columns=[]
 for i in range(m):
  u=1<<i;u2=f.square(u);v3=u2^f.mul(c,u);v4=f.mul(A^z,u2)^f.mul(e,u)
  columns.append((v3>>j)|((v4>>q)<<(n-j)))
 rhs=pack(H0,n,d);width=2*n-j-q
 rows=[(sum(((v>>bit)&1)<<i for i,v in enumerate(columns)),(rhs>>bit)&1)for bit in range(width)]
 sol=norm.solve(rows,m)
 return dict(k0=k0,rows=rows,columns=columns,solution=sol,k_field_bits=n,unknown_bits=m,equations=width,delta=e^f.mul(A^z,c))
def values(sy):
 s=sy['solution'];out=[]
 if s['consistent']:
  for t in range(1<<len(s['basis'])):
   u=s['particular']
   for i,v in enumerate(s['basis']):
    if t>>i&1:u^=v
   out.append(sy['k0']^u)
 return sorted(out)
def naive(a,b,n,mod):
 out=0
 for i in range(n):
  if b>>i&1:out^=a<<i
 while out.bit_length()>n:out^=mod<<(out.bit_length()-n-1)
 return out
def controls():
 calls=0;systems=0;positive=0;h0=0
 for n,mod in[(2,7),(3,11),(4,19)]:
  f=BinaryField(n,mod);q=f.order
  for a in range(q):
   for b in range(q):check(f.mul(a,b)==naive(a,b,n,mod),'independent small field convolution')
  for A in[0,1]:
   for h in(range(q)if n<4 else[0,1]):
    for r in range(q):
     for w in range(q):
      sy=system(f,A,0,1,h,r,w,1);expected=[]
      for k in range(q):
       H=norm.literal(f,A,1,0,1,h,k,r,w)
       if H[3]>>1==0 and pack(H,n,1)==0:expected.append(k)
       calls+=1
      check(values(sy)==expected,'complete literal-norm all-k sets')
      systems+=1;positive+=bool(expected);h0+=h==0
 return dict(systems=systems,literal_k_calls=calls,nonempty=positive,h0_systems=h0,scope='GF4/GF8 allh,r,w,k,A0/1; GF16h0/1 allr,w,k,A0/1',all_sets_equal=True)
def actual():
 data=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=BinaryField(83,int(data['extension_field']['modulus_hex'],16));A=int(data['curve']['a_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400
 cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate'];mask=(1<<83)-1;cases=[]
 for t in[0]+[1<<i for i in range(7)]:
  raw=cert['raw_particular']
  for i,v in enumerate(cert['raw_basis']):
   if t>>i&1:raw^=v
  r=raw&mask;s=raw>>83;cases.append(('planted-relaxation',r,s^f.mul(z,r),0))
 state=0x6b656c696d696e617465
 def word():
  nonlocal state
  state^=(state<<13)&mask;state^=state>>7;state^=(state<<17)&mask;state &=mask
  return state
 random=[(word(),word())for _ in range(16)]
 for r,w in random:cases.append(('random',r,w,None))
 for r,_ in random[:8]:cases.append(('delta-zero',r,f.square(z)^f.mul(h,y)^f.mul(A^z,z^r),None))
 out=[]
 for label,r,w,known in cases:
  sy=system(f,A,z,y,h,r,w,18);sol=sy['solution'];s=w^f.mul(z,r)
  for u in[0,1,1<<34,(1<<35)-1]:
   k=sy['k0']^u;H=norm.literal(f,A,1,z,y,h,k,r,s);base=pack(norm.formula(f,A,z,y,h,sy['k0'],r,s),83,18)
   for i,v in enumerate(sy['columns']):
    if u>>i&1:base^=v
   check(H[3]==u and pack(H,83,18)==base,'actual literal all-k affine map')
  if known is not None:
   u=known^sy['k0'];check(sol['consistent']and u<(1<<35)and all(((a&u).bit_count()&1)==b for a,b in sy['rows']),'actual positive relaxed k retained')
  if label=='delta-zero':check(sy['delta']==0,'delta-zero guard')
  out.append(dict(label=label,r=r,w=w,known_k=known,**sy))
 return dict(cases=out,fixed_h=h,d=18,remaining_outer_r_w_bits=166,scope='all k coefficient-span relaxation for each fixedh/r/w; support/globalroots unclosed')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error,checks=CHECKS),sort_keys=True,separators=(',',':')).encode();assert len(raw)<512*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 cells=out.get('actual',{}).get('cases',[]);summary={}
 for label in['planted-relaxation','random','delta-zero']:
  xs=[c for c in cells if c['label']==label];summary[label]=dict(cases=len(xs),consistent=sum(c['solution']['consistent']for c in xs),ranks=[c['solution']['rank']for c in xs],nullities=[c['solution']['nullity']for c in xs])
 ev=dict(controls=out.get('controls'),profile=summary,error=error,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),ephemeral_scratch=True,external_reviewer=False,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact conditional k elimination:83fieldbits to35unknowns and45binary equations, with positive/singular/delta controls.',evidence=ev,next_actions=['Use rank/compatibility evidence to design guarded r/wcells. k elimination leaves166outerr/wbits, support/centroid/ghost/globalfieldclosure and matched Groebner gate open.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';ev['error']='late report';text=json.dumps(result,separators=(',',':'))
 assert len(text.encode())<8192 and time.monotonic()-START<88;print(text)
if __name__=='__main__':main()
