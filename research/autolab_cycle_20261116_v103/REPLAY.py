from pathlib import Path
import json,hashlib,base64,tarfile,io,sys,runpy
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
runpy.run_path(str(ROOT/'research/autolab_cycle_20261116_v102/REPLAY.py'),run_name='__main__')
saved={};index=json.loads((C/'RAW_INDEX.json').read_text())
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
    assert set(t.getnames())==set(index)
    for name,h in index.items():
        b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
        if name.startswith(('research/table_relations_20261116_v103/','research/relation_zero_locus_20261116_v103b/')):
            p=ROOT/name
            if p.exists():assert p.read_bytes()==b
            else:p.parent.mkdir(parents=True,exist_ok=True);p.open('xb').write(b)
for f in ['REGISTERED_ORACLES_PLAN.json','REGISTERED_PROFILE_PLAN.json','REGISTERED_ZERO_LOCUS_PLAN.json']:
    for name,h in json.loads((C/f).read_text())['expected_sha256'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
oracle=json.loads(saved['research/table_relations_20261116_v103/oracles_raw.json']);profile=json.loads(saved['research/table_relations_20261116_v103/profile_raw.json']);zero=json.loads(saved['research/relation_zero_locus_20261116_v103b/profile_raw.json'])
assert oracle['error']is None and oracle['out']['boolean_cases']==48 and oracle['out']['polynomials_exhausted']==9056
assert profile['error']is None and zero['error']is None
from research.table_relations_20261116_v103.kernel import derive,features,accepted
from research.residue_tables_20261116_v102b.kernel import allowed,multiplication,literal_split
from research.relation_zero_locus_20261116_v103b.kernel import zero_locus,table_mask
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.residue_quotient_20261116_v99b.kernel import rem
D=json.loads((ROOT/'research/table_relations_20261116_v103/DATA.json').read_text());rows=matrix(ROOT/D['matrix'])
for row,z in zip(profile['out']['fields'],zero['out']['fields']):
    s,m=row['s'],row['m'];g=multiplication(s,m);trace=rem(rows[0][4],m);table=sorted(allowed(s,trace,g))
    for degree in row['degrees']:
        masks,basis,rank,_=derive(table,4*s,degree['degree'],D['feature_cap']);assert basis==degree['relations']and rank==degree['rank']
    masks=features(4*s,3);truth,_=zero_locus(4*s,masks,row['degrees'][1]['relations'],536870912);positive=table_mask(4*s,table)
    assert not(positive&~truth)and truth.bit_count()==z['cut_zero_count']and (truth&~positive).bit_count()==z['extraneous_count']
    assert hashlib.sha256(truth.to_bytes(((1<<(4*s))+7)//8,'little')).hexdigest()==z['truth_sha256']
    assert z['extraneous_count']==(496 if s==5 else 0)
    for ex in z['extraneous_examples']:assert not literal_split(ex['H'],1<<s,g)and accepted(ex['code'],masks,row['degrees'][1]['relations'])
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed'
print('C103 all raw/source hashes, complete local vanishing spaces and exhaustive cubic zero sets replayed. GF32 retains496 nonsplit extras per residue; singular-family equality and global all-roots/speed goals remain open.')
