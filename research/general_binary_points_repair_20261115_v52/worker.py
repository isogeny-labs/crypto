from pathlib import Path
import itertools,json,hashlib,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.general_binary_points_20261115_v52 import solver as s
P=Path(__file__).resolve().parent;START=time.monotonic()

def run():
 systems=states=rootcells=zero=poles=ghost=consistent=0;profiles=[]
 for n,mod,A in [(3,11,0),(3,11,1),(4,19,0),(4,19,8)]:
  f=BinaryField(n,mod);q=f.order;z=4;yT=0;B=f.mul(f.square(z),z)^f.mul(A,f.square(z));assert B
  ys={x:[y for y in range(q)if f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^B]for x in range(4)}
  count=0
  for h in range(q):
   for k in sorted({0,1,h,f.mul(h,2),f.mul(h,3)}):
    base,cols,sol=s.model(f,A,B,z,yT,h,k,2);literal=[]
    for r,ss in itertools.product(range(q),repeat=2):
     H=s.coefficients(f,A,B,z,yT,h,k,r,ss);model=[a^s.norm.linear.xor(c[j]for i,c in enumerate(cols)if(r|(ss<<n))>>i&1)for j,a in enumerate(base)];assert tuple(model)==H;states+=1
     if s.norm.pack(f,H,2)[0]==0:literal.append(r|(ss<<n))
    got=[]if sol is None else[sol['particular']^s.norm.linear.apply(sol['basis'],v)for v in range(1<<len(sol['basis']))];assert sorted(got)==sorted(literal);systems+=1;count+=1
    if sol is None:continue
    consistent+=1;S=s.c.split(f,sol)
    for x in range(4):
     want=set()
     for bits in literal:
      H=s.coefficients(f,A,B,z,yT,h,k,bits&(q-1),bits>>n)
      if s.c.old.evaluate(f,H,x)==0:want.add(bits)
     G=f.mul(h,x)^k;lifts=[None]if not G else ys[x];projected=set()
     for y in lifts:
      res=s.point(f,z,yT,h,k,S,x,y)
      if res:
       for v in range(1<<len(res['basis'])):projected.add(sol['particular']^s.norm.linear.apply(sol['basis'],res['particular']^s.norm.linear.apply(res['basis'],v)))
     assert projected==want;rootcells+=1;zero+=x==0;poles+=G==0;ghost+=G==0 and not ys[x]
    assert time.monotonic()-START<45
  profiles.append(dict(n=n,modulus=mod,A=A,B=B,z=z,target_y=yT,systems=count))
 return dict(affine_systems=systems,literal_r_s_states=states,consistent_systems=consistent,complete_root_cells=rootcells,zero_coordinate_cells=zero,G_zero_cells=poles,nonrational_lift_pole_cells=ghost,profiles=profiles)

def main():
 out={};error=None
 try:out=run()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=50:error='late serialized output'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Unnormalized original-coordinate binary norm fibers and complete zero/pole point-root oracles.',evidence=dict(error=error,controls=out,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['If passed, port original-coordinate generalh/k/curve input to native backend and construct positive actual-curve conditional charts; no normalized trace restriction or ghost omission.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<54;print(text)
if __name__=='__main__':main()
