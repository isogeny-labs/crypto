# Graph kernel accounting continuation

The next implementation connects graph modular arithmetic to the existing
`jv_bitops` hooks and replaces sparse row AXPY's tree-map accumulation with a
linear merge of canonical sorted rows. The prior stage23 refusal and stage22
censor remain unchanged. This is kernel work, not a new comparative result.
[pcc:unverified]

Sources checked (KB): MCP is not loaded; repository-wrapper queries for sparse
AXPY and missing native rho prices returned no executable. Receipts are
`/tmp/ic-counted-sparse-kb-search{1,2}.log`. No novelty claim is made.
[pcc:unverified]

Sources read: `docs/units/BITOPS.md`, `src/cryptanalysis/jv_bitops.rs`,
`examples/jv_cover_ic_vs_rho_bitops.rs`, and the observed-map implementation in
`research/pencil_large_prime_20261029/src/sparse.rs`. The existing convention
counts modular add/multiply/invert primitives, data comparisons and scan steps;
it excludes branches, loop overhead, allocation and memory sizes. Sorted sparse
merge is the ordinary linear merge of two unique ordered sequences. Its
precondition holds because normalization and every row operation preserve that
representation. The former map implementation is retained as a test oracle.
[pcc:unverified]

The kernel charges one modular multiplication per consumed right coefficient,
one modular addition on equal keys, and data scans/comparisons/copies as word
operations. RHS arithmetic and source certificates use the same hooks. Negation
of a nonzero coefficient and every attempted inversion are charged. Counting
is inert when the existing global switch is off. Existing APIs and serialized
reports remain unchanged. [pcc:unverified]

This closes the modular-kernel gap, not the entire graph accounting gap.
Normalization reductions, maps/sets, sorting, graph scheduling, cloning, input
mapping and witness-replay bookkeeping still need a coverage audit. A hooked
snapshot must not be labelled a complete graph or IC total yet.
[pcc:unverified]

The rho source review found that `rho_tally` prices extension primitives using
the generic harness, while `TUNED` credits cheaper formulas in a legacy
multiply-equivalent unit. Hooks also fire inside some native rho inversions;
adding a whole hook snapshot to `rho_tally` would double-count those. Complete
comparisons must resolve that pricing explicitly, retaining the strongest
registered reference and the native-cost bands. No new rho pricing is selected
by this kernel change. [pcc:unverified]

Checks selected before implementation: compare merged rows against the former
map algorithm over three odd primes, including zero scale and cancellation;
check exact hooked modular counts and phase attribution on a tiny kernel;
retain all generic/MST rowspace and witness tests. Native targeted tests and
strict Clippy precede any separately frozen counted endpoint trial.
[pcc:unverified]

Native kernel checks subsequently pass all12 generic/MST tests, including768
map-oracle merge cases and the exact3multiply/2add/1invert/23word tiny tally.
Native library/example Clippy passes. The opt-in worker adapter and validation
analyzer emit observed primitives under `work_native` while retaining a null
complete total, explicit gaps and false comparison eligibility. The analyzer
reprices with the authoritative bitop source, checks phase sums/widths, and
rejects nonzero foreign/other hooks and attempts to promote completeness.
All22 analyzer tests pass; original21/23 summaries reproduce byte-exactly.
An inherited bitop enum naming lint is locally allowed only on that included
module; the initial lint log is retained. Actual worker execution and the
prospective Amendment19 diagnostic are separate checks. [pcc:unverified]

The executed public-input worker test subsequently passes: counted and old
rho workers return the actual scalar, with zero foreign/other hook events in
counted mode and the complete total still null. Actual production build and
17sieve/graph tests(two existing ignored) pass. All selected native checks
are complete before Amendment19's sole scientific call. [pcc:unverified]
