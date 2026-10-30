# Symmetric PKM G1–G6: native correctness and accounting prerequisites

2026-10-30. User goal: determine whether symmetric PKM can produce useful
elliptic-curve relations cheaply enough to improve a complete cold,
single-target discrete-logarithm computation. All six stages of
[PKM_GOALS.md](../PKM_GOALS.md) remain in scope. This deliverable is native
software/proof development. No measured panel, cost improvement or asymptotic
result is supplied.

## Requirement status

| Goal | This deliverable | Remaining acceptance requirement |
|---|---|---|
| G1 complete supported-point correctness | Complete-set native collector and separate exhaustive affine checker; fixed development fixtures | Prospectively frozen multi-curve panel and verified supported-domain coverage; keep misses/caps distinct |
| G2 complete cost/yield | Exclusive diagnostic phase ledger, emitted native counters, failed/capped outcomes and cold toy rank/LA path | Complete count coverage, all residual work priced under the required band, isolated admitted clocks and reproducible process-failure journals |
| G3 presentation/external ablation | Chain, plain full summation polynomial and symmetric adapters; canonical JSON and msolve input export | Equal-domain m=4 panel, output-verified pinned fastest external arm, A/A and null controls |
| G4 one improvement | No candidate promoted; retain proposed 20% complete-cost or 2x-memory objective | Frozen equal-output baseline, one justified candidate and held-out paired comparison |
| G5 scaling/reach | No depth or field-size panel launched | Predeclared completed/censored cells, multiple fields and held-out validation within the approved cap |
| G6 cold DLP and proof | Bounded native cold-DLP correctness control; one new abstract chain-order Lean dependency | Certified larger-family pipeline, full accounting, independent replay/rank/yield, matched IC/rho/BSGS, and the central ideal/curve/cost proof bridges |

Scientific execution is blocked by the unchanged checkpoint gate, missing
artifact/lab access and an unqualified native-command image/execution route.
The environment status reports no configured identities, secrets or runtime
variables. The maintained GitHub Isolab workflow currently accepts an ecbench
spec; this standalone example is not an ecbench implementation or a deployed
fixture. No credentials, IAM, pool, quota, funding or lease was changed. The
user authorized the G1–G6 goal; those capability gaps are not a request for the
user to repeat that authorization. See the specific
[problem record](../../../problems/symmetric-pkm-complete-cost.md).

## Native interface and supported domain

`examples/pkm_oracle_program.rs` is additive. The historical
`pkm_symmetric_complete.rs`, its constructors and prior receipt directories
are preserved. The new `algebra.rs` derives its construction helpers from
PR2868 dc7f22a9 and PR2650 adde64fdc, adding thread-local helper-arithmetic
counters. `reference.rs` separately implements ordinary affine arithmetic,
field scanning and distinct-abscissa combination enumeration; it does not call
F4, the constructor, the Montgomery curve code or the factoring API.

The oracle accepts a finite canonical public point, prime 11<p<2^32,
non-singular short-Weierstrass curve, m=2..4, depth1..8, nonzero coset and an
exact-order root of unity. Output is the complete canonical set of signed
point decompositions with distinct abscissae when `complete=true`. Repeated
abscissae and the identity target remain outside the interface. All signed
lifts are checked against the original curve equation and point sum by the
separate affine implementation. No target scalar enters the interface.

`arm` is `symmetric_s5`, `plain_s5` or `plain_chain`; the full-polynomial arms
use S_(m+1), which is S5 specifically at m=4. Plain arms include all summand
orderings. The complete-set collector canonicalizes/deduplicates witnesses,
retains duplicate hits, and never hides the visitor's full-field branching.
Factoring observes the shared visitor deadline. A witness/assignment limit,
timeout or factoring interruption remains incomplete, including when some
valid witnesses were found. A zero solver budget returns unknown before
construction. Construction and native matrix allocation still require an
external whole-process deadline and memory cap; the visitor limit is not a
process cap. The instrument has no supported traversal resume/journal yet.

The separate exhaustive checker is bounded to p<=257 and at most one million
signed tuples. It verifies every returned witness and detects lost outputs
when completeness is claimed. It never supplies answers to the candidate.
Incomplete outputs remain incomplete even if a reference can classify them.
Replay binds the exact input, checks completion/status consistency and phase
conservation, and rejects an unsupported cost-admission flag.

## Cold-DLP correctness control

`toy-dlp` accepts only p<=257 and an independently enumerated prime full-curve
order. This restriction makes subgroup membership checkable without assuming
that n-torsion equals the group generated by P. Its inputs are E,P,Q,n,tower,
seed and caps; it accepts no target scalar or pre-labelled factor-base logs.
It constructs the base, computes public aP+bQ, asks the native oracle for
relations, forms sum(e_j log B_j)-b*k=a, performs counted scalar-field dense
elimination, and independently verifies the recovered scalar equation.
Identity query attempts are charged and retained, with no scalar shortcut.

The record retains queries, all original relation rows, useful-row counts,
rank progression, final RREF, scalar-field operations, setup, verification and
cold process-core intervals. Replay independently checks query, original-row
and scalar equations using affine arithmetic, and recomputes the matrix/rank
with the producer's elimination implementation. It is not an independently
implemented rank checker or cost admission. Unknown/capped queries stay in the
cold interval; native errors/crashes need external process logs, and durable
partial-run journaling remains missing. This is a tiny correctness control,
not the qualified growing-family G6 computation.

## Cost coverage and comparisons

`comparison_admissible` is always false. Constructor helper arithmetic,
visitor multiplication/branch diagnostics, root multiplications, sign/addition
calls, verifier arithmetic and scalar-field LA are emitted with their scopes.
F4 symbolic/substitution work, root additions/inversions, native Montgomery
arithmetic, allocations, hashing and logical bookkeeping still lack complete
counts. These gaps are explicit `unpriced_work`, not silently assigned zero.
No finite complete GAE or speed ratio is produced. Helper inverse/sqrt call
counts are composite diagnostics: their helper multiplications are already
included in the helper multiplication count. Scalar-field LA uses modulus n,
not p. Nested group/field ledgers and these composite calls must not be summed
as disjoint costs without a qualified conversion/accounting rule.

Oracle phase intervals are exclusive: validation/setup; construction and
canonical system hashing; visitor time excluding callbacks; root decode;
lifting; sign search; independent witness verification; and unattributed
process-core overhead. They sum exactly to `total_ns`. The cold toy record
separately reconciles setup, whole query intervals, final scalar verification
and unattributed overhead. Scalar-elimination time is a subset of the query
intervals, not an extra charge. Input loading, process startup, report binding
and serialization are outside these diagnostic process-core clocks and must
be declared by a future measured protocol. Whole-process lifetime high-water
RSS includes previous queries; it is not an isolated per-query peak.

The old first-witness stage driver is not an equal-output cost baseline for
this all-witness instrument. G4 must freeze a complete, verified equal-output
baseline or explicitly define first-witness output for both arms. An msolve
input export does not qualify external rational-root extraction or establish
same-host/external superiority. No external solver was run here.

## Lean scope

The scoped project now audits13 declarations, zero trusted exceptions. The
versioned affine proof receipts build and kernel-replay the final module.

| Declaration in ThreeLeads/PKMChainCoverage.lean | Exact checked scope |
|---|---|
| finite_chain_order | Additive commutative group; a+b!=0 and c!=d imply one of the two last-point orders has finite intermediate sums and the same final sum. |
| affine_pair_nonzero | Mathlib nonsingular affine Weierstrass points over a field with decidable equality; unequal x-coordinates imply their sum is not O. |
| affine_finite_chain_order | Instantiates the chain-order statement on four actual Mathlib affine points, under nonsingularity and x0!=x1,x2!=x3. Pairwise distinct abscissae imply these conditions. |

The coordinate-to-group implication is checked for Mathlib's point type.
The bridge from Rust's representation and the precise polynomial system to
those objects, and all solving/extraction-cost bounds, remain unformalized.
These are standard structural facts, with no global novelty or complexity
claim. The earlier11-declaration software receipt remains as its original
version; the fresh13-declaration receipt validates the stronger module. An
initial scratch attempt lacked the explicit decidable-equality instance; its
failed source/log are retained as plaintext and are not reported as a proof.

## Reproduction and execution gate

Build and direct unit tests are development validation:

```sh
cargo test --release --locked --example pkm_oracle_program -- --test-threads=1
cargo clippy --release --locked --example pkm_oracle_program --no-deps -- -D warnings
tools/lean/proof-gate.sh research/prime_ecdlp_three_leads_20261029/lean
```

After a frozen protocol, tested source/image, supported runner and unchanged
checkpoint gate qualify, the command interfaces are:

```sh
target/release/examples/pkm_oracle_program run fixtures/m4-public-point.json
target/release/examples/pkm_oracle_program verify fixtures/m4-public-point.json OUTPUT.json
target/release/examples/pkm_oracle_program export-msolve fixtures/m4-public-point.json
target/release/examples/pkm_oracle_program toy-dlp fixtures/cold-toy-dlp.json
target/release/examples/pkm_oracle_program verify-toy-dlp fixtures/cold-toy-dlp.json OUTPUT.json
```

Paths above are relative to this study directory; invoke the binary/checkpoint
script from the repository root with the corresponding full request path.
Every production/export/replay command enforces the unchanged checkpoint gate.
These fixture JSON files are public development inputs, not a frozen
experiment.protocol/v1 or permission to bypass any execution gate.

## Sources and unchanged canonical graphs

Checked PKM2016 sections1,3.1–3.2,5 (low-degree-map framework, permutation
symmetry, cost model and open complexity), the retained lead-atlas
LEAD-ICP-pkm-tower-algebraic-oracle, the tower note sections2.1–2.2/3.3–3.7,
PR2632/2650/2868, the mixed-fiber full-solver problem, the current F4 visitor
and deadline-aware root API, and `examples/gb_cas_export.rs` for msolve syntax.
Primary source: https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf .
No new global novelty or asymptotic claim follows. The separate mixed-fiber
negative results are not attributed to this symmetric presentation.

The IC scoreboard, progress timeline, leaderboard and canonical curve graphs
are unchanged: no admitted performance value, curve or isogeny edge was added.
The diagram and inspected PDF explain the implementation and open bridges.
Validation status and exact source/log hashes are in `validation/receipt.json`.

## Final development validation and remaining software limit

The final12-test native suite passes, including full m=4 witness-set agreement
on all35finite points of the two F17 development fixtures. The strengthened
three-presentation two-sum control extracts a nonempty witness set in each arm.
All8legacy regression tests, strict lint, rustfmt, ordinary binary build and
fresh native backlog validation pass. The compiled CLI fails closed at the
unchanged checkpoint gate before reading a controlled nonexistent request.
Fresh final Lean audit passes13declarations/zero trusted exceptions, and the
stronger affine module builds and kernel-replays. Both final PDF pages were
visually inspected.

The broader serial5553-case library run was capped at40minutes:2173passed,
0failed,47ignored,one unchanged JV case active at the cap and3332cases not
reached. The timeout wrapper exits124. No full-library pass is claimed. See
[the named validation problem](../../../problems/release-suite-jv-isogeny-walk-budget.md).
Its raw output exceeds the artifact policy threshold and is NOT_UPLOADED; exact
size/hash/path and a bounded excerpt are retained. This is separate from the
scientific access/accounting/proof gaps and supplies no PKM performance result.
