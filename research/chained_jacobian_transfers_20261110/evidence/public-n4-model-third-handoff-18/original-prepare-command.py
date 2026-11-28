"""Whole closed-capture archive and immutable input custody, I/O only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,io,json,lzma,os,shutil,subprocess,sys,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';scratch=Path('/workspace/scratch')
root_head='824d3dcd5c6de583a7bd4761de4b904877503bee';consumer='b77e7f7ff47eed255590be777bc150b7f78226a6'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==root_head
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
assert not subprocess.check_output(['git','status','--porcelain','--untracked-files=all'],cwd=c)
old=json.loads(lzma.decompress((scratch/'ct76-whole-census-and-nondual-handoff-17/whole-current-inputs.json.xz').read_bytes()));assert len(old)==24445
allowed={str(c/p)for p in ['.resume/RESUME.md','docs/session-logs/2026-11-14.md','experiments/chained-transfer-public-target/CHECKPOINT.md','experiments/chained-transfer-public-target/RESEARCH_QUEUE.md','.gitattributes','pcc/ledger.jsonl']}
changed={p for p,h in old.items()if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h};assert changed==allowed
old_ledger=subprocess.check_output(['git','show','186942f2eb96d4d26b079bd031d5cb3ae2b6bdd0:pcc/ledger.jsonl'],cwd=c)
assert hashlib.sha256(old_ledger).hexdigest()==old[str(c/'pcc/ledger.jsonl')]
assert(c/'pcc/ledger.jsonl').read_bytes().startswith(old_ledger)
prior_links={p:os.readlink(p)for p in old if Path(p).is_symlink()};assert len(prior_links)==23
state='''# Whole model/third-edge custody18 prepared; scientific source held

Consumerb77e7f7f publishes actual oldmatched modelnegative27fb3a andthird-edge6d2c97 (middle169positive, matchedHLP00D24unsupported), finite-residue68cd8c/ee88e7 andinspectedreporta1ee6b. Fullsamekernel/all64/4096/135/wholeoldparity/strict5+9orderedrecords andactualomissioncontrols retained; no newJacobianmap/retwist/reselection oradvantage. [pcc:unverified]

NEW18 includes every declared closed model/build/native/auditor/checkpoint/proof/report root and closedcontrols, complete pending4-filemetadata delta plusimmutableb77e7f7fGitbase. ALL24445 prior inputs/23links remain withONLYsixdeclaredreleasedmetadataupdates andoldledgerexactprefix. Holdallsource/raw/aliases/ownedbytes andconsumerledger until original remote-byte replays/exactpublisherFF/terminal18checks release. Nevercaptureownlivepreparationcaller. [pcc:unverified]

Afterrelease project sixwholefixtures/stdouts into original canonicaldocs and transparentlinks, addstrict22ndpreflightreplay/CI andfreshphysical/ownsource checks. Then freeze a new observer-bookkeeping protocol for incompletecoldcensus, unchangedmath/panel/bounds/wholeledgercontrols. No fourth elliptic edge orobservercodeyet; all global/main/benchmark/vault/KB constraints remain. No paidlaunch/waiver. [pcc:unverified]

---

'''
for p in [c/'.resume/RESUME.md',cs/'CHECKPOINT.md',cs/'RESEARCH_QUEUE.md']:p.write_text(state+p.read_text())
p=c/'docs/session-logs/2026-11-14.md';p.write_text(p.read_text().rstrip()+'\n\n'+state.split('\n---')[0].rstrip()+'\n')
names=['ct77-nondual-model-prototype-01','ct77-nondual-model-build-01','ct77-nondual-model-build-02','ct77-nondual-model-build-03','ct77-current-original-checkpoint-01','ct77-nondual-model-native-01','ct77-nondual-model-audit-01','ct77-nondual-model-obstruction-01','ct77-nondual-model-obstruction-02','ct77-model-negative-report-draft-01','ct77-next-edge-build-01','ct77-next-model-build-01','ct77-next-source-run-discovery-01','ct77-next-original-checkpoint-01','ct77-next-native-01','ct77-next-native-auditor-source-01','ct77-next-native-audit-01','ct77-nonsquare-proof-01','ct77-nonsquare-proof-02','ct77-next-negative-report-draft-01']
assert all((scratch/n).is_dir()for n in names)
controls=[p for p in scratch.iterdir()if p.is_file()and p.name.startswith(('ct77-','ct77_'))and not p.name.startswith(('ct77-prepare-handoff18','ct77_prepare_handoff18'))]
assert not any('private' in p.name.lower()for p in controls)
owned_scope=['experiments/chained-transfer-public-target','docs/research-artifacts/chained-transfer','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','problems/chained-transfer-odd-product-kernel-and-odd-factor.md','problems/README.md']
pending=sorted(set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*owned_scope],cwd=c).split(b'\0'))-{b''})
files=[(p,'whole-runs/'+name+'/'+p.relative_to(scratch/name).as_posix(),True)for name in names for p in sorted((scratch/name).rglob('*'))if p.is_file()or p.is_symlink()]
files +=[(c/n.decode(),'consumer-pending/'+n.decode(),True)for n in pending]
files +=[(p,'closed-controls/'+p.name,True)for p in controls]
files +=[(Path(__file__),'control/original-prepare-command.py',False)]
for name in ['01','02']:
 p=r/('target/ct77-model-square-stage-'+name+'/NonsquareResidues.lean');files.append((p,'actual-proof-stage/'+name+'/NonsquareResidues.lean',True))
assert len({n for _,n,_ in files})==len(files)
o=scratch/'ct77-whole-model-and-third-handoff-18';o.mkdir();d=s/'evidence/public-n4-model-third-handoff-18';d.mkdir();g=s/'evidence/public-n4-current-guard-18';g.mkdir()
archive=o/'whole-model-third-and-closed-captures.tar.xz';current={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in old};rows=[]
with tarfile.open(archive,'w:xz',dereference=False)as t:
 for p,name,hold in sorted(files,key=lambda x:x[1]):
  b=p.read_bytes();h=hashlib.sha256(b).hexdigest();row=dict(path=name,bytes=len(b),sha256=h,local_original=str(p),held_original=hold)
  if p.is_symlink():row['symlink_target']=os.readlink(p)
  if hold:current[str(p)]=h
  info=t.gettarinfo(str(p),arcname=name);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
  if p.is_symlink():t.addfile(info)
  else:t.addfile(info,io.BytesIO(b))
  rows.append(row)
 extras=[('control/root-base.txt',(root_head+'\n').encode()),('control/consumer-base.txt',(consumer+'\n').encode()),('control/consumer-pending.patch',subprocess.check_output(['git','diff',consumer,'--binary','--full-index','--',*owned_scope],cwd=c)),('control/original-prior17-consumer-ledger.jsonl',old_ledger)]
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
(g/'previous17-to-current18.json').write_text(json.dumps(dict(all24445_prior_keys_retained=True,only_declared_released_metadata_updates=sorted(changed),ledger_old_bytes_exact_prefix=True,old_ledger_original_sha256=old[str(c/'pcc/ledger.jsonl')],original_ledger_source='immutable186942f2 Gitblob; exactoriginalbytes also included inarchive',all23_prior_links_retained=prior_links,new_metadata_hashes={p:current[p]for p in sorted(changed)}),indent=2)+'\n')
objects=[]
def obj(src,name,kind):
 dst=d/name;shutil.copyfile(src,dst);objects.append(dict(path=dst.relative_to(r).as_posix(),bytes=dst.stat().st_size,sha256=hashlib.sha256(dst.read_bytes()).hexdigest(),local_original=str(src),kind=kind))
obj(archive,archive.name,'all20wholeclosedroots/closedcontrols/pending4metadatafiles+immutableb77base/actualproofstages')
obj(o/'whole-members.json','whole-members.json','complete unfiltered member index')
obj(o/'whole-current-inputs.json.xz','whole-current-inputs.json.xz','complete new input hold18')
for source,name in [(scratch/'ct77-nondual-model-audit-01/all-native-records.json.xz','second-model-expected-native-01.json.xz'),(scratch/'ct77-nondual-model-audit-01/whole-positive-native.txt.xz','second-model-whole-native-01.txt.xz'),(scratch/'ct77-next-native-audit-01/edge-all-native-records.json.xz','third-edge-expected-native-01.json.xz'),(scratch/'ct77-next-native-audit-01/edge-whole-positive-native.txt.xz','third-edge-whole-native-01.txt.xz'),(scratch/'ct77-next-native-audit-01/model-all-native-records.json.xz','third-model-expected-native-01.json.xz'),(scratch/'ct77-next-native-audit-01/model-whole-positive-native.txt.xz','third-model-whole-native-01.txt.xz')]:obj(source,name,'whole ordered native records or whole stdout, no masks/truncation/deduplication')
spec=importlib.util.spec_from_file_location('original_artifacts',r/'tools/artifacts/artifacts.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m);policy=m.Config.load(r).policy
for x in objects:x['original_policy_requires_store']=m.belongs_in_store(x['path'],x['bytes'],policy)
selected=[x['path']for x in objects if x['original_policy_requires_store']]
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(o/'expected-migration-paths.json').write_text(json.dumps(selected,indent=2)+'\n')
summary=dict(at=datetime.now(timezone.utc).isoformat(),root_base=root_head,consumer_base=consumer,guard18=len(current),prior_keys=24445,declared_metadata_updates=sorted(changed),ledger_old_prefix_preserved=True,whole_closed_roots=names,archive_members=len(rows),every_member_roundtripped=True,consumer_checkpoint='complete pendingmetadata delta+immutableb77e7f7fGitbase; not a duplicate trackedtree archive',pending_consumer_files=len(pending),closed_auxiliary_controls=len(controls),all_objects=objects,original_policy_store_objects=len(selected),all_link_targets=links,all23_prior_links_retained=True,hold='until original remote-byte replays/exactpublisherFF/terminalallguards-links-ownedbytes',scope='unsigned byte custody only; native6d2c97/27fb3a negatives and finiteproof scopes unchanged; strictreplay/coldcensusremedy next')
for p in [d/'scope.json',g/'custody.json']:p.write_text(json.dumps(summary,indent=2)+'\n')
shutil.copyfile(__file__,d/'original-prepare-command.py')
(d/'README.md').write_text('''# Whole matched-model and fixed-third-edge custody18

This archive retains all20 declared closed raw roots and every complete member, actual executables, source snapshots, native outputs, controls, failed attempts, checkpoint receipts and proof stages. Closed auxiliary commands and captures are included; this live preparation caller is excluded. The consumer checkpoint is the complete pending metadata delta plus immutable publishedb77e7f7f Gitbase, not a duplicate trackedtree archive. [pcc:unverified]

All24445 prior keys and every prior23 link remain. Only six declared released consumer metadata files changed, with the old ledger retained as an exact prefix and its complete original bytes in the archive. New source/raw and all current aliases are held until original remote-byte replay, exact publisher fast-forward and terminal checks release18. [pcc:unverified]

Six original full native fixture/stdout objects preserve the second-model and third-edge/model runs. The original artifact policy decides migrations; nonselected objects remain exact Git blobs. This publication does not promote any mathematical, runtime, CI, signed execution, merge or performance standing. After release add canonical fixtures and strict additive replay, then the prospectively frozen cold-census observer remedy. [pcc:unverified]
''')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','NEW18 custody preparation: ALL24445prior inputkeys and23priorlinks preserved, ONLYsixdeclaredreleasedconsumer metadataupdates/oldledgerexactprefix+wholeoldbytes inarchive. ALL20wholeclosed model/build/native/auditor/checkpoint/proof/report roots/everymemberroundtripped/actualexecutables/source/failures/closedcontrols; completependingmetadata delta+immutableb77Gitbase. Sixwhole nativefixtures/stdouts plusarchive/memberindex/inputhold use ORIGINALpolicy store selection; HOLDallsource/raw/aliases/ownedbytes until originalremote replays/exactpublisherFF/terminal18checks. No liveowncaller/privateURLs/key included. Native27fb3a/6d2c97 andproof68cd8c remainunsignednegative/finite scopes; strict22ndreplay/coldcensusremedy next, no source/CI/cost/waiver promotion.']
for p in [d/'objects.json',d/'scope.json',d/'README.md',g/'custody.json',g/'symlink-targets.json',g/'previous17-to-current18.json']:cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(claim+'\n')
root_state='\n\n# Whole model / fixed third-edge handoff18 ready\n\n'+claim+': ALL24445priorinputs/23priorlinks/sixdeclaredreleasedmetadatachanges/oldledgerexactprefix; all20wholeclosedroots/actualbinaries/failures/closedcontrols/pendingmetadata+b77Gitbase andallcomplete members retained. Ninewhole objects withoriginalpolicy-selected store paths HELDuntilgenuineremote replays/exactFF/terminal18guards+ownedbytes. No liveowncaller/privateURL/key captured. Native27fb3a/6d2c97 two supplied-source modelnegatives andfiniteproof68cd8c remainunsigned; strict22ndreplay thennewcoldcensusobserverprotocol next. No paidlaunch/waiver/advantage. [pcc:unverified]\n'
for p in [r/'.resume/RESUME.md',s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md']:p.write_text(root_state+'\n---\n\n'+p.read_text())
p=r/'docs/session-logs/2026-11-14.md';p.write_text(p.read_text().rstrip()+'\n'+root_state.rstrip()+'\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
print(claim,'guard',len(current),'members',len(rows),'links',len(links),'objects',[x['bytes']for x in objects],'selected',len(selected),flush=True)
