import json,time,random,itertools
from .common import P,finish
from research.coprime_prefix_factor_20261116_v119.kernel import lift_k3,multiply
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import pmul
from research.prefix_translation_20261116_v115.kernel import product
START=time.monotonic();out={'exhaustive_F':0,'bounded_pairs':0,'planted_multisets':0,'random_reference_cases':0,'reference_factor_candidates':0,'counterexample_checked':False,'original_complete':False};error=None

def unnormalize(F,d):return [reverse(a,(5-j)*(d-1))for j,a in enumerate(F)]
def within(A,B,d):return all(a.bit_length()<=(2-j)*(d-1)+1 for j,a in enumerate(A))and all(b.bit_length()<=(3-j)*(d-1)+1 for j,b in enumerate(B))
def reference(F,d):
    found=[]
    for a0 in range(1<<(2*(d-1))):
        for a1 in range(1<<(d-1)):
            A=[a0<<1,a1<<1,1];work=list(F);B=[0]*4;out['reference_factor_candidates']+=1
            for j in range(5,1,-1):
                B[j-2]=work[j]
                for i,a in enumerate(A):work[j-2+i]^=pmul(a,B[j-2])
            if not any(work)and [b&1 for b in B]==[1,1,1,1]and within(A,B,d):found.append((A,B))
    assert len(found)<=1
    return found
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed']);catalog={}
    for a0,a1,b0,b1,b2 in itertools.product(range(4),range(2),range(8),range(4),range(2)):
        A=[a0<<1,a1<<1,1];B=[1|(b0<<1),1|(b1<<1),1|(b2<<1),1];F=multiply(A,B);assert tuple(F)not in catalog;catalog[tuple(F)]=(A,B);out['bounded_pairs']+=1
    for v in range(1<<15):
        F=[];shift=0
        for j in range(5):
            width=5-j;F.append((((v>>shift)&((1<<width)-1))<<1)|int(j>=2));shift+=width
        F.append(1);result=lift_k3(unnormalize(F,2),2);expected=catalog.get(tuple(F));assert result['bounded_factorization']==(expected is not None)
        if expected:assert (result['A'],result['B'])==expected
        out['exhaustive_F']+=1
    for d in [2,3]:
        top=1<<(d-1)
        for low in itertools.combinations_with_replacement(range(top),2):
            for high in itertools.combinations_with_replacement(range(top,1<<d),3):
                result=lift_k3(product(low+high),d);assert result['bounded_factorization'];assert result['A']==[reverse(a,(2-j)*(d-1))for j,a in enumerate(product(low))];assert result['B']==[reverse(b,(3-j)*(d-1))for j,b in enumerate(product(high))];out['planted_multisets']+=1
    for d in [3,4]:
        for _ in range(24):
            F=[(rng.randrange(1<<((5-j)*(d-1)))<<1)|int(j>=2)for j in range(5)]+[1];result=lift_k3(unnormalize(F,d),d);want=reference(F,d);assert result['bounded_factorization']==bool(want)
            if want:assert(result['A'],result['B'])==want[0]
            out['random_reference_cases']+=1
    H=[8,12,14,7,3,1];result=lift_k3(H,2);assert result['bounded_factorization'];catalog_roots={tuple(product(xs))for xs in itertools.combinations_with_replacement(range(4),5)};assert tuple(H)not in catalog_roots;out['counterexample_checked']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Independent exhaustive bounded-factor oracle for deterministic2+3lifting,including explicit nonsplit counterexample.','Only profileactualk3circuitif exactfactor oraclepasses;factorizationaloneisnotprefixsupport.')
