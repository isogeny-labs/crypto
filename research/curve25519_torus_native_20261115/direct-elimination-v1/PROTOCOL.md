# Bounded sparse elimination: pre-code qualification protocol

Original objective: discover useful actual Curve25519 relations cheaply
enough to change complete index-calculus scaling. This protocol develops a
native public-system solver interface; its fixed correctness controls are
not a performance, probability, full-base or exponent experiment.

Source-review receipt PCC-20261116-3aeb9e is literature/producer evidence,
unsigned/unverified. Old28-variable/u64 F4 cannot directly accept the actual
system. Implement a NEW isolated tools/torus-sparse-elimination crate over
the existing torus-s4 Field, with dynamic sparse monomials and exact BigUint
coefficients. Old engines, compiler, reference, trace adapter, claims and
source qualifiers remain byte-identical. No new generic mathematical law
or new F4 algorithm is asserted. The first baseline is ordinary sparse
Buchberger, with explicit bounded reduction and standard coprime criterion.

## Native algorithm and output contract

- Accept the public serialized System and public original target only.
  Parse/canonicalize all coefficients modulo its EXACT modulus; never cast
  a255-bit modulus or coefficient to a64-bit field. Prime-field validity is
  an explicit external contract matching Field; reject malformed structure.
- Order monomials by declared graded reverse lexicographic order. Retain
  full exponent/index data; no Boolean quotient unless an input equation
  states that relation. Normalize with counted field inversions; use bounded
  sparse normal forms and degree-priority S-pairs, with coprime criterion.
- Resource ledger charges every field add/mul/inverse (inversion by a
  counted binary Fermat loop), term/monomial visit, pair/reduction step and
  represented storage reservation. This is modeled work/storage, not RSS,
  admitted timing or a normalized attack metric. Record counters even on
  refusal. No invisible fallback, restart or uncharged setup.
- Baseline qualification caps: maximum2048variables,4096input equations,
  degree32,65536stored terms,65536pending pairs,4096basis rows,2^24charged
  primitive/work events,64MiB modeled represented storage. Refuse before
  an allocation/work step that exceeds a declared limit. Each test may lower
  a cap only to exercise the refusal control; do not tune caps after outcomes.
  Admit odd moduli of5..512bits, with primality external. Branch depth is at
  most64; iterative single-root propagation consumes no branch depth. These
  are implementation limits, not mathematical exclusions or a promise that
  full actual systems fit this resource envelope.
- Root extraction first propagates literal univariate linear/quadratic
  equations and tries their rational roots. Instrument the existing Field
  sqrt algorithm on its supported p=5mod8 chart using counted binary powers;
  compare it with unmodified Field.sqrt in separate qualification controls.
  No opaque modular-power cost is priced as a single multiplication.
  Branches are public equation roots, not
  supplied base labels or planted values. Branch choices and all failed
  attempts count. A residual requiring unsupported higher-degree extraction
  is UNSUPPORTED, not absence; implementing generic factorization is a
  separate preregistered extension requiring its primary-source review.
- A ROOT result contains the complete original-variable assignment and must
  satisfy the UNCHANGED original System and independent native group/row
  replay. Partial assignments, a small basis or a passing preconditioner
  are not roots. Original replay cost is recorded separately.
- Distinct outputs: ROOT; CAP(work/storage/terms/pairs/degree); UNSUPPORTED
  extraction; INVALID input/failed control; and algebraic contradiction
  OBSERVATION. No unit-ideal result is called an absence certificate here:
  independent ideal-membership checking is not implemented. Do not expose
  incomplete-basis results as complete bases or turn them into absence.

## Fixed qualification inputs (no target selection after outcomes)

1. Prime101 ordinary-polynomial controls: a triangular linear system; a
   coupled quadratic system requiring elimination; inconsistent equations;
   multiple rational quadratic roots; a nonsquare quadratic; a cubic
   extraction refusal; malformed coefficient/exponent/index inputs. Keep
   explicit original assignments and independent direct evaluations.
2. The same arithmetic patterns at actual p=2^255-19, including coefficients
   exceeding2^64. A1100-variable sparse linear instance checks dynamic
   indexing beyond the earlier28-variable interface without a broad build.
3. Input-system serialization from the unchanged original membership/target
   compiler. Fixed actual distinct q1/k2 base uses h_new=h_old^213156431,
   order_factor60824497, existing curve coefficient486662 and arity2.
   The harness constructs one known-answer target from the two original
   canonical positive points, then passes only System/target to the solver.
   No labels, raw points, signs, scalar or original witness enter it.
   A returned complete assignment must replay the original circuit and
   independently reconstruct/sign-sum the original points. Preserve either
   a root or honest capped/unsupported outcome without fixture replacement.
   This is a different declared bounded base, not a huge-base solve.
4. Serialize the existing1059-variable actual six-member system as a
   dimension/input admission control. Do not launch unrestricted solving:
   a deliberately insufficient deterministic work cap exercises refusal
   before extensive elimination, preserving counters and original subject.
   This control makes no success/yield/performance inference.
5. Force each resource refusal class on fixed tiny inputs. Check that no
   cap/unsupported/incomplete result claims absence, that assignments are
   not truncated and that repeated outputs are not counted as independent
   useful relations. Mutate returned coordinates and require original replay
   to reject them.

Predictions: valid ROOT assignments replay, dynamic/large coefficients are
preserved, and explicit cap/malformed/unsupported controls are distinguished.
The actual bounded relation case is exploratory correctness: record its
observed outcome, not a promised success rate. No success is inferred from
preparing its known target. Any wrong root or accounting/control violation
rejects the IMPLEMENTATION; a budget refusal parks that implementation at
those caps, not the route. Keep every failed source/build/control attempt.

## Before a later comparison or scaling claim

The trace adapter is available but this first engine qualification does not
yet compile a trace-projected relation system. A matched comparison needs
that complete compiler, original-root correspondence/replay, identical public
targets and a named external reference on the same host. Neither this
protocol nor a fixed q1 success can establish gamma. Full original-base
discovery, useful yield, BSGS/indexing, private query tags/one retained row,
selection, sparse kernel, scalar replay and the complete growing family
remain subject to unchanged ROUTE_CRITERIA_V3 and SUCCESS_CRITERIA.

Any actual solver/benchmark campaign still uses existing runner/admission/
budget gates. This pre-code qualification schedules none and authorizes no
paid resource. Preserve source/binary pre-call hashes, complete raw outputs,
native/PCC standing, report/diagram/inspected PDF, then publish through the
existing4397 lane helper. Do not touch parent conflicts or inherited PCC
artifact failures as part of this scientific implementation.
