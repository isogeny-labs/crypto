"""Original publisher exact normal FF under full current24 hold; no force/reset/stash."""
from pathlib import Path
import base64,hashlib,json,os,re,subprocess,sys
r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis');s=r/'research/chained_jacobian_transfers_20261110';source,bot=sys.argv[1:3]
control=Path('/workspace/scratch/ct89-whole-closed-science-handoff-24');out=Path('/workspace/scratch/ct89-handoff24-artifact-ff-01');out.mkdir()
inputs=json.loads((control/'current-inputs.json').read_text());links=json.loads((s/'evidence/public-n4-current-guard-24/symlink-targets.json').read_text());baseline=json.loads((control/'owned-before.json').read_text());objects=json.loads((s/'evidence/public-n4-original-prime-kernel-geometry-handoff-24/objects.json').read_text());index=json.loads((s/'evidence/public-n4-original-prime-kernel-geometry-handoff-24/archive-parts.json').read_text())
scope=json.loads((s/'evidence/public-n4-original-prime-kernel-geometry-handoff-24/scope.json').read_text());assert len(inputs)==scope['current24_guard_keys']and len(links)==len(scope['all_link_targets'])and len(objects)==len(scope['all_objects']);replays=scope['original_policy_store_objects']
log=Path('/workspace/scratch/ct89-handoff24-actual-publication-01/whole-job.log').read_bytes();commits=list(re.finditer(rb'\[detached HEAD ([0-9a-f]+)\]',log));matching=[x for x in commits if bot.startswith(x[1].decode())];assert len(matching)==1 and log.index((str(replays)+' remote objects replayed').encode())<matching[0].start()
def owned(repo,scope):
 names=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=repo).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=repo).split(b'\0')))-{b''})
 return {n.decode():dict(sha256=hashlib.file_digest((repo/n.decode()).open('rb'),'sha256').hexdigest(),link=os.readlink(repo/n.decode())if(repo/n.decode()).is_symlink()else None)for n in names}
def intact():
 assert all(hashlib.file_digest(Path(p).open('rb'),'sha256').hexdigest()==h for p,h in inputs.items())
 assert all(Path(p).is_symlink()and os.readlink(p)==v for p,v in links.items())
 for label,repo in [('root',r),('consumer',c)]:assert owned(repo,baseline['scopes'][label])==baseline['owned'][label],label
intact();assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==source
env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env['GIT_CONFIG_COUNT']=str(n+1);env['GIT_CONFIG_KEY_'+str(n)]='http.https://github.com/.extraheader';env['GIT_CONFIG_VALUE_'+str(n)]='AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode();env['GIT_TERMINAL_PROMPT']='0'
def run(name,cmd,environment=None):
 q=subprocess.run(cmd,cwd=r,env=environment,capture_output=True,timeout=120);(out/(name+'.txt')).write_bytes(q.stdout+q.stderr);(out/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==0;return q
run('fetch',['git','fetch','--no-tags','--no-recurse-submodules','https://github.com/isogeny-labs/crypto.git',bot],env)
assert subprocess.check_output(['git','rev-parse',bot+'^'],cwd=r,text=True).strip()==source
migrations={x['path']for x in objects if x['original_policy_requires_store']};assert len(migrations)==replays
prefixes={str(Path(x).parent)for x in migrations};assert len(prefixes)==1
expected=migrations|{prefix+'/'+name for prefix in prefixes for name in ['.artifacts.json','.gitignore']}
changed=set(subprocess.check_output(['git','diff','--name-only',source,bot],cwd=r,text=True).splitlines());assert changed==expected and len(changed)==replays+2
manifests={prefix:json.loads(subprocess.check_output(['git','show',bot+':'+prefix+'/.artifacts.json'],cwd=r))for prefix in prefixes};assert sum(len(x['files'])for x in manifests.values())==replays
for row in objects:
 data=subprocess.check_output(['git','show',source+':'+row['path']],cwd=r);assert len(data)==row['bytes']and hashlib.sha256(data).hexdigest()==row['sha256'];assert data==Path(row['local_original']).read_bytes()
 if row['original_policy_requires_store']:assert manifests[str(Path(row['path']).parent)]['files'][Path(row['path']).name]==dict(bytes=row['bytes'],sha256=row['sha256'])
 else:assert subprocess.check_output(['git','show',bot+':'+row['path']],cwd=r)==data
pdf='research/chained_jacobian_transfers_20261110/evidence/public-n4-norm-halving-handoff-16/report.pdf';assert subprocess.check_output(['git','show',source+':'+pdf],cwd=r)==subprocess.check_output(['git','show',bot+':'+pdf],cwd=r)
whole=hashlib.sha256();count=0
for row in index['parts']:
 assert row['offset']==count;data=(control/row['name']).read_bytes();assert len(data)==row['bytes']and hashlib.sha256(data).hexdigest()==row['sha256'];whole.update(data);count+=len(data)
assert count==index['bytes']and whole.hexdigest()==index['sha256']
intact();run('exact-ff',['git','merge','--ff-only',bot]);assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot;intact()
run('artifact',['python3','tools/artifacts/artifacts.py','check',source,bot]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check']);intact()
(out/'canonical-manifests.json').write_text(json.dumps(manifests,indent=2)+'\n')
receipt=dict(source=source,bot=bot,exact_parent=source,original_remote_replays_before_bot=replays,exact_changed_paths=sorted(changed),all_original_source_Git_blobs_match_manifest_or_plainGit=True,all_objects_and_originals_retained=True,guard24_count=len(inputs),all_current24_paths_unchanged=True,all_current24_link_paths_and_targets=links,all_owned_bytes_unchanged=True,whole_start_of_hold_owned_baseline=baseline,whole_archive_ordered_reassembly=True,archive_bytes=count,archive_sha256=whole.hexdigest(),ordinary_original_PDF_unchanged=True,normalFF=0,artifact=0,wiring=0,force_reset_stash=False,hold24='RELEASED only after original actual remote replays/exactFF/wholeorderedarchive/allterminalguards-links-ownedbytes',scope='unsigned byte custody; native/model/proof/prospective and CI-wide/performance standing unchanged')
(out/'dirty-preservation.json').write_text(json.dumps(receipt,indent=2)+'\n');print('HOLD24 RELEASED',len(inputs),'inputs',len(links),'links',replays,'original remote replays',len(changed),'exact paths; whole archive/owned bytes/FF-artifact-wiring0',flush=True)
