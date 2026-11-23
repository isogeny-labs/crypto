import json,time,random,hashlib
from collections import Counter
from .common import P,finish
from .kernel import compile_all,forward
from research.local_split_circuit_20261116_v100.kernel import cnf
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
from research.residue_quotient_20261116_v99b.kernel import pmul,rem
START=time.monotonic();out={'charts':[],'numeric_controls':0,'original_complete':False};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rows=matrix(d['matrix']);rng=random.Random(d['seed']);results=compile_all(d)
    for i,((C,outputs),chart)in enumerate(results):
        for _ in range(32):
            sigma=rng.randrange(1<<67);theta=forward(chart,sigma);values=C.evaluate(sigma);H=decode(rows,theta);assert valid_guard(H,theta>>73);H[0]^=pmul(theta>>73,d['modulus'])
            for r in outputs[:-1]:
                s,m=r['s'],r['m'];ok,_=predicate([rem(x,m)for x in H],Arithmetic(2,[(m>>j)&1 for j in range(s+1)]));assert (not any(values[w]for field in r['wires']for w in field))==ok
            assert all(w==0 for field in outputs[-1]['wires']for w in field);out['numeric_controls']+=1
        raw=json.dumps({'nodes':C.nodes,'outputs':outputs},separators=(',',':')).encode()
        with(P/('graph-'+str(i)+'.json')).open('xb')as f:f.write(raw)
        out['charts'].append({'chart':i,'inputs':C.n,'variables':len(C.nodes)-2,'clauses':len(cnf(C,outputs)),'gate_kinds':dict(Counter(n[0]for n in C.nodes)),'graph_sha256':hashlib.sha256(raw).hexdigest()})
    total=sum(r['variables']for r in out['charts']);out.update({'total_variables':total,'total_clauses':sum(r['clauses']for r in out['charts']),'search_promotion':total<48887 and sum(r['clauses']for r in out['charts'])<179895,'total_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Both67-input chart circuits with full numeric residue checks and duplicated total formulation cost.','If combined charts grow, measure an exact unified68-input map using their common66-dimensional direction space and one selector product; no native retry without improved formulation.')
