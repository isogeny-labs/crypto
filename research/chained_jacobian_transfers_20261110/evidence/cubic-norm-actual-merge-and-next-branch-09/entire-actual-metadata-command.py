from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,os,shutil,subprocess,lzma
r=Path('/workspace/crypto');old='58200989b520b1143c85bd572ea60c19caca28c4';new='aa1e8236f8e2c9ee6aec5657e68d74dd32da7794';branch='research/chained-transfer-cubic-norm-ci-custody-20261116'
h=Path('/dev/shm/ct312-norm-genuine-merge-next-branch');h.mkdir()
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0';shutil.copyfile(r/'.git/index',h/'whole-index-before');shutil.copyfile(r/'.git/index',h/'actual-switch-index');env['GIT_INDEX_FILE']=str(h/'actual-switch-index')
def git(*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
assert git('rev-parse','HEAD').decode().strip()==old
assert not git('diff','--cached','--name-only')
n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
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
g=r/'research/chained_jacobian_transfers_20261110/evidence/cubic-norm-actual-merge-and-next-branch-09'
for name in ['whole-actual-fetch.txt','whole-actual-switch.txt','actual-switch.exit']:shutil.copyfile(h/name,g/name)
shutil.copyfile(__file__,g/'entire-actual-metadata-command.py')
(g/'all-entire-current-owned-working-file-records.json.xz').write_bytes(lzma.compress(json.dumps(records,indent=2).encode()))
(g/'actual-merge-branch-custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),PR=4413,expected_head=old,actual_merge=new,fresh_branch=branch,actual_switch_exit=0,entire_committed_trees_equal=True,all_current_owned_files_unchanged=len(records),canonical_index_matches_actual_switch=True,whole_hold=str(h),no_deleted_branch_resurrection=True),indent=2)+'\n')
statement=f'PR4413 actually merged normally with expected head{old} and actual merge{new}; current API confirms closed/merged. Prior current CI6631ae and full8723-key raw/source/index integration6774a3 retain their exact scopes. Actual merge tree is byte-identical to genuine582 head. Normal new branch switch to {branch} from actual merge succeeds, all{len(records)} selected current owned working files and entire actual index preserved. All original source/raw/published ledger and old holds remain intact; no deleted merged branch resurrection, force/rechain/reset/admin bypass, paid work or scientific native population. This records actual merge and branch metadata, not signed mathematical/execution standing.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip();print(claim,new,branch,len(records))
entry=f'\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Cubic-norm current proof component merged; next source checkpoint\n\n## What was done\n{claim}: PR4413 actually merged {new}, expected58200989; both original current proof gates15/0 and all8 started checks pass6631ae, entire8723-key guarded fast-forward6774a3 preserves all952 earlier keys. Actual fresh branch {branch} starts at the genuine equal-tree merge; all{len(records)} current owned files/index retained. [pcc:unverified]\n\n## Decisions made\nPublish the completed currentCI/raw/merge/queue checkpoint, preserving directly pinned norm01 proof/report/workflow. Continue whole-runtime source/context preparation from actual consumer2c compiler success941dbe and complete source snapshot10ced7. [pcc:unverified]\n\n## Open threads\nConsumer source checkpoint, original artifact requirements including the whole543821-byte closure manifest, source-built OCI/runtime Git/checkpoint/ABI and full scientificNEW28 remain separate. Full historical60/240/62/original360/witness population has not run. [pcc:unverified]\n\n## Errors\nNo new integration failure; all earlier failures remain immutable. Daily/monthly exhausted-budgete42949 still prohibits paid builds/jobs. Main ledger fork and locked autoresearcher remain unchanged. [pcc:unverified]\n'
for file in ['docs/session-logs/2026-11-16.md','.resume/RESUME.md']:
 with (r/file).open('a') as f:f.write(entry)
