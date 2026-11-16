from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,os,shutil,subprocess,lzma
r=Path('/workspace/crypto');old='7d23adf049fe53183a469770b4000cb53ede3ad6';new='c8aa3f9d7b1584b859e04ee09ea73800da1ed201';branch='research/chained-transfer-whole-source-custody-20261116'
h=Path('/dev/shm/ct366-word-merge-normal-connection-next-branch');h.mkdir()
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0';shutil.copyfile(r/'.git/index',h/'whole-index-before');shutil.copyfile(r/'.git/index',h/'actual-switch-index');env['GIT_INDEX_FILE']=str(h/'actual-switch-index')
def git(*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
assert git('rev-parse','HEAD').decode().strip()==old
assert not git('diff','--cached','--name-only')
env['GIT_TERMINAL_PROMPT']='0'
# Use the configured existing HTTPS Git connection; no injected shell token.
q=subprocess.run(['git','fetch','--no-tags','--filter=blob:none','https://github.com/isogeny-labs/crypto.git',new],cwd=r,env=env,capture_output=True,timeout=60);(h/'whole-actual-fetch.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0
assert git('rev-parse',old+'^{tree}')==git('rev-parse',new+'^{tree}')
assert old in git('show','-s','--format=%P',new).decode().split()
names=sorted(set(git('ls-files','--modified','--others','--exclude-standard','-z').split(b'\0'))-{b''})
records=[]
for raw in names:
 name=raw.decode();p=r/name;assert p.is_file() and not p.is_symlink();b=p.read_bytes();q=h/'whole-owned-working-files'/name;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(b);records.append(dict(path=name,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()))
q=subprocess.run(['git','-c','gc.auto=0','switch','--create',branch,new],cwd=r,env=env,capture_output=True,timeout=120);(h/'whole-actual-switch.txt').write_bytes(q.stdout+q.stderr);(h/'actual-switch.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
assert git('rev-parse','HEAD').decode().strip()==new and git('symbolic-ref','--short','HEAD').decode().strip()==branch
assert not git('diff','--cached','--name-only')
for x in records:assert hashlib.sha256((r/x['path']).read_bytes()).hexdigest()==x['sha256']
shutil.copyfile(h/'actual-switch-index',r/'.git/index');assert (r/'.git/index').read_bytes()==(h/'actual-switch-index').read_bytes()
g=r/'research/chained_jacobian_transfers_20261110/evidence/cubic-word-actual-merge-and-new-custody-branch-09'
for name in ['whole-actual-fetch.txt','whole-actual-switch.txt','actual-switch.exit']:shutil.copyfile(h/name,g/name)
shutil.copyfile(__file__,g/'entire-actual-metadata-command.py')
(g/'all-entire-current-owned-working-file-records.json.xz').write_bytes(lzma.compress(json.dumps(records,indent=2).encode()))
(g/'actual-merge-branch-custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),PR=4415,expected_head=old,actual_merge=new,fresh_branch=branch,actual_switch_exit=0,entire_committed_trees_equal=True,all_current_owned_files_unchanged=len(records),canonical_index_matches_actual_switch=True,whole_hold=str(h),no_deleted_branch_resurrection=True),indent=2)+'\n')
statement=f'PR4415 actually merged normally with expected head{old} and confirmed actual merge{new}; latest current proof/PCC/all8 successful checks with superseded cancelled history96eb44 and actual8959-key raw/index/source integration89348c preserved. Actual merged tree equals genuine7d head. Fresh owned branch{branch} starts at the actual merge with all{len(records)} selected owned working files/index preserved and no deleted-head resurrection. New raw coefficient theorem2aa7f3/10 bindings/dual27/0/report52078B/e1998c retains counter/constructor/compiler/binary/global/cost scope boundaries. No force/rechain/reset/admin/paid/native historical execution or signed promotion. Next original whole-source byte persistence and portable runtime preparation remain separate.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip();print(claim,new,branch,len(records))
entry=f'\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Cubic-norm current proof component merged; next source checkpoint\n\n## What was done\n{claim}: PR4415 actually merged {new}, expected7d23adf; both original current proof gates27/0 and all8 latest started checks pass96eb44, entire8959-key guarded fast-forward89348c preserves all952 earlier keys. Actual fresh branch {branch} starts at the genuine equal-tree merge; all{len(records)} current owned files/index retained. [pcc:unverified]\n\n## Decisions made\nPublish the completed currentCI/raw/merge/queue checkpoint, preserving directly pinned norm01 proof/report/workflow. Continue original whole-source custody and portable runtime source preparation from publishedConsumeradb8 and actual2c compiler/native/proof941dbe, retaining complete source snapshot10ced7. New word01 assets are frozen. [pcc:unverified]\n\n## Open threads\nOriginal Consumer policy remains seven older snapshot/RESUME objects; whole543821B JSON not selected. Sourceadb8/whole clean context99ea57 is published; empty draft asset failures5ef5dc/8c3f95 retained. Whole source archive825358 needs original store, runtime source-link repair/currentCI and full scientificNEW28 remain separate. Full historical60/240/62/original360/witness population has not run. [pcc:unverified]\n\n## Errors\nNo new integration failure; all earlier failures remain immutable. Daily/monthly exhausted-budgete42949 still prohibits paid builds/jobs. Main ledger fork and locked autoresearcher remain unchanged. [pcc:unverified]\n'
for file in ['docs/session-logs/2026-11-16.md','.resume/RESUME.md']:
 with (r/file).open('a') as f:f.write(entry)
