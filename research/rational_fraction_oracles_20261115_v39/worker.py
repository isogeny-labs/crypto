"""Independent binary-linear oracle for unchanged Euclidean fraction prototype."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.reciprocal_rational_20261115_v38 import prepared as p
P=Path(__file__).resolve().parent;START=time.monotonic()
def division(a,b):
 assert b;q=0
 for j in range(max(0,a.bit_length()-b.bit_length()+1)-1,-1,-1):
  if a>>(j+b.bit_length()-1)&1:q|=1<<j;a^=b<<j
 return q,a
def gcd(a,b):
 while b:a,b=b,division(a,b)[1]
 return a
def kernel(rows,n):
 rows=list(rows);pivot=[];i=0
 for bit in range(n):
  j=next((j for j in range(i,len(rows))if rows[j]>>bit&1),None)
  if j is None:continue
  rows[i],rows[j]=rows[j],rows[i]
  for j in range(len(rows)):
   if j!=i and rows[j]>>bit&1:rows[j]^=rows[i]
  pivot.append(bit);i+=1
 return[(1<<free)^sum(((rows[j]>>free)&1)<<bit for j,bit in enumerate(pivot))for free in range(n)if free not in pivot]
def reference(f,t,d,complete=False):
 A=d-1;B=2*d-2;cols=[f.mul(t,1<<j)>>(A+1)for j in range(B+1)];rows=[sum(((v>>j)&1)<<i for i,v in enumerate(cols))for j in range(f.degree-A-1)];K=kernel(rows,B+1);fractions=set()
 for den in K:
  num=f.mul(t,den);assert num.bit_length()-1<=A;g=gcd(num,den);fractions.add((division(num,g)[0],division(den,g)[0]))
 assert len(fractions)<=1
 if complete:
  truth={D for D in range(1,1<<(B+1))if f.mul(t,D).bit_length()-1<=A};got={0}
  for k in K:got|={v^k for v in list(got)}
  assert got-{0}==truth
 return(None if not K else next(iter(fractions))),len(K)
def check(f,t,d,tiny=False):
 result=p.reconstruct(f.modulus,t,d);want,dim=reference(f,t,d,tiny)
 if want is None:assert result['status']=='no_bounded_fraction'
 else:assert result==dict(status='fraction',numerator=want[0],denominator=want[1])
 return dict(t=t,d=d,result=result,kernel_dim=dim)
def controls():
 records=[];queries=0;bound_refusals=0
 for n,m in[(4,19),(5,37),(6,67),(7,131)]:
  f=BinaryField(n,m)
  for d in range(1,4):
   if 3*d-3>=n:
    try:p.reconstruct(m,1,d)
    except AssertionError:bound_refusals+=1;continue
    raise AssertionError('unsupported degree admitted')
   for t in range(f.order):
    records.append(dict(n=n,modulus=m,**check(f,t,d,True)));queries+=1
 try:BinaryField(4,21)
 except ValueError:invalid_refused=True
 else:raise AssertionError('reducible field admitted')
 return dict(queries=queries,bound_refusals=bound_refusals,invalid_modulus_refused_by_field_backend=invalid_refused,records=records)
def actual():
 data=json.loads((P/'DATA.json').read_text());f=BinaryField(83,data['modulus']);rows=[]
 for case in data['cases']:
  t=f.inv(case['a'])^f.inv(case['b'])if case['kind']=='planted'else case['t'];row=check(f,t,18);row.update(case_id=case['id'],kind=case['kind']);rows.append(row)
  assert time.monotonic()-START<78
 return dict(modulus=f.modulus,d=18,cases=rows,scope='Exact boundedfraction query only; no allpair enumerator/source roots/global search')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<262144
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});cc=out.get('controls',{});ev=dict(error=error,complete_tiny_queries=cc.get('queries'),actual_queries=len(aa.get('cases',[])),bound_refusals=cc.get('bound_refusals'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Unchanged Euclid boundedfraction prototype checked against independent complete binary-linear oracle.',evidence=ev,next_actions=['If correct build exact capped polynomialdivisor pair query and verify complete tiny/full actualpair domains independently; primaryrationalreconstruction novelty review; global2^54triple search remains.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
