from pathlib import Path
import json,hashlib,base64,tarfile,io,sys,runpy
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
runpy.run_path(str(ROOT/'research/autolab_cycle_20261116_v103/REPLAY.py'),run_name='__main__')
saved={};index=json.loads((C/'RAW_INDEX.json').read_text())
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
    assert set(t.getnames())==set(index)
    for name,h in index.items():
        b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
        if name.startswith(('research/singular_quadratic_guard_20261116_v104/','research/relation_cover_20261116_v104b/')):
            p=ROOT/name
            if p.exists():assert p.read_bytes()==b
            else:p.parent.mkdir(parents=True,exist_ok=True);p.open('xb').write(b)
for f in ['REGISTERED_ORACLES_PLAN.json','REGISTERED_COMPILE_PLAN.json','REGISTERED_COVER_PLAN.json']:
    for name,h in json.loads((C/f).read_text())['expected_sha256'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
from research.singular_quadratic_guard_20261116_v104.kernel import compile_all,bad_quadratic
from research.relation_cover_20261116_v104b.kernel import separating_subset
from research.relation_zero_locus_20261116_v103b.kernel import zero_locus,table_mask
from research.table_relations_20261116_v103.kernel import features
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import rem
D=json.loads((ROOT/'research/singular_quadratic_guard_20261116_v104/DATA.json').read_text());rows=matrix(ROOT/D['matrix']);prior=json.loads((ROOT/D['relations_raw']).read_text());cover=json.loads(saved['research/relation_cover_20261116_v104b/profile_raw.json']);assert cover['error']is None
for row in prior['out']['fields']:
    s,m=row['s'],row['m'];q=1<<s;g=multiplication(s,m);trace=rem(rows[0][4],m);bad=set()
    for u in range(q):
        for v in range(q):
            split=any(g[x][x]^g[u][x]^v==0 for x in range(q));a,c=g[u][u],g[v][v];assert bad_quadratic(a,c,s,g)==(not split)
            if not split:bad.add(g[trace][c]|(c<<s)|(g[trace][a]<<(2*s))|(a<<(3*s)))
    selected,stats,truth=separating_subset(4*s,row['degrees'],D['truth_storage_cap_bytes']);table=table_mask(4*s,allowed(s,trace,g));full,_=zero_locus(4*s,features(4*s,3),row['degrees'][1]['relations'],D['truth_storage_cap_bytes']);assert truth==full
    if s==5:assert (truth&~table)==table_mask(4*s,bad)and truth&~table_mask(4*s,bad)==table
    else:assert truth==table
    actual=json.loads(saved['research/relation_cover_20261116_v104b/selected.json']);r=next(r for r in actual['out']['fields']if r['s']==s and r['m']==m);assert r['degrees'][0]['relations']==selected
generated={}
for folder in ['singular_quadratic_guard_20261116_v104','relation_cover_20261116_v104b']:
    d=json.loads((ROOT/'research'/folder/'DATA.json').read_text())
    if folder.startswith('relation_cover'):d['relations_raw']=str(ROOT/'research'/folder/'selected.json')
    c,out,_=compile_all(d);b=json.dumps({'nodes':c.nodes,'outputs':out},separators=(',',':')).encode();name='research/'+folder+'/graph.json';generated[name]=hashlib.sha256(b).hexdigest();p=ROOT/name
    if p.exists():assert p.read_bytes()==b
    else:p.open('xb').write(b)
for name,receipt in json.loads((C/'OMITTED_RECONSTRUCTIBLE.json').read_text()).items():
    match='relation_cover_20261116_v104b'if 'relation-cover' in name or 'relation_cover' in name else 'singular_quadratic_guard_20261116_v104'
    assert generated['research/'+match+'/graph.json']==receipt['sha256'],name
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed'
print('C104 full singular-family certificates, trace oracles, exact separating subsets and both graph hashes replayed. No SAT run or original all-roots/performance claim.')
