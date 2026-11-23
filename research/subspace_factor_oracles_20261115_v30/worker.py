"""Independent first execution of multiplicity-preserving subspace extraction."""
from pathlib import Path
import json,itertools,hashlib,resource,sys,time,collections
from rootfinder.binaryfield import BinaryField
from research.generic_block_oracles_20261115_v28 import worker as o
from research.subspace_factor_20261115_v29 import factor as a
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CURRENT={};CASES=0
class Binary:
 def __init__(self,n,mod):self.field=BinaryField(n,mod);self.p=2;self.degree=n;self.order=1<<n;self.modulus=mod
 def add(self,x,y):return x^y
 def sub(self,x,y):return x^y
 def neg(self,x):return x
 def mul(self,x,y):return self.field._raw_mul(x,y)
 def inv(self,x):return self.field.inv(x)
def mul(F,x,y):
 if F.p!=2:return o.oracle(F,x,y)
 v=0
 for i in range(F.degree):
  if y>>i&1:v^=x<<i
 while v.bit_length()>F.degree:v^=F.modulus<<(v.bit_length()-F.degree-1)
 return v
def add(F,x,y):return x^y if F.p==2 else o.add(F,x,y)
def negative(F,x):return x if F.p==2 else sum((-(x//F.p**i%F.p)%F.p)*F.p**i for i in range(F.degree))
def evaluate(F,H,x):
 v=0
 for c in reversed(H):v=add(F,mul(F,v,x),c)
 return v
def product(F,A,B):
 out=[0]*(len(A)+len(B)-1)
 for i,x in enumerate(A):
  for j,y in enumerate(B):out[i+j]=add(F,out[i+j],mul(F,x,y))
 return out
def from_roots(F,roots):
 H=[1]
 for x in roots:H=product(F,H,[negative(F,x),1])
 return H
def domain(F,B,off):
 out=[]
 for cs in itertools.product(range(F.p),repeat=len(B)):
  x=off
  for c,v in zip(cs,B):x=add(F,x,mul(F,c,v))
  out.append(x)
 assert len(set(out))==len(out);return sorted(out)
def oracle(F,H,D):
 work=H[:];roots=[]
 for x in D:
  count=0
  while len(work)>1 and evaluate(F,work,x)==0:
   Q=[0]*(len(work)-1);Q[-1]=work[-1]
   for j in range(len(Q)-2,-1,-1):Q[j]=add(F,work[j+1],mul(F,x,Q[j+1]))
   assert add(F,work[0],mul(F,x,Q[0]))==0;work=Q;count+=1
  if count:roots.append((x,count))
 return (len(work)==1,roots)
def check(F,H,B,off,want):
 global CURRENT,CASES
 CURRENT=dict(p=F.p,degree=F.degree,H=H,basis=B,offset=off);result=a.supported_roots(F,H,B,off);supported,roots=want
 assert result['supported']==supported
 if supported:
  assert result['status']=='complete_supported_factorization'and result['roots']==roots
  assert result['nodes']<=1+(len(H)-1)*len(B)
 else:assert result['status']=='not_fully_supported'and result['roots']is None
 CASES+=1;assert CASES<=10000
 return result
def controls(out):
 profiles=[('GF4full',Binary(2,7),[1,2],0,3),('GF8prefix',Binary(3,11),[1,2],0,3),('GF8coset',Binary(3,11),[1,2],4,3),('F3',o.g.Field(3),[1],0,3),('F3point',o.g.Field(3),[],1,3),('F5',o.g.Field(5),[1],0,3),('GF9prefix',o.g.Field(3,2),[1],0,3),('GF9full',o.g.Field(3,2),[1,3],0,3),('GF25full',o.g.Field(5,2),[1,5],0,2)]
 for label,F,B,off,maxdegree in profiles:
  D=domain(F,B,off);digest=hashlib.sha256();count=accepted=maximum=0;witness=[]
  polynomials=([*cs,1]for n in range(maxdegree+1)for cs in itertools.product(range(F.order),repeat=n))
  for H in itertools.chain(polynomials,[from_roots(F,[D[0]]*4),from_roots(F,[D[-1]]*5)]):
   want=oracle(F,H,D);result=check(F,H,B,off,want);row=dict(H=H,supported=want[0],roots=want[1]if want[0]else None);digest.update(json.dumps(row,sort_keys=True,separators=(',',':')).encode()+b'\n');count+=1;accepted+=want[0];maximum=max(maximum,result.get('nodes',0))
   if len(witness)<8 or len(H)>maxdegree+1:witness.append(dict(**row,result=result))
  out.append(dict(field=label,p=F.p,degree=F.degree,modulus=getattr(F,'modulus',None),nu=getattr(F,'nu',None),basis=B,offset=off,maxdegree=maxdegree,cases=count,accepted=accepted,rejected=count-accepted,max_nodes=maximum,transcript_sha256=digest.hexdigest(),witnesses=witness))
def actual():
 F=Binary(83,0x800000000200000000007);B=[1<<i for i in range(18)];mask=(1<<18)-1;state=0x30fac70;records=[]
 def word():
  nonlocal state
  state=(state*6364136223846793005+1442695040888963407)&((1<<64)-1)
  return state&mask
 for i in range(32):
  roots=[0]*5 if i==0 else[word()for _ in range(5)]
  if i%4==1:roots[1:]=[roots[0]]*4
  if i%4==2:roots[1]=roots[0];roots[3]=roots[2]
  H=from_roots(F,roots);want=sorted(collections.Counter(roots).items());result=check(F,H,B,0,(True,want));records.append(dict(kind='planted_supported',H=H,planted_roots=roots,result=result))
  if i<16:
   bad=roots[:];bad[-1]=1<<18;H=from_roots(F,bad);result=check(F,H,B,0,(False,[]));records.append(dict(kind='planted_outside',H=H,planted_roots=bad,result=result))
 return dict(records=records,supported=32,rejected=16,field_degree=83,modulus=F.modulus,basis=B,domain_size=1<<18,maximum_nodes=max(x['result'].get('nodes',0)for x in records),gcd_calls=sum(x['result'].get('gcd_calls',0)for x in records),scope='Givenplantedquintics, not normsearch ororiginalroot solve')
def refusals():
 F=o.g.Field(3)
 try:a.supported_roots(F,[0,1],[1,1])
 except AssertionError:dependent=True
 else:raise AssertionError('dependentbasisaccepted')
 refused=a.supported_roots(o.g.Field(37),[0,1],[1]);assert refused['status']=='unsupported_characteristic'and refused['supported']is None and refused['roots']is None
 return dict(dependent_basis_refused=dependent,large_characteristic_unresolved=True)
def main():
 out=dict(controls=[]);error=None
 try:
  rules=json.loads((P/'INPUT_RULES.json').read_text());controls(out['controls']);out['actual']=actual();out['refusals']=refusals();assert CASES==rules['expected_cases']
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['failed_case']=CURRENT
 out['cases']=CASES;raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<1048576
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=[{k:v for k,v in x.items()if k!='witnesses'}for x in out['controls']],actual={k:v for k,v in out.get('actual',{}).items()if k not in['records','basis']},refusals=out.get('refusals'),cases=CASES,worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='First conditional all-root/multiplicity extraction from supported implicit-subspace polynomials.',evidence=ev,next_actions=['Ifpasses compare complete paid support+factorization against existing trace/Frobenius factorizer with reconstructedmultiplicities; outerHsearch, sourceghostcoverage andGBgate remainopen. Failure requiresfreshvariant.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
