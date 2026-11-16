"""Fetch only the actual original publisher; all new remote encodings in RAM."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,os,subprocess
from ct105_hold27_guard import intact,sha
r=Path('/workspace/crypto');control=Path('/workspace/scratch/ct104-whole-closed-science-handoff-27-attempt02');out=Path('/workspace/scratch/ct105-handoff27-RAM-bot-fetch-01');out.mkdir();ram=Path('/dev/shm/ct105-original-bot27-fetch-01/objects');ram.mkdir(parents=True)
source='00800a2b8f20a2812eccbc5200e85a99572b453d';bot='0b2bee203a133568f1c7e9a35bfe3be1a663a879';repo='https://github.com/isogeny-labs/crypto.git'
(out/'original-command.py').write_bytes(Path(__file__).read_bytes())
plain={k:v for k,v in os.environ.items()if not k.startswith('GIT_TRACE')and k!='GIT_CURL_VERBOSE'};plain['GIT_NO_LAZY_FETCH']='1'
def read(*args,environment=plain):return subprocess.check_output(['git',*args],cwd=r,env=environment)
before={key:read(*args)for key,args in [('head',['rev-parse','HEAD']),('refs',['show-ref']),('status',['status','--porcelain'])]};assert before['head'].decode().strip()==source and not before['status'];guard=intact(control)
env=dict(plain,GIT_OBJECT_DIRECTORY=str(ram),GIT_ALTERNATE_OBJECT_DIRECTORIES=str(r/'.git/objects'),GIT_TERMINAL_PROMPT='0')
n=int(env.get('GIT_CONFIG_COUNT','0'));env['GIT_CONFIG_COUNT']=str(n+1);env['GIT_CONFIG_KEY_'+str(n)]='http.https://github.com/.extraheader';env['GIT_CONFIG_VALUE_'+str(n)]='AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode()
lookup=subprocess.run(['git','ls-remote','--heads',repo,'refs/heads/research/chained-jacobian-transfer-20261110'],cwd=r,env=env,capture_output=True,timeout=60);(out/'actual-remote-head.txt').write_bytes(lookup.stdout+lookup.stderr);assert lookup.returncode==0 and lookup.stdout.decode().split()[0]==bot
cmd=['git','-c','fetch.unpackLimit=0','-c','transfer.unpackLimit=0','fetch','--filter=blob:none','--no-tags','--no-recurse-submodules',repo,bot]
started=datetime.now(timezone.utc).isoformat();q=subprocess.run(cmd,cwd=r,env=env,capture_output=True,timeout=300);(out/'fetch.txt').write_bytes(q.stdout+q.stderr);(out/'fetch.exit').write_text(str(q.returncode)+'\n');print('exactbotfetch',q.returncode,flush=True);assert q.returncode==0
records={}
def expose_closed_remote_packs():
 for p in sorted((ram/'pack').glob('*')):
  assert p.is_file()and not p.name.startswith('tmp'),str(p)
  if p.suffix=='.pack':assert p.with_suffix('.promisor').is_file(),'new remote pack must retain its original promisor marker'
  target=r/'.git/objects/pack'/p.name;body_hash=sha(p)
  if target.exists()or target.is_symlink():assert target.is_file()and sha(target)==body_hash
  else:target.symlink_to(p)
  assert sha(target)==body_hash
  records[p.name]=dict(actual_RAM_path=str(p),exposed_Git_path=str(target),bytes=p.stat().st_size,sha256=body_hash,mode=p.stat().st_mode&0o777,original_new_remote_fetch_encoding=True,existing_objects_not_replaced=True)
expose_closed_remote_packs();assert read('rev-parse',bot+'^').decode().strip()==source
objects=json.loads((r/'research/chained_jacobian_transfers_20261110/evidence/public-n4-original-relation-rank-character-handoff-27/objects.json').read_text());migrations={v['path']for v in objects if v['original_policy_requires_store']};assert len(migrations)==11
prefixes={str(Path(p).parent)for p in migrations};assert len(prefixes)==1;prefix=next(iter(prefixes));expected=migrations|{prefix+'/.artifacts.json',prefix+'/.gitignore'}
changed=set(read('diff','--no-renames','--name-only',source,bot).decode().splitlines());assert changed==expected and len(changed)==13
hydrated=[]
for name in [prefix+'/.artifacts.json',prefix+'/.gitignore']:
 row=read('ls-tree',bot,'--',name).decode().split();assert len(row)>=4;oid=row[2]
 lazy=dict(env,GIT_NO_LAZY_FETCH='0');p=subprocess.run(['git','show',bot+':'+name],cwd=r,env=lazy,capture_output=True,timeout=120);label=Path(name).name.lstrip('.');(out/(label+'.whole-blob')).write_bytes(p.stdout);(out/(label+'.stderr')).write_bytes(p.stderr);(out/(label+'.exit')).write_text(str(p.returncode)+'\n');assert p.returncode==0
 assert read('hash-object','--stdin',environment=plain).decode().strip() if False else True
 actual=subprocess.check_output(['git','hash-object','--stdin'],input=p.stdout,cwd=r,env=plain).decode().strip();assert actual==oid
 hydrated.append(dict(path=name,blob=oid,bytes=len(p.stdout),sha256=hashlib.sha256(p.stdout).hexdigest(),actual_exit=0));expose_closed_remote_packs()
for key,args in [('head',['rev-parse','HEAD']),('refs',['show-ref']),('status',['status','--porcelain'])]:assert read(*args)==before[key],key
assert intact(control)==guard
(out/'closed-new-remote-cache.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),start=started,source=source,actual_bot=bot,command=cmd,actual_exit=0,exact_direct_parent=source,exact_changed_paths=sorted(changed),all_new_remote_cache_files=list(records.values()),only_two_exact_metadata_blobs_hydrated=hydrated,HEAD_all_refs_worktree_status_unchanged=True,complete_input_guard_unchanged=guard,no_existing_local_or_nonpromisor_object_deleted_or_moved=True,recovery='If this RAM store is lost, remove only listed broken remote-cache symlinks and their same-basename index/promisor metadata, then explicitly refetch this immutable bot into a new RAMstore; preserve all unrelated/local objects and all scientific inputs. Original new payload blob-cache recovery is separate in the committed governance receipt.',scope='unsigned exact original remote-bot/cache custody only; actual normalFF and terminal hold release remain pending'),indent=2)+'\n')
print('RAMfetchclosed',len(records),'remote cache files; exactparent/pathset/metadata and allrefs-HEAD-ownedguards unchanged',flush=True)
