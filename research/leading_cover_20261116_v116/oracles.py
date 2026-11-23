from pathlib import Path
import json,time,itertools,random
from .common import P,finish
from .kernel import limits,fits,equations
from research.prefix_translation_20261116_v115.kernel import product,translate,independent_compose
START=time.monotonic();out={'root_multisets':0,'odd_cases':0,'stratum_counts':{'1':0,'3':0,'5':0},'repeated_cases':0,'zero_cases':0,'affine_families':0,'affine_point_checks':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed']);cases=[]
    for size in[2,3]:cases.extend((size,xs)for xs in itertools.combinations_with_replacement(range(1<<size),5))
    for _ in range(256):cases.append((4,tuple(sorted(rng.randrange(16)for _ in range(5)))))
    for size,xs in cases:
        c=1<<(size-1);k=sum(bool(x&c)for x in xs);H=product(xs);G=translate(H,c);assert G==independent_compose(H,c)==product([x^c for x in xs]);assert fits(H,limits(size,k))and fits(G,limits(size,5-k));out['root_multisets']+=1;out['repeated_cases']+=int(len(set(xs))<5);out['zero_cases']+=int(0 in xs)
        if(H[4]>>(size-1))&1:
            u=(H[3]>>(2*(size-1)))&1;v=(H[2]>>(3*(size-1)))&1;w=(H[1]>>(4*(size-1)))&1
            assert {(0,0,0):1,(1,1,0):3,(0,0,1):5}[(u,v,w)]==k;out['odd_cases']+=1;out['stratum_counts'][str(k)]+=1
    for _ in range(80):
        size=2+rng.randrange(3);k=rng.randrange(6);base=[rng.randrange(1<<((5-j)*(size-1)+1))for j in range(5)]+[1];directions=[[rng.randrange(1<<((5-j)*(size-1)+1))for j in range(5)]+[0]for _ in range(6)]
        for shift in[False,True]:
            eq,labels=equations(base,directions,size,k,shift)
            for theta in range(64):
                H=list(base)
                for i,v in enumerate(directions):
                    if theta>>i&1:H=[a^b for a,b in zip(H,v)]
                G=translate(H,1<<(size-1))if shift else H;assert all((m&theta).bit_count()%2==rhs for m,rhs in eq)==fits(G,limits(size,5-k if shift else k));out['affine_point_checks']+=1
        out['affine_families']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Independent complete leading-count cover and weighted unshifted/shifted degree-row oracle.','Profile allthree actual strata with the same original quotient lifts;do notdiscardanycase or infer speed fromrank.')
