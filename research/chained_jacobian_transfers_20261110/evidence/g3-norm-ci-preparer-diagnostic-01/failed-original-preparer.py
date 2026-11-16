"""Prepare owned metadata checkpoint, preserving the original governance checks."""
from pathlib import Path
from datetime import datetime,timezone
import json
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110'
text='''# Chained transfers — canonical norm local replay and own CI settled

Independent consumer scientific069644bf/metadataea833df7, readyPR1521 under1520: FULL027b231c exact069/3d all438 guards/23outer+20inner native/tee0; own43400864083 ALL7SUCCESS/new131803082298 ALL36nativeJSON and24 exact original canonical claim digests match,168c99. Whole raw01 partialcollector134968 and raw02 complete790964 retained; raw01 collector raw-vs-canonical assertion fixed via ORIGINAL PCC function after frozenb15b45. Optional artifactZIP storage403/zeroZIPdownload963670; all7 WHOLE logs/API/source retained; no re-poll/re-dispatch. [pcc:unverified]

SIXinitialnormarchives48e581/7cd6ea, BOTHfullarchives82e079/1dd28b and primarysource69520 actual2a1f0f/5327f4 genuinelypublished. Remaining THREEwhole objects awaiting trusted-base migration: existing67172 primarypublicationcollector plus both134968/790964 CIcaptures. No recursive publicationcollector archive will be created for this final control-plane receipt; preserve ALL original APIJSON, whole log losslessXZ, FF/manifests/receipt/hashindex directly. [pcc:unverified]

Use NEW2175guard map evidence/g3-norm-pcc-current-guard-02/current-inputs.json for next artifactFF; oldmap differs only in authorizedreadonlyPCC subjectread frozen451bc2/5327f4. All343producer scientificsource hashes unchanged. Native/math/formal scope unchanged: allfixed5/7/11 classes,partialformalgeometry, no timinggain. [pcc:unverified]

Rootglobal NOTgreen: original6330fixture missing exact2293097/SHA8970667; originalpull2 credentials/anonymousHeadObject403; broadbenchrec historical189problems/106records. MainPR3877 still frozenledger/signing/conflict/reviews; routine09:45d0f30b next13:45 unlessconcrete signal. Vault8edec422locked/KBunavailable/no subagents/no paidbenchmarkadmission. [pcc:unverified]

NEXT: normalpublish this ownedmetadata; retrieve actual migration andWHOLEjob, exactbotFF withNEW2175map; genuineconsumerprojection; consumerCI checkpoint and GitHub queue. Then readychild pre-codeprotocol for existingN4/B2 publicQ API plus additiveV2 q223 field/model/basepoint bridges. Keep legacyN4 sources/V1 controls immutable, withdrawn subgroupenumeration negative result, fixed5/7/11 controls; no genericDLP fallback or new performanceclaim.
'''
(s/'G3_NORM_CI_CHECKPOINT.md').write_text(text)
(r/'.resume/RESUME.md').write_text(text+'\n---\n\n'+(r/'.resume/RESUME.md').read_text())
with (r/'docs/session-logs/2026-11-13.md').open('a') as f:f.write('\n## '+datetime.now(timezone.utc).isoformat()+' — norm own CI whole-capture publication (Codex)\n\n### What was done (commands, files, features)\n\n'+text.split('\n\n',1)[1]+'\n### Decisions made (and why)\n\nRetain initialcollector failure and correctedwholecapture; publish THREEunsplit original objects, never relaxpolicy. Use newauthorized2175guard map.\n\n### Open threads (unfinished, pick up next time)\n\nActualpublication/consumerprojection/checkpoint, then publictargetV2 protocol and native proof work.\n\n### Errors (what went wrong, how it was handled)\n\nCollector raw-vs-canonical digest assertion preserved; originalPCCcanonicalfunction comparison24passes. OptionalZIP403 preserved/zeroZIP, no bypass. [pcc:unverified]\n')
old=Path('/workspace/scratch/ct56_commit_root_publication_followup.py').read_text()
changes={"base='bf9f4640642e58dc6c3a4bae9707fecb3391741b'":"base='006db8eccae560812d12f84983432b317ca36d9a'",'g3-norm-publication-followup-governance-01':'g3-norm-ci-publication-governance-01','ct56-root-norm-publication-followup':'ct56-root-norm-ci-publication',"[('genus3-norm-full-publication-raw-01','whole-raw.tar.xz'),('g3-norm-destination-primary-handoff-01','whole-primary.tar.xz')]":"[('genus3-primary-publication-raw-01','whole-raw.tar.xz'),('g3-norm-ci-raw-handoff-01','own-ci-raw-01.tar.xz'),('g3-norm-ci-raw-handoff-01','own-ci-raw-02.tar.xz')]",'exact_two_data_objects_pending':'exact_three_data_objects_pending',"'ownbf9ledger":"'own006ledger",'ownbf9ledger':'own006ledger','EXACTTWO wholedataarchives (publicationcollector67516 andprimarysource69520)':'EXACTTHREE wholedataarchives (existingprimarypublicationcollector67172 andCIcaptures134968/790964)','both genuinepublicationpending':'allthree genuinepublicationpending','own7CIseparatepending':'own7CI168c99 ALLSEVENsuccess/wholelogsretained','research: retain norm publication and destination source review':'research: checkpoint seven-job norm CI and whole evidence handoff'}
for a,b in changes.items():
    assert a in old,a
    old=old.replace(a,b)
Path('/workspace/scratch/ct56_commit_ci_root.py').write_text(old)
