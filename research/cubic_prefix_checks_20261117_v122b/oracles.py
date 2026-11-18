import json,time,itertools,random
from collections import Counter
from .common import P,finish
from research.exact_cubic_prefix_20261117_v122.kernel import support
from research.prefix_translation_20261116_v115.kernel import product
from research.residue_quotient_20261116_v99b.kernel import pmul
START=time.monotonic();out={'exhaustive_cubics':0,'catalog_multisets':0,'partial_root_rejections':0,'random_cubics':0,'wide_planted_cases':0,'widths':[],'original_complete':False};error=None

def catalog(m):return {tuple(product(xs)):sorted(Counter(xs).items())for xs in itertools.combinations_with_replacement(range(1<<m),3)}
def literal_value(H,x):
    x2=pmul(x,x);return H[0]^pmul(H[1],x)^pmul(H[2],x2)^pmul(x2,x)
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for m in range(1,4):
        roots=catalog(m);out['catalog_multisets']+=len(roots)
        for v in range(1<<(6*m-3)):
            H=[];offset=0
            for j in range(3):
                width=(3-j)*(m-1)+1;H.append((v>>offset)&((1<<width)-1));offset+=width
            H.append(1);wanted=roots.get(tuple(H));got=support(H,m);assert got['supported']==(wanted is not None)
            if wanted is not None:assert got['roots']==wanted and got['all_domain_roots_claim']
            else:
                assert got['roots']is None and not got['all_domain_roots_claim']
                if any(literal_value(H,x)==0 for x in range(1<<m)):out['partial_root_rejections']+=1
            out['exhaustive_cubics']+=1
    for m in [4,5]:
        roots=catalog(m);out['catalog_multisets']+=len(roots)
        for _ in range(128):
            H=[rng.randrange(1<<((3-j)*(m-1)+1))for j in range(3)]+[1];wanted=roots.get(tuple(H));got=support(H,m);assert got['supported']==(wanted is not None)
            if wanted is not None:assert got['roots']==wanted
            else:assert got['roots']is None
            out['random_cubics']+=1
    for m in [4,5,17,32,64]:
        out['widths'].append(m)
        for case in range(32):
            xs=[rng.randrange(1<<m)for _ in range(3)]
            if case%4==0:xs=[xs[0]]*3
            if case%4==1:xs[1]=xs[0]
            if case%4==2:xs[0]=0
            H=product(xs);got=support(H,m);assert got['supported']and got['roots']==sorted(Counter(xs).items());out['wide_planted_cases']+=1
    got=support([0,1,1,1],2);assert not got['supported']and got['roots']is None and literal_value([0,1,1,1],0)==0
except Exception as e:error=repr(e)
finish(out,error,START,'Independent complete cubic prefix-product catalogs verify fullsupportandacceptedrootmultiplicities,includingpartialrootrejections.','Onlyprofile sharedstatecubiccircuitsafter thisoraclepasses;no2^mbranchtree orclaimofemptydomainwhenfullsupportfails.')
