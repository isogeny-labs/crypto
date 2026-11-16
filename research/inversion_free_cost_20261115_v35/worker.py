"""Cold paid component comparison, original input coordinates, exact full output."""
from pathlib import Path
import hashlib,json,resource,statistics,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.inversion_free_full_20261115_v35 import compiler as c
from research.inversion_free_full_20261115_v35.worker import literal
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def cases():
 mask=(1<<83)-1;out=[]
 for i in range(4096):
  vals=[int.from_bytes(hashlib.sha256(b'norm-paid-v35'+i.to_bytes(4,'little')+bytes([j])).digest(),'little')&mask for j in range(4)]
  if i%16==0:vals[0]=0
  if i%16==1:vals[1]=0
  out.append(vals)
 return json.loads(json.dumps(out))
def constants(f,z,y):
 z2=f.square(z);z4=f.square(z2)
 return dict(z=z,y=y,z2=z2,z4=z4,z5=f.mul(z4,z),y2=f.square(y),C5=y^z2)
def strong(f,t,h,k,r,s):
 z=t['z'];h2=f.square(h);k2=f.square(k);r2=f.square(r);w=s^f.mul(z,r);w2=f.square(w);hy2=f.mul(h2,t['y2']);e1=h2^h^z
 e2=r2^f.mul(h,r)^k^f.mul(z,e1)
 e3=f.mul(h,s)^f.mul(k,r)^k2^f.mul(z,e2)
 e4=w2^f.mul(k,w)^f.mul(z,k2)^f.mul(t['z2'],k)^hy2^f.mul(f.mul(h,k),t['y'])^t['z4']
 e5=f.mul(z,w2)^f.mul(t['C5'],k2)^f.mul(z,hy2)^t['z5']
 return[e5,e4,e3,e2,e1,1]
def normalized(f,t,h,k,r,s):
 z=t['z'];u=r^z^f.mul(t['c'],h);v=s^f.mul(z,r)^f.square(z)^f.mul(t['y'],h)^f.mul(t['c'],k)
 return c.norm_coefficients(f,t,h,k,u,v)
def child(method):
 begin=time.monotonic();data=cases();f=BinaryField(83,0x800000000200000000007);z=0x622425e96ba633969263b;y=0x507c02b90217be9ec635f
 ctx=c.compile_context(f,0,z,y)if method=='normalized'else constants(f,z,y);fun=normalized if method=='normalized'else strong
 # Every direct baseline formula is checked before collecting its timed core.
 for h,k,r,s in data[:32]:assert fun(f,ctx,h,k,r,s)==literal(f,0,1,z,y,h,k,r,s)
 rows=[];core=verify=0
 for h,k,r,s in data:
  t=time.monotonic();H=fun(f,ctx,h,k,r,s);core+=time.monotonic()-t
  t=time.monotonic();assert H==literal(f,0,1,z,y,h,k,r,s);verify+=time.monotonic()-t;rows.append(H)
 blob=json.dumps(rows,separators=(',',':')).encode()
 print(json.dumps(dict(method=method,count=len(rows),output_sha256=hashlib.sha256(blob).hexdigest(),output_bytes=len(blob),core_seconds=core,verification_seconds=verify,pre_stdout_seconds=time.monotonic()-begin,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss),separators=(',',':')))
def main():
 out=dict(records=[]);error=None
 try:
  for rep in range(3):
   for method in(['strong','normalized']if rep%2==0 else['normalized','strong']):
    t=time.monotonic();r=subprocess.run([sys.executable,'-m','research.inversion_free_cost_20261115_v35.worker',method],cwd=R,capture_output=True,text=True,timeout=12);elapsed=time.monotonic()-t
    assert r.returncode==0,(r.returncode,r.stderr[-500:]);row=json.loads(r.stdout);row.update(repetition=rep,cold_wall_seconds=elapsed);out['records'].append(row)
   assert time.monotonic()-START<78
  assert len({x['output_sha256']for x in out['records']})==1
  for key in['cold_wall_seconds','core_seconds']:
   out['median_'+key]={m:statistics.median(x[key]for x in out['records']if x['method']==m)for m in['strong','normalized']}
  out['AA_spread']={m:max(x['cold_wall_seconds']for x in out['records']if x['method']==m)/min(x['cold_wall_seconds']for x in out['records']if x['method']==m)-1 for m in['strong','normalized']}
  out['paid_component_ratio']=out['median_cold_wall_seconds']['strong']/out['median_cold_wall_seconds']['normalized'];out['core_ratio']=out['median_core_seconds']['strong']/out['median_core_seconds']['normalized']
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(out=out,error=error,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exploratory paid compiler comparison; no Groebner or original allroots performance claim.',evidence=ev,next_actions=['If no total gain reuse normalized E4block across whole cells rather than optimize isolated scalar evaluation. Freeze exact block oracle with all h/k including rankdrops; no identical noisy retry.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<16384 and time.monotonic()-START<84;print(text)
if __name__=='__main__':
 if len(sys.argv)>1 and sys.argv[1]in['strong','normalized']:child(sys.argv[1])
 else:main()
