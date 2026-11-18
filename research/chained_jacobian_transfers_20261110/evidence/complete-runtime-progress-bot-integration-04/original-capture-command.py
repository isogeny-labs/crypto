from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');p=r/'research/chained_jacobian_transfers_20261110'
g=p/'evidence/complete-runtime-progress-bot-integration-04';g.mkdir()
shutil.copyfile(__file__,g/'original-capture-command.py')
hold=Path('/dev/shm/ct280-root-genuine-bot-current-full-guard')
for name in ['entire-current-hold.json','entire-published-source-ledger.jsonl']:
    (g/(name+'.xz')).write_bytes(lzma.compress((hold/name).read_bytes()))
for name in ['hold-closed-successfully.json','actual-normal-fetch.txt','actual-direct-parent-FF.txt','actual-direct-parent-FF.exit','original-immediate-ledger.txt','original-immediate-ledger.exit','original-immediate-artifact-policy.txt','original-immediate-artifact-policy.exit']:
    shutil.copyfile(hold/name,g/name)
shutil.copyfile('/workspace/scratch/ct280_guarded_genuine_root_runtime_metadata_FF.py',g/'original-guarded-FF-command.py')
checks=json.loads(Path('/dev/shm/ct289-root16929-entire-current-checks.json').read_text());actual=checks['check_runs']
assert len(actual)==8 and sum(x['conclusion']=='success' for x in actual)==7 and sum(x['conclusion']=='skipped' for x in actual)==1
shutil.copyfile('/dev/shm/ct289-root16929-entire-current-checks.json',g/'entire-current-checks.json')
ci=Path('/dev/shm/ct290-root16929-entire-current-PCC-CI')
for name in ['entire-original-job-log.txt','entire-original-member-inventory.json']:
    (g/(name+'.xz')).write_bytes(lzma.compress((ci/name).read_bytes()))
receipt=dict(at=datetime.now(timezone.utc).isoformat(),source='16929c6c52f2f8b1510cac524e227ea7449ba0c2',
    original_current_PCC_run=43824933434,original_current_PCC_job=133156396623,original_current_PCC_artifact=14205569904,
    entire_original_PCC_artifact_bytes=195540,entire_original_PCC_artifact_sha256='f9253512dd8b999d0bf52bd69f95abba281235035ffaa94c9a459b9a4634203b',whole_members=10,
    complete_CI_archive_and_all_members=str(ci),all_started_checks_success=7,main_only_signing_skipped=True,
    actual_direct_parent_FF=0,whole_current_guarded_keys=8634,all952prior_unchanged=True,
    exact_bot_paths=4,whole_remote_replays_before_bot=2,whole_current_hold=str(hold),
    scope='original current CI and metadata-only genuine source integration; no new mathematical or signed execution standing')
(g/'actual-complete-current-guard-and-CI.json').write_text(json.dumps(receipt,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Root runtime-progress genuine16929 directly follows2d548 after2 whole remote replays ad46e6, exact4 metadata/removal paths. Actual guarded direct-parent FF0 preserves all8634 current/all952 prior whole keys and all current unpublished norm source/claims/queue/ledger/report bytes; both whole raw objects restored only under genuine ignores, actual original prefix/artifact0. Complete before/actual indices and whole old/new bytes retained, older immutable hold used only as hardlink source for byte-identical files. Current16929 all7 started checks succeed, main-only sign skipped; reviews/threads empty. Whole195540-byte10-member current PCC artifact14205569904/SHAf9253512dd8b999d0bf52bd69f95abba281235035ffaa94c9a459b9a4634203b and entire job log retained, all member CRC/SHA/extraction complete; original canaries/prefix/all-claim-current-workflow restoration/tests pass. Whole CI archive remains in RAM/GitHub, not falsely in Git; later all-raw custody remains. This closes the scoped two-object pre-bot prerequisite and current merge conditions without force/rechain/admin bypass. Actual normal PR4411 merge remains the next action; new cubic norm source/CI/report publication remains separate.']
for f in sorted(g.rglob('*')):
    if f.is_file():cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,text=True).strip();print(claim)
text=f'''\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Close current runtime metadata store/CI integration\n\n## What was done\n{claim}: actual2d548→16929 guarded FF0, all8634 current/all952 prior keys/whole raw/current unpublished norm source and ledger bytes unchanged; two remote replays precede genuine four-path bot. Current all7 started checks success/sign skipped, whole current PCC artifact/all10 members/full job log retained. [pcc:unverified]\n\n## Decisions made\nNormal expected-head merge4411 after actual empty reviews/threads and applicable current checks. Then use a fresh owned branch from its genuine merged tree for the completed12-statement cubic-norm component, dual-gate CI and inspected report. Add exact source hydrations for this full guard's two whole XZ objects before genuine migration, without changing verifier/policy. [pcc:unverified]\n\n## Open threads\nConsumer correction2c44ee2e is normally published with b72688; actual previous040 source-closure failure97e351 stays whole. Next changed-source CI observation around09:15UTC. Full LP scientific population and paid admission remain blocked by actual exhausted budgets; OCI/ABI/global Jacobian/program and genuine all-raw scientific custody remain open. [pcc:unverified]\n\n## Errors\nRoot pre-migration policy gap is now closed at16929; no source/ledger/raw loss. Consumer failure is omitted unchanged protocol in sparse checkout, corrected without altering frozen bytes. Version01 norm report and workflow bytes are directly pinned by b203ad and must remain immutable; future work uses additive versions. [pcc:unverified]\n'''
with (r/'docs/session-logs/2026-11-16.md').open('a') as f:f.write(text)
with (r/'.resume/RESUME.md').open('a') as f:f.write(text)
