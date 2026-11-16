"""Validate genuine published bot, normal FF and whole immutable metadata hold."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,os,re,shutil,subprocess,tarfile,zipfile
r=Path('/workspace/crypto');old=Path('/workspace/scratch/ct131-ledger-immutable-tag-transport-02');hold=json.loads((old/'exact-source-hold.json').read_text());clone=Path(hold['actual_clone']);source=hold['source']
raw=Path('/dev/shm/ct143-root-transport02-whole-artifact-01');a=raw/'all-members';out=Path('/dev/shm/ct144-root-ledger-transport02-terminal-01');out.mkdir();shutil.copyfile(__file__,out/'original-command.py')
bot=(a/'actual-bot-head.txt').read_text().strip();tag=(a/'actual-new-tag-ref.txt').read_text().strip();assert re.fullmatch('[0-9a-f]{40}',bot)and re.fullmatch('refs/tags/chained-transfer-ledger-metadata02-[0-9]+-[0-9]+',tag)
assert source=='c019950bd36d74bbfa379d346c3943dec591a4df'and bot=='532c2d32e475f81e2562315c269faf3532fc9e4a'
assert all((a/n).read_text().strip()=='0'for n in ['original-migrate.exit','actual-new-tag-push.exit','original.exit','original-prefetch.exit'])
assert not(a/'tag-before.txt').read_bytes()and(a/'tag-after.txt').read_text().split()==[bot,tag]
assert(a/'entire-original-source-ledger.jsonl').read_bytes()==(a/'entire-actual-bot-ledger.jsonl').read_bytes()==(old/'entire-source-ledger.jsonl').read_bytes()
payload=hold['whole_regular_payloads'][0];payload_path=payload['path'];directory=Path(payload_path).parent.as_posix()
assert(a/'entire-original-payload.tar.xz').read_bytes()==Path(payload['local_original']).read_bytes()
assert hashlib.sha256((a/'entire-original-payload.tar.xz').read_bytes()).hexdigest()==payload['sha256']
with zipfile.ZipFile(raw/'whole-original-transport02.zip')as z:
 assert z.testzip()is None and len(z.infolist())==34
 members=[dict(path=i.filename,bytes=i.file_size,sha256=hashlib.sha256(z.read(i.filename)).hexdigest())for i in z.infolist()]
 assert all((a/v['path']).read_bytes()==z.read(v['path'])for v in members)
log=Path('/workspace/scratch/ct143-root-transport02-whole-job.log').read_text();remote_position=log.index('artifacts migrate: 1 remote objects replayed');commit_position=log.index('[detached HEAD '+bot[:10]+'] Move PR data files');assert remote_position<commit_position
workspace_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r);workspace_status=subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=r)
(out/'ongoing-workspace-HEAD.before').write_bytes(workspace_head);(out/'ongoing-workspace-status.before').write_bytes(workspace_status)
env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
def run(name,command,want=0,input=None):
 q=subprocess.run(command,cwd=clone,env=env,input=input,capture_output=True,timeout=180)
 (out/(name+'.stdout')).write_bytes(q.stdout);(out/(name+'.stderr')).write_bytes(q.stderr);(out/(name+'.exit')).write_text(str(q.returncode)+'\n');(out/(name+'.command.json')).write_text(json.dumps(command)+'\n');print(name,q.returncode,flush=True);assert q.returncode==want;return q
def held(expected):
 assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=clone,text=True).strip()==expected
 assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=clone)
 for name,row in hold['full_guarded_contents'].items():
  p=clone/name;assert p.is_file()and p.stat().st_size==row['bytes']and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'],name
 assert hashlib.sha256((clone/'pcc/ledger.jsonl').read_bytes()).hexdigest()==hold['entire_source_ledger_sha256']
 assert(clone/payload_path).read_bytes()==Path(payload['local_original']).read_bytes()
held(source);print('all952-source-files-and-ledger-payload-held0',flush=True)
tree=run('whole-original-source-tree',['git','ls-tree','-r','-z','HEAD']).stdout
manifests={}
for line in tree.split(b'\0'):
 if not line:continue
 info,name=line.split(b'\t',1);mode,kind,oid=info.split();name=name.decode()
 if name=='.artifacts.json'or name.endswith('/.artifacts.json'):
  assert mode in [b'100644',b'100755']and kind==b'blob'and'\n'not in name
  manifests[name]=oid.decode()
assert len(manifests)==3295
(out/'all3295-original-manifest-Git-blobs.json').write_text(json.dumps(manifests,indent=2)+'\n')
patterns=(clone/'.git/info/sparse-checkout').read_bytes();(out/'whole-original-sparse-patterns.txt').write_bytes(patterns)
expanded=patterns.rstrip(b'\n')+b'\n'+b''.join(('/'+name+'\n').encode()for name in sorted(manifests))
run('materialize-all-original-manifests-before-complete-wiring',['git','sparse-checkout','set','--no-cone','--stdin'],input=expanded)
(out/'whole-expanded-sparse-patterns.txt').write_bytes((clone/'.git/info/sparse-checkout').read_bytes())
def original_manifests():
 for name,oid in manifests.items():
  b=(clone/name).read_bytes();assert hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()==oid,name
original_manifests();held(source)
run('actual-bot-bundle-verify',['git','bundle','verify',str(a/'actual-bot-with-original-source-prerequisite.bundle')])
before=run('actual-published-tag-before-fetch',['git','ls-remote','--tags','origin',tag]);assert before.stdout.decode().split()==[bot,tag]
run('normal-published-new-tag-fetch',['git','fetch','--no-tags','origin',tag+':'+tag])
assert subprocess.check_output(['git','rev-parse',tag],cwd=clone,text=True).strip()==bot
assert subprocess.check_output(['git','rev-parse',bot+'^'],cwd=clone,text=True).strip()==source
assert subprocess.check_output(['git','cat-file','commit',bot],cwd=clone)==(a/'entire-actual-bot-commit.txt').read_bytes()
assert subprocess.check_output(['git','show',bot+':pcc/ledger.jsonl'],cwd=clone)==(a/'entire-original-source-ledger.jsonl').read_bytes()
changed=run('actual-published-bot-paths',['git','diff','--no-renames','--name-only','-z',source,bot]).stdout
expected={payload_path,directory+'/.artifacts.json',directory+'/.gitignore'}
assert set(v.decode()for v in changed.split(b'\0')if v)==expected
assert b'\0'.join(sorted(v for v in changed.split(b'\0')if v))+b'\0'==(a/'expected-three-paths.paths0').read_bytes()
for name,original in [(directory+'/.artifacts.json','actual-original-generated-artifacts.json'),(directory+'/.gitignore','actual-original-generated-gitignore.txt')]:assert subprocess.check_output(['git','show',bot+':'+name],cwd=clone)==(a/original).read_bytes()
held(source);original_manifests()
run('actual-normal-fast-forward',['git','merge','--ff-only',bot])
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=clone,text=True).strip()==bot
assert not(clone/payload_path).exists()
ignore=run('original-generated-ignore-before-whole-restore',['git','check-ignore','--no-index',payload_path]);assert ignore.stdout.decode().strip()==payload_path
shutil.copyfile(a/'entire-original-payload.tar.xz',clone/payload_path)
assert not(clone/payload_path).is_symlink()
held(bot);original_manifests()
manifest=json.loads((clone/directory/'.artifacts.json').read_text());assert manifest==dict(schema='artifacts.manifest/v1',files={Path(payload_path).name:dict(sha256=payload['sha256'],bytes=payload['bytes'])})
assert(clone/directory/'.artifacts.json').read_bytes()==(a/'actual-original-generated-artifacts.json').read_bytes()
assert(clone/directory/'.gitignore').read_bytes()==(a/'actual-original-generated-gitignore.txt').read_bytes()
run('original-artifact-check',['python3','tools/artifacts/artifacts.py','check',source,bot])
verified=run('original-complete-local-artifact-verify',['python3','tools/artifacts/artifacts.py','verify',directory]);assert b'1 entries: 1 hydrated, 0 not pulled, 0 problems'in verified.stderr+verified.stdout
run('original-full-manifest-workflow-wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
run('original-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',source])
held(bot);original_manifests()
after=run('actual-published-tag-after-terminal',['git','ls-remote','--tags','origin',tag]);assert after.stdout.decode().split()==[bot,tag]
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r)==workspace_head
assert subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=r)==workspace_status
science=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(science)==343 and all(hashlib.sha256((r/n).read_bytes()).hexdigest()==h for n,h in science.items())
receipt=dict(at=datetime.now(timezone.utc).isoformat(),source=source,actual_bot=bot,actual_published_tag=tag,actual_normal_FF=0,all952_original_keys_retained_byte_exact=True,all3295_original_manifest_blobs_retained=True,whole_source_ledger_SHA256=hold['entire_source_ledger_sha256'],all343_ongoing_science_unchanged=True,whole_regular_payload_restored_only_after_genuine_ignore=True,whole_original_payload=payload,whole_artifact=dict(path=str(raw/'whole-original-transport02.zip'),bytes=1111167,sha256='345d3f23e43a84dc5819e39f0abf0965e4be7e95908e7ba25c9e53d9e917ad12',every_member=members),original_replay_before_real_bot=True,actual_remote_replays=1,new_uploads=0,original_artifact_check_verify_wiring_ledger=0,complete_old_manifest_view_used_for_wiring=True,ongoing_workspace_HEAD_and_status_unchanged=True,held_clone=str(clone),scope='unsigned successful immutable metadata-sidecar custody only; full scientific NEW28 and all global PCC/merge gates separate; no scientific or signed promotion')
(out/'whole-success-terminal-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');(old/'hold-closed-successfully-02.json').write_text(json.dumps(dict(at=receipt['at'],source=source,bot=bot,terminal_receipt=str(out/'whole-success-terminal-receipt.json'),status='closed successful for immutable metadata sidecar only',future='Preserve whole source/payload/raw. Any integration into ongoing metadata branch is an ordinary merge, not that isolated clone FF.'),indent=2)+'\n');print('TERMINAL_METADATA02_CLOSED0',bot,flush=True)
