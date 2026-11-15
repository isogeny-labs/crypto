"""Whole closed evidence, pending source and current byte custody; I/O only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,io,json,lzma,os,shutil,subprocess,sys,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';scratch=Path('/workspace/scratch')
base='f8d1853a6bee6be54385a5c965e0e95528b6c23a';consumer='d5db7698edcb966a6b0b9edf330aba742b7c6dbf'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
old=json.loads((scratch/'ct74-bulk15-handoff-01/current-inputs.json').read_text());assert len(old)==17103
allowed={str(c/p)for p in ['.resume/RESUME.md','docs/session-logs/2026-11-14.md','experiments/chained-transfer-public-target/CHECKPOINT.md','experiments/chained-transfer-public-target/RESEARCH_QUEUE.md','.gitattributes']}
changed={p for p,h in old.items()if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h};assert changed==allowed
at=datetime.now(timezone.utc).isoformat()
state=f'''# Complete norm-halving implemented; genuine whole-data handoff16 — {at}

Native05/7ff793: FOURunfiltered4pass0fail0ignored0filtered/all892guards unchanged; two deliberately shifted origins rejected by original odd-membership validation; allTHREEprior04wholeJSONrecords exact WITHOUTMASKS. Direct/strongbaseline/immutablecomplete-C agree448exceptionals48mixed40publicsector-directions96additionpairs/allfourcorrectedhalves/bothNEW13compositions/fullNEWinverse/sourcechecks. Native04/52c01c andallfailedbuild/constructor/proof records retained. [pcc:unverified]

BothunchangedoriginalLean gates10declarations0trusted/9newexactbindings/82partialunique(3ba28f); exact-Z andunchangedoriginalidentitychecker issue/replay0/missingordinate-termactual101, missingwholecoordinate-recordactual101(d91395). TWO80943BPDFpages BOTHinspected(c33051), fullmodelIDs/editableactualflow/Rustrecordedselectedcallgraph. Existingproposal/C/ABsubjects unchanged. Commonfullnorm+ORDERJacadmission/fullinverse3413013/context/primitive/verificationwork INCLUDEDandUNPRICED; no calibratedcost orECDLPadvantage. [pcc:unverified]

Dedicated21stnormhalvingworkflow/helper written underNORM_HALVING_REPLAY_PROTOCOL_01:21phases19zero/twoactual101/4wholeJSONno masks/two10gates/9bindings. Shell/actionlint pass; fullreplay andownsourceCI PENDING. Existing20workflows/881inputs unchanged. Auxiliary Rust formatting andfixedcertificate added afternative05; fullnewsource willbe independently compiled inpublishedsource replay. [pcc:unverified]

NEWguard16 preservesall17103prior keys withONLYfive declared checkpointmetadata updates, all11wholeclosednative/proof/audit/reportinspectionroots andpendingconsumer/base/patch/ledger/resume, originalfullfixture259212B/report80943B/PNG69912B bytes, completeindices/currentactualproofsources andeightpriorlinktypes. No live preparation/controlstdout enters theheldcaptures. HOLD until originalgenuine remote-byte replays/exactpublisherFF/terminalguards+ownedbytes; then projectcanonicaldocs/relativelinks, freezeadditiveCIreferences, normalconsumercommit/push/freshphysicalreplay/own21CI/currentoriginalcheckpoint. [pcc:unverified]

Aftersettlement advancecompleteprimitivecoverage andprimary/available-prior-work review ofgenuine nondual13 continuation. Root124/clock101/mainledger/signing/review/conflict/fullgeometry/livebenchmarkadmission/lockedKB-vault persist; no paidlaunch orwaiver. Closedderivedincrementals88010036B72files reclaimedwithwholehashreceipt02, outsideallguard15paths; allscientificoutputs/binaries/sealedBcache retained. [pcc:unverified]
'''
# Sizes below are checked directly; the prose size field is descriptive only.
png=cs/'reports/norm-halving-results-01/selected-calls.png';state=state.replace('PNG69912B','PNG'+str(png.stat().st_size)+'B')
for p in [cs/'RESEARCH_QUEUE.md',cs/'CHECKPOINT.md',c/'.resume/RESUME.md']:p.write_text(state+'\n---\n\n'+p.read_text())
with(c/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n'+state)
o=scratch/'ct74-norm-halving-handoff-16';o.mkdir();d=s/'evidence/public-n4-norm-halving-handoff-16';d.mkdir();g=s/'evidence/public-n4-current-guard-16';g.mkdir()
names=['ct74-strong-baseline-native-01','ct74-strong-baseline-native-02','ct74-norm-halving-native-03','ct74-norm-halving-native-04','ct74-norm-halving-native-05','ct74-norm-halving-Lean-01','ct74-norm-halving-Lean-02','ct74-norm-halving-Lean-03','ct74-norm-halving-auxiliary-01','ct74-norm-halving-audit-02','ct74-norm-halving-results-inspection-01'];assert all((scratch/n).is_dir()for n in names)
scope=['experiments/chained-transfer-public-target','docs/research-artifacts/chained-transfer','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','.github/workflows/chained-transfer-norm-halving.yml','problems/chained-transfer-odd-product-kernel-and-odd-factor.md']
pending=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=c).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=c).split(b'\0')))-{b''})
files=[(p,'whole-runs/'+n+'/'+p.relative_to(scratch/n).as_posix())for n in names for p in sorted((scratch/n).rglob('*'))if p.is_file()or p.is_symlink()]
files +=[(c/n.decode(),'consumer-pending/'+n.decode())for n in pending]
for p in [scratch/'ct74-closed-incremental-cleanup-02.json.xz',scratch/'ct74_cleanup_closed_incrementals_02.py',Path(__file__)]:files.append((p,'closed-controls/'+p.name))
assert len({n for _,n in files})==len(files)
current={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in old};members=[];archive=o/'norm-halving-whole-runs-and-pending-source.tar.xz'
controls={'control/base.txt':(consumer+'\n').encode(),'control/pending-full.patch':subprocess.check_output(['git','diff',consumer,'--binary','--full-index','--',*scope],cwd=c),'control/snapshot.json':(json.dumps({'at':at,'root_base':base,'consumer_base':consumer,'whole_roots':names,'pending_paths':len(pending),'scope':'all closed roots whole, no filtering; no live caller/stdout captured; source CI not yet run'},indent=2)+'\n').encode()}
with tarfile.open(archive,'w:xz',dereference=False)as t:
 for p,name in sorted(files,key=lambda x:x[1]):
  b=p.read_bytes();h=hashlib.sha256(b).hexdigest();current[str(p)]=h;row={'path':name,'bytes':len(b),'sha256':h,'local_original':str(p)}
  if p.is_symlink():row['symlink_target']=os.readlink(p)
  members.append(row);info=t.gettarinfo(str(p),arcname=name);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
  if p.is_symlink():t.addfile(info)
  else:t.addfile(info,io.BytesIO(b))
 for name,b in sorted(controls.items()):
  info=tarfile.TarInfo(name);info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b));members.append({'path':name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
with tarfile.open(archive,'r:xz')as t:
 assert len(t.getmembers())==len(members)
 for row in members:
  m=t.getmember(row['path'])
  if 'symlink_target'in row:assert m.issym()and m.linkname==row['symlink_target']
  else:
   b=t.extractfile(m).read();assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']
index=d/'whole-members.json';index.write_text(json.dumps({'whole_roots':names,'all_members_no_filter':True,'all_members_roundtripped':True,'members':members},indent=2)+'\n')
# Pin original producer and consumer scientific inputs again; original tools stay unchanged.
ref=cs/'ci_beta_complete/reference-01.sha256'
for line in ref.read_text().splitlines():
 h,p=line.split('  ',1);q=c/p;assert hashlib.sha256(q.read_bytes()).hexdigest()==h;current[str(q)]=h
current[str(ref)]=hashlib.sha256(ref.read_bytes()).hexdigest()
for p in (cs/'norm-halving-evaluator').rglob('*'):
 if p.is_file():current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
p=Path('/workspace/crypto/target/ct74-norm-halving-proof-stage-01/NormHalving.lean');current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
current[str(archive)]=hashlib.sha256(archive.read_bytes()).hexdigest()
(o/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n');(o/'whole-current-inputs.json.xz').write_bytes(lzma.compress((o/'current-inputs.json').read_bytes()))
links={p:os.readlink(p)for p in current if Path(p).is_symlink()};assert len(links)==8
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n');(g/'previous15-to-current16.json').write_text(json.dumps({'prior_keys':len(old),'all_prior_keys_retained':True,'only_declared_preexisting_changed_paths':sorted(changed),'reason':'five released checkpointmetadata files, oldbytes retained by prior genuine publication; all prior scientific/raw source paths exact','prior_key_after_metadata_updates':{p:current[p]for p in sorted(changed)}},indent=2)+'\n')
objects=[]
def obj(src,name,kind):
 dst=d/name;shutil.copyfile(src,dst);objects.append({'path':dst.relative_to(r).as_posix(),'bytes':dst.stat().st_size,'sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),'local_original':str(src),'kind':kind})
obj(archive,archive.name,'eleven complete closed roots and pending whole source checkpoint')
for src,name,kind in [(c/'docs/research-artifacts/chained-transfer/norm-halving-01/expected-native-05.json.xz','expected-native-05.json.xz','all four full native JSON records'),(cs/'reports/norm-halving-results-01/report.pdf','report.pdf','whole inspected two-page actual-results report'),(png,'selected-calls.png','whole selected-entry rendering'),(o/'whole-current-inputs.json.xz','whole-current-inputs.json.xz','whole current input guard16')]:obj(src,name,kind)
spec=importlib.util.spec_from_file_location('original_artifacts',r/'tools/artifacts/artifacts.py');mod=importlib.util.module_from_spec(spec);sys.modules[spec.name]=mod;spec.loader.exec_module(mod);policy=mod.Config.load(r).policy
for row in objects:row['original_policy_requires_store']=mod.belongs_in_store(row['path'],row['bytes'],policy)
assert all(row['original_policy_requires_store']for row in objects)
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(o/'expected-migration-paths.json').write_text(json.dumps([x['path']for x in objects],indent=2)+'\n')
summary={'at':at,'root_base':base,'consumer_base':consumer,'guard16':len(current),'all17103prior_keys_retained':True,'declared_metadata_updates':sorted(changed),'whole_roots':names,'members':len(members),'whole_index_sha256':hashlib.sha256(index.read_bytes()).hexdigest(),'objects':objects,'actual_remote_replays_required':len(objects),'link_types':links,'hold':'all held input/raw/owned byte edits forbidden until original remote-byte replays/exactFF/terminalguards','scope':'unsigned custody only; native/proofs/report remain their exact existing claims, source/full/CI pending'}
(d/'scope.json').write_text(json.dumps(summary,indent=2)+'\n');(g/'custody.json').write_text(json.dumps(summary,indent=2)+'\n');shutil.copyfile(__file__,d/'original-command.py')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Freshguard16 wholedata custody: all17103prior keysretained withONLYfive declared released checkpointmetadata updates; alleleven wholeclosed baseline/native/Lean/auxiliary/audit/reportinspection roots ANDpendingconsumer/base/patch/ledger/resume preservedwhole withoutfiltering/everymemberroundtripped. OriginalfullFOURrecordfixture259212B/TWO80943BPDFpages/wholePNG andcompletecurrentguard map included; actualproofsources/allold881science+manifest/eightpriorlinks held. UnchangedoriginalConfig+policy requireFIVEgenuine remote-byte replays beforeexactpublisherFF/terminalguard+ownedbyte release; no live owncaller captured, no repack/prune/waiver/nativeCIperformance promotion. SourceCI/fullreplay pending; continue afterrelease.' ]
for p in [d/'objects.json',d/'scope.json',index,g/'custody.json',g/'symlink-targets.json',g/'previous15-to-current16.json']:cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(claim+'\n');print(claim,'guard',len(current),'members',len(members),'fiveobjects',[x['bytes']for x in objects],flush=True)
state='\n\n## Norm-halving whole handoff16 ready\n\n'+claim+' / sourceconsumerd5db7698: native05fourunfiltered/892guards/allprior3wholeJSONexact(7ff793), two10gates/ninebindings/82partial(3ba28f), identity/omission101controls(d91395), TWOinspectedactualreportpages(c33051). Elevenwholeclosedroots+pendingwholeconsumer/base/patch/allfourfixture/PDF/PNG/currentguard16; originalpolicy selectsFIVEobjects, allold17103keys/eightlinks retained. HOLD until genuineoriginalremote replays/exactFF/terminalguards+ownedbytes; thennormalconsumer source/freshphysicalreplay/own21CI. No paidlaunch/rootgreen/benchmark/mergewaiver. [pcc:unverified]\n'
for p in [s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md']:p.write_text(state+'\n---\n\n'+p.read_text())
with(r/'docs/session-logs/2026-11-14.md').open('a')as f:f.write(state)
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
