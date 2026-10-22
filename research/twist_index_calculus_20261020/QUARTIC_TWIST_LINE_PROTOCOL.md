# Quartic twist Frobenius plus cubic automorphism: frozen probe

Frozen 22 October 2026 before implementing or running the native probe.
This is a new hypothesis prompted by reconciling the prior GLS study with
`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` E18. It is not an E18 cost result.

## Hypothesis and exact construction

For primes `p ≡ 1 mod 12`, let `d` be the least quadratic nonsquare in
`Fp`, let `F=Fp[t]/(t⁴-d)`, and let `E0/Fp: y²=x³+b0`.
The element `t` is nonsquare in `F`; set `E/F: y²=x³+b0 t³`.
Write `h=t^((1-p)/2)`, `z=t^(p-1)∈Fp` (order four), and
`ω∈Fp` of order three. On `E(F)` test

* `ψ(x,y)=(h² x^p,h³ y^p)`, the transported `p`-Frobenius;
* `ι(x,y)=(ωx,y)`, the ordinary `j=0` automorphism.

The predicted identities are `ψ⁴=[-1]`, `ι³=1`, and `ψι=ιψ`.
For `x∈Fp`, `ψ_x(x)=z⁻¹x∈Fp`, so the same line base used in E18 is
closed under both maps. If the full group has prime order `r`, their
eigenvalues on `G` should generate order 24, yielding 12 abscissae per
column versus E18's 3. This statement needs explicit point, scalar,
base and cost checks; the predicted asymptotic linear-algebra discount
is not a measured pipeline gain.

## Frozen candidate panel and controls

Use exactly the pairs `(p,b0)=(13,4),(37,2),(73,5),(97,5),
(157,25),(229,6),(241,49),(337,100)`, selected by an untimed
arithmetic screen for prime order. Recount `#E0(Fp)` by exhaustive
Legendre character sum and derive `#E(Fp⁴)` from the exact trace
recurrence and quadratic twist sign. Use deterministic 64-bit primality
checks. Verify `[r]P=O` for the generator and sample points. For `p=13`,
also independently enumerate all `Fp⁴` abscissae and square characters
to check the order without the recurrence. Retain any failed candidate.

For each curve enumerate all `x∈Fp` with liftable points on `E` and all
corresponding `±P`. Check curve membership, both maps, relations,
commutation, scalar eigenvalues and order on every base point. Partition
the physical base under `⟨-1,ι⟩` and `⟨ψ,ι⟩`; record orbit-size
histograms, numbers of unknown columns, and any fixed-point exceptions.
The same exact point set must feed both arms. An independent verifier
must reconstruct the field and group arithmetic and replay these claims.

## Next decision and claim boundary

If the 24-action and line closure verify, extend the existing E18
quartic relation stream and matched rho to this exact panel. Hold the
physical base, relation oracle, target, seed and stopping rule fixed;
measure full-rank rows, linear algebra, target descent, cold total,
and a rho walk folded by all 24 elements, with independent scalar replay.
The same curve identity and isomorphism must be documented before
comparing to historical E18 rows. If the action or closure fails, retain
the counterexample and do not benchmark a fictitious fold. This probe
alone cannot establish a new IC speedup, a lower crossover, or an attack
on a deployed curve.

## Registered follow-up after the structural probe passed

The native probe and independent Python verifier passed on the seven
nonempty-line candidates; `p=13` remains an empty-base control. Before
building the relation-pipeline extension, freeze the full-run panel:
`p∈{37,73,97,157,229,241,337}`, the exact `b0` values above, one
known-answer target `[271]G` per curve, and three independent public
seeds `0,1,2`. For each run, the **same** list of one signed point per
liftable `x∈Fp`, the same pair table, the same residual stream and the
same one-verified-decomposition-per-target choice feed three arms:
flat abscissa columns, cubic-automorphism columns, and combined
twist-Frobenius/cubic columns. Continue each arm until its target log is
independently verified after a full-rank square-core solve, or until
50,000,000 residuals have been tried. Keep failed, censored and
singular attempts. All group additions, field multiplications, row
operations, factor-base construction, table construction, collection,
linear algebra, online recovery and verification must be accounted for
before an end-to-end cost claim.

The matched rho controls use negation, the cubic automorphism and the
full 24-element action on that same target, with 16 independent walks
per curve and seed, each with its own table. The existing E18 r-adding
walk's look-ahead and fruitless-cycle doubling rules remain; charge
endomorphism canonicalisation. Report ratios per curve and growth with
size. A pilot on the three smallest positive curves may precede the
full panel but cannot replace it. If any arm or reference fails, retain
its raw record and withhold success-only speed ratios.

## Separate invariant-line sweep, registered before execution

The `p=13` empty line and the varying densities in the seven positive
rows show that `x∈Fp` need not be the best invariant factor base. The
`p`-Frobenius eigenspaces in `F_{p⁴}` are exactly the four lines
`x∈t^j Fp`, `j=0,1,2,3`; twist Frobenius acts on their abscissae by
`z^{j-1}` and the cubic automorphism by `ω`. For each of the eight
frozen `(p,b0)` pairs, enumerate every nonzero abscissa on all four
lines, test exact liftability, check both endomorphism images on every
lifted point, and report the line sizes and orbit histograms.
Use the same quadratic-character test for all lines and retain zero
lines. The maximum-size line per curve is a post hoc candidate; its
relation yield and complete cost require a separately frozen paired
run. Do not substitute it into the already registered `x∈Fp` panel.

## Post-run v2 accounting and mutation-control amendment

The 21-run v1 panel was committed at `ddd469ee74`; all three IC arms
and all 1,008 rho walks recovered their public known-answer targets,
with 1,821 selected relations independently replayed. The first cost
report correctly counts base construction, orbit maps, pair table,
stream, linear algebra and target verification, but it excludes the
construction of the canonical public model, generator and eigenvalues.
It must not be called a complete cold cost.

Before changing the producer, freeze v2 on the **same** seven curves,
three seeds, one target per run, 16 rho walks per fold, 64-wide batches,
same jump rules, relation oracle, row selection and stopping cap. Add
exclusive counters for common model construction, input/generator
preparation, cubic eigenvalue identification, and twist-Frobenius
geometry/eigenvalue identification. Charge the relevant eigenvalue
work to each IC and rho arm, report common input work separately,
and preserve the earlier post-input count for direct v1 comparison.
Check every factor's asserted scalar coefficient during orbit-map
construction, and inject one incorrect Frobenius eigenvalue per run:
the validator must reject it before the timed relation stream. Retain
the check and outcome in each raw record. Re-run the full 21-instance
panel once as an A/A control; compare all non-timing fields exactly.
Any divergence is an instrument failure. Derive summary CSV and plots
from raw records, and independently replay the v2 relations, ranks,
scalars and controls. Public model and known-answer target creation
remain outside the one-target online interval, but their counted cost
is explicit. No sealed or isolated whole-method speed claim follows
from this L0 experiment.

## Separately registered mixed-line relation-yield experiment

Frozen after the four-line character sweep, before implementing the mixed
producer. For the seven positive `(p,b0)` pairs and seeds `0,1,2`, compare
two bases on the *same* canonical twist, generator and target `[271]G`:
`L0={x∈Fp}` and `L02={x∈Fp}∪{x∈t²Fp}`. Both use exactly one signed lift per
nonzero abscissa, with identical L0 lifts and their fixed signs across arms.
The four-factor signed meet-in-the-middle oracle, 64-wide target batches,
50,000,000 residual cap, flat/cubic/twist24 row encodings, square-core rank
and scalar verification are held fixed. The target `(a,b)` stream comes
from a dedicated RNG seeded by `(p,seed)` and is identical for L0 and L02;
any decomposition choice uses a separate RNG. Charge model, eigenvalue,
base, map, pair-table, stream, row/rank, and verification costs. Record
every target attempted, every decomposition, unique selected trace,
matrix ranks, per-arm stop, and full post-input arithmetic. Reuse the v2
same-target 24-fold rho reference, which is independent of factor base;
do not make a wall-time speed claim from these L0 host runs.

For each base, independently replay liftability, closure, the 24 action,
all emitted relations, encoded rows, rank and recovered scalar. Require
the L0 target stream to be an exact prefix of L02's stream or vice versa,
and require every L0 success at a shared target to be present as a valid
L02 decomposition. Report yield per *same attempted target*, together
with incremental table size and work; raw yield by itself is not the
decision criterion. Preserve failures and censored rows. A full rerun
must match every non-timing field exactly.

## Registered native S5 polynomial-oracle pilot

Frozen after the exact 42-run mixed-line experiment and before building
the pilot. The existing native `gaudry_quartic::SymmetrisedS5` and
`solve_s5_subspace` implement the Weil-restricted degree-five summation
polynomial for four `x∈Fp` factors. On the canonical quartic twist, test
exactly `p=37,73,97`, `seed=0`, using the *first successful* and *first
unsuccessful* target `(a,b)` in each recorded L0 query trace from
`quartic_twist_mixed_v1.ndjson`. Construct the same curve, generator,
target and signed base as the recorded run. Count S5 symmetrisation
precomputation once per curve and each solver phase separately. Compare
all resulting abscissa quadruples to an exact group meet-in-the-middle
oracle on the identical target, and verify each candidate by group
addition. Retain extraneous, missing, censored and timeout outcomes.
Use a 1,800-second process watchdog per curve; a smaller pilot is not a
substitute for the already completed exact relation-yield panel. This
pilot tests whether the repository's existing algebraic oracle executes
correctly on the canonical twist and measures its initial cost. It does
not test the line-0/line-2 union polynomial system, run a full IC solve
with S5, compare with msolve/Magma, or justify a large-size speed claim.

## Registered two-dimensional invariant-space census

Frozen before implementing this separate census. For all eight registered
`(p,b0)` curves, enumerate every `x=a+b t²` with `a,b∈Fp`, except `x=0`,
on the same canonical twist. Test exact liftability using the native field
quadratic-character routine. Retain the separate counts with `b=0`, `a=0`,
and `ab≠0`, including zeros. Check that the two axis counts reproduce the
earlier line sweep. On every lifted abscissa, apply the exact transported
Frobenius and cubic abscissa maps, assert that the images remain lifted,
and compute the full generated abscissa-orbit histogram. Check its sum and
the expected 12-orbit claim rather than assuming it. Report the physical
base size, folded columns, and the number `B(B−1)` of signed pair-table
entries the existing exact four-factor oracle would generate, before hash
collisions. These are structural and work-size data, **not** a relation
yield experiment or a measured memory footprint. Preserve the `p=13`
one-dimensional empty-base control and state whether the larger space
revives it. Any target-query experiment using this base requires a
separately frozen budget and complete work accounting.

Audit the census with the independent quadratic-subfield norm criterion:
for each nonzero `x=a+b t²`, `x³+b₀t³` is a square in `Fp⁴` exactly when
its norm to `Fp²` is a square in `Fp²`. Recompute every count. For one
point per abscissa orbit, obtain a native square root, verify the curve
equation, `ψ⁴(P)=−P`, commutation with `ι`, and the generated signed
point-orbit size 24. Retain a separate machine-readable audit record.

## Registered p=13 two-dimensional relation-and-rank pilot

Frozen after the two-dimensional census and its norm/point audit, before
building the pilot. On the previously empty-line `p=13,b0=4` curve only,
use all 72 liftable nonzero abscissae in `Fp+t²Fp`, one deterministic
signed lift per abscissa, and the first liftable abscissa in lexical
`(a,b)` order as generator. Verify the published prime group order
`r=28,753`, set the sole target to `[271]G`, and use seed zero. Construct
the 24-fold orbit coefficient map and assert every point-to-scalar map.
Use the same fully signed four-factor pair-table oracle and independent
target RNG formula as the mixed-line panel, one selected relation per
successful target, in 64-target batches. Stop at full rank for the six
factor logs plus target log, or after 10,000 processed targets. Re-add
every selected relation, retain every attempted target and decomposition
count, and verify the recovered target scalar and all solved factor logs
by group multiplication. Record setup, base, orbit-map, pair-table,
relation-stream base-field multiplication and group-operation counts;
record modular row operations separately. Preserve a full A/A rerun and
compare all non-timing fields. This is a bounded, exact group-oracle IC
pilot for an additional invariant base, not an S5 polynomial experiment
or an isolated whole-method/rho speed comparison.
Independently replay all attempted target hit counts with a fully signed
pair-sum hash whose ordering differs from the producer, reconstruct each
selected row and its rank, and verify the recovered target by group
multiplication. The audit must pass before interpreting the pilot.

## Registered p=13 twist-quotient pair-table oracle

Frozen after the p=13 2D pilot and independent signed-pair audit, before
building the quotient. Use its exact base signs and seven recorded residual
targets, with no new target sampling. On the same `p=13,b0=4` curve, let
the 24 point actions act on signed two-factor entries modulo global sign;
the induced abscissa action has 12 elements. Enumerate all `B(B−1)`
standard pair entries, classify their exact action orbits, retain one
representative per orbit, and key it by the canonical abscissa of its
pair sum. Preserve all stabilizers and hash-key collisions. For each
recorded residual, run both the existing full signed-pair oracle and a
quotient lookup that reconstructs pair entries from representative and
action tag on demand. Require exact equality of the **entire signed
four-factor decomposition set** on every target, not just the hit count;
require all seven previously selected relations to occur in both.
Record full and quotient setup and seven-query `Fp` multiplication and
group-operation counts separately, plus full entries, quotient orbit
representatives, histogram and canonical hash-key count. Re-run once
and require every non-timing field to agree. Hash probes, allocator
bytes and whole-method performance are outside this bounded structural
and oracle test; it makes no rho or polynomial-system speed claim.

## Registered action-tagged pair index

Frozen after the direct canonical quotient showed identical relation sets
but high per-query canonicalization cost, before building the tagged index.
Compare the repository's full signed-pair hash table with a new index that
stores one pair tuple per orbit and indexes every transformed pair-sum
abscissa by a compact `(orbit, action, y-sign)` posting. Use the same
point action, deterministic factor signs and four-distinct-factor rule.
The tagged lookup must not compute a canonical orbit key per query: it
looks up the required sum's actual abscissa, checks its stored y sign,
and reconstructs the pair terms from the action tag. Use the coordinate
Frobenius with its scalar/monomial multipliers, not a generic `Fp⁴`
multiplication. Audit the action maps and all reconstructed pair sums.

Run two frozen workloads: (1) all seven residuals from the prior p=13
two-dimensional seed-zero pilot, using its exact recorded base and target;
(2) the **first three** seed-zero `p=37,b0=2` residuals from the
single-line record in `quartic_twist_mixed_v1.ndjson`, on the full
two-dimensional `Fp+t²Fp` base. For (2), retain the signed line-0 and
line-2 points from the paired union record, then lift every interior
`a+b t²` abscissa in lexical order using `StdRng` seed
`0x5441_4747_0000_0037`. Verify the resulting base size against the
registered census (624) and verify curve/order/action closure.

For each workload, require exact equality of the **entire signed
four-factor decomposition sets** from full and tagged oracles on every
target. Preserve every failure, mismatch and timeout. Count each arm's
table setup and target-query `Fp` multiplications and group operations,
the number of retained pair tuples, postings and abscissa keys, and
lookup comparisons. Instrument allocator-requested live and peak bytes
for each table's retained structure, including tagged action maps, orbit
tuples and index; report them as requested-byte counts, not RSS. Keep
common base/model/target work separate. Run full and tagged arms in one
process with identical inputs, then a complete A/A rerun; compare all
non-timing fields including allocator-requested bytes. Use a 600-second
watchdog and 8 GiB virtual-memory cap per workload. Shared-host L0
times are diagnostic only; do not claim an isolated whole-method or
external-tool speed result.

## Registered hash-key variant of the tagged pair index

Frozen after both ordered-index workloads and their A/A checks passed, and
before editing the implementation. Repeat the identical p=13 and p=37
workloads above with the same action map, pair-orbit representatives,
postings, y-sign check, allocator instrument and full-table reference.
Replace only binary search over sorted abscissa keys with a hash map from
actual pair-sum abscissa to the same compact key index. Build the map in
charged table setup, include its retained and peak requested bytes, and
count hash lookups and candidate postings. Require exact equality of all
four-factor decomposition sets, independent pair-posting audit, and a
complete non-timing A/A check. Report setup plus all registered query
operation counts and shared-host L0 times for both arms; distinguish a
component setup win from a complete one-target IC or rho speedup.

## Registered direct-posting query variant

Frozen after the hash-key variant and its A/A checks, before code changes.
Repeat the same two frozen workloads and full-table reference. Keep the
orbit-representative setup: perform only one elliptic addition per 12-action
pair orbit and obtain all other pair sums from the action. During setup,
resolve each `(representative, action)` to a compact `(i,j,s,y-sign)`
posting and discard the query-time action map. Look up by actual abscissa
through the same hash-key index. Charge posting expansion in setup and
include the full retained index, representatives and hash map in requested
bytes. Require exact full signed decomposition-set equality, independent
posting-sum audit and non-timing A/A. Report setup/query counts and L0
times separately and in total. The purpose is to test whether the
symmetry-derived setup saving survives a lower-overhead query path;
there is no whole-method or rho speed claim from this oracle comparison.

## Registered p=37 full two-dimensional exact-oracle IC pilot

Frozen after the three pair-index variants completed and passed A/A, before
the p=37 solver was built. On the canonical p=37, b0=2 twist with prime
subgroup order 1,873,633, use the exact 624-point base and signs from the
tagged-index p=37 protocol: 48 axis points from the seed-zero L0/L2 union
record, then 576 interior points lifted lexically with `StdRng` seed
`0x5441_4747_0000_0037`. Re-lift every axis abscissa with `StdRng`
seed `0x5441_4747_4158_0037`, check its y-coordinate up to sign against
the recorded axis point, retain the recorded sign, and charge this axis
work to base construction. The generator and sole target `[271]G` are the
same as the mixed-line panel. Compute both twist eigenvalues, check all
52 folded factor columns by group multiplication, and use the same signed
four-factor full pair-table oracle. Generate the residual stream with
`StdRng(seed ^ p.rotate_left(17) ^ 0x7A12_0A02)`, seed zero; assert that
its first three `(a,b)` pairs match the frozen seed-zero single-line
record. Select one decomposition from each successful residual using
`StdRng(0x1A4F_01D5)`, re-add it, and solve all 52 factor columns plus
the target coefficient over the prime subgroup order. Stop at full rank
or 256 processed residuals; preserve failure at the cap. Verify the target
scalar and every solved factor log by group multiplication. Record all
processed `(a,b,hit-count)` tuples, selected signed terms, row, rank,
phase-by-phase `Fp` multiplications and group operations, modular row
operations, shared-host L0 wall time and the common base/target identity.
Run A/A and require exact equality of every non-timing field. This is an
exact group-oracle pilot; it does not measure the polynomial `S5` solve,
hash/allocator work, isolated wall time or matched rho band.

## Registered p=37 full-stream tagged pair-index replay

Frozen after the p=37 full-2D pilot reached rank 53 and its A/A matched,
before implementation. Reuse exactly that pilot's 624 signed points and
all 53 recorded residual `(a,b)` pairs. Compare the existing full signed
pair table with the direct-posting tagged construction, using the same
actual-abscissa hash lookup and one group addition per pair orbit as in
the three-query registered experiment. Demand equality of the *entire*
signed four-factor decomposition set for each of the 53 residuals and
audit every posting by a separate group addition. Record setup and
53-query field multiplications, group operations, live and peak requested
bytes, hash lookups, candidate postings and shared-host L0 times. A/A
must match every non-timing field. Use a 600-second watchdog and 8 GiB
virtual-memory cap per run. Do not combine this table replacement with
matrix, base or rho costs from another run into a claimed cold speedup;
report any arithmetic projection explicitly as a derived partial cost.

## Registered dense same-residual relation extraction

Frozen after observing the p=37 full-2D 53-query run and its 53-residual
pair-index replay, before the dense extractor is built. Reuse the exact
public target `[271]G`, subgroup orders, signed bases, orbit maps and
seed-zero residual stream from the p=13 and p=37 full-2D pilots. On each
curve, enumerate **all** signed four-distinct-factor decompositions of the
first residual with the existing full pair-table oracle. Process them in
the oracle's deterministic lexicographic order. Turn every decomposition
into the target/factor-log matrix equation. Audit the unanchored rank
(all factor logs plus target) on the complete first-query decomposition
set. Then use the public fact that the first base point is the generator:
fix its folded factor log to 1, move that column's coefficient to the
right side, and solve only the remaining factor logs plus target. This
anchor must be checked by group multiplication and cannot use the known
target scalar. Re-add each accepted relation as a group check; retain a
row only when it raises anchored rank. If the first residual does not
reach anchored full rank, continue the same frozen residual stream for at
most 64 queries. The oracle enumerates the complete decomposition set for
each queried residual even if rank is reached early. Stop at anchored
rank 6 (p=13) or 52 (p=37), solve, and verify the target scalar and every
factor log by group multiplication. Record complete per-query hit counts,
unanchored first-query rank, total decompositions scanned before anchored
full rank, accepted signed rows, rank trajectory, all phase-by-phase field
multiplications and group operations, modular matrix operations, retained
query count and L0 wall time. Re-run A/A and independently replay the
accepted equations, rank and answer from the recorded signed base using
a second elimination method and all-group-log enumeration. The oracle's
complete first-query hit count must agree with the earlier p=13/p=37
records. This tests dense extraction plus a public known-log anchor; it
does not price polynomial S5, hash/allocator work or matched rho.

## Registered combined tagged-index and dense-row p=37 exact-oracle solve

Frozen on 22 October 2026 after the separate full-2D dense solve and
direct-posting tagged 53-residual replay, before constructing the combined
producer. Reuse the canonical p=37, b0=2, order-1,873,633 twist, exact
624-point signed base and public `[271]G` target. Charge model creation,
axis re-lifts, interior base lifts, input checks, both eigenvalue checks,
all 52 orbit-map columns, the action-tagged direct-posting table, the first
seed-zero residual, dense anchored row selection, matrix solve and all
factor/target checks in **one producer run**. Enumerate the complete signed
four-distinct-factor decomposition set of the first residual in the same
lexicographic order as the full-table oracle. Keep every rank-increasing
row after fixing the generator factor log to 1; retain full selected terms,
rank progression, candidate count and answer. The target scalar must not
enter row construction. Stop at anchored rank 52 or, if absent, continue
the frozen stream with a 64-residual cap; retain a cap failure. For the
registered first residual, independently compute the full-table signed
decomposition set in a separately counted reference arm and require exact
set equality. Do not charge the reference arm to the tagged cold path.
Count model/base/input/eigen/map/table/stream/check `Fp` multiplications,
group operations, matrix operations, hash lookups/candidate postings,
retained and peak allocator-requested bytes, and shared-host L0 phase times.
Track reference-arm setup/query counts separately. Record a non-timing A/A
diff and reuse the independent dense point-keyed four-sign auditor to
reconstruct the entire first-query count, selected rows, rank and target.
Use a 300-second watchdog and 8 GiB memory cap per run. Compare the
result only with the earlier **same-target counted exact-oracle** baseline;
do not combine costs from separate runs or claim an isolated timing,
polynomial-S5, calibrated whole-method or rho improvement. Publish a
source-linked run report, graph and PDF, and retain negative outcomes.

## Registered mixed-line polynomial peeling pilot

Frozen on 22 October 2026 after the combined exact-oracle result and before
implementing the polynomial peeling producer. Reuse the canonical `p=37,
b0=2, r=1,873,633` curve, the seed-zero `L0 ∪ L2` 48-point signed base,
public target `[271]G`, and the **first four residuals in their existing
order** from `quartic_twist_mixed_v1.ndjson`. Do not select residuals by
polynomial outcome. Partition a four-distinct-abscissa decomposition by its
number `k=0,…,4` of factors on `L2`.

For `k=0` and `k=4`, solve the symmetrised `S5` on the relevant single line,
scaling the elementary symmetric coordinates by `s^j` for line scale
`s=1` or `t²`. For `k=1`, enumerate each signed `L2` point, subtract it from
the residual, and solve a symmetrised `S4` in the three remaining `L0`
abscissae. For `k=3`, enumerate each signed `L0` point and solve the same
`S4` on `L2`. For `k=2`, enumerate each unordered pair of distinct signed
`L2` points and solve the explicit symmetric `S3` in the two remaining
`L0` abscissae. The `S4` polynomial is the `S3`/`S3` resultant, independently
interpolated in the elementary symmetric functions of its three factor
variables and checked against fresh direct resultant evaluations. All
systems must use the **same residuals** and the exact recorded curve model.

Keep all returned field solutions, splitting failures, repeated/foreign
abscissae and group-invalid candidates in separate counters; verify every
accepted candidate by enumerating its physical signs and adding the four
points. Reconstruct the complete signed relation set for each `k` and
residual with an independently keyed group-pair oracle, and demand exact
set equality before any cost interpretation. Record precomputation,
per-system Weil restriction, elimination, splitting, peeling and group
verification counts, plus L0 wall and peak memory. Retain `None`, timeouts
and OOMs as censored failures. Run an identical A/A repetition; do not call
this a complete IC or rho speedup. The pilot succeeds only for the `k`
arms whose four-residual complete signed sets agree; a missing arm stays
explicitly incomplete. Use an 8-GiB memory cap and 1,200-second watchdog
per arm. A full polynomial relation stream, same-host msolve/Magma baseline
and matched one-target rho are separate subsequent gates.

## Registered mixed-pattern-only one-target polynomial IC discriminator

Frozen on 22 October 2026 after the first four-residual peeling pilot
showed complete signed-set agreement for `k=1,2,3`, before implementing
this one-target producer. Use the same `p=37, b0=2, r=1,873,633`, public
`[271]G` target, seed-zero 48-point `L0 ∪ L2` base and **full 61-residual
frozen stream**. Reconstruct the base, twist eigenvalues and four folded
columns in the producer; verify exact equality with the frozen coordinates
and target. Do not read the known scalar when building relations or solving.

For every residual, enumerate all complete signed relations from only the
`k=1,2,3` polynomial peeling systems above. Deliberately omit both
expensive all-one-line `S5` arms, `k=0,4`; this is a restricted factor
decomposition strategy, not a claim that the full union polynomial system
has been solved. Sort accepted relations by `(k, four signed terms)` and
retain every row that raises rank after fixing the public generator
factor log to 1. Stop on anchored rank four, or record a failure after
all 61 residuals. Solve for the other three factor logs and target scalar,
then verify every recovered factor log and the target by group multiplication.
The exact group-pair oracle must not enter the counted IC path.

Charge cold curve/base/eigen/orbit-map setup, `S4` interpolation, every
`S4`/`S3` solve including failed or zero-root systems, peeling, root
filtering, group sign verification, matrix work and answer checks in the
same producer. Record field multiplications, group operations, modular
operations, allocations/hash work and exclusive L0 phase times. Run the
strongest same-target 24-fold rho in the same process and price both arms
at 0, 0.1 and 1 group-addition equivalents; retain a separate matched-rho
interval and count all its phases. Repeat A/A and independently replay
the selected signed relations, orbit coefficients, rank and answer by a
scalar-keyed subgroup-log oracle. Use an 8-GiB cap and 1,200-second
watchdog. Treat isolated ABAB timings and same-input external Gröbner
comparison as separate gates before a ranked whole-method speed claim.
