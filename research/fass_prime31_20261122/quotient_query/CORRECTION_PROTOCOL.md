# Single corrected-source qualification: reader input binding

Frozen at 2026-11-23 05:26:04 UTC before the corrected build or any corrected native call.
This supplements, and does not replace, [PROTOCOL.md](PROTOCOL.md), SHA256
`6b7732995c5f6f2864d90ff0ae64d3f459c67d82bfef0ba9f69534d7fdf2211f`.
It uses that protocol's one retry after a diagnosed implementation defect.
All first-attempt sources, calls, certificate bytes, manifests and report
remain intact. No field, curve, target, selector, resource limit or scientific
case is selected using a new result.

## Diagnosed defect and required correction

Source review found that the persisted reader dispatched from the supplied
sidecar kind. Relabeling an S3 sidecar as synthetic could skip S3 input
binding while leaving its algebraic certificate and raw certificate hash
unchanged. The recorded first-call inputs were genuine and received their
recorded S3 checks. That observed success does not excuse this input-contract
defect. The first raw output did not separately bind sidecar bytes.

Only the live reader, producer-side metadata emission, CLI dispatch and
interface documentation change. The query kernel, tensor-norm arithmetic,
independent arithmetic, prior oracle, SHA256 source, fixture inputs and
scientific 95-case panel stay unchanged. The corrected producer records the
exact serialized sidecar SHA256 and byte count for the same 27 persisted
cases, and explicit nulls for nonpersisted sidecars.

The corrected reader has an independently written registry of exactly those
27 fixture IDs. It requires the complete expected case set and dispatches on
expected kind, not a sidecar choice. Curve field, coefficients, degree,
target abscissa and lift policy match that registry. The target ordinate is
the canonical smaller rational ordinate. Synthetic fixtures bind exact
field and F/G/H. Existing independent identities, grid and S3 binding remain.
With a raw JSONL path, sidecar SHA256 and byte count must also match; with no
raw path, the fixed fixture's mathematical input remains checked, without a
claim of producer-byte provenance.

## Fixed reader controls

Run each of the following 17 mutations both with and without raw records:

1. S3 kind changed to synthetic.
2. S3 field modulus changed.
3. Curve A changed.
4. Curve B changed.
5. Selector degree changed.
6. Target X changed.
7. Liftability policy changed.
8. Target ordinate replaced by the other valid sign.
9. Synthetic kind changed to S3.
10. Replace synthetic-zero by the otherwise valid empty-F fixture.
11. Replace synthetic-zero by the otherwise valid empty-G fixture.
12. Replace synthetic-zero by the otherwise valid unit fixture.
13. Rename one case to an unknown ID.
14. Remove an expected case.
15. Add an unknown extra case.
16. Remove an input sidecar.
17. Change the sidecar case_id.

These require 34 rejections. With raw records, additionally reject harmless
sidecar whitespace changes, an altered input_sha256, an altered input_bytes,
a missing input_sha256, and a missing input_bytes: five more rejections.
Require three positive controls: the complete unmodified panel with raw,
the complete unmodified panel without raw, and a whitespace-only sidecar
change without raw. Total: 39 rejection controls and three positive controls.
All operate on fresh in-memory copies through the same reader core. No saved
first or second producer artifact is modified. No solver or norm is invoked
by the reader-control path. The full controls invocation requires raw input.

## Source and call custody

Create a new immutable source snapshot under producer/attempt2, with the
same transitive compiled inputs and original protocol, theorem, proof map,
prediction, plus this supplement. Inventory every file using an ignore-free
method, including Cargo.lock, before building. Bind the binary before calls.
Retain stdout, stderr, exit status, pre/post hashes and all failures.

Use the original limits without increases: standalone build 120 seconds,
one job and 2 GiB virtual memory; each qualifier, reader, reader-control or
separate-role replay call 180 seconds, one thread and 2 GiB virtual memory.
The already-running root gate finishes first. Never overlap native calls.
A second corrected-build or corrected-qualification failure ends this
attempt; preserve the record instead of adding an unplanned retry.

The corrected producer reruns the exact 95 cases and persists the exact
predetermined 27 certificates. The reviewer replays the same frozen binary,
compares corrected raw and all proof/sidecar bytes, then separately runs
--check and --check-controls once each. Compare the first and corrected
scientific records after removing only the new sidecar-binding fields and
header schema additions; compare all persisted certificate/sidecar bytes.
A difference is retained and investigated, never silently normalized away.

## Interpretation and publication

This is a contract correction and finite development qualification, with
unsigned PCC standing. It adds no performance cohort, new curve fixture,
new scientific target, signed-relation result or formal proof. First-call
lockfile custody and sidecar hash omissions remain explicit. The final
report distinguishes its two source versions and includes the separate root
gate outcome. Do not infer a global Groebner basis, a fast-half-gcd runtime,
a useful signed-row count or complete signed EMPTY from these records.
