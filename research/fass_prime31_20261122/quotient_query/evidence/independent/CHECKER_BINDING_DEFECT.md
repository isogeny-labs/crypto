# Persisted-reader input-binding defect and correction review

Date: 2026-11-23 UTC. This is source analysis before corrected-source
qualification. The original evidence, `MATH_REVIEW.md` and first-attempt
`READOUT.md` remain unchanged. No corrected native build or run is reported
in this note. All conclusions have unverified PCC standing.
[pcc:unverified]

## Exact defect

The first reader deserializes a sidecar, then selects its input check using
that sidecar's `kind` field. Its S3 branch performs original-curve input
binding, while its synthetic branch performs no additional mathematical
fixture binding. The raw record binds certificate bytes but contains no
input-sidecar hash or expected kind.

Consequently, changing an S3 sidecar's kind to `synthetic`, while preserving
its case identifier and root lists, takes the synthetic branch. Its unchanged
algebraic certificate, root grid and raw certificate hash still pass. The
original S3 binding is skipped. This is a source-derived counterexample to
the intended input contract; it was not executed as an extra native call
while the root gate was running. The flaw is in the reader's choice of what
to verify, not in its polynomial identity checker. [pcc:unverified]

All 20 S3 sidecars observed in the first persisted-reader invocation were
actually labeled S3 and received their checks. Their retained bytes also
matched the separate-role replay. Those observations remain valid. They do
not establish that the old reader rejects a relabeled input. The output's
explicit statement that sidecar bytes were not previously hashed is an
honest limitation, but it does not repair the dispatch defect.

The affected original reader has SHA256
`290ce46f9a7d731452a46caeeaf3cf35af5125b71c68e14a754183e47586b0fd`
in the immutable first-attempt source snapshot. That snapshot has not changed.

## Correction contract

The coordinator froze [CORRECTION_PROTOCOL.md](../../CORRECTION_PROTOCOL.md),
SHA256 `0ba412cb149482a4cf96208ee37219df78f535e9941edc9f616940f30d4cbfe9`,
before the corrected build or any corrected native call. It preserves the
95 scientific cases, their 27-object persistence choice and all resource
bounds. Only reader input binding, producer metadata, CLI routing and
interface documentation change.

The corrected reader independently specifies the following complete panel:

| Case family | Expected field and curve | Degree and targets | Left policies | Count |
| --- | --- | --- | --- | ---: |
| `f97-d2` | p=97, A=2, B=3 | d=2, X in {0,1,3,30} | all and liftable | 8 |
| `f97-rank-one-d2` | p=97, A=96, B=0 | d=2, X in {0,1,2} | all and liftable | 6 |
| `prime31-d7` | p=2147483647, A=847514846, B=2087444724 | d=7, X in {0,1,2} | all and liftable | 6 |
| Synthetic | p=97; exact F, G and H below | Seven fixed inputs | Fixed complete roots | 7 |

It requires exactly these 27 identifiers and their 54 certificate/sidecar
filenames. Unknown, extra or missing cases and missing sidecars are errors.
It dispatches from the independently specified expected kind; the supplied
kind must agree. S3 field, coefficients, selector degree, target X and lift
policy must match the case identifier's expected fixture. The target y must
be the smaller rational ordinate and satisfy the curve equation. The
existing independent S3 routine still checks the complete selector, left
and right root inventories, actual H and liftability policy.

For synthetics, the independently expanded baseline polynomials over F97 are
F=[91,11,91,1] and G=[0,2,94,1], with low-first coefficients. The five H
values are 1, 0, tx-1, t(t-1)x+(t-1)(t-2), and x-t. The empty-F case instead
has F=1, and the empty-G case has G=1; both have canonical H=0. The reader
requires those exact field, F/G/H and complete root inventories, in addition
to the existing certificate identities. A different mathematically valid
synthetic certificate cannot stand in for the named input.

With raw JSONL supplied, the producer now commits to the exact input-sidecar
bytes through `input_sha256` and `input_bytes`. The producer serializes the
metadata once and uses those same bytes for hashing and persistence. The
reader checks both fields before parsing the sidecar or dispatching its
semantic checks. Without raw JSONL, the fixed registry still binds the
mathematical input, but no producer-byte provenance is claimed.

These are complementary checks: the registry prevents an attacker from
choosing the verification problem, while the raw hash and length bind the
specific serialized input used by the recorded producer call.
[pcc:unverified]

## Fixed correction controls

The controls use the same reader core as the filesystem entry point. Each
mutation starts from fresh in-memory copies of the original proof files or
raw bytes. The control path writes no proof object, invokes no solver or
norm constructor, and returns an error if any required rejection or positive
check has the wrong result.

The frozen protocol names 17 semantic/artifact mutations, each checked with
and without raw JSONL, for 34 required rejections. It additionally names
five raw-only rejection checks, for 39 total. The three positive checks are
the complete original panel with raw, the complete original panel without
raw, and a harmless whitespace-only sidecar change without raw.

Three controls substitute otherwise valid synthetic certificates and their
input sidecars for `synthetic-zero`: empty-F, empty-G and unit. Their generic
algebraic identities remain valid. Rejection without raw must therefore
come from the fixed fixture contract, which separately tests the F, G and H
obligations. The target-ordinate sign control uses 87 in place of 10 at
x=0 on the ordinary F97 fixture; both square to 3, but only 10 is the
declared smaller representative. The whitespace control separates semantic
equivalence from exact-byte provenance.

The 39/3 figures in this section are the frozen required counts, not
execution results. Actual outcomes must be retained from the corrected
immutable binary and reported separately.

## Source review and unchanged arithmetic

The final corrected reader prepared for review has SHA256
`a75480fdf4a3731f5106381bba7847b3c93d6963a825656b1a68feb8510072fe`.
The native implementer performed a separate read-only review against the
frozen correction protocol and found no remaining source issue in the
registry constants, expected-kind dispatch, raw/bare paths, canonical
ordinate guard or exact control list. Formatting parsed successfully; no
corrected compilation or algorithm execution is asserted here.
[pcc:unverified]

The independently written arithmetic module remains
`f727fc31a662eb3a7e7516eff674f57aa28beecf346a0eb94f7bb8bb0fadfe2c`.
The query kernel remains
`8b0ee7f0c5b405d05714f662e0b82f700e2d62fa620718776b6567a44beb02f9`,
and the tensor-norm module remains
`f77d2f28f83a490a24b8f5642d7e432734b5530f727c3bccb4027b02c35548ca`.
These match the first-attempt sources. No new field, target, selector,
algebraic hypothesis or signed-relation conclusion was introduced by this
reader correction.
