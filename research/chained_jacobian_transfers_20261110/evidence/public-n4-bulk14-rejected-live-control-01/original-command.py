"""Whole closed-capture publication and current byte hold. No research calculations."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,io,json,lzma,os,shutil,subprocess,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';scratch=Path('/workspace/scratch')
base='d2389ee6fd51517b699b3333dd173f9692cb9b3b';consumer='d5db7698edcb966a6b0b9edf330aba742b7c6dbf'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
assert not(cs/'norm-halving-evaluator').exists()
groups={
 'arithmetic-full-negative-helper-and-CI': ['ct72-arithmetic-full-01','ct72-arithmetic-full-01-layout','ct72-arithmetic-full-01-canaries','ct72-ca9-own-CI-01','ct72-helper-empty-cache-control-01','ct72-398-own-CI-02'],
 'beta-components-negatives-full-and-CI': ['ct73-beta-native-01','ct73-beta-lean-01','ct73-beta-lean-02','ct73-quadratic-picard-native-01','ct73-quadratic-picard-native-02','ct73-quadratic-source-edit-failure-01','ct73-quadratic-identity-01','ct73-quadratic-identity-02','ct73-quadratic-identity-03','ct73-quadratic-identity-04','ct73-quadratic-norm-Lean-01','ct73-quadratic-norm-Lean-02','ct73-quadratic-norm-Lean-03','ct73-source-own-CI-01','ct73-beta-producer-cwd-controls-01','ct73-beta-full-01','ct73-beta-replay-layout-01','ct73-c77-own-CI-01','ct73-beta-report-draft-01','ct73-beta-report-draft-02','ct73-beta-primary-review-01'],
 'complete-audits-corrected-CI-and-next-protocol': ['ct73-b691-CI-audit-01','ct73-b691-CI-audit-02','ct73-cc69-own-CI-01','ct73-cc69-CI-audit-01','ct73-PCC-log-handoff-01','ct74-primary-review-01','ct74-proposal-draft01','ct74-proposal-draft02']}
assert len({n for names in groups.values()for n in names})==sum(map(len,groups.values()))
assert all((scratch/n).is_dir()for names in groups.values()for n in names)
assert(scratch/'ct73-beta-full-01/driver-build-cache').is_dir()
old=json.loads((scratch/'ct73-PCC-log-handoff-02/current-inputs.json').read_text());assert len(old)==7093
assert all(Path(p).exists()and hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in old.items())
o=scratch/'ct74-bulk14-handoff-01';o.mkdir();d=s/'evidence/public-n4-closed-bulk-handoff-14';d.mkdir();g=s/'evidence/public-n4-current-guard-14';g.mkdir();at=datetime.now(timezone.utc).isoformat()
state=f'''# Closed bulk capture publication and next norm-halving implementation — {at}

Complete-C sourceb691/freshfull666cb7 andcorrectedcc69 own20distinct21automaticCI allSUCCESS/strictRust275f33 all419native+whole7113census/73canonical75actualverdicts/all15+17+20phaseguards/originalcheckpointreadbacks settled asunsigneddata. Originalb691missingPNGnegative6a5f3c remainswhole. GenuinePNG12 andblockinglog13 released afteractualremote replays/exactFF/terminalguards; rootd1f056/consumeraf2fcb pendingrootmetadata nowincluded. [pcc:unverified]

Prospective norm-halving6dc948/primary3da2fc andinspectedTWO63746BPDFpages arepublished d5db7698, BEFOREnewcode. Strongnorm-assistedelliptic-scalarbaselinefirst, commonadmission/unpricedcosts/completeallfourhalves/E2correction/all448exceptional+mixedpublic+both13controls/faithfulconditionalLean andexactalgebra required. Existinghalvingquartic Rust/Lean reused; SAMEellipticdomainpreimage bridge explicit. Newcrateabsent/no newnative orperformance result. [pcc:unverified]

NEWguard14 wholepublication: ALLselectedclosedca9full+negativeCI/helpercontrol/398CI; A-Bnative/proof/quadratic/identityfailures/sourceCInegative/cwdcontrol/freshfull INCLUDINGsealed125MBbuildcache/physicalsource/C77CI/reportdrafts; bothb691audits/currentcc69whole21logs/fourZIPs/allmembers/audit/wholefailedhandoffarchive andprospectiveprimary/reportdrafts; pendingcurrentmetadata/sourcecheckpoint andwholecurrentguardmap. Each selecteddirectory iswhole withoutfilters/pruning, eachmember hashchecked, completeindices retained. HOLD all7093priorinputs plusallselectedrawbytes/currentprotocolsubjects/actualproofsources/EIGHTlinks andbothrepoownedbytes untilgenuinepublisher remoteretrievals/exactbotFF/terminalchecks. [pcc:unverified]

Afterrelease implementnorm-halving sibling underpublished6dc948; boundedunfilteredRust/Lean correctness thenfullreport/source/freshreplay/ownCI andnextcurrentguard forfuturedata. Preserveeverynegative; no unchangedscience reruns justforstatus. Existingroot124/clock101/mainledger/reviews/signing/conflict/fullgeometry/generalclosedcoords/livebudgetcapacity/matchingimage/calibratedcost/vaultKB gatesremainopen; no paidlaunch/waiver. Closedincrementals119175955B51files onlywere removedwithfullhashreceipt/all7093guards preserved; sealedBfullcache andallexecutableskept. Continueunblockedwork. [pcc:unverified]
'''
for p in [cs/'RESEARCH_QUEUE.md',cs/'CHECKPOINT.md',c/'.resume/RESUME.md']:p.write_text(state+'\n---\n\n'+p.read_text())
with(c/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n'+state)
scope=['experiments/chained-transfer-public-target','docs/research-artifacts/chained-transfer','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','.github/workflows','problems/chained-transfer-odd-product-kernel-and-odd-factor.md']
pending=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=c).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--no-renames','--name-only','-z'],cwd=c).split(b'\0')))-{b''})
current=old.copy();all_indices=[];objects=[]
for label,names in groups.items():
 files=[(p,'runs/'+name+'/'+p.relative_to(scratch/name).as_posix())for name in names for p in sorted((scratch/name).rglob('*'))if p.is_file()or p.is_symlink()]
 controls={}
 if label=='complete-audits-corrected-CI-and-next-protocol':
  for name in pending:
   p=c/name.decode();assert p.exists()or p.is_symlink();files.append((p,'consumer-pending/'+name.decode()))
  for p in sorted((cs/'reports/norm-halving-proposal-01').iterdir()):files.append((p,'published-protocol-report/'+p.name))
  for p in sorted(scratch.glob('ct74-proposal-final03-page-*.png')):files.append((p,'final-proposal-pages/'+p.name))
  for p in sorted(scratch.glob('ct74*caller*.txt')):files.append((p,'commands/'+p.name))
  files.extend([(scratch/'ct74-closed-incremental-cleanup-01.json.xz','commands/ct74-closed-incremental-cleanup-01.json.xz'),(cs/'NORM_HALVING_PROTOCOL_01.md','published-protocol/NORM_HALVING_PROTOCOL_01.md')])
  controls={'control/base.txt':(consumer+'\n').encode(),'control/pending-full.patch':subprocess.check_output(['git','diff',consumer,'--binary','--full-index','--',*scope],cwd=c),'control/snapshot.json':(json.dumps(dict(at=at,consumer_base=consumer,root_base=base,pending_owned_paths=len(pending),all_groups=groups,new_crate_absent=True,exact_next_action='implement norm-halving under published6dc948 after hold14release'),indent=2)+'\n').encode()}
 assert len({name for _,name in files})==len(files)
 archive=o/(label+'.tar.xz');members=[]
 with tarfile.open(archive,'w:xz',dereference=False)as t:
  for p,name in sorted(files,key=lambda x:x[1]):
   b=p.read_bytes();h=hashlib.sha256(b).hexdigest();row=dict(path=name,bytes=len(b),sha256=h,local_original=str(p))
   if p.is_symlink():row['symlink_target']=os.readlink(p)
   members.append(row);current[str(p)]=h;info=t.gettarinfo(str(p),arcname=name);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
   if p.is_symlink():t.addfile(info)
   else:t.addfile(info,io.BytesIO(b))
  for name,b in sorted(controls.items()):
   info=tarfile.TarInfo(name);info.size=len(b);info.mode=0o644;t.addfile(info,io.BytesIO(b));members.append(dict(path=name,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()))
 with tarfile.open(archive,'r:xz')as t:
  assert len(t.getmembers())==len(members)
  for row in members:
   member=t.getmember(row['path'])
   if 'symlink_target'in row:assert member.issym()and member.linkname==row['symlink_target'];continue
   b=t.extractfile(member).read();assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']
 index=d/(label+'-members.json');index.write_text(json.dumps(dict(whole_roots=[str(scratch/n)for n in names],members=members,all_files_no_filter=True,all_members_roundtripped=True,symlink_targets_preserved=True,source_archive_bytes=archive.stat().st_size),indent=2)+'\n')
 all_indices.append(dict(path=index.relative_to(r).as_posix(),members=len(members),bytes=index.stat().st_size,sha256=hashlib.sha256(index.read_bytes()).hexdigest()))
 dest=d/archive.name;shutil.copyfile(archive,dest);b=dest.read_bytes();objects.append(dict(path=dest.relative_to(r).as_posix(),bytes=len(b),sha256=hashlib.sha256(b).hexdigest(),local_original=str(archive),kind='whole closed-root archive'))
 print(label,len(members),len(b),flush=True)
for suffix in ['6dc948','3da2fc','275f33']:
 p=c/('pcc/claims/PCC-20261114-'+suffix+'.json');current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
 for proof in json.loads(p.read_text())['proofs']:
  if proof['kind']=='attestation':
   q=c/proof['subject']['path'];h=proof['subject']['sha256'];assert hashlib.sha256(q.read_bytes()).hexdigest()==h;current[str(q)]=h
for p in [cs/'evidence/beta-complete-PCC-hydration-failure-01/PCC-whole-failure.log.xz',c/'docs/research-artifacts/chained-transfer/beta-complete-PCC-failure-01/PCC-whole-failure.log.xz',c/'.gitattributes',Path('/workspace/crypto/research/chained_jacobian_transfers_20261110/successor-preflight/src/two_primary_quotient.rs'),Path('/workspace/crypto/research/chained_jacobian_transfers_20261110/lean/TwoPrimaryQuotient.lean')]:current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
for directory in [scratch/'ct73-complete-driver-01/producer-staged-source',scratch/'ct73-beta-full-01/driver/producer-staged-source']:
 for p in directory.glob('*.lean'):current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
links={p:os.readlink(p)for p in current if Path(p).is_symlink()};assert len(links)>=8
assert set(json.loads((r/'research/chained_jacobian_transfers_20261110/evidence/public-n4-complete-beta-current-guard-13/symlink-targets.json').read_text())).issubset(links)
raw=(json.dumps(current,indent=2)+'\n').encode();(o/'current-inputs.json').write_bytes(raw);whole=lzma.compress(raw);(o/'whole-current-inputs.json.xz').write_bytes(whole);dest=d/'whole-current-inputs.json.xz';dest.write_bytes(whole)
objects.append(dict(path=dest.relative_to(r).as_posix(),bytes=len(whole),sha256=hashlib.sha256(whole).hexdigest(),local_original=str(o/'whole-current-inputs.json.xz'),kind='whole source+closedcapture guard map'))
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=at,guard14=len(current),all_prior7093paths_retained_byte_identical=True,all_closed_capture_member_bytes_also_held=True,all_prospective_protocol_and_actual_proof_sources=True,links=links,whole_map=dict(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()),whole_map_XZ=dict(bytes=len(whole),sha256=hashlib.sha256(whole).hexdigest()),hold='no heldsource/raw/owned byte edits until actualremote replays/exactFF/terminalchecks',sealed_B_full_cache_preserved_whole=True),indent=2)+'\n')
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(d/'indices.json').write_text(json.dumps(all_indices,indent=2)+'\n');shutil.copyfile(__file__,d/'original-command.py')
(d/'scope.json').write_text(json.dumps(dict(at=at,root_base=base,consumer_base=consumer,groups=groups,whole_roots=sum(map(len,groups.values())),three_whole_archive_groups=True,whole_indices=all_indices,objects=objects,guard14=len(current),links=len(links),remote_availability='pending original genuine publisher',scope='unsigned data custody; no native/scientific/verifier/workflow/policy edits'),indent=2)+'\n')
for row in all_indices:
 if row['bytes']>1048576:objects.append(dict(path=row['path'],bytes=row['bytes'],sha256=row['sha256'],local_original=str(r/row['path']),kind='whole large member index'))
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');(o/'expected-object-paths.json').write_text(json.dumps([x['path']for x in objects],indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','NEWguard14 whole closed-bulk handoff: THREEwholearchivegroups/all35declaredclosedroots pluscompletependingmetadata/base d5db7698/sourceprotocol andwholecurrentinputmap; everyregularmember roundtripped/hashchecked andeverylinktarget preserved; ALLprior7093paths+allclosedrawbytes+publishedprospective6dc948/actualproofsources/currentlinks held. Entireca9full/negativeCI/helper/398CI, A-Bcomponents-andfailures/sourceCInegative/cwdcontrols/freshfullINCLUDINGsealedBbuildcache/physicalsource/C77CI/reportdrafts; bothb691audits/currentcc69whole21logs/fourZIPs/allmembers/audit/wholefailedhandoffarchive/primary/proposaldrafts. Completeindices no split/prune; originald1f056publicationmetadata included. HOLDuntiloriginalgenuine remote-byte replay/exactbotFF/allguards+ownedbytes. Nextimplement6dc948 afterrelease. Unsigned, no math/execution/performance promotion orpolicywaiver.']
for p in [d/'objects.json',d/'indices.json',d/'scope.json',g/'custody.json',g/'symlink-targets.json']:cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(claim+'\n')
for p in [s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md']:p.write_text(state+'\nRoot handoff '+claim+' / guard14 '+str(len(current))+' paths / '+str(len(links))+' links; genuinepublisherpending. [pcc:unverified]\n\n---\n\n'+p.read_text())
with(r/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n'+state+'\nHandoff '+claim+'; metadata only, original343producer hashes unchanged. [pcc:unverified]\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
print(claim,flush=True);print(json.dumps(dict(guard14=len(current),links=len(links),roots=sum(map(len,groups.values())),objects=[dict(path=x['path'],bytes=x['bytes'])for x in objects],indices=all_indices)),flush=True)
