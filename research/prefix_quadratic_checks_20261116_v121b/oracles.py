import json,time
from .common import P,finish
from .circuit import prefix_quad
from research.triangular_prefix_quadratic_20261116_v121.kernel import solve
from research.residue_quotient_20261116_v99b.kernel import pmul,basis,preimage,apply
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.local_nilpotency_20261116_v100.kernel import Arithmetic
START=time.monotonic();out={'raw_coefficient_cases':0,'literal_raw_roots_tested':0,'boolean_assignments':0,'field_coefficient_cases':0,'fast_field_routes':0,'fallback_routes':0,'wrap_counterexample_checked':False,'original_complete':False};error=None

def matrix_roots(a,b,m,square,multiply):
    piv,K=basis([square(1<<j)^multiply(a,1<<j)for j in range(m)])
    try:r=preimage(b,piv)
    except ValueError:return []
    return sorted((r^apply(K,z),2 if a==0 else 1)for z in range(1<<len(K)))
try:
    data=json.loads((P/'DATA.json').read_text())
    for m in range(1,6):
        for a in range(1<<m):
            for b in range(1<<(2*m-1)):
                want=[(r,2 if a==0 else 1)for r in range(1<<m)if pmul(r,r)^pmul(a,r)^b==0];got=solve(a,b,m);assert got['complete']and got['roots']==want;assert matrix_roots(a,b,m,lambda r:pmul(r,r),pmul)==want;out['raw_coefficient_cases']+=1;out['literal_raw_roots_tested']+=1<<m
        if m<=4:
            C=Circuit(3*m-1);a=[2+j for j in range(m)];b=[2+m+j for j in range(2*m-1)];checks,root=prefix_quad(a,b,C)
            for x in range(1<<C.n):
                av=x&((1<<m)-1);bv=x>>m;wanted=solve(av,bv,m)['roots'];values=C.evaluate(x);assert(not any(values[w]for w in checks))==bool(wanted)
                if wanted:assert sum(values[w]<<i for i,w in enumerate(root))==wanted[0][0]
                out['boolean_assignments']+=1
    for n,mod in [(2,7),(3,11),(4,19),(5,37)]:
        f=Arithmetic(2,[(mod>>j)&1 for j in range(n+1)])
        for m in range(1,n+1):
            for a in range(1<<m):
                for b in range(f.q):
                    want=[(r,2 if not a else 1)for r in range(1<<m)if f.sq(r)^f.mul(a,r)^b==0];assert matrix_roots(a,b,m,f.sq,f.mul)==want;out['field_coefficient_cases']+=1
                    if n>2*m-2:
                        got=solve(a,b,m)['roots']if b<(1<<(2*m-1))else[];assert got==want;out['fast_field_routes']+=1
                    else:out['fallback_routes']+=1
    f=Arithmetic(2,[1,1,1]);assert solve(0,3,2)['roots']==[]and matrix_roots(0,3,2,f.sq,f.mul)==[(2,2)];out['wrap_counterexample_checked']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Independent literal/matrix/Boolean oracles for inversionfreeexactquadraticprefixroots,includingfieldwrapdispatch.','Profileallleadingdegreeguardsandfullfactorformulation;keeptheprovedfielddegreeconditionandfallback;notoriginalchaincompleteness.')
