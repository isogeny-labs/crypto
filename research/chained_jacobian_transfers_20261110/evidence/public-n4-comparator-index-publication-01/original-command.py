"""Actual original metadata publication and exact terminal held-byte release."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,re,subprocess,sys
from ct108_index_sidecar_guard import intact
r=Path('/workspace/crypto');source='00be7d5432324d2686686fbb57498f3c968fa07a';bot=sys.argv[1]
out=Path('/workspace/scratch/ct109-index-sidecar-artifact-ff-01');out.mkdir();(out/'original-command.py').write_bytes(Path(__file__).read_bytes())
prefix='research/chained_jacobian_transfers_20261110/evidence/public-n4-comparator-index-sidecar-01'
objects=json.loads((r/prefix/'objects.json').read_text());assert len(objects)==2
env=dict(os.environ,GIT_NO_LAZY_FETCH='1')
def git(*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
def run(name,cmd):
 q=subprocess.run(cmd,cwd=r,env=env,capture_output=True,timeout=120);(out/(name+'.txt')).write_bytes(q.stdout+q.stderr);(out/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==0
def guard(label):
 v=intact();assert not git('status','--porcelain=v1','--untracked-files=all');(out/(label+'-full-guard.json')).write_text(json.dumps(v,indent=2)+'\n');print(label,v,flush=True);return v
fetch=json.loads(Path('/workspace/scratch/ct109-index-sidecar-RAM-bot-fetch-01/closed-new-remote-cache.json').read_text())
assert fetch['actual_exit']==0 and fetch['source']==source and fetch['actual_bot']==bot
assert git('rev-parse','HEAD').decode().strip()==source and git('rev-parse',bot+'^').decode().strip()==source
before=guard('before')
log=Path('/workspace/scratch/ct109-index-sidecar-actual-original-publication-01/whole-job.log').read_bytes()
replays=list(re.finditer(rb'artifacts migrate: (\d+) remote objects replayed; staged pointers are ready',log))
commits=[m for m in re.finditer(rb'\[detached HEAD ([0-9a-f]+)\]',log)if bot.startswith(m[1].decode())]
assert len(replays)==1 and replays[0][1]==b'2'and len(commits)==1 and replays[0].start()<commits[0].start()
changed=set(git('diff','--no-renames','--name-only',source,bot).decode().splitlines())
expected={row['path']for row in objects}|{prefix+'/.artifacts.json',prefix+'/.gitignore'};assert changed==expected and len(changed)==4
manifest_bytes=git('show',bot+':'+prefix+'/.artifacts.json');manifest=json.loads(manifest_bytes)
assert manifest['schema']=='artifacts.manifest/v1'and set(manifest['files'])=={Path(row['path']).name for row in objects}
for row in objects:
 data=git('show',source+':'+row['path']);assert len(data)==row['bytes']and hashlib.sha256(data).hexdigest()==row['sha256']
 assert data==Path(row['local_original']).read_bytes()==(r/row['path']).read_bytes()
 assert (r/row['path']).is_file()and not(r/row['path']).is_symlink()
 assert manifest['files'][Path(row['path']).name]==dict(bytes=row['bytes'],sha256=row['sha256'])
 decoded=lzma.decompress(data)
 if Path(row['path']).name=='complete-current-guard.json.xz':assert decoded==Path('/workspace/scratch/ct108-index-sidecar-custody-01/complete-current-guard.json').read_bytes()
 else:assert decoded[:4]==b'DIRC','complete original Git index encoding'
pdf='research/chained_jacobian_transfers_20261110/evidence/public-n4-original-relation-rank-character-handoff-27/original-relation-character-result-01.pdf'
assert git('rev-parse',source+':'+pdf)==git('rev-parse',bot+':'+pdf)
run('exact-ff',['git','merge','--ff-only',bot]);assert git('rev-parse','HEAD').decode().strip()==bot
assert guard('after-ff')==before
run('artifact',['python3','tools/artifacts/artifacts.py','check',source,bot]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
terminal=guard('terminal');assert terminal==before
(out/'canonical-manifest.json').write_bytes(manifest_bytes)
(out/'terminal-release.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=source,bot=bot,original_run=43750430453,original_job=132922943345,actual_original_remote_replays_BEFORE_actual_bot=2,exact_direct_parent=source,exact_changed_paths=sorted(changed),all_two_regular_source_payload_blobs_equal_whole_originals=True,both_entire_XZ_integrity_and_whole_decoded_guard_checked=True,whole_original_index_encoding_preserved=True,exact_two_original_manifest_entries=manifest['files'],all_prior65886_keys179links_retained=True,complete_terminal_guard=terminal,normalFF=0,original_artifact_check=0,original_wiring_check=0,full_original_job_bytes=len(log),full_original_job_sha256=hashlib.sha256(log).hexdigest(),sidecar_hold='RELEASED only after actual original replays before actual bot, exact FF and all terminal source/link/owned/status/ledger guards',scope='unsigned complete immutable metadata custody only; full scientific custody NEW28 and comparator execution remain pending'),indent=2)+'\n')
print('METADATA SIDECAR HOLD RELEASED; original2replays-before-actualbot/exact4paths/full66706-185-538guards/normalFF-artifact-wiring0',flush=True)
