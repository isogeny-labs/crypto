import json,time,itertools,random
from .common import P,finish
from research.factor_local_support_20261116_v120.kernel import split_output,reduce_factor
from research.local_split_circuit_20261116_v100.kernel import Circuit,BF
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,literal_split,predicate,roots_product
from research.simple_prefix_lift_20261116_v117b.kernel import reverse
from research.residue_quotient_20261116_v99b.kernel import rem
START=time.monotonic();out={'monic_polynomials':0,'factor_product_pairs':0,'reduction_checks':0,'counterexample_fields':[],'original_complete':False};error=None

def product(A,B,f):
    H=[0]*(len(A)+len(B)-1)
    for i,a in enumerate(A):
        for j,b in enumerate(B):H[i+j]=f.add(H[i+j],f.mul(a,b))
    return H
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for s,mod in [(1,3),(2,7),(3,11)]:
        f=Arithmetic(2,[(mod>>j)&1 for j in range(s+1)]);q=f.q
        for degree in [2,3]:
            C=Circuit(s*degree);H=[[2+j*s+i for i in range(s)]for j in range(degree)]+[[1]+[0]*(s-1)];wires=split_output(H,BF(s,mod,C))
            for coefficients in itertools.product(range(q),repeat=degree):
                x=sum(a<<(j*s)for j,a in enumerate(coefficients));values=C.evaluate(x);got=not any(values[w]for row in wires for w in row);assert got==literal_split(list(coefficients)+[1],f);out['monic_polynomials']+=1
        pairs=list(itertools.product(range(4),range(8)))if s==1 else[(rng.randrange(q*q),rng.randrange(q**3))for _ in range(64)]
        for a,b in pairs:
            A=[(a>>(j*s))&(q-1)for j in range(2)]+[1];B=[(b>>(j*s))&(q-1)for j in range(3)]+[1];H=product(A,B,f);assert(literal_split(A,f)and literal_split(B,f))==predicate(H,f)[0];out['factor_product_pairs']+=1
        for _ in range(32):
            C=Circuit(0)
            for degree in [2,3]:
                normalized=[rng.randrange(1<<((degree-j)*2+1))for j in range(degree)]+[1];factor=[[(a>>i)&1 for i in range((degree-j)*2+1)]for j,a in enumerate(normalized)];got=reduce_factor(factor,3,s,mod,C);values=C.evaluate(0);actual=[sum(values[w]<<i for i,w in enumerate(v))for v in got];want=[rem(reverse(a,(degree-j)*2),mod)for j,a in enumerate(normalized)];assert actual==want;out['reduction_checks']+=1
        A=[1,1,1];B=[rem(8,mod),rem(4,mod),rem(2,mod),1];H=product(A,B,f);ok=literal_split(A,f)and literal_split(B,f);assert ok==predicate(H,f)[0]and ok==(s==2);out['counterexample_fields'].append({'degree':s,'accepted':ok})
except Exception as e:error=repr(e)
finish(out,error,START,'Exhaustive monicquadratic/cubic support checks retainmultiplicity and match literalroots and productpredicate.','ProfilecombinedfactorizationandBOTHfactorsupportsets,with equallyscopedoriginalquinticcontrol and recoveredfactorwires.')
