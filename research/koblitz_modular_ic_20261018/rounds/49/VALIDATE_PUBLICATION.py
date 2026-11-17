from pathlib import Path
import hashlib,json,subprocess,re
h=Path(__file__).resolve().parent;repo=h.parents[3];m=json.loads((h/'raw-meta.json').read_text());r=json.loads((h/'results.json').read_text())
for line in (h/'source.sha256').read_text().splitlines():
 digest,name=line.split(maxsplit=1);p=(h/name).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==digest
 blob=subprocess.check_output(['git','show',m['source_revision']+':'+str(p.relative_to(repo))],cwd=repo);assert hashlib.sha256(blob).hexdigest()==digest
for row in r['runs']:
 p=h/'runs'/row['attempt']
 if row.get('compiler_started') is False:
  assert (p/'gate.log').exists() and not (p/'compile.exit').exists()
  blob=subprocess.check_output(['git','show',row['source_revision']+':'+str((h/'REPLAY_LEAN.sh').relative_to(repo))],cwd=repo)
  assert hashlib.sha256(blob).hexdigest()==row['runner_sha256'];continue
 subprocess.run(['sha256sum','-c',str(p/'source.sha256')],check=True,stdout=subprocess.DEVNULL)
 if row['kind']=='Singular':
  log=(p/'native.log').read_text();ok=int((p/'exit.txt').read_text())==0 and not re.search(r'^ *\?',log,re.M) and row['marker'] in log.splitlines()
  assert ok==(row['status']=='accepted')
 else:
  assert int((p/'compile.exit').read_text())==row['compile_exit']
  if row['status']=='accepted':assert '0 failing, 0 trusted' in (p/'audit.log').read_text()
v=json.loads((h/'artifact-verification.json').read_text());assert v['expected_sha256']==m['raw_sha256'] and v['all_member_hashes_verified'] and v['deterministic_repack_verified']
(h/'publication-validation.json').write_text(json.dumps({'source_pins_verified':m['source_pins'],'raw_files_verified':v['regular_files_verified'],'attempt_source_hashes_verified':True,'checkpoint_rejection_historical_runner_verified':True,'terminal_receipts_checked':True,'independent_scientific_replay':False,'independent_semantic_review':False},indent=2)+'\n')
print('R49 historical source, terminal receipts and fresh archive byte checks pass')
