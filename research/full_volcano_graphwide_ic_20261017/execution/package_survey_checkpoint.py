"""Metadata-only packaging of a native checkpoint. Never aggregate or solve.

Usage: REPO CHECKPOINT SUMMARY REPORT GRAPH_DIR HELPERS FRESH_STAGE FRESH_ARCHIVE [DELIVERY_NOTE] [--link-sessions]
"""
import argparse
import json
import shutil
import subprocess
from pathlib import Path
from package_checkpoint import archive_tree, digest
from checkpoint_staging import stage_checkpoint

parser = argparse.ArgumentParser(description=__doc__)
for name in ('repo', 'checkpoint', 'summary', 'report', 'graphs', 'helpers', 'stage', 'archive'):
    parser.add_argument(name, type=Path)
parser.add_argument('delivery_note', type=Path, nargs='?')
parser.add_argument('--link-sessions', action='store_true',
                    help='Link only session payloads from an immutable native snapshot; same filesystem required')
args = parser.parse_args()
repo, checkpoint, summary, report, graphs, helpers, stage, archive = (
    getattr(args, name) for name in ('repo', 'checkpoint', 'summary', 'report', 'graphs', 'helpers', 'stage', 'archive'))
delivery_note = args.delivery_note
assert not stage.exists() and not archive.exists(), "fresh output required"
inv=json.loads((checkpoint/'inventory.json').read_text())
doc=json.loads((summary/'summary.json').read_text())
manifest=json.loads((checkpoint/'campaign/campaign.json').read_text())
assert inv['ok'] and inv['outcome_totals']==doc['outcome_totals']
assert manifest['ecbench_sha256']==doc['binary_sha256']
assert not inv['states'].get('running',0)
staging = stage_checkpoint(checkpoint, stage, link_sessions=args.link_sessions)
shutil.copytree(summary,stage/'summary')
shutil.copytree(report,stage/'report')
if delivery_note:
    shutil.copy2(delivery_note,stage/'DELIVERY_NOTE.md')
expected={}
with (stage/'campaign/queue.jsonl').open() as f:
    for line in f:
        r=json.loads(line);expected[r['graph']]=r['input']['graph_sha256']
(stage/'graphs').mkdir()
for name,sha in expected.items():
    source=graphs/f'{name}.sqlite';assert digest(source)==sha
    shutil.copy2(source,stage/'graphs'/source.name)
(stage/'source').mkdir()
for path,name in [('src/cryptanalysis/ecbench/runner.rs','runner.rs'),
                  ('src/cryptanalysis/ecbench/methods.rs','methods.rs'),
                  ('src/cryptanalysis/ic_framework/plugins.rs','plugins.rs'),('Cargo.lock','Cargo.lock')]:
    raw=subprocess.check_output(['git','show',f'{manifest["source_head"]}:{path}'],cwd=repo)
    (stage/'source'/name).write_bytes(raw)
execution=repo/'research/full_volcano_graphwide_ic_20261017/execution'
for name in ['summarize.rs','checkpoint.rs','render_survey_checkpoint.py',
             'package_survey_checkpoint.py','package_checkpoint.py','checkpoint_staging.py',
             'test_checkpoint_staging.py','WATCHDOG_REPAIR_PROTOCOL.md']:
    shutil.copy2(execution/name,stage/'source'/name)
for name in ['volcano-ic-checkpoint','volcano-ic-summarize-v2']:
    shutil.copy2(helpers/name,stage/'bin'/name)
(stage/'README.txt').write_text(f'''Repaired release survey: stopped, integrity-checked checkpoint.
{inv['vertices_audited']} audited of {inv['vertices_planned']} planned vertices;
{inv['strata_audited']} of {inv['strata_planned']} strata sampled.
Accepted attempts {doc['measured_records']}; outcomes {json.dumps(doc['outcome_totals'],sort_keys=True)}.
Ledger states {json.dumps(inv['states'],sort_keys=True)}.

Earlier interrupted attempts are retained, marked by the unchanged native
driver's zero-job stops and excluded from accepted means. Their termination
causes were not observed. DELIVERY_NOTE.md records this checkpoint's actual
stop or recovery. No sealed session or mathematical budget was modified.

inventory.json binds record seals, receipt/file hashes, source/queue identity
and exact existing successful/exhausted replay populations. This is not a
fresh or independent solver replay. Native summary/CSV retain min/max/mean,
all phases and actual n. Timeouts have unknown final counters, not zero.
GAE with unpriced work is a lower bound. Exhaustion is not a successful DLP.

This binary is distinct from historical dev99 and stopped release70 builds;
never pool repeated workloads or cross-binary measurements. Every capsule
retains host metadata. Future resumed runtime epochs must remain identifiable;
wall observations are unqualified and no speed comparison is admitted.

The exact queue, binaries and source are in campaign/. The native graph files
are complete only for their declared fields and finite degree sets, with
24823 vertices / 51 strata / 69300 directed maps. Original A--E full graphs,
full benchmark matrix, exact representation registration, transported targets,
full graph-wide composite-PH comparison, independent arithmetic replay and
large-extension finite-field IC remain open. Original C's separate retained-
prefix full-selected-point-order PH/k=1 diagnostic does not discharge them.

Linux x86-64 inspection: bin/volcano-ic-driver status campaign
Native summary: bin/volcano-ic-summarize-v2 /fresh/summary campaign
Explicit resume only: bin/volcano-ic-driver run campaign 24823 MAX_SECONDS MAX_SPOOL_BYTES
Choose the outer time/storage guards explicitly; see DELIVERY_NOTE.md when
included. These are execution guards, not changes to child mathematical budgets.
The snapshot has no STOP marker; never run resume merely to inspect it.
Interrupted work retries into a new numbered session; it never edits the old.
Full source revision {manifest['source_head']}; exact binaries supplied.
Byte-reproducible compilation is not asserted. Source snapshots and compiler
identity do not replace the native receipt/record verification.
''')
lines=[f'{digest(p)}  {p.relative_to(stage).as_posix()}' for p in sorted(stage.rglob('*')) if p.is_file()]
(stage/'MANIFEST.sha256').write_text('\n'.join(lines)+'\n')
archive_tree(stage,archive)
print(json.dumps({'path':str(archive),'bytes':archive.stat().st_size,'sha256':digest(archive),'staging':staging}))
