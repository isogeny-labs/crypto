#!/usr/bin/env python3
"""Verify frozen run admission, source hashes, recorded counts and proof audits.
This is an evidence-integrity check, not an independent mathematical replay.
"""
from pathlib import Path
import hashlib,json,re
base=Path(__file__).resolve().parent.parent
repo=base.parents[2]
counts={'accepted_native':0,'native_attempts':0,'proof_named':0,'proof_audited_current':0,'accepted_proof_runs':0}
for rid in [27,28,29]:
 here=base/str(rid);record=json.loads((here/'results.json').read_text())
 for row in record['runs']:
  run=here/'runs'/row['id']
  if row['stage']=='native':counts['native_attempts']+=1
  if row['status']!='accepted':continue
  if row['stage']=='native':
   text=(run/'native.log').read_text();assert int((run/'exit.txt').read_text())==0
   assert not re.search(r'^\s*\?',text,re.M);assert re.search(r'^[A-Z0-9_]+_ALL_CHECKS_PASS$',text,re.M)
   counts['accepted_native']+=1
  elif row['stage']=='proof':
   assert int((run/'audit.exit').read_text())==0
   assert re.search(r'audited \d+ theorems.*0 failing, 0 trusted', (run/'audit.log').read_text())
   counts['accepted_proof_runs']+=1
  hashfile=run/'source.sha256'
  if hashfile.exists():
   for line in hashfile.read_text().splitlines():
    digest,name=line.split(maxsplit=1);name=name.strip();leaf=Path(name).name
    candidates=[run/leaf,run/'source/src'/leaf,run/'source'/leaf,here/'lean'/leaf,base/'27/lean'/leaf,repo/'tools/lean'/leaf]
    if not Path(name).is_absolute():candidates.append(run/name)
    candidates.append(Path(name))
    existing=[p for p in candidates if p.exists() and p.is_file()]
    assert existing,(rid,row['id'],name,'no source candidate')
    assert any(hashlib.sha256(p.read_bytes()).hexdigest()==digest for p in existing),(rid,row['id'],name,'source mismatch')
 current=record['proof'];counts['proof_named']+=current['named_theorems'];counts['proof_audited_current']+=current['audited_declarations']
 named=sum(len(re.findall(r'^theorem\s',p.read_text(),re.M))for p in (here/'lean').glob('*.lean'))
 assert named==current['named_theorems'],(rid,named,current)
quad=(base/'27/runs/quadrics27-03/native.log').read_text()
assert re.search(r'QUADRICS27_COUNTS sourceC27[^\n]*\n68\n2346\n196\n2150\n',quad)
assert re.search(r'QUADRICS27_COUNTS quotientH27[^\n]*\n34\n595\n98\n497\n',quad)
maplog=(base/'27/runs/mapsolve27-02/native.log').read_text()
assert 'checks,failures\n910\n0\n'in maplog
assert 'Singular error: no more memory'in maplog
assert int((base/'27/runs/mapsolve27-02/exit.txt').read_text())==14
assert not(base/'27/runs/rr27-02/exit.txt').exists()
assert re.search(r'^\s*\?',(base/'27/runs/laurent27-corrupt-01/native.log').read_text(),re.M)
r29=json.loads((base/'29/results.json').read_text());deps=r29['image_dependencies']['records']
assert len(deps)==4 and deps[0]['frobenius_offsets_id_neg_T_negT']==[218,-1,-1,-1]
assert deps[1]['frobenius_offsets_id_neg_T_negT']==[6,-1,-1,-1]
assert all(d['frobenius_offsets_id_neg_T_negT']==[-1,-1,-1,-1]for d in deps[2:])
assert counts=={'accepted_native':21,'native_attempts':32,'proof_named':40,'proof_audited_current':41,'accepted_proof_runs':4},counts
print(json.dumps({'schema':'koblitz.retained-integrity/v1','accepted':True,'scope':'run admission, source hashes, recorded native counts and current proof audits; not independent mathematical replay','counts':counts},indent=2))
