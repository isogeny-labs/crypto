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
