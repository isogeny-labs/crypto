# Checked polynomial-consequence training contract

Frozen before implementation, 2026-10-30. Parent runtime d9a69910198d; isolated
branch codex/ml-dataset-algebraic-targets-20261030 based18e2a8f275ee. T886 owns the
new module/research directory. T828 explicitly delegates shared prefix/export
edits in this isolated worktree; the parent worktree remains frozen under tests.
This is native software development, not a scientific experiment or cloud wave.

## Mathematical object and prior work

For a retained pre-native polynomial system F over an explicitly declared prime
field, a target g is admitted only after checking g = sum(c_i X^a_i F[j_i]).
Reuse the native prefix certificate finder's right_from_left witnesses from
already ideal-equality-certified nodes, then independently replay each single
identity with the existing direct containment checker. Do not create another
arithmetic engine or label the searched certificate as a native execution trace.
This first increment does not salvage rows from a failed whole-node equality
search; that requires a later one-direction per-target search contract.

A witnessed degree reduction is certificate-dependent: the largest degree of a
nonzero generator-multiple summand exceeds the nonzero result degree. Zero
polynomials have null degree; zero results stay restricted as trivial/zero rows
with no degree-drop claim. Empty/zero generators cannot inflate witness degree.
Exact first/last fall degrees, minimum derivation degree, closure completeness,
nonmembership and native instruction totals remain null or false.

Checked prior work: Huang-Kosters-Yang-Yeo,
https://arxiv.org/html/1505.02532v2, Definitions2.1/2.5. Their bounded closure
permits multiplication of derived polynomials; last fall quantifies all ideal
elements. A checked generator identity alone does not satisfy those stronger
obligations. Existing prefix_algebra.rs directly checks identities;
pc_degree_harness.rs operational labels are not imported as certified targets.

## Native API and error boundary

Extend prefix_algebra.rs with verify_identity(generators,target,proof,n,p,limits)
returning Result<IdentityAudit,String>. IdentityAudit retains Work and a typed
Verified/UnknownCap outcome. Caps have typed kinds, without matching error
strings. Keep partial coefficient work on caps. Reuse verify_containment and
Ring arithmetic; maintain existing equivalent() behavior/tests.

Reject invalid rings/dimensions, noncanonical or out-of-range coefficients,
duplicate monomials/proof keys, invalid generator indices, malformed exponent
encodings and false identities. Polynomial row/term/degree bounds, proof storage,
and arithmetic budgets yield UnknownCap; an uninspected tail is not certified.
The API retains p<2^62 support and decimal-string field coefficients. The actual
recursive native adapter remains m3..15,p<2^32 and must say so.

Report result_degree, max_generator_multiple_degree and witnessed_degree_drop
only for verified identities. New identity work counts direct multiplication and
accumulation, excluding parsing, primality, graph construction and I/O. Existing
equality-audit work combines search and prior identity checking; do not relabel
it pure search cost or combine either count with native CPU instructions.

## Public inputs and restricted targets

Add algebraic-features.jsonl, schema ml-dataset.algebraic-features/v1. Its inputs
contain task prime-polynomial-consequence/v1, the existing strict initial public
projection, ordinary ring (decimal modulus, variable count, empty quotient rules),
active original variables, decimal fixed bindings and the pre-native source
polynomial graph. Use the existing graph construction and stable input digest.
Every target for a context shares the same feature identity. No output basis,
row/node/event ID, proof, cost, outcome or degree-reduction label enters inputs.

Add restricted/algebraic-records.jsonl for target graph, multiplier witness,
identity audit/work, nullable degree labels and source target/event/node/row joins.
Use derivation_origin separately_searched_generator_identity_for_retained_native_basis_row.
Set native_operation_witness=false, exact_first_fall_degree=null,
exact_last_fall_degree=null, minimal_derivation_degree=null. Retain the equality
audit reference and its correctly named combined-work scope. A capped row has
no certified target/proof; it retains its reason and partial work.

Route a new polynomial_consequence record kind through the existing replay
callback to separate sorters. Static and branch-feature identities remain stable.
Feature allowlist v6 explicitly names all three model-input files. Add manifest
counts for admitted/nonzero/zero/drop/unknown outcomes, with clear counting units.
The actual dataset export CLI and immutable retry must install and verify these
new files. Source raw artifacts remain canonical and unchanged.

## Bounded independent channel

Give consequence extraction a separate cumulative budget: 10,000,000 direct
coefficient operations, 4,096 examined target rows, 64MiB of serialized derived
feature/record payload. The small terminal censoring record is outside that
payload budget and separately bounded. Count the exact declared payload scope;
final sorter envelope bytes are not silently included in this count.

Admit rows immediately. The first exceeded channel budget emits one restricted
unknown-suffix record and disables further consequence extraction for that
stream, preserving earlier rows. Existing branch replay can continue under its
unchanged budget. Per-identity limits use remaining channel budget; no per-row
reset may allow unbounded aggregate work. A malformed admitted identity is still
a hard export error. No model learns a capped check as a negative example.

## Validation and delivery obligations

- Exact cancellation and zero-identity checks; reject coefficient/generator/ring
  corruption, duplicate monomials and duplicate multiplier keys.
- Exact coefficient representation above2^53, including p=2^61-1 arithmetic.
- Retain earlier verified rows after an operation/row/output cap. Preserve partial
  counts and emit only one terminal unknown record.
- Several targets share one public context identity. Future results, provenance
  and costs cannot change its bytes. Target coefficients/proofs stay restricted.
- Replay native fixture events and the actual export CLI; verify joins, allowlist,
  artifact hashes, immutable retry and unchanged source inventory.
- Preserve all failures. Freeze the source before execution and run required
  full-library/lint/CLI gates before publishing. Do not run competing builds while
  the parent pipeline is active; this worktree is implementation-only for now.

This increment leaves the full Koblitz/binary/prime, curve/twist, n23..31/47..61,
p2^61-1,m2..15 campaign and remaining solver targets intact. Unsupported/capped
cells remain coverage gaps. No generated corpus, exact fall-degree result,
trained-model advantage or new SAT solver is established by this implementation.
