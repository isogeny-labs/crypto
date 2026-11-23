# Complete raw 200000-conflict N83 evidence publication

The user-authorized bucket holds the49,724,040-byte raw archive with184
payload files. Fresh S3 GET matches the original archive bytes and SHA256;
all184payload hashes and the frozen payload-manifest hash pass. HEAD/GET
agree on length/ETag and report AES256 encryption. Both completed run audits
reproduce identically using the downloaded native guard-v8 on freshly
extracted raw records, with unchanged original input/worker paths. Original
and downloaded worker/guard/panel bytes also match. Deterministic repacking
of the fresh extraction matches the entire downloaded archive byte-for-byte.
This is fresh-record replay on the collection host, not a new-host timing
or portability claim.

- Bucket/region: `setec-astronomy-artifacts`, `us-east-2`.
- Logical name: `store:evidence/n83/natural-200k-20261019/runs.tar.xz`.
- Key: `store/sha256/a0/a08bf2ed305580de0bb78a68cf5484ded2e8117d8eae65364b4f2dc6d72abcd0`.
- Archive SHA256: `a08bf2ed305580de0bb78a68cf5484ded2e8117d8eae65364b4f2dc6d72abcd0`.
- Payload-manifest SHA256: `50ed21f061200f7e203ffae344da5f92a25f0572fb46e58594d27137c9f4bab7`.
- [Index](store/index.json), [push](push.json), [HEAD](s3-head.json), [GET](s3-get.json).
- [Downloaded archive hash](download.sha256), [manifest hash](download-manifest.sha256),
  [payload manifest](payloads.sha256), [184payload checks](download-file-checks.log).
- [Native replay recipe](verify-fresh-native.sh), [fresh audits](fresh-replay/),
  [ELF/panel checks](fresh-path-byte-checks.txt), [repack hash](repacked.sha256),
  [verification status](verification-status.txt).

Both K2048/K4096 completed200000conflicts with zero models, relations and
rank, normal exits and verified cleanup. Source7f07562459 identifies the
measured guard-v8 and explicit core14/30 placement; the retained indexed
worker is unchanged. No timed solve was retried. Descriptive solver clocks
759947.847210/786700.645545ms and outer clocks765409.242176/797133.398274ms
are separate intervals, n=1 each, without A/A timing or best-base admission.
No useful natural relation, UNSAT/absent-decomposition proof, cold winner,
unknown-scalar recovery or matched-rho ratio follows.

Every actual attempt and output is included: v7 CPU15/31 reservation refusal
before launch, first v8 Docker RPC launch failure after quiet preflight,
original and corrected plans/specs/cards/actual raw relays/receipts/assessments,
streams, phase/query/summary/config/custody/resource records, cleanup and
later delayed-container inspection/correction, readiness check, exact worker,
guardsv7/v8 and analysisv9 ELFs, public inputs, host/toolchain/build/test logs,
full relevant sources at660f0f4b10/914b0fba02/7f07562459/4d0abec5c, root and
standalone Cargo manifests/locks, report data/source/SVG/150dpiPNG and inspected
2-page PDF. The superseded K4096v7 plan remains explicitly unexecuted.

The failed v8 guard's immediate absence snapshot was not durable: a later
observation found a delayed container in Created state, Runningfalse/Pid0/
zero StartedAt. Root retained full inspect, removed only that task's orphan
and confirmed absence before fresh same-image readiness and v8b planning.
[Setup records](setup/) preserve this correction without rewriting the original
failure-cleanup.json. No worker launch is established for either failed setup.

Analysis-onlyv9 fixes the summary draft's launch-failure mislabel and passes
18unit+7card tests. Measured guardv8 passed17unit+7card; v7 passed16unit+7card.
Prior library/worker validation is inherited. Post-base-merge full serial
4398pass/9socket-denied/118ignored was followed by an elevated38-test affected
module pass covering all9denials, plus19exporter and2model-checker tests.
Both failed and successful logs are archived; no newly clean single full-suite
run is claimed. Earlier summary drafts are archived as superseded, not used
as final failure classifications or replacement of raw evidence.

[Collection recipe](collect-evidence.sh), [upload/check recipe](publish-raw.sh).
Download the exact bucket/key, extract with `tar -xJf runs.tar.xz`, then from
raw-bundle/ run `sha256sum -c SHA256SUMS`. Original absolute paths remain in
specs; restore them or use a separately audited relocation for historical
native replay. Do not rewrite custody or relabel old timings as new runs.
No credentials, runtime environment dumps, Cargo caches or host filesystem
are included. Docker image digest/actual inspect are retained, not its full
filesystem. No automatic L2 claim is made.

Earlier [100K raw publication](../native-natural-100k-20261019/PUBLICATION.md)
and [XOR/1000/10000 publication](../native-xor-indexed-20261019/PUBLICATION.md)
remain at their original content addresses. [New report](../../reports/run-20261019T1922Z/REPORT.md)
and [native summary](SUMMARY.json) record the current result and limits.

The fresh-download/replay/repack proof and complete compact new evidence
are additionally archived as `store:evidence/n83/natural-200k-20261019/verification.tar.xz`.
Its own transport receipts remain in git, avoiding recursive self-inclusion.
See [proof push](verification-push.json), [fresh proof GET](verification-get.json)
and [matching fresh proof hash](verification-download.sha256).
