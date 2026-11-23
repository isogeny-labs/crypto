# Source review: exact matrices and shared S3 elimination

Date: 2026-11-22. Tracking: PR #4922, `lane/s3-shared-elimination`.

## Review status and provenance

This is a producer-independent source review within the same assistant team:
the reviewer did not author the implementation. It is not a separate trusted
verifier, signed PCC verdict, or formal proof. All findings are
`[pcc:unverified]`. The reviewer read source and derived arithmetic and row-space
invariants; the reviewer did not compile, execute scientific tests, benchmark,
or dispatch jobs. Algorithmic test cases mentioned below were inspected, not
reported as passing.

The inspected development snapshot preceded the code commit; its checkout HEAD
was the task-only commit `38f1467ebee77b6fc507d5539760ff7589880114`.

| Reviewed file | SHA-256 of inspected bytes |
| --- | --- |
| `src/cryptanalysis/exact_modular_matrix.rs` | `20a809c6b072bec647b12f3a347c0c8bfb2ddc4f641acaf2e7fb6bc9df7c3b33` |
| `src/cryptanalysis/shared_macaulay.rs` | `850d424798d33d50d162e437c85a8467aa4b7a69792f973dd176cf1e59063996` |
| `examples/shared_s3_matrix.rs` (after callback correction) | `87d06d5c272cce98993dd5271e7c5cff6c143a25f1699f007ef39da613e9f74e` |

## Findings and corrections

Inspection found no unresolved correctness blocker in the arithmetic and
row-space transformation. The initial favorable fixture review missed a
callback-contract defect that execution subsequently exposed; the correction
and review limitation are recorded below. [pcc:unverified]

| Review finding | Resolution inspected |
| --- | --- |
| The initial fresh comparator depended on shared preparation and repeated parameter specialization for every multiplier row. | `FreshMacaulay` now prepares metadata without numerical elimination or projection. A target specializes each generator once before copying multiplier rows. Fresh and shared use the identical frozen envelope and reduction policy. |
| Common rank must be observed without a second elimination charged to fresh. | Incremental elimination records common-prefix rank during one pass; out-of-order pivots, processed-zero-row compaction, and the final pivot permutation preserve the basis invariant. |
| Aggregated report counters could wrap; kernel counters could be double-counted. | Checked additions replace unchecked merges. Documentation identifies kernel counts as a subset of logical products. Coverage remains explicitly incomplete. |
| The originals helper initially exposed ascending cache order despite the polynomial API's leading-first convention. | Returned originals are now sorted by descending degree and lexicographic order. F4 also normalizes its input. |
| FMA boundary and matrix rejection coverage needed explicit cases. | Source now contains a pre-correction remainder of -1 and a noncanonical destination rejected without mutation, plus the explicit quotient-error bound. |
| Rank-drop tests needed interaction with common projection. | Source now checks common `x-y` with dynamic `y+t` and `y+1`, including the drop at `t=1`, against independently assembled rows. Separate cases cover fresh success when shared setup is refused. |

### Source-review miss exposed by integration execution

The original fixture reviewed at SHA-256
`869f744651050827f67754af1c923bfefa78b32233e45d64e6fb17de62f10c89`
returned `solutions.len() <= 459` from its visitor. The existing
`solve_visit` contract stops on `true`, so this stopped at the first root.
The integration execution retained in
`evidence/integration-tests.postcall.log` rejected that result as incomplete.
The reviewer read this failure record and acknowledges missing the callback
polarity during source inspection. The initial fixture assessment therefore
did not establish correct traversal. [pcc:unverified]

After the owner corrected the source, the reviewer inspected both callback
sites against the production contract: exhaustive visitation now returns
`solutions.len() > 459`, while relation search returns `witness.is_some()`
after equation and signed-point checks. No-witness early stopping is explicitly
incomplete. The original domain cap, completion requirements, and solution-set
equality assertions remain. The table above pins this corrected source.
Replay after correction belongs to a separate producer receipt; this review
asserts no new test pass. [pcc:unverified]

### Arithmetic argument inspected

For binary64 unit roundoff `u=2^-53` and canonical residues modulo an admitted
prime `p<2^32`, the quotient estimate has absolute error below
`p*(3u+3u^2+u^3)<1`. Its floor differs from the exact floor by at most one.
The product error and fused cancellation residual are exactly representable
integers of magnitude below `2^34`; their sum and the modular correction are
exact. Split16 products and partial sums satisfy
`2^21*65535^2<2^53`. Both general-prime Horner recombination and Mersenne31
recombination remain within their integer bounds. Matrix shape, work, payload,
and residue validation precede output writes. [pcc:unverified]

### Row-space argument inspected

For common RREF `[I|B]`, projection `[A|C] -> C-A*B` preserves the row space
together with the common rows. Linearity permits projecting pencil coefficient
blocks separately; the code recombines them at the target before residual
elimination. It freezes multipliers from union support and recomputes target
pivots. Common consequences are retained, and omitted high-degree generators
remain in `original_equations`. A bounded preparation is not a completed
Groebner basis, and a resource refusal is not UNSAT. [pcc:unverified]

## Fixture boundary inspected

The expanded S3 pencil preserves its coefficients. F4 inputs retain original
equations and factor-base domains, with auxiliary variable over full F17.
Interrupted visitation is treated as incomplete. Signed lifting checks the
exact point sum and the finite prefix represented by the auxiliary coordinate.
The fixed sign choice `G+(-G)+G=G` is rejected for every finite auxiliary
coordinate; the fixture makes no complete projective-chart claim.
[pcc:unverified]

The toy recovery function receives public `Q` only. It uses an explicit `G`
anchor, inserts `sum(c_i*log_G(P_i))-k=a (mod 19)`, and checks `kG=Q`.
The known scalar stays in the caller. End-to-end toy F4 integration and recovery
use the integer backend; all-backend coverage concerns preconditioning and
equation equality. The p31 fixture is a known point-sum and matrix check, not
a p31 unknown-scalar recovery or full-field search. [pcc:unverified]

## Remaining evidence

Development execution belongs in a separately pinned receipt with commands,
source revision, outputs, and exit statuses. This review asserts no test pass,
performance improvement, complete-cost comparison, or signed PCC standing.
The admitted performance comparison remains open under `PROTOCOL.md`.
[pcc:unverified]

Sources inspected include the three files above, existing `fp_backend::Fp32`,
F4 normalization and visitor contracts, and the existing signed abscissa lifter
and incremental relation matrix. Prior-work findings and source links are in
`REPORT.md`, section 6. Research KB access was unavailable; this review makes
no novelty or absence-of-prior-work claim.
# Prime31 follow-up source review

The November 22 follow-up was independently inspected by a second session
agent, within the same team. Reviewed example SHA-256:
`2312503e4a71fd5d9118c6ef35dba8a4ab56d36ea1b484fba208ee55907bd1f2`.
This is source review, not a separate-host execution or signed PCC verdict.

The reviewer identified noncanonical public coordinates as an input-boundary
defect: the underlying on-curve predicate reduces modulo p, while later group
operations assume canonical coordinates. The example now rejects x or y
outside [0,p) before group arithmetic, and includes both malformed-coordinate
controls. No remaining substantive defect was identified in the frozen source.
[pcc:unverified]

Reviewed obligations: finite signed-pair completeness for the declared rational
non-2-torsion base, repeated-x doubling and cancellation handling, retained
original equations, full auxiliary domain, subgroup-modulus signs and anchor,
public-Q input without the expected scalar, incomplete-outcome rejection,
and the distinction between floating-point preconditioning and unchanged
integer F4/relation linear algebra. The source explicitly limits the new
recovery to a structured known-answer correctness fixture. Runtime evidence
is recorded separately under `prime31/`.

## Matched public-recovery continuation

The subsequent seven-arm driver is reviewed separately in
[matched_recovery/REVIEW.md](matched_recovery/REVIEW.md), including its exact
source hashes and the accepted-assignment/replay-failure retention fixes.
The associated development receipt records execution after that review. The
initial review and failures above remain historical evidence for their own
source revisions; they are not silently retargeted to the changed example.
