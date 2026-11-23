"""Data custody only; enforce original unchanged policy and input hold16."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, shutil, subprocess

r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
base='830ba7e5394cb2eba0313b1e07c8b6bb32695aa4'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-handoff21-governance-01';d.mkdir()
shutil.copyfile(__file__,d/'original-command.py')
scope=[str(s.relative_to(r)),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','docs/session-logs/2026-11-15.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
guard=s/'evidence/public-n4-current-guard-21'
inputs=json.loads(Path('/workspace/scratch/ct82-whole-closed-science-handoff-21/current-inputs.json').read_text())
links=json.loads((guard/'symlink-targets.json').read_text())
assert len(inputs)>=40443 and len(links)>=150
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
    ps=paths();f=Path('/workspace/scratch/ct82-root-whole21-01.paths0')
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
expected=set(json.loads(Path('/workspace/scratch/ct82-whole-closed-science-handoff-21/expected-migration-paths.json').read_text()))
assert expected and {x.split()[1] for x in p.stdout.decode().splitlines() if x.strip()}==expected,p.stdout.decode()
(d/'custody.json').write_text(json.dumps(dict(base=base,original_governance_exits=0,all343_producer_hashes_unchanged=True,original_native_workflow_verifier_policy_benchmarks_unchanged=True,artifact_actual_exit=1,exact_original_policy_selected_paths=sorted(expected),new_subject_hashes_and_staging_checked=True,guard21=len(inputs),links=links,handoff=json.loads((s/'evidence/public-n4-paired-fourth-handoff-21/scope.json').read_text()),scope='unsigned wholedata21; every member of14declaredclosedscientificroots andcontrols/proof/report/checkpoint/wholeCI/native16recorddata pluspublishedc031base+pending4metadata. All40443prior keys/150links preserved, exact32MiBarchiveparts, no scientific/CI-wide/signed/performance promotion'),indent=2)+'\n')
statement='Handoff21 governance: original343producerhashes unchanged andoriginalnative-workflows-verifier-policy-benchmarks unchanged. Originalledger/prose/working+cachedwhitespace/wiring0/exactsubjecthashes-staging. Originalartifact1 selects EXACT declaredoriginal-policy objects; allunselectedobjects stayexactGitblobs. ALL40443prior keys/150oldlinks pluscurrent21science/source/raw/actualbinaries-librarycopies/failures/proofs/reports/wholeCI/checkpoints/currentc031base+pending4metadata HELDuntilgenuineoriginalremote replays BEFOREactualbot/exactnormalFF/orderedwholearchive reconstruction/terminal21guards-links-ownedbytes. Exact32MiBbyteparts avoidGitHub100MiBlimitwithoutpolicychange/datafilter/recordloss. All14declaredwholeclosedscientificroots/memberhashes roundtrip; previous20genuinepayload remainsheld/exactwithoriginalmanifest metadata,no recursivepayloadcapture. Native75338a D67negative/D39positive, proofs/report andprospective5caf67 unchanged. No newJacpointmap, signing,CI-wide,coldcensus,paidlaunch,performance,novelty ormergewaiver.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip()
Path('/workspace/scratch/ct82-root-whole21-01-governance.claim').write_text(cl+'\n')
selected=stage();intact()
subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','commit','--quiet','-m','research: publish whole fourth-edge science and frozen complete-chain protocol'],cwd=r,check=True)
intact()
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
