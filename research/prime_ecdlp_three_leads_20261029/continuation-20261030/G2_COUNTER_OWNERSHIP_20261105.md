# G2 counter retention and ownership

2026-11-05. This implements two emitter fixes identified in the historical
G2_ACCOUNTING_AUDIT.md; it is not complete cost qualification or optimization.
[Fresh native validation](validation/g2-counters-20261105/receipt.json) passes
18 oracle tests, strict lint/format, CLI build and actual checkpoint-refusal
message/expected exit1 on sourcec41dc4fb2. The unchanged library evidence
remains failed/capped incomplete; no full-suite pass is claimed.

## Implemented changes

`Request::validate_counted` retains ordinary-affine target-validation counters
in a caller-owned accumulator, including arithmetic before a canonical off-curve
refusal. Early schema/noncanonical refusals do not invent field work. The oracle
producer emits `ledger.validation_reference_counts`; cold setup merges this
validation work into `setup_reference_counts` exactly once. Unpriced export and
replay retain the old validation wrapper. Failed top-level calls still return
errors without a durable full-panel record: journaling remains open.

Each public query attempt owns a local `GroupOps`. It is retained even for an
identity RHS and merged into the total once. New explicit per-attempt and aggregate
`generation_field_counts`/`query_generation_field_counts` emit the shared
`GroupOps` fields that serde deliberately skips. The shared schema is unchanged.
The block states coordinate modulus p and ordinary-affine implementation; scalar
elimination stays modulo n. On the fixed development fixture p=17 and n=19.

## Nonoverlapping ownership

| Block | Owner and interpretation | Aggregation/pricing rule |
| --- | --- | --- |
| validation reference | oracle validation, or cold setup's own validation call | Separate from helper arithmetic; distinct implementations need their own units/calibrations |
| per-attempt group/field | this attempt's query generation including identity RHS | the aggregate view of these same attempts |
| aggregate group/field | sum of all query-generation attempts | whole-group calibration plus its nested field-call pricing |
| helper/reference/scalar inverse calls | diagnostics; multiplication helpers already expand their Fermat powering | a whole-inverse price on top of those expanded leaves |
| query `PrimeCurve` inverse count | whole `invmod` call; internals are not emitted as expanded field operations | an assumed expanded inverse cost without a matching calibration |

Query field calls remain partial: additions/subtractions, inverse internals,
comparisons, scalar sampling, stream/container work and other logical classes
are not complete. Native reductions are explicitly emitted; zero in this ordinary
affine implementation is not an inference about all implementations. FastCurve
setup, solver/root coverage, whole-process and independent-replay accounting,
failed-process journal, qualified runtime/resume and matched external/rho arms
remain open. There is no cost aggregator or GAE conversion here.

## Meaningful native checks

- Accepted target: exactly one point test, four multiplications and two additions
  survive serialization; off-curve/noncanonical/schema refusals conserve the
  caller accumulator and do not fabricate unavailable work.
- A 128-attempt capped correctness control includes identity queries; each emitted
  attempt is checked against reconstruction from public a,b, and sums exactly to
  the aggregate field/group ledger. Setup retains 17² enumeration point checks
  plus one target and one generator check.
- Scalar inversion over F19 records one inverse diagnostic plus six expanded
  products for exponent17, preserving the hierarchy used by the ownership rule.

These are software regression controls, not scientific panels. Request/producer
schemas remain v1 with additive output fields; replay v2 still validates
mathematical rows/certificates, not the correctness/completeness of counters.
The earlier byte-identical-producer-prefix statement applies to sourcefaade only;
this accounting change intentionally modifies producer setup/query telemetry.
Old evidence is not reinterpreted as containing the new blocks.

Sources checked before choosing the fix: primary PKM2016 Algorithm1/§3.2
(<https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>), the repository
measurement unit contract (`docs/ic/measurement/README.md` §3), the actual
`ic_boundary::{GroupOps,FieldOps,PrimeCurve}` call paths, and both prior G2
audit/recheck notes. Query generation, relation collection and scalar LA are
separate work owners. Standard count retention is accounting engineering, not
an ECDLP algorithm or mathematical novelty result. G1-G3/G6 partial; G4/G5
unrun; comparison_admissible=false, all scientific metrics null.
