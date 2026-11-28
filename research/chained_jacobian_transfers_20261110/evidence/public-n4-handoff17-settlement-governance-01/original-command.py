"""Governance and normal publication of completed custody metadata only."""
from pathlib import Path
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='b9616569ecf182f54a32e86afdc5213ec33eeca0'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
g=s/'evidence/public-n4-handoff17-settlement-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd):
 q=subprocess.run(cmd,cwd=r,capture_output=True);raw=q.stdout+q.stderr;(g/(name+'.txt.xz'if len(raw)>60000 else name+'.txt')).write_bytes(lzma.compress(raw)if len(raw)>60000 else raw);(g/(name+'.exit')).write_text(str(q.returncode)+'\n');print(name,q.returncode,flush=True);assert q.returncode==0,raw[:1000]
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','scripts','.github/workflows','benchmarks',str(s/'genus3-prime-descent')],cwd=r)
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]]);run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();p=Path('/workspace/scratch/ct77-root-handoff17-settled.paths0');p.write_bytes(b'\0'.join(x for x in ps if x!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(p),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
 selected=set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for name in selected:
  if name.startswith(b'pcc/claims/'):
   for proof in json.loads((r/name.decode()).read_text()).get('proofs',[]):
    if proof.get('kind')=='attestation':
     v=proof['subject'];assert hashlib.sha256((r/v['path']).read_bytes()).hexdigest()==v['sha256'];assert v['path'].encode()in selected or subprocess.run(['git','cat-file','-e',base+':'+v['path']],cwd=r,capture_output=True).returncode==0
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(g/'custody.json').write_text(json.dumps(dict(base=base,all343_originalproducerhashes_unchanged=True,original_native_workflows_verifiers_policies_benchmarks_unchanged=True,all_recorded_governance_exits=0,new_subject_hashes_and_staging=True,actual_publication_claim='PCC-20261114-ddd7a6',hold17='released after5replays/24445allbytes/23links/exactFF',scope='metadata only, unsigned, no rootregression-green/performance/mergewaiver'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Handoff17 settledmetadata governance: all343originalproducerhashes unchanged; originalledger/prose/working+cachedwhitespace/wiring/artifact0/exactsubjecthash+staging; originalnative-workflows-verifiers-policies-benchmarks unchanged. Actualddd7a6 fivegenuine remote replays/exactb9616569e/24445guards+ownedbytes+23links/hold17released retained. Native252fd8/proofsff888c/report7c153a/whole38roots6577members preserved; full64torsion/matchedJ2next under23ab48; fresh18forfuturedata; allunsigned/no rootgreen-performance ormergewaiver.']
for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: checkpoint whole census and nondual elliptic publication17'],cwd=r,check=True);head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip();print(claim,head,len(selected),flush=True)
q=subprocess.run(['python3','/workspace/scratch/publish_normal_git_checkpoint.py','isogeny-labs/crypto',base,head,'research/chained-jacobian-transfer-20261110','/workspace/scratch/ct77-root-handoff17-settlement-publication-01'],cwd=r,capture_output=True);Path('/workspace/scratch/ct77-root-handoff17-settlement-publication-caller.txt').write_bytes(q.stdout+q.stderr);print('normalpub',q.returncode,flush=True);assert q.returncode==0
