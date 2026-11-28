"""Direct-parent original bot FF with complete byte/raw/index custody."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
r=Path('/workspace/crypto');source='a193a1bbffbc5e535cf1e66d4b2a7a19d4708738';bot='032a36cffd7e85e9986f13486c79bb7c3cd31735'
h=Path('/workspace/scratch/ct406-root-whole-source-full-current-guard');h.mkdir()
env=os.environ.copy();env.update(GIT_OPTIONAL_LOCKS='0',GIT_TERMINAL_PROMPT='0')
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()==source
old=json.loads(Path('/workspace/scratch/ct131-ledger-immutable-tag-transport-02/exact-source-hold.json').read_text())
paths=set(old['full_guarded_contents'])
scope=['research/chained_jacobian_transfers_20261110','pcc','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.github/workflows/pcc-gate.yml','.github/workflows/chained-transfer-odd-subgroup-proof-01.yml','.github/workflows/chained-transfer-cubic-constructor-proof-01.yml','tools/pcc','tools/artifacts','.artifacts/config.json','.gitignore','.gitattributes']
paths.update(p.decode()for p in subprocess.check_output(['git','ls-files','-z','--',*scope],cwd=r,env=env).split(b'\0')if p)
paths.update(p.decode()for p in subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z'],cwd=r,env=env).split(b'\0')if p)
def row(name):
 p=r/name;assert p.is_file(),name;b=p.read_bytes()
 return {'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'symlink':os.readlink(p)if p.is_symlink()else None}
before={name:row(name)for name in sorted(paths)}
raws=[
 'research/chained_jacobian_transfers_20261110/evidence/cubic-word-current-own-CI-and-genuine-migration-07/PCC-entire-original-job-log.txt.xz',
 'research/chained_jacobian_transfers_20261110/evidence/cubic-word-current-final-byte-guard-08/entire-current-hold.json.xz',
 'research/chained_jacobian_transfers_20261110/evidence/cubic-word-current-final-byte-guard-08/entire-published-source-ledger.jsonl.xz',
 'research/chained_jacobian_transfers_20261110/evidence/whole-guarded-source-original-object-12/entire-original-guarded-source.tar.xz']
assert all(name in before and not(r/name).is_symlink()for name in raws)
for name,value in before.items():
 p=r/name;q=h/'whole-current-working-bytes'/name;q.parent.mkdir(parents=True,exist_ok=True)
 prior=Path('/dev/shm/ct359-root-word-current-merge-whole-byte-guard/whole-current-working-bytes')/name
 if name==raws[-1]:
  side=Path('/workspace/scratch/ct374-entire-source-payload-byte-sidecar/entire-original-guarded-source.tar.xz')
  assert side.is_file()and not side.is_symlink()and hashlib.sha256(side.read_bytes()).hexdigest()==value['sha256']
  os.link(side,q)
 elif prior.is_file()and prior.stat().st_size==value['bytes']and hashlib.sha256(prior.read_bytes()).hexdigest()==value['sha256']:
  if prior.stat().st_dev==q.parent.stat().st_dev:os.link(prior,q)
  else:q.symlink_to(prior)
 else:shutil.copyfile(p,q)
 assert hashlib.sha256(q.read_bytes()).hexdigest()==value['sha256']and q.stat().st_size==value['bytes']
source_ledger=subprocess.check_output(['git','show',source+':pcc/ledger.jsonl'],cwd=r,env=env)
(h/'entire-published-source-ledger.jsonl').write_bytes(source_ledger)
shutil.copyfile(r/'.git/index',h/'entire-index-before');shutil.copyfile(r/'.git/index',h/'actual-FF-index');env['GIT_INDEX_FILE']=str(h/'actual-FF-index')
q=subprocess.run(['git','fetch','--no-tags','--filter=blob:none','https://github.com/isogeny-labs/crypto.git',bot],cwd=r,env=env,capture_output=True,timeout=60);(h/'entire-actual-normal-fetch.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0
assert subprocess.check_output(['git','show','-s','--format=%P',bot],cwd=r,env=env,text=True).strip()==source
assert subprocess.check_output(['git','show',bot+':pcc/ledger.jsonl'],cwd=r,env=env)==source_ledger
expected=set(raws)
for name in raws:expected.update([str(Path(name).parent/'.artifacts.json'),str(Path(name).parent/'.gitignore')])
actual=set(subprocess.check_output(['git','diff','--name-only',source,bot],cwd=r,env=env,text=True).splitlines());assert actual==expected and len(actual)==10
for parent in {str(Path(name).parent)for name in raws}:
 manifest=json.loads(subprocess.check_output(['git','show',bot+':'+parent+'/.artifacts.json'],cwd=r,env=env))
 assert manifest['files']=={Path(name).name:{'bytes':before[name]['bytes'],'sha256':before[name]['sha256']}for name in raws if str(Path(name).parent)==parent}
(h/'entire-current-hold.json').write_text(json.dumps({'source':source,'genuine_bot':bot,'all_952_prior_keys_retained':True,'whole_current_guarded_contents':before,'whole_raw_payloads':raws,'exact_ten_metadata_Git_paths':sorted(actual),'working_ledger_distinct_from_published_ledger':True,'time':datetime.now(timezone.utc).isoformat(),'whole_byte_backups':'all complete bytes retained; immutable old RAM holds referenced explicitly; whole14MB payload regular same-filesystem hardlink to complete sidecar'},indent=2)+'\n')
assert all(row(name)==value for name,value in before.items())
q=subprocess.run(['git','merge','--ff-only',bot],cwd=r,env=env,capture_output=True,timeout=180);(h/'entire-actual-direct-parent-FF.txt').write_bytes(q.stdout+q.stderr);(h/'actual-direct-parent-FF.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
for name in raws:
 q=subprocess.run(['git','check-ignore','--no-index','--',name],cwd=r,env=env,capture_output=True);(h/(Path(name).name+'.genuine-ignore.txt')).write_bytes(q.stdout+q.stderr);assert q.returncode==0
 target=r/name;assert not target.exists()and not target.is_symlink();os.link(h/'whole-current-working-bytes'/name,target)
shutil.copyfile(h/'actual-FF-index',r/'.git/index');assert(r/'.git/index').read_bytes()==(h/'actual-FF-index').read_bytes()
assert all(row(name)==value for name,value in before.items())
for label,cmd in [('original-current-parent-ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',source]),('original-current-artifact-policy',['python3','tools/artifacts/artifacts.py','check',source,bot])]:
 q=subprocess.run(cmd,cwd=r,env=env,capture_output=True,timeout=120);(h/(label+'.stdout')).write_bytes(q.stdout);(h/(label+'.stderr')).write_bytes(q.stderr);(h/(label+'.exit')).write_text(str(q.returncode)+'\n');assert q.returncode==0
assert all(row(name)==value for name,value in before.items())
(h/'hold-closed-successfully.json').write_text(json.dumps({'time':datetime.now(timezone.utc).isoformat(),'actual_genuine_bot_direct_parent_FF':bot,'all_current_whole_byte_symlink_records_unchanged':len(before),'all952_prior_keys':True,'all4_whole_raw_payloads_restored_under_genuine_ignores':True,'whole14MB_source_payload_regular_hardlink_retained':True,'working_ledger_and_unpublished_both_proof_components_unchanged':True,'canonical_index_equals_actual_FF_index':True,'original_prefix_artifact_policy':0,'terminal':'closed successfully; whole hold retained'},indent=2)+'\n')
print(json.dumps({'actual_genuine_FF':bot,'all_current_keys':len(before),'all952_prior':True,'whole_raw_restored':4,'terminal':'closed successfully'}),flush=True)
