"""Metadata-only checkpoint after actual producer CI and publication."""
from pathlib import Path
from datetime import datetime,timezone
import subprocess,json,hashlib,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
base='9ee50f2a2090e28d1ca41cec416b77779955604f'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
now=datetime.now(timezone.utc).isoformat()
text='''# Current G3 checkpoint — producer CI and all published raw complete

Scientific source a719f6f75f67576ce3835d8acc6e47bb25a3cbe4 and corrected CI control source3d49af7f3d0a28e76b79e4825c9d2324e85853d2 are published. Dedicated own CI43388101185/job131761241701 SUCCESS,actual27e47f/custody279820:four native tests,nine bounded CLI points/golden comparisons,two original proof gates and11 exact bindings. All24 native JSON records equal accepted unit03. Whole45,679-byte UTF8 job/API/standard9-file15,108-byte archive retained. Actual CI is settled;do not repoll or redispatch. Initial zero-job failure1fd396 and frozen context repaira6d76b/b02647 remain immutable. [pcc:unverified]

Native unit03c162f3/CLI188d44/local integrated2d6fec/proofscopee6590f unchanged:all148/356/1324 source points,all7312 generating edges,all457 odd scalar images,even kernel2 each;all geometric branch/local-jet and origin controls. Two Lean modules14 audited including3 helpers,zero trusted;11 exact bindings. Actual geometry/reducer bridges remain written/native. Native3-page50,024-byte PDF,integration1-page33,148-byte PDF and continuation2-page41,376-byte PDF ALL inspected. No measured computational advantage. [pcc:unverified]

Actual raw publication COMPLETE at three distinct scopes:FOUR0775c2/f95;EIGHT920f04/bb6;FIVE9b93a4/9ee50f2a2090e28d1ca41cec416b77779955604f. Latest trusted-base43388106212/job131761261514 replays5 objects BEFORE directparent3d/exact13-path bot. Canonical manifests match original source Git blobs;normal local FF/artifact/wiring0 preserves2133 native inputs and owned dirty bytes. Whole427,588-byte UTF8 log/32-file64,772-byte publication archive/member+raw+archive hashes/exact roundtrip retained. No new pending large-data object is inferred from metadata. [pcc:unverified]

ROOT GLOBAL NOT GREEN:original cold900s124 beforetests/ae818d;warm1800s124 after6,330 suite began/d99199,all2133 guards unchanged. Exact original-binary diagnostic101/823054 identifies missing original ecbench records.jsonl,2364e3. Canonical manifest2,293,097bytes/SHA8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738. Original artifact pull2 because AWS CLI absent;scoped Git history returned no committed blob. No fabricated fixture or modified pricing/test logic. Next available remedy:install original AWSCLI in scratch and try SAME read-only pull;only after exact hydration rerun the exact failing test. [pcc:unverified]

ACTIVE NEXT RESEARCH:independent canonical-class norm in cryptanalysis ready PR1521,branch research/chained-transfer-g3-norm-20261113 under ready1520. Protocolac58df/sourcec3b8d74c ACTUALLY published before code,424a30,343 producer hashes/whole20,796-byte old six-job workflow. Static01 API-name101 wholee1f736,correction4710c0 frozen before edit;static02 strict0 whole22fb44. Fresh-cache unit01 four unfiltered tests ACTIVE under900seconds,357 guarded inputs,raw /workspace/scratch/ct56-g3-norm-unit-01. No norm result yet. Require independent full Jacobian counts/BFS,all norm edges,both[2] compositions,odd recovery,CLI,faithful written/feasible Lean proof and independent own CI. Original graphs checked:proposal diagram unchanged until actual norm outcome. [pcc:unverified]

NEXT:collect original consumer unit terminal/finalguards before selected-source edits;preserve failures/continue unblocked tasks. GitHub PR3877 and persistent issue3904 refreshed with actual CI/publication/fixture/norm state. Routine global check next09:44UTC;main conflict/frozen ledger/signing/review requirements remain. No force/rebase/rechain/lineage selection. Old C0/T20 full/ALL6CI/ALL4raw settled;KB127 source fallback/vault8edec422 locked/no subagents/no paid benchmark admission. Earlier active/pending text in historical files is superseded here.
'''
(s/'G3_DESCENT_STATUS.md').write_text(text)
line='Current canonical G3:see G3_DESCENT_STATUS.md. Producer CI43388101185/27e47f COMPLETE;all4+8+5 raw publications COMPLETE latest9b93a4/9ee. Root global missing original ecbench fixture2364e3 unresolved. Independent norm readyPR1521/protocolac58df publishedc3b BEFOREcode;static02 strict0/22fb44;full four-test unit01 ACTIVE/357guards. Next collect terminal before edits;main signing/ledger/conflict gates separate. Earlier CURRENT/ACTIVE paragraphs historical. [pcc:unverified]\n\n---\n\n'
for p in [s/'QUEUE.md',s/'CHECKPOINT.md',r/'.resume/RESUME.md']:
    p.write_text(line+p.read_text())
with (r/'docs/session-logs/2026-11-13.md').open('a') as f:
    f.write(f'''\n## {now} — actual producer CI and five-object publication checkpoint (Codex)

### What was done (commands, files, features)
- Actual own CI27e47f/custody279820:43388101185 exact3d SUCCESS,all4 native/9CLI/2proof/11bindings and24 matching JSON records. Actual final five-object publication9b93a4/custody67c6c8:43388106212/job131761261514 replays5 before9ee/directparent3d/exact13paths;normal FF/artifact/wiring0 and2133 native/dirty bytes unchanged. Whole raw/log/API/member digests/roundtrip retained. PR3877/issue3904 refreshed;norm ready1521/protocol published beforecode. [pcc:unverified]

### Decisions made (and why)
- Commit/publish metadata only;producer native/proof/control sources remain byte exact. Continue independent consumer unit01 while preserving selected357 inputs. Existing graphs remain the proposal until actual norm results.

### Open threads (unfinished, pick up next time)
- Consumer full native norm,proof/CLI/CI/report;root original fixture read remedy;main ledger/signing/conflict/review gates;destination IC and admitted complete comparisons.

### Errors (what went wrong, how it was handled)
- Root global remains NOTgreen due original timeout/missing fixture2364e3. Initial producer zero-job CI and API-name static consumer failure preserved;no criterion weakening or inferred speedup.
''')
d=s/'evidence/genus3-ci-completion-governance-01';d.mkdir();shutil.copyfile(__file__,d/'original-command.py')
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-13.md','.resume/RESUME.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd):
    p=subprocess.run(cmd,cwd=r,capture_output=True);(d/(name+'.txt')).write_bytes(p.stdout+p.stderr);(d/(name+'.exit')).write_text(str(p.returncode)+'\n');print(name,p.returncode,flush=True);assert p.returncode==0,(p.stdout+p.stderr).decode()[:1600]
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
run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
    ps=paths();f=Path('/workspace/scratch/ct56-root-ci-complete.paths0');f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0')
    subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True)
    subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True)
    selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
    for p in subjects:assert subprocess.run(['git','cat-file','-e',base+':'+p],cwd=r,capture_output=True).returncode==0 or p.encode() in selected,p
    return selected
stage();run('cached-whitespace',['git','diff','--cached','--check'])
tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(d/'custody.json').write_text(json.dumps(dict(base=base,all343_current_producer_hashes_unchanged=True,no_source_or_workflow_change=True,own_ledger_prose_whitespace_wiring_artifact=0,new_subject_hashes_and_staging=True,actual_CI='27e47f/279820 SUCCESS exact3d',actual_five_publication='9b93a4/67c6c8 exact9ee',rootglobal='notgreen missingfixture2364e3',next='consumer ready1521 unit01 active'),indent=2)+'\n')
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Metadata-only G3 CIcompletion checkpoint:own9eeledger/prose/workingcachedwhitespace/wiring/artifact0,all343producer hashes/source/workflows unchanged,newsubjecthashandstagingcoverage. ActualdedicatedCI27e47f/279820 allphases success;actual5publication9b93a4/67c6c8 complete. Rootglobal remains missingfixture2364e3;norm1521 fourtest unit01 active,not acceptance. No math/test/verifier/trust/benchmark change.']
for p in sorted(d.iterdir()):c+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(c,cwd=r,text=True).strip();Path('/workspace/scratch/ct56-root-ci-complete.claim').write_text(cl+'\n')
selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True)
subprocess.run(['git','commit','--quiet','-m','research: checkpoint complete genus-three CI and raw publication'],cwd=r,check=True)
print(cl,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
