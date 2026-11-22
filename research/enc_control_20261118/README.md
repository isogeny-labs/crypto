# Encoding control: is the subgroup factor base easy, or only cheaply encoded? (2026-11-18)

**Every number here is `[pcc:unverified]` pilot data.** No pcc claim, no study
result, no leaderboard or `benchmarks/` row, no `S`, no rho comparison. m = 2,
p = P1 = 2013265921, one curve, local shared 4-vCPU box (load 3.0–26 during the
run; the metrics are deterministic, wall times are context only).

- Preregistration, frozen before data: [`PREREGISTRATION.md`](PREREGISTRATION.md)
  (commit `36e5cc3912`). Exploratory post-hoc arm, prediction written before it
  ran: [`ADDENDUM_cosetunion.md`](ADDENDUM_cosetunion.md) (commit `3140d465f0`).
- Instrument: [`examples/enc_control.rs`](../../examples/enc_control.rs) (built by
  [`harness/`](harness/)), F4 = `src/cryptanalysis/f4_fp.rs`, HKY =
  `src/cryptanalysis/hky_fp_tower.rs`. Main-run binary SHA-256 in
  [`binary.sha256`](binary.sha256); the addendum binary (adds only the
  `cosetunion` arm and a `closing_degree` field) is
  `96b55d953b2d3af126d202c0f19460ebfe3590c93b9018269fecf0b2e2abb24d`.
- Raw rows: [`raw/rows.jsonl`](raw/rows.jsonl) (main, 137 verified encodings,
  408 F4 systems; a planted target is skipped when `F` holds fewer than two curve abscissae), [`raw/addendum_rows.jsonl`](raw/addendum_rows.jsonl), logs
  `raw/progress.txt`, `raw/addendum_progress.txt`. Scripts: [`run.sh`](run.sh),
  [`run_addendum.sh`](run_addendum.sh).
- Figure: [`fig_enc_control.pdf`](fig_enc_control.pdf) (pgfplots,
  [`make_pdf_fig.sh`](make_pdf_fig.sh)) and [`fig_enc_control.svg`](fig_enc_control.svg)
  (Rust, `examples/enc_control_svg.rs`, [`make_fig.sh`](make_fig.sh)); data
  [`fig_data.tsv`](fig_data.tsv). No PNG: this box has no SVG rasterizer.

## The control

For a set `F ⊂ F_p`, `|F| = 2^k`, every tower arm uses the same unknowns
`u_0..u_{k−1}` per summand (`x = u_0`), the same chain `u_j² = u_{j+1}`, the same
`S₃(x_1, x_2, x_R)`, the same curve and the same random targets. Only the closing
relation differs: `u_{k−1}² = R_F(u)`, `R_F = x^{2^k} mod ∏_{f∈F}(x − f)` written
in u-monomials (`x^n = u^{bits(n)}`). `R_F = 1` for the subgroup `H`, a constant
for a coset, and a dense multilinear polynomial of degree `k` (all `2^k`
coefficients non-zero) for a random set.

**The encoding is correct (exact computation, every cell).** For each of the 203
encodings built (main + addendum): all `2^k` points of `F` satisfy the block;
the F4 quotient of the block has dimension exactly `2^k`; the univariate
eliminant has exactly `2^k` roots in `F_p`. So the variety under `x = u_0` is
exactly `F`, and reduced. **Negative control:** replacing `R_F` by `R_F + 1`
(bits: `P_F + 1`, single: `∏ + 1`) leaves none of the points of `F` on the
variety in every cell; it was detected 203/203 times. The tower-shaped encoding
of an arbitrary set makes sense and computes.

## Results — `D` = F4 solving degree on random (inconsistent) targets

All 2 random targets per cell were inconsistent (F4 derived 1). `D` on the
planted target equals the random-target `D` in every finished cell. Ops = counted
`F_p` multiplications (`field_ops`) for one random target. "×H" = ops / tower-H ops
at the same k. Draws: tower-rand, coset, bits-rand 3 each; dose 2 (k ≤ 6) / 1
(k ≥ 7); **A/A spread of `D` is 0 in every cell**; ops agree across the draws of
tower-rand, coset and bits-rand to ≥ 10 significant figures (shape-determined); dose draws differ by ≤ 5% at r = 2 (which points are replaced).

| k | \|F\| | tower-H `D` (HKY `d_F`) | coset `D` (HKY) | tower-rand `D` | tower-rand ops ×H | dose r=1 `D` | bits-H `D` | bits-rand `D` | bits-rand ops ×H | randbase `D` |
|--:|--:|:--:|:--:|:--:|--:|:--:|:--:|:--:|--:|--:|
| 2 | 4 | 4 (4) | 4 (4) | 4 | 1.9 | 4 | 4 | 4 | 3.6 | 6 |
| 3 | 8 | 4 (4) | 4 (4) | 4 | 1.8 | 4 | 5 | 5 | 8.1 | 10 |
| 4 | 16 | 4 (4) | 4 (4) | 5 | 2.0 | 5 | 5 | 5 | 21 | 18 |
| 5 | 32 | 4 (4) | 4 (4) | 5 | 1.7 | 5 | 6 | 6 | 44 | 34 |
| 6 | 64 | 5 (5) | 5 (5) | 6 | 1.7 | 6 | 7 | 7 | 121 | 66 |
| 7 | 128 | 5 (5) | 5 (5) | 7 | 2.3 | 7 | 8 | 8 | 101 | 130 |
| 8 | 256 | 5 (5) | 5 (5) | 8 | 1.8 | 8 | censored (300 s) | censored | — | 258 |

tower-H ops: 2.1e3, 4.0e4, 6.6e5, 7.9e6, 1.1e8, 9.9e8, 1.0e10 at k = 2..8.
HKY first fall is 4 for tower-H and coset at every k. (The F4 Hilbert
first-fall diagnostic reads 1 for every arm on planted targets and is
uninformative here; it compares against a semi-regular sequence these
overdetermined systems are not.)

**Gate passed.** tower-H reproduces `d_F` = 4, 4, 4, 4, 5 at k = 2..6 (the
earlier table's values, now at P1 and with `x = u_0` rather than a scaled
coset), and F4 `D` = HKY last fall on all 14 tower-H and 42 coset random-target
systems.

### Dose-response (H with r points replaced)

`D` is **all-or-nothing**: at every k ≥ 4, one replaced point already gives the
full tower-rand `D` (k = 6: 5 → 6 at r = 1; k = 7: 5 → 7; k = 8: 5 → 8), and it
stays there for every r up to `2^k`. At k ≤ 3 nothing moves (`D` = 4 is set by
`S₃`'s degree). The reason is visible in the encoding: `∏(x − f)` for
`H` minus one point plus another is `(x − f')(x^{2^k} − 1)/(x − h)`, whose
`R_F` already has all `2^k` coefficients non-zero (logged as
`closing_terms_nonzero` = `2^k` for every dose cell).

The **operation count** rises gradually instead (panel b): at k = 8, ops ×H =
1.21 (r = 1), 1.31 (2), 1.68 (4), 1.74 (8), 1.76 (r ≥ 16 = tower-rand); at k = 7,
1.47, 1.64, 2.27, 2.29, 2.33 …; it saturates once r ≈ 4–16.

### Exploratory: coset unions dial the closing degree (not preregistered)

`F` = union of `2^j` random cosets of the subgroup of order `2^{k−j}`: not a
subgroup, not a coset (for 0 < j), but `R_F` lives in `u_{k−j}..u_{k−1}` and has
degree exactly `j` (logged). Same instrument, 2 draws (k ≤ 7), 1 (k = 8), A/A
spread 0.

| k | j = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|
| 4 | 4 | 4 | 4 | 4 | 5 | | | | |
| 5 | 4 | 4 | 4 | 5 | 5 | 5 | | | |
| 6 | 5 | 5 | 5 | 5 | 5 | 5 | 6 | | |
| 7 | 5 | 5 | 5 | 5 | 5 | 5 | 6 | 7 | |
| 8 | 5 | 5 | 5 | 5 | 5 | 6 | 6 | 7 | 8 |

Prediction `D = max(D_H(k), j)` up to +1: held in all 35 cells (+1 at k = 4, j = 4;
k = 5, j = 3; k = 8, j = 5). A set with `2^7` arbitrary-looking coset
representatives at k = 8, j = 7 behaves exactly like a random set of the same
closing degree; a union of 16 cosets (j = 4) behaves exactly like `H`.

## Predictions vs outcome

| arm | prediction (prereg §6) | outcome |
|:--|:--|:--|
| tower-H | 4,4,4,4,5; ≤ 6 to k = 8; HKY = D | **held** (…, 5, 5, 5; HKY = D everywhere) |
| coset | = tower-H | **held exactly** (same `D`, same ops to the unit) |
| tower-rand | grows with k, in [k+2, 2k+4] | **grows, but lower than predicted**: `D = max(4, k)` for k ≠ 4 (5 at k = 4), i.e. at the bottom edge of my range minus 2 |
| dose | jump at r = 1, then flat | **held** for `D`; ops rise gradually instead |
| bits-H ≈ bits-rand, above tower-H | | **held**: `D = k + 1` for both, ops equal within 4% for k ≥ 5 |
| randbase | `2^k + 2` | **held** |

## Interpretation

1. **The tower-shaped encoding of a random set does not have low `D`.** Under the
   preregistered rule this is the "exceeds tower-H and grows with k" outcome:
   tower-rand `D` = 8 vs tower-H 5 at k = 8, gap 0, 0, +1, +1, +1, +2, +3 over
   k = 2..8. Having log-many variables and a quadratic chain is **not
   sufficient**; the random set's encoding is exponentially better than the
   single polynomial (`D` = k vs `2^k + 2`) but still not flat.
2. **But what separates them is the degree of the encoding, not
   subgroup-ness.** In every arm `D` sits at the largest input degree plus at
   most 2: `S₃` (degree 4) for tower-H / coset, the degree-`k` closing relation
   for tower-rand and dose, the degree-`k` interpolant (+1) for bits, the
   degree-`2^k` polynomial (+2) for randbase. The coset-union dial shows the same
   thing on sets that are **not** subgroups or cosets: `D` follows the closing
   degree `j` and nothing else. And in the bits encoding, where `H` and a random
   set both have a degree-k interpolant, `H` has no advantage at all (same `D`,
   same ops). So, at this scale and with this instrument, **no evidence of
   structure beyond the encoding's degree**: the subgroup is easy *because* its
   tower-shaped closing relation has degree 0 (equivalently: `x^{2^k}` is constant
   on `H`).
3. **The two readings of the question collapse into one statement.** "Has a cheap
   encoding" has to mean "has a closing relation of low degree in the
   tower variables", and that is itself a strong algebraic property: `R_F` has
   degree ≤ j iff every non-zero coefficient of `∏(x − f)` below `x^{2^k}` sits
   at an exponent with at most j binary ones (a sparsity condition on the
   coefficients of the set's vanishing polynomial). Coset unions of
   `H_{k−j}` meet it (only exponents divisible by `2^{k−j}` occur); a random set
   does not (all `2^k` coefficients non-zero). The dose-response shows how fragile
   it is: one point out of 256 replaced destroys it completely (`D` 5 → 8).
4. **Cost is a softer signal than `D`.** The operation ratio tower-rand / tower-H
   is only 1.7–2.3× through k = 8, even though `D` climbs from 5 to 8: the
   high-degree steps of the random arm are few and narrow at these sizes. Whether
   the `D` gap turns into an exponential cost gap at larger k is not measured here.

## What this does NOT show

- Anything at m ≥ 3, other primes, other curves (one curve, one prime), or
  beyond k = 8 (`|F|` = 256). No index-calculus cost, no relation yield, no `S`,
  no rho comparison, no security statement.
- That *some other* encoding of a random set could not do better: only three
  encodings were tried (tower-shaped, multilinear bits, single polynomial).
- Every number is `[pcc:unverified]`; claims are pilot readings, not registered
  results. No `experiment.protocol/v1` JSON or unified export was written before
  data; the frozen record is `PREREGISTRATION.md`, and `raw/*.jsonl` is the full
  row export. Analysis was not label-masked (deterministic metric).

## Ceiling

Local, 4 shared vCPU, `RAYON_NUM_THREADS=2`, 300 s per system. Tower arms reach
k = 8 comfortably (one k = 8 tower-rand system: 32–64 s, 1.5–3 min
wall per cell at load 8–25). The bits encoding is the bottleneck: ~150 s per
system at k = 7, censored at k = 8 (300 s, reached degree 9 when stopped).
Main run 1 h 47 min, addendum 13 min. k = 9 tower arms would be roughly
10× k = 8 (ops grow ~10× per k), i.e. ~10 min per system.

## Reproduce

```bash
export CARGO_TARGET_DIR=$PWD/target
(cd research/enc_control_20261118/harness && cargo test --release --example enc_control \
   && cargo build --release --example enc_control --example enc_control_svg)
bash research/enc_control_20261118/run.sh            # main (prereg) run
bash research/enc_control_20261118/run_addendum.sh   # exploratory coset unions
research/enc_control_20261118/make_fig.sh && research/enc_control_20261118/make_pdf_fig.sh
```

Note: the earlier study's raw rows carry `p = 786433` although its README says P1;
this run uses P1 = 2013265921 throughout and reproduces the same tower values.
