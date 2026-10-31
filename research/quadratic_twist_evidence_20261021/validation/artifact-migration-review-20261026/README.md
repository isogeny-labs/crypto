# Artifact-migration maintenance; no new scientific result

Reviewed published parent `cc585a113b20b676efe3054e19e839c3e40c8e62` against
`3c3e976e81bd6ce5872fa3677429779ac74150f8`. Maintenance follows current main
`5ba20b6d28dde4e77431f86b8c618327b0e27a2e`, its AGENTS
`09c8281346151fbfd8e62a65ed25796dcd81d440`, and
[the repository artifact policy](https://github.com/isogeny-labs/crypto/blob/5ba20b6d28dde4e77431f86b8c618327b0e27a2e/docs/artifacts.md).
No new ECDLP mechanism, mathematical claim, timing or benchmark result.

The migration moved32twist payloads into19artifact manifests. Every manifest
SHA-256 and size equals the previous Git bytes, including the native model
source and older reports/validation records. Those manifests and frozen byte
bindings remain unchanged. Physical S3 object availability is **not verified**.
Historical bytes remain available through the immutable earlier Git commit;
the metadata fixture is not a replacement S3-verification receipt.

The same commit introduced55loose files from an unrelated known-log benchmark
study. None of those paths existed at3c3e976e8; all55fail the unmodified current
artifact policy. This repair removes those additions from the current PR tree,
preserving their complete bytes at immutable parentcc585a113 and their exact
names/hashes/sizes in the archived audit ledger. It changes no benchmark row,
archive predicate or old scientific result and uploads/deletes no remote object.

Existing native/cover workflows still needed the moved input bytes. The repair
adds the standard read-only `artifacts.py pull` bootstrap to the native audit
and all three cover checkouts, with sparse manifest/tool paths and an explicit
workspace root for the cover Lean job. Parsed comparison preserves every old
verification command, condition and job. No new workflow or uniform/proof-draft
activation is added. The three metadata helper files are copied unchanged from
current main, not patched global gates:

| Helper | Exact Git blob |
| --- | --- |
| `tools/artifacts/artifacts.py` | `49eeba6a894ce4f13082941de68d44bc28ab1b86` |
| `tools/artifacts/test_artifacts.py` | `b4c135e60bbbf60dc59c3c08e1e7829d772f910d` |
| `tools/artifacts/wire_workflows.py` | `87d473f8332c23a16f44d1ed470d83d894ab48de` |

Actual ordinary validation:

- Fresh root `cargo test --release --locked --offline --lib --quiet`:
  4479passed,119existing ignores,0failures; no library/Cargo source changed.
- Twelve standard metadata-tool tests and four workflow parser tests pass.
  Tool round-trips use temporary local `file://` storage; S3 argument tests
  inspect arguments only. No real S3 request or checkpoint upload occurs.
- A separate local file-I/O fixture restores all32newly migrated payloads from
  the19exact manifests and older Git bytes, checking every hash/size. This
  tests path restoration, not curve arithmetic or remote object availability.
- Strict wiring check passes for the two affected workflows. No checker is
  relaxed. Naming and both existing draft cards pass after materializing their
  tracked sparse inputs with the documented helper.
- Final committed artifact/size/diff/source checks are recorded separately.

Before correction, the policy check fails on55files; first wiring checks fail
on custom step names; the first local fixture selected no manifests because
they were unstaged. Logs/initial workflow sources remain in the raw archive.
The fixture now stages manifests, the workflow uses the canonical step name,
and original verification steps are preserved. Sparse Markdown/card inputs
were materialized rather than misreported absent or removed from lint.

Parent-head CI has only failed review121431200155/run40280066325. Its annotation
says the job never started due account payments/spending limit; zero steps ran.
No code/proof verdict or submitted approval follows. The current public branch
rules have pull-request/non-fast-forward/deletion entries but **no
required_status_checks entry**, unlike the earlier gate/size/artifacts snapshot.
Legacy branch protection remains403, so other requirements are unknown. This
does not waive the user's named checks, all applicable CI or the keep-open hold.
Auto-merge had been re-enabled at05:23:11Z and was disabled again. Resulting-head
CI must be assessed separately; no old receipt is substituted.

All24raw metadata/ordinary-validation files land in deterministic `runs.tar.xz`
(16,512bytes). `RUNS.sha256` verifies the extracted files; repacking sorted GNU
tar with fixed date `2026-10-26 00:00:00Z`, owner/group0, numeric ownership,
mode `u+rwX,go+rX,go-w`, then `xz -6 -T1`, yields identical bytes. No payload
data, credentials, presigned token or chat transcript is in that archive.

```
mkdir FRESH_DIR
tar -xJf research/quadratic_twist_evidence_20261021/validation/artifact-migration-review-20261026/runs.tar.xz -C FRESH_DIR
(cd FRESH_DIR && sha256sum -c /absolute/repository/path/research/quadratic_twist_evidence_20261021/validation/artifact-migration-review-20261026/RUNS.sha256)
```

The raw audit ledger names each previous Git payload and manifest. To repeat
the file-I/O fixture, stage the19manifests in a fresh temporary Git repository,
place each older Git blob in a local directory at `sha256/<first2>/<digest>`,
and run the recorded `artifacts.py pull --jobs 2` against that `file://`
directory. Verify32restored paths against the ledger. Do not substitute that
local result for real-store or native scientific verification.

No new scientific/Lean run occurred: uniform96arms/896targets/210comparisons
remain planned, the recurrence draft remains unelaborated, historical
208public/269audited declarations and17identities remain unchanged. Pending
checkpoint upload approval/token, Conductor, integration, research and review
holds persist. No repeat approval question, upload, credential reuse, paid
compute, fake last-push or gate bypass. Existing research graphs/PDFs have no
new measurement and are not regenerated by this storage/CI maintenance.
