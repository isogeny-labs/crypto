# Adaptive normalized transport reaches quartic recovery

2026-11-05. T-1722. Native development construction; inherited prime-field,
prime-subgroup, phase-domain and conditional seed-law assumptions apply.

## Cursor invariant and transitions

After processing L>=1 offsets, the cursor holds the geometry tail after
Q+(L-1)P, the column vector for that point, the preceding column vector when
L>=2, and the x coordinates of the middle point's fixed-index multiples. The
seed column uses the middle public point's x coordinate. It also holds the
number of permitted remaining steps. Indices, base-point squares, anchors and
strides belong to the borrowed prepared source. [pcc:unverified]

begin constructs its initial window and geometry tail in the same pass.
For advance, the existing geometry tail advances by the known P/stride points.
The new normalized vector is selected in this order: public anchor, direct
initialization if L<2, otherwise the established homogeneous recurrence from
the saved two vectors and middle coordinates. New middle coordinates come from
the already converted new geometry. Rotate previous/older, increment L, and
decrement remaining. Induction gives the same represented values as the full
window construction. Anchor precedence handles zero vectors at identity;
subsequent P and2P anchors avoid using zero predecessors. [pcc:unverified]

The cursor is consumed by advance and skip_sign. It is not Clone. On error,
no advanced state is returned. A zero remaining count is checked before any
geometry advancement. begin rejects length0, checked length+max_extra overflow,
and a total beyond the constructor's max_window. Lane caps are inherited.
Prepared source and geometry lifetimes bind the cursor to its original setup.
[pcc:unverified]

## Character consumption and lazy decoding

advance evaluates the four-input factored residual at a finite new point and
calibrates its homogeneous ratio. Identity gives character0. skip_sign advances
geometry and every normalized column, but calls no residual and returns no
character or ratio. When the initial window has length1, its first non-anchor
extension can still require a direct phase/value initialization; skipping the
character does not skip that necessary state construction. [pcc:unverified]

The quartic program constructor, terms, fiber corrections, and evaluate method
are retained. Its initial14-character request uses the new normalized consumer.
The existing exceptional fibers request offset14 or15. For an offset15 request,
the source skips character14 while advancing the state, then evaluates15.
The runtime receives a public point, never a target scalar label. Known labels
and forward multiplication belong to tests and checker records. [pcc:unverified]

Recovery reports initial direct initializations and initial conversions in
addition to the existing extra steps, skipped characters and extra conversions.
Those fields separate initial and incremental work; they are not complete costs.
The implementation retains per-step vector allocations and pair copying,
geometry conversion batches, column recurrences, final monomial powers,
resultants, program construction and decoder arithmetic. Initial full grids are
dropped after consumption; the cursor keeps the projective tail and bounded
column state. The next task is whole-path arithmetic/allocation accounting.

## Validation design and evidence scope

Unit checks compare split windows with the frozen full-window consumer on
F5/r7, F7/r13 and F509/r479. Initial lengths1,2,3,7 cover the direct-initialization
boundary; starts include identity crossings. Skip every third offset and check
actual residual, phase and value-context call counters. Compare evaluated
ratios by cross-products. Check lane updates, conversion counts and exhausted
limits, including arithmetic-overflow rejection. [pcc:unverified]

The recovery check uses all20 established synthetic public targets, including
all16 exceptional-fiber inputs, compares results and routing with the frozen
batch-reciprocal implementation, and forward-checks each recovered scalar.
It retains the original4563-case research cohort without replacing it.
The separate checker imports no producer modules; it uses quadratic-residue
membership, the preserved program/rules, and affine point generation. It checks
the complete479-input polynomial identity and the20 recovery records, including
initialization/conversion accounting. Negative packets alter a recovered scalar
and an initialization count. The checker is from the same producing session,
not an independent reviewer. See receipts and logs for outcomes.

## Sources and scope

KB unavailable: relevant MCP tools are absent and scripts/research-kb status
reported no executable. Read the saved multiseed consumer/transport, the adaptive
geometry Tail::advance contract, and the quartic/batch recovery adapters and
checker. The mathematical antecedents remain those of the predecessor
DERIVATION.md files, including the previously read NIST DLMF23.10.3 addition
identity. This round changes cursor state management and source integration;
it adds no new normalization theorem or novelty-priority claim. No cloud job
or comparative timing panel ran. Local unit tests remain development work.
