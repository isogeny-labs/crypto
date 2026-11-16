from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,re,shutil,subprocess
r=Path('/workspace/crypto');p=r/'research/chained_jacobian_transfers_20261110';base='575b3cf41082596ebf8757a4974d1a27bdcd0301'
h=Path('/workspace/scratch/ct457-complete-basis-normal-checkpoint');h.mkdir()
shutil.copyfile(r/'.git/index',h/'entire-index-before');shutil.copyfile(r/'.git/index',h/'actual-current-index')
env={**os.environ,'GIT_OPTIONAL_LOCKS':'0','GIT_TERMINAL_PROMPT':'0','GIT_INDEX_FILE':str(h/'actual-current-index')}
def git(*args):return ['git','-c','gc.auto=0','-c','splitIndex.maxPercentChange=100','-c','splitIndex.sharedIndexExpire=never',*args]
assert subprocess.check_output(git('rev-parse','HEAD'),cwd=r,env=env,text=True).strip()==base
assert subprocess.run(git('diff','--cached','--quiet'),cwd=r,env=env).returncode==0
g=p/'evidence/complete-basis-proof-and-whole-CI-source-governance-25';g.mkdir()
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
run('original575-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('current-authored-prose',['python3','tools/pcc/pcc.py','lint-prose','docs/session-logs/2026-11-16.md','.resume/RESUME.md',str(p/'QUEUE.md'),str(p/'CUBIC_BASIS_INJECTIVITY_03.md'),str(p/'report-cubic-basis-injectivity-03/report.typ')])
run('two-new-frozen-workflow-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-shellcheck=','.github/workflows/chained-transfer-cubic-full-frobenius-proof-02.yml','.github/workflows/chained-transfer-cubic-basis-injectivity-proof-03.yml'])
source=p/'lean/CubicBasisInjectivity22303.lean';frozen=p/'evidence/cubic-basis-injectivity-final-03/CubicBasisInjectivity22303.lean'
assert source.read_bytes()==frozen.read_bytes()and hashlib.sha256(source.read_bytes()).hexdigest()=='9bd383b7e9d3ad241fa92e24d3b853cdba7011bf36d0cbe66fec89a7999e5ead'
assert source.read_bytes().startswith((p/'lean/CubicFullFrobenius22302.lean').read_bytes())
for f in ['tools/pcc/pcc.py','pcc/keyring.json','tools/lean/proof-gate.sh','tools/lean/AuditAxioms.lean','tools/artifacts/artifacts.py','.artifacts/config.json']:
 assert(r/f).read_bytes()==subprocess.check_output(git('show',base+':'+f),cwd=r,env=env)
stage();run('first-cached-whitespace',git('diff','--cached','--check'))
tree=subprocess.check_output(git('write-tree'),cwd=r,env=env,text=True).strip()
raw=run('original-policy-exact-new-whole-objects',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
payloads={m.group(2).decode():int(m.group(1).replace(b',',b''))for m in re.finditer(rb'^ +([0-9,]+)  (.+)$',raw,re.M)}
assert payloads and all(name.startswith('research/chained_jacobian_transfers_20261110/evidence/')for name in payloads)
objects=[]
for name,size in sorted(payloads.items()):
 f=r/name;assert f.is_file()and not f.is_symlink()and f.stat().st_size==size
 objects.append(dict(path=name,bytes=size,sha256=hashlib.sha256(f.read_bytes()).hexdigest(),scope='entire original payload; no arbitrary shard or truncated data'))
(g/'EXACT_WHOLE_NEW_POLICY_OBJECTS.json').write_text(json.dumps(objects,indent=2)+'\n')
w=r/'.github/workflows/pcc-gate.yml';old=w.read_text();lines=old.splitlines(keepends=True);count=0
for i,line in enumerate(lines):
 if 'python3 tools/artifacts/artifacts.py pull --jobs 16 'in line and 'whole-Consumer-original-docs-payloads-30/payloads/'in line:
  lines[i]=line.rstrip('\n')+' '+' '.join(sorted(payloads))+'\n';count+=1
assert count==2
w.write_text(''.join(lines))
for name in payloads:assert w.read_text().count(name)==2
(g/'actual-additive-PCC-hydration-wiring.json').write_text(json.dumps(dict(original_current_source_sha256=hashlib.sha256(old.encode()).hexdigest(),new_current_source_sha256=hashlib.sha256(w.read_bytes()).hexdigest(),two_original_hydration_commands_extended=2,new_whole_payload_paths=sorted(payloads),all_other_original_workflow_bytes_unchanged=True,original_verifiers_keyring_policy_unchanged=True,Consumer_frozen_PCC_workflow='not edited'),indent=2)+'\n')
baseline=h/'pcc-gate-original575.yml';baseline.write_text(old)
new=run('current-PCC-template-diagnostic',['/workspace/scratch/actionlint-bin/actionlint','-shellcheck=','.github/workflows/pcc-gate.yml'],1)
prior=run('original-PCC-template-diagnostic',['/workspace/scratch/actionlint-bin/actionlint','-shellcheck=',str(baseline)],1)
assert new.replace(b'.github/workflows/pcc-gate.yml',b'<same-workflow>')==prior.replace(str(baseline).encode(),b'<same-workflow>')
stage();run('final-cached-whitespace',git('diff','--cached','--check'))
tree=subprocess.check_output(git('write-tree'),cwd=r,env=env,text=True).strip()
raw=run('final-original-policy-whole-object-set',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
found={m.group(2).decode():int(m.group(1).replace(b',',b''))for m in re.finditer(rb'^ +([0-9,]+)  (.+)$',raw,re.M)};assert found==payloads
argv=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Normal clean-worktree checkpoint for complete cubic basis injectivity03 source9bd383b7: unchanged original dual37/0, exact frozen02 prefix, twelve complete bindings97a994 and inspected two-page43572B report/source-pinned CI9211ba. Original575 prefix/authored prose/new two-workflow actionlint/cached checks0, all original verifier/keyring/policy bytes unchanged. Original artifact policy selects exactly the attached whole new object set; both original Root PCC hydration commands add all exact paths without other workflow changes. Current PCC template diagnostic1 is byte-identical to original575 after exact filename normalization, not green/waived. All full current old published88/genuine04eCI147-member/replay3c4318, storage/index/schema/binding failures and new immutable575 tag/current issue3904 are retained. Commit finished03/evidence normally first, then merge genuine04e with clean worktree so save_state needs no dirty-tree stash; no uncommitted result/hook bypass/source-test relaxation. New current PR branch source/ownCI/full remote replay still follow; no native/paid/signed promotion.']
for f in sorted(g.iterdir()):argv+=['--proof','attestation:'+str(f.relative_to(r))]
claim=subprocess.check_output(argv,cwd=r,env=env,text=True).strip()
entry=f'''\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Commit complete03 and whole current evidence before normal genuine merge\n\n## What was done\n{claim}: source9bd383b7/full original dual37/0/twelve complete bindings97a994/inspected report-CI9211ba ready. Original575 prefix/prose/new workflows/cached0, original policy selects exact{len(payloads)} whole new objects; both Root PCC commands hydrate all of them. Old template diagnostic1 equals original575, retained/not green. All original tools/keyring/policy/native/frozen01/02 assets unchanged. [pcc:unverified]\n\n## Decisions made\nNormal commit now saves completed03 and all custody/negative/queue data. Keep working tree clean through the next normal genuine04e merge; capture actual commit/merge in scratch and record both afterward, avoiding the dirty-tree stash allocation. No scientific run or hook skip. [pcc:unverified]\n\n## Open threads\nGenuine merge/new existing4425 branch/tag/current full02+03 CI/remote replay/merge; exact Consumer manifests/guards/CI and next encoded norm proof. [pcc:unverified]\n\n## Errors\nBoth checkout128 and stash-capacity128, complete partial copies/cache-only index delta/schema/binding failures remain. No unchanged failed command or criterion waiver. [pcc:unverified]\n'''
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
