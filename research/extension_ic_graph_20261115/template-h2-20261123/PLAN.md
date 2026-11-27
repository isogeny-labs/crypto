# H2 phase 1: provenance-template construction control

Proposed, local, Rust only. No performance result is asserted. PR #5094
continues the existing graph-IC lane after the consumed H1 coordinate panel.

Question: can an exact original-generator provenance constructor produce a
bounded reusable template for the actual six-coordinate Nagao family?
This phase implements the unreduced provenance-template construction from
Larsson–Åström–Oskarsson (2017), equations 9–12. Module-syzygy reduction is
a later distinct step; this prototype must not be called syzygy-minimal.

## Construction and correctness

Use the existing Rust `certified-relations::reference_criteria` only to
construct a development Groebner basis with exact original-input cofactors.
Take the union of generator/monomial-multiple keys in those cofactors.
Compile the coefficient matrix of those multiples and their column map.
Online, fill and eliminate that fixed matrix, retaining the row-operation
coefficients as explicit original-generator membership witnesses. Extract
the minimal leading-monomial rows; accept only after exact membership,
original-input containment and all S-pair checks, plus the frozen leading
monomial fingerprint. A failed check means refusal requiring the caller's
normal F4 fallback. Never return an empty root set for refusal.

The prototype returns a complete basis rather than only an action matrix.
This isolates the provenance/support construction before a border-only
or syzygy-pruned variant. The integrated default remains unchanged.

## Qualification, then one bounded construction probe

1. Native unit controls: ordinary parameter family, zero/degree drop,
   changed pivot/leading ideal, inconsistency, malformed support/prime,
   explicit resource cap, and mutated provenance. Every accepted basis
   must pass the existing exact checker; refusals remain visible.
2. Freeze the qualified source/executable hashes and publish before launch.
3. Exactly one guarded probe reads the immutable H1 saved `bases.jsonl`.
   Development is case 0 only; held-out application is case 1 only, if
   development produces a bounded template. All other saved cases remain
   unconsumed by this new probe. This is exposed same-fixture evidence.
4. Native generic reference construction is run once on development case 0.
   Apply the template once on case 0 and once on case 1. Do not expand a
   refused template or repeat the call under the same protocol.
5. Cross-check accepted ideals against the saved unrestricted F4 bases in
   both directions using exact polynomial normal forms. A complete basis
   does not need a new assumption that the quotient dimension equals 64.

Source/input checksum pins accompany the qualified executable. Original
input is `/workspace/ic-local-runs/action-operator-h1-20261123-01/output/bases.jsonl`;
its SHA-256 is frozen in `input.sha256` before the call. The original H1
records and all failed attempts are preserved.

## Costs, limits and interpretation

Local backend authorized by the user; one thread; 8 GiB virtual-memory
limit; 120-second outer limit with 5-second TERM grace. No paid resource.
Bound the compiled template to 512 rows, 1024 columns, 1,000,000 coefficient
and provenance cells, maximum total monomial degree 16 and at most 16
variables. Training also retains the existing reference constructor's
pair/basis/proof budgets. Emit phase-start records before expensive work.

Record construction and exact-check field-operation counters separately
from online fill/elimination/check counters, matrix shape, refusal reason
and basis equality checks. These counts have different coverage from F4's
partial counters: do not claim a speed ratio from them. No runtime or IC/rho
parity claim. This phase has no fresh target or relation-yield measurement.

The first falsifier is failure to construct within these explicit limits.
A large unreduced template can motivate a distinct syzygy-pruning attempt;
it does not refute every compiled template. If construction works, freeze a
separate comparison with the already integrated traced F4 and action/root
recovery, charging learning, checks, fallback and all setup. Do not repeat
the consumed H1 or prior Runpod commands.

## Sources and custody

- Larsson–Åström–Oskarsson 2017, §§2–3, exact primary text retained under
  `/tmp/ic-hypotheses-20261123/sources/larsson2017.{pdf,txt}`; linked and
  hashed in `../hypotheses-20261123/README.md`.
- Larsson et al. 2018, §§1–2; same retained primary-source packet.
- Existing `crates/certified-relations/src/lib.rs`, its exact checker and
  `src/cryptanalysis/f4_fp/{certified,reuse}.rs`; the generic reference
  carries explicit provenance, while the structured quotient recognizer
  alone does not establish support for this Nagao family.
- KB unavailable: MCP not loaded; repository wrapper has no executable.
  Conductor executable unavailable. No novelty or absence claim.

S3 checkpoint/artifact upload credentials remain unavailable. Use the
explicitly authorized local backend with published Git plan/source and
local hash custody, as in H1; do not assert cloud/isolation admission.
Commit compact results and raw manifests. Oversized raw data stays LOCAL
pending the required artifact-store publication; no splitting to avoid
artifact limits. Stop and preserve partial output on timeout or refusal.
