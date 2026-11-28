import json,time,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import propagate,Budget
from research.residue_quotient_20261116_v99b.problem import build
from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply
START=time.monotonic();out={'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text())
    E,eb,widths,tables,F,fb,last,details=build(Path(d['old_problem']))
    piv,K=basis(E);R=[preimage(1<<i,piv)for i in range(72)];FR=[apply(F,r)for r in R]
    kp,_=basis([apply(F,k)for k in K]);free=[i for i in range(16)if i not in kp]
    assert len(free)==12 and len(K)==len(kp)==4
    def quotient(v):
        for i in sorted(kp,reverse=True):
            if v>>i&1:v^=kp[i][0]
        return sum(((v>>i)&1)<<j for j,i in enumerate(free))
    domains=[];offset=0
    for width,table in zip(widths,tables):
        domains.append([(x,quotient(apply(FR,x<<offset)))for x in sorted(table)]);offset+=width
    domains.append([(x,quotient(x))for x in sorted(last)])
    target=quotient(fb^apply(FR,eb));setup=time.monotonic()-START
    result=propagate(domains,12,target,Budget(d['work_cap']))
    assert result['exact_single_relation_tuple_count']==274316578275
    before=[len(x)for x in domains];after=[len(x)for x in result.pop('kept')]
    out.update({'tables':details,'before':before,'after':after,'removed':sum(before)-sum(after),'removal_fraction':1-sum(after)/sum(before),'setup_seconds':setup,'total_seconds':time.monotonic()-START,'result':result,'target':target,'domains_sha256':hashlib.sha256(json.dumps(domains).encode()).hexdigest()})
    with(P/'domains.json').open('x')as f:json.dump({'domains':domains,'target':target,'bits':12},f,separators=(',',':'))
except Exception as e:error=repr(e)
finish(out,error,START,'Actual fixed-trace block-domain support propagation through exact eighth-residue quotient.','If no cut, retire unary propagation; condition largest blocks under a separately frozen cell diagnostic and measure remaining outer-cover size.')
