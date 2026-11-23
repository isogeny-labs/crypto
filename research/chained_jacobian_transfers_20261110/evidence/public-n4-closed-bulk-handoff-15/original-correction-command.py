"""Preserve rejected hold14 and replace its live auxiliary log with a closed log."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,io,json,lzma,os,shutil,subprocess,sys,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';scratch=Path('/workspace/scratch');oldout=scratch/'ct74-bulk14-handoff-01';olddata=s/'evidence/public-n4-closed-bulk-handoff-14'
before=json.loads((oldout/'current-inputs.json').read_text());caller=scratch/'ct74-bulk14-prepare-caller-01.txt'
changes=[p for p,h in before.items()if hashlib.sha256(Path(p).read_bytes()).hexdigest()!=h];assert changes==[str(caller)],changes
e=s/'evidence/public-n4-bulk14-rejected-live-control-01';e.mkdir();shutil.copyfile(caller,e/'whole-terminal-caller.txt');shutil.copyfile(scratch/'ct74_prepare_bulk14.py',e/'original-command.py')
indices=json.loads((olddata/'indices.json').read_text());partial=[]
for index in indices:
 rows=json.loads((r/index['path']).read_text())['members']
 for row in rows:
  if row.get('local_original')==str(caller):
   with tarfile.open(oldout/'complete-audits-corrected-CI-and-next-protocol.tar.xz','r:xz')as t:body=t.extractfile(row['path']).read()
   assert caller.read_bytes().startswith(body)and hashlib.sha256(body).hexdigest()==row['sha256'];(e/'captured-live-prefix.txt').write_bytes(body);partial.append(row)
assert len(partial)==1
(e/'rejection.json').write_text(json.dumps(dict(rejected_hold=14,original_claim='PCC-20261114-ac449a',caller_exit=1,guard_count=len(before),sole_changed_path=str(caller),all7093_scientific_paths_and_all35_closed_roots_unchanged=True,cause='auxiliary current preparation caller log captured while still being written; final guard correctly rejected',captured_prefix=partial[0],all_three_whole_archives_and_indices_retained_unchanged=True,no_publisher_or_native_failure=True,successor='NEWguard15 pins the whole nowclosed terminalcaller, all prior keys and currentmetadata; original archives remain immutable negative-and-positive custody'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Bulk hold14/ac449a rejected1 BEFOREpublication: sole changed key among17103 was auxiliary live preparationstdout captured in its ownsnapshot. ALL7093sciencepaths/ALL35closedroots remainbyteidentical; originalthreewholearchives+indices preserved, capturedprefix andwholeterminalcaller retained. No native/publishernegative inferred. NEWguard15 willpinwholeclosedcaller andcurrentmetadata, retainallpriorkeys, andpublishoriginalimmutablearchives withoutrepacking/pruning.']
for p in sorted(e.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
reject=subprocess.check_output(cmd,cwd=r,text=True).strip()
state='''# Fresh hold15 corrects rejected auxiliary-log hold14\n\nBulk14/ac449a preparation1 rejected beforepublication because its own livecaller grew; all35closedroots/all7093sciencepaths andallthreewholearchives/indices areintact. Wholecapturedprefix+terminalcaller andexplicitrejection retained. NEWguard15 retainsall17103keys withtheclosedcallerhash andcurrentcheckpointmetadata, allpublished6dc948inputs/currentlinktypes, andallownedbytes. Original35rootarchives remainimmutable, includingthepartialauxiliaryprefix asnegative; wholeterminalcontrol isseparate. No repeatedscience/no data omitted orpruned. [pcc:unverified]\n\nComplete-C cc69own20workflows/21automaticCI/strict275f33all419/7113census/73canonical75verdicts/fresh666cb7 settledunsigned. Normhalving6dc948/protocolsource d5db7698 ispublished BEFOREnewcode; newcrateabsent. Originald1f056 publicationmetadata andallolderca9/398/A-B/C77/Bfullsealedcache/currentCI/proposalrawcaptures areincludedinthisoriginalbulkbatch. HOLDuntilgenuinepublisherreplays/exactFF/terminal15checks; thenimplement strongestbaseline andnormhalving perprotocol. Root124clock101/mainledger/signing/reviews/conflict/actualgeometry/cost/admission/vaultKB remainopen; no paidlaunch orwaiver. [pcc:unverified]\n'''
for p in [cs/'RESEARCH_QUEUE.md',cs/'CHECKPOINT.md',c/'.resume/RESUME.md']:p.write_text(state+'\n---\n\n'+p.read_text())
with(c/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n'+state)
o=scratch/'ct74-bulk15-handoff-01';o.mkdir();d=s/'evidence/public-n4-closed-bulk-handoff-15';d.mkdir();g=s/'evidence/public-n4-current-guard-15';g.mkdir()
scopes=['experiments/chained-transfer-public-target','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','docs/research-artifacts/chained-transfer','.github/workflows','problems/chained-transfer-odd-product-kernel-and-odd-factor.md']
names=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scopes],cwd=c).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=c).split(b'\0')))-{b''})
archive=o/'current-metadata-and-whole-terminal-control.tar.xz';members=[]
with tarfile.open(archive,'w:xz')as t:
 for p,name in [(c/n.decode(),'consumer/'+n.decode())for n in names]+[(p,'rejected14-control/'+p.name)for p in sorted(e.iterdir())]:
  b=p.read_bytes();info=tarfile.TarInfo(name);info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b));members.append(dict(path=name,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()))
with tarfile.open(archive,'r:xz')as t:
 for row in members:
  b=t.extractfile(row['path']).read();assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']
(d/'current-metadata-members.json').write_text(json.dumps(members,indent=2)+'\n')
current={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in before};changed=[dict(path=p,previous=before[p],current=h,reason='terminalauxiliarylog'if p==str(caller)else'checkpointmetadata after rejectedhold14; scientificbytesunchanged')for p,h in current.items()if h!=before[p]]
allowed={str(caller),str(cs/'RESEARCH_QUEUE.md'),str(cs/'CHECKPOINT.md'),str(c/'.resume/RESUME.md'),str(c/'docs/session-logs/2026-11-14.md')};assert {x['path']for x in changed}.issubset(allowed),changed
assert all(current[p]==h for p,h in json.loads((scratch/'ct73-PCC-log-handoff-02/current-inputs.json').read_text()).items())
raw=(json.dumps(current,indent=2)+'\n').encode();(o/'current-inputs.json').write_bytes(raw);compressed=lzma.compress(raw);(o/'whole-current-inputs.json.xz').write_bytes(compressed)
links={p:os.readlink(p)for p in current if Path(p).is_symlink()}
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n');(g/'rejected14-to-current15.json').write_text(json.dumps(changed,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),guard15=len(current),all17103keys_retained=True,all7093prior_scientific_paths_and_all35closed_roots_byte_identical=True,only_postrejection_changes=changed,all_link_types_targets=links,whole_terminalcaller_retained_separately=True,original_three_archives_and_indices_not_repacked=True,whole_map=dict(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()),hold='no heldsource/raw/owned edits until originalreplays/exactFF/terminal15checks'),indent=2)+'\n')
objects=json.loads((olddata/'objects.json').read_text())
for row in objects:
 p=r/row['path'];assert hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256'];dest=o/('original-'+p.name);shutil.copyfile(p,dest);row['local_original']=str(dest)
for src in [archive,o/'whole-current-inputs.json.xz']:
 dest=d/src.name;shutil.copyfile(src,dest);body=dest.read_bytes();objects.append(dict(path=dest.relative_to(r).as_posix(),bytes=len(body),sha256=hashlib.sha256(body).hexdigest(),local_original=str(src),kind='whole currentmetadata+terminalcontrol'if src==archive else'fresh complete guard15 map'))
spec=importlib.util.spec_from_file_location('original_artifacts',r/'tools/artifacts/artifacts.py');mod=importlib.util.module_from_spec(spec);sys.modules[spec.name]=mod;spec.loader.exec_module(mod);policy=json.loads((r/'.artifacts/config.json').read_text())['policy']
for row in objects:row['original_policy_requires_store']=mod.belongs_in_store(row['path'],row['bytes'],policy)
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(d/'scope.json').write_text(json.dumps(dict(root_base='d2389ee6fd51517b699b3333dd173f9692cb9b3b',consumer_base='d5db7698edcb966a6b0b9edf330aba742b7c6dbf',rejection_claim=reject,original_bulk_claim='PCC-20261114-ac449a',objects=objects,guard15=len(current),links=len(links),all35whole_roots_in_original_three_archives=True,current_metadata_members=len(members),actual_remote_replay_required=sum(x['original_policy_requires_store']for x in objects),no_repack_or_native_changes=True,scope='unsigned whole custody; bothrejected14 andfresh15 retained'),indent=2)+'\n');shutil.copyfile(__file__,d/'original-command.py')
(o/'expected-migration-paths.json').write_text(json.dumps([x['path']for x in objects if x['original_policy_requires_store']],indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','FreshNEWguard15 after explicitrejected14 liveauxiliarylog: ALL17103prior keys retained, ALL7093sciencepaths/ALL35closedroots unchanged; onlyclosedterminalcaller andpostrejectioncheckpointmetadata hashesupdated. OriginalTHREEwholebulkarchives andcompleteindices unchanged/notrepacked; wholeterminalcaller+currentmetadata separatelyroundtripped; wholefreshguardmap. Eightphysicalobjects classified by originalunchangedartifactpolicy. Alllinktypes/currentprospective6dc948/actualproofsources andownedbytes HELDuntilgenuine remote-byte replays/exactbotFF/terminal15checks. Rejectedac449a/partialauxiliaryprefix preserved, no failurewaiver/pruning/nativechanges. Nextimplementpublishednormhalving afterrelease; unsignedcustodyonly.']
for p in [d/'objects.json',d/'scope.json',d/'current-metadata-members.json',g/'custody.json',g/'symlink-targets.json',g/'rejected14-to-current15.json']:cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(claim+'\n')
for p in [s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md']:p.write_text(state+'\nNewhandoff '+claim+' / guard15 '+str(len(current))+' / all'+str(len(links))+' links; holdactive. [pcc:unverified]\n\n---\n\n'+p.read_text())
with(r/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n'+state+'\nNewhandoff '+claim+'; original343producer inputsunchanged. [pcc:unverified]\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
print(reject,claim,flush=True);print(json.dumps(dict(guard15=len(current),links=len(links),objects=len(objects),remote_required=sum(x['original_policy_requires_store']for x in objects),metadata_archive_bytes=archive.stat().st_size)),flush=True)
