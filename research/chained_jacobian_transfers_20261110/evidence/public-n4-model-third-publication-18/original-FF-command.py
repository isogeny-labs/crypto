"""Exact original publisher fast-forward under fresh guard16; no force/reset/stash."""
from pathlib import Path
import base64,hashlib,json,os,subprocess
r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis');source='c35045cefbaf48048858a6783133ddd1ec133a77';bot='3a689cf8bb2a60355f8a909d74f42e8e6dd3187b';s=r/'research/chained_jacobian_transfers_20261110'
o=Path('/workspace/scratch/ct78-handoff18-artifact-ff-01');o.mkdir();control=Path('/workspace/scratch/ct77-whole-model-and-third-handoff-18');baseline=json.loads((control/'owned-before.json').read_text());inputs=json.loads((control/'current-inputs.json').read_text());links=json.loads((s/'evidence/public-n4-current-guard-18/symlink-targets.json').read_text());objects=json.loads((s/'evidence/public-n4-model-third-handoff-18/objects.json').read_text())
assert len(inputs)==24853 and len(links)==23 and len(objects)==9
log=Path('/workspace/scratch/ct78-handoff18-actual-publication-01/whole-job.log').read_bytes();assert log.index(b'8 remote objects replayed')<log.index(b'[detached HEAD 3a689cf8b]')
def owned(repo,scope):
 names=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=repo).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=repo).split(b'\0')))-{b''})
 return {n.decode():(dict(sha256=hashlib.sha256((repo/n.decode()).read_bytes()).hexdigest(),link=os.readlink(repo/n.decode())if(repo/n.decode()).is_symlink()else None)if(repo/n.decode()).exists()or(repo/n.decode()).is_symlink()else dict(deleted=True))for n in names}
def intact():
 assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in inputs.items())
 assert all(Path(p).is_symlink()and os.readlink(p)==target for p,target in links.items())
 for name,repo in [('root',r),('consumer',c)]:assert owned(repo,baseline['scopes'][name])==baseline['owned'][name],name
intact();assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==source
env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env['GIT_CONFIG_COUNT']=str(n+1);env['GIT_CONFIG_KEY_'+str(n)]='http.https://github.com/.extraheader';env['GIT_CONFIG_VALUE_'+str(n)]='AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode();env['GIT_TERMINAL_PROMPT']='0'
def run(name,cmd,environment=None):
 q=subprocess.run(cmd,cwd=r,env=environment,capture_output=True,timeout=120);(o/(name+'.txt')).write_bytes(q.stdout+q.stderr);(o/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==0;return q
run('fetch',['git','fetch','--no-tags','--no-recurse-submodules','https://github.com/isogeny-labs/crypto.git',bot],env)
assert subprocess.check_output(['git','rev-parse',bot+'^'],cwd=r,text=True).strip()==source
migrations={row['path']for row in objects if row['original_policy_requires_store']};assert len(migrations)==8
directories={str(Path(p).parent)for p in migrations};assert len(directories)==1
expected=migrations|{prefix+'/'+name for prefix in directories for name in ['.artifacts.json','.gitignore']};changed=set(subprocess.check_output(['git','diff','--name-only',source,bot],cwd=r,text=True).splitlines());assert changed==expected and len(changed)==10
manifests={prefix:json.loads(subprocess.check_output(['git','show',bot+':'+prefix+'/.artifacts.json'],cwd=r))for prefix in directories};assert sum(len(x['files'])for x in manifests.values())==8
for row in objects:
 data=subprocess.check_output(['git','show',source+':'+row['path']],cwd=r);assert len(data)==row['bytes']and hashlib.sha256(data).hexdigest()==row['sha256'];assert Path(row['local_original']).read_bytes()==data
 if row['original_policy_requires_store']:assert manifests[str(Path(row['path']).parent)]['files'][Path(row['path']).name]==dict(bytes=row['bytes'],sha256=row['sha256'])
 else:assert subprocess.check_output(['git','show',bot+':'+row['path']],cwd=r)==data
pdf='research/chained_jacobian_transfers_20261110/evidence/public-n4-norm-halving-handoff-16/report.pdf'
assert subprocess.check_output(['git','show',source+':'+pdf],cwd=r)==subprocess.check_output(['git','show',bot+':'+pdf],cwd=r)
intact();run('exact-ff',['git','merge','--ff-only',bot]);assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot;intact()
run('artifact',['python3','tools/artifacts/artifacts.py','check',source,bot]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check']);intact()
(o/'canonical-manifests.json').write_text(json.dumps(manifests,indent=2)+'\n')
receipt=dict(source=source,bot=bot,exact_parent=source,exact_changed_paths=sorted(changed),all_nine_original_source_Git_blobs_match_manifest_or_plainGit=True,ordinary_Git_80943B_original_PDF_unchanged=True,all_nine_declared_object_originals_retained=True,guard18_count=len(inputs),all24853paths_unchanged=True,all23_link_paths_and_targets=links,whole_start_of_hold_owned_baseline=baseline,all_owned_bytes_unchanged=True,normalFF=0,artifact=0,wiring=0,force_reset_stash=False,hold18='RELEASED only after original8remote replays/exactFF/terminal18guard+ownedbyte checks',scope='unsigned custody; no mathematical/execution/performance standing promotion')
(o/'dirty-preservation.json').write_text(json.dumps(receipt,indent=2)+'\n');print('hold18 RELEASED:24853paths/23links/ownedbytes/exact10paths/8originalentries+oneplainGitobject+ordinaryPDF preserved/FF-artifact-wiring0',flush=True)
