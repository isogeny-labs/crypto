from pathlib import Path
import json,hashlib,sys,io,tarfile,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));idx=json.loads((C/'RAW_INDEX.json').read_text());files={}
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as t:
 assert set(t.getnames())==set(idx)
 for n,h in idx.items():
  b=t.extractfile(n).read();assert hashlib.sha256(b).hexdigest()==h;files[n]=b
for n,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
def raw(n):return json.loads(files[n])
g=raw('research/residue_growth_20261116_v100/raw.json');assert g['error']is None;assert [r['active_quotient_bits']for r in g['out']['profiles']]==[12,32,52,71,72,72,72]and not any(r['allocated_dense_states']for r in g['out']['profiles'])
n=raw('research/local_nilpotency_20261116_v100/raw.json');assert n['error']is None and len(n['out']['tiny'])==10 and sum(r['cases']for r in n['out']['tiny'])==4808 and n['out']['repeated_shortcut_counterexamples']==106 and len(n['out']['large'])==6
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,predicate
D=json.loads((ROOT/'research/local_nilpotency_20261116_v100/DATA.json').read_text())
for spec,row in zip(D['large'],n['out']['large']):
 f=Arithmetic(spec['p'],spec['mod']);assert predicate(row['positive'],f)[0]and not predicate(row['negative'],f)[0]
P='research/local_split_circuit_20261116_v100';r=raw(P+'/raw.json');assert r['error']is None;r=r['out'];assert r['tiny_checks']==1056 and r['actual_assignments']==16 and r['cnf_variables']==51096 and r['cnf_clauses']==186935 and not r['solver_called']
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
G=raw(P+'/graph.json');c=Circuit(76);c.nodes=[tuple(n)for n in G['nodes']];clauses=cnf(c,G['outputs']);text='p cnf '+str(len(c.nodes)-2)+' '+str(len(clauses))+'\n'+''.join(' '.join(map(str,v))+' 0\n'for v in clauses);assert text.encode()==files[P+'/formula.cnf']and hashlib.sha256(text.encode()).hexdigest()==r['cnf_sha256'];definition_count=sum(3 if n[0]=='and'else 4 if n[0]=='xor'else 2 if n[0]=='not'else 0 for n in c.nodes)
for v in[0,1,(1<<76)-1]:
 values=c.evaluate(v);assert all(any(bool(values[abs(x)+1])==(x>0)for x in clause)for clause in clauses[:definition_count])
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed';print('C100 hashes,rank/memorypreflight,large-fieldnilpotencycontrols,andexactBooleanCNFbinding verified. NoSATsearch/original-rootcompletion.')
