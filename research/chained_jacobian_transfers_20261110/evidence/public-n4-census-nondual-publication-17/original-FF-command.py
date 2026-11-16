"""Exact original publisher fast-forward under fresh guard16; no force/reset/stash."""
from pathlib import Path
import base64,hashlib,json,os,subprocess
r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis');source='3f528b7c44724492ab9fba2a8c0b5b66c54a393b';bot='b9616569ecf182f54a32e86afdc5213ec33eeca0';s=r/'research/chained_jacobian_transfers_20261110'
o=Path('/workspace/scratch/ct76-handoff17-artifact-ff-01');o.mkdir();control=Path('/workspace/scratch/ct76-whole-census-and-nondual-handoff-17');baseline=json.loads((control/'owned-before.json').read_text());inputs=json.loads((control/'current-inputs.json').read_text());links=json.loads((s/'evidence/public-n4-current-guard-17/symlink-targets.json').read_text());objects=json.loads((s/'evidence/public-n4-census-nondual-handoff-17/objects.json').read_text())
assert len(inputs)==24445 and len(links)==23 and len(objects)==5
log=Path('/workspace/scratch/ct76-handoff17-actual-publication-01/whole-job.log').read_bytes();assert log.index(b'5 remote objects replayed')<log.index(b'[detached HEAD b9616569e]')
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
migrations={row['path']for row in objects if row['original_policy_requires_store']};assert len(migrations)==5
directories={str(Path(p).parent)for p in migrations};assert len(directories)==1
expected=migrations|{prefix+'/'+name for prefix in directories for name in ['.artifacts.json','.gitignore']};changed=set(subprocess.check_output(['git','diff','--name-only',source,bot],cwd=r,text=True).splitlines());assert changed==expected and len(changed)==7
manifests={prefix:json.loads(subprocess.check_output(['git','show',bot+':'+prefix+'/.artifacts.json'],cwd=r))for prefix in directories};assert sum(len(x['files'])for x in manifests.values())==5
for row in objects:
 data=subprocess.check_output(['git','show',source+':'+row['path']],cwd=r);assert len(data)==row['bytes']and hashlib.sha256(data).hexdigest()==row['sha256'];assert Path(row['local_original']).read_bytes()==data
 if row['original_policy_requires_store']:assert manifests[str(Path(row['path']).parent)]['files'][Path(row['path']).name]==dict(bytes=row['bytes'],sha256=row['sha256'])
 else:assert subprocess.check_output(['git','show',bot+':'+row['path']],cwd=r)==data
pdf='research/chained_jacobian_transfers_20261110/evidence/public-n4-norm-halving-handoff-16/report.pdf'
assert subprocess.check_output(['git','show',source+':'+pdf],cwd=r)==subprocess.check_output(['git','show',bot+':'+pdf],cwd=r)
intact();run('exact-ff',['git','merge','--ff-only',bot]);assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot;intact()
run('artifact',['python3','tools/artifacts/artifacts.py','check',source,bot]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check']);intact()
(o/'canonical-manifests.json').write_text(json.dumps(manifests,indent=2)+'\n')
receipt=dict(source=source,bot=bot,exact_parent=source,exact_changed_paths=sorted(changed),all_five_original_source_Git_blobs_match_manifests=True,ordinary_Git_80943B_original_PDF_unchanged=True,all_five_declared_object_originals_retained=True,guard17_count=len(inputs),all24445paths_unchanged=True,all23_link_paths_and_targets=links,whole_start_of_hold_owned_baseline=baseline,all_owned_bytes_unchanged=True,normalFF=0,artifact=0,wiring=0,force_reset_stash=False,hold17='RELEASED only after original5remote replays/exactFF/terminal17guard+ownedbyte checks',scope='unsigned custody; no mathematical/execution/performance standing promotion')
(o/'dirty-preservation.json').write_text(json.dumps(receipt,indent=2)+'\n');print('hold17 RELEASED:24445paths/23links/ownedbytes/exact7paths/5originalentries+ordinaryPDF preserved/FF-artifact-wiring0',flush=True)
