"""Publish completed static proofs and whole original shared-store data."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,json,lzma,os,re,shutil,subprocess,sys
r=Path('/workspace/crypto');base='d3c9dc35627dae5b058e1b1396fdbcca87af6b4a';branch='research/chained-transfer-odd-subgroup-frobenius-20261116'
env=os.environ.copy();env.update(GIT_OPTIONAL_LOCKS='0',GIT_TERMINAL_PROMPT='0')
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip()==base
assert subprocess.check_output(['git','symbolic-ref','--short','HEAD'],cwd=r,env=env,text=True).strip()==branch
idx=Path('/workspace/scratch/ct418-root-new-proof-whole-index');idx.mkdir();shutil.copyfile(r/'.git/index',idx/'entire-before');shutil.copyfile(r/'.git/index',idx/'actual-current');env['GIT_INDEX_FILE']=str(idx/'actual-current')
p=r/'research/chained_jacobian_transfers_20261110';g=p/'evidence/odd-subgroup-frobenius-shared-store-source-governance-32';g.mkdir();shutil.copyfile(__file__,g/'entire-original-command.py')
scope=[str(p.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-16.md','.resume/RESUME.md','.gitattributes','.github/workflows/pcc-gate.yml','.github/workflows/chained-transfer-odd-subgroup-proof-01.yml','.github/workflows/chained-transfer-cubic-constructor-proof-01.yml']
def names():
 return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r,env=env).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0')))-{b''})
def run(label,args,want=0):
 q=subprocess.run(args,cwd=r,env=env,capture_output=True,timeout=120);raw=q.stdout+q.stderr;(g/(label+'.txt.xz')).write_bytes(lzma.compress(raw));(g/(label+'.exit')).write_text(str(q.returncode)+'\n');print(label,q.returncode,flush=True);assert q.returncode==want;return raw
sp=importlib.util.spec_from_file_location('original_artifact_policy',r/'tools/artifacts/artifacts.py');m=importlib.util.module_from_spec(sp);sys.modules[sp.name]=m;sp.loader.exec_module(m);policy=m.Config.load(r).policy
payloads={n.decode():(r/n.decode()).stat().st_size for n in names()if(r/n.decode()).is_file()and not(r/n.decode()).is_symlink()and m.belongs_in_store(n.decode(),(r/n.decode()).stat().st_size,policy)}
assert len(payloads)>=14 and len([n for n in payloads if '/whole-Consumer-original-docs-payloads-30/payloads/'in n])==10
workflow=r/'.github/workflows/pcc-gate.yml';text=workflow.read_text();needle='          python3 tools/artifacts/artifacts.py pull --jobs 16 ';assert text.count(needle)==2
lines=text.splitlines(keepends=True)
for i,line in enumerate(lines):
 if line.startswith(needle):
  assert all(n not in line for n in payloads);lines[i]=line.rstrip('\n')+' '+' '.join(sorted(payloads))+'\n'
workflow.write_text(''.join(lines))
(g/'all-actual-complete-new-policy-payloads.json').write_text(json.dumps([{'path':n,'bytes':b,'sha256':hashlib.sha256((r/n).read_bytes()).hexdigest()}for n,b in sorted(payloads.items())],indent=2)+'\n')
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','.artifacts/config.json','pcc/keyring.json'],cwd=r,env=env)
for path in ['lean/CubicNormDescent.lean','lean/CubicNormWordBridge01.lean','CUBIC_WORD_BRIDGE_01.md','report-cubic-word-bridge-01/report.typ','report-cubic-word-bridge-01/REPORT.pdf']:
 assert(p/path).read_bytes()==subprocess.check_output(['git','show',base+':'+str((p/path).relative_to(r))],cwd=r,env=env)
for file,sha in [('ClearedChainOddSubgroup01.lean','ffd54023b7b5f1b26880d26c9feba6ae4c0de0cbe8d86c9f5e03fd9bb17ba002'),('CubicConstructorFrobenius22301.lean','e81532c9df6d5c416f26693ed7849d70e512d5225558adbab5147266f1e59738')]:assert hashlib.sha256((p/'lean'/file).read_bytes()).hexdigest()==sha
run('original-merged-parent-ledger-prefix',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
authored=[n.decode()for n in names()if n.endswith(b'.md')and b'/evidence/'not in n]
authored += [str((p/d/'report.typ').relative_to(r))for d in ['report-odd-subgroup-chain-cancellation-01','report-cubic-constructor-frobenius-01']]
run('current-authored-prose',['python3','tools/pcc/pcc.py','lint-prose',*authored])
raw=run('preserved-original-unsigned-tag-prose-negative',['python3','tools/pcc/pcc.py','lint-prose',str((p/'evidence/cubic-constructor-derivation-report-CI-05/entire-first-unsigned-prose-derivation.md').relative_to(r))],1);assert b'line 32' in raw and b'1 finding' in raw
raw=run('unchanged-historical-benchrec',['python3','tools/benchrec.py','gate'],1);assert hashlib.sha256(raw).hexdigest()=='dc4fc3d1e178e01f99a53162a18e81d96fb8c783b57b25929a986ceb38e17b18'
for file in ['chained-transfer-odd-subgroup-proof-01.yml','chained-transfer-cubic-constructor-proof-01.yml']:run('original-scoped-wiring-'+file,['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/'+file])
run('actual-three-workflows-actionlint',['/workspace/scratch/actionlint-bin/actionlint','-oneline','.github/workflows/chained-transfer-odd-subgroup-proof-01.yml','.github/workflows/chained-transfer-cubic-constructor-proof-01.yml','.github/workflows/pcc-gate.yml'])
view=idx/'actual-base-workflow'/'pcc-gate.yml';view.parent.mkdir();view.write_bytes(subprocess.check_output(['git','show',base+':.github/workflows/pcc-gate.yml'],cwd=r,env=env))
a=run('original-base-PCC-template-diagnostic',['python3','tools/artifacts/wire_workflows.py','--check',str(view)],1);b=run('current-PCC-template-diagnostic',['python3','tools/artifacts/wire_workflows.py','--check','.github/workflows/pcc-gate.yml'],1);assert a==b
shutil.copyfile(workflow,g/'actual-current-PCC-workflow.yml')
def stage():
 ns=names();f=idx/'selected-paths0';f.write_bytes(b'\0'.join(n for n in ns if n!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,env=env,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,env=env,check=True)
 actual=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r,env=env).split(b'\0'))-{b''};assert actual==set(ns)
 for n in actual:
  if n.startswith(b'pcc/claims/'):
   for proof in json.loads((r/n.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     s=proof['subject'];assert hashlib.sha256((r/s['path']).read_bytes()).hexdigest()==s['sha256'];assert s['path'].encode()in actual or subprocess.run(['git','cat-file','-e',base+':'+s['path']],cwd=r,env=env,capture_output=True).returncode==0
stage()
q=subprocess.run(['git','diff','--cached','--check'],cwd=r,env=env,capture_output=True);raw=q.stdout+q.stderr;(g/'entire-first-cached-whitespace.txt.xz').write_bytes(lzma.compress(raw));(g/'first-cached-whitespace.exit').write_text(str(q.returncode)+'\n')
if q.returncode:
 assert q.returncode==2;paths=set(re.findall(rb'^([^\n]+):[0-9]+: ',raw,re.M));assert paths
 for name in paths:assert b'/evidence/'in name and name.endswith((b'.txt',b'.stdout',b'.stderr',b'.json'))
 with(r/'.gitattributes').open('a')as f:
  for name in sorted(paths):f.write('\n'+name.decode()+' -whitespace\n')
 (g/'exact-captured-raw-whitespace-paths.json').write_text(json.dumps([n.decode()for n in sorted(paths)],indent=2)+'\n')
stage();run('final-cached-whitespace',['git','diff','--cached','--check'])
tree=subprocess.check_output(['git','write-tree'],cwd=r,env=env,text=True).strip();raw=run('original-policy-before-genuine-migration',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
found={x.group(2).decode():int(x.group(1).replace(b',',b''))for x in re.finditer(rb'^ +([0-9,]+)  (.+)$',raw,re.M)};assert found==payloads
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Publish two frozen additive static proof components after actualRoot4422 merged3c9/freshbranch180f9a: odd-subgroup7 new exact declarations dual9/0 a8584b/aeea67 and cubic constructor/Frobenius12 exact declarations dual13/0 cf6114/f09c88, both inspected two-page reports/diagrams and exact-source-pinned dual original-gate CI. New19 named statements retain all annihilator/Bezout/characteristic/basis hypotheses and actual geometry/cardinality/full-field/q-power/Rust powmod/compiler/counter/native/cost gaps; old norm/word source/report/workflows unchanged. Original merged-parent ledger/current authored prose/both new scoped wirings/three-workflow actionlint/final cached0; captured original missing-tag Markdown remains explicit negative1, old benchrec1 exact historical SHA and PCC template1 byte-identical to actual base. Exact whole payload set in attached original policy output, including full currentPCC archive/log/current9299-key guard/source ledger and10 complete Consumer shared-store payloads d0e075; precise both-job PCC hydration added before genuine original upload/full remote replay/bot. Original policy/keyring/verifiers/native untouched, no data sharding/truncation/exception or signed/native/paid promotion. Whole original source actual4-object remote replay/currentCI2e5c4c/guardd8e523/merge180f9a, all cast/prose/shape/ENOSPC/transport failures, currentConsumerbb whole746-member CI3e5411 and authoritative queue d73982 retained. Normal new source/tag/readyPR/current own proof+PCC/migration/guard/merge follow. Consumer genuine manifests only after actual Root original migration, never fabricate a Consumer bot or edit frozen PCC source; full scientificNEW28/runtime/admission/LP remain separate.']
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip()
entry='\n---\n\n'+datetime.now(timezone.utc).isoformat()+'\n# Publish nineteen scoped new statements and complete original-store handoff\n\n## What was done\n'+claim+': new frozen19 named theorem bindings/dual9/0 and13/0/inspected reports/source-pinned workflows ready. Original prefix/authored prose/new wirings/actionlint/cached0; preserved raw prose negative1, exact old benchrec/PCC template diagnostics1 retained. Exact whole payload policy set and precise both-job hydrations include10 whole Consumer objects. [pcc:unverified]\n\n## Decisions made\nNormal new owned Root branch/tag/ready PR from actuald3c9, then genuine original full remote replay/current proof/PCC CI/guard/merge. Consumer normal genuine manifest commit only after actual shared-store upload. [pcc:unverified]\n\n## Open threads\nOwn remote proof/currentCI/migration; actual field/geometry/program/cost bridges and full LP/runtime/ABI/admission/NEW28 remain explicit. [pcc:unverified]\n\n## Errors\nAll first cast/prose/shape/backup/transport failures preserved; no policy/source-guard waiver, forced history or paid launch. [pcc:unverified]\n'
for name in ['docs/session-logs/2026-11-16.md','.resume/RESUME.md']:
 with(r/name).open('a')as f:f.write(entry)
stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,env=env,check=True);subprocess.run(['git','-c','gc.auto=0','commit','--quiet','-m','research: prove odd subgroup chain cancellation and cubic Frobenius fixed subspace'],cwd=r,env=env,check=True)
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,env=env,text=True).strip();assert subprocess.check_output(['git','rev-parse','HEAD^'],cwd=r,env=env,text=True).strip()==base;shutil.copyfile(idx/'actual-current',r/'.git/index');assert(r/'.git/index').read_bytes()==(idx/'actual-current').read_bytes();print(claim,head,'whole_new_payloads',len(payloads),flush=True)
url='https://github.com/isogeny-labs/crypto.git'
for kind,ref in [('new-owned-branch','refs/heads/'+branch),('new-immutable-tag','refs/tags/chained-transfer-odd-subgroup-frobenius-source-20261116-'+head[:12])]:
 out=idx/kind;out.mkdir();q=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'entire-before.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0 and not q.stdout
 q=subprocess.run(['git','push','--no-force','--porcelain',url,head+':'+ref],cwd=r,env=env,capture_output=True,timeout=180);(out/'entire-actual-push.log.xz').write_bytes(lzma.compress(q.stdout+q.stderr));(out/'actual-push.exit').write_text(str(q.returncode)+'\n');assert q.returncode==0
 q=subprocess.run(['git','ls-remote',url,ref],cwd=r,env=env,capture_output=True,timeout=60);(out/'entire-after.txt').write_bytes(q.stdout+q.stderr);assert q.returncode==0 and q.stdout.decode().split()[0]==head
 (out/'actual-normal-publication.json').write_text(json.dumps({'time':datetime.now(timezone.utc).isoformat(),'source':head,'parent':base,'ref':ref,'actual_normal_new_push':0,'exact_readback':True,'force':False,'authentication':'normal inherited configured Git; no manual token header'},indent=2)+'\n');print(kind,head,flush=True)
