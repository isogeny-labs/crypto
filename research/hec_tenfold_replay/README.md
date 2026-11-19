# End-to-end index calculus on small-genus hyperelliptic Jacobians

This experiment measures a cold, single-target solve against the registered pencil implementation on identical divisors. Each arm verifies its scalar in the Jacobian. The registered panel has four generator–target pairs on each of four curves per genus. The finite-panel evidence supports a constant-factor engineering gain. The general mathematical speedup and its attribution remain open.

| Comparison | Cases with paired median at least 10× in every measured process | Lowest paired median |
|---|---:|---:|
| Registered genus 2, original LLVM execution, four processes | 16 / 16 | 11.81× |
| Registered genus 3, original LLVM execution, four processes | 16 / 16 | 10.21× |
| Registered genus 4, original LLVM execution, four processes | 16 / 16 | 10.18× |
| Genus 2 expanded development panel, alternating doublings, two processes | 128 / 128 | 10.36× |
| Genus 2 frozen holdout, alternating doublings, two processes | 126 / 128 | 7.89× |

The expanded panel exposed a degree-one generator. The corrected solver doubles that generator and, if needed, shifts a degree-one target by the lifted generator. It converts the verified scalar back to the original pair. The development panel informed that correction. The holdout changed the seed from 2026111904 to 2026111905 after freezing the code. Both holdout misses were at field prime 127, cases 26 and 27. Their forward / reverse ratios were 7.89 / 7.95 and 9.42 / 9.11.

## Native replay

Run from this directory with a C++17 compiler, rustc, and a POSIX shell. The native build uses the checked-in Rust modules and C++ kernels. Build products go to a temporary directory. Native compilation and LLVM execution use different linking and code-generation paths. Compare ratios within each run.

```sh
./replay.sh g2-registered forward
./replay.sh g3 forward
./replay.sh g4 forward
HEC_PAIR_SEED=2026111905 ./replay.sh g2 forward
HEC_PAIR_SEED=2026111905 ./replay.sh g2 reverse
```

The first native forward replay used arm64 macOS, rustc 1.93.1, and Apple clang 21.0.0. All scalar checks passed. Native episode minima were 13.05×, 10.22×, and 9.97× for the three registered panels respectively, with 47 / 48 reaching 10×. The native genus-2 holdout reached 10× on 126 / 128 cases, with a 7.89× minimum. Wall times can vary on the shared host.

Each output line beginning with `P` gives genus, field prime, case, repetition, arm, and elapsed nanoseconds, followed by diagnostic fields. Divide arm 0 time by the candidate time in the same repetition, then take the median per case. In the three registered drivers the candidate is arm 2. In the expanded genus-2 driver arms 1, 2, and 3 are the arithmetic, Fibonacci, and alternating-doubling collectors. The holdout comparison uses arm 3. The generator and target are fixed across all arms within each case. Timings include catalogue construction, collection, elimination, and scalar verification. Instance selection and creation of the benchmark inputs precede timing.

## Mathematical checks

```sh
c++ -O3 -std=c++17 certificates.cpp -o /tmp/hec-certificates
/tmp/hec-certificates
```

The certificate driver independently interpolates signed triples of factor-base points and enumerates slope collisions in each quadratic divisor class of the selected prime-order subgroups at 127 and 257. It checks equality class by class and verifies the first two moments of the number of distinct-point relations over every 36-term arithmetic progression with nonzero increment. It also compares elimination with and without removing rows incident to singleton columns. Every reported certificate must give the planted scalar.

For translates D + jT, a certificate exists exactly when the signed incidence matrix M makes Mz = 1 inconsistent modulo the subgroup order, assuming both public divisors are nonzero in the subgroup. A left-kernel combination with nonzero coefficient sum cancels the factor-base variables and preserves a public relation. Every surviving relation determines the scalar since D and T are nonzero in the same prime-order subgroup. The criterion concerns the rows supplied to the linear system.

In a separate first-certificate experiment on 8,192 sampled input pairs per field, alternating doublings reduced mean translate counts from 26.12 to 20.01 at 127 and from 22.89 to 20.39 at 257. Corresponding elimination-update reductions were 49.5% and 31.5%. No tenfold improvement attributable to the sequence alone has been established. Factor-base restriction as a collection–elimination tradeoff already appears in the literature. [Gaudry, Thomé, Thériault and Diem](https://eprint.iacr.org/2004/153.pdf)

## Evidence files

- `results/g2-registered-*.log` and `g2-validation-*.log` preserve timed rows from the original three-arm genus-2 comparison. The latter uses 16 additional inputs on the same curves.
- `results/g{3,4}-registered-*.txt` contain per-case paired medians from four original processes. Columns are field prime, case, pencil/candidate, and comparison-arm/candidate. The original per-repetition timings for these two genera are absent from this checkpoint.
- `results/g2-expanded-*.txt` and `g2-holdout-*.txt` contain field prime, case, generator and target polynomial coefficient counts, then median ratios for arms 0/1, 0/2, 0/3, 1/2, 1/3, and 2/3. The two files per panel use opposite timing orders.
- `results/native-*.txt` contain field prime, case, paired repetition count, and native pencil/candidate median. `results/certificates.txt` records the finite mathematical checks.
- `harness/` preserves the original machine-specific LLVM driver text for provenance. `replay.sh`, `sources/`, and `variants/` supply the standalone native build. The source snapshots retain the arithmetic and benchmark controls of the historical drivers.

The frozen holdout report plots both timing orders for every case. A clean export of the committed tree reproduced the mathematical output byte for byte and verified all 4,096 arm timings in the reverse-order holdout. [Holdout report (PDF)](reports/holdout-2026111905/report.pdf)

## Fresh comparison across all three genera

Seed 2026111906 tests 32 input pairs per field in genus 2 and eight per field in genera 3 and 4. Each comparison uses four fields and opposite timing orders. For genus 2 the choice fixed before this run was Fibonacci translates at field prime 127 and alternating doublings elsewhere. Genus 3 and genus 4 use the candidate previously measured on the registered panel.

| Genus | Cases reaching 10× in both orders | Lowest paired median |
|---|---:|---:|
| 2 | 122 / 128 | 8.50× |
| 3 | 30 / 32 | 9.45× |
| 4 | 32 / 32 | 10.08× |

Six genus-2 misses occurred at field prime 127 (cases 13, 14, 17, 19, 25 and 31). In genus 3, field prime 127 case 5 missed in both orders and field prime 257 case 1 missed in reverse order. All timed solves verified the expected scalar. On seed 2026111905 the Fibonacci arm cleared the two previous doubling misses, but the field rule introduced other misses. The evidence does not support universal 10× performance.

Use `HEC_PAIR_SEED=2026111906 ./replay.sh g2 forward` for the four-arm genus-2 comparison. Select arm 2 at prime 127 and arm 3 elsewhere. For genus 3 or 4 use `HEC_PAIR_SEED=2026111906 HEC_PAIRS=8 ./replay.sh g3 forward` (substitute `g4` for genus 4), then repeat in reverse order. The defaults preserve the registered generator seeds and case counts for those two genera.

The field-rule table stores seed, timing order, prime, case, four arm medians in nanoseconds, and the median of paired reference/candidate ratios. The medians use eight repetitions. The compressed genus-2 timing log covers seed 2026111906 in reverse order. The other three genus-2 processes retain case summaries. The fresh genus-3 and genus-4 tables store timing order, prime, case, and median paired ratio over 32 repetitions. The compressed logs retain all timed rows for both orders in those genera.

### Direct two-place relations in genus 2

A translated divisor with split quadratic support gives a relation immediately when both points belong to the retained factor base. Repeated roots contribute with multiplicity two. In the finite subgroup check, every split decomposition was recomposed with Cantor arithmetic, and every recovered scalar was checked against its input. Over 8,192 pairs per field, adding these rows to the signed triples cut probes to the first certificate from 163,851 to 154,700 at p = 127 and from 167,787 to 162,663 at p = 257. All cases finished within 96 probes. Allowing two distinct outside points reduced the totals further to 153,083 and 162,170, while increasing the number of rows processed. Choosing the translate sequence from early rank gains took more probes than fixed alternating doublings at both primes.

The native experiment used seed 2026111907 and 32 pairs per field. Each entry below is the median across cases of the median of eight paired elapsed-time ratios. Ratios above one favor the solver with direct rows. The incremental comparison uses identical alternating doublings and changes only direct-row collection.

| Field prime | Incremental, forward | Incremental, reverse | Pencil / direct, forward | Pencil / direct, reverse |
|---:|---:|---:|---:|---:|
| 127 | 1.022× | 0.987× | 17.32× | 17.34× |
| 257 | 1.018× | 0.979× | 15.49× | 15.21× |
| 1021 | 0.998× | 0.960× | 18.86× | 19.21× |
| 4093 | 1.035× | 0.949× | 18.52× | 18.21× |

The direct-row arm cleared 10× against the pencil on 126 of 128 cases in both orders. Its incremental wall-time benefit did not reproduce across orders, so the existing solver remains the default.

Run `c++ -std=c++17 -O3 -DHEC_RELATION_ABLATIONS certificates.cpp -o /tmp/hec-relations && /tmp/hec-relations` here to reproduce `results/relation-ablations.txt`. For each field the first four lines beginning with `first` cover Fibonacci, alternating doublings, interleaving, and rank feedback. The next four cover triples alone and triples plus split pairs allowing zero, one, or two outside columns. Each line reports prime, arm, total probes, rows, elimination updates, censored cases, median, 95th percentile, maximum, and counts earlier, tied, or later than the first arm in its group. Run `harness/direct-pairs.zsh` to rebuild and time the native comparison. Both original process logs and their correctness audits are checked into `results/direct-pairs-*`.

### Phase timing on the eight misses from seed 2026111906

Separate diagnostic builds timed the same inputs in both execution orders. The following percentages are within-case medians of the phase time divided by total candidate time in each repetition. In genus 2, the six misses at p = 127 spent 72.6–92.9% in the combined relation collector and elimination kernel in forward order and 73.4–92.9% in reverse order. This measurement does not separate collection from elimination.

| Genus 3 case | Quartic catalogue and rank solve | Coordinates of both divisors | Scalar verification and final field arithmetic |
|---|---:|---:|---:|
| p = 127, case 5 | 60.8% / 60.8% | 2.4% / 2.4% | 35.9% / 35.9% |
| p = 257, case 1 | 76.3% / 79.5% | 1.2% / 1.2% | 22.0% / 18.0% |

Both orders point to the cost of constructing enough independent relations. At p = 127, even eliminating both genus-3 coordinate evaluations would save only about 2.4% of measured candidate time. The original 9.45× reverse-order result would need a 5.5% candidate-time reduction to reach 10× at fixed reference time. Comparing those percentages assumes phase shares transfer between builds.

The four compressed timing logs under `results/phase-g*` retain the original column layout. For genus 2, read arm 2 at p = 127. Its first three diagnostic columns are nanoseconds in stream setup, the relation kernel including fallback, and the final native verification. Ignore diagnostic columns on the pencil arm, which does not refresh the native counters. For genus 3, read arm 2. Its first three counters surround the quartic setup call, both coordinate evaluations, and final arithmetic plus verification. Instrumentation adds clock overhead. Use the uninstrumented comparison above for speedup claims.

### Earliest genus-3 certificate on the 32 seed-2026111906 inputs

Let D and T denote the two factor-base coefficient vectors over the subgroup scalar field. After reducing both by a basis of the catalogue rows seen so far, a nonzero reduced D and a proportional reduced T give the scalar. The test derives it from one nonzero coordinate, checks proportionality on every coordinate, and compares the result with the planted scalar. On the original single representations, the first certificate occurred at the final catalogue row in all 32 cases.

For each degree-three divisor, the experiment also enumerates the split residual quartics from the pencil v + t u. Each additional representation differs from the original by a relation. Each difference reduced to zero against the completed catalogue. We inserted the differences before processing the catalogue in its original order. With both input pencils, every case needed fewer ordinary catalogue poles. The first certificate preceded full rank of the augmented rows in 13 cases.

| Additional pencils | Fewer total passes | Equal | More | Net passes saved over 32 cases |
|---|---:|---:|---:|---:|
| Generator | 6 | 22 | 4 | 2 |
| Target | 6 | 19 | 7 | −1 |
| Both | 4 | 17 | 11 | −7 |

The pass count charges one unit per input pencil and one per ordinary catalogue pole visited. These operations can have different costs. The replay uses exhaustive residual factorization and exact prefix tests to measure the earliest possible stop. It makes no elapsed-time claim. At p = 127 case 5, adding both pencils moved the certificate from row 69 to row 58 and pole 11 to pole 9. At p = 257 case 1, it moved from row 125 to row 104 and pole 14 to pole 12. Charging the two extra pencils cancels the pole saving on both misses.

Run `ruby harness/g3-prefix.rb results/g3-prefix-rows.gz results/g3-prefix-inputs.txt` and compare stdout with `results/g3-prefix.txt`. The compressed fixture retains catalogue emission order and both original signed coefficient vectors. The inputs file stores the curve coefficient and the generator and target Mumford polynomials. The arms column uses 0 for the generator and 1 for the target.

### How much of the catalogue time goes to elimination?

We replayed the four genus-3 and four genus-4 catalogue matrices, removed pivot equations, solved the residual systems, and reconstructed all original factor-base coordinates. On all eight matrices the reconstructed vector satisfied every original equation and matched ordinary elimination up to a nonzero scalar.

The degree-one/two filter removes a column by substituting its shortest pivot equation into at most one other row. In genus 3 it left cores with 33/40, 58/69, 101/126 and 151/196 columns. In genus 4 it left 19/20, 23/25, 26/29 and 35/37 columns. A second filter admits higher-degree pivots when a structural upper bound guarantees that the total number of nonzero entries cannot grow during substitution. This reduced four matrices to one free coordinate (genus 3 at 73, genus 4 at 37, 43 and 59). Selecting pivots, performing substitutions, and reconstructing coordinates still cost work.

We measured a separate timing control to quantify the opportunity. Each process captured the original stopping row and factor-base solution in one untimed catalogue run. It then alternated ordinary catalogue construction with a version that consumed the same row stream and supplied the recorded solution at that row. Each of the 128 pairs per field checked the anchor, canonical ordinates and complete value array. The table gives median paired ordinary/replay catalogue times in processes with opposite starting orders.

| Genus | Prime | Forward | Reverse |
|---|---:|---:|---:|
| 3 | 73 | 1.509× | 1.464× |
| 3 | 127 | 1.538× | 1.508× |
| 3 | 257 | 1.748× | 1.737× |
| 3 | 383 | 2.061× | 2.078× |
| 4 | 37 | 1.076× | 1.085× |
| 4 | 43 | 1.104× | 1.108× |
| 4 | 59 | 1.118× | 1.118× |
| 4 | 67 | 1.174× | 1.176× |

The timing control measures the catalogue stage in isolation, with a previously computed solution. It does not time the filters or provide an independent solver. Even on three genus-4 matrices that reduce to one free coordinate, bypassing rank determination gave only 1.08–1.12× on this stage. Reducing collection work is the larger opportunity there.

Run `ruby harness/matrix-filter.rb results/g3-prefix-rows.gz results/g4-catalogue-rows.txt` to reproduce the filter table. To rerun the diagnostic build from this directory on macOS, use `ruby harness/catalogue-rank-control.rb 3 | c++ -std=c++17 -O3 -x c++ - -o /tmp/hec-rank-control`, then run the binary once with no arguments and once with `reverse`. Substitute 4 for genus 4. Each timing row stores genus, prime, repetition, arm (0 ordinary, 1 replay), and elapsed nanoseconds. The four original streams are under `results/catalogue-rank-*`.

### Genus-4 pencil through a point with multiplicity four

For a nonbranch point A = (h,y) and z = x − h, let T₃ be the degree-three Taylor polynomial of the local branch at A. The functions y − T₃(z) − bz⁴ have a pole of order nine at infinity and vanish at A to order at least four. After subtracting 4A the residual divisor has degree five. A completely split residual gives a relation between 4A and up to five signed affine places, including repeated places. One extra bucket per point suffices to group the residuals by b, since for z ≠ 0 its bucket index is the fourth Taylor coefficient at A plus z times the existing fifth divided difference.

The independent check formed the monic quintic [f(h+z) − (T₃(z)+bz⁴)²]/z⁴, checked exact division and exhaustively compared field roots, ordinates and multiplicities with the emitted buckets. It checked all 6,005 pencil members on the four curves. The numbers of completely split residuals were 5, 14, 15 and 23 at primes 37, 43, 59 and 67 respectively. Every optimized row consumed during the untimed subset checks annihilated the existing factor-base values. All successful catalogues agreed with the original kernel vector up to scalar.

We tested all 15 nonempty subsets of the four families. At p = 37 every proper subset of the three original families exhausted the available poles before reaching rank 19. Adding the fourth family to all three reduced poles to rank on every curve. It increased the number of pencil members visited in three of the four fields.

| Field | Poles, original → four families | Members visited, original → four families | Original / four-family full solve, forward / reverse |
|---:|---:|---:|---:|
| 37 | 17 → 15 | 1,796 → 2,147 | 0.995× / 0.990× |
| 43 | 15 → 11 | 1,858 → 1,831 | 1.064× / 1.061× |
| 59 | 18 → 15 | 3,074 → 3,478 | 1.017× / 1.017× |
| 67 | 14 → 11 | 2,658 → 2,834 | 1.042× / 1.036× |

The full solve used seed 2026111908, eight new pairs per curve, and 32 paired repetitions per case in each timing order. Columns report the median across cases of the per-case paired median. The original three-family solver and the four-family solver each reached 10× against the pencil reference on 30/32 cases in both orders. Every scalar matched its input. The fourth family improved full solve time in both orders on 22/32 cases. Results are specific to these curves and host. The default stays unchanged.

From this directory, run `ruby harness/g4-fourth-pencil.rb verify | c++ -std=c++17 -O3 -x c++ - -o /tmp/g4-check && /tmp/g4-check` for the exhaustive residual check. Omitting `verify` produces the subset benchmark; run the binary with no arguments and again with `reverse`. Its columns are prime, family bit mask (8 selects the fourth family), repetition (−1 for validation), rank attained flag, rank, poles at success and pencil members visited, then nanoseconds. A zero pole count on failure denotes exhaustion. Run `harness/g4-fourth-pencil.zsh` for the complete native comparison. The timing arms are pencil reference, original optimized solver, and four-family solver respectively. The medians file stores prime, case, order, pencil/four-family and original/four-family ratios. Raw timing and audit streams are in Git.

### Matching quadratic places in genus 4

The first existing pencil fixes five copies of its pole. When the residual has rational support accounting for all but two degrees, consecutive synthetic divisions recover the two coefficients of the remaining monic quadratic from the leading coefficients of the norm. Its coefficients in global x coordinates index a table. A repeat of the support identifies two ordinates that agree or differ by sign in the quadratic extension. Subtracting or adding the rational parts of their equations accordingly eliminates the quadratic place. The implementation stores the first partial in compact form and computes orientations only when the support repeats. It uses the three original pencil families and adds matched rows from the first.

| Field | Partial rows before stop | Matches increasing rank | Poles, original → matching | Pencil members, original → matching |
|---:|---:|---:|---:|---:|
| 37 | 50 | 3 | 17 → 15 | 1,796 → 1,578 |
| 43 | 59 | 5 | 15 → 11 | 1,858 → 1,401 |
| 59 | 81 | 4 | 18 → 16 | 3,074 → 2,790 |
| 67 | 87 | 6 | 14 → 12 | 2,658 → 2,264 |

All 18 matches increased rank. An untimed pass checked irreducibility and the curve equation modulo every residual quadratic encountered before stopping. Both catalogue arms reconstructed the same projective factor-base values. Each full solve independently verified its scalar. The complete benchmark rotates the pencil reference, the original three-family solver and the quadratic-matching solver on identical inputs. Each case has 32 repetitions in each of two opposite timing orders on one host.

| Input seed | Cases faster than original in both orders | Cases at least 10× pencil in both orders, original → matching |
|---:|---:|---:|
| 2026111908 (previous panel) | 17/32 | 30 → 30 |
| 2026111909 | 29/32 | 31 → 32 |
| 2026111910 (fresh replication) | 14/32 | 31 → 31 |

At p = 43 all 24 pairs improved in both orders. Across those six processes the median case speedup was 1.069–1.094×. At p = 59 and 67 the field medians fell below one in reverse order on two seeds. Reducing members visited therefore supports a collection-work saving on all four curves, with a smaller and field-dependent elapsed-time effect. The solver default stays unchanged.

Run `ruby harness/g4-quadratic-places.rb | c++ -std=c++17 -O3 -x c++ - -o /tmp/g4-quadratics`, then run the binary with no arguments and again with `reverse`. Its columns are prime, repetition, matching enabled, poles, members visited, partials, matches, ramified partials checked, matches increasing rank and nanoseconds. Repetition −1 enables the extension-field checks. Run `HEC_PAIR_SEED=2026111910 harness/g4-quadratic-places.zsh` for the full benchmark; its default seed is 2026111909. Case summaries store prime, case, order and paired medians for pencil/original, pencil/matching and original/matching. The raw logs and audits for all three seeds are checked into Git.

### Genus-3 quartics with one or two quadratic places

We held the original pole sequence and stopping fibre fixed on each field. For every pencil member we formed its residual norm polynomial and removed its rational roots with multiplicity. For a quartic without rational roots we tested for two quadratic factors using coefficient comparison. Writing the factors as (z²+az+d)(z²+cz+e), each candidate constant d determines e. The cubic and linear coefficients determine a and c when d differs from e. The remaining coefficient verifies the factorization; the equal-constant case requires a quadratic equation. For each retained quadratic we verified irreducibility and the ordinate's curve equation in its residue field. Support and ordinate sign identify the place across different poles.

The exact replay inserts the signed equations over the subgroup scalar field, eliminating quadratic columns first. Each resulting rational equation was verified against the original factor-base solution. The table gives the first pencil member at which the rational system attains the original rank. The last two columns count one-place equations read and independent rational rows they supplied by that point in the one-place run.

| Prime | Original | One quadratic allowed | Up to two allowed | One-place equations read | Independent rows from matches |
|---:|---:|---:|---:|---:|---:|
| 73 | 542 | 473 | 465 | 119 | 7 |
| 127 | 1,363 | 1,145 | 1,145 | 327 | 11 |
| 257 | 3,546 | 2,952 | 2,935 | 754 | 19 |
| 383 | 4,282 | 3,958 | 3,958 | 973 | 11 |

One-place matches reduced the number of visited members by 7.6–16.8%. Including two-place equations did not save an additional pole on any field. The count of independent rows per 100 one-place equations fell from 5.9 to 1.1 over these four instances. Field size and curve both vary, so the observed trend does not establish an asymptotic law.

A native one-place prototype computed the support and orientation from the leading coefficients, stored unmatched rows in a hash table, and submitted matched rows to the existing rank solver. Seed 2026111911 used eight fresh input pairs per field, 32 repetitions, and opposite timing orders. All complete solves verified the scalar. Median original/matching full-solve ratios, forward/reverse, were 0.658/0.668, 0.553/0.548, 0.460/0.456 and 0.437/0.434 at increasing primes. Matching was slower on all 32 cases in both orders. Against the pencil reference, 28 original cases and zero matching cases reached 10× in both orders. The default stays unchanged. The two-place result above measures rank only.

Run `ruby harness/g3-quadratic-census.rb | c++ -std=c++17 -O3 -x c++ - -o /tmp/g3-census` followed by `/tmp/g3-census | ruby harness/g3-quadratic-rank.rb` to reproduce the rank table. The compressed stream also supports independent rank replay without recompilation. Run `harness/g3-quadratic-places.zsh` for the native timings. Arm 0 is the pencil, arm 1 the original optimized solver, and arm 2 the one-place prototype. The medians file stores prime, case, order and paired ratios for pencil/original, pencil/matching and original/matching.

### A second quartic residual pencil in genus 3

On the odd-degree-seven model, write z = x − h at a nonbranch point A. The cubic family y − T₂(z) − bz³ has a pole of order seven at infinity and at least three zeros at A. Its residual has degree four. The original family y − T₃(z) − tz⁴ also has degree-four residuals (including infinity where applicable). Away from A their bucket indices satisfy b = s₃ + zt, with s₃ the third Taylor coefficient. Both families can therefore use a single scan of the affine points. The implementation marks candidate buckets in a bitset, checks multiplicities, and inserts relations into the same rank solver. Its single-family control uses the same bucket representation.

An independent polynomial multiplication check compared every accepted cubic residual with the full product of its signed affine places' linear factors, including multiplicities, and checked each ordinate. The resulting catalogue values agreed with the original up to a nonzero scalar on each curve.

| Prime | Poles to rank, original → combined | Rows to rank, original → combined |
|---:|---:|---:|
| 73 | 8 → 5 | 39 → 40 |
| 127 | 11 → 6 | 69 → 69 |
| 257 | 14 → 7 | 125 → 128 |
| 383 | 12 → 7 | 195 → 209 |

The development comparison (seeds 2026111912, 2026111913, and the earlier 2026111906) used four arms: pencil reference, original optimized solver, bitset single-family control, and combined families. Across six timing processes the field median for original/combined ranged from 1.042–1.071× at 127, 1.096–1.136× at 257, and 0.884–0.932× at 383. At 73 it ranged from 0.990–1.013×. These are medians across eight cases of per-case medians over 32 paired repetitions.

Before measuring seeds 2026111914–1916 we selected combined families at 127 and 257 and the original implementation at 73 and 383. Every one of the 48 fresh cases on the selected fields ran faster in both orders. Across all four fields, the rule and original reached 10× against the pencil on 92/96 and 91/96 cases respectively. Both counts use the slower of the two per-case paired medians. This selection tests new inputs on the same four curves.

In the four-arm harness, identical original code paths on the unselected fields showed execution-order differences. A separate balanced crossover on fresh seed 2026111917 removed the pencil arm and ran original/candidate/candidate/original and its reverse assignment in alternating repetitions. We divided total original time by total candidate time within each quartet, then took the median over 32 quartets per case. The field medians in the two processes were 1.055×/1.065× at 127 and 1.123×/1.123× at 257. On the two same-code control fields they were 0.999×/1.000× at 73 and 1.000×/0.999× at 383. All timed solves verified their scalars. The selection remains experimental.

Run `harness/g3-cubic-pencil.zsh panel`, `rule`, or `crossover` for the three comparisons. The compressed archive contains every timing and audit log. Timing columns follow the native driver format; crossover arms 0 and 3 use the original code and arms 1 and 2 use the selected candidate. The four-arm medians file stores seed, order, prime, case, then ratios 0/1, 0/2, 0/3, 1/2, 1/3 and 2/3. The counts file stores prime, mode, poles, original-family rows and cubic-family rows (mode 0 is the uninstrumented original).

### Transfer to different genus-3 curves over the same fields

For each prime we selected two additional smooth curves of the form y² = x⁷ + x³ + cx + 1. We scanned c in ascending order, excluding the previously measured coefficient. We factored the exact order of the point (0,1) obtained by baby-step giant-step. Its largest prime divisor had to meet the original 1,000 and quarter-Hasse-lower-bound criteria and the native solver limit of 20 million. Each generator passed the subgroup-order check. This selection used no timings or rank results. All eight selected curves completed. We used eight input pairs per curve and seed 2026111918.

The comparison retained the pencil reference, the original optimized solver, the single-pencil bitset control and the two-pencil collector. A second process pair used the balanced crossover with the original and two-pencil solvers on every curve. Each case had 32 repetitions per process, and every timed result matched the planted scalar. The candidate's short scalar check used the curve-specific prime order and a conservative cofactor bound: the least common multiple of all possible positive cofactors up to the integer Hasse upper bound divided by the subgroup order. This avoids assuming the cofactor from a previous curve. All three catalogues produced the same factor-base coordinates up to a nonzero scalar.

| p | c | Poles to rank, single → two | Elimination coefficient updates, single → two | Crossover speedup, forward / reverse |
|---:|---:|---:|---:|---:|
| 73 | 33 | 15 → 9 | 467 → 455 | 1.030× / 1.032× |
| 73 | 57 | 17 → 8 | 877 → 566 | 1.126× / 1.138× |
| 127 | 15 | 20 → 12 | 960 → 1,318 | 1.053× / 1.055× |
| 127 | 45 | 12 → 6 | 2,418 → 2,032 | 1.090× / 1.089× |
| 257 | 54 | 13 → 8 | 9,814 → 11,807 | 0.986× / 0.992× |
| 257 | 78 | 9 → 6 | 15,338 → 20,931 | 0.870× / 0.878× |
| 383 | 42 | 13 → 7 | 24,395 → 21,813 | 1.098× / 1.101× |
| 383 | 80 | 14 → 7 | 37,001 → 41,022 | 1.013× / 1.011× |

The exact update count comes from a separate untimed replay of the pivot elimination loop. Six curves improved on every pair in both crossover orders. Both p = 257 curves slowed down on every pair. At c = 78 the two-pencil matrix consumed 171 rows against 146 and needed 36% more coefficient updates. The original solver reached 10× against the pencil reference on 55/64 cases and the two-pencil solver on 50/64. Applying the earlier field-only choice (two pencils at 127 and 257) to the paired results gives 48/64. This last count is derived from the measured arms. Field size alone did not transfer as a selection rule.

Run `harness/g3-transfer.zsh` to reselect the curves, audit the catalogues and coefficient counts, and rerun the full-solve comparisons. The curves file stores prime, coefficient, subgroup order and multiplier of the starting point. The results archive contains all four timing streams and audits. The median table gives per-case ratios for pencil/original, pencil/two-pencil, original/two-pencil and the balanced crossover. All ratios compare times within the same repetition or quartet.

### Shorter pivots on the same genus-3 relation stream

All arms in this test used the two-pencil collector on the eight transfer curves. Suppose an incoming equation W and a stored pivot E have the same leading column with nonzero coefficients. Replacing E by the normalized W and continuing elimination with the difference W − (Wⱼ/Eⱼ)E preserves their span over the subgroup field. Any later row can use the shorter pivot. The implementations append a replacement and finish the current subtraction against the old pivot. Previous pivot equations remain available for reconstruction.

We tested two ways to decide when a replacement is worthwhile. The scan counts nonzero coefficients modulo the subgroup prime, abandoning the scan once the current pivot cannot be improved. It checks an initial collision when the input is shorter, or a subsequent collision when the old pivot has at least eight tail entries. The bitset version tracks an upper bound on nonzero columns, tests pivots with at least four tail entries, and exchanges only when that bound is strictly smaller. Both versions reproduced the original projective factor-base coordinates, pole counts and accepted row counts on all eight curves.

| p | c | Coefficient updates, fixed pivot → scan → bitset | Bitset exchanges | Bitset full-solve ratio, forward / reverse |
|---:|---:|---:|---:|---:|
| 73 | 33 | 455 → 477 → 441 | 4 | 0.982× / 0.987× |
| 73 | 57 | 566 → 505 → 493 | 4 | 0.977× / 0.981× |
| 127 | 15 | 1,318 → 1,201 → 1,281 | 10 | 0.979× / 0.980× |
| 127 | 45 | 2,032 → 1,872 → 1,942 | 4 | 0.973× / 0.966× |
| 257 | 54 | 11,807 → 10,622 → 11,582 | 14 | 0.961× / 0.962× |
| 257 | 78 | 20,931 → 19,400 → 20,180 | 23 | 0.964× / 0.933× |
| 383 | 42 | 21,813 → 18,303 → 18,621 | 22 | 1.072× / 1.076× |
| 383 | 80 | 41,022 → 36,238 → 35,805 | 51 | 0.990× / 0.981× |

The counts measure the innermost elimination coefficient updates and exclude the work of selecting, normalizing and storing replacements. The timing ratios include the entire solve. Each curve had eight fresh inputs (seed 2026111919) and 32 balanced quartets per process. Ratios above one favor the modified solver. The bitset reduced updates on all eight curves and improved full solves on all eight inputs of just one curve, p = 383, c = 42. On the two p = 257 curves, an exact scan cut updates by 10.0% and 7.3%, but the median elapsed-time ratios were only 0.882/0.881 and 0.836/0.834. In the separate four-arm bitset panel, the unchanged and modified two-pencil solvers cleared 10× against the pencil in both orders on 54/64 and 52/64 cases respectively. The fixed-pivot solver remains the default.

Run `harness/g3-pivot-exchange.zsh scan` or `harness/g3-pivot-exchange.zsh mask` for the corresponding replay. The four-arm panel uses pencil, two-pencil fixed pivot, intermediate control and two-pencil modified pivot. The scan control exchanges on initial collisions only; the bitset control traverses the same support bitsets without exchanges. The crossover compares the fixed and modified pivots. Counts files contain OPS lines with prime, coefficient, variant, coefficient updates and exchanges, followed by catalogue records in the transfer format. Medians columns use the transfer format, with original and combined denoting fixed and modified pivots here. Raw timing and audit logs for both implementations are in the archive.

### Selecting genus-2 factor-base places by quadratic character

For the curve with constant coefficient 4 at p = 127 and 257, take the factor g of the curve polynomial to be X − 81 and X² + 222X + 97 respectively. The certificate checks divisibility and irreducibility of the quadratic. At a nonbranch rational place, retain x if g(x) is a nonzero square. Evaluating functions attached to factors of the curve polynomial is motivated by explicit descent. The coverage and timing results below are specific to these experiments. [Poonen and Schaefer, Explicit descent for Jacobians of cyclic covers](https://math.mit.edu/~poonen/papers/descent.pdf)

The census enumerates signed triples with distinct x coordinates mapping to a degree-two divisor in the selected subgroup, excluding overlaps with its Mumford u polynomial. Each eligible triple in the original base has zero or two nonsquare values. Consequently scanning only the positive places encounters at least one root of every such triple. Exhaustive subset search also gives an exact lower bound on how many fixed places a scan must probe to encounter at least one or at least two roots of every triple.

| p | Base size | Eligible signed triples | Minimum one-root scan | Minimum two-root scan | Eligible triples in a full-size positive base |
|---:|---:|---:|---:|---:|---:|
| 127 | 26 | 10,230 | 12 | 25 | 20,310 |
| 257 | 52 | 43,642 | 22 | 51 | 87,270 |

The one-root cover and the timed factor base use different membership rules. For coverage, one positive root suffices. For the timed collector, all three roots must lie in the base, and the search extends to larger x until the original cardinality is reached. Restricting the base to positive places inside the original x range gave 1,726 and 5,990 eligible triples, versus 814 and 3,020 in ascending prefixes of the same reduced sizes.

In the initial two seeds (2026111920–21), the full-size positive base beat the ascending base on 127/128 inputs in both timing orders. Relative to the existing pencil solver, 127/128 cleared 10× in both orders with the positive base and 122/128 with the ascending base. This comparison includes the earlier collector and solver improvements.

The independent input seed 2026111922 used 32 inputs per field and 16 balanced quartets per process. The table reports the median across inputs of each input's median original-time / selected-time ratio. The p = 4093 curve uses X − 268 and 410 places in each base. The p = 1021 control executes identical selection code in both arms.

| p | Forward | Reverse |
|---:|---:|---:|
| 127 | 1.217× | 1.237× |
| 257 | 1.276× | 1.255× |
| 1021, control | 1.000× | 1.000× |
| 4093 | 1.438× | 1.446× |

A separate prefix-scan experiment stopped after two roots in a three-quarter prefix or one root in a quarter prefix and reconstructed residual candidates. At p = 4093 the one-root arm reduced point probes from 981,130 to 414,060 across 32 inputs, but ran at 0.220× and 0.217× of the original speed. Reusing the signs already computed by the two-root scan gave 0.997× and 0.957×. The timed character experiment uses the full scan of its selected base.

Run `harness/g2-scan.zsh` with `original`, `optimized`, `character` or `crossover` for the native experiments. To replay a census, pipe `ruby harness/g2-scan-cover.rb` (optionally with `expanded`) to `c++ -I. -O3 -std=c++17 -x c++ - -o /tmp/hec-cover` from this directory and run the binary. The default census columns are p, base size, required hits, eligible signed triples, unsigned supports, triples reached by the prefix, minimum scan size, omitted-set witness mask, search nodes, positive places, signed triples entirely positive and signed triples in the equal-size prefix. Expanded output gives p, base size and eligible triples. Git contains the timing and audit logs in `results/g2-scan-raw.tar.gz`. The medians files give order, p and input index followed by pencil/original, pencil/second, pencil/third, original/second and original/third. Crossover files instead give the original/selected quartet ratio.

### Character selection and catalogue rank in genus 3

Five of the eight transfer curves have reducible curve polynomials. The other three, (73,33), (73,57) and (127,45), have irreducible degree-seven polynomials over their base fields. On each reducible curve we used its lowest-degree monic irreducible factor. We kept the nonbranch places where that factor evaluates to a nonzero square, and retained the rational branch points in both arms. The control took the same number of nonbranch places in ascending x order. Each arm exhausted both pencils over all its retained poles. Ranks are modulo the curve's subgroup prime.

| p, c | Nonbranch places | Rows, prefix / character | Rank, prefix / character | Rank needed |
|---|---:|---:|---:|---:|
| 127, 15 | 21 | 1 / 8 | 1 / 8 | 20 |
| 257, 54 | 58 | 55 / 108 | 55 / 57 | 57 |
| 257, 78 | 63 | 63 / 159 | 62 / 62 | 62 |
| 383, 42 | 88 | 114 / 249 | 86 / 87 | 87 |
| 383, 80 | 100 | 263 / 461 | 99 / 99 | 99 |

The positive base reaches a one-dimensional nullspace on four curves. The equal-size prefix does so on two. At (127,15) adding the first ten nonsquare places reaches rank 30 on 31 places. Nine additions reach rank 26 on 30 places. This is a threshold for that particular ordering. Every reduced basis annihilates the independently reconstructed full-catalogue coordinate vector restricted to its columns, and every restriction is nonzero.

From this directory, pipe `ruby harness/g3-character-census.rb` to `c++ -std=c++17 -O3 -x c++ - -o /tmp/hec-character-census`, then run the binary. The output fields are p, curve coefficient, arm, base size, rows and rank. Arms 0 and 1 use the full base and matched prefix. Arms 2 through 14 use the character base with zero through twelve extra nonsquare places. This census measures catalogue completeness. Full-solve timings for the restricted genus-3 bases remain open.

### Full solves with genus-3 character bases

The solver constructs the two-pencil catalogue for each input pair and decomposes both divisors over the retained places. The quartic completion accepts a candidate only when both reconstructed roots have known coordinates. The shift fallback likewise checks membership of every factor. Audited runs compare the reduced catalogue to the full catalogue on retained coordinates, check the quartic divisor sum and cross-check successful quartic decompositions by shifting. All 64 input scalars matched their planted values on each of two seeds.

The first experiment used character selection on all five eligible transfer curves, including ten extra places on (127,15). The second used a fresh seed and kept the full base on (127,15), where the pure character base was rank-deficient. Each curve used eight inputs and 32 balanced quartets per input, in forward and reverse processes. Times include catalogue construction, decomposition and scalar verification. Ratios above one favor the selected solver. Three irreducible-polynomial curves ran as identical-code controls, with median ratios between 0.993 and 1.001 across the two seeds and orders.

| p, c | Seed 2026111923, forward / reverse | Seed 2026111924, forward / reverse |
|---|---:|---:|
| 127, 15 | 0.588 / 0.587 | 1.000 / 0.999 (full-base control) |
| 257, 54 | 1.019 / 1.011 | 0.803 / 0.804 |
| 257, 78 | 1.295 / 1.280 | 1.216 / 1.209 |
| 383, 42 | 1.116 / 1.120 | 1.139 / 1.125 |
| 383, 80 | 1.112 / 1.094 | 1.115 / 1.110 |

At (257,54), the restricted base needed the shift fallback for two of sixteen divisor coordinates on the first seed and six on the second. The full base completed all sixteen directly on each seed. At (127,15), the first seed's selected catalogue needed 31 poles to reach rank, versus 12 for the full catalogue. These costs help explain why rank alone was insufficient to select a faster solver.

Against the pencil baseline, the full and selected solvers respectively cleared 10× in both orders on 55 and 53 of 64 inputs on seed 2026111923, and on 54 and 53 on seed 2026111924. Character selection improved full solves reproducibly on three of five eligible curves. The full-base solver remains the default.

Replay with `harness/g3-character-solve.zsh all` or `harness/g3-character-solve.zsh positive-only`. The tracked raw archives include the comparison panel, balanced crossover and decomposition audits. Both medians files use the transfer-panel format.

### Retrying a missed genus-3 decomposition after one place addition

When the direct quartic decomposition of D misses, the retry computes D+A for the catalogue anchor A and attempts the same quartic decomposition once more. If the coordinate is found, linearity gives coord(D) = coord(D+A) − coord(A) modulo the subgroup order. A second miss takes the original shift fallback. The audit cross-checks each successful retry against that fallback on D. The comparison uses the same catalogue in both arms, with the positive base on the four curves whose positive base alone passed the rank test, and the full base elsewhere.

| Input seed | Initial misses among 128 divisor coordinates | Coordinates recovered by one retry | Full solves at least 10× faster than pencil in both orders, before / after |
|---|---:|---:|---:|
| 2026111925 | 19 | 10 | 54 / 57 of 64 |
| 2026111926 | 15 | 11 | 52 / 60 of 64 |

The crossover compares the entire solve with and without the retry. At (257,78), the median ratio across inputs that actually needed a retry was 2.030× / 2.028× in forward / reverse order on the first seed and 2.034× / 2.032× on the second. Across all eight inputs on that curve, the ratios were within 0.3% of parity because most inputs decomposed directly. At (257,54) the all-input ratio was near parity on the first seed and 1.115× / 1.113× on the second. At (383,42) the affected-input median was below parity on the first seed. All 128 input scalars were verified. The 10× counts compare against the earlier pencil solver and include catalogue and descent improvements together.

Run `HEC_SHIFT_SEED=2026111925 harness/g3-character-solve.zsh shift` and repeat with seed 2026111926. The comparison panel uses pencil, selected catalogue without retry, full catalogue without retry, and selected catalogue with retry. The crossover compares only the two selected-catalogue arms. Run `ruby harness/g3-shift-summary.rb OUTPUT_DIRECTORY` on the output directory for the per-curve table. The affected-input subset includes every input requiring a retry, including failed retries. Git stores the full timing and audit streams.

### Screening genus-4 pencil members by intersection degree

For each pole and each of the three pencil families, the rational affine intersection multiplicities are already known during collection. Adding the multiplicity at infinity and the fixed multiplicity at the pole gives the degree supported on rational places. A completely split residual has the target degree, five in the first family and six in the other two. The packed implementation adds one to the first-family counter to give all three families the same threshold. It initializes the exceptional parameters separately, accumulates the point contributions in byte counters, and tests eight counters per machine word. Since each counter lies between zero and six, adding 122 per byte sets the high bit exactly at six, with no carry between bytes. The alternative mask implementation marks plausible parameters and performs the original degree check on those parameters.

| Field | Pencil members in exhaustive scan | Candidates after mask | Candidates after packed test | Relations reproduced |
|---:|---:|---:|---:|---:|
| 37 | 2,220 | 117 | 22 | 22 |
| 43 | 3,225 | 165 | 48 | 48 |
| 59 | 5,133 | 180 | 45 | 45 |
| 67 | 7,437 | 281 | 107 | 107 |

Both implementations reproduced every row coefficient and its original position across the full catalogues. The packed test still accumulates multiplicities and reads the degree counters for every block. The reduction in candidates measures the work after that scan.

On seeds 2026111927 and 2026111928 each field had eight fresh inputs, 32 repetitions and two timing orders. The packed implementation improved all 64 full solves in both orders. Relative to the existing three-family solver, field medians across seeds and orders ranged from 1.050–1.058× at p = 37, 1.037–1.043× at p = 43, 1.062–1.074× at p = 59 and 1.027–1.037× at p = 67. All 64 cleared 10× against the pencil reference in both orders, compared with 62 for the existing solver in the same binary. The simpler mask improved all 64 full solves too, and both its arms cleared 10× on 63 of them.

Use `harness/g4-degree-screen.zsh mask` or `harness/g4-degree-screen.zsh packed` for the timing runs. Append `expanded` for the 32-input-per-field replication. The audit generator accepts `mask audit` or `packed audit` and emits a standalone C++17 program. The medians reducer accepts the timing logs as arguments. The raw archives also retain the untimed verification streams.

The expanded seed 2026111929 used 32 fresh inputs per field. The packed solver improved all 128 in both orders. Both the existing and packed solvers cleared 10× on 127/128. The remaining case was p = 43, input 9, where the packed solver achieved 9.797× and 9.823×, versus 9.311× and 9.441× for the existing solver. All scalar checks passed. This replication varies the input pairs on four fixed curves.

### Genus-4 transfer to different curves

We scanned the same family y² = x⁹ + x³ + cx + 1 at the four original primes. For each field the picker scans c in ascending order. It selects up to two smooth curves for which the place (0,1) has a certified prime-order multiple, with subgroup prime at least 1,000, at most 20 million and at least one quarter of the lower Hasse bound. The test excludes the original and previously selected values of c³ modulo p. Multiplication of x by a cube root of unity sends c to ac without changing the other coefficients, so failing to exclude these values would admit copies of the same curve.

Two curves qualified at each of p = 43 and 59. No additional orbit qualified at 37 or 67 under this picker. PARI/GP's Frobenius characteristic polynomial gives Jacobian orders 3,688,816 and 3,222,209 at 43, and 11,395,238 and 11,917,372 at 59. These distinguish the selected curves from each other and from the original curves over the same fields (orders 4,082,476 and 12,132,012). The driver uses the resulting cofactor in scalar verification.

| p, c | Three-family rank / required | Four-family poles to rank | Four-family solves ≥10× on first / second seed | Four-family median vs pencil, second seed forward / reverse |
|---|---:|---:|---:|---:|
| 43, 7 | 21 / 21 | 14 | 16/16, 31/32 | 15.60× / 15.81× |
| 43, 10 | 20 / 20 | 13 | 16/16, 32/32 | 19.67× / 20.23× |
| 59, 15 | 18 / 26 | 22 | 16/16, 32/32 | 14.80× / 15.22× |
| 59, 20 | 28 / 28 | 12 | 16/16, 32/32 | 19.33× / 19.50× |

On (59,15) all 27 poles of the three-family catalogue gave 20 accepted rows and rank 18. The packed screen returned the same deficient rank. The fourth family, fixing four copies of a point and leaving a residual of degree five, brought the combined catalogue to rank 26 after 22 poles and 28 accepted rows. The three-family arms are recorded as rank-deficient for every input on this curve. On the other three curves the packed and original three-family coordinates agreed exactly and the four-family coordinates agreed up to scalar.

The first seed (2026111930) used 16 input pairs per curve; the second (2026111931) used 32. All four-family scalars were verified. In each order the benchmark rotated the pencil reference, original three-family, packed three-family and four-family solvers for 32 repetitions per input. Each available candidate rebuilt its catalogue, decomposed the input pair and verified the scalar inside the elapsed time. Qualification and catalogue availability checks preceded timing. The reported speedup is pencil time divided by four-family time. The smallest paired median on the replication was 9.392×.

Run `harness/g4-transfer.zsh` and `harness/g4-transfer.zsh expanded`. The fixture rows give p, c, subgroup prime and the multiplier for (0,1). The rank file gives p, c, arm, anchor (−1 on exhaustion), poles, accepted rows and rank. The medians files identify curve, input and order, then give the three speedup ratios against pencil. The raw archives include both orders and all audit streams.
