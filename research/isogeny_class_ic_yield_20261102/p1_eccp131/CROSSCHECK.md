# P1 follow-up: Rust port of the construction scripts, and independent cross-check of the Macaulay/XL engine

Written 2026-11-02 by the P1 follow-up builder. Nothing here is a measurement run of the P1 study: no
`run` rows were produced, the full measurement was not launched, and no claim about ECCp-131's difficulty
follows from it. Branch `claude/isogeny-class-ic-yield-20261102`, not committed.

**Verdicts.**

* **A (Python removal): done.** The three construction scripts are now Rust subcommands (libpari 2.17.2 through
  dlopen); `targets.json` is reproduced byte-for-byte, `neighbours.json` and `tower.json` are reproduced to the byte
  except wall-clock timing fields. The `.py` files are deleted.
* **B (engine cross-check): the engine agrees.** On 252 systems (7 primes from 2^14 to 2^61-1, m2t3 / m3t2 plus 3 m2t4,
  null, planted, abscissa-planted and ladder targets) there is **no disagreement**: no solution-set difference, no
  quotient-dimension difference, no inconsistency difference, no per-degree rank/pivot/staircase difference against an
  independent dense elimination. **Two limits are material and are stated first:**
  1. **`f4_fp` does not run at p >= 2^32** (it panics in `Barrett::new`: "the prime must be below 2^32"; the module
     doc says "Small primes (p < 2^32)"). The assignment's "p < 2^63" is wrong for the code as committed. The comparison
     of the engine **with F4** therefore covers primes below 2^32 only (147 systems at p = 2^32-5, 2^31-1, 1048571,
     16363). The three primes above 2^32 (2^61-1, 288230376151711687 ~ 2^58, 35184372088763 ~ 2^45; 105 systems) are
     checked against the exact enumeration and the dense elimination, **not against F4**. The repository's other
     64-bit F4, `f4_prime_dynamic` (reached as `f4_tower_generic::dynamic`, p <= 2^61-1), was tried on all of them and
     stops with `WorkLimit` after 3 steps (its hard cap is 16,777,216 modelled work units, not raisable), so it gives no
     basis; the tower-ring F4 `f4_tower_generic` (the engine the repo's PKM pilots use, which takes these systems in
     its quotient-ring form) was **not** wired up. See "Not done".
  2. **The two degrees are not comparable by a theorem, and PROTOCOL.md's relation holds only empirically.** See section B3.

## A. Rust port of `build_neighbours.py`, `build_tower.py`, `make_targets.py`

Code: `rust/src/pari.rs` (dlopen binding, the `p2tool/src/pari.rs` mechanism plus library-mode string evaluation),
`rust/gp/construct.gp` (the GP source evaluated by libpari), `rust/src/construct.rs` (subcommands `build-neighbours`,
`build-tower`, `make-targets`, `jsoncmp`; SHA-256 seeding, ICV1 identities, JSON in Python's `json.dump(indent=1)` layout
with exact big integers, `rust/src/jsonv.rs`). libpari: `P1_LIBPARI` or `--libpari` (here the cypari2 venv's
`cypari2.libs/libpari-3a78ce10.so.2.17.2`, PARI 2.17.2). `run_full.sh inputs` rebuilds and compares all three.

Comparison rule: **canonical JSON** = parsed, object keys sorted, compact, floats re-printed from their value,
with the named timing keys dropped at any depth; `p1_eccp131 jsoncmp A B --skip ...` prints the raw SHA-256 of each file,
the canonical SHA-256, and `byte_identical` / `canonical_identical`. The Rust writer keeps Python's key insertion
order and layout, so the files also match line by line.

| file | committed (Python) raw sha256 | Rust raw sha256 | byte-identical | canonical sha256 (both sides) | skipped keys |
|:--|:--|:--|:--|:--|:--|
| `targets.json` | `95a71df2493f7cdc91c64eac12637b58dfaa7af064300875705cc7d369fe05ff` | `95a71df2493f7cdc91c64eac12637b58dfaa7af064300875705cc7d369fe05ff` | **yes** | `4c9ec0c0e53bf6fc63bf67681a26d443871b3e143994233af4ba887a7566a896` | none |
| `neighbours.json` | `0b654414221c62c88cedbcc63d0e2cca2ab88a12d4aa4cc8f6c6b9084a4dd805` | `db759c9d1563c2df6466e6b5f1647455a5c13e79059fa2e38de6a13429b97d97` | no (timings only) | `2bec5f858f29e6e137ef80f45766e4e4c27d9e5cd1124b623743514f44616f3c` | `timing_s`, `wall_s`, `build_total_s_sum` |
| `tower.json` | `b82248b83e987e6b375ea1c15e5cc41b346894097c24e97cb2b6e01ef01a19d9` | `6de88d6c769565bd4137b101d30edb4e6b9b532b4b631b9ef6845257189b9afb` | no (one line) | `87161807b2205e168d153e9f52a01c4d3f14c700ed6bb9f4f0831c3de866f7e1` | `wall_s` |

* `tower.json`: `diff` shows exactly one differing line, `"wall_s"`.
* `neighbours.json`: with the lines of the nine timing keys removed from both files, `diff` is empty (3 min 56 s build in
  Rust vs 181 s in Python; all 27 neighbours, all checks true, same slugs, kernel polynomials, images).
* `targets.json` was built from the committed `neighbours.json` (32 targets x 31 arms); the canonical hash of the Rust
  `neighbours.json` equals that of the committed one, so the input is interchangeable.
* Copies of the Rust outputs and the build log: `crosscheck/results/part_a/`.
* **Fragility, recorded in PROTOCOL.md section 13:** PARI's `ellgroup(E,,1)` picks its generator from PARI's random
  state. The committed tower is reproduced only if no other randomised PARI call (`ellcard`, `isprime`) runs before the
  first `ellgroup`. My first Rust attempt called `P1init` (which runs `ellcard` and `isprime`) first and produced a
  different `t = 5` coset (`V_sorted` differed for t = 5 only; t = 2..4 matched); `build-tower` now sets only `p` before the
  search and reproduces the file.
* Unit tests (`cargo test --release`): decimal/hex round trip, the ICV1 identity of E0 against the values
  `scripts/curve_id.py` produced (slug, `model_sha256`, `icv1`), and the JSON layout.
* Python removed: `build_neighbours.py`, `build_tower.py`, `make_targets.py` deleted (working tree; `build_neighbours.log` is
  the historical Python log and is kept); PROTOCOL.md (sections 1, 4, 6, 12, 13) and `run_full.sh` updated.

## B. Cross-check of `rust/src/engine.rs` against the repository's F4

### B1. What was compared, and how

Crate `crosscheck/` (`p1_crosscheck`, depends on the repository crate by path). It compiles the P1 binary's own
`main.rs` (hence `build_system`, `assignment`, `count_solutions`, the Montgomery field, `engine.rs`, all **unchanged**
apart from `pub(crate)` visibility on eleven items of `main.rs` and `pub(crate) mod` on four module declarations) and
runs the same polynomial systems through the repository's F4 `f4_fp::f4` (primary reference, p < 2^32). The field
modulus is a runtime parameter of the engine (`fp::set_modulus`, 3 mod 4), one prime per process.

* **Systems.** The same PKM isogeny tower (`build-tower --p P`: auxiliary curve with a cyclic 2^4 subgroup, chain of Velu
  2-isogenies, all asserted inside the GP code) and chained Semaev S3 system as P1: m2t3 (6 unknowns, 7 equations),
  m3t2 (7, 8), and for one prime m2t4 (8, 9). Three random curves per prime and config. Target kinds:
  `null` (abscissa of a random curve point), `planted` (m distinct factor-base points with a point on the curve, signs
  random, the sum is the target; the planted assignment is asserted to zero every equation), `abscissa` (consistent
  but not point-based: a tower abscissa pair/triple and a root of S3, so K >= 1 without rational y), `ladder` (planted
  plus one random quadric through the planted point, as P1's positive control).
* **Primes** (all 3 mod 4, all < 2^61): 2305843009213693951 (2^61-1), 288230376151711687, 35184372088763, 4294967291
  (2^32-5, the largest `f4_fp` accepts), 2147483647, 1048571, 16363.
* **Solution set (exact).** Ground truth by enumeration of the tower: every variable is determined by the tower
  abscissas (u_0 = x of each element, the rest follow from the chain; for m = 3 the middle variable w is a root of a quadratic),
  so all F_p-solutions are enumerated from V (S3 as a quadratic in its last argument, closed form); the planted instances additionally verify S3 against
  elliptic-curve addition (the planted assignment must zero every equation).
  This truth set is independent of both solvers. F4 side: a substitution tree over `f4_fp::f4` (one F4 per node, a
  univariate in the last variable, roots by the repository's Cantor-Zassenhaus `glv_invariant_base::poly_roots_fp_counted_until`),
  candidates re-evaluated on the input. `f4_fp::solve` (the repository's own substitution solver) cannot be used
  directly above ~2^22: it finds roots by scanning 0..p and `solve_visit` refuses p >= 2^32; for p <= 2^22 it is run as
  well and agrees with the tree (69 systems). The engine emits **counts, not points** (design), so for the engine the
  comparison is of `n_standard_monomials_final`.
* **Dimension of the quotient.** Standard monomials of the leading monomials of the F4 reduced basis (enumerated in the
  box fixed by the pure powers), against `count_solutions` K (solutions over the closure) and the engine's `n_std`.
* **Degrees.** Engine `D_solve` (refutation degree when K = 0); F4 `solving_degree` (degree of the step at which 1 or the
  last new element appeared), `solving_degree_max` (highest step degree at which something new was learned),
  `degree_reached` (highest pair degree processed).
* **Rank profile.** An independent dense elimination (own monomial enumeration, own degrevlex comparator, own arithmetic,
  full Gaussian elimination with columns descending) of the same Macaulay matrix, run for every degree whose matrix has
  at most 2000 columns (D <= 7 for 6 unknowns, D <= 6 for 7, D <= 5 for 8). Compared per degree with the engine: rows
  (cumulative), columns, rank, new pivots, new pivots with leading degree < D, zero reductions, number of standard
  monomials, variables with a pure power, whether the constant is a pivot. (Operation counts, `nnz_pivots` and the
  pivots' row contents are elimination-order dependent and are not compared.) F4 has no Macaulay matrix, so "rank profile
  per degree" is not comparable with F4; the comparable F4 quantity is the final quotient dimension and the staircase size.

### B2. Results

Rows: `crosscheck/results/rows_<p>.jsonl` (one JSON row per system: all fields above, per-degree engine/dense data),
`log_<p>.txt`, `summary_<p>.json`, `summary_table.tsv` (from `summarize.sh`); towers in `results/towers/`.

| quantity | result |
|:--|:--|
| systems | 252: m2t3 141, m3t2 108, m2t4 3; null 85, planted 55, abscissa 84, ladder 28 |
| systems with any failure | **0** |
| engine status | `solved` on all 252 (no cap, no `contradiction_with_K`) |
| F4-comparable systems (p < 2^32) | 147 |
| solution set: enumeration == F4 tree | **130 / 130** non-ladder systems (49 with the empty set, 81 with 1..13 solutions; 18 m3 systems have fewer F_p-rational solutions than K, the rest have w in F_p^2 and F4 and enumeration agree on the rational ones) |
| ladder: F4 finds exactly the planted point | 17 / 17 (dimension 1) |
| quotient dimension F4 == K (`count_solutions`) | 130 / 130 (ideal radical; the engine's termination criterion `n_std = K` is therefore sound on these shapes) |
| engine `n_std` == F4 dimension == K | 147 / 147 consistent systems (K = 0: constant pivot in all) |
| inconsistency flag engine == F4 | 147 / 147 |
| `f4_fp::solve` == substitution tree (p <= 2^22) | 69 / 69 |
| dense reference vs engine, per degree | 126 systems, **688 (system, degree) pairs, 0 mismatches** (rows, cols, rank, new pivots, new pivots below D, zero reductions, n_std, pure powers, constant pivot) |
| p >= 2^32 (105 systems; 94 non-ladder) | engine `n_std` == K on all 94 (36 null refutations included); dense reference: 52 systems agree, 0 mismatch; F4: not available |

(Exact counts per kind are in `results/summary_table.tsv` and `rows_*.jsonl`; the table above states the totals computed by
`summarize.sh` and a `jq` pass over the rows.)

Degrees, p < 2^32 (F4 columns from `f4_fp`; each cell is the set of values over the n systems; K and dimension are
identical across the compared quantities):

| config | kind | n | engine D_solve | F4 solving_degree | F4 solving_degree_max | F4 degree_reached | K = F4 dim = engine n_std |
|:--|:--|--:|:--|:--|:--|:--|:--|
| m2t3 | null | 24 | 10 | 2 | 5 | 5 | 0 |
| m2t3 | planted | 22 | 9 | 2 | 5 | 5 | 2 |
| m2t3 | abscissa | 24 | 9 | 2 | 5 | 5 | 1 or 2 |
| m2t3 | ladder | 11 | 6 | 4 | 4 | 4 | 1 (dim; K of the base system is not the ladder's) |
| m3t2 | null | 24 | 10 | 2 | 6 | 6 | 0 |
| m3t2 | planted | 10 | 10 | 2 | 6 | 6 | 6 |
| m3t2 | abscissa | 24 | 10 | 2 or 3 | 6 | 6 | 1, 3, 6 or 13 |
| m3t2 | ladder | 5 | 7 | 4 | 5 | 5 | 1 |
| m2t4 | null / planted / ladder | 1 each | 12 / 11 / 7 | 3 / 3 / 3 | 5 / 5 / 5 | 5 / 5 / 5 | 0 / 2 / 1 |

### B3. Degree semantics and the expected relation

* **F4 degree** = degree of the step matrix (lcm degree of the critical pairs processed; all rows have that degree).
  The normal strategy climbs, learns elements, and may descend again when an element's leading degree is below its step
  degree ("degree fall"). **Engine D** = the Macaulay degree at which the span of all rows `mu f_i` of degree <= D has a
  staircase with exactly K standard monomials and a pure power per variable (or contains 1 when K = 0).
* **What is provable.** Every row F4 uses is a multiple of a basis element; a basis element created at step d lies in the
  span of Macaulay rows of degree <= d. Multiplying it by a monomial of degree d' - deg(element) (a later, lower step) needs
  rows of degree up to d + (d' - deg) in the Macaulay matrix. So F4's top step degree is **not** an upper bound of the
  engine's D in general, and the engine's D is **not** a lower bound either. The two coincide when no degree falls occur
  (regular sequences). These systems fall (first fall degree observed by the engine: 5 on every null/planted system, 4 on
  ladders).
* **What is observed.** `D_engine > F4 top step` on **147 / 147** comparable systems, never equal, never below
  (D_engine 6..12 vs F4 4..6; the null/planted ratio is about 2). This agrees with the P1 PROTOCOL's statement that the
  engine's D "is an upper bound on F4's degree" **as an observation on these shapes, not as a theorem**; the protocol text
  should say so. It also means engine `D_solve` must not be read as F4's solving degree or as a time/degree-of-regularity
  statement about F4; the protocol already says "No claim about F4 time is made".
* The check that would have caught a violated expectation is recorded in each row as
  `degree_relation_engine_vs_f4` (`engine_below_f4_top_step` would be written as the note `UNEXPECTED_ORDER`); none occurred.

### B4. Positive control (can the harness see an engine bug?)

`crosscheck/mutation_check.sh` copies the crate to a scratch directory, breaks the copy of `engine.rs`, and runs one prime.
Both mutants are flagged:

* **M1** (wrong sign in the pivot application on columns with `id % 7 == 3`): 6 / 6 systems with failures, dense mismatch at
  every degree from 4, engine `contradiction_with_K` on every consistent system (constant derived although solutions exist).
* **M2** (staircase ignores divisibility by the last variable): 5 / 6 systems with failures, all in `n_std`, at D = 5 and 6
  of the dense reference. **Only the dense reference** sees M2; at the final degree the mutant still reported `n_std = K` on
  the m2t3 planted systems, so the F4/K comparison alone does not catch a staircase error.

(An earlier m2t4 probe ran a stale mutant binary by mistake -- a shared target directory -- and was reported by the harness as a
dense mismatch at D = 5; that is the harness doing its job, the real build agrees on m2t4, and the script now builds mutants
under distinct binary names.)

### B5. Not done / limits

* **No F4 comparison above 2^32** (see top). `f4_tower_generic` (tower-ring F4, p <= ~2^61) was not wired; it would give the
  independent F4 at 2^61 and is the natural next step.
* The engine returns no points, so engine-vs-F4 solution sets are compared by count only; correctness of individual
  solutions rests on enumeration == F4 (for F4) and on the dense/count agreement (for the engine).
* The dense per-degree comparison stops at 2000 columns (D <= 7 / 6 / 5 for 6 / 7 / 8 unknowns); the engine's high-degree
  profile (D up to 12, 8008..19448 columns) is checked only through its end state (n_std = K = F4 dimension, constant
  pivot iff F4 inconsistent) and by the degrees below.
* m3t3 and larger towers were not run (the P1 protocol puts them out of budget); `f4_fp` at 10 unknowns was not tried.
* 3 curves per prime and 2 targets per kind: a sanity cross-check of the instrument, not a statistical study. At the tiny
  primes (2^14, 2^20) no null target hit a decomposition (K = 0 on all 84 null systems).
* Timing is not compared (no claim about F4 vs engine time).
* The 27 neighbour slugs are still not in `docs/curves/registry.json`.

## C. Reproduce

    # Part A (needs libpari 2.17: export P1_LIBPARI=...so.2.17.2)
    cd rust && cargo build --release --locked && cargo test --release
    BIN=rust/target/release/p1_eccp131 ./run_full.sh inputs            # rebuild + jsoncmp the three inputs
    # Part B (builds the whole repository crate once; about 13 min on 4 shared cores)
    cd crosscheck && cargo build --release
    BIN_P1=../rust/target/release/p1_eccp131 BIN_XC=target/release/p1_crosscheck ./run_crosscheck.sh results_new
    ./summarize.sh results_new/rows_*.jsonl
    ./mutation_check.sh /tmp/xc_mut results/towers/tower_2147483647.json 2147483647   # positive control

Note: the header comment of `rust/src/engine.rs` still says `f4_fp` "is limited to p < 2^63"; the file was left byte-identical
(it is the instrument under test), the correct bound is 2^32.
