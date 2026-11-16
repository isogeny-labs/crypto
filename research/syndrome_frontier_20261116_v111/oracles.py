import json,time,random,itertools
from .common import P,finish
from .kernel import build_code,accepted,recover,Widths,RankCap
from research.residue_quotient_20261116_v99b.kernel import apply,basis
START=time.monotonic();out={'cases':0,'original_assignments':0,'domain_assignments':0,'cuts':0,'cap_checked':False,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(80):
        n=3+case%4;widths=[2,2,2];ambient=6;C=[rng.randrange(1<<ambient)for _ in range(n)];offset=rng.randrange(1<<ambient)
        tables=[sorted(set(rng.randrange(4)for j in range(rng.randrange(5))))for i in range(3)]
        code=build_code(C,offset,widths,tables,n);literal=set()
        for theta in range(1<<n):
            out['original_assignments']+=1;v=offset^apply(C,theta)
            if all((v>>(2*i))&3 in table for i,table in enumerate(tables)):literal.add(theta)
        recovered=set()
        for choice in itertools.product(*[h['domain']for h in code['hulls']]):
            out['domain_assignments']+=1
            if not accepted(code,choice):continue
            theta,K=recover(code,choice,C,offset)
            recovered.update(theta^apply(K,z)for z in range(1<<len(K)))
        assert recovered==literal
        W=Widths(code['blocks'])
        for mask in range(1,7):
            A=[v for i,b in enumerate(code['blocks'])if mask>>i&1 for v in b];B=[v for i,b in enumerate(code['blocks'])if not(mask>>i&1)for v in b]
            SA={apply(A,x)for x in range(1<<len(A))};SB={apply(B,x)for x in range(1<<len(B))}
            assert len(SA&SB)==1<<W.cut(mask);out['cuts']+=1
        out['cases']+=1
    try:Widths([[1],[2],[3]],0)
    except RankCap:out['cap_checked']=True
    assert out['cap_checked']
except Exception as e:error=repr(e)
finish(out,error,START,'Independent full-root recovery and literal separator-intersection oracles.','Profile actual full residue-code ordering widths before any frontier allocation.')
