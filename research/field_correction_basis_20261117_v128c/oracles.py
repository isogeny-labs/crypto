import json,time,random
from .common import P,finish
from .kernel import audited_lift
from research.odd_coprime_factor_20261116_v119c.kernel import initial,multiply,lift as full_lift,reference_d2,trim
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
START=time.monotonic();out={'contexts':[],'planted_pairs':0,'reference_cases':0,'inverse_checks':0,'coefficient_identity_checks':0,'basis_equivalence_checks':0,'full_lift_comparisons':0,'counterexamples':0,'repeated_factor_checks':0,'original_complete':False};error=None

def check(got,d,f):
 m=d-1;beta=f.neg(3%f.p);allrows=[];basis=[]
 for row in got['audit']:
  n=row['precision'];e=row['error'];a0,a1,b0,b1,b2=row['correction'];assert (not any(row['expired']))==(not any(row['basis']));out['basis_equivalence_checks']+=1;allrows+=row['expired'];basis+=row['basis']
  if n>m:assert e[4]==0 and f.add(a1,b2)==0;out['coefficient_identity_checks']+=1
  if n>2*m:assert e[3]==0 and f.add(f.add(a0,f.mul(beta,a1)),b1)==0;out['coefficient_identity_checks']+=1
 assert len(allrows)==6*m and len(basis)==3*m and (not any(allrows))==(not any(basis));out['basis_equivalence_checks']+=1
 return basis
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed'])
 for spec in data['fields']:
  f=Arithmetic(spec['p'],spec['mod']);out['contexts'].append({'name':spec.get('name'),'p':f.p,'degree':f.n,'q':f.q})
  for a in (range(1,f.q)if f.q<=49 else [rng.randrange(1,f.q)for _ in range(8)]):assert f.mul(a,f.inv(a))==1;out['inverse_checks']+=1
  for case in range(12):
   d=2+case%2;m=d-1;A=[[0]+[rng.randrange(f.q)for _ in range(2*m)],[0]+[rng.randrange(f.q)for _ in range(m)],[1]];B=[[b]+[rng.randrange(f.q)for _ in range((3-j)*m)]for j,b in enumerate(initial(f))];F=multiply(A,B,f);got=audited_lift(F,d,f);check(got,d,f);old=full_lift(F,d,f);assert got['bounded_factorization']==old['bounded_factorization']==True;assert [trim(v)for v in got['A']]==[trim(v)for v in A]and [trim(v)for v in got['B']]==[trim(v)for v in B];out['full_lift_comparisons']+=1;out['planted_pairs']+=1
  F=[[0],[0]]+[[v]for v in initial(f)];repeated=audited_lift(F,2,f);check(repeated,2,f);assert repeated['bounded_factorization'];out['repeated_factor_checks']+=1
  F=[[0,0,0,0,0,1],[0]]+[[v]for v in initial(f)];bad=audited_lift(F,2,f);basis=check(bad,2,f);assert not any(basis)and not bad['bounded_factorization'];out['counterexamples']+=1
  for _ in range(4):
   leading=[0,0]+initial(f);F=[[leading[j]]+[rng.randrange(f.q)for _ in range(5-j)]for j in range(6)];got=audited_lift(F,2,f);check(got,2,f)
   if f.q<=9:assert got['bounded_factorization']==bool(reference_d2(F,f));out['reference_cases']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Correction-basis identities,exact bounded-factor control and defect-onlycounterexamples across17coefficientfields.','Keep all-root/curve/prime-interval coverage separate;only freeze stronger binarysearch after independent support and fieldbasis controls pass.')
