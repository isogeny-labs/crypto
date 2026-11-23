"""Whole immutable metadata publication; exact byte/link guards, no science math."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
from ct105_hold27_guard import sha,state
r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis');scratch=Path('/workspace/scratch')
s=r/'research/chained_jacobian_transfers_20261110';cs=c/'experiments/chained-transfer-public-target'
base='990ec6b152fbc8e5bad4bf31c1231f6aa4d2e465';consumer='5f23dd29a375fd301ba1e37d008ed2b0d04a7191'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
o=scratch/'ct108-index-sidecar-custody-01';o.mkdir();(o/'original-preparer.py').write_bytes(Path(__file__).read_bytes())
prior=json.loads((scratch/'ct104-whole-closed-science-handoff-27-attempt02/current-inputs.json').read_text())
links=json.loads((scratch/'ct104-whole-closed-science-handoff-27-attempt02/symlink-states.json').read_text())
assert len(prior)==65886 and len(links)==179
allowed={str(repo/path)for repo,study,ck,q in [(r,s,'PUBLIC_TARGET_CHECKPOINT.md','QUEUE.md'),(c,cs,'CHECKPOINT.md','RESEARCH_QUEUE.md')]for path in ['.resume/RESUME.md','pcc/ledger.jsonl','.gitattributes','docs/session-logs/2026-11-15.md',str(study.relative_to(repo)/ck),str(study.relative_to(repo)/q)]}
current={};changes={}
for path,old in prior.items():
    actual=sha(path);current[path]=actual
    if actual!=old:changes[path]=dict(previous_sha256=old,current_sha256=actual)
(o/'released-metadata-changes.json').write_text(json.dumps(changes,indent=2)+'\n')
assert set(changes)<=allowed, 'Unexpected old held byte change; inspect complete differences, do not reset or expand scientific exceptions'
for path,old in links.items():assert Path(path).is_symlink()and state(path)==old,path
names=subprocess.check_output(['git','ls-files','-z','--',str(cs.relative_to(c)),'docs/research-artifacts/chained-transfer','.resume/RESUME.md','.gitattributes','pcc/ledger.jsonl','pcc/claims','docs/session-logs/2026-11-15.md'],cwd=c).split(b'\0')
dirty=subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',str(cs.relative_to(c)),'docs/research-artifacts/chained-transfer','.resume/RESUME.md','.gitattributes','pcc/ledger.jsonl','pcc/claims','docs/session-logs/2026-11-15.md'],cwd=c).split(b'\0')
for n in sorted((set(names)|set(dirty))-{b''}):
    p=c/n.decode();assert p.is_file()or p.is_symlink(),str(p)
    if p.is_symlink():links[str(p)]=state(p)
    if p.is_file():current[str(p)]=sha(p)
status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=c)
owned={}
dirty=set(dirty)|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=c).split(b'\0'))
for n in sorted(dirty-{b''}):
    p=c/n.decode();assert p.is_file()or p.is_symlink();owned[n.decode()]=dict(sha256=sha(p)if p.is_file()else None,literal_link=os.readlink(p)if p.is_symlink()else None)
guard=dict(at=datetime.now(timezone.utc).isoformat(),contents=current,literal_link_states=links,all_prior65886_content_keys_and179links_retained=True,declared_released_metadata_changes=changes,consumer_HEAD=consumer,consumer_status=status.decode(),consumer_owned=owned,consumer_exact_ledger_sha256=sha(c/'pcc/ledger.jsonl'),producer_ledger_prefix_commit=base,scope='temporary metadata sidecar custody; future whole scientific custody NEW28 remains separate')
body=json.dumps(guard,sort_keys=True,indent=2).encode()+b'\n';(o/'complete-current-guard.json').write_bytes(body);packed=lzma.compress(body);assert lzma.decompress(packed)==body;(o/'complete-current-guard.json.xz').write_bytes(packed)
target=s/'evidence/public-n4-comparator-index-sidecar-01';target.mkdir()
index=cs/'evidence/whole-relation-comparator-source-governance-failure-01/whole-failed-staged-index.xz'
objects=[]
for p,name in [(index,'whole-failed-staged-index.xz'),(o/'complete-current-guard.json.xz','complete-current-guard.json.xz')]:
    q=target/name;shutil.copyfile(p,q);assert p.read_bytes()==q.read_bytes()
    objects.append(dict(path=q.relative_to(r).as_posix(),bytes=q.stat().st_size,sha256=sha(q),local_original=str(p)))
(target/'objects.json').write_text(json.dumps(objects,indent=2)+'\n')
(target/'scope.json').write_text(json.dumps(dict(source=base,consumer=consumer,content_keys=len(current),literal_links=len(links),owned_consumer_paths=len(owned),all_prior65886_keys179links=True,guard_file_sha256=hashlib.sha256(body).hexdigest(),objects=objects,all_whole_original_bytes_retained=True,hold='consumer owned/source/raw/ledger held until actual original replays-before-bot/exactFF/terminal byte-link-ledger guards',scope='complete immutable metadata sidecar; not whole scientific custody28 or a comparator result'),indent=2)+'\n')
(target/'original-preparer.py').write_bytes(Path(__file__).read_bytes())
(target/'README.md').write_text('# Complete failed-index metadata sidecar\n\nPreserve the full immutable index snapshot selected by the unchanged original artifact policy, plus its complete current content/link/owned-byte guard. Consumer source-governance02/claimcdcaa2 remains a retained artifact-gate failure. The whole2,140,576-byte index is pinned by4a8f8f; no byte is trimmed, renamed or split to evade policy. [pcc:unverified]\n\nHOLD consumer owned bytes, source/raw and ledger through actual original remote replays BEFORE the actual bot, exact normal FF and terminal guards. Project only actual original canonical entries and preserve the full subject bytes in a transparent relative alias after release. Future full scientific custody NEW28 remains separate. No comparator population or fresh diagnostic CI has run. [pcc:unverified]\n')
print('whole sidecar prepared',len(current),'content keys',len(links),'literal links',len(owned),'owned consumer paths;',len(changes),'released metadata updates; objects',[(v['bytes'],v['sha256'])for v in objects],flush=True)
