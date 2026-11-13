"""Data-only handoff checkpoint; original artifact policy remains enforced."""
from pathlib import Path
from datetime import datetime,timezone
import json,subprocess,hashlib,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='e63db78c6a96b3c0caf63e4eaf00544c03255434'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-v2-final-metadata-governance-01';d.mkdir();shutil.copyfile(__file__,d/'original-command.py')
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
 ps=paths();f=Path('/workspace/scratch/ct58-root-public-n4-v2-final.paths0');f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True);selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for p in subjects:assert subprocess.run(['git','cat-file','-e',base+':'+p],cwd=r,capture_output=True).returncode==0 or p.encode() in selected,p
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree]);expected=set()
(d/'custody.json').write_text(json.dumps(dict(base=base,ownledger_prose_whitespace_wiring=0,all343producerhashes_unchanged=True,no_native_or_workflow_or_verifier_or_benchmark_change=True,artifact_actual_exit=0,pending_large_data_objects=sorted(expected),subjects_hash_and_staging=True,rootfixture='add561 actual accessgap;notgreen'),indent=2)+'\n')
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','PublicN4 finalmetadata governance owne63ledger/prose/workingcachedwhitespace/wiring/artifact0/all343producerhashes unchanged/newsubjecthashstagingcoverage. ActualV2source093/255paths/f0135 committed and11genuineWHOLEarchive/indexobjects637e1a20436e plus3smallGitindices retained;alloriginalfailures/no thresholdwaiver. Finalcontrolplane retaineddirectwhole/no recursivearchive. P1full67baa2/CIa5a143/rhoe5d852/actualV2a5a4a6-a59e6a/17partiallocalbindings unsigned;newV2fullCI/completed-descent candidate/costadmission andmaingates remainOPEN;old2283guardreleased/stalefutureFFmustnewversion.']
for p in sorted(d.iterdir()):c+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(c,cwd=r,text=True).strip();Path('/workspace/scratch/ct58-root-public-n4-v2-final-governance.claim').write_text(cl+'\n')
selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: checkpoint actual q223 bridge and complete seven-attempt data publication'],cwd=r,check=True)
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
