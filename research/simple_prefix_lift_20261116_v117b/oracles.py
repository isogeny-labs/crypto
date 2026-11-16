from pathlib import Path
import json,time,random
from .common import P,finish
from .kernel import high_root,evaluate,normalize,lift_circuit
from research.prefix_translation_20261116_v115.kernel import product
from research.leading_cover_20261116_v116.kernel import limits
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'numeric_cases':0,'literal_root_tests':0,'planted_cases':0,'no_root_cases':0,'boolean_cases':0,'boolean_assignments':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for size in[2,3,4]:
        c=1<<(size-1)
        for case in range(64):
            if case<32:
                roots=[c+rng.randrange(c)]+[rng.randrange(c)for _ in range(4)];H=product(roots);out['planted_cases']+=1
            else:H=[rng.randrange(1<<(bound+1))for bound in limits(size,1)]+[1];H[4]=c|rng.randrange(c)
            root,tail,series=high_root(H,size);literal=[]
            for x in range(c,2*c):
                out['literal_root_tests']+=1
                if evaluate(H,x)==0:literal.append(x)
            assert literal==([root]if tail==0 else[]);assert len(literal)<=1
            out['no_root_cases']+=int(not literal);out['numeric_cases']+=1
    for case in range(24):
        size=2+case%2;n=5;C=Circuit(n);G=[]
        for j in range(6):
            width=(5-j)*(size-1)+1;bits=[]
            for bit in range(width):
                if bit==0:value=1 if j in[4,5]else 0
                elif j==5:value=0
                else:
                    value=rng.randrange(2)
                    for k in range(n):
                        if rng.randrange(2):value=C.xor(value,2+k)
                bits.append(value)
            G.append(bits)
        roots,tail,low=lift_circuit(G,size,C)
        for theta in range(1<<n):
            values=C.evaluate(theta);coeff=[sum(values[w]<<i for i,w in enumerate(g))for g in G];H=[sum(((a>>i)&1)<<((5-j)*(size-1)-i)for i in range((5-j)*(size-1)+1))for j,a in enumerate(coeff)];root,residual,series=high_root(H,size);assert sum(values[w]<<i for i,w in enumerate(roots))==root;assert sum(values[w]<<i for i,w in enumerate(tail))==residual;assert not any(values[w]for w in low);out['boolean_assignments']+=1
        out['boolean_cases']+=1
except Exception as e:error=repr(e)
finish(out,error,START,'Independent literal polynomial high-root existence and exhaustive Boolean Hensel-lift equivalence.','Profile actual k1coefficient-family circuit only after uniqueness/finite-tail contract passes;this is not allfive-root support.')
