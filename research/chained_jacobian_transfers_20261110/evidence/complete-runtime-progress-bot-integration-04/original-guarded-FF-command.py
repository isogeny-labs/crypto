"""Exact direct-parent metadata FF with whole current byte/index/raw custody."""
from pathlib import Path
from datetime import datetime,timezone
import os,subprocess,json,hashlib,shutil,base64
r=Path('/workspace/crypto');source='2d54889ac1032d59fe8b5410ad26ff705cec5bd7';bot='16929c6c52f2f8b1510cac524e227ea7449ba0c2'
control=Path('/dev/shm/ct280-root-genuine-bot-current-full-guard');control.mkdir()
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==source
old=json.loads(Path('/workspace/scratch/ct131-ledger-immutable-tag-transport-02/exact-source-hold.json').read_text())
paths=set(old['full_guarded_contents'])
scope=['research/chained_jacobian_transfers_20261110','pcc','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.github/workflows/pcc-gate.yml','tools/pcc','tools/artifacts','.artifacts/config.json','.gitignore']
paths.update(p.decode() for p in subprocess.check_output(['git','ls-files','-z','--',*scope],cwd=r).split(b'\0') if p)
paths.update(p.decode() for p in subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z'],cwd=r).split(b'\0') if p)
def row(path):
 p=r/path;assert p.is_file(),path;b=p.read_bytes()
 return {'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'symlink':os.readlink(p)if p.is_symlink()else None}
before={name:row(name) for name in sorted(paths)}
for name in before:
 p=r/name;assert not Path(name).is_absolute() and '..' not in Path(name).parts
 q=control/'whole-current-working-bytes'/name;q.parent.mkdir(parents=True,exist_ok=True)
 prior=Path('/dev/shm/ct244-root-genuine-bot-current-full-guard/whole-current-working-bytes')/name
 if prior.is_file() and hashlib.sha256(prior.read_bytes()).hexdigest()==before[name]['sha256'] and prior.stat().st_size==before[name]['bytes']:
  os.link(prior,q)
 else:q.write_bytes(p.read_bytes())
raws=['research/chained_jacobian_transfers_20261110/evidence/complete-CI-current-bot-integration-03/'+name for name in ['entire-current-hold.json.xz','entire-published-source-ledger.jsonl.xz']]
assert all(name in before and not(r/name).is_symlink()for name in raws)
source_ledger=subprocess.check_output(['git','show',source+':pcc/ledger.jsonl'],cwd=r);(control/'entire-published-source-ledger.jsonl').write_bytes(source_ledger)
shutil.copyfile(r/'.git/index',control/'entire-index-before');shutil.copyfile(r/'.git/index',control/'actual-FF-index')
env=os.environ.copy();env['GIT_INDEX_FILE']=str(control/'actual-FF-index');env['GIT_OPTIONAL_LOCKS']='0';n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
q=subprocess.run(['git','fetch','--no-tags','https://github.com/isogeny-labs/crypto.git',bot],cwd=r,env=env,capture_output=True,timeout=60);(control/'actual-normal-fetch.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0
assert subprocess.check_output(['git','show','-s','--format=%P',bot],cwd=r,env=env,text=True).strip()==source
assert subprocess.check_output(['git','show',bot+':pcc/ledger.jsonl'],cwd=r,env=env)==source_ledger
expected=set(raws)
for raw in raws:expected.update([str(Path(raw).parent/'.artifacts.json'),str(Path(raw).parent/'.gitignore')])
actual=set(subprocess.check_output(['git','diff','--name-only',source,bot],cwd=r,env=env,text=True).splitlines());assert actual==expected
for raw in raws:
 manifest=json.loads(subprocess.check_output(['git','show',bot+':'+str(Path(raw).parent/'.artifacts.json')],cwd=r,env=env))
 assert manifest['files']=={Path(n).name:{'bytes':before[n]['bytes'],'sha256':before[n]['sha256']} for n in raws if Path(n).parent==Path(raw).parent}
(control/'entire-current-hold.json').write_text(json.dumps({'source':source,'genuine_bot':bot,'all_952_prior_keys_retained':True,'whole_current_guarded_contents':before,'whole_raw_payloads':raws,'exact_four_Git_paths':sorted(actual),'working_ledger_is_separate_from_published_source_ledger':True,'at':datetime.now(timezone.utc).isoformat()},indent=2)+'\n')
assert all(row(name)==value for name,value in before.items())
q=subprocess.run(['git','merge','--ff-only',bot],cwd=r,env=env,capture_output=True,timeout=180);(control/'actual-direct-parent-FF.txt').write_bytes(q.stdout+q.stderr);(control/'actual-direct-parent-FF.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()==bot
for raw in raws:
 q=subprocess.run(['git','check-ignore','--no-index','--',raw],cwd=r,env=env,capture_output=True);(control/(Path(raw).name+'.genuine-ignore.txt')).write_bytes(q.stdout+q.stderr);assert q.returncode==0
 target=r/raw;assert not target.exists();shutil.copyfile(control/'whole-current-working-bytes'/raw,target)
shutil.copyfile(control/'actual-FF-index',r/'.git/index');assert (r/'.git/index').read_bytes()==(control/'actual-FF-index').read_bytes()
assert all(row(name)==value for name,value in before.items())
for name,cmd in [('original-immediate-ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',source]),('original-immediate-artifact-policy',['python3','tools/artifacts/artifacts.py','check',source,bot])]:
 q=subprocess.run(cmd,cwd=r,env=env,capture_output=True);(control/(name+'.txt')).write_bytes(q.stdout+q.stderr);(control/(name+'.exit')).write_text(str(q.returncode)+'\n');assert q.returncode==0
assert all(row(name)==value for name,value in before.items())
(control/'hold-closed-successfully.json').write_text(json.dumps({'genuine_bot':bot,'actual_direct_parent_FF':0,'all_prior_952_and_current_keys_unchanged':len(before),'all_whole_raw_restored_under_genuine_ignores':True,'all_owned_working_ledger_queue_metadata_bytes_unchanged':True,'published_source_ledger_byte_identical':True,'canonical_index_byte_identical_to_actual_FF_index':True,'original_ledger_and_artifact_policy':0,'terminal':'closed successfully; whole hold remains immutable','at':datetime.now(timezone.utc).isoformat()},indent=2)+'\n')
print(json.dumps({'actual_FF':bot,'whole_current_keys':len(before),'all_prior952':True,'whole_raw_preserved':2,'terminal':'closed successfully'}))
