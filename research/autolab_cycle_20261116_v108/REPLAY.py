from pathlib import Path
import json,hashlib,base64,tarfile,io,sys,collections,statistics,math,ast
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
    for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
        b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
for row in json.loads((C/'SYNTAX_PREFLIGHT.json').read_text())['files']:ast.parse((ROOT/row['path']).read_text())
from research.local_nilpotency_20261116_v100.kernel import Arithmetic
P='research/discriminant_roots_20261116_v108/';panel=json.loads(saved[P+'PANEL.json']);expected={'squarefree':[1]*5,'single_repeat':[1,1,1,2],'max_repeat':[1,2,2],'nonrational':[1],'five_repeat':[5]};ratios=[];checked=0
for group,count in [('binary',384),('odd',252)]:
    data=json.loads(saved[P+'cold_'+group+'_raw.json']);assert data['error']is None;rows=data['out']['runs'];assert len(rows)==count;by=collections.defaultdict(list)
    for r in rows:
        spec=next(x for x in panel[group]if x['field']['name']==r['field']);case=next(x for x in spec['cases']if x['name']==r['case']);f=Arithmetic(spec['field']['p'],spec['field']['mod']);roots=r['roots'];assert sorted(e for x,e in roots)==expected[r['case']]and len({x for x,e in roots})==len(roots)
        for x,e in roots:
            H=list(case['H']);n=0
            while len(H)>1:
                v=0
                for a in reversed(H):v=f.add(f.mul(v,x),a)
                if v:break
                Q=[0]*(len(H)-1);Q[-1]=H[-1]
                for i in range(len(H)-2,0,-1):Q[i-1]=f.add(H[i],f.mul(x,Q[i]))
                H=Q;n+=1
            assert n==e
        by[(r['field'],r['case'],r['method'])].append(r['wall_seconds']);checked+=1
    assert all(len(v)==3 for v in by.values())
    if group=='binary':
        for field,case in {(r['field'],r['case'])for r in rows}:
            median=lambda m:statistics.median(by[(field,case,m)])
            ratios.append(min(median(m)for m in ['direct','radical','initial'])/median('dispatch_initial'))
summary=json.loads((C/'SUMMARY.json').read_text())['binary'];geo=math.exp(sum(map(math.log,ratios))/len(ratios));assert math.isclose(geo,summary['specialized_kernel_geomean_control_over_candidate'],rel_tol=1e-12)
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed'
print('C108 hashes and all',checked,'cold root/multiplicity rows independently verified; specialized ratio',geo,'does not establish a qualified original-system speedup.')
