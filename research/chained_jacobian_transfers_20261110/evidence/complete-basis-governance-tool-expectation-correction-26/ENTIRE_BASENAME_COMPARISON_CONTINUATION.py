from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,re,shutil,subprocess
r=Path('/workspace/crypto');p=r/'research/chained_jacobian_transfers_20261110';base='575b3cf41082596ebf8757a4974d1a27bdcd0301'
h=Path('/workspace/scratch/ct457-complete-basis-normal-checkpoint');h.mkdir(exist_ok=True)

env={**os.environ,'GIT_OPTIONAL_LOCKS':'0','GIT_TERMINAL_PROMPT':'0','GIT_INDEX_FILE':str(h/'actual-current-index')}
def git(*args):return ['git','-c','gc.auto=0','-c','splitIndex.maxPercentChange=100','-c','splitIndex.sharedIndexExpire=never',*args]
assert subprocess.check_output(git('rev-parse','HEAD'),cwd=r,env=env,text=True).strip()==base
assert subprocess.run(git('diff','--cached','--quiet'),cwd=r,env={**env,'GIT_INDEX_FILE':str(r/'.git/index')}).returncode==0
g=p/'evidence/complete-basis-proof-and-whole-CI-source-governance-25';g.mkdir(exist_ok=True)
scope=['research/chained_jacobian_transfers_20261110','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.github/workflows/pcc-gate.yml','.github/workflows/chained-transfer-cubic-basis-injectivity-proof-03.yml']
def names():
 return sorted((set(subprocess.check_output(git('ls-files','--modified','--others','--exclude-standard','-z','--',*scope),cwd=r,env=env).split(b'\0'))|set(subprocess.check_output(git('diff','--cached','--name-only','-z'),cwd=r,env=env).split(b'\0')))-{b''})
def stage():
 ns=names();(h/'selected-paths0').write_bytes(b'\0'.join(x for x in ns if x!=b'.resume/RESUME.md')+b'\0')
 subprocess.run(git('add','--sparse','--pathspec-from-file='+str(h/'selected-paths0'),'--pathspec-file-nul'),cwd=r,env=env,check=True)
 subprocess.run(git('add','--sparse','-u','--','.resume/RESUME.md'),cwd=r,env=env,check=True)
 assert set(subprocess.check_output(git('diff','--cached','--name-only','-z'),cwd=r,env=env).split(b'\0'))-{b''}==set(ns)
 return ns
def run(label,args,want=0,save=True):
 q=subprocess.run(args,cwd=r,env=env,capture_output=True,timeout=120)
 if save:
  (g/(label+'.stdout.xz')).write_bytes(lzma.compress(q.stdout));(g/(label+'.stderr.xz')).write_bytes(lzma.compress(q.stderr));(g/(label+'.exit')).write_text(str(q.returncode)+'\n')
 print(label,q.returncode,flush=True);assert q.returncode==want,(label,q.stdout[-500:],q.stderr[-500:]);return q.stdout+q.stderr
objects=json.loads((g/'EXACT_WHOLE_NEW_POLICY_OBJECTS.json').read_text());payloads={x['path']:x['bytes']for x in objects}
baseline=h/'pcc-gate-original575.yml'
new=lzma.decompress((g/'current-PCC-template-diagnostic.stdout.xz').read_bytes())+lzma.decompress((g/'current-PCC-template-diagnostic.stderr.xz').read_bytes())
assert (g/'current-PCC-template-diagnostic.exit').read_text().strip()=='0'
prior=run('actual-original575-PCC-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-shellcheck=',str(baseline)])
assert new==prior==b''
for name in ['chained-transfer-cubic-full-frobenius-proof-02.yml','chained-transfer-cubic-basis-injectivity-proof-03.yml']:
 run('original-scoped-wiring-'+name,['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/'+name])
a=run('actual-original575-PCC-wiring-diagnostic',['python3','tools/artifacts/wire_workflows.py','--check',str(baseline)],1)
b=run('actual-current-PCC-wiring-diagnostic',['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/pcc-gate.yml'],1)
assert a.replace(str(baseline).encode(),b'<same-workflow>')==b.replace(b'.github/workflows/pcc-gate.yml',b'<same-workflow>')
run('final575-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('final-current-authored-prose',['python3','tools/pcc/pcc.py','lint-prose','docs/session-logs/2026-11-16.md','.resume/RESUME.md',str(p/'QUEUE.md'),str(p/'CUBIC_BASIS_INJECTIVITY_03.md'),str(p/'report-cubic-basis-injectivity-03/report.typ')])
shutil.copyfile(__file__,g/'ENTIRE_CURRENT_GOVERNANCE_CONTINUATION.py')
shutil.copyfile('/workspace/scratch/ct457_commit_complete_basis_and_hydrate_whole_evidence.py',p/'evidence/complete-basis-governance-tool-expectation-correction-26/ENTIRE_ORIGINAL_FAILED_METADATA_GOVERNOR.py')
stage();run('final-cached-whitespace',git('diff','--cached','--check'))
tree=subprocess.check_output(git('write-tree'),cwd=r,env=env,text=True).strip()
raw=run('final-original-policy-whole-object-set',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
found={m.group(2).decode():int(m.group(1).replace(b',',b''))for m in re.finditer(rb'^ +([0-9,]+)  (.+)$',raw,re.M)};assert found==payloads
argv=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Normal clean-worktree checkpoint for complete cubic basis injectivity03 source9bd383b7: unchanged original dual37/0, exact frozen02 prefix, twelve complete bindings97a994 and inspected two-page43572B report/source-pinned CI9211ba. Original575 prefix/authored prose/new two-workflow actionlint/cached checks0, all original verifier/keyring/policy bytes unchanged. Original artifact policy selects exactly the attached whole new object set; both original Root PCC hydration commands add all exact paths without other workflow changes. Current and original575 actionlint both0; original wire_workflows PCC diagnostic1 is byte-identical to original575 after exact filename normalization, not green/waived. Two new scoped wirings0. Original producer tool-expectation failureb19f5c retained. All full current old published88/genuine04eCI147-member/replay3c4318, storage/index/schema/binding failures and new immutable575 tag/current issue3904 are retained. Commit finished03/evidence normally first, then merge genuine04e with clean worktree so save_state needs no dirty-tree stash; no uncommitted result/hook bypass/source-test relaxation. New current PR branch source/ownCI/full remote replay still follow; no native/paid/signed promotion.']
for f in sorted(g.iterdir()):argv+=['--proof','attestation:'+str(f.relative_to(r))]
claim=subprocess.check_output(argv,cwd=r,env=env,text=True).strip()
entry=f'''\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Commit complete03 and whole current evidence before normal genuine merge\n\n## What was done\n{claim}: source9bd383b7/full original dual37/0/twelve complete bindings97a994/inspected report-CI9211ba ready. Original575 prefix/prose/new workflows/cached0, original policy selects exact{len(payloads)} whole new objects; both Root PCC commands hydrate all of them. Current/original575 actionlint0; original wiring diagnostic1 equals575, retained/not green. Two new scoped wirings0; producer expectation errorb19f5c preserved. All original tools/keyring/policy/native/frozen01/02 assets unchanged. [pcc:unverified]\n\n## Decisions made\nNormal commit now saves completed03 and all custody/negative/queue data. Keep working tree clean through the next normal genuine04e merge; capture actual commit/merge in scratch and record both afterward, avoiding the dirty-tree stash allocation. No scientific run or hook skip. [pcc:unverified]\n\n## Open threads\nGenuine merge/new existing4425 branch/tag/current full02+03 CI/remote replay/merge; exact Consumer manifests/guards/CI and next encoded norm proof. [pcc:unverified]\n\n## Errors\nBoth checkout128 and stash-capacity128, complete partial copies/cache-only index delta/schema/binding failures remain. No unchanged failed command or criterion waiver. [pcc:unverified]\n'''
for f in[r/'docs/session-logs/2026-11-16.md',r/'.resume/RESUME.md']:
 with f.open('a')as out:out.write(entry)
ns=stage()
for name in ns:
 if name.startswith(b'pcc/claims/'):
  for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
   if proof.get('kind')=='attestation':
    obj=proof['subject'];assert hashlib.sha256((r/obj['path']).read_bytes()).hexdigest()==obj['sha256'],obj['path']
subprocess.run(git('diff','--cached','--check'),cwd=r,env=env,check=True)
subprocess.run(git('commit','--quiet','-m','research: derive cubic basis injectivity and preserve whole proof custody'),cwd=r,env=env,check=True)
head=subprocess.check_output(git('rev-parse','HEAD'),cwd=r,env=env,text=True).strip();assert subprocess.check_output(git('rev-parse','HEAD^'),cwd=r,env=env,text=True).strip()==base
shutil.copyfile(h/'actual-current-index',r/'.git/index')
assert(r/'.git/index').read_bytes()==(h/'actual-current-index').read_bytes()
assert subprocess.check_output(git('status','--porcelain','--untracked-files=all'),cwd=r,env=env)==b''
(h/'ACTUAL_NORMAL_CLEAN_CHECKPOINT.json').write_text(json.dumps(dict(time=datetime.now(timezone.utc).isoformat(),head=head,parent=base,changed_paths=len(ns),selected_whole_payloads=objects,normal_commit_hooks_not_skipped=True,complete_owned_worktree_clean=True,canonical_index_equals_actual=True,next='normal clean-worktree genuine04e merge; do not append repo logs until commit+merge piece recorded together'),indent=2)+'\n')
print(claim,head,'whole_objects',len(objects),'paths',len(ns),flush=True)
