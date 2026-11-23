"""Prospective one-payload original-policy transport; preserve all failed gates."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, shutil, subprocess

repo=Path('/workspace/crypto');study=repo/'research/chained_jacobian_transfers_20261110'
base='1487e3a55efaacf899ab2553af7ff8fbccae22f6'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()==base
failed=study/'evidence/public-n4-concurrent-main-merge-ledger-divergence-governance-01'
assert(failed/'artifact.exit').read_text().strip()=='1'
archive=study/'evidence/public-n4-concurrent-main-merge-ledger-divergence-01/entire-original-publication-and-remote-divergence-evidence.tar.xz'
assert archive.stat().st_size==726536 and hashlib.sha256(archive.read_bytes()).hexdigest()=='a7072da26d05a0e27ba2f43f26584887979484e1faee0e2e08594fb108e73fe1'
command=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Preserve original metadata-governance artifact rejection: all343science/ledger/prose/workingcachedwhitespace/wiring0, original unchanged artifact check1 selects the entire726536B ledger-divergence archive pinned99baab, SHAa7072da26d05a0e27ba2f43f26584887979484e1faee0e2e08594fb108e73fe1. No finalgovernancecommit or sourcepublication occurred in this attempt. Complete failedcommand/actualoutputs/stagedfilenames retained; payloadnottrimmed-renamed-split and no policyexception. Follow originaltrusted artifact migration/replay-before-bot/exactnormalFF/terminalguards on preserved append-only history; fullscientificNEW28 separate. Unsigned failuredataonly.']
for path in sorted(failed.iterdir()):command+=['--proof','attestation:'+path.relative_to(repo).as_posix()]
claim=subprocess.check_output(command,cwd=repo,text=True).strip()
control=Path('/workspace/scratch/ct121-ledger-divergence-artifact-custody-01');control.mkdir()
shutil.copyfile(__file__,control/'original-preparer.py')
shutil.copyfile(archive,control/'entire-original-payload.tar.xz')
objects=[dict(path=archive.relative_to(repo).as_posix(),bytes=archive.stat().st_size,sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),local_original=str(control/'entire-original-payload.tar.xz'))]
(control/'objects.json').write_text(json.dumps(objects,indent=2)+'\n')
intent=archive.parent/'METADATA_TRANSPORT_01.md'
intent.write_text('''# Prospective complete ledger-divergence metadata transport

The unchanged original artifact policy selects the complete726536-byte archive (SHA256 a7072da26d05a0e27ba2f43f26584887979484e1faee0e2e08594fb108e73fe1) pinned99baab. Keep every byte and all34members. Original artifact-governance failure is preserved; no payload is renamed, trimmed or split, and no policy exception is requested. This is a metadata sidecar, separate from pending complete scientific custodyNEW28. [pcc:unverified]

Publish a normal source commit on research/chained-transfer-preserved-20261115-ct120. Its PR targets a new immutable snapshot branch at2d46258a, the last normally published unmodified ledger prefix, so the trusted artifact tool/policy remain exactly original. The existing shared223 main merge and its ledger-prefix rejection remain untouched and explicitly blocked in3877. This continuation does not resolve that conflict or authorize a rechain. [pcc:unverified]

HOLD the entire regular archive and its full external byte copy, all343 protected producer inputs, all original tool/script/workflow/policy bytes, source HEAD/status/new subjects and full source ledger until the actual original upload and replay finish before the actual bot commit. Require a direct-parent bot with exactly the archive removal plus its parent .artifacts.json and .gitignore, exact canonical original entry, a normal fast-forward, full original payload bytes restored only at its genuinely ignored path, complete ledger/source/subject guards and original artifact/wiring checks. Partial/failed outcomes remain complete. Consumer work is independent; this sidecar does not project a Consumer subject or assert whole scientific custody. [pcc:unverified]
''')
entry=f'''# Preserve original artifact-policy failure; route complete metadata archive through original store

## What was done
{claim}: Rootdivergence metadata governance passed ledger/prose/workingcachedwhitespace/wiring/all343science, then originalartifact1 selected the full726536-byte34member archive pinned99baab. Full failure and stagedfilename record remain. No commit/push occurred in that failed governor; local1487 stays normallypublished on preservedbranch. [pcc:unverified]

## Decisions made
Use the original trusted artifact migration with this one entire archive, no trim/rename/split/exception. Freeze METADATA_TRANSPORT_01.md before sourcepublication: stablebase snapshot2d46258a, preservedbranch source, all343 science and originaltools/workflow/policy/archive/newsubject/sourceledger holds through actual replays-before-bot/exact3pathnormalFF/terminalguards. This metadata sidecar has no Consumer projection and is separate from NEW28. [pcc:unverified]

## Open threads
Source transportgovernance/publication/readycontinuationPR, actual artifactmigration/terminalclosure; original3877 frozenledger conflict; Consumer1573freshCI and primary-reviewed unassisted completion/fullscientificNEW28. [pcc:unverified]

## Errors
Original artifact1 is retained and handled by the original store, not relabeled as0. Remote223 fails original2d-prefix check; no historyrepair or forcepush. All prior failures remain. [pcc:unverified]
'''
for path in [repo/'docs/session-logs/2026-11-15.md',study/'QUEUE.md',study/'PUBLIC_TARGET_CHECKPOINT.md']:
    with path.open('a')as out:out.write('\n'+datetime.now(timezone.utc).isoformat()+'\n\n'+entry+'\n---\n')
resume=repo/'.resume/RESUME.md';resume.write_text(entry+'\n---\n\n'+resume.read_text())
print(claim,flush=True)
