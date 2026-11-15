"""First independent odd-characteristic norm-coordinate compiler validation."""
from pathlib import Path
import json,itertools,hashlib,resource,sys,time
from research.generic_block_oracles_20261115_v28 import worker as o
from research.odd_norm_20261115_v28 import compiler as c
g=o.g;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CURRENT={};CASES=0
def neg(F,a):return sum((-(a//(F.p**i)%F.p)%F.p)*(F.p**i)for i in range(F.degree))
def total(F,vs):
 out=0
 for v in vs:out=o.add(F,out,v)
 return out
def expected(F,a,b,z,y,h,k,r,s):
 mul=lambda x,y:o.oracle(F,x,y);sq=lambda x:mul(x,x);sub=lambda x,y:o.add(F,x,neg(F,y));two=2
 t=neg(F,total(F,[mul(sq(z),z),mul(r,sq(z)),mul(s,z),mul(o.add(F,mul(h,z),k),y)]))
 N=[sub(sq(t),mul(b,sq(k))),sub(sub(mul(two,mul(s,t)),mul(a,sq(k))),mul(two,mul(b,mul(h,k)))),sub(sub(o.add(F,sq(s),mul(two,mul(r,t))),mul(two,mul(a,mul(h,k)))),mul(b,sq(h))),sub(sub(o.add(F,mul(two,t),mul(two,mul(r,s))),sq(k)),mul(a,sq(h))),sub(o.add(F,sq(r),mul(two,s)),mul(two,mul(h,k))),sub(mul(two,r),sq(h)),1]
 work=N[:];H=[0]*6
 for i in range(6,0,-1):H[i-1]=work[i];work[i-1]=o.add(F,work[i-1],mul(z,work[i]));work[i]=0
 assert work[0]==0 and H[5]==1
 return dict(r=r,s=s,t=t,H=H)
def case(F,label,a,b,z,y,h,k,r,s):
 global CURRENT,CASES
 CURRENT=dict(field=label,a=a,b=b,z=z,y=y,h=h,k=k,r=r,s=s);want=expected(F,a,b,z,y,h,k,r,s);e1=neg(F,want['H'][4]);e2=want['H'][3];got=c.compile_moments(F,a,b,z,y,h,k,e1,e2)
 assert got==want;CASES+=1;assert CASES<=100000
 return dict(h=h,k=k,r=r,s=s,e1=e1,e2=e2,t=got['t'],H=got['H'])
def refuse(F,a,b,z):
 try:c.compile_moments(F,a,b,z,0,0,0,0,0)
 except AssertionError:return True
 raise AssertionError('invalid target accepted')
def small(out):
 for p,degree,label in[(3,1,'F3'),(5,1,'F5'),(3,2,'GF9'),(5,2,'GF25')]:
  F=g.Field(p,degree,2);q=F.order;points=itertools.product(range(q),repeat=4)if q<=9 else samples(q);digest=hashlib.sha256();buckets={};witness=[];count=0;zero_h=zero_k=0
  for index,(h,k,r,s)in enumerate(points):
   record=case(F,label,1,1,0,1,h,k,r,s);digest.update(json.dumps(record,sort_keys=True,separators=(',',':')).encode()+b'\n');count+=1;zero_h+=h==0;zero_k+=k==0
   if q<=9:buckets.setdefault((h,k),set()).add((record['e1'],record['e2']))
   if index<4 or index in[q*q-1,q**3-1,q**4-1]or((h==0 or k==0)and r==s==0):
    if len(witness)<48:witness.append(record)
  if q<=9:assert len(buckets)==q*q and all(len(v)==q*q for v in buckets.values())
  assert c.compile_moments(F,1,1,0,1,0,0,0,0)['H']==[0,0,0,0,0,1]
  out.append(dict(field=label,p=p,degree=degree,nu=2,cases=count,complete_parameter_domain=q<=9,bijection_per_hk=q<=9,h0_cases=zero_h,k0_cases=zero_k,transcript_sha256=digest.hexdigest(),witnesses=witness,quintuple_zero_retained=True,invalid_target_rejected=refuse(F,1,1,0)))
def samples(q):
 yield 0,0,0,0
 state=0x29a55
 for i in range(255):
  values=[]
  for j in range(4):state=(state*6364136223846793005+1442695040888963407)&((1<<64)-1);values.append(state%q)
  if i%4==0:values[0]=0
  if i%4==1:values[1]=0
  yield tuple(values)
def wide(out):
 for preset in json.loads((R/'lab/research/nist_presets.json').read_text())['curves']:
  p=int(preset['p_hex'],16);F=g.Field(p);a=int(preset['a_hex'],16);b=int(preset['b_hex'],16);z=int(preset['gx_hex'],16);y=int(preset['gy_hex'],16);assert (y*y-z**3-a*z-b)%p==0
  records=[]
  for i in range(32):
   h=0 if i%4==0 else(p-7-i*97)%p;k=0 if i==0 or i%4==1 else((1<<(p.bit_length()-1))+i*65537)%p;r=0 if i==0 else(p-3-i*31)%p;s=0 if i==0 else((1<<(p.bit_length()-2))+i)%p
   records.append(case(F,preset['name'],a,b,z,y,h,k,r,s))
  out.append(dict(field=preset['name'],prime=p,a=a,b=b,z=z,y=y,cases=len(records),records=records,invalid_target_rejected=refuse(F,a,b,z),scope='norm-coordinate compiler only; no originalsource roots or curve/order proof'))
def main():
 out=dict(small=[],wide=[]);error=None
 try:
  small(out['small']);wide(out['wide'])
  class Two:
   def add(self,a,b):return a^b
  try:c.compile_moments(Two(),0,0,0,0,0,0,0,0)
  except AssertionError:out['characteristic_two_rejected']=True
  else:raise AssertionError('characteristic2accepted')
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['failed_case']=CURRENT
 out['cases']=CASES;raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1048576
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,small=[{k:v for k,v in x.items()if k!='witnesses'}for x in out['small']],wide=[dict(field=x['field'],cases=x['cases'])for x in out['wide']],cases=CASES,characteristic_two_rejected=out.get('characteristic_two_rejected'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Independent oddnormcompiler coordinate bijection with zero/repeated/char3 and NISTcurve targets.',evidence=ev,next_actions=['If passed integrate with separatelyproved support/intervalsolver andallsourceghostchartcoverage; no arbitraryfieldh/k enumeration or matchedspeedclaim. A failure requiresfreshrepair andretainedcounterexample.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
