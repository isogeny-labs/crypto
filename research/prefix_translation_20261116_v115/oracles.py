from pathlib import Path
import json,time,itertools,random
from .common import P,finish
from .kernel import product,translate,independent_compose,bounded,degree_equations
from research.residue_quotient_20261116_v99b.kernel import apply
START=time.monotonic();out={'root_multisets':0,'odd_leading_cases':0,'all_upper_cases':0,'repeated_cases':0,'zero_root_cases':0,'linearization_cases':0,'original_complete':False};error=None
try:
    cfg=json.loads((P/'DATA.json').read_text());rng=random.Random(cfg['seed'])
    cases=[]
    for d in [2,3]:cases += [(d,xs)for xs in itertools.combinations_with_replacement(range(1<<d),5)]
    for _ in range(128):cases.append((4,tuple(sorted(rng.randrange(16)for _ in range(5)))))
    for _ in range(128):cases.append((4,tuple(sorted(8+rng.randrange(8)for _ in range(5)))))
    for d,xs in cases:
        c=1<<(d-1);H=product(xs);shifted=translate(H,c)
        assert shifted==independent_compose(H,c)==product([x^c for x in xs])
        out['root_multisets']+=1;out['repeated_cases']+=int(len(set(xs))<5);out['zero_root_cases']+=int(0 in xs)
        if (H[4]>>(d-1))&1:
            k=sum(bool(x&c)for x in xs);w=(H[1]>>(4*(d-1)))&1
            assert k in[1,3,5]and bool(w)==(k==5)and bounded(shifted,d)==(k==5)
            out['odd_leading_cases']+=1
            if w:assert max(x^c for x in xs)<c;out['all_upper_cases']+=1
    for _ in range(80):
        d=2+rng.randrange(3);n=6;base=[rng.randrange(1<<((5-j)*(d-1)+1))for j in range(5)]+[1];directions=[[rng.randrange(1<<((5-j)*(d-1)+1))for j in range(5)]+[0]for _ in range(n)];rows,labels=degree_equations(base,directions,d)
        for theta in range(1<<n):
            H=list(base)
            for i,v in enumerate(directions):
                if theta>>i&1:H=[a^b for a,b in zip(H,v)]
            assert all((mask&theta).bit_count()%2==rhs for mask,rhs in rows)==bounded(translate(H,1<<(d-1)),d)
        out['linearization_cases']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Independent multiset products/composition and exhaustive raw-bit linearization of translated degree constraints.','Measure actual additional rank only on w=1chart with original M*q lifts;retain w=0and unresolved nonlinear support.')
