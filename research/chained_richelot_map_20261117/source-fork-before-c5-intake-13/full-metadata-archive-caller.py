"""Full metadata/index custody before canonical intake; no research execution."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,subprocess,tarfile,lzma
r=Path('/workspace/crypto-chained-balanced-main');o=r/'research/chained_richelot_map_20261117/source-fork-before-c5-intake-13';o.mkdir(exist_ok=False)
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def git(*a):return subprocess.check_output(['git','-c','gc.auto=0',*a],cwd=r,env=env)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
assert git('rev-parse','HEAD').decode().strip()=='b10c9199e72fa149352f8f844c4286d9f43a3b0c'
roots=['pcc','tools/pcc','tools/artifacts','tools/lessons.py','tools/autonomy_report.py','tools/goal_brief.py','.artifacts/config.json','docs/lessons','benchmarks/agent-autonomy','docs/session-logs/2026-11-17.md','docs/ACTIVE.md','AGENTS.md']
files={}
for rel in roots:
 p=r/rel
 if p.is_file():files[rel]=p
 elif p.is_dir():
  for q in sorted(p.rglob('*')):
   if q.is_file() and '__pycache__' not in q.parts:files[str(q.relative_to(r))]=q
 else:assert rel=='docs/ACTIVE.md',rel
index=Path(env['GIT_INDEX_FILE']);files['index/entire-owned-current-index']=index
archive=Path('/run/lock/ct758-entire-old-source-PCC-journal-tools-and-index.tar.xz');assert not archive.exists()
with tarfile.open(archive,'w:xz',preset=9) as t:
 for name,p in sorted(files.items()):t.add(p,arcname=name,recursive=False)
rows=[]
with tarfile.open(archive,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for name,p in sorted(files.items()):
  m=t.getmember(name);b=t.extractfile(m).read();assert b==p.read_bytes() and m.mode==p.stat().st_mode&0o7777;rows.append({'path':name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':m.mode})
oid=git('hash-object','-w',str(archive)).decode().strip();rel=str(o.relative_to(r))+'/entire-old-source-PCC-journal-tools-and-index.tar.xz'
assert git('cat-file','blob',oid)==archive.read_bytes()
git('update-index','--add','--cacheinfo','100644',oid,rel)
out=o/'FULL_OLD_SOURCE_METADATA_INVENTORY.json.xz';out.write_bytes(lzma.compress((json.dumps(rows,indent=2)+'\n').encode(),preset=9))
receipt={'time':datetime.now(timezone.utc).isoformat(),'previous_complete_native_result_checkpoint':'b10c9199e72fa149352f8f844c4286d9f43a3b0c','canonical_target':'c5cbb67b19fe39864da7f8c9b29726696ba01580','complete_old_metadata_index_archive':{'logical_path':rel,'retained_full_container':str(archive),'Git_blob':oid,'bytes':archive.stat().st_size,'sha256':sha(archive),'members':len(rows),'all_full_member_bytes_and_modes_compared':True,'worktree_container_intentionally_unmaterialized':True},'old_ledger':{'rows':len((r/'pcc/ledger.jsonl').read_bytes().splitlines()),'bytes':(r/'pcc/ledger.jsonl').stat().st_size,'sha256':sha(r/'pcc/ledger.jsonl')},'all_old_claims_archived':sum(x['path'].startswith('pcc/claims/') for x in rows),'original_protected_indices_and_shared_stores_untouched':True,'no_claim_rechain_or_policy_change':True,'canonical_main_should_reference_immutable_old_public_checkpoint_for_full_containers':True}
(o/'OLD_SOURCE_FORK_CUSTODY.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
