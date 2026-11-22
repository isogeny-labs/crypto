# Complete raw 400000-conflict N83 evidence publication

The authorized bucket holds45,357,172bytes and172payload files covering all
3timed attempts: admitted K2048-v10, failed K4096-v10 orchestration and
admitted corrective K4096-v11. Fresh S3 GET matches the complete original
archive bytes/SHA256. All172payload hashes and the frozen payload-manifest
hash pass. HEAD/GET agree on length/ETag and report AES256 encryption.
Both admitted native audits reproduce identically on freshly extracted
records using their corresponding exact downloaded guard (v10/v11).
Downloaded worker/guards/input panels compare byte-for-byte with the original
checked paths. Deterministic fresh-extraction tar/xz repack matches the
complete downloaded archive bytes. This is fresh-record replay on the
collection host with unchanged original absolute paths, not new-host timing
or portability proof.

- Bucket/region: `setec-astronomy-artifacts`, `us-east-2`.
- Logical name: `store:evidence/n83/natural-400k-20261019/runs.tar.xz`.
- Key: `store/sha256/3d/3d244ed73b4c3f853e60167e4a9bd49736681ea3fc939505ab14331fcb8fff9d`.
- Archive SHA256: `3d244ed73b4c3f853e60167e4a9bd49736681ea3fc939505ab14331fcb8fff9d`.
- Payload-manifest SHA256: `6bd2792996e598db8e6cf49b692b17c55092059bf909c3963feac32b81076a0e`.
- [Index](store/index.json), [push](push.json), [HEAD](s3-head.json), [GET](s3-get.json).
- [Fresh archive hash](download.sha256), [manifest hash](download-manifest.sha256),
  [payload manifest](payloads.sha256), [172checks](download-file-checks.log).
- [Native replay recipe](verify-fresh-native.sh), [fresh native outputs](fresh-replay/),
  [original/downloaded byte checks](fresh-path-byte-checks.txt),
  [matching repack hash](repacked.sha256), [verification status](verification-status.txt).

Both admitted runs complete400000conflicts with zero models, relations and
rank, exit0/no OOM/no wall censor/cleanup verified; director checks exact
relays/commands/output digests. K2048 solver1513432.752910ms and outer
1519074.033700ms; corrective K4096 solver1540183.375695ms and outer
1550621.509279ms. Different intervals; descriptive n=1 admitted per K/cap,
not pooled repetitions or a base-time ranking. Worker/query/inputs remain
unchanged, same strict full core14/30 and guarded1CPU/4GiB Docker envelope.
No useful natural relation, UNSAT/absent-decomposition proof, best base,
unknown-scalar recovery, full-cold winner or matched-rho ratio follows.

The initial K4096-v10 worker reported400000conflicts and zero yield, but
Docker inspect failed with empty stderr; its Docker process exit status,
final exit/OOM state, outer.json and terminal audit are missing. It remains
LAUNCH_FAILED_NO_AUDIT. The original timeout wording was too strong: timeout
is inferred from stalled RPCs and the15-second wait policy, not a retained
exit124. [Final observations](INSPECTION-OBSERVATIONS.md) correct interpretation
without rewriting the frozen [original addendum](INSPECTION-V11.md) or raw
failure. The later exact-name ps query confirms absence; [captured query](setup/)
remains separate from the immediate cleanup snapshot.

[Fresh native SAT-trace-only replay](FAILED-TRACE-REPLAY.md) validates that
failed attempt's solve_return/Unknown/400000 counter trace and matches the
original native replay bytes. COMPLETE here means SAT phase/counter trace
only, not original-group full query/custody audit or complete Docker state.
It does not admit the original failed run or fabricate missing final records.

After director assessment, the reasoning tier explicitly froze one corrective
K4096 repeat. Sourcecad91042bf/guard-v11 extends only read-only inspect wait
15→60seconds, preserves other15-second RPC deadlines, never retries commands
and records future exit status.19unit+7card tests pass. Original source
d216680177/guard-v10 passed18unit+7card. The corrective3000-second mechanical
ceiling separates bounded orchestration overhead from the unchanged2400-second
worker cap. Maximum cumulative worker allocation7200seconds covers all3attempts;
one timed K4096 repeat is documented, not silently or automatically rerun.
Its first failed command's1598.372932784seconds and worker-reported pre-summary
1549664.280075ms remain visible, outside the complete-run timing table. A
successful bounded reacquisition does not prove universal daemon reliability
or a root cause. K2048 was not repeated.

The raw archive preserves every new stream, summary/event/phase/query,
config/source/custody/resource/cleanup record, actual failure, exact specs/
cards/actual relays/receipts/assessments, original/corrected plans, host/toolchain/
build/test/readiness data, exact worker/guardsv10/v11 ELFs, public inputs,
root/standalone manifests/locks and full relevant source660f0f4b10/d216680177/
cad91042bf. Source-linked report/data/Typst/render source/SVG/150dpiPNG and
inspected3-page PDF include a separate failure/repeat diagram. Native research,
replay and derived results; Python only lays out recorded data and inspects PDF.

Prior worker/library validation is inherited: original4398pass/118ignored;
post-base-merge serial4398pass/9socket-denied/118ignored, elevated38-test
module rerun covering all9denials, plus19exporter/2model tests. Both failed
and successful raw logs are archived. No new clean single full-suite run or
new worker/library rebuild is claimed. No automatic L2 admission is made.

[Collection recipe](collect-evidence.sh), [transport/check recipe](publish-raw.sh).
Download the exact bucket/key, extract `tar -xJf runs.tar.xz`, then from
raw-bundle/ run `sha256sum -c SHA256SUMS`. Restore original absolute input/
worker paths or use a separately audited relocation; do not rewrite custody
or relabel old timings. No credentials, environment dumps, Cargo caches or
host filesystem are included. Docker image digest and actual inspect data
are retained, not its full filesystem. Older [200K](../native-natural-200k-20261019/PUBLICATION.md),
[100K](../native-natural-100k-20261019/PUBLICATION.md) and [XOR](../native-xor-indexed-20261019/PUBLICATION.md)
publications stay at their original content addresses.

[Report and inspected PDF](../../reports/run-20261019T2015Z/REPORT.md),
[native summary](SUMMARY.json), [problem and scoped exit criteria](../../../../problems/n83-native-docker-inspection-audit-gap.md).
The complete compact evidence and fresh download/replay/repack proof are
additionally archived as `store:evidence/n83/natural-400k-20261019/verification.tar.xz`.
Its own transport receipts stay in git to avoid recursive self-inclusion:
[proof push](verification-push.json), [fresh proof GET](verification-get.json),
[matching proof-download hash](verification-download.sha256).
