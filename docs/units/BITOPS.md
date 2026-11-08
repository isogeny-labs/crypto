# `bitop/v1`: one unit of work for every ECDLP method

Every ECDLP cost in this repository, in `aburan28/cryptanalysis` and in cairn's
bound objectives is charged in one versioned accounting convention: the
**bit operation** (`bitop/v1`). Its primitive weights are fixed
gate-equivalent formulas. They define a reproducible accounting unit; they are
not physical gate counts, circuit lower bounds, or estimates of a particular
implementation's synthesized circuit. A method's cost is

```text
work = Σ count(primitive) · gates(primitive, width)
```

an exact integer under this convention, determined by the recorded counts,
widths, and these formulas. No host, compiler or run enters the pricing. The
implementation is `src/cryptanalysis/bitops.rs`.
`docs/units/bitop-v1.vectors.json` freezes the formula weights for every
primitive at a spread of widths. Any other implementation must reproduce every
row exactly. The implementation and vector field named `gates` is retained for
compatibility and means the convention's gate-equivalent weight.

## Why one unit

Before `bitop/v1` the repositories charged work in at least six units:

- group-addition equivalents, with non-group work converted by nanosecond
  ratios from one host;
- solver work priced by this host's wall time, or not at all;
- `full_prices.json` ratios;
- reference picoseconds (`cryptanalysis` ic-bench);
- Callgrind instructions;
- wall-clock online speedups.

You cannot divide a figure in one of these by a figure in another. An
exponent fitted in one is not an exponent in another.

The worst consequence was silent: work with no conversion ratio was charged
zero. Examples:

- linear algebra on any curve without a row in `docs/ic/calibration.json`;
- Buchberger, SAT and PDP solver work.

Every index-calculus total was therefore a lower bound, and the ratio of two
lower bounds bounds nothing.

## The conventions

Throughout, `b` is the bit length of `p`, `n` the degree of `F_{2^n}`, and
`w` the bit length of the subgroup order `r`.

### Integers

| primitive | model units | derivation |
|---|---|---|
| `w`-bit addition | `5w` | ripple carry, a full adder is 2 XOR + 2 AND + 1 OR |
| `w × w` product | `w² + 5w(w−1)` | `w²` partial-product ANDs, `w − 1` row additions |

### Prime field `F_p` (Montgomery representation)

| primitive | model units |
|---|---|
| `field_add` | `2·5b + 3b = 13b`: an addition, a conditional subtraction of `p`, and a `b`-bit select |
| `field_mul` | `2·(b² + 5b(b−1)) + 13b`: the product, the reduction's product, and the final subtraction |
| `field_sqr` | as `field_mul` |
| `field_inv` | `2b · 13b`: `2b` steps of the binary extended Euclid, each a subtraction, a comparison and a select |
| `field_sqrt` | `2b · field_mul`: exponentiation (`b` squarings, `b/2` products) plus a `b/2`-product allowance for the Tonelli–Shanks loop |
| `legendre` | as `field_inv` (the Jacobi algorithm is a gcd) |

### Binary field `F_{2^n}` (polynomial basis)

Reduction is by a pentanomial, i.e. four low terms. A trinomial is cheaper;
the convention takes the dearer modulus so that no curve is flattered by its
own.

| primitive | model units |
|---|---|
| `field_add` | `n` XORs |
| `field_mul` | `n² + (n−1)² + 4(n−1)`: schoolbook carry-less product plus reduction |
| `field_sqr` | `4(n−1)`: spreading the bits is wiring, then the reduction |
| `field_inv` | `2n · 5n`: polynomial extended Euclid, `2n` steps of a shift-XOR pair and a select |
| `as_solve` | `n² + n(n−1)`: half-trace as a precomputed `n × n` matrix over `F_2` times a vector |
| `canon` | `as_solve + 2n²`: normal-basis conversion, then the least of `n` rotations |

### `F_{p⁴}` (`ecbench::fp4`)

| primitive | model units |
|---|---|
| `field_mul`, `field_sqr` | `16 · field_mul(F_p)` (schoolbook) |
| `group_add`, `group_double` | `97 · field_mul(F_p)` (`ecbench::fp4`'s `C_ADD`, the extension inversion through the norm) |

### Group operations (affine, as every counted group in `ecbench` computes them)

| curve | addition | doubling |
|---|---|---|
| `y² = x³ + ax + b` over `F_p` | `I + 2M + S + 6A` | `I + 2M + 2S + 8A` |
| `y² + xy = x³ + ax² + b` over `F_{2^n}` | `I + 2M + S + 9A` | `I + 2M + 2S + 5A` |

A scalar multiplication counted as such (rather than as the additions and
doublings it performed) is charged `1.5·w` additions. That is the convention
of the strong rho reference, which is the only method that counts one this
way.

### Arithmetic modulo `r` and bit-packed work

| primitive | model units |
|---|---|
| `modr_mul` | `field_mul` of `F_p` at `w` bits |
| `modr_add` | `field_add` of `F_p` at `w` bits |
| `modr_inv` | `field_inv` of `F_p` at `w` bits (binary extended Euclid) |
| a multiply-subtract of elimination (`row_op`) | `modr_mul + modr_add` |
| `word_op` | 64: one 64-bit XOR, AND or compare of a bit-packed algorithm (F4 and Macaulay rows, Gray-code enumeration) |
| `hash_probe` | `4κ` with `κ` the bits of a field element: a fold-XOR hash, a key comparison and a select. Keys are field elements, already uniform, so no multiplier is needed. A table's real cost is memory, which is reported beside the unit and never folded in |

## What the unit leaves out, and how that is caught

The convention deliberately leaves out several things, which are reported
beside it and never folded into the model-unit total:

- **Memory.** Table sizes and peak resident memory.
- **Control.** Branches, loop overhead and allocation.
- **Uncounted work.** Anything a method does that no counter sees.

The **instruction audit** is a diagnostic for missing counters, not proof that
all work is counted or that the formula weights match an implementation's
circuit. `ecbench audit-ir` replays a record under Callgrind and compares each
phase's instructions per model unit with the reference walk's ratio. A phase
whose ratio is far above the reference walk's may be doing work its counters
miss, such as F4 symbolic preprocessing when only elimination XORs are
counted. A record that fails the audit is not admitted as a bound; passing it
does not validate the gate-equivalent formulas as physical gate counts.

Work that was uncounted when sessions were committed is counted in a phase's
`work_native` map, never in `native`, so committed records keep replaying
exactly. The code that counts an item of a family's uncounted list also writes
the marker `counted:<item>`; a record that carries the marker leaves the item
off its `known_uncounted` list, and a record without it (every record written
before the counter existed) keeps it.

An algebraic engine whose native unit counts one kind of work (the
elimination's word XORs, say) reports its whole run beside it, in
`SolverStats.extra`: `bitop_complete = 1` and the `bitop_word_ops`,
`bitop_hash_probes` and `bitop_field_*` counts of every primitive it
performed. The F4 family (`f4-f2`, `matrix-f4`, `matrix-f5`, `inherited-f4`,
`f6-ic`, and the `pdp3-*`, `pdp-split` and symmetrised oracles around them)
gathers those counts in `src/cryptanalysis/bitop_tally.rs`, one unit per
inner-loop iteration that touches a matrix word or a monomial mask: the
Macaulay and symbolic-preprocessing builds, monomial sorts (`k·⌈log₂ k⌉`
comparisons), hashing and column indexes, packing, pivot searches and lead
scans, read-back, the splitting tree's substitutions and scans, and the
`F_{2^n}` operations of the summation-polynomial build and of each lift.
Across calls the flag holds only while every call set it.

A record whose counters include one with no `bitop/v1` rule has no total. Its
`work.complete` is `false` and the unpriced counters are named. An unknown
cost is never written as zero.

### Self-check work (policy, 2026-10-21)

Every harness charges self-checks the same way, so their totals compare:

- **The domain is checked once, outside counted work.** Curve parameters,
  the generator and the target are validated when the instance is built. In
  ecbench this happens in `workload.rs` on scratch counters that are thrown
  away; it is the same for every arm.
- **The solution check is algorithm work, charged per target.** A method
  that verifies its own answer (rho's `[d]G = Q`, index calculus's `verify`
  phase) pays for it, because a real solver must.
- **Re-checking operands inside the arithmetic is not algorithm work.** Code
  that re-validates every input and output of a group operation (the
  hyperelliptic `checked_*` arithmetic re-runs `HyperellipticCurveP::validate`
  and `MumfordDivisorP::validate` on each call, 39–59% of all counted gates
  in `research/hec_bitop_20261020`) is a defensive check of the
  implementation, which a solver does not need. It is counted, so it can be
  seen, but outside the headline: a harness reports it as a separate
  validation subset, and the cost it compares is the total minus that
  subset (`algorithm_bitops`).

A harness that cannot separate operand re-checks from the algorithm's own
work says so and reports its total as including them.

## Relation to the unpriced-work band (AGENTS.md §5a)

§5a requires a comparison in group-addition equivalents to do two things:

- list each arm's unpriced counters;
- rank the arms only where the order holds with that work weighted at 0, 0.1
  and 1 GAE per operation.

A comparison in `bitop/v1` between two complete arms has no unpriced counters
for either side, so the band has nothing to weigh. The listing is
`work.unpriced` and `work.known_uncounted`, both empty. That leaves two
conditions for ranking in the unit:

- both arms' records are complete;
- both arms pass the instruction audit.

When either arm is incomplete, its `bitop/v1` total is a lower bound, as
§5a says of any partial price. Such an arm has no `bitop/v1` bound
(`bound fit --unit bitop/v1` refuses it), and its group-addition comparison
still goes through the band. For the same reason `--unit` and `--weight`
exclude each other.

## Extensions

An extension classifies counters, or names a field, that the convention had
no rule for. It never changes a primitive's weight, so
`docs/units/bitop-v1.vectors.json` and every figure priced before it are
unchanged; it can only turn an unpriced counter or an unpriceable record into
a priced one. Each is dated, and its tests are in
`src/cryptanalysis/ecbench/work.rs`.

### 2026-10-21 (ic-acct-6)

| counter or family | rule | why |
|---|---|---|
| `projected_base_membership_checks_uncharged` | not work (sub-total) | the `[r]P = O` check is `CountedGroup::mul`, whose additions and doublings the phase already charges |
| `projected_base_dedup_probes_uncharged` | `hash_probe` | the duplicate-abscissa guard: one keyed-set insert of a field element (the unit prices a keyed-set probe, not the container) |
| `projected_base_duplicate_points` | not work (outcome) | |
| `projected_base_column_map_entries_uncharged` | not work (size) | the column map's vectors are memory; the indexes derived from them are the `base_index_builds` item |
| `aux_supersingular` | not work (flag) | |
| `native_field_muls_uncharged`, `native_field_sqrs_uncharged` (`ic.cm_folded_*`) | `field_mul`, `field_sqr` | the native backend (`research/conductor_gap_cm_orbits_20261015/native/group.hpp`) increments them in every `mul` and `sq`, those inside its Jacobian doublings and mixed additions included |
| `native_group_doubles_uncharged`, `native_group_mixed_uncharged`, `native_scalar_calls_uncharged`, `native_factor_base_scans_uncharged` | not work (sub-totals) | their products are the two counters above |
| `native_queries_uncharged`, `native_hits_uncharged`, `native_rank_uncharged`, `native_rejected_duplicate_columns_uncharged`, `native_pair_table_keys_uncharged` | not work (outcomes and sizes) | the probes and inserts behind them are in the item below |
| `native_table_bytes_uncharged` | not work (memory) | |
| `independent_replay_wall_ns` | excluded diagnostic | wall-clock time is not work and is never priced |
| `ic.cm_folded_*` | `known_uncounted` item `native_backend_exponentiations_field_adds_hashing_modr_elimination_and_cm_map` | the backend's `power` exponentiations (inversions, square roots, the reciprocal modulo `r`), field additions, hash inserts and probes, arithmetic modulo `r`, elimination and the CM map are done with no counter; the records stay incomplete and say so |
| family `extension`, slug `icv1-f2m<N>-…` | `F_{2^N}` | `gaudry_quartic_bin`'s `y² + xy = x³ + ax² + b` over `F_{2^{4m}}`, `N = 4m`, `field_bits = m`; a binary field is priced by its degree, whatever tower the code computes in |
| family `extension`, slug `icv1-fp<b>k4-…` | `F_{p⁴}`, `p` of `b` bits | the `F_{p⁴}` convention above |

Left unpriced on purpose (both decided on 2026-11-07, below):

- `abscissae_enumerated_uncharged`: the counter is the size of the enumerated
  weight set, not the number of subsets summed nor their weight, so one
  increment has no fixed primitive sum.
- `solver_conflicts`: a CDCL conflict is not a fixed number of primitives.

### 2026-10-29 (packed pair-table filter)

| counter or family | rule | why |
|---|---|---|
| `table_filter_bit_sets` | `6 · word_op` | `mitm-prime-blocked:m=2,filter=1`: one bit set per stored entry of the packed pair table's membership filter (the key shift, the multiply-shift hash, the word index and bit shift, the OR); the probe's filter read happens inside a lookup the oracle already counts |

The filter's memory (8 to 16 bits per stored entry) is not added to the
`bitop/v2` oracle-table term, which prices `table_insert_attempts · (κ + w)`
and so already bounds the packed table from above by more than the filter
adds.

### 2026-11-07 (ic-acct-7, complete counters)

| counter or family | rule | why |
|---|---|---|
| `combination_modr_muls` | `modr_mul` | a product modulo `r` in a column-log combination: `ic.shared_rank`'s descent `Σ coef·log` and `·h⁻¹`, the `h⁻¹ = h^{r−2}` exponentiation's products, a full-rank check's `coef·log` |
| `combination_modr_adds` | `modr_add` | a sum modulo `r` in the same combinations (`Σ`, and `− a` for the residual shift) |

`ic.shared_rank` also writes, at the site that does the work, the counters the
2026-10-16 instrumentation already prices: `target_rng_draws` (every 64-bit
word its `StdRng` hands out, counted by a forwarding wrapper, for the rank
trials and the residual shifts), `row_build_words`, `row_build_modr_adds`,
`row_build_modr_muls` (its rows, the transcript's copy and the target's
dense row), its matrix's `gauss_entries_scanned` and `pivot_inversion_steps`,
the folded table's `table_insert_attempts`, one
`curve_equation_checks_uncharged` for the target's `is_on_curve`, and the
`counted:` markers of the four items it had not counted.

### 2026-11-07 (the counters left unpriced on purpose, decided)

Each counter the 2026-10-21 extension left unpriced, and the uncounted work it
named, is decided here from what the code executes per unit. No primitive
weight and no vector changes (`committed_vectors_are_frozen`), and no record
that was complete changes.

| counter or item | decision | rule or reason |
|---|---|---|
| `abscissae_enumerated_uncharged` (`cyclotomic`, `onb-weight`) | **priced from the parameters** | `CyclotomicField::weight_set` sums every `w`-subset of its atoms (`p = 2n + 1` powers of γ, or the `n` ONB elements) with `w` additions and inserts the sum into an ordered set (one keyed-set probe): `C(atoms, w) · (w·field_add + hash_probe)`, read from the `factor_base` parameter and the curve's degree, never from the count (the deduplicated set's size). A record without a parseable `w`, or whose count exceeds `C(atoms, w)`, keeps it unpriced. The field's construction (the root-of-unity search's exponentiations, the ONB inversion) and the `points_with_x` lift test of each abscissa visited have no counter: such records carry `known_uncounted` item `cyclotomic_field_construction_and_abscissa_lift_tests` |
| `sqrt_solves` over `F_{2^n}` (`field_sqrt` over a binary field) | **priced as the Artin–Schreier lift** | on `y² + xy = x³ + ax² + b` no square root is taken: the counter counts `lift_x` attempts (`gaudry_quartic_bin`, as every binary `points_with_x`), each `x²`, `1/x²`, `b·x⁻²`, two additions and the half-trace solve: `field_sqr + field_inv + field_mul + 2·field_add + as_solve`. `bin-t-line` records also carry `known_uncounted` item `bin_t_line_abscissa_map_and_lift_tails` (the map `t ↦ β(t + 1)/t` and a successful lift's product, sign draw and negation) |
| `field_sqrt` over `F_{p⁴}` | kept unpriced | `gaudry_quartic::Fp4::sqrt` is Tonelli–Shanks with a random non-residue search and an input-dependent loop: a non-square input costs one exponentiation by `(q − 1)/2`, `q = p⁴` (about `6b` `F_{p⁴}` products), a square at least four (about `24b`) plus about `6b` for each random non-residue trial (geometric, mean 2) and the loop's squarings, so no fixed primitive sum exists, and an allowance in the manner of `F_p`'s (`2b` products) would undercount a square input several-fold. The 7 `icv1-fp5k4-…` `ic.pipeline` records stay incomplete (they are incomplete through the solver gap and the five index-calculus items as well) |
| `solver_conflicts` (`sat-cdcl` records written before `bitop_complete`) | kept unpriced | `sat.rs` charges a conflict's analysis by the literals of the conflict clause and of every reason clause visited, its backjump by the trail entries undone, and propagation by the watch-list clauses scanned (`Solver::word_ops`); none is fixed per conflict, and the old records carry conflicts, decisions and propagations but no clause or literal visits. Re-executing the 8 smallest committed records (`research/ecbench_yield_sweep_20261004/sessions/koblitz`, seqs 3–48) on the counted engine reproduces their conflicts and propagations exactly and gives 3,092–3,185 word operations a conflict on one arm (`sub6-da-sat`) and 4,595–4,712 on the other (`sub8-da-sat`); a propagation is 50.6–53.0. No constant fits either within 1%. The 29 records stay incomplete; a re-run on the current binary prices them exactly (`bitop_complete = 1`), which is the route (`research/bitop_audit_pricing_20261107/sat/`) |
| `solver_F_q_multiplications` (`d4-bin-macaulay`) | kept unpriced | the count is not one primitive: `BinD4Oracle` adds `F_q` products, `F_q` inversions (one each) and the large-prime merge's products modulo `r` (`lp_merge_mulmods`) into the same total; the `F_q` arithmetic is a log/exp table lookup, and the engine's additions and Macaulay builds have no counter (`solver_engine_work`). The 17 `icv1-f2m…` records stay incomplete |
| `solver_F_p_multiplications` (`gaudry-d4`) | kept unpriced | the count is modelled, not executed: an `F_{p⁴}` product is charged 13 or 19 `F_p` products (Karatsuba or schoolbook, against the convention's 16) and an inversion 16 plus an uncounted `F_p` inversion. The 7 `icv1-fp5k4-…` records stay incomplete |
| `solver_F_p_multiplications_(grid)` (`pkm-semaev`) | kept unpriced | `PkmSemaev::grid` adds 6 a `S₃` evaluation where the closure executes 8 `mulm` (two by small constants) and its additions, 2 per one-variable candidate where 3 `mulm` run, and 8 per pair test where 8 run: a modelled count, not the products executed. The 104 records stay incomplete |
| `solver_F_p_multiply-adds_(elimination_only)` (`pkm-semaev`) | kept unpriced | the elimination's multiply-adds are counted and nothing else of the engine (the system build, the trace fallback, the root extraction); pricing them would raise a lower bound and complete nothing. The 8 records stay incomplete |
| `CM_FOLDED_NATIVE_GAP` (`ic.cm_folded_*`) | kept listed | `folded.cpp` computes inversions and roots by `power` (a variable-length square-and-multiply), relation signatures as `std::to_string` strings in a `std::set`, pivots by `power(row[k], r − 2, r)`, and the CM map, none with a counter. The doublings' and mixed additions' field additions are fixed by `group.hpp`'s formulas and could be priced, but that would raise 477 committed lower bounds without completing any record, so it is left for the owner. The 477 records stay incomplete until the backend counts its work |

Work that was never counted stays listed: the five index-calculus items
(`gauss_column_scans_and_pivot_inversions`, `relation_row_construction`,
`walk_bookkeeping`, `table_insert_attempts`, `base_index_builds`) and the
solver gaps leave a record only when a re-run on an instrumented binary
counts them.

## Changing the convention

A committed figure must reprice to the same integer forever, so these formulas
and their weights are frozen. A different convention gets a new name:

- Karatsuba products;
- a trinomial-aware reduction;
- a memory-cost model: that one is `bitop/v2`, below.

Every record keeps its primitive counts, not only its total, so any later
version re-prices the whole history without re-running it.

## `bitop/v2`: memory

`bitop/v2` is `bitop/v1` plus a declared memory term. Its eight open
definition choices are listed, with evidence and a recommendation each, under
"Open definition choices" below; every one is recommended, pending owner
sign-off, not settled.  A table-based method
trades gates for memory, and a unit that counts gates alone can flatter it;
`bitop/v2` makes the trade visible without touching the gate figure.  It is
frozen as this section states it; the entry widths are in
`docs/units/bitop-v2.vectors.json`, which the test
`committed_v2_vectors_are_frozen` holds to the implementation
(`src/cryptanalysis/bitops.rs`, `src/cryptanalysis/ecbench/work.rs`).
`bitop/v1` is untouched: every v1 figure, vector and committed record
reprices exactly as before.

A record priced in `bitop/v2` reports four figures:

- `bitops`: the `bitop/v1` gate total of the same record, the same integer.
- `memory_bits`: the bits of the structures the method holds, computed from
  fields committed records already carry.
- `at = bitops × memory_bits`: an area–time product, an exact integer.
- `memory_complete` with `memory_unpriced`: whether every structure the
  rule names has its field, and which are missing.  A missing field makes
  `memory_bits` and `at` lower bounds; it is never charged zero silently.

Throughout, `κ` is the bits of a field element (`element_bits()`: `b` for
`F_p`, `n` for `F_{2^n}`, `4b` for `F_{p⁴}`) and `w` the bits of `r`
(`max(1, r_bits)`, as the gate formulas take it).

### Entry widths

Every stored structure is charged as a count of entries times a declared
width.  The widths are what an implementation would store with no packing
or compression; they are conventions, like the gate weights, not measured
footprints.

| structure | one entry holds | bits |
|---|---|---|
| distinguished-point table (rho, kangaroo, the claw, the strong Koblitz rho) | an affine point and its two coefficients modulo `r` | `2κ + 2w` |
| BSGS baby-step table | a point key and its index | `κ + w` |
| factor-base point | an affine point | `2κ` |
| relation-matrix entry | one coefficient modulo `r` | `w` |
| oracle table entry (pair, orbit, fiber and auxiliary BSGS tables) | a field-element key and an index | `κ + w` |
| solver peak | a byte | `8` |

A kangaroo stores a distance where rho stores two coefficients; the
distinguished-point width is a declared upper bound for it, the same for
every walk.  Walk state in flight (the lanes of the strong rho, the
coefficient pair of each walk) is left out: it is a constant number of
entries, and no record counts it.

### Which fields a record kind is read from

**Generic methods** (`rho.*`, `bsgs.*`, `kangaroo.*`, `claw.*`).  The entry
count is the first of `counters.inserts_uncharged`,
`counters.table_inserts_uncharged`, `counters.table_entries`,
`counters.distinguished_points` the record carries, the reading the bound
fitter's memory axis already uses (`bounds::MEMORY_COUNTERS`).  A method whose
id starts with `bsgs` is charged the baby-step width; every other generic
method the distinguished-point width.  A record carrying none of the four
reports `memory_unpriced = ["table_entries"]`.

**Index calculus** (`ic.*`).  The sum of:

- *factor base*: `factor_base.signed_points · 2κ`.  Without a
  `factor_base` block: `factor_base.signed_points` is unpriced.
- *relation matrix*: `rows · columns · w`, with `rows` =
  `counters.matrix_rows`, else the `linear_algebra` phase's native `rows`;
  `columns` = `factor_base.columns + 1` (the target column), else the
  `linear_algebra` phase's native `columns` (which already counts it).  This
  is the **dense** size and is declared as an upper bound: the matrices are
  sparse (two or three non-zeros a row for the pipeline's decompositions)
  and the elimination the records count works on that sparsity.  A missing
  source is unpriced as `matrix_rows` or `factor_base.columns`.
- *oracle tables*: where a phase carries the marker
  `counted:table_insert_attempts`, `Σ table_insert_attempts · (κ + w)` over
  the phases' `work_native` maps (zero for an oracle without a table; an
  attempt is kept or not, so attempts bound the entries kept from above).
  Without the marker, `Σ (pair_table_entries + fiber_table_entries +
  aux_bsgs_table_entries) · (κ + w)` over phase natives and counters where
  any of them is present; else `table_insert_attempts` is unpriced.
- *solver*: `8 · solver.peak_bytes` when the record has a solver block
  with `peak_bytes > 0`; a solver block with `peak_bytes = 0` leaves
  `solver.peak_bytes` unpriced (the engine did not measure it); a record
  with no solver block (a table oracle) has no solver term.
- *native backend table*: `8 · counters.native_table_bytes_uncharged` where
  the counter is present (the `ic.cm_folded_native_v1` backends declare
  their table size this way).
- `ic.k4_gaudry` keeps its oracle table in no counter: it is charged no
  table term and lists `oracle_table_entries` unpriced in place of the
  oracle-table rule.

`memory_bits` is the **sum** of the terms, not their maximum: the base and
the oracle's tables live through the relation search, the matrix is built
beside them and the solver runs inside it, so the sum is the declared size
of what is held at the pipeline's widest point.  It is declared, not
measured: `time.max_rss_kib` is the whole process and never enters.

### Records that cannot be priced completely

Each of these is reported with `memory_complete = false` and the missing
fields named; its `memory_bits` and `at` are lower bounds:

- index-calculus records written before 2026-10-16, whose phases carry no
  `work_native` and so no `counted:table_insert_attempts` marker and no
  `*_table_entries` native (`table_insert_attempts`);
- records with a solver block whose `peak_bytes` is `0` (the
  `d4-macaulay`, SAT, `fghr` and `line-s4` engines; `solver.peak_bytes`);
- `ic.k4_gaudry` (no `factor_base` block, no `matrix_rows`, no oracle table
  size);
- index-calculus records with no `factor_base` block, the native backends
  among them (`factor_base.signed_points`, `factor_base.columns`,
  `matrix_rows`; their `native_table_bytes_uncharged` is charged);
- generic records carrying none of the four memory counters.

The `extension` family has no `bitop/v1` field and therefore no price in
either unit: `ecbench work` reports the same error for it under
`bitop/v2` as under `bitop/v1`.

There is no per-phase memory figure: the structures are held across
phases, and the fields a record carries do not say when each was
allocated or freed.  The per-structure terms are reported instead
(`memory_terms`).

### Ranking in `bitop/v2`

Two arms rank in `bitop/v2` only where

- both records are complete in gates (`complete`) and in memory
  (`memory_complete`), and
- the order is the same in `bitops` and in `at`.

Where the two orders disagree, the arms trade gates for memory and
`bitop/v2` ranks neither; the comparison reports both figures.  A lower
bound in either figure ranks nothing against a complete one, as in
`bitop/v1`.

### Open definition choices: recommended, pending owner sign-off

The eight definition choices PR #2639 left to the owner are **not settled**.
The rules above are what the code computes today. Each recommendation below
is **recommended, pending owner sign-off**, and none is implemented: every
one would change a `bitop/v2` figure, and no change was clearly better than
the frozen rule on the evidence. No `bitop/v1` figure depends on any of
them.

The evidence comes from `examples/bitop_v2_choices.rs`, which prices each
alternative beside the frozen rule:

- the 12,080 verified records of the 25 hold-out sessions of
  `research/bitop_ladder_20261019` (P16–P40: 560 records a session for P16–P28, 400 for P29–P40);
- the 477 `ic.cm_folded_*` records of the archive.

Outputs are in `research/bitop_audit_pricing_20261107/v2/`.

| # | choice | frozen rule | what the alternative changes on committed records | recommendation (pending owner sign-off) |
|--:|:--|:--|:--|:--|
| 1 | sum or max of the terms | sum | generic methods have one term: no change. `ic.pipeline`: sum/max is 1.37 median, [1.03, 1.61]. No ranking of the 393 hold-out arm pairs changes | **keep the sum**: the base and the tables live through the relation search, and the max under-declares by at most the number of terms |
| 2 | dense or sparse matrix | dense `rows·columns·w` | sparse (non-zeros at construction, `row_build_modr_adds`, times `w + ⌈log₂ columns⌉`) is 23.7× smaller median, [6, 1,111]. The matrix is 24% of the IC sum (median, max 34%), so the IC total falls 1.29× median (max 1.48×). IC costs 3.9–193× the generic arms in gates and 11–10⁶× in `at`, so no IC-versus-generic order can flip | **keep dense** as the declared bound: the hold-out's `incremental-gauss` builds every row as `|F| + 1` zeroed words (`row_build_words`), so dense is what the measured code holds. Report the sparse figure beside it as a diagnostic, and adopt it only for a sparse engine that records its peak non-zeros |
| 3 | insert attempts as the table bound | `Σ table_insert_attempts · (κ + w)` | attempts never exceed the `2^κ` field-element keys (max 3.1% of them, median 0.1%), so collisions can make attempts exceed entries kept by about `attempts/2^{κ+1}`, at most 1.6% here. The table is 73% of the IC sum (median, [62%, 97%]) | **keep attempts**: an upper bound that is tight to about 2% on the hold-out, read from a counter every current record carries |
| 4 | kangaroo distinguished-point width | `2κ + 2w` | an affine point, one distance and the herd bit, `2κ + w + 1`, cuts the kangaroo's memory by `w − 1` bits an entry. Pairs ranked under the rule, `bsgs.negation` against `kangaroo.vow`: 8 → 6 of 25 | **adopt `2κ + w + 1`** for kangaroo entries: it is what the walk stores, and the shared width over-declares the kangaroo only. It moves 2 hold-out rankings to "trade", so it needs the owner |
| 5 | `at = bitops × memory_bits` and its ranking rule | rank only where the gate and `at` orders agree | every IC-versus-generic pair agrees (204/204). The rule leaves the true trades unranked: `bsgs` against `kangaroo` 17/25, `bsgs` against `rho` 22/50, and IC against IC (two IC arms on one curve) 17/39 | **keep** both the product and the rule: they rank where nothing trades and say "trade" otherwise. A single `at` order would rank BSGS against rho by memory alone |
| 6 | what `bound fit --unit bitop/v2` does | fits the `bitop/v1` gates; memory-incomplete records are lower bounds | across the 25 hold-out sizes the memory exponents differ from the gate exponents (slope against `log₂ r`: rho 0.57 gates / 0.30 memory / 0.87 `at`; kangaroo 0.60 / 0.31 / 0.91; BSGS 0.60 / 0.55 / 1.16; IC 0.71 / 0.65 / 1.36) | **fit memory and `at` as two further exponents** beside the gate fit, each with its interval, so that v2 has a slope of its own: today v2's fit is v1's with lower-bound marks |
| 7 | excluding walk state | excluded | the jump table and walks in flight at the distinguished-point width are, as a share of the declared total: rho median 13%, max 3.4×; kangaroo median 3%, max 12×; IC median 0.45%, max 13%. No hold-out ranking changes | **include the jump table** (from the `jumps` / `jump_count` / `walk_jumps` counters records already carry) as a `walk` term: at small sizes it exceeds the stored table, and leaving it out flatters the walks there |
| 8 | native backends' completeness | read `native_table_bytes_uncharged`; base, matrix and tables unpriced | all 477 `ic.cm_folded_*` records are memory-incomplete (`factor_base.signed_points`, `factor_base.columns`, `matrix_rows`, `table_insert_attempts`), the same records that are gate-incomplete (`CM_FOLDED_NATIVE_GAP`) | **keep them incomplete**, and have a backend declare each structure's bytes (base, matrix, tables) the way it declares its table. Mapping its outcome counters (`native_rank_uncharged`, `native_pair_table_keys_uncharged`) to entries would be a guess |

### Where it is emitted

`ecbench work --unit bitop/v2` writes one `ecbench.work/v2` line per record
(`bitops` identical to the `bitop/v1` line, then `memory_bits`,
`memory_terms`, `at`, `memory_complete`, `memory_unpriced`);
`ecbench bitop-vectors --unit bitop/v2` prints the frozen entry widths;
`ecbench bound fit --unit bitop/v2` fits the same gate totals as
`bitop/v1` and marks a record incomplete in memory as a lower bound.
Without `--unit`, or with `--unit bitop/v1`, every command's output is the
`bitop/v1` output, byte for byte.
