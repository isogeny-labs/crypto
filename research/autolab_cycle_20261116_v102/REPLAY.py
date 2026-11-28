from pathlib import Path
import json,hashlib,base64,tarfile,io,sys,runpy,random
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
runpy.run_path(str(ROOT/'research/autolab_cycle_20261116_v101/REPLAY_PORTABLE_V3.py'),run_name='__main__')
index=json.loads((C/'RAW_INDEX.json').read_text());saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
    assert set(t.getnames())==set(index)
    for name,h in index.items():
        b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
missing_native=set()
for plan in ['REGISTERED_ORACLES_PLAN.json','REGISTERED_SEARCH_PLAN.json','REGISTERED_TABLE_PLAN.json']:
    for name,h in json.loads((C/plan).read_text())['expected_sha256'].items():
        p=ROOT/name
        if name.startswith('vendor/z3_c101/')and not p.exists():missing_native.add(name);continue
        assert hashlib.sha256(p.read_bytes()).hexdigest()==h,name
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
search=json.loads(saved['research/xor_first_20261116_v102/search_raw.json']);assert search['error']=='nativeunknown'and search['out']['last_answer']=='unknown'and not search['out']['models']and not search['out']['original_complete']
assert search['out']['profile']['affine_terms']==335396
tables=json.loads(saved['research/residue_tables_20261116_v102b/profile_raw.json']);assert tables['error']is None and tables['out']['total_bdd_nodes']==82643
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.xor_first_20261116_v102.kernel import atoms,quotient,accepts,restore
rng=random.Random(1021611)
for case in range(72):
    n=1+case%5;c=Circuit(n)
    for i in range(12):
        k=['xor','not','and'][i%3];a=rng.randrange(len(c.nodes));b=rng.randrange(len(c.nodes));c.op(k,a,None if k=='not'else b)
    zero=[rng.randrange(len(c.nodes))for _ in range(1+case%3)]
    if case%9==0:zero=[1]
    if case%9==1:zero=[0]
    forms,products,count=atoms(c.nodes,n);q=quotient(count,products,[forms[w]for w in zero]);expected={v for v in range(1<<n)if not any(c.evaluate(v)[w]for w in zero)};observed=[]
    for v in range(1<<len(q['free'])):
        assignment=sum(((v>>i)&1)<<j for i,j in enumerate(q['free']))
        if accepts(q,assignment):observed.append((restore(q,assignment)>>1)&((1<<n)-1))
    assert set(observed)==expected and len(observed)==len(expected)
from research.residue_tables_20261116_v102b.kernel import multiplication,allowed,diagram,evaluate,literal_split
from research.residue_quotient_20261116_v99b.kernel import rem
from research.guarded_local_search_20261116_v101.kernel import matrix
D=json.loads((ROOT/'research/residue_tables_20261116_v102b/DATA.json').read_text());rows=matrix(ROOT/D['matrix'])
for row in tables['out']['fields']:
    s,m=row['s'],row['m'];g=multiplication(s,m);table=allowed(s,rem(rows[0][4],m),g);nodes,root=diagram(4*s,table)
    data=b''.join(v.to_bytes((4*s+7)//8,'little')for v in sorted(table));assert hashlib.sha256(data).hexdigest()==row['table_sha256']and len(nodes)==row['bdd_nodes']
    for v in list(sorted(table))[:8]:assert evaluate(nodes,root,v)and literal_split([(v>>(s*i))&((1<<s)-1)for i in range(4)]+[row['trace'],1],1<<s,g)
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed'
print('C102 exact hashes, exhaustive quotient projection and fourteen regenerated tables/BDDs passed. Native search rerun needs pinned library files:',len(missing_native),'; original all-roots and qualified performance remain unproved.')
