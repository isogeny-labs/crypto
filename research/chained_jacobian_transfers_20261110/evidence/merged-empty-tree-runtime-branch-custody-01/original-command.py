from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');old='3dff0d3c46d8fda98016bf1afea71499c553a2ce';new='5ae1a82cc453102e63c828affda5b17faf65eabe'
branch='research/chained-transfer-runtime-progress-20261116'
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0'
def git(*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
assert git('rev-parse','HEAD').decode().strip()==old
assert not git('diff-tree','--no-commit-id','--name-status','-r',old,new)
assert git('rev-parse',old+'^{tree}')==git('rev-parse',new+'^{tree}')
assert not git('diff','--cached','--name-only')
hold=Path('/dev/shm/ct263-root-empty-tree-merge-and-branch-custody');hold.mkdir()
shutil.copyfile(r/'.git/index',hold/'entire-index-before')
scope=['research/chained_jacobian_transfers_20261110','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md']
names=sorted(set(git('ls-files','--modified','--others','--exclude-standard','-z','--',*scope).split(b'\0'))-{b''})
records=[]
for raw in names:
    n=raw.decode();f=r/n
    assert f.is_file() and not f.is_symlink()
    q=hold/'entire-owned-working-files'/n;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(f,q)
    records.append(dict(path=n,bytes=f.stat().st_size,sha256=hashlib.sha256(f.read_bytes()).hexdigest()))
q=subprocess.run(['git','-c','gc.auto=0','switch','--create',branch,new],cwd=r,env=env,capture_output=True)
(hold/'entire-actual-Git-switch.log').write_bytes(q.stdout+q.stderr)
(hold/'actual-switch.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
assert git('rev-parse','HEAD').decode().strip()==new
assert git('symbolic-ref','--short','HEAD').decode().strip()==branch
assert not git('diff','--cached','--name-only')
for x in records:assert hashlib.sha256((r/x['path']).read_bytes()).hexdigest()==x['sha256']
shutil.copyfile(r/'.git/index',hold/'entire-index-after')
g=r/'research/chained_jacobian_transfers_20261110/evidence/merged-empty-tree-runtime-branch-custody-01';g.mkdir()
shutil.copyfile(__file__,g/'original-command.py')
for name in ['entire-actual-Git-switch.log','actual-switch.exit']:shutil.copyfile(hold/name,g/name)
(g/'all-current-owned-working-files.json.xz').write_bytes(lzma.compress(json.dumps(records,indent=2).encode()))
(g/'actual-metadata-only-branch-custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),old_head=old,new_genuine_merged_head=new,
    new_owned_branch=branch,source_trees_equal=True,actual_switch_exit=0,entire_selected_current_working_files=len(records),
    all_current_owned_bytes_unchanged=True,staged_tree_clean=True,whole_index_and_file_hold=str(hold),
    no_deleted_branch_resurrection=True,no_source_or_raw_deletion=True),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Root genuine merged5ae1 has exactly the same entire Git tree as prior genuine3dff. Actual normal new-branch switch from3dff to5ae1 succeeds without modifying any staged/source tree or selected current owned working file; complete per-file working copies/index before/after and all selected SHA/size records retained. Uses new owned research/chained-transfer-runtime-progress-20261116; never resurrects deleted merged4401 head. Earlier8483-key scientific/raw guard270903 stays unchanged and is not rerun or replaced. No merge/rechain/force/data deletion or scientific execution. This metadata-only current source is the parent for the next ordinary checkpoint.']
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip()
text=f'''\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Resume from genuine merged Root tree on a fresh checkpoint branch\n\n## What was done\n{claim}: actual normal switch from3dff to genuine merged5ae1 on fresh owned runtime-progress branch0; entire committed trees equal, all{len(records)} selected current owned files/index retained byte-exact. Persistent GitHub queue/readyPR refreshd853c5 now records actual5e80CI8db45c and exhausted-budget e42949 plus runtime1169c2; entire prior/current API replies retained. [pcc:unverified]\n\n## Decisions made\nPublish finished Root currentCI/genuine migration8483-keyFF/merge/storage/queue evidence continuously; extend exact original artifact hydration inputs for the two new whole over-policy metadata objects before the real bot. Keep original verifiers/keyring/policy and immutable older workflow snapshots. [pcc:unverified]\n\n## Open threads\nConsumer driver source0405429a normally published with4d830d; offline metadata/input byte checks efa891 and first-whitespace failurea69dba retained. Actual new compiler-only CI expected after spaced observation around08:15UTC. Full60 historical computation/OCI/ABI/known admission/genuine scientificNEW28 and main ledger remain open. [pcc:unverified]\n\n## Errors\nOriginal budget daily/monthly exhaustion and pool timeout stop paid work, not all source/proof/custody work. Driver source review/function/footer corrections and GNU-time metadata failure remain whole. Root new branch changes no source tree or raw bytes. [pcc:unverified]\n'''
with (r/'docs/session-logs/2026-11-16.md').open('a') as f:f.write(text)
with (r/'.resume/RESUME.md').open('a') as f:f.write(text)
print(claim,len(records))
