from pathlib import Path
from itertools import product
import json,hashlib,datetime
from adapter import diagnostic_row
D=Path(__file__).parent
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
sources=[R/'research/pencil_large_prime_20261029/src/native_solver.rs',R/'research/pencil_lifecycle_20261030/integration/five-followups/rank-information/src/lib.rs',D/'adapter.py',D/'PLAN.md',D/'check.py']
def bindings():return {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
before=bindings();(D/'input-bindings-before.json').write_text(json.dumps(before,indent=2)+'\n')
def rank(rows,width,p):
    a=[list(row[:width]) for row in rows];i=0
    for j in range(width):
        pivot=next((k for k in range(i,len(a)) if a[k][j]%p),None)
        if pivot is None:continue
        a[i],a[pivot]=a[pivot],a[i];inv=pow(a[i][j],-1,p);a[i]=[v*inv%p for v in a[i]]
        for k in range(len(a)):
            if k!=i:
                v=a[k][j];a[k]=[(x-v*y)%p for x,y in zip(a[k],a[i])]
        i+=1
    return i
def check(p,rows):
    converted=[diagnostic_row(p,row) for row in rows]
    rhs={(x,k) for x,k in product(range(p),repeat=2) if all((row[0]*x+row[1]*k)%p==row[2] for row in rows)}
    got={(x,k) for x,k in product(range(p),repeat=2) if all((row[0]*x+row[1]*k+row[2])%p==0 for row in converted)}
    wrong={(x,k) for x,k in product(range(p),repeat=2) if all((row[0]*x+row[1]*k+row[2])%p==0 for row in rows)}
    ranks=[rank(rows,w,p) for w in (1,2,3)];other=[rank(converted,w,p) for w in (1,2,3)]
    assert got==rhs and wrong=={((-x)%p,(-k)%p) for x,k in rhs} and ranks==other
    ks=sorted({k for x,k in got});assert (ranks[1]==ranks[2] and ranks[1]==ranks[0]+1)==(len(ks)==1)
    return {'legacy':rows,'normalized':converted,'solutions_count':len(got),'target_values':ks,'ranks':ranks}
counts={}
for p in (5,7):
    records=[check(p,[list(row)]) for row in product(range(p),repeat=3)]
    (D/f'rows-p{p}.json').write_text(json.dumps(records,separators=(',',':'))+'\n');counts[str(p)]=len(records)
fixed={'empty':[],'nuisance_only':[[1,0,2]],'target_only':[[0,1,5]],'two_row':[[1,1,5],[1,2,3]],'inconsistent':[[0,0,1]],'target_zero':[[0,1,0]]}
results={name:check(7,rows) for name,rows in fixed.items()};assert results['target_only']['target_values']==[5] and results['two_row']['target_values']==[5]
(D/'fixed-controls.json').write_text(json.dumps(results,indent=2)+'\n')
invalid=[(1,[0,0]),(True,[0,0]),(7,[0]),(7,[0,7]),(7,[0,-1]),(7,[0,True]),(7,'ab'),(7,[0,1.0])]
for p,row in invalid:
    try:diagnostic_row(p,row)
    except ValueError:pass
    else:raise AssertionError((p,row))
after=bindings();assert before==after;(D/'input-bindings-after.json').write_text(json.dumps(after,indent=2)+'\n')
summary={'schema':'research.software-control/v1','one_row_cases_by_modulus':counts,'fixed_multirow_cases':len(fixed),'invalid_input_cases':len(invalid),'all_checks_passed':True,'input_bindings_unchanged':True,'independent_review':'pending','actual_collector_streams_checked':0,'scientific_execution':False,'cost_ratio':None,'counterexample':{'modulus':7,'legacy':[0,1,5],'normalized':[0,1,2],'correct_target':5,'unconverted_homogeneous_target':2,'both_rank_triples':[0,1,1]}}
(D/'status.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary))
