from pathlib import Path
import hashlib,json,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];m=json.loads((h/'raw-meta.json').read_text());r=json.loads((h/'results.json').read_text())
for line in(h/'source.sha256').read_text().splitlines():
 digest,name=line.split(maxsplit=1);p=(h/name).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==digest
 blob=subprocess.check_output(['git','show',m['source_revision']+':'+str(p.relative_to(repo))],cwd=repo);assert hashlib.sha256(blob).hexdigest()==digest
for row in r['runs']:
 p=h/'runs'/row['attempt']
 if row['status']=='accepted':
  check=subprocess.run(['sha256sum','-c',str(p/'source.sha256')],capture_output=True,text=True);assert check.returncode==0
 if row['kind']=='native' and row['status']=='accepted':assert row['native_exit']==0 and row['build_exit']==0
 if row['kind']=='Lean' and row['status']=='accepted':assert '0 failing, 0 trusted' in row['audit_log']
assert r['split']['gate'] and r['comparison']['coordinates_recover']
assert len(r['old_line_comparisons'])==22
failure=r['failed_launches']['lean48-01'];old=(h/failure['retained_runner']).read_bytes();assert hashlib.sha256(old).hexdigest()==failure['runner_sha256']
blob=subprocess.check_output(['git','show',failure['source_revision']+':'+str((h/'REPLAY_LEAN.sh').relative_to(repo))],cwd=repo);assert blob==old
v=json.loads((h/'artifact-verification.json').read_text());assert v['expected_sha256']==m['raw_sha256'] and v['all_member_hashes_verified'] and v['deterministic_repack_verified']
(h/'publication-validation.json').write_text(json.dumps({'source_pins_verified':m['source_pins'],'raw_files_verified':v['regular_files_verified'],'accepted_attempt_source_hashes_verified':True,'rejected_launcher_historical_bytes_verified':True,'terminal_receipts_checked':True,'independent_scientific_replay':False,'independent_semantic_review':False},indent=2)+'\n')
print('Publication source hashes, terminal receipts and fresh archive checks pass')
