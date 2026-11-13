"""Retain unchanged original-test failures and read-only environment evidence."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';o=Path('/workspace/scratch/ct69-original-root-reap-diagnostics-01')
d=s/'evidence/original-root-reap-diagnostics-01';d.mkdir()
assert json.loads((o/'guards.json').read_text())==dict(count=2137,changed=[]) and(o/'guard.exit').read_text().strip()=='0'
labels=['ok_without_observed_reap_cannot_mark_or_read_sidecars','publication_collision_preserves_previous_bytes_and_pending_failure']
results=[]
for name in labels:
    raw=(o/(name+'.log')).read_bytes();assert(o/(name+'.exit')).read_text().strip()=='101'
    assert b'No such file or directory (os error 2)'in raw and b'0 passed; 1 failed; 0 ignored; 0 measured; 6329 filtered out'in raw
    (d/(name+'.log.xz')).write_bytes(lzma.compress(raw));shutil.copyfile(o/(name+'.exit'),d/(name+'.exit'))
    results.append(dict(test=name,original_exit=101,panic='DiagnosticSessionClock::begin().unwrap():No such file or directory(os error2)',scope='one failed/6329filtered each;publication invariants not reached'))
reads=[]
for name,cmd in [('boot-id',['cat','/proc/sys/kernel/random/boot_id']),('time-namespace',['readlink','/proc/self/ns/time']),('time-offsets',['cat','/proc/self/timens_offsets']),('kernel',['uname','-sr'])]:
    p=subprocess.run(cmd,capture_output=True);(o/(name+'.log')).write_bytes(p.stdout+p.stderr);(o/(name+'.exit')).write_text(str(p.returncode)+'\n')
    reads.append(dict(name=name,command=cmd,original_exit=p.returncode,stdout_bytes=len(p.stdout),stdout_sha256=hashlib.sha256(p.stdout).hexdigest(),stderr=p.stderr.decode()))
assert [x['original_exit']for x in reads[:3]]==[0,1,1]
(d/'environment-reads.json').write_text(json.dumps(reads,indent=2)+'\n')
source=r/'src/cryptanalysis/ecbench/runner.rs';lines=source.read_text().splitlines()
(d/'original-clock-source.txt').write_text('\n'.join(lines[203:224])+'\n')
for name in ['guards.json','plan.json','original-command.sh','original-checkpoint-push.log','original-checkpoint-push.exit','original-ledger-optout.log','original-ledger-optout.exit','KB-status.log','KB-status.exit','KB-diagnostic.log','KB-diagnostic.exit','KB-negative.log','KB-negative.exit','conductor.log','conductor.exit']:
    raw=(o/name).read_bytes()
    if name.endswith('.log'):(d/(name+'.xz')).write_bytes(lzma.compress(raw))
    else:(d/name).write_bytes(raw)
summary=dict(at=datetime.now(timezone.utc).isoformat(),results=results,all2137guards_unchanged=True,source_path=str(source.relative_to(r)),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),source_original=True,clock_required_reads=reads[:3],observed_blocker='container lacks original mandatory /proc/self/ns/time and /proc/self/timens_offsets;both tests panic at clock initialization before publication assertions',not_established='publication assertions on a capable host/global unfiltered root pass/full suite totals',next='retain original tests/clock/security checks;use a Linux host exposing required time namespace metadata for future original regression;no unchanged whole retry on this container',scope='ordinary exact unit diagnostics;not mathematical/performance/merge evidence')
(d/'observation.json').write_text(json.dumps(summary,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Original SAMEbinary exacttwo diagnostic-reap tests EACH101/0pass1fail6329filtered/no timeout/ALL2137guards0. Both panic in ORIGINALDiagnosticSessionClock::begin().unwrap NoSuchFile/os2 BEFOREpublicationassertions. Independent readonlyoriginalclockinputs:bootidread0,readlink/proc/self/ns/time1,read/proc/self/timens_offsets1;both mandatorynamespace files absent onthiscontainer. No test/source/clock/namespace/privilege/recompile change. Wholelog/backtrace/exits/environmentreads/sourceexcerpt retained;ordinaryunit originalbypass0/checkpointpush1/KBconductor127. Globalunfilteredrootretry03 remains124/unknowntotals;publicationassertions oncapablehost remainuntested. NextcapableLinuxhost originalregression;do not weakenclockchecks orrepeatunchangedwhole suite. Unsigned/no performance/globalgreen/mergewaiver.']
for p in sorted(d.iterdir()):cmd+=['--proof','attestation:'+str(p.relative_to(r))]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();print(claim,flush=True)
note='''\n## {at} — original diagnostic-reap failures located\n\n### What was done (commands, files, features)\n{claim}:SAMEoriginalbinary exacttwo failures EACH101/onefailed6329filtered/all2137guards0. BOTHpanic at originalDiagnosticSessionClock::begin().unwrap beforepublicationassertions. Originalrequiredbootidread0;readonlynamespace/time-offset reads1/missingfiles. Wholelogs/backtraces/exits/sourceexcerpt/environmentreads retained. [pcc:unverified]\n\n### Decisions made (and why)\nThiscontainer lacks mandatory/proc/self/ns/time and/proc/self/timens_offsets. Preserveoriginalclock/securitychecks/tests;futureoriginalglobalregression requirescapableLinuxhost. No unchangedglobalretry orsource/privilege/namespacechange. [pcc:unverified]\n\n### Open threads (unfinished, pick up next time)\nRootfull03 remains124/unknowntotals;publicationinvariants notexecuted bytheseexactfailures. Actualdata43525890255/13replays/e563da1b1/exact15paths/3455guards hold07released43d6ba/genuineconsumerbabda6. NextconsumerICsourcecheckpoint/730full/14CI;originalS3uploaderCIroute/costcoverage/commonoriginbridge. [pcc:unverified]\n\n### Errors (what went wrong, how it was handled)\nExact101failures preserved;originalS3checkpointpush1/KBconductor127 unchanged. No globalpass/benchmarkadmission ormergewaiver. [pcc:unverified]\n'''.format(at=datetime.now(timezone.utc).isoformat(),claim=claim)
for p in [r/'docs/session-logs/2026-11-13.md',r/'.resume/RESUME.md',r/'problems/chained-transfer-admission-and-ledger-sync.md',s/'PUBLIC_TARGET_CHECKPOINT.md']:
    with p.open('a')as f:f.write(note)
