from pathlib import Path
import json,hashlib,runpy,sys,base64,tarfile,io
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
def archive(folder):
    d=json.loads((folder/'ARCHIVE_PARTS.json').read_text());parts=[]
    for part in d['parts']:
        b=(folder/part['path']).read_bytes();assert len(b)==part['bytes']and hashlib.sha256(b).hexdigest()==part['sha256'];parts.append(b)
    b=b''.join(parts);assert hashlib.sha256(b).hexdigest()==d['aggregate_sha256']
    p=folder/'RAW_EVIDENCE.tar.xz.b64'
    if p.exists():assert p.read_bytes()==b
    else:p.open('xb').write(b)
    return b
archive(C)
old=archive(ROOT/'research/autolab_cycle_20261116_v100')
graph_path='research/local_split_circuit_20261116_v100/graph.json'
with tarfile.open(fileobj=io.BytesIO(base64.b64decode(old)),mode='r:xz')as t:b=t.extractfile(graph_path).read()
assert hashlib.sha256(b).hexdigest()=='343dc0f003f65258028e8ad0a7ac9da6069fe1ebbd319ed65b63d93356aac174'
p=ROOT/graph_path
if p.exists():assert p.read_bytes()==b
else:p.parent.mkdir(parents=True,exist_ok=True);p.open('xb').write(b)
from research.guarded_local_search_20261116_v101.kernel import guarded,matrix
from research.local_split_circuit_20261116_v100.kernel import cnf
D=json.loads((ROOT/'research/guarded_local_search_20261116_v101/DATA.json').read_text())
c,outputs,delta=guarded(str(p),matrix(ROOT/D['matrix']));clauses=cnf(c,outputs)
text='p cnf '+str(len(c.nodes)-2)+' '+str(len(clauses))+'\n'+''.join(' '.join(map(str,v))+' 0\n'for v in clauses)
assert hashlib.sha256(text.encode()).hexdigest()=='1d459da6ca814dc462d2030bb293398f6dadafe88e32538424b4997c1429f3f9'
target=ROOT/'research/guarded_local_search_20261116_v101/guarded.cnf'
if target.exists():assert target.read_text()==text
else:target.open('x').write(text)
assert json.loads(json.dumps(delta))==json.loads((ROOT/'research/guarded_local_search_20261116_v101/guard_delta.json').read_text())
missing_native=[]
for f in ['REGISTERED_ORACLE_PLAN.json','REGISTERED_SEARCH_PLAN.json']:
    for name,h in json.loads((C/f).read_text())['expected_sha256'].items():
        path=ROOT/name
        if name.startswith('vendor/z3_c101/')and not path.exists():missing_native.append(name);continue
        assert hashlib.sha256(path.read_bytes()).hexdigest()==h,name
runpy.run_path(str(C/'REPLAY_V2.py'),run_name='__main__')
print('C101 exact base graph, guard delta and CNF reconstructed; source hashes checked; native files omitted:',len(set(missing_native)))
