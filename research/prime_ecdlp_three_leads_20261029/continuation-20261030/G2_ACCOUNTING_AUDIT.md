# G2 accounting audit: what must be emitted before a comparison

2026-10-30. Read-only source audit of
`faade165a46fd51e5a0903ba8e4df85a134c68db`, with the 17-object binding in
validation/integrated/source-bindings.json. This is a concrete implementation
gap map, not a completed ledger, experiment protocol or priced result. The
remote 0cc5 merge changes eight dependency/source objects; recheck this map
against that integration before using it for new measurements.

## Findings that affect the next implementation

1. **Expanded inversions must not be priced twice.** Constructor `invm` and
   reference `Curve::inv` count an inverse call and execute Fermat powering
   through the same counted multiplication helpers. Scalar `Elimination::inv`
   likewise counts its inverse call plus expanded multiplication work. Treat
   these call counts as diagnostics if pricing the expanded leaf operations;
   do not additionally charge a full inverse weight for the same execution.
   The same hierarchy issue applies to helper square-root calls and counted
   point operations. A leaf-operation total and a calibrated whole-operation
   total are alternative accounting views, not additive components.
2. **Some validation arithmetic is currently discarded.** `Request::validate`
   constructs a temporary ordinary affine `reference::Curve`, checks the target
   and drops its `Counts`. `Ledger.validation.helper_arithmetic` does not
   contain those reference operations. `FastCurve::new` setup also uses its
   own arithmetic rather than the constructor helper counter. A timer enclosing
   both is not an operation-count substitute.
3. **The solver count is explicitly partial.** F4's `FIELD_OPS` covers row
   reduction and named included checks, not all arithmetic in symbolic
   preprocessing, substitution or polynomial evaluation. Its native logical
   counter is documented as a lower bound and is not emitted in the oracle's
   current ledger. Root multiplication counters omit other root arithmetic.
   Existing `unpriced_work` strings identify classes without their counts;
   AGENTS §5a therefore forbids ranking even at the zero-weight endpoint.
4. **Coordinate and scalar costs need separate identities.** Oracle arithmetic
   is modulo p, elimination modulo n. For the public fixture these are17 and19.
   The scalar counts, coordinate helper counts and query-generation `GroupOps`
   cannot be added as one field-operation unit without a frozen per-class
   conversion. A count of point-addition calls is not a field-multiplication
   count. Arithmetic at a different modulus or implementation must retain its
   own class/calibration identity.
   Moreover, `GroupOps.field_ops` and `montgomery_reductions` have
   `#[serde(skip)]`: serializing `query_generation_counts` discards those
   in-memory details. Emit a separate explicit count block rather than changing
   the frozen meaning of the shared `GroupOps` schema or inferring zeros.
5. **Diagnostic clocks do not close the process interval.** The exclusive
   oracle phases and cold producer phases reconcile, but startup, input parsing,
   binding, serialization, independent replay and transport are not all inside
   those clocks. Process-lifetime VmHWM is not an isolated per-query peak. Keep
   an external process envelope and an explicit inclusion policy; do not price
   a missing phase as zero or infer L2 admission from reconciled nanoseconds.
6. **An error return is not a durable failed-call record.** Producer errors or
   a killed process may leave no serialized oracle/LA record. Existing unit
   tests and the native-validation shell supervise software checks, not the
   full scientific target stream. A source-bound native supervisor must journal
   the attempt before child execution and preserve terminal failures/caps and
   unfinished attempts on resume. No such full-panel entrypoint is implemented
   or qualified by this audit.

## Coverage and closure map

| Phase / original source | Existing evidence | Required implementation and discriminating check |
| --- | --- | --- |
| Request/curve validation; `Request::validate`, `run` | Helper delta; diagnostic validation time; ordinary affine correctness check. | Retain temporary reference counts and native curve-setup classes; malformed input must leave a terminal refusal/attempt record without being charged as a successful oracle call. |
| Curve/base setup; `pipeline::setup` | Helper/reference counts and cold setup interval; exact tiny whole-curve order enumeration. | Emit every setup class and base identity, include cache generation/storage once per cold target and declare reuse count. No factor-base logarithms supplied to the producer. |
| Polynomial construction; `algebra.rs`, `system` | Helper ±/× calls with expanded inversion/power work; system digest; construction interval. | Emit comparisons, monomial/container and allocation work; publish counter hierarchy so high-level calls do not duplicate their children. Equality tests must compare complete canonical systems or a separately justified equivalence. |
| F4/visitor; `f4_fp.rs` | Row-reduction multiplications, runs/degrees/matrix dimensions, domain evaluations, branch counts. | Cover normalization, substitution, evaluation, pair selection, monomial/hash probes, assembly/sort and intermediate failure work. Assert counted-leaf conservation on fixed deterministic code paths; retain every unsuccessful call. |
| Root extraction; counted root API and root decode callback | Distinct root-multiplication counters in the visitor and decoder; decode helper delta. | Count additions/subtractions/inversions, repeated roots and factoring/control work, with explicit ownership of each nested call. Completion/cap controls must retain partial work, not a complete-miss label. |
| Lifts/signs/native curve checks | Lift helper arithmetic, signed tuples, point-addition call count, independent verification counts. | Emit native Montgomery/setup/point arithmetic or price a frozen whole-call implementation without double charging field children. Retain nonresidues, rejected roots and duplicate witness effort. |
| Public query generation | `GroupOps` plus one interval per attempt, including identity attempts. | Freeze group/field calibration and count scalar-selection/stream/logical work. Identity queries remain charged and skipped; planted and unconditional target laws remain separate. |
| Relation encoding and scalar LA | Original rows, rank-increasing yield, scalar multiplication/subtraction/inverse calls; independent replay certificates. | Count row allocation/normalization/copy/comparison/provenance and remainder arithmetic; keep inversion children separate. Use explicit row weights/kernel witnesses to check outcomes, not an assumed full-rank probability. |
| Recovery and verification | Final scalar equation and reference counts; diagnostic verification interval. | Declare algorithmic verification versus independent audit intervals; emit and price each included class. Independent replay has no complete counters yet and is outside producer clocks. |
| Process, memory, persistence and resume | Process-lifetime RSS and current negative checkpoint control; no qualified panel route. | Qualify source/binary/image/command, process/memory limits, monotone event IDs and atomic durable attempts/outcomes. A resumed censored record keeps its work and does not receive an uninterrupted timing claim. |
| External solver and rho/BSGS references | No qualified matched comparison in this continuation. | Pin equal-domain external input/extraction, source/options, strongest same-target references and every unpriced reference class. Refuse ranking if either arm lacks an emitted required count. |

## Acceptance before computing ratios

- Freeze the complete class inventory and ownership before measured data.
  Every class has a concrete emitter, unit, field/implementation identity,
  inclusion interval and either a terminal count or an explicit unavailable
  state. Unsupported classes are not coerced to zero.
- Price counted residual native operations symmetrically at0,0.1,1 GAE,
  preserving the same treatment for IC, external and rho/BSGS arms. These
  weights do not fill in unknown counts. Conversions between field/group/time
  units require the matching frozen calibration identity on both arms.
- Verify counter hierarchy and aggregation over setup plus every attempted
  query, including misses, caps, rejected/duplicate outputs and failed
  processes. Report cost per verified decomposition only with a positive
  denominator, and separately per independently rank-increasing relation.
- A terminal complete record must reconcile requested/attempted/completed/
  capped/failed/not-reached counts, original-row useful rank and final recovery.
  Timeouts are censored, not zero cost or refutations. No finite speed ratio
  follows from a timed-out baseline.
- Preserve original G4 thresholds and G5/G6 criteria. The strongest reference
  and the full pricing band must support the same claimed ordering. A missing
  class, unmatched calibration or ordering reversal keeps it unresolved.

## Sources and limits

Primary PKM2016 Algorithm1/§3.2 includes relation dependence and Step5 linear
algebra; it does not justify using decomposition time alone as cold DLP cost.
Repository AGENTS §5a requires every unpriced counter and the symmetric band.
`docs/ic/measurement/README.md` §3 distinguishes calibrated GAE/RPS, raw counts
and time, and labels incomplete totals as lower bounds. Relevant actual code:
`examples/pkm_oracle_program.rs`, its `algebra.rs`, `pipeline.rs`, `reference.rs`,
`src/cryptanalysis/f4_fp.rs` and `src/cryptanalysis/ic_boundary.rs`.

No cryptographic computation or pricing was performed for this audit. No source
or acceptance rule was changed. The hierarchy observations follow directly
from the named function call paths, not from a new performance experiment.
They must be covered by meaningful native counter tests when implemented.
G2 remains partial and `comparison_admissible=false`.
