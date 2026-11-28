# Corrected-reader qualification: independent readout

Date: 2026-11-23 UTC. The corrected qualifier replay, saved-proof reader and
reader-control call each completed once with exit status zero. All required
39 rejection controls and three positive controls passed. This is
separate-role, same-host, unsigned finite development evidence using the
same immutable binary as the corrected producer. It is not a signed PCC
verdict, a second-host replication, a formal proof or a performance study.
[pcc:unverified]

## Protocol and exact source version

The calls used the original [protocol](../../PROTOCOL.md), SHA256
`6b7732995c5f6f2864d90ff0ae64d3f459c67d82bfef0ba9f69534d7fdf2211f`,
and its frozen [correction supplement](../../CORRECTION_PROTOCOL.md), SHA256
`0ba412cb149482a4cf96208ee37219df78f535e9941edc9f616940f30d4cbfe9`.
The supplement permits one diagnosed reader-contract correction and leaves
the scientific 95-case panel, solver, norm and independent arithmetic
unchanged. The original reader's defect and its observed first-attempt
results remain in [CHECKER_BINDING_DEFECT.md](CHECKER_BINDING_DEFECT.md) and
[READOUT.md](READOUT.md).

The immutable executable SHA256 is
`5f9d61a4bb9c63f226c9d90b175faa31758fb6aaf8711b51fa4767407ac6acec`.
The corrected reader source SHA256 is
`a75480fdf4a3731f5106381bba7847b3c93d6963a825656b1a68feb8510072fe`.
The complete 15-file source inventory has SHA256
`51d1fd2e790592d6772557714581381e500f3e195a130b6ef0bde4479f28e91f`.
This inventory includes Cargo.lock and the correction protocol before the
corrected build; it does not retroactively change the first producer's
omitted lockfile hash.

Every reviewer call had its own fresh output directory and distinct
pre/post-call manifests. Each used a 180-second deadline, 2-GiB virtual
memory limit and one configured worker. They ran sequentially after parent
release, with no source edit, rebuild, native retry or resource increase.
All 15 source files, the binary, producer raw data and retained producer
proofs passed their post-call hash checks. All three stderr files are empty.
[pcc:unverified]

## Completed calls

| Reviewer call | Actual result | Evidence |
| --- | --- | --- |
| Corrected qualifier replay | Exit 0; exact producer stdout and complete saved-object equality | [Output](replay2/qualifier.stdout.jsonl), [comparisons](replay2/comparison-status.txt), [pre-call](replay2/pre-call-manifest.txt), [post-call](replay2/post-call-manifest.txt) |
| Saved-proof reader | Exit 0; 27 certificates, 20 S3 inputs, 111 identities, 275 grid cells | [Output](checker2/checker.stdout.json), [pre-call](checker2/pre-call-manifest.txt), [post-call](checker2/post-call-manifest.txt) |
| Fixed reader controls | Exit 0; 39 required rejections and three positive checks | [Output](controls2/controls.stdout.json), [pre-call](controls2/pre-call-manifest.txt), [post-call](controls2/post-call-manifest.txt) |

The corrected replay output is 40,665 bytes, SHA256
`92d1d2d715da2cc51410f119237dbb75307a67180c9970187c407ab5e26f01c5`.
The saved-proof reader output has SHA256
`4bc08baee8aa5577d70ff832f2bccfcdcfcc182a98b200ccef70a2568dec9261`.
The reader-control output has SHA256
`8f80fc42b50e757980d615c7062334f1c5513231f73d1a93d537b7c14a45cedd`.

## Unchanged scientific outputs

The reviewer replay matches the corrected producer stdout byte-for-byte.
Its 27 complete certificate files and 27 input sidecars also match that
producer exactly. A separate comparison finds all 54 files identical to
their first-attempt versions. The sorted 54-file hash inventory retains
digest `30b8233a10ebbeb43c3ee3c4d4e854bd0020b5d87792d0ea1965e3b14456d5ef`.
[pcc:unverified]

The two producer outputs differ only by the new input-sidecar hash and size
fields and their schema descriptions. The retained
[projection](replay2/scientific-record-projection.jq) removes precisely
`input_sha256` and `input_bytes` from case records and those two descriptions
from the scope record. It does not remove counts, certificate hashes,
support, degree partitions, masks, construction diagnostics or other
scientific fields. Both projected outputs are byte-identical, with SHA256
`f19a4806299fa117bb24fda0a2da4bf599fe22337ea5732e4dba7763f4d23ef3`.
The projection uses JSON parsing and field deletion only; it performs no
cryptographic arithmetic or result recomputation.

This comparison includes all 95 recorded certificate hashes and sizes.
It is not a claim that all 95 complete objects were saved. The preselected
27 were retained; the other 68 were checked inline and recorded by digest
and size. Both versions keep that same persistence scope.

| Scientific qualifier category | Reproduced count |
| --- | ---: |
| All cases | 95 |
| Curve / synthetic cases | 88 / 7 |
| Direct tensor-norm constructions checked | 83 |
| Polynomial identities / complete-grid cells | 618 / 24,454 |
| Certificate mutations rejected / invalid inputs refused | 318 / 13 |
| Empty affine queries | 41 |
| Aggregate affine support across the declared cases | 853 |
| Complete certificates persisted | 27 |

These are the executed fixed-panel counts in the
[replay output](replay2/qualifier.stdout.jsonl). The new 39 reader controls
are additional input-binding checks, distinct from the existing 318
certificate mutations and 13 input refusals. [pcc:unverified]

## Corrected saved-proof contract

The reader reports `input_contract="fixed_retained_panel_v2"`,
`input_registry_binding=true`, `aggregate_certificate_binding=true`, and
`input_sidecar_bytes_previously_hashed_in_raw=true`. It requires exactly
the 27 predefined identifiers and their certificate/input file pairs.
Expected kind and parameters come from its independent registry rather
than a sidecar's choice of verification branch.

All 20 S3 cases passed registry matching, canonical smaller target-ordinate
checking, independent curve/selector/right-message/H input binding,
certificate identities and full grids. All seven synthetic cases passed
the independently specified exact field/F/G/H and root-inventory contract,
as well as their certificate identities and grids. Raw records additionally
bind the exact serialized sidecar bytes, before parsing and semantic
dispatch. [pcc:unverified]

The reader's `raw_case_records=95` counts parsed case records. Its complete
object checking still covers 27 certificates, not the 68 omitted objects.
Both the ordinary reader and control path report
`producer_solver_called=false`; their reviewed dispatch invokes only the
reader and independent verification routines.

## Executed rejection controls

The following 17 mutations were each rejected both with raw JSONL and
without it, giving the frozen 34 rejections. The table summarizes the
actual per-control records, including their retained rejection reasons.
[pcc:unverified]

| Mutation | With raw | Without raw |
| --- | --- | --- |
| S3 kind changed to synthetic | Rejected | Rejected |
| S3 field changed | Rejected | Rejected |
| Curve A changed | Rejected | Rejected |
| Curve B changed | Rejected | Rejected |
| Selector degree changed | Rejected | Rejected |
| Target X changed | Rejected | Rejected |
| Liftability policy changed | Rejected | Rejected |
| Target y replaced by its other valid sign | Rejected | Rejected |
| Synthetic kind changed to S3 | Rejected | Rejected |
| Synthetic-zero replaced by valid empty-F fixture | Rejected | Rejected |
| Synthetic-zero replaced by valid empty-G fixture | Rejected | Rejected |
| Synthetic-zero replaced by valid unit fixture | Rejected | Rejected |
| Case renamed to unknown identifier | Rejected | Rejected |
| Expected case removed | Rejected | Rejected |
| Unknown extra case added | Rejected | Rejected |
| Input sidecar removed | Rejected | Rejected |
| Sidecar case identifier changed | Rejected | Rejected |

Five further raw-only controls were also rejected:

| Mutation | Actual result |
| --- | --- |
| Harmless whitespace appended to a sidecar | Rejected by byte/hash binding |
| Recorded input SHA256 changed | Rejected by byte/hash binding |
| Recorded input byte count changed | Rejected by byte/hash binding |
| Recorded input SHA256 omitted | Rejected as missing input hash |
| Recorded input byte count omitted | Rejected as missing input size |

The original downgrade is rejected with raw by the sidecar hash/size guard,
and without raw by the fixed expected-kind check. Thus the latter rejection
does not depend on byte provenance. The three otherwise valid synthetic
substitutions are rejected without raw by the exact fixture binding; their
generic algebraic validity cannot substitute for the named input. The
opposite valid target ordinate is rejected without raw by the canonical
smaller-ordinate condition. These actual reasons isolate the repaired
contract rather than merely observing an arbitrary failure somewhere in
the reader. [pcc:unverified]

All controls use fresh in-memory copies through the same reader core. The
control output reports `proof_artifacts_modified=false`; the post-call
[proof hash check](controls2/proof-postcheck.txt) confirms every original
saved certificate and sidecar retained its bytes.

## Executed positive controls

| Positive control | Actual result |
| --- | --- |
| Complete original panel with raw JSONL | Passed: 27 certificates and 20 S3 inputs |
| Complete original panel without raw JSONL | Passed: 27 certificates and 20 S3 inputs |
| Whitespace-only sidecar change without raw JSONL | Passed: 27 certificates and 20 S3 inputs |

The whitespace pair distinguishes mathematical input equality from exact
producer-byte provenance. The bare mode checks the fixed mathematical
fixtures; the raw-bound mode additionally requires the recorded bytes.
The output records all three positive controls and all 39 negative controls,
with `passed=true` only after they finish. [pcc:unverified]

## Preserved mathematical scope

The unchanged degree-63/X1 records retain aggregate degree mass
`[[1,1],[2,66]]`, the unique degree-one root 1934056707, and coordinate-mask
counts `zeros=133,row=67,pair=67,union=133,other=0` for both left selectors.
They still have `persist=false` and `norm=null`. Their identities and grids
were checked inline by the corrected replay; they are outside the saved
reader's 27-object scope. Their interpretation is the one given in the
first [independent readout](READOUT.md) and frozen
[cancellation prediction](../../CANCELLATION_PREDICTION.md).
[pcc:unverified]

This correction does not change the affine certificate theorem, the
per-component reduced lexicographic bases, or the limitation that a union
of component bases is not a global basis. It does not infer useful signed
relations from coordinate support. Complete signed witness extraction,
identity charts, subgroup checks, duplicate handling and group replay remain
the consuming solver's obligations.

The full repository release-library gate is separate from these scoped
calls. Its retained parent record reports a timeout during dependency
compilation with no library tests run. This readout does not mark that gate
passed. First-attempt source, manifests, outputs, proof bytes, reviews and
the diagnosed defect all remain preserved; the corrected evidence adds a
new source version rather than rewriting those records.
