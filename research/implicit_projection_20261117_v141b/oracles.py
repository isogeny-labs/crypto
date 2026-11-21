import time
START=time.monotonic()
from pathlib import Path
import json,itertools,random
from .common import P,finish
from .kernel import floor_sum,PrimeProjection,SubspaceProjection,ProjectionBudget,solve_prime_boxes
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
out={'floor_sum_oracles':0,'prime_witness_oracles':0,'prime_member_oracles':0,'prime_root_oracles':0,'contexts':[],'nist':[],'binary83':None,'refusals':0,'all_registered_contexts_complete':False,'original_complete':False};error=None

def span(f,offset,B):
 values=[offset]
 for b in B:values=[f.add(a,f.mul(c,b))for a in values for c in range(f.p)]
 return set(values)
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed'])
 for n in range(data['floor_n_max']+1):
  for m in range(1,data['floor_modulus_max']+1):
   for a in range(-data['floor_signed_bound'],data['floor_signed_bound']+1):
    for b in range(-data['floor_signed_bound'],data['floor_signed_bound']+1):assert floor_sum(n,m,a,b)[0]==sum((a*i+b)//m for i in range(n));out['floor_sum_oracles']+=1
 for p in data['small_primes']:
  intervals=[(a,b)for a in range(p)for b in range(a,p)]
  for jbox in intervals:
   for pbox in intervals:
    for r in range(p):
     S=PrimeProjection(p,jbox,pbox,r);expected={(a-r*b)%p for a in range(jbox[0],jbox[1]+1)for b in range(pbox[0],pbox[1]+1)}
     for x in range(p):assert S.witness_count(x)==sum(jbox[0]<=(x+r*b)%p<=jbox[1]for b in range(pbox[0],pbox[1]+1));out['prime_witness_oracles']+=1
     assert set(S.members())==expected;assert S.cardinality is None or S.cardinality==len(expected);out['prime_member_oracles']+=1
  for boxes in itertools.product(intervals,repeat=2):
   c=lambda z:(sum(z)-1)%p;k=2%p
   if not k:continue
   got=solve_prime_boxes(p,boxes,[1,1],k,0,c);expected=sorted(z for z in itertools.product(*[range(a,b+1)for a,b in boxes])if c(z)==0);assert got['complete']and got['roots']==expected;out['prime_root_oracles']+=1
 S=PrimeProjection(5,(0,-1),(0,4),1);assert S.cardinality==0 and S.witness_count(0)==0
 assert solve_prime_boxes(5,[(0,1)]*2,[1,1],2,0,lambda z:(sum(z)-1)%5)['roots']==[(0,1),(1,0)]
 specs=json.loads(Path(data['field_data']).read_bytes())['fields']
 for spec in specs:
  begin=time.monotonic();f=Arithmetic(spec['p'],spec['mod']);choices=[[]]+[[f.p**i for i in range(d)]for d in sorted({1,min(2,f.n),f.n if f.n<=3 else 2})];checks=0;direct=0
  for Bj in choices:
   for Bp in choices:
    for oj,op,r in itertools.product([0,f.q-1],[0,f.q-1],sorted({0,1,f.q-1,f.p if f.n>1 else 1})):
     S=SubspaceProjection(f,oj,Bj,op,Bp,r);assert S.contains(S.offset);assert all(S.contains(f.add(S.offset,b))for b in Bj);checks+=1
     if f.q<=64:
      A=span(f,oj,Bj);B=span(f,op,Bp);expected={f.sub(a,f.mul(r,b))for a in A for b in B};assert S.cardinality==len(expected);assert set(S.members())==expected
      for x in range(f.q):assert S.contains(x)==(x in expected)
      direct+=1
     else:
      for _ in range(data['query_samples']):
       a=oj;b=op
       for v in Bj:a=f.add(a,f.mul(rng.randrange(f.p),v))
       for v in Bp:b=f.add(b,f.mul(rng.randrange(f.p),v))
       assert S.contains(f.sub(a,f.mul(r,b)))
      if S.rank==0:assert S.cardinality==1
      if S.rank==f.n:assert all(S.contains(rng.randrange(f.q))for _ in range(3))
  out['contexts'].append({'name':spec.get('name',str(spec['p'])+'^'+str(f.n)),'projection_cases':checks,'complete_direct_cases':direct,'seconds':time.monotonic()-begin})
  if spec.get('name')=='GF2^83':
   B=[1<<i for i in range(18)];same=SubspaceProjection(f,0,B,0,B,1);shift=SubspaceProjection(f,0,B,0,B,1<<18);assert same.rank==18 and shift.rank==36 and same.cardinality==1<<18 and shift.cardinality==1<<36;assert shift.contains((1<<36)-1)and not shift.contains(1<<36)
   for S in [same,shift]:
    try:S.members(data['max_member_enumeration'])
    except ProjectionBudget:out['refusals']+=1
    else:raise AssertionError('largeenumerationadmitted')
   out['binary83']={'same_ratio_rank':18,'shift_ratio_rank':36,'cardinalities':[same.cardinality,shift.cardinality],'large_enumeration':False}
  if spec.get('name')in data['nist_dimensions']:
   p=f.p;d=data['nist_dimensions'][spec['name']];M=(1<<d)-1;begin=time.monotonic();S=PrimeProjection(p,(0,M),(0,M),1);T=PrimeProjection(p,(0,M),(0,M),p-1);assert S.cardinality==T.cardinality==2*M+1;assert S.witness_count(0)==M+1 and S.witness_count(M)==1 and S.witness_count(p-M)==1 and S.witness_count(M+1)==0;assert T.witness_count(M)==M+1 and T.witness_count(2*M)==1 and T.witness_count(2*M+1)==0;V=PrimeProjection(p,(0,M),(0,M),(p-1)//2);assert V.witness_count(0)==1
   W=PrimeProjection(p,(0,M),(0,M),rng.randrange(M+2,p-M-2))
   for _ in range(data['query_samples']):
    a=rng.randrange(M+1);b=rng.randrange(M+1);assert V.contains((a-V.r*b)%p)and W.contains((a-W.r*b)%p)
   result=solve_prime_boxes(p,[(0,M)]*3,[1,1,1],3,0,lambda z:(sum(z)-1)%p,cap=data['max_member_enumeration']);assert not result['complete']and result['roots']is None;out['refusals']+=1;out['nist'].append({'name':spec['name'],'source_bits':d,'segment_count':len(S.segments),'projection_cardinality':S.cardinality,'arbitrary_ratio_mode':W.describe()['mode'],'max_floor_steps':max(S.max_steps,T.max_steps,V.max_steps,W.max_steps),'query_seconds':time.monotonic()-begin,'large_root_enumeration':False})
 out['all_registered_contexts_complete']=len(out['contexts'])==len(specs);out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Exact implicit prime-interval and extension-subspace projections,independent small oracles and full-width domain queries','If passed,integrate descriptors as bounded native-domain predicates or derive exact implicit candidate indexing;never enumerate a refused large projection or claim large-curve solving from domain-query timing.')
