# F4 over a mixed-radix tower: the cube rule (engine extension)

Branch `perf/crypto-tower-cuberule`, cut from
`claude/performance-optimization-parallel-vgenii` at `009e796a7a`. Not pushed.

**What this is.** An engine extension, not a performance change. It lets the
tower F4 engine run towers whose steps have radix 3 (`y_j^3 = y_{j+1}`, the cube
rule), mixed radix (`2, 2, 3`), and the radix-4 control (`y_j^4 = y_{j+1}`), the
build that `RESEARCH_PRIME_FIELD_BARRIER_MAP_20261104.md` section 6.1 ("Build
required") and `research/non_semaev_decomposition_20261113/README.md` section 6
name as the prerequisite of the radix-versus-levels discriminator.

**What this is not.** No research experiment was run. The radix-versus-levels
cells (`m = 4`, `|V| in {2^6, 2^12}`, three primes, two planted targets, a
shape-matched null per arm) go through `isolab-eks` with a frozen task card, a
resume checkpoint and a `bench.record`. Every `D` below is **engineering smoke
output** from toy instances (seconds, local, unisolated), printed to show the
engine runs. None is a result, none compares arms under a preregistered
protocol, and none carries a replay certificate, a validator report or a matched
rho reference. All figures are **[pcc:unverified]**.

## 1. Design in one page

`src/cryptanalysis/f4_fp_tower.rs` (the radix-2 engine) keeps its single rewrite
type, `SquareRule`, its square-free `u128` monomials (64 tower bits, 4 free
bytes, a 32-bit degree) and its degree-as-popcount. It is **untouched**: the
only edit to it is `pub(crate)` on items the new engine shares (its linear
algebra and basis element type; the diff of that file is 44 changed lines,
each the same line with `pub(crate) ` inserted: checked mechanically, no logic).

The extension is a new module, `src/cryptanalysis/f4_fp_tower_radix.rs`
(`cryptanalysis::f4_fp_tower_radix`), the same algorithm over

```text
R = F_p[y_0 .. y_{n-1}, u_0 ..] / (y_j^{r_j} - rule_j),   r_j in {2, 3, 4}
```

A separate type was chosen over a feature gate or a widened `Mono` because
`f4_tower_generic`, `sig_fp_tower`, `hky_fp_tower` and the PKM builders all use
the radix-2 key layout directly (`mask_of`, `free_of`, `mono`, ...), and
because it gives the strongest identity check available: a pure radix-2 ring run
through both engines must give the same F4 trace (section 5, test 5).

### Monomial key (`XMono = u128`)

```text
bit 127 .. 112   degree            16 bits, the exponent sum
bit 111 ..  80   free exponents    4 x 8 bits, each stored as 255 - e
bit  79 ..   0   tower exponents   40 x 2 bits, lane j at bit 2j, stored as 3 - e
```

- Integer order of the key is grevlex with variable 0 largest (the order
  `f4_fp` and the radix-2 engine use): degree first, then the smaller exponent of
  the last variable wins, because the exponent fields are complemented and the
  last variable is the most significant.
- Limits: 40 tower variables (the barrier map's "enough for N <= 40"), 4 free
  variables with exponents below 256, degrees below 2^16, `p < 2^32`.
- Degree is the exponent sum `sum_j e_j + sum_k f_k`, not a popcount.
- Divisibility, lcm, quotient and coprimality are lane-wise on the 2-bit lanes
  (SWAR: `lanes_gt` compares all lanes at once); `quotient` never borrows because
  each lane of the dividend is at least the divisor's.

### Rules (`PowRule`)

```text
y_j^r = sum_{k<r} ( c[k] + b[k] * y_next ) * y_j^k          r = radix in {2,3,4}
```

- `r = 2` with `c = [d, c]`, `b = [b, a]` is the radix-2 `SquareRule`
  (`PowRule::from_square`); `RadixRing::from_square_ring` converts a whole ring.
- `r = 3` is the cube rule (`PowRule::cube`, alias `CubeRule`); `PowRule::kummer`
  gives `y^r = y_next` (or `y^r = 1` on a top level). A Dickson step
  `y^3 = 3y + y_next` is `c = [0,3,0], b = [1,0,0]` (tested).
- Leading monomial. Every other monomial of the right-hand side is
  `y_j^k` or `y_j^k y_next` with `k <= r-1`: degree at most `r`, and at degree `r`
  only `y_j^{r-1} y_next`, which is below `y_j^r` in grevlex because the later
  variable has the larger exponent. So `LM = y_j^r` for every rule, the leads are
  pairwise coprime, and the rules are a Groebner basis of the polynomial-ring
  ideal. `RadixRing::check` enforces: radix 2..=4, no term above
  `y^(r-1)`, `next` strictly later, a top rule without `y_next` terms.
- Standard monomials: `prod_j r_j` exponent vectors with `e_j < r_j`.

### Carry on overflow (products and normal form)

`mul_var_into(e, j)`: if `e_j + 1 < r_j` the lane is incremented. Otherwise
`y_j^{r_j}` is replaced by the rule: lane `j` is cleared and for each `k < r`
the terms `c[k] * y^{e0 + k e_j}` and (recursively, since `y_next` may itself
overflow and cascade up the tower) `b[k] * y_next * y^{e0 + k e_j}` are emitted.
`exp_product_into` multiplies two exponent vectors: lanes that stay within
`r_j - 1` are added outright (one SWAR test: `lanes_gt(eb, cap - ea)`), the
overflowing lanes are multiplied in one variable at a time with a merge after
each so lists do not grow exponentially. `mul_terms` keeps the radix-2 engine's
fast path: a term with no overflow gets `key + constant` (degree up, complemented
lanes and free bytes down, nothing borrows), stays distinct and descending, and
goes straight into the output; only carried terms are sorted and merged.

### Tower pairs

The S-polynomial of a basis element `g` against `y_v^r - tail_v`, for a tower
variable `v` with `e_v(LM g) >= 1`, has `lcm = y_v^{r_v} * LM(g) / y_v^{e_v}`, so
the multiplier is `y_v^{r_v - e_v}`. In the quotient the rule term vanishes, so
the matrix row is the single row `NF(y_v^{r_v - e_v} * g)` at degree
`deg LM(g) + (r_v - e_v)`.

| radix | `e_v(LM g)` | multiplier | pair degree |
|:--|:--|:--|:--|
| 2 | 1 | `y_v` | `deg + 1` (as the radix-2 engine) |
| 3 | 1 | `y_v^2` | `deg + 2` |
| 3 | 2 | `y_v` | `deg + 1` |
| 4 | 1, 2, 3 | `y_v^3, y_v^2, y_v` | `deg + 3, +2, +1` |

Tower pairs are never pruned and never used to prune, as in the radix-2 engine.
Critical pairs go through Gebauer-Moeller on the lane-wise lcm; the chain
criterion enumerates the monomials between `LM(h)` and the lcm with a
mixed-radix odometer; `reducer_for` enumerates divisors in the order the
radix-2 engine does (for 0/1 lanes `(s-1) & mask`), so the first shortest
reducer, and therefore the matrix, is the same on a radix-2 ring.

### What is shared with the radix-2 engine (not copied)

`Fp`, `Row`, `Acc`, `echelon`, `eliminate` (Faugere-Lachartre, `B' = A^-1 B`,
full-rank exit, lazy accumulator, AVX2/AVX-512 kernels), `Echelon`, `ElimStats`,
`Elem` (dense echelon rows as basis elements), `RowDesc`, `ProductCache`,
`MonoMap`, `number_of`, `order_columns`, `StepTrace`, `TowerF4Options`. The
radix engine reuses `TowerF4Options` (deadline, `max_degree`, `stop_staircase`,
`max_nnz`, `max_dense`, `full_rank_exit`, `on_step`, `on_stop`).

### Not supported in the radix engine (use the radix-2 engine)

Weighted gradings (`TowerF4Options::weights`), trace learning and replay
(`record_trace`, `replay`; `f4_radix` panics if they are set), the signature
engine (`sig_fp_tower`), the generic-field engine (`f4_tower_generic`, primes
above 2^32), the `hky_fp_tower` and PKM system builders. Porting them is a
mechanical repeat of this change over their own copies of the key functions and
is deferred; the discriminator's cells at `p < 2^32` do not need them.

## 2. Invariants

1. The radix-2 engine's source is unchanged but for visibility; its keys, its
   order, its trace and its fingerprints are unchanged (section 4).
2. On a radix-2 ring the radix engine's F4 run equals the radix-2 engine's
   step for step: every `StepTrace` count, `muladds`, pair counters, the
   solving degree, and the basis element by element (test 5).
3. Normal forms are unique: `from_raw` and `mul_terms` equal an independent
   dense-exponent reduction by the rules (test 3).
4. Key order is grevlex and the monomial algebra is exact (test 1).
5. The result does not depend on the product cache size (test 11).
6. A refutation (`1` in the ideal) is the same as no solution in `V^m`; a
   non-refuting run's standard monomials count exactly the solutions (the tower
   ring is reduced, `R = F_p^{V^m}`), and every solution lies on every basis
   element (tests 6 to 9).

## 3. Files

| file | change |
|:--|:--|
| `src/cryptanalysis/f4_fp_tower_radix.rs` | new: keys, `PowRule`, `RadixRing`, `XPoly`, `f4_radix`, `RadixF4Report`, `x_staircase_at_most`, 15 tests |
| `src/cryptanalysis/f4_fp_tower.rs` | visibility only (`pub(crate)`) on shared items |
| `src/cryptanalysis/mod.rs` | `pub mod f4_fp_tower_radix;` |
| `docs/perf/20261114-f4-tower-cuberule.md` | this note |

## 4. Verification against the base

Everything here is local and **[pcc:unverified]**: a shared 4-vCPU VM, unisolated,
Cargo `dev` and `test` profiles at `opt-level = 1` (env overrides),
`-j2`, `CARGO_INCREMENTAL=0`. (Fingerprints are integer outputs and do not
depend on the optimisation level; no wall or `Ir` figure is claimed, because
this is not a performance change.)

**(1) Existing tests, unchanged and passing** (test binary built from the
candidate; module filters, `cargo test --lib`):

| filter | result |
|:--|:--|
| `cryptanalysis::f4_fp_tower::` | 29 passed, 0 failed |
| `cryptanalysis::f4_fp::` | 44 passed, 0 failed |
| `cryptanalysis::sig_fp_tower` | 8 passed, 0 failed |
| `cryptanalysis::f4_tower_generic` | 39 passed, 0 failed |
| `pkm` (42 tests: `ic_framework::pkm_semaev`, `ecbench::methods`, `ecbench::band`, `pkm_criterion`, `ml_dataset`) | 42 passed, 0 failed (2966 s at 4 threads) |
| `cryptanalysis::hky_fp_tower` | no unit tests in the module (0 matched); its examples are not built here |

The `pkm` run used the binary built from the first commit; the later commit
changes only test code in the new module.

**(2) perfbench `fp_gb` fingerprints identical to base.** Two example
binaries, `cargo build --example perfbench`: `base` at `009e796a7a`
(sha256 `3438de85...16b8`) and `cand` at `14e0e03d3c` (sha256 `9da51829...7d62`;
the later commit touches only tests). `perfbench run --filter fp_gb/
--samples 1 --min-samples 1 --max-seconds 1 --warmup-seconds 0`:

- 27 kernels, same ids; base and candidate fingerprints are **equal on all 27**
  at the default 4 threads, and again with `RAYON_NUM_THREADS=1` and `=2`
  (the thread-dependent kernels `tower_f4_kummer_m2_n12` and `_m3_n9` differ
  between thread counts in both binaries alike, as `docs/perf/fingerprints.json`
  records);
- the candidate also equals the recorded golden `docs/perf/fingerprints.json`
  on all 19 recorded `fp_gb` kernels at 1 and at 2 threads (the other 8 are
  Full-tier kernels the golden file does not list; base equality covers them).

| kernel | fingerprint (base = candidate) |
|:--|:--|
| `fp_gb/f4_katsura6_p65521` | `d9bc84f8f1e014c8` |
| `fp_gb/f4_katsura7_p65521` | `9efdf23df9cf4cfb` |
| `fp_gb/f4_cyclic5_p65521` | `c06751ee98f77960` |
| `fp_gb/f4_cyclic6_p65521` | `a908de79cc53065e` |
| `fp_gb/f4_quad_n7_p65521` | `fe40a23c3590ae67` |
| `fp_gb/f4_quad_n8_p65521` | `c1c45fc4390634ce` |
| `fp_gb/f4_cubic_n4_p31` | `d66a1e112304d13c` |
| `fp_gb/f4_solve_cubic_n3_p29` | `c57ca1327e4b1a64` |
| `fp_gb/f4_solve_quart_n3_p31` | `fa40034c64f5535c` |
| `fp_gb/f4_solve_quad_n4_p31` | `b63adcf1704709a6` |
| `fp_gb/f4_pkm_kummer_m2_t5` | `bce5dd8bb6a82b62` |
| `fp_gb/f4_pkm_kummer_m3_t2` | `eec47de270189d32` |
| `fp_gb/f4_pkm_kummer_m2_t6` | `aba0e55bab2ccace` |
| `fp_gb/f4_autoreduce_pkm_m2_t4_x2` | `ae3970352ca7670e` |
| `fp_gb/tower_f4_kummer_m2_n12` | `5cff44f1ccff1c76` |
| `fp_gb/tower_f4_kummer_m2_n12_planted` | `913e061eb88d6887` |
| `fp_gb/tower_f4_kummer_m3_n9` | `430638c12d88fd3a` |
| `fp_gb/tower_f4_kummer_m4_n8` | `8fa4a2b10311a05c` |
| `fp_gb/tower_f4_kummer_m2_n16` | `7702a1d8b7984359` |
| `fp_gb/sig_tower_kummer_m2_n12` | `c8dce7b8aad654cb` |
| `fp_gb/sig_tower_kummer_m3_n9` | `b5b233da56b179b3` |
| `fp_gb/sig_tower_kummer_m3_n9_degsteps` | `6e4ef971ec95f5e2` |
| `fp_gb/sig_tower_kummer_m2_n14` | `fa6f086bf730d9f8` |
| `fp_gb/gaudry_s4_macaulay_p1039` | `3599b084fef59c3d` |
| `fp_gb/gaudry_quartic_generic_d3_p269` | `6400bf7912e8394a` |
| `fp_gb/gaudry_quartic_generic_d4_p269` | `209d0040a8c6a4a5` |
| `fp_gb/buchberger_katsura3_p32003` | `52a6163ee4192edc` |

**(3) Mutation check of the new tests** (one extra build, then reverted; not
part of the commit): three deliberate faults applied together to
`f4_fp_tower_radix.rs`: the tower multiplier always `y_v` while the degree stays
`deg + (r - e)` (M1), `reducer_for` taking the longest instead of the shortest
reducer (M2), `lanes_down` not refilling the lower lanes (M3). 6 of the 15 new
tests failed: the degree-one cube pair test and the radix-3 refutation test
(wrong answers), the mixed-radix test, both radix-2 identity tests (different
`muladds` and step counts), and the divisor-walk test (9 divisors visited
instead of 81). Two tests that still passed under that combination (the Dickson
tower, the radix-4 control) are therefore less sensitive to M1; the others cover
it. The faults were not isolated one at a time.

## 5. Tests (new, `cryptanalysis::f4_fp_tower_radix::tests`)

All use deterministic seeds. Where a "solution set" is checked, the
reference is an exhaustive search over `V^m` of the raw equations in `x`
(independent of every ring), and the engine is additionally checked against the
generic `f4_fp` run on the same ideal presented as a plain polynomial ring
(raw equations plus the tower relations `y_j^r - rule_j` as ordinary
polynomials, no tower rewriting): the minimal leading monomials of the two
Groebner bases (the radix basis plus the tower leads `y_j^{r_j}`, against the
generic F4's basis) must be the same monomial ideal.

1. `keys_order_like_grevlex_and_the_monomial_algebra_is_exact`: 4000 random pairs
   of exponent vectors (12 tower lanes with exponents 0..=3, 2 free): key order
   equals a reference grevlex comparator; divides, lcm, quotient, coprime,
   degree and the round trip agree with exponent-vector arithmetic.
2. `the_divisor_walks_visit_every_divisor_once`: the `reducer_for` odometer
   visits every divisor once in descending mixed-radix order; `any_between`
   visits every monomial in `[lo, l]` but `l` exactly once.
3. `products_match_a_generic_reduction`: rings of radix `[3]`, `[3,3]`,
   `[2,3,2]`, `[4,3,2,2]`, `[3,4,3]`, `[4,4]`, random full-support tails, with and
   without a free variable: `from_raw` and a product by a ring monomial equal an
   independent dense reduction, are strictly descending, and a stored element
   multiplies as its polynomial does.
4. `a_free_exponent_past_255_is_refused`.
5. `a_radix_2_ring_gives_the_radix_2_engines_run_step_for_step`: six ring
   shapes (Kummer, and general `a,b,c,d` rules, with and without a free
   variable, up to `p = 65521`), three random systems each, `max_degree = 7`:
   identical `StepTrace` counts per step (20 fields), `muladds`, pair counters,
   `solving_degree_max`, and the bases element by element. These random systems
   all end in refutations, so
   `a_radix_2_ring_gives_the_radix_2_engines_run_on_planted_systems` repeats
   the comparison on consistent planted Kummer systems (`p = 97..257`, up to 5
   levels, `m = 2, 3`), complete runs and `max_degree = 4` cut runs, up to
   694 tower pairs and 3.5M multiply-adds a run.
6. `a_degree_one_tower_pair_is_the_cube_rules_degree_two_row`: `y_0 - 1` on a
   radix-3 tower (`p = 19`): the first step has degree 3 with one tower pair,
   one solution; `y_0 + 1` is refuted.
7. `radix_3_towers_refute_exactly_the_empty_systems_and_count_the_rest`:
   `3^t` for `t = 1..4` (`p = 19, 109, 109, 163`), planted and unplanted.
8. `mixed_radix_decomposition_systems_agree_with_exhaustive_search_and_the_generic_f4`:
   nine `(p, radices)` pairs (`[2,3,2]`, `[3,2,2]`, `[2,2,3]`, `[3,3,2]`,
   `[2,3,3]`, `[3,3,3]`, `[3,3]`, `[3,2]`), `m = 2, 3` blocks, planted and
   unplanted: solutions equal the exhaustive search, the staircase counts them,
   and (up to 7 variables) the monomial ideal equals the generic F4's.
9. `the_radix_4_control_has_the_radix_2_solution_set`: the same random
   equations in `x` on the radix-2 presentation (`2t` levels) and the radix-4
   presentation (`t` levels, `y^4 = y_next`), `p = 17, 97, 257`, `t = 2, 3`:
   identical solution set in `x`, identical counts, and every radix-2 solution
   restricted to its even levels is the radix-4 solution (the ideal of the
   radix-4 ring is the radix-2 ring's with the odd levels eliminated).
10. `a_radix_3_dickson_tower_agrees_with_exhaustive_search`: `y_{j+1} = y_j^3 - 3 y_j`
    (rule `c = [c_top, 3, 0]`, `b = [1, 0, 0]`), `p = 109, 163`: exercises the
    `y_j` and `y_next` terms; exhaustive search and generic F4 agree.
11. `the_run_does_not_depend_on_the_product_cache` (cache sizes 0, 37, default),
    `a_free_variable_is_eliminated_like_a_chain_link_on_a_radix_3_tower`,
    `the_stop_and_the_degree_bound_behave_like_the_radix_2_engines`,
    `the_ring_check_refuses_what_the_engine_does_not_handle`.

**Result** (final binary, `cargo test --lib f4_fp_tower_radix`): 15 passed,
0 failed, 2.5 s. (Test 5 is two tests: Kummer/general rules ending in
refutations, and planted consistent systems with a complete and a
degree-cut run.)

## 6. Smoke outputs (engineering only, not results)

Printed by the tests with `--nocapture`. `D` is `solving_degree_max`, in the [pcc:unverified]
unit of the presentation: the exponent sum, so a radix-4 digit counts up to 3.
Seeded toy instances: 1 to 3 random raw equations of degree <= 2 in each `x_i`
(<= 3 in the control), planted or not, `m` blocks as listed. They show the engine solves
and cross-checks; they say nothing about `D` as a function of radix at the
experiment's sizes. (In the control table `D` radix-4 is `D` radix-2 plus 2 on
every row; that is how the two gradings count the same toy equations, a
bookkeeping artefact of exponent sums with digits up to 3, not a finding.)

Mixed and radix-3 decomposition systems (test 8; `D` of the radix engine; solutions = exhaustive count):

| p | radices | m | equations | planted | solutions | D |
|--:|:--|--:|--:|:--|--:|--:|
| 13 | [3, 2] | 2 | 1 | yes | 2 | 5 |
| 13 | [3, 2] | 2 | 1 | no | 2 | 4 |
| 13 | [3, 2] | 3 | 2 | yes | 4 | 6 |
| 13 | [3, 2] | 3 | 3 | yes | 1 | 5 |
| 19 | [3, 3] | 2 | 1 | yes | 4 | 3 |
| 19 | [3, 3] | 2 | 1 | no | 4 | 5 |
| 19 | [3, 3] | 3 | 2 | yes | 1 | 6 |
| 19 | [3, 3] | 3 | 3 | yes | 1 | 5 |
| 37 | [2, 3, 2] | 2 | 1 | yes | 2 | 5 |
| 37 | [2, 3, 2] | 2 | 1 | no | 8 | 4 |
| 37 | [2, 3, 2] | 3 | 2 | yes | 2 | 5 |
| 37 | [2, 3, 2] | 3 | 3 | yes | 1 | 4 |
| 37 | [3, 2, 2] | 2 | 1 | yes | 6 | 4 |
| 37 | [3, 2, 2] | 2 | 1 | no | 0 | 4 |
| 37 | [3, 2, 2] | 3 | 2 | yes | 2 | 5 |
| 37 | [3, 2, 2] | 3 | 3 | yes | 1 | 5 |
| 61 | [2, 2, 3] | 2 | 1 | yes | 4 | 4 |
| 61 | [2, 2, 3] | 2 | 1 | no | 4 | 4 |
| 61 | [2, 2, 3] | 3 | 2 | yes | 1 | 5 |
| 61 | [2, 2, 3] | 3 | 3 | yes | 1 | 4 |
| 73 | [2, 3, 3] | 2 | 1 | yes | 2 | 5 |
| 73 | [2, 3, 3] | 2 | 1 | no | 6 | 5 |
| 73 | [2, 3, 3] | 3 | 2 | yes | 4 | 5 |
| 73 | [2, 3, 3] | 3 | 3 | yes | 1 | 4 |
| 73 | [3, 3, 2] | 2 | 1 | yes | 6 | 5 |
| 73 | [3, 3, 2] | 2 | 1 | no | 4 | 4 |
| 73 | [3, 3, 2] | 3 | 2 | yes | 3 | 5 |
| 73 | [3, 3, 2] | 3 | 3 | yes | 1 | 5 |
| 109 | [3, 3] | 2 | 1 | yes | 1 | 4 |
| 109 | [3, 3] | 2 | 1 | no | 0 | 4 |
| 109 | [3, 3] | 3 | 2 | yes | 1 | 4 |
| 109 | [3, 3] | 3 | 3 | yes | 1 | 5 |
| 109 | [3, 3, 3] | 2 | 1 | yes | 9 | 5 |
| 109 | [3, 3, 3] | 2 | 1 | no | 7 | 5 |
| 109 | [3, 3, 3] | 3 | 2 | yes | 2 | 6 |
| 109 | [3, 3, 3] | 3 | 3 | yes | 1 | 5 |

Radix-4 control (test 9; same equations in `x`; `D` is the exponent sum in each presentation; equal solution counts, identical solution sets):

| p | radix-4 levels | m | trial | solutions | D radix-2 (2t levels) | D radix-4 (t levels) |
|--:|--:|--:|--:|--:|--:|--:|
| 17 | 2 | 2 | 0 | 13 | 4 | 6 |
| 17 | 2 | 2 | 1 | 14 | 5 | 7 |
| 17 | 2 | 2 | 2 | 19 | 5 | 7 |
| 17 | 2 | 2 | 3 | 14 | 5 | 7 |
| 17 | 2 | 2 | 4 | 13 | 5 | 7 |
| 17 | 2 | 2 | 5 | 18 | 5 | 7 |
| 97 | 2 | 2 | 0 | 2 | 4 | 6 |
| 97 | 2 | 2 | 1 | 2 | 4 | 6 |
| 97 | 2 | 2 | 2 | 5 | 5 | 7 |
| 97 | 2 | 2 | 3 | 1 | 4 | 6 |
| 97 | 2 | 2 | 4 | 0 | 4 | 6 |
| 97 | 2 | 2 | 5 | 4 | 4 | 6 |
| 257 | 2 | 2 | 0 | 3 | 5 | 7 |
| 257 | 2 | 2 | 1 | 1 | 4 | 6 |
| 257 | 2 | 2 | 2 | 3 | 4 | 6 |
| 257 | 2 | 2 | 3 | 2 | 4 | 6 |
| 257 | 2 | 2 | 4 | 1 | 4 | 6 |
| 257 | 2 | 2 | 5 | 1 | 5 | 7 |
| 257 | 3 | 2 | 0 | 16 | 5 | 7 |
| 257 | 3 | 2 | 1 | 18 | 5 | 7 |
| 257 | 3 | 2 | 2 | 25 | 5 | 7 |
| 257 | 3 | 2 | 3 | 17 | 5 | 7 |
| 257 | 3 | 2 | 4 | 20 | 4 | 6 |
| 257 | 3 | 2 | 5 | 16 | 4 | 7 |

Free variable on a radix-3 tower (test 11, `p = 19`, `u = x_1 x_2`, `u^2 = 5`): 9 solutions, D 4.

## 7. How the experiment uses it

`f4_radix(&inputs, &RadixRing, &TowerF4Options)` mirrors `f4_tower`;
`RadixF4Report` carries the same counters (`solving_degree_max`, `muladds`,
`max_cols`, `max_nnz`, per-step `StepTrace`). Build a radix-3 or mixed tower
with `PowRule::kummer(radix, next)` per level; a Kummer coset `g * mu_N`,
`N = prod r_j`, needs `N | p - 1` and has `y_0 = x / g`. The radix-4 control is
the same `V` and the same equations with levels `y_0, y_2, y_4, ...`. Pass gate
named in the barrier map (the radix-4 presentation reproduces the radix-2
solution sets on every committed round-2 system) is exactly test 9 on toy
sizes; it still has to be run on the committed round-2 systems by the
experiment's own task card. Under AGENTS.md section 3 the discriminator is a
prediction of null until its falsifier fires; this note builds the instrument
and makes no claim about it.

## 8. Reproduce

```bash
export CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_INCREMENTAL=0
cargo test --lib -j2 f4_fp_tower_radix                   # new tests
cargo test --lib -j2 f4_fp_tower:: f4_fp:: sig_fp_tower f4_tower_generic pkm
cargo run --example perfbench -- run --filter fp_gb/ --samples 1 --min-samples 1
```
