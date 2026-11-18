"""Whole closed-capture archive and immutable input custody, I/O only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,io,json,lzma,os,shutil,subprocess,sys,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';scratch=Path('/workspace/scratch')
root_head='4b0871aa9e7467f69956192d0008bfee3a88b297';consumer='2850df0635816704f870317a09164f88a8770a2f'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==root_head
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=c)
old=json.loads((scratch/'ct77-whole-model-and-third-handoff-18/current-inputs.json').read_text());assert len(old)==24853
allowed={str(c/p)for p in ['.resume/RESUME.md','docs/session-logs/2026-11-14.md','experiments/chained-transfer-public-target/CHECKPOINT.md','experiments/chained-transfer-public-target/RESEARCH_QUEUE.md','.gitattributes','pcc/ledger.jsonl']}
changed={p for p,h in old.items()if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h};assert changed==allowed
old_ledger=subprocess.check_output(['git','show','b77e7f7ff47eed255590be777bc150b7f78226a6:pcc/ledger.jsonl'],cwd=c)
assert hashlib.sha256(old_ledger).hexdigest()==old[str(c/'pcc/ledger.jsonl')]
assert(c/'pcc/ledger.jsonl').read_bytes().startswith(old_ledger)
prior_links={p:os.readlink(p)for p in old if Path(p).is_symlink()};assert len(prior_links)==23
state='# Whole twisted-pair/census custody19 prepared; all scientific source and ledger held\n\nPublishedconsumer2850df06 contains supported separate-instance pair2a0549, conditionaloldprime215bcc andring/inverseproofs, TWOinspected61121BresultPDFpages; originalD12/D24 negatives staynegative. Fullcold04actual124/18complete andbothreuse0 isnegativef0f3ae; originalpartial/full14oldpacketdiagnosisb3a1bc0/control101 andONEinspected64187BnegativePDFc4fbde retaincomplete/incomplete/missingcases, no prices/time/gain. [pcc:unverified]\n\nNEW19 retains every declared closedct78-ct80 run/source/binary/failedbuild/control/proof/checkpoint/whole147memberCI/full6.66Mlog/firstandfinalreport capture plusallnormalpublicationreceipts, currenttracked scientificsource hashes andimmutable2850Gitbase+pending4metadata delta. ALL24853prior18keys/23priorlinks/ownedbytes andoldb77ledgerprefix preserved; onlysixreleasedmetadataupdates. Holdsource/raw/aliases/ownedbytes/consumerledger until originalremote-byte replays BEFOREbot/exactFF/terminal19release. No ownlivecaller/privateDownloadAPI captured; derivedcachesexcluded, allactualexes/preparedlibrarycopieskept. [pcc:unverified]\n\nAfterrelease canonical new9recordfixture/stdout andpartialdatasets, additive23rdpairreplay/CI thenNEWpaired fixedfourth-edge protocol beforecode. Originalcomposite may skipunsupportedJac intermediates; fixednu12 isseparateinstance andneveroriginal-logtransport. No fourth-edge code/outcome, completeJacpoint evaluator orcalibratedadvantage. Main/root/globalgeometry/program/signed/benchmark/vault/KB gaps remain; no paidlaunch/waiver/force/rechain/novelty. [pcc:unverified]\n\n---\n\n'
for p in [c/'.resume/RESUME.md',cs/'CHECKPOINT.md',cs/'RESEARCH_QUEUE.md']:p.write_text(state+p.read_text())
p=c/'docs/session-logs/2026-11-14.md';p.write_text(p.read_text().rstrip()+'\n\n'+state.split('\n---')[0].rstrip()+'\n')
names=sorted(q.name for q in scratch.iterdir() if q.is_dir() and (q.name.startswith(('ct78-','ct79-','ct80-')) or (q.name.startswith('ct73-') and 'publication' in q.name)) and not ('build-cache' in q.name or q.name.endswith('-cache')))
assert names and all((scratch/n).is_dir() for n in names)
controls=[q for q in scratch.iterdir() if q.is_file() and (q.name.startswith(('ct78-','ct78_','ct79-','ct79_','ct80-','ct80_')) or (q.name.startswith('ct73-') and 'publication' in q.name))]
assert not any(q.name.startswith('private-') for q in controls)
owned_scope=['experiments/chained-transfer-public-target','docs/research-artifacts/chained-transfer','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','problems/chained-transfer-odd-product-kernel-and-odd-factor.md','problems/README.md']
pending=sorted(set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*owned_scope],cwd=c).split(b'\0'))-{b''})
files=[(p,'whole-runs/'+name+'/'+p.relative_to(scratch/name).as_posix(),True)for name in names for p in sorted((scratch/name).rglob('*'))if p.is_file()or p.is_symlink()]
files +=[(c/n.decode(),'consumer-pending/'+n.decode(),True)for n in pending]
files +=[(p,'closed-controls/'+p.name,True)for p in controls]
files +=[(Path(__file__),'control/original-prepare-command.py',False)]
for stage,name in [('ct79-counter-storage-stage-01','CounterStorage.lean'),('ct80-twist-homogeneity-stage-01','TwistHomogeneity.lean'),('ct80-twist-factor-stage-01','TwistingFactor.lean'),('ct80-old-prime-obstruction-stage-01','OldPrimeObstruction.lean')]:
 source=r/'target'/stage/name;files.append((source,'actual-proof-stage/'+stage+'/'+name,True))
assert len({n for _,n,_ in files})==len(files)
o=scratch/'ct81-whole-twisted-pair-and-census-handoff-19';o.mkdir();d=s/'evidence/public-n4-twisted-pair-census-handoff-19';d.mkdir();g=s/'evidence/public-n4-current-guard-19';g.mkdir()
archive=o/'whole-twisted-pair-census-and-closed-captures.tar.xz';current={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in old};rows=[]
# Hold current tracked owned science/claim bytes as well as the immutable Gitbase.
owned_tracked=subprocess.check_output(['git','ls-files','-z','--',*owned_scope,'.github/workflows'],cwd=c).split(b'\0')
for name in owned_tracked:
 if name:
  source=c/name.decode();assert source.is_file() or source.is_symlink();current[str(source)]=hashlib.sha256(source.read_bytes()).hexdigest()

with tarfile.open(archive,'w:xz',dereference=False)as t:
 for p,name,hold in sorted(files,key=lambda x:x[1]):
  b=p.read_bytes();h=hashlib.sha256(b).hexdigest();row=dict(path=name,bytes=len(b),sha256=h,local_original=str(p),held_original=hold)
  if p.is_symlink():row['symlink_target']=os.readlink(p)
  if hold:current[str(p)]=h
  info=t.gettarinfo(str(p),arcname=name);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
  if p.is_symlink():t.addfile(info)
  else:t.addfile(info,io.BytesIO(b))
  rows.append(row)
 extras=[('control/root-base.txt',(root_head+'\n').encode()),('control/consumer-base.txt',(consumer+'\n').encode()),('control/consumer-pending.patch',subprocess.check_output(['git','diff',consumer,'--binary','--full-index','--',*owned_scope],cwd=c)),('control/original-prior18-consumer-ledger.jsonl',old_ledger)]
 for name,b in extras:
  info=tarfile.TarInfo(name);info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b));rows.append(dict(path=name,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()))
with tarfile.open(archive,'r:xz')as t:
 assert len(t.getmembers())==len(rows)
 for row in rows:
  member=t.getmember(row['path'])
  if 'symlink_target'in row:assert member.issym()and member.linkname==row['symlink_target']
  else:
   b=t.extractfile(member).read();assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']
current[str(archive)]=hashlib.sha256(archive.read_bytes()).hexdigest()
(o/'whole-members.json').write_text(json.dumps(dict(all_whole_closed_roots=names,all_members_no_filter=True,all_members_roundtripped=True,members=rows),indent=2)+'\n')
(o/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n');(o/'whole-current-inputs.json.xz').write_bytes(lzma.compress((o/'current-inputs.json').read_bytes()))
links={p:os.readlink(p)for p in current if Path(p).is_symlink()};assert all(links.get(p)==v for p,v in prior_links.items())
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n')
(g/'previous18-to-current19.json').write_text(json.dumps(dict(all24853_prior_keys_retained=True,only_declared_released_metadata_updates=sorted(changed),ledger_old_bytes_exact_prefix=True,old_ledger_original_sha256=old[str(c/'pcc/ledger.jsonl')],original_ledger_source='immutable2850e7f7f Gitblob; exactoriginalbytes also included inarchive',all23_prior_links_retained=prior_links,new_metadata_hashes={p:current[p]for p in sorted(changed)}),indent=2)+'\n')
objects=[]
def obj(src,name,kind):
 dst=d/name;shutil.copyfile(src,dst);objects.append(dict(path=dst.relative_to(r).as_posix(),bytes=dst.stat().st_size,sha256=hashlib.sha256(dst.read_bytes()).hexdigest(),local_original=str(src),kind=kind))
obj(archive,archive.name,'all_declared_whole_closed_roots/closedcontrols/pending4metadatafiles+immutable2850base/actualproofstages')
obj(o/'whole-members.json','whole-members.json','complete unfiltered member index')
obj(o/'whole-current-inputs.json.xz','whole-current-inputs.json.xz','complete new input hold19')
for source,name in [(scratch/'ct80-twisted-pair-native-02/expected-whole-native-01.json','fixed-twisted-pair-expected-native-02.json.xz'),(scratch/'ct80-twisted-pair-native-02/native.txt','fixed-twisted-pair-whole-native-02.txt.xz'),(scratch/'ct80-observer-partial-diagnostic-native-04/actual-partial-vectors.json','observer-partial-vectors-04.json.xz'),(scratch/'ct80-observer-partial-diagnostic-native-04/actual-partial-old-whole-parity.json','observer-partial-old-whole-parity-04.json.xz')]:
 generated=o/name;generated.write_bytes(lzma.compress(source.read_bytes()));obj(generated,name,'WHOLE ordered native records/stdout or entire explicitlypartial vector/whole14pair dataset; no masks/truncation/deduplication')
spec=importlib.util.spec_from_file_location('original_artifacts',r/'tools/artifacts/artifacts.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m);policy=m.Config.load(r).policy
for x in objects:x['original_policy_requires_store']=m.belongs_in_store(x['path'],x['bytes'],policy)
selected=[x['path']for x in objects if x['original_policy_requires_store']]
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(o/'expected-migration-paths.json').write_text(json.dumps(selected,indent=2)+'\n')
summary=dict(at=datetime.now(timezone.utc).isoformat(),root_base=root_head,consumer_base=consumer,guard19=len(current),prior_keys=24853,declared_metadata_updates=sorted(changed),ledger_old_prefix_preserved=True,whole_closed_roots=names,archive_members=len(rows),every_member_roundtripped=True,consumer_checkpoint='complete pendingmetadata delta+immutable2850e7f7fGitbase; not a duplicate trackedtree archive',pending_consumer_files=len(pending),closed_auxiliary_controls=len(controls),all_objects=objects,original_policy_store_objects=len(selected),all_link_targets=links,all23_prior_links_retained=True,hold='until original remote-byte replays/exactpublisherFF/terminalallguards-links-ownedbytes',scope='unsigned byte custody only; native6d2c97/27fb3a negatives and finiteproof scopes unchanged; strictreplay/coldcensusremedy next')
for p in [d/'scope.json',g/'custody.json']:p.write_text(json.dumps(summary,indent=2)+'\n')
shutil.copyfile(__file__,d/'original-prepare-command.py')
(d/'README.md').write_text("""# Whole fixed-twist pair and closed observer census custody19

Every declared closed source/build/native/auditor/CI/checkpoint/proof/report root and normal-publication receipt is retained with a complete unfiltered member index. All actual executables/prepared libraries, failures, interruptions and first/final report sources remain; derived build caches and private download APIs are excluded. The consumer checkpoint is immutable2850df06 plus complete pending metadata, with current tracked owned scientific/claim inputs held. [pcc:unverified]

All24853 prior18 keys,23 old links and owned bytes remain. Only six declared released consumer metadata files changed; oldb77 ledger bytes remain an exact prefix and are also inside the archive. Hold every input/alias until original remote-byte replay BEFORE publisher bot, exact FF and terminal19 owned-byte/source checks release. [pcc:unverified]

Whole nine-record native fixture/stdout and entire explicitlypartial observer vector/14-packet-pair datasets preserve2a0549/b3a1bc. Original artifact policy decides migrations; unselected objects stay exact Git blobs. This is unsigned custody, not mathematical/runtime/signed/merge/performance promotion. Original model negatives, fullcold120 compile/census failures and all root/main/benchmark/vault/KB gaps remain. No paid launch or waiver. [pcc:unverified]
""")
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','NEW19 whole custody preparation: ALL24853prior18keys/23oldlinks/ownedbytes retained, ONLYsixreleasedconsumer metadataupdates, oldb77 ledgerexactprefix+wholeoldbytes. ALLdeclaredclosedct78-ct80 scientificsource/build/native/failure/interruption/proof/report/checkpoint/full147memberCI/full6.66Mlog/publicationreceipts/everymemberroundtrip; allactualexes/preparedlibraries retained, derivedcaches/privateDownloadURLs/ownlivecaller excluded. Immutable2850Gitbase+pending4metadata delta andcurrenttracked scientific/claim source holds. Whole new9recordfixture/stdout andexplicitpartialvectors/14packetpairs follow ORIGINALstorepolicy. Holduntiloriginalremote-byte replays BEFOREbot/exactFF/terminal19 guards-links-ownedbytes. Native2a0549/newinstance, oldprime215bcc andcoldnegativef0f3ae/partialb3a1bc keepunsignedscopes; no original-logtwisttransfer, fullcensus, completeJacobianmap, paidlaunch/waiver/novelty/performancepromotion.']
for p in [d/'objects.json',d/'scope.json',d/'README.md',g/'custody.json',g/'symlink-targets.json',g/'previous18-to-current19.json']:cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(claim+'\n')
root_state='\n\n# Whole fixed-twist / observer census handoff19 ready\n\n'+claim+': ALL24853prior18inputs/23oldlinks/sixreleasedmetadataupdates/oldb77ledgerprefix; everydeclaredclosedct78-ct80run/control/source/actualexe/library/failure/CI/proof/report/checkpoint/member retained, currentownedscience held plusimmutable2850Gitbase/pendingmetadata. Originalpolicy-selectedobjects HELDuntilactualremote-byte replays BEFOREpublisherbot/exactFF/terminal19guards+ownedbytes. Newinstance2a0549/conditionaloldprime215bcc andnegativecoldf0f3ae/partialb3a1bc stayunsigned; no completeJacobianmap, original-logtwisttransfer, fullcold120, paidlaunch/waiver/novelty/advantage. Afterrelease canonicalfixtures/additive23CI andnewpairedfixedfourth-edge protocol beforecode. [pcc:unverified]\n'
for p in [r/'.resume/RESUME.md',s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md']:p.write_text(root_state+'\n---\n\n'+p.read_text())
p=r/'docs/session-logs/2026-11-14.md';p.write_text(p.read_text().rstrip()+'\n'+root_state.rstrip()+'\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
print(claim,'guard',len(current),'members',len(rows),'links',len(links),'objects',[x['bytes']for x in objects],'selected',len(selected),flush=True)
