from pathlib import Path
import json,csv,hashlib
p=Path(__file__).resolve().parent
result={'schema':'research-reconciliation/v1','task':'T-1587','historical_task':'T-1566','status':'failed_attempt_native_details_missing','recorded_utc':'2026-11-05','attempt_elapsed_seconds':142.52437313701375,'attempt_exit_code':1,'cleanup_complete_reported':True,'cleanup_native_receipt_available':False,'compiler_invoked':None,'tests_passed':None,'failure_cause':None,'new_execution':False,'canonical_source_changed':False,'canonical_source_sha256':'481f94cc389a1e79880fc168c3a98b5bb37e9411694d424632d4f8f389e5ed00','candidate_sha256':'555a5c62339a927579599d46c7e94265e52497dc56bacf0b13bb53c2dcd133c4','historical_task_status':'failed','historical_attempts':4,'historical_final_attempt_state':'stale','historical_final_failure_class':'lease_expired','missing_native_paths':['/var/tmp/pencil-root-positive-1nchhc1e/root-check-launch1/launch-receipt.json','/var/tmp/pencil-root-positive-1nchhc1e/root-check-attempt1/worker-receipt.json'],'limits':{'wall_seconds':1800,'cpu':2,'memory_bytes':17179869184,'attempts':1,'deadline':'2026-11-04T06:00:00Z'},'cost':{'planned_existing_host_overlap_usd':0.84672,'actual_usd':None,'scientific_campaign_debit':False},'source_scope':'Packet-only cfg(test) source; no accepted root test result'}
(p/'reconciliation.json').write_text(json.dumps(result,indent=2)+'\n')
with (p/'reconciliation.csv').open('w') as f:
 w=csv.writer(f);w.writerow(['attempt','status','reported_elapsed_seconds','compiler_invoked','test_count','source']);w.writerow(['T1566-root-check','failed',result['attempt_elapsed_seconds'],'unknown','unknown','root-check-execution.stdout; root-check-execution.exit'])
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="260" viewBox="0 0 900 260"><rect width="900" height="260" fill="white"/><g font-family="sans-serif" fill="#183344"><text x="24" y="32" font-size="22">T-1566: preserved validation evidence</text><rect x="24" y="65" width="250" height="100" rx="8" fill="#e7eef4"/><rect x="315" y="65" width="260" height="100" rx="8" fill="#fce5df"/><rect x="616" y="65" width="260" height="100" rx="8" fill="#eee"/><g font-size="17"><text x="38" y="95">Recipe and candidate retained</text><text x="38" y="125">1 attempt; 1,800 s cap</text><text x="330" y="95">Launcher summary: failed</text><text x="330" y="125">142.524 s; exit 1</text><text x="630" y="95">Native detail missing</text><text x="630" y="125">Test count / cause: unknown</text></g><path d="M274 115H315 M575 115H616" stroke="#183344" stroke-width="2"/><text x="24" y="205" font-size="16">One attempted validation; no comparable performance series.</text><text x="24" y="233" font-size="16">Source: saved execution stdout/exit. Cleanup is reported by the launcher summary.</text></g></svg>'''
(p/'reconciliation.svg').write_text(svg)
md='''# T-1566 root validation: preservation and reconciliation

The saved launcher output records a failed attempt lasting 142.52437313701375 seconds, exit 1, and cleanup_complete=true. This is a recorded launcher assertion; native supervisor and worker receipts are unavailable. Compilation entry, test counts, native cgroup readbacks, exact failure cause and actual dollar cost are unknown.

The named temporary workspace is absent. Current systemd readback is LoadState=not-found, inactive/dead, MainPID=0, empty InvocationID. Both ordinary and privileged journal queries returned no entries. The absent unit's Result=success does not establish historical validation success. No matching artifact manifest was found in the continuation packet tree.

The exact attempted envelope was 1,800 seconds, 2 CPUs, 16 GiB memory, zero swap, TasksMax 512, one attempt/no automatic retry, deadline 2026-11-04 06:00 UTC. A USD 0.84672 planned existing-host overlap was recorded within the prior USD 55 estimate; actual spend is unknown. The release classified this as ordinary source validation without scientific campaign debit. Earlier preparation failures remain verbatim; earlier cumulative source preparation time remains unknown.

Conductor T-1566 is failed with four stale attempts, all lease_expired, no active scope, fencing epoch 8. Final attempt ended 2026-11-03 22:54:58.571505 UTC. T-1587 is preservation/publication only, not renewed execution permission.

Canonical pencil.rs is unchanged at SHA-256 481f94cc389a1e79880fc168c3a98b5bb37e9411694d424632d4f8f389e5ed00. The positive test remains a packet-only candidate, SHA-256 555a5c62339a927579599d46c7e94265e52497dc56bacf0b13bb53c2dcd133c4. No accepted root-library validation follows from this packet.

All original packet files are preserved byte-for-byte in preserved-original-manifest.json. Oversized input inventories use the repository artifact store; .artifacts.json and artifact-readback.json record hashes, byte counts, durable URIs and verified fresh downloads. Restore with the repository tools/artifacts/artifacts.py pull command after the manifest is committed. Reconciliation JSON and CSV use null/unknown for missing evidence. The editable SVG is a provenance diagram because there is only one attempted validation, not a comparative performance series.

Next action: publish the retained evidence and candidate as unvalidated source preparation. Any future validation requires a new valid bounded execution release and coordination ownership. No new builds, tests, jobs or retries were performed here.

Sources: root-check-execution.stdout; root-check-execution.exit; root-check-freeze.json; root-check-coordinator-release.json; cache-preparation-receipt.json; cache-preparation-v2-receipt.json; source-pins.json; live Conductor task/attempt readback; live systemd and journal readback. Canonical graphs and cohort records remain unchanged.
'''
(p/'RECONCILIATION.md').write_text(md)
text='''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= T-1566 root validation reconciliation
#image("reconciliation.svg", width: 100%)
'''
for para in md.split('\n\n')[1:]:text+='\n'+para.replace('_','\\_')+'\n'
(p/'report.typ').write_text(text)
import typst
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
print('Rendered report.pdf')
