#!/usr/bin/env python3
"""Read-only retained integrity and publication validation, not math replay."""
from pathlib import Path
import hashlib,json,re
here=Path(__file__).resolve().parent
record=json.loads((here/'results.json').read_text())
for row in record['runs']:
 p=here/'runs'/row['id']
 if row['stage']=='native':
  s=(p/'native.log').read_text();assert int((p/'exit.txt').read_text())==0
  assert s.rstrip().splitlines()[-1]==row['marker'];assert not re.search(r'^\s*\?',s,re.M)
 else:
  assert int((p/'compile.exit').read_text())==0;assert int((p/'audit.exit').read_text())==0
  last=(p/'audit.log').read_text().rstrip().splitlines()[-1]
  assert re.fullmatch(r'audited '+str(row['audited'])+r' theorems in \[.*\]; 0 failing, 0 trusted \(native_decide\)',last)
source_counts={}
for rid in (24,25,26,27,28,29,30):
 p=here.parent/str(rid);n=0
 for line in(p/'source.sha256').read_text().splitlines():
  digest,name=line.split(maxsplit=1);assert hashlib.sha256((p/name.strip()).read_bytes()).hexdigest()==digest,(rid,name);n+=1
 source_counts[str(rid)]=n
assert source_counts=={'24':22,'25':26,'26':31,'27':66,'28':23,'29':28,'30':38}
old=(here/'../29/RECORDER_VALIDATION.md').read_text();expected=json.loads(re.search(r'```json\n(.*?)\n```',old,re.S).group(1));assert len(expected)==4
fresh=Path('/var/tmp/koblitz-r30-fresh-extraction/publication-attempts/r29-recorder')
for name,digest in expected.items():assert hashlib.sha256((fresh/name).read_bytes()).hexdigest()==digest
v=json.loads((here/'artifact-verification.json').read_text());assert v['fresh_GetObject_download'] and v['all_member_hashes_verified'] and v['deterministic_repack_verified'];assert v['regular_files_verified']==102
out={'schema':'koblitz.publication-integrity/v1','accepted':True,'native_runs':5,'proof_runs':2,'current_named_theorems':9,'current_audited_declarations':13,'source_pins_pass':source_counts,'fresh_archive_regular_files':102,'historical_software_logs_match_published_sha256':4,'independent_mathematical_replay':False}
print(json.dumps(out,indent=2))
