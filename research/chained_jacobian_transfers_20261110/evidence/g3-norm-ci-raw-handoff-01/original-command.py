"""Whole CI raw custody and publication handoff; no scientific computation."""
from pathlib import Path
from datetime import datetime,timezone
import subprocess,json,hashlib,shutil
c=Path('/workspace/cryptanalysis'); cs=c/'experiments/chained-transfer-reference'
r=Path('/workspace/crypto'); s=r/'research/chained_jacobian_transfers_20261110'
d=cs/'evidence/g3-norm-own-ci-acceptance-01';d.mkdir()
plan=json.loads(Path('/workspace/scratch/ct56-g3-norm-own-ci-raw-02/plan.json').read_text())
plan['whole_custody_claim']='PCC-20261113-8f9f11'
plan['partial_collector_custody']='PCC-20261113-d82dd3'
plan['collector_correction']='PCC-20261113-b15b45'
plan['optional_artifact_access']='PCC-20261113-963670'
(d/'acceptance.json').write_text(json.dumps(plan,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Unsigned actual own CI43400864083 exact069644bf: ALLSEVEN jobs completed SUCCESS; retain allWHOLEAPI/logs and438 source snapshots in ONE790964-byte raw02 archive8f9f11. Newjob131803082298 ALL36nativeJSON equal original12consumer+24producer;all24 ORIGINALPCC verdictsPASS/exact IDs+commits+ORIGINALcanonicalclaimdigests match. Originaldriver success marker/step establishes reported exitguard scope; standalone exit files/ZIPs NOT downloaded (optionalstorage403,963670). Initial rawSHA-vs-canonical collector assertion preserved partiald82dd3 and frozenb15b45 correction;no native/verifier edit. CI SETTLED;no repoll/redispatch. No signedexecution/fullgeometry/performance verdict.','--proof','attestation:'+str((d/'acceptance.json').relative_to(c))]
claim=subprocess.check_output(cmd,cwd=c,text=True).strip();Path('/workspace/scratch/ct56-own-ci-acceptance.claim').write_text(claim+'\n');print('CI acceptance',claim,flush=True)
h=s/'evidence/g3-norm-ci-raw-handoff-01';h.mkdir();objects=[]
for label in ['own-ci-raw-01','own-ci-raw-02']:
    source=cs/'evidence'/('g3-norm-'+label)
    meta=json.loads((source/'custody.json').read_text());name=label+'.tar.xz'
    raw=(source/'whole-raw.tar.xz').read_bytes()
    assert len(raw)==meta['archive_bytes'] and hashlib.sha256(raw).hexdigest()==meta['archive_sha256']
    (h/name).write_bytes(raw)
    objects.append(dict(root_path=str((h/name).relative_to(r)),consumer_path=str((source/'whole-raw.tar.xz').relative_to(c)),bytes=len(raw),sha256=meta['archive_sha256']))
(h/'handoff.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),objects=objects,whole_original_trees=True,partial_and_corrected_collector_both_retained=True,availability='PENDING genuine trusted-base migration and remote replay; not inferred from local copies',direct_PCC_archive_subjects=False,standing='unsigned byte custody only'),indent=2)+'\n')
shutil.copyfile(__file__,h/'original-command.py')
claim=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Whole normCI rawhandoff TWO complete unsplit trees: partialcollector134968 bytes and correctedcomplete790964 bytes, exact source custody/archive SHA matches. All7 actual SUCCESS jobs retained;no ZIPdownload inferred;no native/performance assertion from archivebytes. Genuine trusted-basepublication PENDING, along with existing67172-byte primarypublicationcollector. No newdirectlargePCCsubject.','--proof','attestation:'+str((h/'handoff.json').relative_to(r))],cwd=r,text=True).strip();print('root handoff',claim,flush=True)
