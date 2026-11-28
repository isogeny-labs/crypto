"""Data-only handoff checkpoint; original artifact policy remains enforced."""
from pathlib import Path
from datetime import datetime,timezone
import json,subprocess,hashlib,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='9a1df0f5ac0abf50c4e6ed5f671c48a453dd5c80'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
current='''Current canonical G3:producer CI27e47f and all4+8+5 raw publications complete. Independent norm ready1521/unit02 75275f FOURpass/full148,356,1324domains/both2compositions/explicitcontrols;CLIaf3d78/nineclasses12processcontrols;13exactbindings localpass/formalscopec88b36. SIXwholeconsumerarchives4ea88c pending actualtrustedpublication. Rootfixture authenticread2 unablecredentials/anonymous1 HeadObject403,add561;globalregression NOTgreen. Next genuine6objectmigration/consumerreport/source/independentCI;no computationaladvantage yet. Earlier active text historical. [pcc:unverified]\n\n---\n\n'''
for p in [s/'G3_DESCENT_STATUS.md',s/'QUEUE.md',s/'CHECKPOINT.md',r/'.resume/RESUME.md']:p.write_text(current+p.read_text())
with (r/'docs/session-logs/2026-11-13.md').open('a') as f:f.write(f'''\n## {datetime.now(timezone.utc).isoformat()} — independent norm native handoff and fixture access diagnosis (Codex)

### What was done (commands, files, features)
- 4ea88c:exact SIXwholeconsumer native archives copied to original trusted publication route;remote bytes pending. Consumerunit02 75275f complete fullgroups/both2compositions/explicitcontrols;CLIaf3d78 nineclasses12processcontrols;13exactlocalPCCbindings andboth originalgates13/0trusted,c88b36. add561/aca431:whole actualoriginalAWSread2 unablecredentials/anonymous1 HeadObject403/nofixture;existing broadbenchgate1/189problems106records andnormal9apublication0 retained. [pcc:unverified]

### Decisions made (and why)
- Publish data-only handoff using normal expected-head Git;artifact1 EXACTSIXpendingobjects stays explicit,not a policy waiver. Preserve immutable whole archives/failed attempts. Continue consumer derivation/proofmap/report/CI/source work independently of missing root fixture.

### Open threads (unfinished, pick up next time)
- Actual6 remote replays/canonicalbot/consumer genuine manifests;consumer visual report/source/independentfull/ownCI. Rootglobal requires authentic fixture credentials/access;mainledger/signing/conflict/reviews and actual benchmark admission remain open.

### Errors (what went wrong, how it was handled)
- Installing originalAWSCLI removes tool absence but exposes actualcredential limit;anonymous access403. No fixture fabrication,benchmark gate promotion,source/test/pricing/verifier edit or inferred mathematical impossibility.
''')
d=s/'evidence/g3-norm-raw-handoff-governance-01';d.mkdir();shutil.copyfile(__file__,d/'original-command.py')
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-13.md','.resume/RESUME.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd,expected=0):
 p=subprocess.run(cmd,cwd=r,capture_output=True);(d/(name+'.txt')).write_bytes(p.stdout+p.stderr);(d/(name+'.exit')).write_text(str(p.returncode)+'\n');print(name,p.returncode,flush=True);assert p.returncode==expected,(p.stdout+p.stderr).decode()[:1500];return p
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','benchmarks','.github/workflows',str((s/'genus3-prime-descent').relative_to(r))],cwd=r)
subjects=[]
for p in paths():
 if p.startswith(b'pcc/claims/'):
  for proof in json.loads((r/p.decode()).read_text()).get('proofs',[]):
   if proof.get('kind')=='attestation':
    x=proof['subject'];assert hashlib.sha256((r/x['path']).read_bytes()).hexdigest()==x['sha256'];subjects.append(x['path'])
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode() for p in paths() if p.endswith(b'.md')]]);run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();f=Path('/workspace/scratch/ct56-root-norm-handoff.paths0');f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True);selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for p in subjects:assert subprocess.run(['git','cat-file','-e',base+':'+p],cwd=r,capture_output=True).returncode==0 or p.encode() in selected,p
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();p=run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
expected={str((s/'evidence/g3-norm-consumer-raw-handoff-01'/(name+'.tar.xz')).relative_to(r)) for name in ['static-01','static-02','static-03','unit-01','unit-02','cli-01']};assert {x.split()[1] for x in p.stdout.decode().splitlines() if x.strip()}==expected,p.stdout.decode()
(d/'custody.json').write_text(json.dumps(dict(base=base,ownledger_prose_whitespace_wiring=0,all343producerhashes_unchanged=True,no_native_or_workflow_or_verifier_or_benchmark_change=True,artifact_actual_exit=1,exact_six_data_objects_pending=sorted(expected),subjects_hash_and_staging=True,rootfixture='add561 actual accessgap;notgreen'),indent=2)+'\n')
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Data-only normrawhandoff governance:own9aledger/prose/workingcachedwhitespace/wiring0,all343producerhashes/no sourceworkflowverifierbenchmarkedit/subjecthashstagingcoverage. ArtifactACTUAL1 EXACTSIX wholeconsumerobjects awaiting genuine migration;not availabilitywaiver. Authenticrootfixture originalpull2 missingcredentials/anonymous1 HeadObject403 remains unresolved. Consumerfullnative/formal/CLI separateunsignedclaims;no globalpass orspeedup.']
for p in sorted(d.iterdir()):c+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(c,cwd=r,text=True).strip();Path('/workspace/scratch/ct56-root-norm-handoff-governance.claim').write_text(cl+'\n')
selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: preserve independent genus-three norm native records'],cwd=r,check=True)
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
