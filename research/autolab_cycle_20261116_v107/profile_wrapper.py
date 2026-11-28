import contextlib,io,json,runpy,sys,time
from pathlib import Path
from research.local_nilpotency_20261116_v100.kernel import Arithmetic
C=Path(__file__).resolve().parent;START=time.monotonic();group=sys.argv[3]
capture=io.StringIO()
with contextlib.redirect_stdout(capture):runpy.run_module('research.discriminant_roots_20261116_v106d.profile',run_name='__main__')
result=json.loads(capture.getvalue());out=result['evidence']['out'];P=C.parents[1]/'research/discriminant_roots_20261116_v106d';panel=json.loads((P/'PANEL.json').read_text());counts={'squarefree':[1,1,1,1,1],'single_repeat':[1,1,1,2],'max_repeat':[1,2,2],'nonrational':[1],'five_repeat':[5]};checked=0
for row in out['runs']:
    spec=next(s for s in panel[group]if s['field']['name']==row['field']);case=next(x for x in spec['cases']if x['name']==row['case']);f=Arithmetic(spec['field']['p'],spec['field']['mod']);roots=row['roots'];assert sorted(m for x,m in roots)==counts[row['case']];assert len({x for x,m in roots})==len(roots)
    for x,multiplicity in roots:
        H=list(case['H']);observed=0
        while len(H)>1:
            value=0
            for c in reversed(H):value=f.add(f.mul(value,x),c)
            if value:break
            Q=[0]*(len(H)-1);Q[-1]=H[-1]
            for i in range(len(H)-2,0,-1):Q[i-1]=f.add(H[i],f.mul(x,Q[i]))
            H=Q;observed+=1
        assert observed==multiplicity
    checked+=1
receipt={'group':group,'completed_rows_verified':checked,'independent_arithmetic':'original Arithmetic, not new byte-squaring/inverse class','known_fixture_count_rule':counts,'all_memberships_multiplicities_and_constructed_counts_passed':True,'wrapper_seconds':time.monotonic()-START};(C/('COUNT_CERTIFICATE_'+group+'.json')).open('x').write(json.dumps(receipt,indent=2)+'\n');result['evidence']['independent_fixture_certificate']=receipt
limit=235 if group=='binary'else145
if time.monotonic()-START>=limit:result['status']='inconclusive';result['evidence']['wrapper_deadline_exceeded']=True
text=json.dumps(result);assert time.monotonic()-START<limit+4;print(text)
