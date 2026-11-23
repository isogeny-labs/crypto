import json,time,itertools,random
from .common import P,finish
from .kernel import tiny
from research.prefix_translation_20261116_v115.kernel import product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import rem
START=time.monotonic();out={'exhaustive_assignments':0,'planted_assignments':0,'random_assignments':0,'accepted':0,'q_alias_checks':0,'original_complete':False};error=None

def catalog(d):
    top=1<<(d-1);return {tuple(product(low+high)):list(low+high)for low in itertools.combinations_with_replacement(range(top),2)for high in itertools.combinations_with_replacement(range(top,1<<d),3)}
def decode(x,d):
    F=[];offset=0
    for j in range(5):
        width=(5-j)*(d-1);F.append((((x>>offset)&((1<<width)-1))<<1)|int(j>=2));offset+=width
    F.append(1);return tuple(reverse(v,(5-j)*(d-1))for j,v in enumerate(F))
def encode(H,d):
    x=0;offset=0
    for j,v in enumerate(H[:5]):x|=(reverse(v,(5-j)*(d-1))>>1)<<offset;offset+=(5-j)*(d-1)
    return x
try:
    data=json.loads((P/'DATA.json').read_text());rng=random.Random(data['seed'])
    for d in [2,3]:
        candidate,control=tiny(d);C,oo,roots=candidate;D,co,oldroots=control;expected=catalog(d)
        cases=list(range(1<<C.n))if d==2 else[encode(H,d)for H in expected]+[rng.randrange(1<<C.n)for _ in range(128)]
        for index,x in enumerate(cases):
            v=C.evaluate(x);w=D.evaluate(x);H=decode(x,d);want=expected.get(H);got=not any(v[z]for row in oo for field in row['wires']for z in field);old=not any(w[z]for row in co for field in row['wires']for z in field);assert got==old==(want is not None);assert [[v[z]for z in row]for row in roots]==[[w[z]for z in row]for row in oldroots]
            if got:assert sorted(sum(v[z]<<i for i,z in enumerate(row))for row in roots)==want;out['accepted']+=1
            if d==2:out['exhaustive_assignments']+=1
            elif index<len(expected):out['planted_assignments']+=1
            else:out['random_assignments']+=1
        if d==2:
            H=product([0,0,2,2,2]);alias=list(H);alias[0]^=11;assert [rem(a,11)for a in H]==[rem(a,11)for a in alias];v=C.evaluate(encode(alias,2));assert any(v[z]for row in oo for field in row['wires']for z in field);out['q_alias_checks']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Exactraw root-product verifier matches isolatedfullsupportcontrolandindependentfive-rootcatalogs,includingrepeated/qaliascases.','Profileactualsamecontract/rootoutput costs afteroraclepasses;noGF83originalchaincountorqualifiedspeedupfromsmallsets.')
