# Native acceptance fixture02 independent source review

Reviewed exact acceptance70cd094b308ffa2bceff5cb04510f91bda677fc9ede36e0c65b81c26d4839b87
and runa54615462ef0b5f7953bbe03e72542a7ad26cad4711f9936f8c12c8f5cd95937.
Scope: four earlier findings, source-only. No builds, tests, generator, cloud,
external messages or live edits. Authored tests are not executed evidence.

## Required correction1: readiness gate indexing can panic

acceptance.rs403-404 indexes an &serde_json::Map with gates[gate] after checking
only cardinality6. Replacing maintained-clippy with an unknown key leaves6 keys
but panics on the missing expected key, rather than returning a controlled
admission error. The new missing-gate test removes a key and stops at length5,
so it does not cover this actual boundary. Use fallible get or exact-key-set
validation and a separately rehashed equal-cardinality renamed-gate control.
Author confirmed the issue and plans a separate revision03.

## Required correction2: two frozen-input read paths bypass caps

acceptance.rs806 uses inherited unbounded hash(&protocol) on a copied frozen
protocol before command admission;825 uses fs::read on the frozen expected plan.
The new bounded helpers otherwise correctly reject oversized metadata before
read/hash and stop reads at cap+1 even if input grows. These two remaining paths
must use the bounded reader/hash helper. A prior successful admission does not
bound a later read after file growth. Add a path-level control exercising the
helper actually used by plan_inputs. Generated native outputs and copied binary
have separately documented limits and are not conflated with these input caps.
Author confirmed both bypasses and plans a separate revision03.

## Findings already addressed

- frozen() hashes through the existing public protocol_canonical_bytes; it no
  longer routes fractional alpha through the old integer-only helper. The
  visibility companion changes no implementation. The new synthetic power-shape
  test includes alpha0.05/target0.8 and rejects changed alpha. This is a software
  canonicalization test, not an executed official-schema approval.
- progress() distinguishes never_started, attempted_generation_incomplete,
  generated_check_incomplete and checked_complete. Durable start intent is
  explicitly distinguished from successful spawn. Successful generation needs
  an exit0, not-timed-out, not-cancelled receipt and no spawn error; checked cells
  require that receipt. Later verification/export failure cannot become unrun.
- A complete synthetic admission fixture now reaches all required input/plan/
  protocol/readiness checks, then11 separately rehashed negative cases target
  specific gate/receipt/evidence/runtime predicates. These attestations are
  clearly synthetic and do not launch commands or claim research evidence.
- bounded_limit/checked_files/admit/input-copy enforce actual size before hashing
  and cap+1 read limits. The boundary tests exercise exact/one-over and simulated
  post-metadata growth; the two bypasses above are the remaining exceptions.

## Precision limit on failure steps

Command failures retain their label and receipt in the command ledger. Local
inspection, semantic equality, and immutable retry failures leave a global error
and the per-cell generated_check_incomplete state, but not an explicit per-cell
failed validation-step field. The observed cell is retained honestly; exact
stage diagnosis can require reconstructing the command sequence. Parent has
been informed to decide whether the explicit failed_step field is required in03.

## Disposition

Do not freeze02 for execution until the two concrete required corrections are
reviewed. Full acceptance is explicitly false: copied-control workflow/API
fixtures/official two-workflow export remain separate pending gates. No broader
completion or production integration judgment is made by this review.
