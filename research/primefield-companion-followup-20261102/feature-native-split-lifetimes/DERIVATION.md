# Separate value, square and cube lifetimes in one arena

T-1830, 2026-11-07. Derived storage bound and native correctness checks.

## Typed interval construction

The previous allocator retained a value, its square and its cube as one bundle.
Now distinguish three entities (i,0),(i,1),(i,2) for logical node i. The value
always exists; the square/cube exist only when the public schedule flags them.
All present entities are born at step i. Their lifetimes initially end at i,
which covers local power preparation: square reads the value; cube reads square
and value. Extend only the entity actually consumed by later recurrence steps:

- Odd step(a,b,c,d): value(a), cube(b), value(c), cube(d).
- Even step(a,b,c,d,e): value(a), value(b), square(c), value(d), square(e).
- Final output i: value(i) only, pinned until time V.

Allocate inclusive intervals in birth order. Expire only intervals ending before
the new birth; all entities born at one step remain mutually exclusive during
that step. Reuse a free field slot or create one. Compile each operand directly
to its entity's physical slot. An absent power gets usize::MAX metadata and is
never dereferenced because preparation is guarded by the same flags.

Runtime allocates a single arena. Install the value, prepare its square if
flagged, then its cube if flagged. Later steps read exactly the compiled operand
slots. Source input, arithmetic modes, geometry and resultant logic are unchanged.
The optional direct Schedule workspace cap is now in field-element slots,
reported by field_slots(); the older module's live_slots() counted bundles.
The modules are separate: no silent reinterpretation of an existing API is made.

## Proof and bounds

Invariant: every live typed entity has a distinct slot containing the correct
field element. Birth-order allocation only reuses expired slots. Every operand
use extended its own entity's interval through that instruction, so it cannot
be overwritten prematurely. Each flagged power is prepared at birth before
future consumers. Value output pinning preserves extraction order and duplicates.
Induction gives equality to the bundled evaluator in both representations,
including zero values and antipodal inputs. No power is recomputed and no field
operation is added or removed.

Let P be the maximum number of simultaneous typed intervals. The greedy allocator
uses exactly P slots: when it creates a slot, all previous slots are occupied
by live intervals; otherwise it reuses an expired one. This is optimal only for
the fixed inclusive typed intervals and execution order. Final-instruction
in-place reuse, output streaming, recomputation and reordering are not optimized.

Let L be the prior whole-bundle interval peak. Each typed interval is contained
in its node's bundle interval, and each bundle contributes at most three typed
entities. Pointwise typed occupancy is therefore at most three times bundled
occupancy, proving P<=3L. Strict inequality can result from absent powers,
earlier last uses, and output pinning of values without their powers.

## Storage and arithmetic accounting

Per-point field-array payload is16P bytes (two-u64 field wrappers), replacing
three L-entry arrays with48L bytes of payload. This is an allocation-size bound,
not a measurement of total RSS or wall time. Headers, capacity, allocator,
constructor temporaries, outputs, geometry and polynomial storage are additional.
The single allocation also replaces three vector allocations, without an elapsed
cost claim. Setup stores three slots per logical node instead of one; flags,
logical/physical schedules and outputs remain. Compile time and metadata stay
O(V+K), with additional constant factors charged once per Source instance.

The shared-power saving2E+4O-S-C and the caller mode-choice difference
(O-E+A+1)M-I remain unchanged. Typed intervals only change addresses. The direct
evaluate_bounded API rejects a cap below P before allocating the arena. The
source retains its logical-node cap (default4096); normal evaluation uses the
computed P automatically. No total-process memory ceiling is asserted.

Caller-certified prime fields, canonical/nonsingular/nonzero-y point conditions,
word-size bounds and the source's prime/coprime subgroup conditions are retained.
Maximum-index tests concern integer recursion, not cryptographic-size fields.

## Executed checks and replay

validation-01 checks4100plans (singletons0..4095 plus the four prior request
lists). An independent typed-consumer scan reconstructs endpoints; owner
simulation rejects live aliasing; an independent difference-array sweep computes
peakP; every plan agrees. Each also verifies P<=3L against the prior bundled
compiler and unchanged power savings; strict nonbase reductions exist.

160660outputs agree across both modes and all previous bundled/shared/unshared/
recursive routes. Exact-P bounded evaluation passes; P-1 rejects. Existing
4096power-saving checks,100cost choices, logical cap/topology, duplicate/empty,
maximum-index and domain controls remain. Source tests retain1108points,
24windows, direct products/covariance/signs and class/resultant controls.
recovery-01 retains617additionaltargets/5929calls/zero unresolved and63public
comparisons/511calls. Normal build has seven inputs excluding old references.
All native checks pass; warnings remain. Original4563cohort unchanged; no timing,
cloud run, full Cargo-suite rerun or independent signed verdict.

Generate with generate.py then generate-recovery.py; run validate.py and
validate-recovery.py with distinct new directories. Receipts save compiler,
commands, input and stdout hashes; publication.json pins deliverables. Preserve
all earlier evidence.

## Sources and next construction

KB unavailable (wrapper no executable). Read prior bundled allocator and proof;
[Poletto-Sarkar1999, section4/Figure1](https://c9x.me/compile/bib/linearscan.pdf)
was read in the preceding component and supplies the established interval
expiration method. Typed containment and exact peak proofs are derived above.
No allocation novelty or formal-proof claim. Next account arena metadata and
remaining source phases together before choosing more storage specialization;
output emission at production can remove final pinning but requires its own
ordering/cost proof.
