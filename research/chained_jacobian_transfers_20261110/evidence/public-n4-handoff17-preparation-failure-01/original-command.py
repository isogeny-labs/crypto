"""Whole closed capture publication and byte holds; I/O only, no research arithmetic."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,io,json,lzma,os,shutil,subprocess,sys,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';scratch=Path('/workspace/scratch');root_head='6444d7f50efb48e7cb49124eb5bc940456e2e9bc';consumer='186942f2eb96d4d26b079bd031d5cb3ae2b6bdd0'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==root_head
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=c)
old=json.loads((scratch/'ct74-norm-halving-handoff-16/current-inputs.json').read_text());assert len(old)==17911
allowed={str(c/p)for p in ['.resume/RESUME.md','docs/session-logs/2026-11-14.md','experiments/chained-transfer-public-target/CHECKPOINT.md','experiments/chained-transfer-public-target/RESEARCH_QUEUE.md','.gitattributes','pcc/ledger.jsonl']}
changed={p for p,h in old.items()if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h};assert changed==allowed
with tarfile.open(scratch/'ct74-norm-halving-handoff-16/norm-halving-whole-runs-and-pending-source.tar.xz','r:xz')as t:old_ledger=t.extractfile('consumer-pending/pcc/ledger.jsonl').read()
assert hashlib.sha256(old_ledger).hexdigest()==old[str(c/'pcc/ledger.jsonl')]
assert(c/'pcc/ledger.jsonl').read_bytes().startswith(old_ledger)
root_scope=[str(s.relative_to(r)),'.gitattributes','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','problems/chained-transfer-admission-and-ledger-sync.md']
root_pending=sorted(set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*root_scope],cwd=r).split(b'\0'))-{b''})
state='''# Genuine whole-data handoff17 prepared; hold before further source edits\n\nConsumer186942f2 publishesactualnondual phaseA252fd8, bothoriginalcompositiongates/twobindingsff888c andONEinspected46015Breport7c153a. E2 a(12,143,39)b(115,82,144)/2088c9...; bothnew13identities/whole169curve/separable/nondual12gcd versusolddual168/control101/fourwholeJSON/no masks. Full64torsion/matchedF223J2 andsecondJacobianmaps remainopen. Priorpartialarithmetic12cold/108missing/two60reuse/unpricedtradeoff preserved; no calibratedadvantage. [pcc:unverified]\n\nNEW17 wholearchives includeallclosedfresh02/ownCI/allnative binaries/source/primary/failedcanary/ledger/proof/report/partialcounts/nativeA andcompleteconsumer plusprior16rootpendingmetadata. All17911priorpaths retained; onlysixreleasedconsumer metadataupdates declared, ledgeroldbytes remainexactprefix andwholeoldarchive. Preserveallsource/raw/elevenlinktargets/ownedbytes untiloriginalgenuine remote-byte replays, exactpublisherFF andterminal17checks releasehold. No ownlivecaller captured. No code orconsumerledger editsduringhold. [pcc:unverified]\n\nAfterreleaseprojectoriginalnativeA100644Bfixture and67256BwholepositiveXZ into canonicaldocswithtransparentlinks, checkpoint normally, thenimplementalreadyprospective23ab48 full64actualgamma2conjugate producttransport/matchedkernel/HLPmodel, preservingunsupportedmatchedoutcome. Root124/clock101/mainledger/signing/reviews/conflicts/fullgeometry/livebenchmarkadmission/matchedrefs/lockedvault8edec422/KB127 remain. No paidlaunch orwaiver. [pcc:unverified]\n\n---\n\n'''
for p in [c/'.resume/RESUME.md',cs/'CHECKPOINT.md',cs/'RESEARCH_QUEUE.md']:p.write_text(state+p.read_text())
f=c/'docs/session-logs/2026-11-14.md';f.write_text(f.read_text().rstrip()+'\n\n'+state.split('\n---')[0].rstrip()+'\n')
names=['ct74-norm-halving-full-01','ct74-norm-halving-full-02','ct74-norm-halving-driver-02','ct74-norm-halving-replay-layout-01','ct74-norm-halving-replay-layout-02','ct74-norm-halving-own-CI-01','ct74-norm-halving-CI-audit-01','ct75-norm-arithmetic-review-01','ct75-nondual13-primary-review-01','ct75-norm-arithmetic-proposal-inspection-01','ct75-norm-arithmetic-layout-01','ct75-norm-arithmetic-ledger-01','ct75-norm-arithmetic-fixture-01','ct75-norm-arithmetic-controls-01','ct75-norm-arithmetic-parity-01','ct75-norm-arithmetic-mode-canary-01','ct75-norm-arithmetic-old-canaries-01','ct75-norm-arithmetic-quad-canaries-01','ct75-norm-arithmetic-quad-canaries-02','ct75-norm-arithmetic-quad-canaries-03','ct75-qcanary-feature-diagnosis-01','ct75-norm-arithmetic-analyzer-build-01','ct75-norm-arithmetic-panels-01','ct75-norm-arithmetic-cold-timeout-01','ct75-norm-arithmetic-partial-analysis-01','ct75-norm-inverse-proof-01','ct75-nondual13-proposal-inspection-01','ct75-norm-partial-results-inspection-01','ct75-original-fixture-cache-closure-01','ct75-current-original-checkpoint-01','ct76-checkpoint-discovery-01','ct76-current-original-checkpoint-01','ct76-nondual13-build-01','ct76-nondual13-native-01','ct76-nondual13-audit-01','ct76-nondual-composition-proof-01','ct76-nondual-composition-proof-02','ct76-phaseA-results-inspection-01']
assert all((scratch/n).is_dir()for n in names)
# Freeze the exact list of already-closed auxiliary commands/callers, never this live caller.
controls=[p for p in scratch.iterdir()if p.is_file()and p.name.startswith(('ct74-','ct74_','ct75-','ct75_','ct76-','ct76_'))and not p.name.startswith(('ct76-prepare-handoff17','ct76_prepare_handoff17'))]
assert not any('private' in p.name.lower()for p in controls)
owned_scope=['experiments/chained-transfer-public-target','docs/research-artifacts/chained-transfer','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','.github/workflows/chained-transfer-norm-halving.yml','problems/chained-transfer-odd-product-kernel-and-odd-factor.md']
consumer_names=sorted(set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*owned_scope],cwd=c).split(b'\0'))-{b''})
files=[(p,'whole-runs/'+name+'/'+p.relative_to(scratch/name).as_posix(),True)for name in names for p in sorted((scratch/name).rglob('*'))if p.is_file()or p.is_symlink()]
files +=[(c/p.decode(),'consumer-source/'+p.decode(),True)for p in consumer_names if(c/p.decode()).is_file()or(c/p.decode()).is_symlink()]
files +=[(r/p.decode(),'root-prior16-pending/'+p.decode(),False)for p in root_pending if(r/p.decode()).is_file()or(r/p.decode()).is_symlink()]
files +=[(p,'closed-controls/'+p.name,True)for p in controls]
files +=[(Path(__file__),'control/original-prepare-command.py',False)]
for p in [r/'target/ct75-norm-inverse-proof-stage-01/QuadraticInverse.lean',r/'target/ct76-nondual-composition-stage-01/NondualComposition.lean',r/'target/ct76-nondual-composition-stage-02/NondualComposition.lean']:files.append((p,'actual-proof-stage/'+p.parent.name+'/'+p.name,True))
assert len({n for _,n,_ in files})==len(files)
o=scratch/'ct76-whole-census-and-nondual-handoff-17';o.mkdir();d=s/'evidence/public-n4-census-nondual-handoff-17';d.mkdir();g=s/'evidence/public-n4-current-guard-17';g.mkdir()
archive=o/'whole-census-nondual-and-closed-replays.tar.xz';current={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in old};rows=[]
with tarfile.open(archive,'w:xz',dereference=False)as t:
 for p,name,hold in sorted(files,key=lambda x:x[1]):
  b=p.read_bytes();h=hashlib.sha256(b).hexdigest();row={'path':name,'bytes':len(b),'sha256':h,'local_original':str(p),'held_original':hold}
  if p.is_symlink():row['symlink_target']=os.readlink(p)
  if hold:current[str(p)]=h
  info=t.gettarinfo(str(p),arcname=name);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
  if p.is_symlink():t.addfile(info)
  else:t.addfile(info,io.BytesIO(b))
  rows.append(row)
 for name,b in [('control/root-base.txt',(root_head+'\n').encode()),('control/consumer-base.txt',(consumer+'\n').encode()),('control/consumer-pending.patch',subprocess.check_output(['git','diff',consumer,'--binary','--full-index','--',*owned_scope],cwd=c))]:
  info=tarfile.TarInfo(name);info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b));rows.append({'path':name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
with tarfile.open(archive,'r:xz')as t:
 assert len(t.getmembers())==len(rows)
 for row in rows:
  member=t.getmember(row['path'])
  if 'symlink_target'in row:assert member.issym()and member.linkname==row['symlink_target']
  else:
   b=t.extractfile(member).read();assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']
current[str(archive)]=hashlib.sha256(archive.read_bytes()).hexdigest()
(o/'whole-members.json').write_text(json.dumps({'all_whole_closed_roots':names,'all_members_no_filter':True,'all_members_roundtripped':True,'members':rows},indent=2)+'\n')
(o/'whole-members.json.xz').write_bytes(lzma.compress((o/'whole-members.json').read_bytes()))
(o/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n');(o/'whole-current-inputs.json.xz').write_bytes(lzma.compress((o/'current-inputs.json').read_bytes()))
links={p:os.readlink(p)for p in current if Path(p).is_symlink()};assert len(links)==11
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n');(g/'previous16-to-current17.json').write_text(json.dumps({'all17911prior_keys_retained':True,'only_declared_released_metadata_updates':sorted(changed),'ledger_old_bytes_exact_prefix':True,'old_ledger_original_sha256':old[str(c/'pcc/ledger.jsonl')],'original_old_ledger_archive':'genuine16 consumer-pending/pcc/ledger.jsonl','new_metadata_hashes':{p:current[p]for p in sorted(changed)}},indent=2)+'\n')
objects=[]
def obj(src,name,kind):
 dst=d/name;shutil.copyfile(src,dst);objects.append({'path':dst.relative_to(r).as_posix(),'bytes':dst.stat().st_size,'sha256':hashlib.sha256(dst.read_bytes()).hexdigest(),'local_original':str(src),'kind':kind})
obj(archive,archive.name,'all38wholeclosedroots/allcontrols/currentwholeconsumer/prior16rootpendingmetadata')
obj(o/'whole-members.json.xz','whole-members.json.xz','complete unfiltered member index')
obj(o/'whole-current-inputs.json.xz','whole-current-inputs.json.xz','complete input hold17')
obj(scratch/'ct76-nondual13-audit-01/all-native-records.json.xz','expected-native-01.json.xz','allFOURwhole actual elliptic native records')
obj(scratch/'ct76-nondual13-audit-01/whole-positive-native.txt.xz','whole-positive-native.txt.xz','whole unfiltered native stdout/stderr, no truncation')
spec=importlib.util.spec_from_file_location('original_artifacts',r/'tools/artifacts/artifacts.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m);policy=m.Config.load(r).policy
for x in objects:x['original_policy_requires_store']=m.belongs_in_store(x['path'],x['bytes'],policy)
assert all(x['original_policy_requires_store']for x in objects)
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(o/'expected-migration-paths.json').write_text(json.dumps([x['path']for x in objects],indent=2)+'\n')
summary={'at':datetime.now(timezone.utc).isoformat(),'root_base':root_head,'consumer_base':consumer,'guard17':len(current),'prior_keys':17911,'declared_metadata_updates':sorted(changed),'ledger_old_prefix_preserved':True,'whole_closed_roots':names,'archive_members':len(rows),'every_member_roundtripped':True,'consumer_source_files':len(consumer_names),'prior_root_pending_files':len(root_pending),'closed_auxiliary_controls':len(controls),'all_five_objects':objects,'all11_link_targets':links,'hold':'until original5remote replays/exactpublisherFF/terminalallguard17+ownedbytes checks','scope':'unsigned whole custody; no source/native/CI/performance ormerge-waiver promotion; full64torsion/matchedJ2 still next'}
(d/'scope.json').write_text(json.dumps(summary,indent=2)+'\n');(g/'custody.json').write_text(json.dumps(summary,indent=2)+'\n');shutil.copyfile(__file__,d/'original-prepare-command.py')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','NEW17 wholedata custody: all17911prior keys retained withONLYsixdeclared releasedconsumer metadataupdates/oldledgerexactprefix. All38wholeclosedfresh-full-CI-primary-prototype-failedcanary-ledger-proof-report-nativeA roots/currentwholeconsumer/prior16rootpendingmetadata/allclosedauxiliarycontrols/everymemberroundtripped. FIVEoriginalpolicyS3objects includewholearchive/memberindex/inputhold/allFOURnativeA100644BXZ/wholepositive67256BXZ; allactualbinaries/source/failures/elevenlinktargets retained. HOLDallsource/raw/ownedbytes untiloriginalgenuine5remote replays/exactpublisherFF/terminal17checks; no liveowncaller captured. Native252fd8/proofsff888c/report7c153a andpartialarith b5fd45/471848/ee232d remainunsigned; full64torsion/matchedJ2next, no paidlaunch/waiver/advantage.']
for p in [d/'objects.json',d/'scope.json',g/'custody.json',g/'symlink-targets.json',g/'previous16-to-current17.json']:cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(claim+'\n');print(claim,'guard',len(current),'members',len(rows),'objects',[x['bytes']for x in objects],flush=True)
state='\n\n# Wholecensus/nondual data handoff17 ready\n\n'+claim+': actualnondual elliptic252fd8/conditionalcompositionff888c/inspectedreport7c153a andpartialarithmetic b5fd45/471848/ee232d preserved. All38wholeclosedroots/currentwholeconsumer/prior16rootpending/closedcontrols/allmemberswhole; ALL17911priorkeys retained withsixdeclaredreleasedmetadataupdates andledgerexactprefix. FIVEgenuine originalobjects/elevenlinktargets HELDuntilremote-byte replay/exactpublisherFF/terminal17guard+ownedbytes; no liveowncaller captured. AfterreleaseprojectoriginalnativeAfixture/log canonicaldocs, thenactual64torsion/matchedprimefieldJ2preflight; no source/fullJacobian/cost promotion. [pcc:unverified]\n'
for p in [r/'.resume/RESUME.md',s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md']:p.write_text(state+'\n---\n\n'+p.read_text())
f=r/'docs/session-logs/2026-11-14.md';f.write_text(f.read_text().rstrip()+'\n'+state.rstrip()+'\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
