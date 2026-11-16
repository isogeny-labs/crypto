"""Preserve original failure and publish the new thin original-tool transport."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,lzma,os,re,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='c019950bd36d74bbfa379d346c3943dec591a4df'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
branch='codex/chained-transfer-ledger-immutable-20261115-c019';basebranch='checkpoint/chained-transfer-ledger-immutable-base-20261115-c019'
subprocess.run(['git','switch','--create',branch],cwd=r,check=True,capture_output=True)
g=s/'evidence/public-n4-ledger-immutable-tag-source-governance-02';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
control=Path('/workspace/scratch/ct131-ledger-immutable-tag-transport-02');hold=json.loads((control/'exact-source-hold.json').read_text());clone=Path(hold['actual_clone'])
assert len(hold['full_guarded_contents'])==952
for name in ['exact-source-hold.json','original-command.py','actual-clone.log','actual-clone.exit','actual-checkout.log','actual-checkout.exit','whole-original-sparse-patterns.txt']:shutil.copyfile(control/name,g/('exact-source-clone-'+name))
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343
def intact():
 for n,h in refs.items():assert hashlib.sha256((r/n).read_bytes()).hexdigest()==h,n
 assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=clone,text=True).strip()==base
 assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=clone)
 for n,row in hold['full_guarded_contents'].items():
  p=clone/n;assert p.is_file()and p.stat().st_size==row['bytes']and hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'],n
 assert hashlib.sha256((clone/'pcc/ledger.jsonl').read_bytes()).hexdigest()==hold['entire_source_ledger_sha256']
 for row in hold['whole_regular_payloads']:assert (clone/row['path']).read_bytes()==Path(row['local_original']).read_bytes()
 return dict(all343_protected_scientific_hashes=refs,actual_source_clone_files=952,whole_source_ledger_sha256=hold['entire_source_ledger_sha256'],clone_HEAD=base,clone_status='clean',source_payload_whole=True)
(g/'entire-source-and-clone-guard.json').write_text(json.dumps(intact(),indent=2)+'\n')
scope=[s.relative_to(r).as_posix(),'.github/workflows/chained-transfer-ledger-artifact-transport-02.yml','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-15.md','.resume/RESUME.md','.gitattributes']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(n,c,want=0):
 q=subprocess.run(c,cwd=r,capture_output=True);b=q.stdout+q.stderr;(g/(n+('.txt.xz'if len(b)>60000 else'.txt'))).write_bytes(lzma.compress(b)if len(b)>60000 else b);(g/(n+'.exit')).write_text(str(q.returncode)+'\n');print(n,q.returncode,flush=True);assert q.returncode==want;return q.stdout
assert not subprocess.check_output(['git','diff','--name-only',base,'--','tools','scripts','src','isogeny_algos','.artifacts/config.json','benchmarks',str(s/'genus3-prime-descent')],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('actionlint',['/workspace/scratch/actionlint-bin/actionlint','.github/workflows/chained-transfer-ledger-artifact-transport-02.yml']);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check']);run('benchrec',['python3','tools/benchrec.py','gate']);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]])
def stage():
 names=paths();f=Path('/workspace/scratch/ct135-root-immutable-transport.paths0');f.write_bytes(b'\0'.join(p for p in names if p!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(names)
 for n in selected:
  if n.startswith(b'pcc/claims/'):
   for proof in json.loads((r/n.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     t=proof['subject'];assert hashlib.sha256((r/t['path']).read_bytes()).hexdigest()==t['sha256'];assert t['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+t['path']],cwd=r,capture_output=True).returncode==0
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,prior_hold01_closed_failed=True,new_exact_source_hold=hold['actual_clone'],actual_static_governance_exits=0,all_original_tools_policies_old_workflows_science_unchanged=True,new_thin_workflow_only=True,source_ledger_952_clone_file_whole_payload_hold_active=True,actual_transport_run=False,no_synthetic_bot_or_manifest=True,scope='unsigned prospective source governance; original actual upload/replay/direct-parent3path bot/new tag/normalFF/terminal acceptance remain pending'),indent=2)+'\n')
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','ProspectiveRootoriginalledgerartifacttransport02 source afterfailedclosed01e3c377: actualindependentc019cloneclean952wholefiles/all343protectedscience/sourceledger/full726536Bpayload/externalcopy held, completesourceguardandoriginalcommandsretained. Concreteversionedworkflowusesimmutableoriginal2dartifacttool-policy-prefetch andc019source/1487base, exactentirereplayBEFORErealbot/directparent3paths/fullbundle/fullpartialoutputs, genuinelynewtagabsence-normalpush-readback, thenisolatedsourcenormalFF/terminalguards. Allsourceledger/actionlint/wiring/benchrec/prose/cachedwhitespace/artifact0; onlynewthinworkflowallowed, originalscience/tools/policies/oldworkflowsunchanged. Priorbranchracefullfailurepreserved, sourcepublishednewcodexfeature/base/tagwithoutforceorledgerrechain. Noactual02migration/newscientificrun/NEW28/solve/costacceptanceyet. Unsignedsourcegovernance.']
for p in sorted(g.iterdir()):c+=['--proof','attestation:'+p.relative_to(r).as_posix()]
for n in ['.github/workflows/chained-transfer-ledger-artifact-transport-02.yml',s.relative_to(r).as_posix()+'/LEDGER_ARTIFACT_TRANSPORT_02.md']:c+=['--proof','attestation:'+n]
claim=subprocess.check_output(c,cwd=r,text=True).strip()
entry=f'''# Freeze original artifact transport02 and immutable source clone

## What was done
{claim}: new thin workflow/protocol static gates0; exact original c019 clone holds952 files, full source ledger, all343 science inputs and whole payload/external copy. e3c377 retains attempt01 full failed publication and closed hold. [pcc:unverified]

## Decisions made
Publish concrete02 source on new codex feature/checkpoint base and genuinely new source tag. Actual original migration must replay whole bytes before its real direct-parent three-path bot, publish a new run tag, and complete normal FF plus terminal guards in the isolated source clone. Never force moved shared branches, rechain frozen ledgers or manufacture canonical metadata. [pcc:unverified]

## Open threads
Ready PR and actual transport02; full scientific NEW28; Consumer LP proof/native development and fresh CI03 materialization; original shared-ledger prefix conflicts. [pcc:unverified]

## Errors
Prior normal branch publication was rejected after successful whole-object replay. It remains failed;02 addresses publication with a new tag and exports the real bot bundle on every outcome. No02 success yet. [pcc:unverified]
'''
for n in ['docs/session-logs/2026-11-15.md',s.relative_to(r).as_posix()+'/QUEUE.md',s.relative_to(r).as_posix()+'/PUBLIC_TARGET_CHECKPOINT.md']:
 with(r/n).open('a')as out:out.write('\n'+datetime.now(timezone.utc).isoformat()+'\n\n'+entry+'\n---\n')
f=r/'.resume/RESUME.md';f.write_text(entry+'\n---\n'+f.read_text());stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);intact()
subprocess.run(['git','commit','--quiet','-m','research: publish original artifact transport with immutable tag custody'],cwd=r,check=True);head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();intact();print(claim,head,flush=True)
out=Path('/workspace/scratch/ct135-root-immutable-transport-new-ref-publication-01');out.mkdir();env=os.environ.copy();n=int(env.get('GIT_CONFIG_COUNT','0'));env.update({'GIT_CONFIG_COUNT':str(n+1),'GIT_CONFIG_KEY_'+str(n):'http.https://github.com/.extraheader','GIT_CONFIG_VALUE_'+str(n):'AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+env['GH_TOKEN']).encode()).decode(),'GIT_TERMINAL_PROMPT':'0'})
url='https://github.com/isogeny-labs/crypto.git';tag='refs/tags/chained-transfer-ledger-transport02-source-20261115-'+head[:12];targets={base:'refs/heads/'+basebranch,head:'refs/heads/'+branch};refspecs=[base+':refs/heads/'+basebranch,head+':refs/heads/'+branch,head+':'+tag];refs=['refs/heads/'+basebranch,'refs/heads/'+branch,tag]
before=subprocess.run(['git','ls-remote',url,*refs],cwd=r,env=env,capture_output=True,timeout=60);(out/'new-refs-before.txt').write_bytes(before.stdout+before.stderr);assert before.returncode==0 and not before.stdout
q=subprocess.run(['git','push','--atomic','--no-force','--porcelain',url,*refspecs],cwd=r,env=env,capture_output=True,timeout=180);(out/'actual-push.log').write_bytes(q.stdout+q.stderr);(out/'actual-push.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
after=subprocess.run(['git','ls-remote',url,*refs],cwd=r,env=env,capture_output=True,timeout=60);(out/'new-refs-after.txt').write_bytes(after.stdout+after.stderr);assert after.returncode==0 and set(tuple(x.split())for x in after.stdout.decode().splitlines())=={(base,refs[0]),(head,refs[1]),(head,tag)}
(out/'receipt.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),source=head,base=base,feature=branch,base_branch=basebranch,new_source_tag=tag,actual_atomic_normal_push=0,all_three_remote_refs_exact=True,all_refs_absent_before=True,no_force_or_existing_ref_update=True,source_clone_952_guards_unchanged=True,transport02_run=False),indent=2)+'\n');print('newfeature-base-tag-publication0',tag,flush=True)
