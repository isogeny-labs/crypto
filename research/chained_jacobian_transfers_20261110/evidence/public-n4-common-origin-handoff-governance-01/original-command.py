"""Data custody only; enforce original unchanged policy and input hold07."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, shutil, subprocess

r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
base='073229fe4a89d090ea427bf916c9e07cbe4c5ae8'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-common-origin-handoff-governance-01';d.mkdir()
shutil.copyfile(__file__,d/'original-command.py')
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
guard=s/'evidence/public-n4-common-origin-current-guard-08'
inputs=json.loads((guard/'current-inputs.json').read_text())
links=json.loads((guard/'symlink-targets.json').read_text())
assert len(inputs)==4230
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
    assert p.returncode==expected,(p.stdout+p.stderr).decode()[:2000]
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
    ps=paths();f=Path('/workspace/scratch/ct70-root-common-origin.paths0')
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
expected=set(json.loads(Path('/workspace/scratch/ct70-root-publication-expected.json').read_text()))
assert len(expected)==8 and {x.split()[1] for x in p.stdout.decode().splitlines() if x.strip()}==expected,p.stdout.decode()
(d/'custody.json').write_text(json.dumps(dict(base=base,ownledger_prose_whitespace_wiring=0,all343producerhashes_unchanged=True,no_native_or_workflow_or_verifier_or_benchmark_change=True,artifact_actual_exit=1,exact_policy_selected_data_objects_pending=sorted(expected),subjects_hash_and_staging=True,current_guard08_count=4230,symlink_targets=links,rootfixture='authentic bytes restored/1ea795 targeted pass;unfiltered root retry03 124/two visible failures/fulltotalsunknown;notgreen'),indent=2)+'\n')
statement='Data-only common-origin handoff governance own073229fe:originalledger/prose/working+cachedwhitespace/wiring0/all343producerhashes unchanged/no native/workflow/verifier/benchmarkedit/allnewsubjecthashes+stagingcovered. OriginalartifactACTUAL1 selectsEXACT8largewholeobjects underunchangedpolicy;SEVENwholeclosedcaptures+SEVENcompleteindices+whole79399-bytefinalPDF15objects/8S3pending+7smallwholeGit/allfailures/memberroundtrips/sourceclosures intact. Includeswhole94file unpublishedsource/report/754CI/PCCledger/resume+base0a+patch checkpoint/restorablebeforeconsumercommit/no recursivepublicationarchive. Freshguard08actual4230native3304/current754CI+self755/latestphysical730/prior703/root2137/originalbinaryfixture/ALLreports+canonicalPDFs+linktypes HOLDuntilgenuinebotFF/finalguards. Old07historicalreleased/ONLYpostpublicationoldcanonicalREADMEdelta mapped/alloldscienceunchanged. Native8a77f3/written8ffc9c/reused57partialbindings/report6271ce;source755full/16CI241native57digests pending. Originaltransportffe675remote+localgate0exactclean39e9/laterchangesstale;rootfull124/clock101/unknowntotals remain. Remoteavailabilitypending;unsigned/no cost/globalrootgreen/mergewaiver.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip()
Path('/workspace/scratch/ct70-root-common-origin-governance.claim').write_text(cl+'\n')
selected=stage();intact()
subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','commit','--quiet','-m','research: preserve complete origin-control and checkpoint captures'],cwd=r,check=True)
intact()
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
