"""Data custody only; enforce original unchanged policy and input hold10."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, shutil, subprocess

r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
base='556c5ddb454ebdea9a70e2a1a33bdb74152cfbf5'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-complete-beta-png-handoff-governance-01';d.mkdir()
shutil.copyfile(__file__,d/'original-command.py')
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
guard=s/'evidence/public-n4-complete-beta-current-guard-12'
inputs=json.loads(Path('/workspace/scratch/ct73-png-handoff-02/current-inputs.json').read_text())
links=json.loads((guard/'symlink-targets.json').read_text())
assert len(inputs)==6210
def intact():
    assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in inputs.items())
    assert all(Path(p).is_symlink() and os.readlink(p)==v for p,v in links.items())
intact()
def paths():
    return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd,expected=0):
    p=subprocess.run(cmd,cwd=r,capture_output=True)
    (d/(name+'.txt')).write_bytes(p.stdout+p.stderr)
    (d/(name+'.exit')).write_text(str(p.returncode)+'\n')
    print(name,p.returncode,flush=True)
    assert p.returncode==expected,(p.stdout+p.stderr).decode(errors='replace')[:2000]
    return p
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text())
assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','benchmarks','.github/workflows',str((s/'genus3-prime-descent').relative_to(r))],cwd=r)
subjects=[]
for p in paths():
    if p.startswith(b'pcc/claims/'):
        for proof in json.loads((r/p.decode()).read_text()).get('proofs',[]):
            if proof.get('kind')=='attestation':
                x=proof['subject'];assert hashlib.sha256((r/x['path']).read_bytes()).hexdigest()==x['sha256'];subjects.append(x['path'])
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base])
run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode() for p in paths() if p.endswith(b'.md')]])
run('whitespace',['git','diff','--check','--',*scope])
run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
    ps=paths();f=Path('/workspace/scratch/ct73-root-png.paths0')
    f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0')
    subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True)
    subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
    selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''}
    assert selected==set(ps)
    for p in subjects:
        assert subprocess.run(['git','cat-file','-e',base+':'+p],cwd=r,capture_output=True).returncode==0 or p.encode() in selected,p
    return selected
stage();run('cached-whitespace',['git','diff','--cached','--check'])
tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip()
p=run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree],1)
expected=set(json.loads(Path('/workspace/scratch/ct73-png-handoff-02/expected-migration-paths.json').read_text()))
assert len(expected)==3 and {x.split()[1] for x in p.stdout.decode().splitlines() if x.strip()}==expected,p.stdout.decode()
(d/'custody.json').write_text(json.dumps(dict(base=base,ownledger_prose_whitespace_wiring=0,all343producerhashes_unchanged=True,no_native_or_workflow_or_verifier_or_benchmark_change=True,artifact_actual_exit=1,exact_three_whole_objects_pending=sorted(expected),subjects_hash_and_staging=True,current_guard12_count=len(inputs),symlink_targets=links,whole_checkpoint_members=631,scope='whole PNG151899B/source+17terminalStageCdirectories checkpoint7964660B/currentguardXZ137336B; unsigned data, no policy waiver'),indent=2)+'\n')
statement='Whole complete-beta PNG/source handoff governance: original ledger/prose/working+cachedwhitespace/wiring0/all343producerhashes unchanged/allnewsubjecthash+stagingchecked/no rootnative-engine-workflow-verifier-benchmarkchange. Originalartifact ACTUAL1 selectsEXACTTHREE wholePNG151899B f4a048/checkpoint7964660B631members/wholeguardmap137336B under unchanged policy. ALLpendingconsumer files+17wholeterminalStageCnative/proof/caller/reportdirectories/base44df/patch/PCCledger/resume preserved; no splitprune/resize. NEWguard12 all6210paths/SIXlinks/bothrepoownedbytes HOLDuntilgenuine3remote-byte replays/exactbotFF/terminalguards. Prior5268allretained; ONLYtwo dependencyhelper398 postrelease11 changes mapped. Completeconsumer881+self882/full20CIpending; priorA-B/ca9/helper/398/C77/futurefullCI wholecaptures NEWguard13. Allunsigned/no calibratedprice/advantage/rootgreen/mergewaiver.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip()
Path('/workspace/scratch/ct73-root-png-governance.claim').write_text(cl+'\n')
selected=stage();intact()
subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','commit','--quiet','-m','research: preserve complete transfer source and whole report image for publication'],cwd=r,check=True)
intact()
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
