# Independent evaluation of the genus-4 positive control

2026-11-16. Status: **reproduced under the frozen pricing; unqualified
complete-cost advantage against the strongest rho is not established**.
All new execution, arithmetic and measurement assertions in this report are
`[pcc:unverified]`: these are independent audit outputs, not signed PCC standing.
The original frozen evidence is preserved. No paid resources were used.

All **144 original runs** (36 each of c, v, w and negation rho) replayed
with zero mismatches in the retained shared counters. All 144 recovered
scalars passed the separate Rust arithmetic verifier.

## What the evidence supports

The reported 0.528 cost ratio is arithmetically correct for the original
model. Its 0.099 reading at weight 0.1 is also correct. The IC changes improve
the existing double-large-prime IC implementation, with a substantial
coordinate-Wiedemann component improvement even when additions are included.
However, the weight-1 comparison changes materially with operation granularity
and the rho implementation. The later producer errata already retract the
unqualified Q3 weight-1 headline. This audit supports that correction.

The targets are hyperelliptic Jacobian DLPs. These particular destination
groups cannot accept a nonzero homomorphism from an odd prime-order elliptic
subgroup over the same prime field: their prime factors are too large and
their remaining cofactors are at most 2. This is a concrete obstruction for
these destinations, not a theorem excluding other transfer constructions.

## Evidence and method

The original raw JSONL was recovered from historical Git commit
`6df35c0a4bf537cce0674f25b3279a242a5ef498`; its 78,049 bytes hash to the
recorded SHA-256
`e0f7050d3401da7db046a91bf1b0fb872dcf86023bd2c5c1caddd6c7a2efa32f`.
The file's omission from the later tree is artifact migration, not missing
historical evidence. The benchmark record's `uploaded: false` is preserved.

The replay uses Rust source commit
`2ca7ece552ad276908db4ec211c2ab452283dd58`. A focused crate assembles
hash-checked, unchanged source slices for the original genus ladder, relation
graph, peeling, both Wiedemann solvers and rho walk. It avoids unrelated root
workspace dependencies. It is **not** a bit-identical rebuild of the historical
binary. Rust 1.100.0, rand 0.8.5 and the complete new lockfile are recorded.

The recalculation is new Rust code. The separate certificate verifier uses
dynamic polynomials, extended-gcd Cantor composition and a right-to-left scalar
ladder. It calls no producer curve-arithmetic routines. Existing validator
dumps are treated as input to be checked, not trusted correctness booleans.
The checker validates divisor equations and squarefreeness, checks the returned
scalar against G and T, and checks the replay's prime-order torsion witness.

Execution receipts and compact recalculations are under `out/`; the raw archive
and its hashes are described in `EVIDENCE.md`. Run status is recorded in
`out/CHECKS.md`; no unfinished check counts as a pass.

## Recalculated original results

At p=8209 there are **three fixed curve/target pairs and four solver seeds**,
not twelve independently sampled targets. Ratios below are ratios of mean
costs. One GAE means one Cantor addition or doubling within this genus.

| Original arm w vs negation rho, p=8209 | IC mean cost | Rho mean cost | IC/rho |
|---|---:|---:|---:|
| Frozen weight 0.1: G + 0.1 M vs G + 0.1 C | 5,319,050 | 53,749,964 | 0.098959 |
| Frozen weight 1: G + M vs G + C | 51,638,679 | 97,726,831 | 0.528398 |
| Diagnostic: G + M vs rho G only | 51,638,679 | 48,863,646 | 1.056791 |
| Diagnostic: G + M + A vs G + C | 150,750,335 | 97,726,831 | 1.542569 |

M is IC relation plus LA multiplications; A is its emitted additions; C is
rho's emitted canonicalisation counter, floored at its step count. The third
row is a rho lower-bound sensitivity. The fourth row prices IC additions
without pricing all rho operations and is intentionally an asymmetric
diagnostic. **Neither diagnostic is a fair, fully calibrated cost model.**

Original mean w counters: G=172,424; relation multiplications=43,447,215;
LA multiplications=8,019,040; relation additions=39,052,493; LA additions=
60,059,163. Thus about 99.1 million additions are outside the frozen price.
Keeping rho as frozen, pricing each addition above 0.4650 of a multiplication
crosses parity at weight 1. That threshold is a sensitivity, not a measured
hardware conversion.

The original reported interval is [0.382, 0.754]. An independent
100,000-resample **paired** curve/seed bootstrap gives [0.382152, 0.748895].
The original analysis resamples numerator and denominator seeds independently
within each sampled curve; this audit's different interval is explicitly a
different estimator and RNG, not an exact replication claim. Two of the
twelve paired weight-1 runs lose to rho; none loses under the frozen weight
0.1 reading. Three curves do not establish population-wide uncertainty.

## Comparison with index-calculus controls

The controls are c (double large prime plus parity filter, original LA),
v (coordinate Wiedemann), and w (v plus the unit-T walk). Same curves,
targets, factor-base policy, restart, trial caps and solver seeds.

| p | w/c LA multiplications | w/c LA mul+add | w/c total at frozen weight 1 | w/c total with IC additions, weight 1 |
|---:|---:|---:|---:|---:|
| 2053 | 0.221801 | 0.309931 | 0.698127 | 0.563591 |
| 4099 | 0.229473 | 0.318092 | 0.676219 | 0.534459 |
| 8209 | 0.233120 | 0.317497 | 0.660984 | 0.507077 |

These are improvements over the named internal IC control in their stated
count models. Coordinate Wiedemann retains its gain with additions included.
The incremental unit-T LA gain is smaller than the multiplication-only count
suggests: w/v is 0.704902 in multiplications but 0.933476 in mul+add. Unit
coefficients take addition-only branches, moving work out of M. The restart
cap is inactive at every registered instance; it cannot be credited for this
experiment's gain.

This comparison is **not a demonstrated IC frontier result**. Double-large-
prime IC is an established algorithm; Diem's primary theorem gives expected
soft-O(q^(2-2/g)) for fixed genus and the stated group hypotheses, so the
genus-4 exponent is 3/2 rather than rho's 2. Strong relevant controls include
factor-base/large-prime tuning, sieving-style relation generation, structured
elimination plus coordinate or block Wiedemann, and faster genus-4 group laws.
The current PR's session log contains newer sieve/SGE/fast-group-law smoke
results; those are not a frozen matched end-to-end panel and are not promoted
here. The old c arm is therefore a historical baseline, not certified as the
fastest current IC implementation.

External comparison: **not run**. No Magma, FLINT/NTL solver executable or
licensed Jacobian-DLP comparator is installed here. Magma documents Diem-style
plane-curve IC, but a same-input genus-4 comparison requires a supported curve
model, divisor export, the algorithm actually selected and complete cold
costs. Finite-field multiplicative DLP timings, genus-3 quartic timings and
isolated matrix kernels are not substitutes. No comparable external genus-4
timing was established by the literature search, so no speed ratio is invented.

## Rho strength and the later follow-up

Keep the follow-up separate: it has nine targets (three per curve), two solver
seeds, and emitted rho window counters. Rust recalculation reproduces its
point estimates: Q1 weight 1=0.468526; Q3=0.141821; Q4=0.123273.
These are **not replacements** for the original 12-row 0.528398 cell.

The 16-key short-cycle scan dominates the registered rho's Q3 native cost.
Plain rho does no negation canonicalisation, but the wrapper still floors
its C counter at one per step. Removing an unused cycle window is a relevant
stronger control, not an algorithmic IC advance. The independent Rust check
with `cycle_window=0` is recorded beside the original counter comparison in
`out/lean-rho-*.jsonl` and `out/CHECKS.md`.

| Follow-up, weight 1 | IC/reference ratio | Scope |
|---|---:|---|
| Q3, registered negation rho | 0.141821 | Includes its per-step window scan |
| Q3 sensitivity, plain rho without window, C still floored | 1.048890 | Unpriced hashing/coefficient work remains |
| Same sensitivity with actual C=0 for plain rho | 2.097645 | Still not a complete calibrated rho price |

The last two rows deliberately expose reference sensitivity; they do not
claim every other native operation is free. The audit's paired target/seed
bootstrap gives [0.775419, 1.419270] for 1.048890 and
[1.550740, 2.838354] for 2.097645. The later frozen validator independently
reported the same point estimates and slightly different intervals. A fair
complete model must also include rho hashing, coefficient updates, keys,
table work and IC's corresponding word operations, using consistent
granularity and hardware calibration.

The producer's calibration also recalculates: IC/rho is 0.007687 at its
recorded rates and 0.048648 at ten times those rates (rho priced by G).
This supports the direction of a physical-cost advantage on that producer
host, but is not a new timing or complete calibrated price: mod-r additions
were not calibrated, the LA uses the mod-p addition rate, and rho overhead
was outside the isolated Cantor primitive. See `out/calibrated-recalculation.json`.

At weight 0.1 the original and follow-up wins survive every examined
sensitivity. At weight 1 the original frozen-model win is reproducible, but
an unqualified fastest-rho, complete-cost win is not established. Choosing
equal price for a scalar compare, a vector compare, a residue multiplication
and a canonicalisation macro is a convention, not a timing calibration.

## Verification, omissions, memory and failures

The IC collector includes factor-base setup, failed smoothness tests, relation
collection, graph combination, peeling, Wiedemann, scalar recovery and one
counted final scalar check. All completed replay rows retain their failures
and mismatches. The original record contains no failed or exhausted run;
this bounded observation is not a zero failure-probability theorem.

The second scalar check (`mul_rtl`) is executed but its field tally is reset
on IC; rho's additional wrapper check also lies outside its counted totals.
Full per-relation checking was disabled in the producer benchmark. This audit
separately enabled it on one complete p=8209 w run, with unchanged shared
counters except the intentionally zeroed Cantor tallies. It then exported and
independently checked **all 3,927 accepted relations**, including every place,
the split divisor sum, and the ca*G+cb*T invariant. No relation failed.

The separate naive audit verifier used 287,040,111 field multiplications,
5,375,071 field inversions and 186,693,029 field additions. Its slow full
certificate checking is **not** a production solver stage. It is not added
to IC alone or hidden inside a claimed solver speedup. Requiring that audit
again for every production solve would define a different workload.

Still unpriced or inconsistently granular: IC coefficient tests/index loads
in LA, vector allocation/sorting, factor-base and large-prime hash lookups;
rho keys/mixes, coefficient updates, negations/selects, distinguished-point
table operations and collision inversion. `la_add` also includes bookkeeping
such as DSU operations. It is not purely a count of arithmetic additions.
Curve search, admission and order computation are outside the solver totals;
target construction is properly kept outside both unknown-target intervals.
No ECDLP transfer, route construction, map evaluation or recovery appears.

The original record has no peak-RSS or phase memory measurements. On the
representative instrumented w replay, whole-process VmHWM was 3,232 KiB;
the separate independent relation verifier used 18,216 KiB because it loaded
the transcript. These are descriptive checks on the local host, not memory
complexity claims or matched resource-envelope evidence. At p=8209 the mean
core is about 1,706 rows by 1,667 columns; sparse storage, graph and factor
base must remain in future resource accounting. Rho's distinguished-point
storage must be measured too. No isolated wall-clock speedup is claimed.

## Transfer analysis

Let the destination have #J(F_p)=h*r, with h in {1,2} and r prime. The Rust
checker independently tests r's primality and computes rational Hasse-Weil
bounds containing exactly one multiple of r, namely h*r. Independent replay
certificates supply a nonzero G with [r]G=0. Thus the order calculation does
not rely solely on a producer's `order` field.

An odd prime-order source subgroup H in E(F_p) has order l <=(sqrt(p)+1)^2.
For every benchmark instance r exceeds that upper bound, and l does not
divide h. Hence gcd(l,#J(F_p))=1. The image of a group homomorphism from H
has order dividing both orders, so its image is trivial. At p=8209 a safe
integer elliptic-order upper bound is 8,464, while r is at least
2,262,066,593,400,287. The obstruction also excludes the intended ordinary
prime-order 2^31-1-field baseline from these finite destination groups by
the same order-divisibility argument, even without a same-characteristic
assumption. The written argument is standard finite-group reasoning, not a
new Lean-kernel-checked theorem.

A cover C->E over F_p can instead give a pullback into a Jacobian with an
elliptic factor; when its degree is coprime to the source subgroup order,
pushforward after pullback acts by that degree. Such a destination must have
the source order as a divisor and will not be one of these h<=2 groups. Its
large cofactor, subgroup distribution, relation yield and full solver costs
need new evidence. Applying the generic genus-4 q^(3/2) IC estimate over the
same prime field to an elliptic source of order about q does not beat the
source's q^(1/2) rho bound.

For any alternative scalar-independent route, require explicit algorithms
for public P and Q, subgroup injectivity and recovery, and compare

    route setup + map(P,Q) + destination solve + recover + verify

against the best relevant **source IC pipeline and optimized rho**, in one
calibrated unit and resource envelope. Cross-characteristic abstract cyclic
group isomorphism is not an executable cheap map; differing prime fields
provide no field embedding. This audit supplies no such map. Tian's cubic
extension-field construction uses Weil restriction and does not supply a
prime-field instance of that bridge.

## Remaining acceptance gaps and next checks

| Requirement | Status and precise gap |
|---|---|
| Original counts and recovered scalars | See final replay/certificate receipts in out/CHECKS.md |
| Complete accepted relation collection | Independently checked for one full p=8209 run; not every collection |
| LA counts | Original routines replayed; no independent transcript of every matrix operation |
| Fair complete-cost weight-1 win | Not established; consistent native granularity and optimized rho needed |
| IC frontier superiority | Not established; newer internal and external end-to-end controls missing |
| Memory/failure generalization | Representative RSS and bounded successes only; no scaling/tail study |
| Prime-field ECDLP transfer | These destinations ruled out for the stated source subgroups; alternative route absent |
| Formal promotion | No signed PCC verdict or VALIDATION.json; no leaderboard promotion |

Next discriminating check: a frozen Rust panel pairing the best sieve/SGE/
Wiedemann IC arm with window-free plain rho and a properly tested periodic-
check negation rho, using symmetric primitive counters, measured mod-p/mod-r
rates, phase RSS and independent targets. Compare fast genus-4 group laws on
both arms. The present local replay spends no money; any paid extension
requires a separate exact configuration, finite proposed cost and explicit
approval before launch.

## Sources checked

- [PR #4409](https://github.com/isogeny-labs/crypto/pull/4409).
- [Frozen record](https://github.com/isogeny-labs/crypto/blob/1a3f5a3d464b60bb0d3ce69459d3b47e9118d06f/benchmarks/records/20261116T1540Z-genus4-positive-control.json).
- [Frozen protocol](https://github.com/isogeny-labs/crypto/blob/1a3f5a3d464b60bb0d3ce69459d3b47e9118d06f/research/genus4_positive_control_20261116/PROTOCOL.md).
- [Historical raw data](https://github.com/isogeny-labs/crypto/blob/6df35c0a4bf537cce0674f25b3279a242a5ef498/research/genus4_positive_control_20261116/run.jsonl).
- [Genus-4 red-team report](https://github.com/isogeny-labs/crypto/blob/2193ace0fdfaebc24e7366a8b95c3408af04674f/research/genus4_positive_control_20261116/redteam/REDTEAM.md).
- [Later validator and B13--B18](https://github.com/isogeny-labs/crypto/blob/2193ace0fdfaebc24e7366a8b95c3408af04674f/research/genus4_rho_refs_20261116/validation/validator-report.md).
- [Later errata](https://github.com/isogeny-labs/crypto/blob/2193ace0fdfaebc24e7366a8b95c3408af04674f/research/genus4_rho_refs_20261116/ERRATA.md).
- [Diem, double-large-prime IC](https://arxiv.org/pdf/math/0606607), theorem, algorithm and storage assumptions.
- [Tian, cubic-extension cover attacks](https://arxiv.org/pdf/2012.07173), introduction and transfer construction.
- [Magma plane-curve IC documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1473), algorithm/interface constraints; not executed.

KB unavailable: no loaded MCP tool and the repository wrapper exits 127.
Used the listed primary repository evidence and primary papers instead.
Canonical dashboards remain unchanged because this audit does not produce a
signed, independently promoted speedup or a new validated frontier row.
