# ±P pairing with a shared inversion in the linear 2 + 1 scan (`lin21-ht`)

Local pilot, 2026-11-16, shared 4-vCPU container, single-threaded processes.
Every number below is `[pcc:unverified]`. No benchmark record, leaderboard row
or `benchmarks/README.md` row follows from this, whatever the outcome. Protocol
frozen before the measured run: [`PROTOCOL.md`](PROTOCOL.md), commit
`b2836595af` (sha256 `658fae13…e53d491`). All computation, checks, statistics
and tables are Rust ([`main.rs`](main.rs), [`linear21_pm.rs`](linear21_pm.rs));
[`build.sh`](build.sh) and [`run.sh`](run.sh) only start the compiler and the
binary. Generated: [`tables.md`](tables.md), [`results.json`](results.json);
raw per-target rows: [`raw/`](raw/).

## Result

- **The pairing is not in `lin21-ht`; the library has half of it.** The scan
  arm visits `P₁` and `−P₁` as two separate leaves and inverts their common
  denominator twice. The library's `add_many_lazy_paired_x` /
  `add_many_lazy_soa_paired` already share that denominator, in other scans.
  Nothing existing shares the arm's second inversion (`1/x(Q)`).
- **`lin21-ht-pm` (one shared inversion per pair): fails the pre-registered
  rule.** Whole-leaf counted field operations, baseline `lin21-ht` / PM =
  1.056, 1.053, 1.051 at n = 47, 53, 59; whole-leaf wall `lin21-ht` / PM =
  1.013, 1.000, 0.997, inside the A/A band at all three. `[pcc:unverified]`
- **The library's existing pairing plus the second batched inversion (`LP`):
  inconclusive.** Counted (modelled t stage) 1.082, 1.077, 1.072; wall 1.044,
  1.034, 1.019, outside the A/A band at n = 47 and 53, not at n = 59.
  `[pcc:unverified]`
- **Why:** in the repository arm the t stage is 26 to 29% of the counted field
  operations but 2.4 to 3.5% of the wall time (n = 47 to 59). The wall ceiling
  for any t-stage change is 1.025 to 1.036. The earlier port-based figures
  (t stage 14% of the leaf, whole-leaf wall ratio 1.10 to 1.19) do not carry
  over to the repository arm.
- The repository example `ic_algebraic_floor` built at HEAD `8d039abd94`
  (11 min 09 s), so nothing here is port-based.

## 1. What the existing t stage does

`examples/ic_algebraic_floor/linear21.rs` (sha256 `0554f40f…2068bb22`, copied
unchanged as [`linear21.rs`](linear21.rs)):

- Base and negatives: `points`, `negated` (`linear21.rs:99-100`, `:187`). The
  factor base holds both lifts of every abscissa of `V`, adjacent in base
  order (`P`, `−P`), plus the 2-torsion point `(0, 1)` alone: 524,926 pairs
  and 1 single at `1:59:20`.
- `decide` (`:431-481`), per block of 256 base points (`:438`, `:449`):
  1. `FastCurve::add_many(r, &negated[lo..hi], …)` (`:452-453`): full affine
     `Q_i = R − P_i`, abscissa and ordinate
     (`src/cryptanalysis/koblitz_fast.rs:333-413`). One `Gf2::batch_inv` over
     the 256 denominators `x_R + x_i` (`:387`), then per point one product for
     the slope (`:409`) and one squaring plus one product in
     `add_with_lambda` (`:301-306`).
  2. `Gf2::batch_inv` on the 256 abscissae `x(Q_i)` for `1/t` (`linear21.rs:457-461`).
  3. the leaf `pair_half_trace(q, uinv[off], …)` per point (`:465-476`).
- Both signs are separate leaves. `x(−P₁) = x(P₁)`, so the denominator
  `x_R + x₁` enters the first batch twice and is inverted twice. No sharing.
- Per abscissa `x₁` (two leaves), field operations of the t stage:
  16 products + 2 squarings (2 × (3 + 1 + 1) products and 2 squarings in the
  addition, 2 × 3 products in the second inversion), plus 9 products and one
  true inversion per batch call, i.e. 2 inversions per 256 leaves.
- The inversion is already batched across `x₁` with interleaved chains:
  `Gf2::batch_inv` (`src/cryptanalysis/semaev_decomp.rs:657`) deals the batch
  to 4 chains kept in vector registers (`batch_inv_clmul`, `LANES = 4`,
  `:687-759`), 3 products per element, 9 products and one Itoh-Tsujii
  inversion per call.
- Existing sharing of the denominator between `±P₁`, elsewhere in the library:
  `FastCurve::add_many_lazy_paired_x` (`koblitz_fast.rs:454-514`, scalar; it
  hands over to the eight-lane kernel when that is available, `:463-466`) and
  `add_many_lazy_soa_paired` (`:583-604`, eight-lane AVX-512 kernel
  `fold_into_paired`). Callers: `ic_boundary.rs:8743`, `:8765`;
  `koblitz_index_calculus.rs:7909`; `src/bin/ecbench/pdp3_koblitz.rs:732`,
  `:757`. `add_many_lazy` (`:425-442`) drops the ordinate product.
  `lin21-ht` calls none of them.

## 2. Arms

| arm | t stage per pair, after the batched inversion | inversions per 256 leaves |
|---|---|---|
| `A1`, `A2`: `lin21-ht`, the repository file | 16 mul + 2 sqr | 2 |
| `BC`: same calls, this study's copy of the leaf | 16 mul + 2 sqr | 2 |
| `PM`: `lin21-ht-pm` | 11 mul + 4 sqr | 1 |
| `LZ`: `add_many_lazy` + `batch_inv` | 14 mul + 2 sqr (model) | 2 |
| `LP`: `add_many_lazy_soa_paired` + `batch_inv` | 11 mul + 2 sqr (model) | 2 |

`PM` ([`linear21_pm.rs`](linear21_pm.rs), `pm_t_inline`): with `e = x_R + x`,
`Π = x·x_R`, `g = Π + √b`, the two abscissae `t₁ = x(R − P)`, `t₂ = x(R + P)`
are the roots of `e²t² + Πt + Π² + b`. One batched inverse of `e·g` gives
`1/e = g/(eg)` and `e/g = e²/(eg)`; then `λ = (y_R + y)/e`,
`t₁ = λ² + λ + e + a`, `t₂ = t₁ + Π/e²`, `1/t₁ = t₂·(e/g)²`,
`1/t₂ = t₁·(e/g)²`. `y(Q)` is formed only when a verification needs it. A pair
with `e = 0` or `g = 0`, and the single 2-torsion point, go through one
addition and one inversion each (1 leaf per target in every cell).

In counted operations the library's existing pairing with a second batched
inversion (`LP`, 13 per pair) is below the single-inversion formula (`PM`, 15
per pair): the Vieta route saves one batched element (3 products) and spends
5 operations to recover the three quotients.

## 3. Stage breakdown of the repository arm (measured run, "no" class)

Amdahl ceilings stated before measuring, from the development seed
(`PROTOCOL.md` §7): 1.40 field operations, 1.13 priced, 1.036 wall at n = 47.
Measured:

| n | d | leaves/target | A1 ns/leaf | field ops/leaf | t stage, field ops | t stage, elimination priced | t stage, wall | ceiling: field ops | priced | wall |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 23 | 8 | 237 | 163 | 24.8 | 37.6% | 22.4% | 10.1% | 1.603 | 1.288 | 1.113 |
| 29 | 10 | 1,017 | 261 | 26.9 | 34.8% | 18.6% | 6.1% | 1.535 | 1.229 | 1.065 |
| 31 | 11 | 2,101 | 309 | 31.4 | 30.0% | 15.9% | 4.9% | 1.428 | 1.189 | 1.052 |
| 37 | 13 | 8,191 | 381 | 33.4 | 28.2% | 13.6% | 4.0% | 1.393 | 1.157 | 1.041 |
| 41 | 14 | 16,223 | 397 | 31.0 | 30.5% | 13.5% | 3.9% | 1.440 | 1.156 | 1.041 |
| 47 | 16 | 65,363 | 475 | 33.0 | 28.8% | 11.7% | 3.5% | 1.404 | 1.133 | 1.036 |
| 53 | 18 | 262,523 | 594 | 35.1 | 27.2% | 10.2% | 3.0% | 1.374 | 1.114 | 1.031 |
| 59 | 20 | 1,049,853 | 733 | 37.1 | 25.9% | 9.0% | 2.4% | 1.349 | 1.099 | 1.025 |

The t stage costs 15 to 18 ns per leaf (library code: inlined `pclmulqdq`
products, 4-lane inversion). The rest of the leaf costs 146 to 715 ns: its
`d + 4` field operations of the system build are calls from the example crate, and the `F₂` elimination
and byte-table maps are not field operations. Field operations are therefore a
poor predictor of this arm's wall time.

## 4. Counted operations per target ("no" class, means; `tables.md` §3)

Field operations = `M + S + (n + 8)·I` (primary, per the task). Priced =
`8M + 8S + 8(n+8)I + table lookups + word operations` (the earlier pilot's
price vector, elimination included).

| n | BC mul | BC sqr | BC inv | PM mul | PM sqr | PM inv | BC/PM field ops | BC/PM priced | BC/PM t stage | BC/LP field ops (model) | BC/LP priced (model) | BC/LZ field ops (model) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 23 | 4,873 | 946 | 2 | 4,268 | 1,182 | 3 | 1.0610 | 1.0354 | 1.180 | 1.0951 | 1.0545 | 1.0420 |
| 29 | 22,960 | 4,069 | 8 | 20,378 | 5,085 | 6 | 1.0638 | 1.0332 | 1.208 | 1.0984 | 1.0504 | 1.0387 |
| 31 | 54,759 | 10,494 | 18 | 49,422 | 12,594 | 11 | 1.0562 | 1.0290 | 1.216 | 1.0848 | 1.0432 | 1.0329 |
| 37 | 229,855 | 40,930 | 64 | 209,086 | 49,120 | 34 | 1.0536 | 1.0251 | 1.220 | 1.0804 | 1.0371 | 1.0309 |
| 41 | 431,069 | 64,897 | 128 | 389,932 | 81,119 | 66 | 1.0589 | 1.0252 | 1.223 | 1.0876 | 1.0369 | 1.0334 |
| 47 | 1,867,539 | 261,489 | 512 | 1,701,824 | 326,851 | 258 | 1.0560 | 1.0220 | 1.226 | 1.0819 | 1.0318 | 1.0312 |
| 53 | 8,025,603 | 1,050,148 | 2,052 | 7,360,058 | 1,312,670 | 1,028 | 1.0533 | 1.0193 | 1.228 | 1.0768 | 1.0275 | 1.0294 |
| 59 | 34,194,254 | 4,199,552 | 8,202 | 31,532,709 | 5,249,404 | 4,103 | 1.0509 | 1.0171 | 1.230 | 1.0723 | 1.0239 | 1.0277 |

`BC`'s repository-level counters equal `lin21-ht`'s on all 632 targets, so the
`BC` row is the repository arm's count. The PM inversion counts include the
two single inversions of the 2-torsion leaf. All ratios `[pcc:unverified]`.

## 5. Wall time ("no" class, per-target minimum over repetitions, paired bootstrap 95%; `tables.md` §4, §5)

Whole leaf, baseline `A1` (`lin21-ht`) over arm:

| n | A/A band: A1/A2 | A1/PM | A1/BC | BC/PM | A1/LZ | A1/LP |
|--:|--|--|--|--|--|--|
| 23 | 0.974 [0.966, 0.983] | 0.835 [0.826, 0.845] | 0.829 [0.818, 0.840] | 1.008 [1.001, 1.017] | 0.826 [0.817, 0.835] | 0.840 [0.828, 0.851] |
| 29 | 0.989 [0.975, 1.000] | 0.978 [0.969, 0.987] | 0.964 [0.955, 0.972] | 1.015 [1.004, 1.026] | 1.002 [0.993, 1.010] | 1.014 [1.007, 1.021] |
| 31 | 0.995 [0.989, 1.000] | 0.981 [0.975, 0.987] | 0.975 [0.969, 0.981] | 1.006 [1.000, 1.012] | 1.008 [1.001, 1.014] | 1.012 [1.006, 1.018] |
| 37 | 0.996 [0.993, 0.998] | 0.997 [0.994, 1.000] | 0.989 [0.982, 1.000] | 1.008 [0.997, 1.015] | 1.013 [1.006, 1.018] | 1.026 [1.020, 1.038] |
| 41 | 1.003 [0.995, 1.012] | 1.007 [0.995, 1.021] | 0.998 [0.982, 1.013] | 1.009 [0.998, 1.020] | 1.017 [1.006, 1.027] | 1.039 [1.031, 1.049] |
| 47 | 1.002 [0.996, 1.010] | 1.013 [1.005, 1.023] | 1.006 [0.991, 1.021] | 1.007 [0.996, 1.019] | 1.014 [1.005, 1.023] | 1.044 [1.026, 1.060] |
| 53 | 0.994 [0.984, 1.004] | 1.000 [0.992, 1.007] | 0.992 [0.982, 1.002] | 1.008 [0.999, 1.017] | 1.001 [0.992, 1.008] | 1.034 [1.027, 1.041] |
| 59 | 1.008 [0.999, 1.019] | 0.997 [0.988, 1.008] | 0.982 [0.970, 0.995] | 1.015 [1.005, 1.025] | 1.008 [0.999, 1.019] | 1.019 [1.008, 1.030] |

t stage alone, `BC` t stage over arm:

| n | BC ns/leaf | PM ns/leaf | LZ ns/leaf | LP ns/leaf | BC/PM | BC/LZ | BC/LP |
|--:|--:|--:|--:|--:|--|--|--|
| 23 | 16.6 | 16.5 | 17.5 | 12.5 | 1.006 [1.002, 1.011] | 0.947 [0.933, 0.962] | 1.329 [1.315, 1.343] |
| 29 | 16.0 | 15.1 | 12.7 | 9.1 | 1.056 [1.048, 1.062] | 1.261 [1.249, 1.272] | 1.753 [1.733, 1.775] |
| 31 | 15.1 | 14.1 | 11.3 | 8.2 | 1.075 [1.069, 1.080] | 1.345 [1.313, 1.367] | 1.836 [1.820, 1.848] |
| 37 | 15.1 | 13.7 | 10.2 | 7.6 | 1.104 [1.097, 1.112] | 1.488 [1.474, 1.499] | 2.002 [1.983, 2.023] |
| 41 | 15.6 | 14.0 | 10.7 | 8.2 | 1.116 [1.103, 1.130] | 1.454 [1.429, 1.478] | 1.913 [1.878, 1.949] |
| 47 | 16.7 | 14.2 | 11.8 | 8.9 | 1.179 [1.167, 1.190] | 1.414 [1.395, 1.433] | 1.866 [1.844, 1.887] |
| 53 | 17.7 | 14.8 | 13.3 | 9.1 | 1.196 [1.192, 1.201] | 1.332 [1.322, 1.341] | 1.939 [1.928, 1.950] |
| 59 | 17.9 | 15.1 | 14.0 | 9.5 | 1.188 [1.180, 1.195] | 1.280 [1.267, 1.293] | 1.896 [1.885, 1.906] |

Readings:

- The t stage alone: `BC`/`PM` is 1.18 to 1.20 at n ≥ 47 (counted 1.23).
  `BC`/`LP` is 1.87 to 1.94: the library's eight-lane kernel with its existing
  pairing has the lowest t-stage time of the four, below the scalar
  single-inversion formula.
- Whole leaf: 3 ns saved out of 475 to 733 ns per leaf is inside the A/A
  spread for `PM`. `LP` saves about 8 ns per leaf.
- `BC` is not the repository arm in wall time: `A1/BC` is 0.83 at n = 23 and
  0.96 to 1.01 above. The copied leaf carries a per-target cost that shows at
  237 leaves per target. `BC/PM` (same leaf, different t stage) is 1.006 to
  1.015 everywhere, lower end at or above 0.996.
- The A/A interval at n = 23 to 37 does not contain 1 (0.974 to 0.996): slot
  bias of the same size as the smallest effects. The band used by the rule is
  widened symmetrically for this.

## 6. Decision under the pre-registered rule (`tables.md` §6)

- **`PM` (`lin21-ht-pm`), field operations: fails.** Counted 1.0560, 1.0533,
  1.0509 at n = 47, 53, 59 (≥ 1.05 at all three); wall 1.013, 1.000, 0.997,
  inside the A/A band at all three. `[pcc:unverified]`
- **`PM`, elimination priced: fails**, on both clauses: 1.0171 at n = 59
  (< 1.02) and wall inside the band. `[pcc:unverified]`
- **`LP` (existing library pairing + second batched inversion), either
  counted metric: inconclusive.** Counted 1.082, 1.077, 1.072 (field
  operations, modelled t stage) or 1.032, 1.028, 1.024 (priced); wall outside
  the band at n = 47 (1.044 [1.026, 1.060]) and n = 53 (1.034 [1.027, 1.041]),
  not at n = 59 (1.019 [1.008, 1.030] against a band edge of 1.019).
  `[pcc:unverified]`

The lead as stated (pairing with one shared inversion as a reduction for the
repository's scan arm) is settled negative at this arm's current shape: the
counted reduction is real and about 5% of field operations, 2% with the
elimination priced, and it does not reach the wall clock because the t stage
is 2.4 to 3.5% of it. The gap to matched rho is untouched by this pilot and is
not measured here.

## 7. Gates (`tables.md` §1; all pass)

- Verdict and first witness equal across all six arms on 632/632 targets
  (512 "no", 56 natural "yes", 64 planted); every witness adds up to `R`.
- Full decomposition set of `PM` equals `BC`'s on 632/632; of `LZ` and `LP` on
  480/480 (n ≤ 47; not run at n = 53, 59, per protocol).
- n = 23, 29, 31: the set equals exhaustive pair enumeration on 240/240.
- `BC`'s counters equal the repository arm's on 632/632; `PM`'s leaf counters
  equal `BC`'s on 632/632. All 64 planted targets decided "yes".
- The library's paired kernel ran on every block (361,945 blocks, 0 declined).
- Rerun of cell `1:23:8` after the measured run: all 87 non-timing columns of
  its 80 rows byte-identical.

## 8. Deviations and what was not covered

1. Harness compiled with `rustc` against the cargo-built library `rlib`
   instead of a scratch crate with a path dependency (declared in the
   protocol). No repository file changed.
2. Arms `BC`, `LZ`, `LP` added to the three asked for; the rule evaluated on
   two counted metrics (declared in the protocol).
3. `LZ` and `LP` counted t stages are a model (3 products per batched element,
   9 and one inversion per batch), not instrumented library code.
4. n = 59: 2 repetitions instead of 3, no natural "yes" targets, two shards run
   in parallel with each other; n = 53 ran in parallel with the small cells.
   Another agent used the machine throughout.
5. Development (seed 7) exposed two defects, fixed before the freeze: `PM`'s
   trial counter (bookkeeping), and the paired t stage reading the field
   through a second pointer, which kept the table-path branch in its loop.
   No source change after the freeze: sha256 of `main.rs`, `linear21_pm.rs`,
   `linear21.rs`, binary as in `PROTOCOL.md` §9 and `raw/binary.sha256`.
6. `PM`'s t stage is scalar code outside the library; an eight-lane version
   was not written. From the `LP` reading it would move the whole leaf by at
   most the 1.025 to 1.036 ceiling.
7. Not covered: the leaf itself (97% of the wall time: products called across
   the crate boundary, tracked elimination with a zeroed 1 KB pivot array);
   `lin21` (`Variant::Full`); other curves, even n, `b ≠ 1`, n > 59;
   end-to-end relation or ECDLP cost; matched rho.
8. `research-kb`, the lead atlas and `ecdb` are not installed in this
   container and were not queried; no novelty statement is made. No figure or
   PDF was made. `tools/lessons.py record` was not run (it writes outside this
   directory).

## 9. What an admissible measurement would need

An isolab-eks run (AGENTS.md §9a) of the same binary on an isolated host with
source and image identity checked, a `bench.record/v1` record with raw data
in S3, a replay certificate and validator report named by `VALIDATION.json`,
and a matched rho reference with the §5a price band. Given the wall ceiling
above, the stage to submit would be the leaf, not the t stage.

## 10. Rebuild and rerun the smallest cell

```sh
T=/tmp/scan_pm_target sh research/scan_pm_pairing_20261116/build.sh && D=$(mktemp -d) && /tmp/scan_pm_target/scan_pm run --cells 1:23:8 --seed 20261116 --out $D && /tmp/scan_pm_target/scan_pm summarize --dir $D --out $D
```

The first step is a full `cargo build --release --example ic_algebraic_floor`
(about 11 minutes cold, about 1.4 GB); the cell itself takes 0.1 s. Counted
and gate columns reproduce exactly; wall columns vary. All cells:
`T=… OUT=<new dir> sh research/scan_pm_pairing_20261116/run.sh` (about 12
minutes, two processes).
