"""Exact-byte transport repair after a rejected unpublished Git payload; I/O only."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, os, shutil, subprocess
r=Path('/workspace/crypto');c=Path('/workspace/cryptanalysis');s=r/'research/chained_jacobian_transfers_20261110';scratch=Path('/workspace/scratch')
base='4b0871aa9e7467f69956192d0008bfee3a88b297';rejected='925a2b443838306cac6edf2a8d27b71495f2a3e0';branch='research/chained-jacobian-transfer-20261110'
old=scratch/'ct81-whole-twisted-pair-and-census-handoff-19';out=scratch/'ct81-exact-byte-transport20';out.mkdir()
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==rejected
assert not subprocess.check_output(['git','status','--porcelain','-uno'],cwd=r)
inputs=json.loads((old/'current-inputs.json').read_text());links=json.loads((s/'evidence/public-n4-current-guard-19/symlink-targets.json').read_text())
def intact():
 assert all(hashlib.file_digest(Path(p).open('rb'),'sha256').hexdigest()==h for p,h in inputs.items())
 assert all(Path(p).is_symlink()and os.readlink(p)==v for p,v in links.items())
intact()
failure=json.loads((scratch/'ct81-root19-payload-publication-01/receipt.json').read_text());assert failure['original_exit']==1
raw=(scratch/'ct81-root19-payload-publication-01/push.log').read_bytes();assert b'GH001'in raw and b'100.00 MB'in raw
changed=subprocess.check_output(['git','diff','--name-only',base,rejected],cwd=r,text=True).splitlines();assert not any(str(r/p)in inputs for p in changed)
archive=old/'whole-twisted-pair-census-and-closed-captures.tar.xz';archive_rel='research/chained_jacobian_transfers_20261110/evidence/public-n4-twisted-pair-census-handoff-19/'+archive.name
assert archive.stat().st_size==150067144 and archive_rel in changed
snapshot={p:hashlib.sha256(subprocess.check_output(['git','show',rejected+':'+p],cwd=r)).hexdigest()for p in changed if p!=archive_rel}
(out/'original-unpublished-payload-files.json').write_text(json.dumps(snapshot,indent=2)+'\n')
subprocess.run(['git','branch','-m',branch+'-rejected19'],cwd=r,check=True)
subprocess.run(['git','switch','-c',branch,base],cwd=r,check=True)
for p,h in snapshot.items():
 dst=r/p;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(subprocess.check_output(['git','show',rejected+':'+p],cwd=r));assert hashlib.sha256(dst.read_bytes()).hexdigest()==h
intact()
d=s/'evidence/public-n4-twisted-pair-census-handoff-19';e=s/'evidence/public-n4-exact-byte-transport20';e.mkdir();g=s/'evidence/public-n4-current-guard-20';g.mkdir()
objects=json.loads((d/'objects.json').read_text());logical=[x for x in objects if x['path']==archive_rel];assert len(logical)==1
transport=[x for x in objects if x['path']!=archive_rel];parts=[];whole=hashlib.sha256();offset=0
with archive.open('rb')as src:
 for i in range(2):
  b=src.read(80*1024*1024);assert b;name=archive.name+f'.part-{i:04d}';p=out/name;p.write_bytes(b);whole.update(b)
  row=dict(path=(d/name).relative_to(r).as_posix(),bytes=len(b),sha256=hashlib.sha256(b).hexdigest(),local_original=str(p),kind='ordered exact unchanged archive byte segment',original_policy_requires_store=True,offset=offset)
  os.link(p,d/name);parts.append(row);transport.append(row);offset+=len(b)
 assert not src.read(1)
assert offset==logical[0]['bytes']and whole.hexdigest()==logical[0]['sha256']
(d/'.gitignore').write_text('# Original rejected archive is reconstructed from the exact ordered parts; no single-object publication is claimed.\n'+archive.name+'\n')
index=dict(at=datetime.now(timezone.utc).isoformat(),original_payload_rejected=rejected,original_payload_retained_local_branch=branch+'-rejected19',unchanged_remote_base=base,history_rewritten=False,all_original_claimed_subject_bytes_preserved=True,original_logical_archive=logical[0],parts=parts,reassembly='Concatenate the two whole parts in this exact order without decoding or modifying them; require the original whole byte count and SHA256 before extracting. Original 7359 members and their hashes stay unchanged.',original_single_S3_object_publication_accepted=False,success_requires='Original trusted artifact-autofix remote-byte replay of all seven policy-selected objects before its real bot commit; exact normal FF, all held input/link/owned-byte checks, and whole-archive ordered part reassembly hash. No handoff19 success is inferred from its failed push.')
(e/'archive-parts.json').write_text(json.dumps(index,indent=2)+'\n');(e/'objects.json').write_text(json.dumps(transport,indent=2)+'\n')
for p in ['receipt.json','push.log','expected-head-lookup.log']:shutil.copyfile(scratch/'ct81-root19-payload-publication-01'/p,e/('rejected19-'+p))
shutil.copyfile(out/'original-unpublished-payload-files.json',e/'original-unpublished-payload-files.json');shutil.copyfile(__file__,e/'original-command.py')
for n in ['01','02']:shutil.copyfile(scratch/('ct81-closed-derived-cleanup-'+n+'.json.xz'),e/('closed-derived-cleanup-'+n+'.json.xz'))
for p in [d/'objects.json',d/'scope.json',d/'README.md',s/'evidence/public-n4-current-guard-19/custody.json',s/'evidence/public-n4-current-guard-19/previous18-to-current19.json',s/'evidence/public-n4-current-guard-19/provenance-correction-01.json']:
 inputs[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
(out/'current-inputs.json').write_text(json.dumps(inputs,indent=2)+'\n');(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n');(out/'expected-migration-paths.json').write_text(json.dumps(sorted(x['path']for x in transport if x['original_policy_requires_store']),indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(guard20=len(inputs),all40437_guard19_keys_and_bytes_preserved=True,links=len(links),original19_push='actual1/GH001/unpublished/single-object criterion unmet',transport20='two exact byte parts; whole archive unchanged',hold='all scientific inputs, consumer metadata/ledger and link targets remain held until actual original remote replays, real publisher exact FF, complete byte reassembly and terminal checks'),indent=2)+'\n')
state='# Exact-byte transport20 replaces rejected unpublished payload19\n\nActual925a2b44 normalpush1/GH001: whole150067144B archive exceeds GitHub100MiB; no remote ref moved. Original failed commit retained on local research/chained-jacobian-transfer-20261110-rejected19; every original claimed subject and whole archive stays exact. Fresh candidate has SAME unchanged remoteparent4b0871aa; no published history rewrite/force/rebase. Two original-policy byte segments reconstruct the identical archive, not a repack or filtered dataset. Original19 single-object publication is UNMET; transport20 requires genuine seven-object replay BEFOREactualbot/exactFF/full ordered reassembly hash/all40437+sixsubject guards/150links/ownedbytes. All consumer source/metadata/ledger remainHELD. [pcc:unverified]\n\n## What was done\nPreserved exact rejection/receipt/localcommit and all held bytes; segmented unchanged archive and recorded complete byte offsets/SHA256. Original343science/policy/tools/workflows remain unchanged. [pcc:unverified]\n\n## Decisions made\nUse existing trusted artifact publisher on each GitHub-sized part, retain complete logical archive/member criteria and the failed original publication. No fake single-object manifest entry or hold release. [pcc:unverified]\n\n## Open threads\nNormalpublish transport20, genuine original remote replay/exactFF/whole reassembly/terminal checks, then canonical newpair fixtures/additive23CI and NEWfixedfourth-edge protocol beforecode. [pcc:unverified]\n\n## Errors\nOriginal wholedata19 preparation/governance passed; publication alone was rejectedGH001. No scientific negative or criterion acceptance is inferred. [pcc:unverified]\n\n---\n\n'
for p in [r/'.resume/RESUME.md',s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md']:p.write_text(state+p.read_text())
p=r/'docs/session-logs/2026-11-14.md';p.write_text(p.read_text().rstrip()+'\n\n'+state.split('\n---')[0].rstrip()+'\n')
p=r/'.gitattributes';p.write_text(p.read_text().rstrip()+'\n'+(e/'rejected19-push.log').relative_to(r).as_posix()+' -whitespace\n')
statement='Transport20 preparation preserves actual rejected unpublished925a2b44 normalpush1/GH001/whole150067144B archive above GitHub limit. Failed commit retained localbranch, fresh candidate from SAME unchanged remote4b0871aa; no published history rewrite. Original19 single-object publication UNMET. TWOexact originalarchive byteparts retain whole SHA/size/all7359members, original six policyselected objects become seven transportobjects with all other exactbytes retained. All40437prior heldinputs plus six immutable original19subjects/150links preserved; consumer source metadata ledger HELD until genuine original remote replays BEFOREbot/exactFF/whole reassembly/terminal checks. Unchanged original343science/tool/policy/workflows. Unsigned custody; no science/CI/performance/waiver promotion.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for p in sorted(e.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
for p in sorted(g.iterdir()):cmd+=['--proof','attestation:'+p.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();intact();print(claim,'guard',len(inputs),'parts',[x['bytes']for x in parts],flush=True)
