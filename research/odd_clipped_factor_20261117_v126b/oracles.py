import json,time,random
from .common import P,finish
from .kernel import initial,inverse_block,matvec,multiply,lift,reference_d2,trim
from research.odd_coprime_factor_20261116_v119c.kernel import lift as full_lift
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
START=time.monotonic();out={'contexts':[],'planted_pairs':0,'random_reference_cases':0,'linear_blocks':0,'field_inverse_checks':0,'full_lift_comparisons':0,'exact_product_checks':0,'noncoprime_dispatch_checks':0,'original_complete':False};error=None
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for spec in data['fields']:
        f=Arithmetic(spec['p'],spec['mod']);assert f.p%2;out['contexts'].append({'name':spec.get('name'),'p':f.p,'degree':f.n,'q':f.q})
        for a in (range(1,f.q)if f.q<=49 else [rng.randrange(1,f.q)for _ in range(16)]):assert f.mul(a,f.inv(a))==1;out['field_inverse_checks']+=1
        S,I=inverse_block(f)
        for _ in range(32):
            e=[rng.randrange(f.q)for _ in range(5)];assert matvec(S,matvec(I,e,f),f)==e;out['linear_blocks']+=1
        for case in range(16):
            d=2+case%2;A=[[0]+[rng.randrange(f.q)for _ in range(2*(d-1))],[0]+[rng.randrange(f.q)for _ in range(d-1)],[1]];B=[[b]+[rng.randrange(f.q)for _ in range((3-j)*(d-1))]for j,b in enumerate(initial(f))];F=multiply(A,B,f);got=lift(F,d,f);old=full_lift(F,d,f);assert got['bounded_factorization']==old['bounded_factorization'];out['full_lift_comparisons']+=1;assert got['bounded_factorization'];assert [trim(v)for v in multiply(got['A'],got['B'],f)]==[trim(v)for v in F];out['exact_product_checks']+=1;assert [trim(v)for v in got['A']]==[trim(v)for v in A]and [trim(v)for v in got['B']]==[trim(v)for v in B];out['planted_pairs']+=1
        if f.q<=9:
            for _ in range(12):
                leading=[0,0]+initial(f);F=[[leading[j]]+[rng.randrange(f.q)for _ in range(5-j)]for j in range(6)];got=lift(F,2,f);want=reference_d2(F,f);assert got['bounded_factorization']==bool(want)
                if want:assert [trim(v)for v in got['A']]==[trim(v)for v in want[0][0]]and [trim(v)for v in got['B']]==[trim(v)for v in want[0][1]]
                out['random_reference_cases']+=1
        # Same-cluster repeated factors are covered by deterministic planted factor pairs.
        bad=[[0]for _ in range(6)];bad[-1]=[1]
        try:lift(bad,2,f)
        except AssertionError:out['noncoprime_dispatch_checks']+=1
        else:raise AssertionError('noncoprime-leadingformwasaccepted')
except Exception as e:error=repr(e)
finish(out,error,START,'Degree-clipped odd factor lifting across13coefficientfields, five NIST primes, repeated-factor cases and independent small references.','Fieldgenericfactorliftingisnotprimeinterval/curvesupportcoverage;nextmeasurebothfactorsupportandwholecoupledformulation.')
