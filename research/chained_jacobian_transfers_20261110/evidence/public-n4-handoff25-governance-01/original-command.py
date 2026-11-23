"""Data custody only; enforce original unchanged policy and input hold25."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, shutil, subprocess

r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
base='27a680b622a69c96f2acc8a98d693eecd29ccc84'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-handoff25-governance-01';d.mkdir()
shutil.copyfile(__file__,d/'original-command.py')
scope=[str(s.relative_to(r)),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','docs/session-logs/2026-11-15.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
guard=s/'evidence/public-n4-current-guard-25'
inputs=json.loads(Path('/workspace/scratch/ct91-whole-closed-science-handoff-25/current-inputs.json').read_text())
links=json.loads((guard/'symlink-targets.json').read_text())
assert len(inputs)>=49753 and len(links)>=172
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
    ps=paths();f=Path('/workspace/scratch/ct91-root-whole25-01.paths0')
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
expected=set(json.loads(Path('/workspace/scratch/ct91-whole-closed-science-handoff-25/expected-migration-paths.json').read_text()))
assert expected and {x.split()[1] for x in p.stdout.decode().splitlines() if x.strip()}==expected,p.stdout.decode()
(d/'custody.json').write_text(json.dumps(dict(base=base,original_governance_exits=0,all343_producer_hashes_unchanged=True,original_native_workflow_verifier_policy_benchmarks_unchanged=True,artifact_actual_exit=1,exact_original_policy_selected_paths=sorted(expected),new_subject_hashes_and_staging_checked=True,guard25=len(inputs),links=links,handoff=json.loads((s/'evidence/public-n4-original-full-group-handoff-25/scope.json').read_text()),scope='unsigned custody25; whole full-original701/failed693/all21bindings/report761003/source-executable-preparedcopies/checkpoints18-19/ALLpending675 bytes; all49753prior24keys172links/bothledgerprefixes HELD; no cost-globalCI-signing promotion'),indent=2)+'\n')
statement=f'Handoff25governance: all343originalproducer sciencehashes/native/tools/workflows/verifier/policy/benchmarks unchanged; originalledger/prose/working-cachedwhitespace/wiring0/exactnewsubjects-staging. Originalartifact1 selects EXACT{len(expected)}declared originalpolicy paths; allotherobjects remainordinaryGit. All49753prior24keys172links/both exactconsumerledgerprefixes andALLpending675ownedbytes HELD untilORIGINALremote replays BEFOREactualpublisher/exactFF/reassembly/terminal25checks. Everydeclaredclosedct90root/failed693/success701/fiveactualomissions/source-executables-fullpreparedcopies/proofs/all21bindings/inspected58175Breport/checkpoints18-19/cleanup11fullinventory retained; two private signeddownloadmetadata files excludedonlyandretainedlocally. Prior genuine24archive24881096B/parts stayheld-exact withoutrecursivecopy. Nativee245d0/proofs7b5f49-da2fe7/report761003 andalloldnegatives unsigned; canonical701afteractualpublication/freshadditiveCI next; no calibrated/globalCI/signed/mergepromotion.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip()
Path('/workspace/scratch/ct91-root-whole25-01-governance.claim').write_text(cl+'\n')
selected=stage();intact()
subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','commit','--quiet','-m','research: preserve full original-group chain, failed run and proofs through genuine custody25'],cwd=r,check=True)
intact()
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
