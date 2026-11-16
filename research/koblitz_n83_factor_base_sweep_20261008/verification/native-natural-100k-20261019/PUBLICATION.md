# Complete raw 100000-conflict N83 evidence publication

The authorized bucket holds the36,358,524-byte raw archive. Fresh S3 GET
matches the full archive bytes and SHA256; all129payload SHA256 checks pass.
The payload manifest hash matches its frozen value. HEAD/GET agree on
length/ETag and report AES256 encryption. Both original audit-bearing run
records reproduce identically using the downloaded native analysis runner
on freshly extracted raw records, with unchanged original input/worker paths.
The extracted worker and all public input panels also compare byte-for-byte
with those original checked paths. This is fresh-record replay on the
collection host, not a new-host measurement or portability claim.

- Bucket/region: `setec-astronomy-artifacts`, `us-east-2`.
- Logical name: `store:evidence/n83/natural-100k-20261019/runs.tar.xz`.
- Key: `store/sha256/95/9597d79e7a5ab37e5934268534a4f8c3a5357bac9169e8d49986119a97641c0d`.
- Archive SHA256: `9597d79e7a5ab37e5934268534a4f8c3a5357bac9169e8d49986119a97641c0d`.
- Payload-manifest SHA256: `e34e1ffa3754e63a73d20fb0095aca26755d5bdf58c5d2a41d980ae1a2b33537`.
- [Index](store/index.json), [push](push.json), [HEAD](s3-head.json), [GET](s3-get.json).
- [Downloaded archive hash](download.sha256), [manifest hash](download-manifest.sha256),
  [payload manifest](payloads.sha256), [129file checks](download-file-checks.log).
- [Native replay script](verify-fresh-native.sh), [replayed audits](fresh-replay/),
  [original/downloaded path checks](fresh-path-byte-checks.txt).
- [Deterministic fresh-extraction repack](repacked.sha256) matches the complete
  downloaded archive byte-for-byte under the retained tar/xz recipe.

The archive preserves both completed100000-conflict arms, every actual raw
stream/config/summary/source/resource/phase/query record, original/corrective
specs/cards/actual relays/assessments, both pre-solver setup failures and
unattempted plans, exact guard/analysis/worker ELF, full frozen source660f0f4b10,
Cargo manifests/locks, public inputs, host/toolchain data, inherited validation
logs, report source/data/SVG/150dpiPNG and inspected PDF. Missing original v6
receipt/raw relay remains missing; the actual mechanical failure report is
preserved and is not fabricated as solver output. v6b dirty-checkout refusal
and stopped receipt are retained. No timed solver was retried.

The immutable image digest and actual inspect/config data are retained;
a full Docker filesystem, Cargo cache, host image, credentials or runtime
environment dump is not included. The measured code/worker is frozen;
subsequent base-merge validation is separately labeled and does not change
the measured source or retroactively pool timings.

[Collection recipe](collect-evidence.sh). Retrieve with the native store
index or AWS `get-object` for the exact bucket/key, extract with
`tar -xJf runs.tar.xz`, then from `raw-bundle/` run
`sha256sum -c SHA256SUMS`. Original absolute paths remain in frozen specs;
restore them for exact historical replay or use a separately audited relocation.
Do not rewrite old custody or relabel those records as a new run.

The earlier69,465,536-byte [XOR/1000/10000-conflict publication](../native-xor-indexed-20261019/PUBLICATION.md)
and earlier49MB publication remain at their original content addresses.
This archive adds the new bounded-search evidence. Useful natural yield,
rank, full cold-runtime base selection and a matched-rho ratio remain unresolved.

[Post-base-merge validation](validation/POST-MERGE.md) is separate from the
measured worker's inherited validation: the full serial run had nine local
socket permission denials; all38affected module tests then passed with
loopback permission, including all nine denied tests. Exporter19 and model
checker2 pass. Both failed and successful logs are preserved; this does not
claim a second clean single all-tests run or change the frozen measurements.

The replay/download/repack proof files and complete failed/successful
post-merge test logs are additionally stored as
`store:evidence/n83/natural-100k-20261019/verification.tar.xz`.
[Proof push](verification-push.json), [fresh proof GET](verification-get.json)
and [matching fresh-download hash](verification-download.sha256) record the
content address and successful byte comparison. Its own transport receipts
remain in git to avoid recursive self-inclusion.
