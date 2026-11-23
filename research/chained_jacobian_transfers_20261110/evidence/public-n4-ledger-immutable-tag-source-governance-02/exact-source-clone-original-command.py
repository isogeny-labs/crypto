"""Hold the immutable real source in its own repository for later normal FF."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,subprocess

repo=Path('/workspace/crypto');control=Path('/workspace/scratch/ct131-ledger-immutable-tag-transport-02');control.mkdir()
clone=Path('/dev/shm/ct131-ledger-immutable-source-02')
source='c019950bd36d74bbfa379d346c3943dec591a4df'
old=json.loads(Path('/workspace/scratch/ct121-ledger-divergence-artifact-custody-01/source-hold.json').read_text())
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()==source
before_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo)
before_status=subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=repo)
command=['git','clone','--shared','--no-checkout','--single-branch','--branch','research/chained-transfer-preserved-20261115-ct120',str(repo),str(clone)]
q=subprocess.run(command,capture_output=True,timeout=120);(control/'actual-clone.log').write_bytes(q.stdout+q.stderr);(control/'actual-clone.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
for key,value in [('gc.auto','0'),('remote.origin.url','https://github.com/isogeny-labs/crypto.git'),('remote.origin.promisor','true'),('remote.origin.partialclonefilter','blob:none')]:
    subprocess.run(['git','config',key,value],cwd=clone,check=True)
paths=set(old['full_guarded_contents'])
prefix=Path(old['whole_regular_payloads'][0]['path']).parent.as_posix()
patterns=['/'+name for name in sorted(paths)]+['/.github/workflows/','/tools/artifacts/','/.artifacts/config.json','/.gitignore','/'+prefix+'/.artifacts.json','/'+prefix+'/.gitignore']
(control/'whole-original-sparse-patterns.txt').write_text('\n'.join(patterns)+'\n')
subprocess.run(['git','sparse-checkout','init','--no-cone'],cwd=clone,check=True,capture_output=True)
subprocess.run(['git','sparse-checkout','set','--no-cone','--stdin'],cwd=clone,input=('\n'.join(patterns)+'\n').encode(),check=True,capture_output=True)
q=subprocess.run(['git','checkout','--detach',source],cwd=clone,capture_output=True,timeout=180);(control/'actual-checkout.log').write_bytes(q.stdout+q.stderr);(control/'actual-checkout.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
subprocess.run(['git','switch','--create','custody/ledger-metadata02-source'],cwd=clone,check=True,capture_output=True)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=clone,text=True).strip()==source
status=subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=clone);assert not status
contents={}
for name,row in old['full_guarded_contents'].items():
    path=clone/name;assert path.is_file()and path.stat().st_size==row['bytes']and hashlib.sha256(path.read_bytes()).hexdigest()==row['sha256'],name
    contents[name]=row
for row in old['whole_regular_payloads']:
    p=clone/row['path'];assert not p.is_symlink()and p.read_bytes()==Path(row['local_original']).read_bytes()
for directory in [clone/'.github/workflows',clone/'tools/artifacts']:
    for path in directory.rglob('*'):
        if path.is_file():contents[path.relative_to(clone).as_posix()]=dict(bytes=path.stat().st_size,sha256=hashlib.sha256(path.read_bytes()).hexdigest())
for name in ['.gitignore','.artifacts/config.json']:
    path=clone/name;contents[name]=dict(bytes=path.stat().st_size,sha256=hashlib.sha256(path.read_bytes()).hexdigest())
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo)==before_head
assert subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=repo)==before_status
hold=dict(at=datetime.now(timezone.utc).isoformat(),source=source,actual_clone=str(clone),source_is_real_published_commit=True,source_status=status.decode(),all406_prior_keys_retained=True,whole_regular_payloads=old['whole_regular_payloads'],full_guarded_contents=contents,entire_source_ledger_sha256=hashlib.sha256((clone/'pcc/ledger.jsonl').read_bytes()).hexdigest(),original_workspace_HEAD_status_unchanged=True,hold='exact source clone contents/ledger/payload until original complete replay before actual bot/new tag/normalFF and terminal guards; ongoing metadata workspace is separate',full_scientific_NEW28=False)
(control/'exact-source-hold.json').write_text(json.dumps(hold,indent=2)+'\n')
(control/'entire-source-ledger.jsonl').write_bytes((clone/'pcc/ledger.jsonl').read_bytes())
(control/'original-command.py').write_bytes(Path(__file__).read_bytes())
print(source,len(contents),'whole files held in independent real-source clone',flush=True)
