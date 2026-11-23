"""Checkpoint real fixture restoration and running original unit gate;no source edits."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';base='cdfce1bb4011d36a7c2512d6c5340f17bafd9b3b'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
now=datetime.now(timezone.utc).isoformat()
q='''# Current research queue — {at}

Actual3primefieldJacobian52maps/source45ee and refinementfb9ee/55partialbindings are published in cryptanalysisPR1523. Full703b38c7f/own12CI65dd08108native55canonical settled. NewtargetICnativee869bd solvesall15publiccases/all60attempts/2543guards;consumernew-target-ic source awaiting proof/report/strictCI publication. Standing unsigned,no computational advantage. [pcc:unverified]

1. Finish newtargetIC mathematicalsoundness/faithfulLean/report/strictpublishedsourcefullCI under prospective98c6f0/2aee. Preserveall60attempts includingfailures;costcoverage unknown. [pcc:unverified]
2. Collect original unfiltered rootlibraryretry03 running:/workspace/scratch/ct68-root-library-regression-03;UNCHANGEDcargo test --release --lib --offline --locked/SAME1800sbound/2137guards. Authenticfixture1ea795 nowrestoredoriginalSHA8970667e andsamebinaryformerlyfailedtestpasses1/6329filtered. Originalglobalgate remainsopen untilterminal;existingcontainernamespace/otherfixturefailures mayremain. [pcc:unverified]
3. Pack ct65full/CI,ct66native,ct67failedsetup/full02/12CI,ct68newIC+authenticfixture/targeted+fullregression wholecaptures+completeindices viaunchanged genuinepublisher. NEWcurrentguard07 required beforefuturebotFF;guard06 historical/released. [pcc:unverified]
4. Construct complete odddegree13^3 beta coordinates for matchedtarget;writtenfactor/both13/polarization13 exists,completecoordinatesopen. Noneof3currentgeometrickernelsfree52;wronguntransportedkernelcontrol retained. [pcc:unverified]
5. Original S3checkpoint/livebudget/capacity/completecold+onlinecost/strongestmatchedICrho references,actualgeometricformal/signedexecution/mainledger/signing/reviews/conflict/vault/KBecdb remain. No benchmarkadmission ormergewaiver. [pcc:unverified]

Authenticateddata route708cb8/3baaworkflow43516010720/job132198018704 successfully originalpull0/verify0/exact2293097bytes;WHOLEartifact14081195285/150168bytesSHA4df508a9/all16members/originalsource-tool-config-manifestbyteexact/localverify0/Gitstatusunchanged. Targetedtest1ea795 usesSAMEbinarySHA82029f63;all2137guards0,2historicalconsumerworkflowversions separatelypreserved. Originalrawfailedpreparation01 andpriorlocalnocreds/anonymous403 failuresremain. Ordinaryunitcheckpointoptout documented7433ce/originalPCCbypass+pushattempt1;performanceadmissionstillclosed. [pcc:unverified]
'''.format(at=now)
p=s/'QUEUE.md';p.write_text(q+'\n---\n\n'+p.read_text())
note='''
## {at} — authentic original fixture restored; original library regression retry

Actual1ea795:prospective708cb8/2aeebefore dataonly3baaworkflow;43516010720/job132198018704SUCCESS/original3dtool/config/manifest/pull0verify0;WHOLEartifact14081195285/150168bytesSHA4df508a9/all16members bytechecked. Exactoriginal2293097bytefixtureSHA8970667e restored inignoredoriginalpath/originalverify0/Gitstatusunchanged. SAMEcachedoriginalbinarySHA82029f63/exactformerfailedtestnow1pass/6329filtered/all2137guards0. Firstpreparation01stoppedbefore nativeon2historicalconsumerworkflowhashes;all2131otherinputs unchanged;37e18b keepsbothcurrent+historicalWFguards,no source/WFedit. [pcc:unverified]

Rootoriginalunfilteredretry03 is actuallyrunning at /workspace/scratch/ct68-root-library-regression-03:UNCHANGEDcargo test --release --lib --offline --locked/SAME1800sbound/originalwarmcache/2137guards,protocol7433ce. Ordinaryunitexception explicitlydocumented inAGENTS/checkpointhook;originalPCCappend-bypass0 andoriginalcheckpointpush1(noprofile/token) retained. This is notbenchmarkadmission;globalrootstillnotgreen beforeterminal. Closedderivedcompilerct65/ct67caches deletedonlyafter0/0/fullrawretained;ct55originalbinary andct57warmcache kept. [pcc:unverified]

Consumerrefinementfb9e fullb38c7f703/own12CI65dd08108native55canonical settled. ActualnewtargetICe869bd/all60attempts/all15publiccases/2543guards/82wholeJSON;allreportedanswers actualtarget+independentoriginalsourcechecked. Codecurrentlyuncommittednew-target-ic;nextmathematicalsoundness/faithfulLean/report/strictCIpublication. Fullcosts/strongestalgorithmreference/performanceadmission/completebetacoordinates/actualgeometry/mainmerge/vault/KB gapsremain. [pcc:unverified]
'''.format(at=now)
with(r/'docs/session-logs/2026-11-13.md').open('a')as f:f.write(note)
with(r/'problems/chained-transfer-admission-and-ledger-sync.md').open('a')as f:f.write(note)
d=s/'evidence/root-fixture-regression-current-metadata-01';d.mkdir()
for name in ['plan.json','original-command.sh','original-ledger-optout.log','original-ledger-optout.exit','original-checkpoint-push.log','original-checkpoint-push.exit']:shutil.copyfile(Path('/workspace/scratch/ct68-root-library-regression-03')/name,d/name)
shutil.copyfile('/workspace/scratch/ct68-derived-cache-cleanup-01.json',d/'derived-cache-cleanup.json')
g=s/'evidence/root-fixture-restoration-governance-01';g.mkdir();shutil.copyfile(__file__,g/'original-command.py')
scope=[str(s.relative_to(r)),'pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-13.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
def paths():return sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=r).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0')))-{b''})
def run(name,cmd):
 p=subprocess.run(cmd,cwd=r,capture_output=True);(g/(name+'.txt')).write_bytes(p.stdout+p.stderr);(g/(name+'.exit')).write_text(str(p.returncode)+'\n');print(name,p.returncode,flush=True);assert p.returncode==0,(p.stdout+p.stderr).decode(errors='replace')[:1500]
refs=json.loads(Path('/workspace/cryptanalysis/experiments/chained-transfer-reference/evidence/g3-norm-primary-01/upstream-current-inputs.json').read_text());assert len(refs)==343 and all(hashlib.sha256((r/p).read_bytes()).hexdigest()==h for p,h in refs.items())
assert not subprocess.check_output(['git','diff','--name-only',base,'--','src','isogeny_algos','tools','benchmarks','.github/workflows',str((s/'genus3-prime-descent').relative_to(r))],cwd=r)
subjects=[]
for p in paths():
 if p.startswith(b'pcc/claims/'):
  for proof in json.loads((r/p.decode()).read_text()).get('proofs',[]):
   if proof.get('kind')=='attestation':
    x=proof['subject'];assert hashlib.sha256((r/x['path']).read_bytes()).hexdigest()==x['sha256'];subjects.append(x['path'])
run('ledger',['python3','tools/pcc/pcc.py','ledger','verify','--base',base]);run('prose',['python3','tools/pcc/pcc.py','lint-prose',*[p.decode()for p in paths()if p.endswith(b'.md')]]);run('whitespace',['git','diff','--check','--',*scope]);run('wiring',['python3','tools/artifacts/wire_workflows.py','--check'])
def stage():
 ps=paths();f=Path('/workspace/scratch/ct68-root-fixture-metadata.paths0');f.write_bytes(b'\0'.join(p for p in ps if p!=b'.resume/RESUME.md')+b'\0');subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(f),'--pathspec-file-nul'],cwd=r,check=True);subprocess.run(['git','add','--sparse','-u','--','.resume/RESUME.md'],cwd=r,check=True);selected=set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=r).split(b'\0'))-{b''};assert selected==set(ps)
 for p in subjects:assert subprocess.run(['git','cat-file','-e',base+':'+p],cwd=r,capture_output=True).returncode==0 or p.encode()in selected,p
 return selected
stage();run('cached-whitespace',['git','diff','--cached','--check']);tree=subprocess.check_output(['git','write-tree'],cwd=r,text=True).strip();run('artifact',['python3','tools/artifacts/artifacts.py','check',base,tree])
(g/'custody.json').write_text(json.dumps(dict(parent=base,all343_original_producer_inputs_unchanged=True,no_native_workflow_verifier_or_benchmark_change=True,original_ledger_prose_whitespace_cachedwhitespace_wiring_artifact=0,all_new_PCC_subject_hashes_staging=True,actual_fixture='1ea795/original2293097bytesSHA8970667e/localverify0/wholeZIP4df508a9/16members',targeted='sameoriginalbinary/exacttest1pass6329filtered/native0/2137guards',global_regression='unchangedunfiltered1800s retry03 running;notgreen',scope='ordinaryunitoptoutdocumented/PCCbypass+originalcheckpointpush1;performanceadmissionstillclosed'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Root authenticfixture checkpoint governance owncdfce:all343originalproducerinputs unchanged/no native/workflow/verifier/benchmarkchange;originalledger/prose/whitespace/cachedwhitespace/wiring/artifact0/allnewsubjects hash/staging covered. Actual1ea795 authenticatedoriginalfixture2293097bytesSHA8970667e restoredoriginalverify0/wholeartifact14081195285SHA4df508a9/16members;SAMEoriginalbinary exactformerfailedtestpasses1/6329filtered/2137guards0. Originalglobalunfilteredretry03/SAME1800bound running/notgreen;ordinaryunitexception7433ce/originalPCCbypass+pushattempt1 retained,no performanceadmissionwaiver. Consumerrefinement703fullb38c7f/own12CI65dd08108native55digests settled;actualnewICe869bd15publiccases/60attempts awaitsproofreportstrictCIpublication. Futuregenuinewholepublication/NEWguard07 andmainledger/signing/reviews/conflict/geometry/vault/KB gapsremain.']
for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();selected=stage();subprocess.run(['git','diff','--cached','--check'],cwd=r,check=True);subprocess.run(['git','commit','--quiet','-m','research: restore authentic root fixture and replay original regression'],cwd=r,check=True)
print(claim,subprocess.check_output(['git','log','-1','--format=%H %T %P'],cwd=r,text=True).strip(),len(selected),flush=True)
