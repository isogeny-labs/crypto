from pathlib import Path
import json,hashlib,base64,tarfile,io,sys,runpy
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
runpy.run_path(str(ROOT/'research/autolab_cycle_20261116_v104/REPLAY.py'),run_name='__main__')
saved={};index=json.loads((C/'RAW_INDEX.json').read_text())
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
    assert set(t.getnames())==set(index)
    for name,h in index.items():
        b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
        if name.startswith(('research/trace_zero_predicate_20261116_v105/','research/predicate_dispatch_20261116_v105b/')):
            p=ROOT/name
            if p.exists():assert p.read_bytes()==b
            else:p.parent.mkdir(parents=True,exist_ok=True);p.open('xb').write(b)
missing=set()
for f in ['REGISTERED_ORACLES_PLAN.json','REGISTERED_COMPILE_PLAN.json','REGISTERED_DISPATCH_PLAN.json']:
    for name,h in json.loads((C/f).read_text())['expected_sha256'].items():
        p=ROOT/name
        if name.startswith('vendor/z3_c101/')and not p.exists():missing.add(name);continue
        assert hashlib.sha256(p.read_bytes()).hexdigest()==h,name
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
from research.trace_zero_predicate_20261116_v105.kernel import normalization,inverse,translate_poly,field_isomorphism,compile_all
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.table_relations_20261116_v103.kernel import derive
from research.relation_cover_20261116_v104b.kernel import separating_subset
from research.local_nilpotency_20261116_v100.kernel import Arithmetic
D=json.loads((ROOT/'research/trace_zero_predicate_20261116_v105/DATA.json').read_text());templates=json.loads(saved['research/trace_zero_predicate_20261116_v105/templates.json'])
for row in templates['fields']:
    s,m=row['s'],row['m'];q=1<<s;G=multiplication(s,m);f=Arithmetic(2,[(m>>i)&1 for i in range(s+1)]);records=[]
    for t in range(q):
        for v in [0]+[1<<j for j in range(4*s)]:
            h=[v>>(s*i)&(q-1)for i in range(4)];g=normalization(h,t,G);assert inverse(g,t,G)==h and translate_poly(h+[t,1],t,f)==g+[0,1]
    for degree in [2,3]:
        masks,basis,_,_=derive(sorted(allowed(s,0,G)),4*s,degree);records.append({'degree':degree,'relations':basis})
    selected,_,_=separating_subset(4*s,records,D['truth_storage_cap_bytes']);assert selected==row['relations']
for s,m in D['fields']:
    canonical=next(r['m']for r in templates['fields']if r['s']==s);mapping=field_isomorphism(s,m,canonical);a=multiplication(s,m);b=multiplication(s,canonical)
    for x in range(1<<s):
        for y in range(1<<s):assert mapping[x^y]==mapping[x]^mapping[y]and mapping[a[x][y]]==b[mapping[x]][mapping[y]]
from research.predicate_dispatch_20261116_v105b.kernel import compile_all as dispatch
graphs={}
for folder,fn in [('trace_zero_predicate_20261116_v105',compile_all),('predicate_dispatch_20261116_v105b',dispatch)]:
    d=json.loads((ROOT/'research'/folder/'DATA.json').read_text());c,outputs,_=fn(d);raw=json.dumps({'nodes':c.nodes,'outputs':outputs},separators=(',',':')).encode();graphs[folder]=hashlib.sha256(raw).hexdigest();p=ROOT/'research'/folder/'graph.json'
    if p.exists():assert p.read_bytes()==raw
    else:p.open('xb').write(raw)
for name,h in json.loads((C/'OMITTED_GRAPH_HASHES.json').read_text()).items():assert graphs['predicate_dispatch_20261116_v105b'if 'dispatch' in name else 'trace_zero_predicate_20261116_v105']==h,name
result=json.loads(saved['research/predicate_dispatch_20261116_v105b/search_raw.json']);assert result['error']=='nativeunknown'and result['out']['search_executed']and not result['out']['models']and result['out']['clauses']==179895
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed'
print('C105 template normalization, field isomorphisms, exact graph/source hashes and preserved UNKNOWN replayed. Native rerun needs omitted pinned files:',len(missing),'; no original completion or qualified speed ratio.')
