# R7.6 results: R6.2 finished (pooled rho across an isogeny class, paired intervals, and path-finding cost against component size)

Toy instances only; every number is a count (group additions plus doublings,
field `M`/`I`, neighbour enumerations). No wall time is claimed (shared 4-vCPU
VM, other builds running, no isolation level earned). No sub-rho claim and
nothing about a deployed curve. Base `origin/main` 332716c307; the library
`src/` is unchanged. Predecessors: R6.2 (#2230, corrected by #2543); its files
under `../../round6/r6_2_pooled_rho/` are untouched, every file here is new and
versioned (`*_v1`).

## Hypotheses, falsifiers, verdicts

**H1.** Pooling targets across an isogeny class of size C gives a total rho
cost per solved instance that scales as O(sqrt(C)·sqrt(N)) (about a sqrt(C)
pooled gain) against separate rho runs. *Falsifier (as given):* the paired
ratio pooled/separate with its 95% bootstrap CI shows no gain at multiple C, or
the gain is below the sqrt prediction by more than the CI.

*Verdict H1: supported as a scaling law, falsified as the equality "gain =
sqrt(C)".* Stated so the two parts cannot be confused:

- *Gain exists:* in all 7 runs (4 values of r, 2^24.7 to 2^36.9) the gain is
  above 1 with the 95% interval excluding 1 at every C >= 4 (at C = 4 the lower
  bounds are 1.3 to 1.7); at C = 2 the interval includes 1 in two runs. The "no
  gain at multiple C" falsifier is not met.
- *Exponent:* the exponent of the gain in C over C = 2..256 is 0.430 to 0.522
  across the runs (table below); the pooled total scales with exponent 0.49 to
  0.57 and the separate total with 0.99 to 1.02. The interval contains 0.5 in 3
  of 7 runs (2^32.0 both, 2^36.9) and the ideal Kuhn–Struik value over this range,
  0.489, in all 5 runs with r >= 2^28; the two runs at r = 2^24.7 are lower
  (0.458 [0.429, 0.488] and 0.430 [0.396, 0.463]).
- *Constant:* the gain is below sqrt(C) by more than its interval from C = 32 up
  in every run. At C = 256 it is 11.0 to 13.9 against 16 (gain/sqrt(C) = 0.69,
  0.69, 0.78, 0.80, 0.83, 0.84, 0.87 for r = 2^24.7 to 2^36.9), because the ideal
  Kuhn–Struik gain at C = 256 is itself 14.19 (0.887·sqrt(C)). Against that ideal,
  the measured pooled/separate at C = 256 is 1.28, 1.29, 1.14, 1.11, 1.06, 1.05
  and 1.02 times the ideal for r = 2^24.7 to 2^36.9; only the largest run's
  interval contains 1 ([0.994, 1.053]). So the literal reading of the falsifier (gain below sqrt(C)
  by more than the CI) holds; the scaling reading (exponent about 0.5, gain equal
  to the ideal at large r) survives.
- *Transport control (the R6.2 open item):* B/C (class targets transported to
  E_0 and pooled, over same-curve targets pooled with the same seed) has its
  95% interval excluding 1 in 5 of 56 (run, C) cells (4 above 1, 1 below; none
  in the same direction across runs), and inside [0.95, 1.05] in 13 of the 14
  cells with C >= 128 (18 of 56 overall; at smaller C most intervals are wider
  than the band, so equivalence is shown at large C, and at small C only
  "no difference detected"). The two arms are the same walk on points
  of the same law (transport scales the logarithm by a unit), so 1 is expected.
  Pricing the transport into the pooled arm (61 to 163 M and 4 to 18 I per target)
  raises pooled/separate at C = 256 by 0.2% to 5.2% at I = 10 M and leaves the
  exponent and the verdict unchanged.

**H2.** The corrected path-finding cost per pair is O(sqrt(C)) for a bidirectional
search. *Falsifier:* the log-log exponent of cost against component size, with
95% CI over >= 6 sizes spanning >= 2 decades, excludes 0.5 (or is not
distinguishable from it only for lack of data: reported as undetermined).

*Verdict H2: refuted for this search.* Over 40 classes (p of 12 to 34 bits),
32 distinct component sizes from 8 to 16 702 curves (3.3 decades; 230 pairs, every
route re-validated forward), the exponent of the total cost per pair
(search enumerations + certifications + validation enumerations) is **1.13
[1.06, 1.21]**; for search enumerations alone 1.13 [1.05, 1.22]. 0.5 is outside
every interval, including the sub-fits (>= 100 curves: 1.09 [0.91, 1.30]; < 1000
curves: 1.16 [0.98, 1.28]; >= 1000 curves, 11 classes: 1.28 [0.75, 1.90], wide).
Exponent 1 is excluded by the all-classes fit only mildly and is inside the
interval of each sub-fit with >= 100 curves; the data say "about linear, slightly
above", not a more precise law. Total per pair over C is 1.8 (C near 13) rising to
3.0 to 4.4 above 300 curves. The component fraction the search expands (search
enumerations / 4C, derived) is 0.44 on average for C >= 100 (0.03 to 0.68 over 19
classes). The R6.2 statement "the O(sqrt(class size)) hypothesis is not shown"
becomes "contradicted at 3.3 decades". This is a statement about breadth-first
bidirectional search over certified l in {2, 3, 5, 7} edges; it says nothing about
an Elkies walk or other path-finding.

## Batch half (measured, `runs/batch_r*_sym*_v1.json`)

Command: `r7_6_r62_batch --mode batch --rbits R --sym S --trials T --seed 1
--nboot 10000` (release). Per trial one planted key on each of the first 256
curves of a breadth-first class of the base curve E_0; the separate arm solves
each transported target by its own `rho_walk_with` (ecbench `rho.negation`), the
pooled arms are `rho_batch_with` (ecbench `rho.multitarget_pooled`) on the first
C targets (C = L), pooled B on class targets, pooled C on fresh E_0 targets
(control). Separate and pooled B share keys and seeds (separate cost at C is the
prefix sum). Intervals: percentile bootstrap over trials. Every recovered log was
verified on its original curve (181 000+ logs, `verified_logs` per file).

| run | p | r (= 2^) | h | class curves | edges by l (2, 3, 5, 7) | edges dropped, no dual (5, 7) | trials | verified logs |
|:--|--:|--:|--:|--:|:--|:--|--:|--:|
| r24 sym0 | 822259187 | 24.71 | 30 | 256 | 1, 128, 0, 126 | 247, 0 | 30 | 38280 |
| r24 sym1 | 2603577871 | 24.66 | 98 | 256 | 1, 8, 60, 186 | 0, 0 | 30 | 38280 |
| r28 sym0 | 11018534707 | 28.04 | 40 | 256 | 129, 126, 0, 0 | 246, 246 | 24 | 30624 |
| r28 sym1 | 59119696609 | 28.88 | 120 | 256 | 5, 6, 55, 189 | 0, 0 | 24 | 30624 |
| r32 sym0 | 923790068429 | 32.03 | 210 | 256 | 1, 30, 224, 0 | 0, 195 | 14 | 17864 |
| r32 sym1 | 842728029151 | 32.02 | 194 | 256 | 1, 104, 58, 92 | 0, 0 | 14 | 17864 |
| r36 sym1 | 1498933511111 | 36.86 | 12 | 256 | 3, 68, 64, 120 | 0, 0 | 6 | 7656 |

`sym1` draws p = +-1 mod 5 and mod 7, so every l = 5, 7 edge has a certified
dual; `sym0` leaves p free, and the l = 5 or 7 edges without a dual are dropped
(the corrected rule; their counts are in the column "edges dropped"). The r28
sym0 base curve is the R6.2 batch's base curve (same seed), which is why it has
l = 2, 3 edges only. Edges in the table are the edges to new curves found in the
breadth-first build of the 256-curve class.

Pooled/separate, gain and exponents (95% bootstrap over trials; "KS" is the ideal
Kuhn–Struik ratio at the same C, a closed form; pooled/separate over KS above 1
means more pooled cost than ideal):

| run | gain at C = 64 | gain at C = 256 | gain/sqrt(C) at 256 | pooled/separate over KS at 256 | gain exponent in C (2..256) | pooled-cost exponent | separate-cost exponent |
|:--|:--|:--|:--|:--|:--|:--|:--|
| r24 sym0 | 6.49 [6.34, 6.65] | 11.08 [10.93, 11.22] | 0.692 [0.683, 0.701] | 1.281 [1.264, 1.297] | 0.458 [0.429, 0.488] | 0.551 [0.533, 0.569] | 1.009 [0.986, 1.032] |
| r24 sym1 | 6.55 [6.40, 6.70] | 11.01 [10.88, 11.15] | 0.688 [0.680, 0.697] | 1.288 [1.272, 1.304] | 0.430 [0.396, 0.463] | 0.569 [0.551, 0.590] | 0.999 [0.976, 1.024] |
| r28 sym0 | 6.64 [6.48, 6.81] | 12.40 [12.18, 12.61] | 0.775 [0.761, 0.788] | 1.144 [1.126, 1.164] | 0.468 [0.442, 0.495] | 0.525 [0.504, 0.550] | 0.993 [0.972, 1.015] |
| r28 sym1 | 6.79 [6.57, 6.99] | 12.76 [12.55, 12.97] | 0.798 [0.784, 0.811] | 1.112 [1.094, 1.130] | 0.468 [0.440, 0.498] | 0.532 [0.507, 0.557] | 1.000 [0.980, 1.023] |
| r32 sym0 | 6.81 [6.60, 7.04] | 13.33 [13.12, 13.55] | 0.833 [0.820, 0.847] | 1.064 [1.047, 1.082] | 0.466 [0.430, 0.506] | 0.533 [0.509, 0.559] | 0.998 [0.970, 1.030] |
| r32 sym1 | 6.98 [6.77, 7.21] | 13.51 [13.17, 13.84] | 0.844 [0.823, 0.865] | 1.050 [1.025, 1.078] | 0.514 [0.461, 0.563] | 0.504 [0.474, 0.540] | 1.018 [0.990, 1.048] |
| r36 sym1 | 6.94 [6.56, 7.29] | 13.88 [13.49, 14.28] | 0.868 [0.843, 0.892] | 1.022 [0.994, 1.053] | 0.522 [0.461, 0.590] | 0.494 [0.465, 0.527] | 1.016 [0.981, 1.072] |

Predictions: gain exponent 0.5 (strict) or 0.489 (ideal Kuhn–Struik over
C = 2..256, native `--mode fit`); pooled-cost exponent 0.5; separate-cost
exponent 1. gain/sqrt(C) at C = 64: 0.81 to 0.87; at C = 16: 0.82 to 0.91 (full
per-C tables of every run, with pooled/separate, gain, gain/sqrt(C), the KS ratio
and B/C: `reports/run-20261101T0039Z/report.pdf` and the JSON). Pooled/separate
at C = 256 is 0.0720 [0.0700, 0.0741] at r = 2^36.9 (ideal 0.0705) and 0.0903 to
0.0908 at r = 2^24.7. The distance from the ideal shrinks as r grows at fixed C
(1.28 at 2^24.7 to 1.02 at 2^36.9); its cause is not tested here (a finite-r
overhead that grows with C is the obvious candidate and is a proposal, not a
result).

Transport control: B/C interval excludes 1 at r28 sym0 C = 4, 8, 16 (1.144
[1.012, 1.288], 1.086 [1.001, 1.179], 1.101 [1.043, 1.167]), r28 sym1 C = 256
(0.984 [0.970, 0.999]) and r36 sym1 C = 64 (1.054 [1.008, 1.109]); R6.2's point
estimates for the r28 sym0 base curve at C = 4 and 16 were 0.914 and 0.966, so the
direction is not reproducible, consistent with chance in 56 intervals that share
trials. Transport costs 61 to 163 M and 4 to 18 I per target in these classes
(R6.2: 163.0 M + 18.1 I at its largest L, same base curve; here 161.8 M + 18.0 I).

## ecbench half (session `ECBS1h4b0bd52ebc3a`, `sessions/pooled-vs-rho-l0/`)

`spec.json` (`ECS1h0f7350931294`): `rho.negation` (reference) against
`rho.multitarget_pooled` with `batch` = 4, 16, 64, 256, on the registered curves
`icv1-fp20-t727-cd198a38`, `icv1-fp24-t1059-6df599df`,
`icv1-fp28-tm3161-380f655a`, `icv1-fp32-t74357-b7396dac` (16 workloads each, 2
measured rounds, no warm-up, `--cpus none`: L0, operation counts only). 640
records, all verified. The pooled arm has the record's target plus L-1 public
companions on the *same* curve, so it is the control arm C of the class
experiment, not the transported arm B (the harness has no method that transports
targets along an isogeny class; adding one is an `ecbench-extend` task, not done
here). The ratio is the harness's ratio of totals over matched (workload, round)
pairs, `same_seeds: true`; pooled/separate and the gain are derived as ratio/L
and L/ratio (separate cost taken as L times the mean single-target S).

| L | ecbench ratio pooled/rho.negation, 64 workloads, 128 pairs | pooled/separate (derived) | gain (derived) | per-curve gain (r = 2^19.1, 2^24.0, 2^28.0, 2^31.7) |
|--:|:--|:--|:--|:--|
| 4 | 2.310 [2.097, 2.549] | 0.5775 [0.5243, 0.6372] | 1.73 [1.57, 1.91] | 1.66 ; 1.74 ; 1.66 ; 1.90 |
| 16 | 4.991 [4.549, 5.486] | 0.3119 [0.2843, 0.3429] | 3.21 [2.92, 3.52] | 2.94 ; 3.25 ; 3.21 ; 3.59 |
| 64 | 11.354 [10.309, 12.524] | 0.1774 [0.1611, 0.1957] | 5.64 [5.11, 6.21] | 4.61 ; 6.00 ; 5.93 ; 6.96 |
| 256 | 28.164 [24.968, 31.609] | 0.1100 [0.0975, 0.1235] | 9.09 [8.10, 10.25] | 6.35 ; 10.12 ; 10.90 ; 13.41 |

(Cluster bootstrap over workloads; the harness's linearised-t interval, also
recorded, is slightly narrower. Per-curve intervals, 16 workloads each, are in
the comparison files and are wide.) The harness finds a gain at every L
(interval excluding 1). Gain rises with r at L = 256 (6.4, 10.1, 10.9, 13.4) and
at 2^31.7 agrees with the example (13.4 against 13.3 to 13.5); at 2^28.0 it is
lower than the example (10.9 against 12.4 to 12.8) because its reference is one
`rho.negation` run per workload and round (per-curve mean S of the reference:
1.292, 1.016, 0.867, 0.959 for 2^19.1 to 2^31.7, against about 1.04 over all
four), while the pooled arm's own S at L = 256 (52.1, 25.7, 20.4, 18.3) matches
the example (20.2 at 2^28.04, 18.4 at 2^32.03; R6.2: 20.2). The
pooled-vs-single ordering holds at every pricing weight at every L (0, 0.1, 1
GAE per `canonicalisations_uncharged`, a counter both arms carry). The harness's
wall-time fields are not used: L0, contended. An earlier session of the same
workloads (1152 records: a warm-up round and an A/A arm, whose `rho-neg` against
`rho-neg-aa` operation ratio was exactly 1) gave 2.297, 4.914, 11.263 and 27.934
at L = 4, 16, 64, 256 (receipt SHA-256 67496bbed2ef...); it is not committed
because its 16.5 MB `records.jsonl` is over the repository's 10 MiB gate, and the
committed session drops those two features (counts are deterministic).

Audit (`ecbench verify --replay-all --exit-code`): 640 records, 640 verified, 640
replays reproduced, 0 problems. Receipt
`sessions/pooled-vs-rho-l0.audit.json`, SHA-256
`e120c0cb25583e61b40c14053070fc87240def172853a7b319d725114f783780`. The auditor
ran on the session's own host class (env class `ECBENV3h13e37662a118`), so this is
a same-host replay certificate and **not** an independent one; the independent
receipt (another host class) and any L2 session need the isolab/EKS route, which
is reserved for benchmark admission and was not used (no task card). **No
admitted benchmark and no `vs_rho` claim is made.** The ecbench binary
(SHA-256 `38e694e8b30f51e61b497df35753f52188a1f967ff9f0f1a2e37e030befe35a0`)
was built from `origin/main` 332716c307 plus untracked files only (the harness
records `commit ?`).

## Path half (measured, `runs/scaling_v1.json`, `runs/scaling_v2.json`, `runs/scaling_fit_v1.json`)

Commands (`--mode scaling`): v1 `--pbits 12,14,...,30 --classes 3 --pairs 6
--cap 12000 --attempts 30 --seed 7`; v2 `--pbits 26,28,30,32,34 --classes 2
--pairs 5 --cap 24000 --attempts 25 --seed 8`; fit `--mode fit --inputs
scaling_v1.json,scaling_v2.json --nboot 10000`. Each class is a random curve whose
certified component (edges kept only where `SmallIsogeny::dual` certifies the
reverse edge, l in {2, 3, 5, 7}) has 8 to cap curves, with >= 2 degrees having a
rational kernel; pairs are uniform in the component; the search is the R6.2
corrected `search` over `EdgeSet::Certified` and each returned route is
re-validated forward. Cost per pair = search enumerations + certifications +
validation enumerations (R6.2's definition). Bootstrap: classes, then pairs
within a class, 10000 resamples.

| subset | cost | classes | sizes | exponent (95%) | contains 0.5 | contains 1 |
|:--|:--|--:|:--|:--|:--|:--|
| all | total | 40 | 8..16702 | 1.130 [1.059, 1.207] | no | no |
| all | search only | 40 | 8..16702 | 1.132 [1.054, 1.215] | no | no |
| size >= 100 | total | 19 | 129..16702 | 1.085 [0.911, 1.297] | no | yes |
| size >= 1000 | total | 11 | 1050..16702 | 1.276 [0.746, 1.904] | no | yes |
| size < 1000 | total | 29 | 8..728 | 1.159 [0.978, 1.284] | no | yes |
| >= 3 degrees with a kernel at the start curve | total | 17 | 8..16702 | 1.097 [0.954, 1.217] | no | yes |
| <= 2 degrees | total | 23 | 8..13760 | 1.160 [1.077, 1.269] | no | no |

| size bin | classes | mean size | mean total per pair | total/C mean (min..max) | total/sqrt(C) mean |
|:--|--:|--:|--:|:--|--:|
| 8..29 | 13 | 13 | 25 | 1.79 (0.95..3.23) | 6.5 |
| 30..99 | 8 | 57 | 74 | 1.45 (0.35..4.28) | 10.3 |
| 100..299 | 3 | 166 | 437 | 2.59 (0.79..4.10) | 33.3 |
| 300..999 | 5 | 520 | 2026 | 3.87 (2.61..4.51) | 87.9 |
| 1000..2999 | 5 | 1717 | 5429 | 3.01 (0.33..4.63) | 127.0 |
| 3000..9999 | 3 | 5974 | 25324 | 4.43 (2.64..5.72) | 332.6 |
| 10000..99999 | 3 | 13924 | 44439 | 3.29 (2.30..3.85) | 381.1 |

Total/sqrt(C) grows about 60-fold across the table where the hypothesis needs
it flat. Per class the ratio total/C ranges from 0.33 to 5.72, so the component
size is not the only variable (the structure of the class graph matters; one
class of 1050 curves costs 0.33 C); the exponent is a fit across classes, not a law
of one class. Proposed, untested mechanism: the graph is a Cayley graph of the
class group on at most 4 small primes, whose ball growth is polynomial in the
radius, so bidirectional search expands a constant fraction of the component
(measured 0.44 on average above 100 curves) rather than its square root.

## Controls and checks

- The corrected search and its R6.2 controls are the same code; the example's
  unit tests (`cargo test --release --example r7_6_r62_batch`) check the
  Kuhn–Struik ratio, the bootstrap and OLS helpers, forward validation of certified
  routes on a prime with p not +-1 mod 5, 7, and that `--sym` primes keep the
  l = 5, 7 edges.
- Every pooled and separate log was verified on its original curve in the batch
  half (assertions in the run); every route in the path half was re-validated
  forward (assertion in the run); the ecbench session passed `verify --replay-all` (640 of 640).
- Replay of the native records: `replay_check.py` re-runs each recorded command
  with the committed binary source and compares everything except the contended
  wall-time note; result in `replay_certificate.json` (the 7 batch records and
  the v1 scaling and fit records are replayed; `scaling_v2.json`, 33 minutes, is
  not replayed in this session).

## Reproduce

From the repository root, release builds, `B=target/release/examples/r7_6_r62_batch`:

```sh
cargo build --release --example r7_6_r62_batch --bin ecbench
R=research/isogeny_fibers_20261010/round7/r7_6_r62_batch
$B --mode batch --rbits 24 --sym 0 --trials 30 --seed 1 --nboot 10000 --out $R/runs/batch_r24_sym0_v1.json
$B --mode batch --rbits 24 --sym 1 --trials 30 --seed 1 --nboot 10000 --out $R/runs/batch_r24_sym1_v1.json
$B --mode batch --rbits 28 --sym 0 --trials 24 --seed 1 --nboot 10000 --out $R/runs/batch_r28_sym0_v1.json
$B --mode batch --rbits 28 --sym 1 --trials 24 --seed 1 --nboot 10000 --out $R/runs/batch_r28_sym1_v1.json
$B --mode batch --rbits 32 --sym 0 --trials 14 --seed 1 --nboot 10000 --out $R/runs/batch_r32_sym0_v1.json
$B --mode batch --rbits 32 --sym 1 --trials 14 --seed 1 --nboot 10000 --out $R/runs/batch_r32_sym1_v1.json
$B --mode batch --rbits 36 --sym 1 --trials 6  --seed 1 --nboot 10000 --out $R/runs/batch_r36_sym1_v1.json
$B --mode scaling --pbits 12,14,16,18,20,22,24,26,28,30 --classes 3 --pairs 6 --cap 12000 --attempts 30 --seed 7 --nboot 10000 --out $R/runs/scaling_v1.json
$B --mode scaling --pbits 26,28,30,32,34 --classes 2 --pairs 5 --cap 24000 --attempts 25 --seed 8 --nboot 10000 --out $R/runs/scaling_v2.json
$B --mode fit --inputs $R/runs/scaling_v1.json,$R/runs/scaling_v2.json --nboot 10000 --out $R/runs/scaling_fit_v1.json
target/release/ecbench run --wait --cpus none --spec $R/spec.json --out $R/sessions/pooled-vs-rho-l0
target/release/ecbench verify --dir $R/sessions/pooled-vs-rho-l0 --replay-all --exit-code --out $R/sessions/pooled-vs-rho-l0.audit.json
for L in 4 16 64 256; do target/release/ecbench compare --dir $R/sessions/pooled-vs-rho-l0 --a rho-neg --b pooled-L$L --save; done
python3 -I $R/replay_check.py $B /tmp/r76-replay --skip scaling_v2.json
python3 $R/reports/run-20261101T0039Z/plot.py && python3 -I $R/reports/run-20261101T0039Z/print_tables.py
```

The example's batch and scaling modes are deterministic in their seeds (replayed
identical). The binary that produced the committed records was built before a
whitespace-only `rustfmt` pass and a test-only edit; `replay_certificate.json`
records its SHA-256.

## Graphs checked

New in `reports/run-20261101T0039Z/`: `fig1_batch_gain_vs_L.svg`,
`fig2_batch_exponent_and_control.svg`, `fig3_ecbench_pooled_gain.svg`,
`fig4_path_cost_vs_component.svg` (plot source `plot.py`, data in `runs/` and
`sessions/`), `report.typ` and `report.pdf` (inspected page by page). The R6.2
graphs (`../../round6/r6_2_pooled_rho/reports/`) are frozen records and are left
unchanged; the R6.2 batch graph is superseded by fig1 for the corrected re-run
(the R6.2 batch itself was not wrong, only unpaired). No scoreboard, leaderboard
or lab-browser entry changes: nothing here measures an index-calculus variant.
PNG renders are not committed (the artifact policy sends a PNG over 64 KiB to the
store, which needs credentials this session does not have); `plot.py` writes them
beside the SVGs.

## Limits

- ecbench: same-curve pooled arm only (not the transported arm), L0 counts-only,
  single host class, same-host receipt; the reference arm is one run per workload,
  so its intervals are wider than the example's, which runs every separate walk. No
  isolated or independent-runner measurement, no EKS/isolab run, no admitted
  benchmark. No wall time is reported.
- The class pooled here is the first C curves of a breadth-first class (C <= 256),
  one base curve per run (seven base curves over four r values: two each at
  2^24.7, 2^28 and 2^32, one at 2^36.9), not the whole isogeny class; p is a toy size (r from 2^24.7 to
  2^36.9, p to 2^41), keys and curves are unregistered toy instances identified by
  p, a, b in the records. C up to 256 only; the dependence on L/sqrt(r) at larger C
  is not measured. Trials: 30, 24, 14 and 6 by r; the 2^36.9 intervals are wide
  (6 trials).
- Path half: 40 classes, 5 to 6 pairs each, sizes capped at 12 000 (v1) or 24 000
  (v2), so components above the cap are excluded and the sample leans toward small
  components at p >= 2^30 (one, one and two skipped over the cap at 30, 32, 34 bits
  in v2); pairs uniform in the component; the bidirectional breadth-first search on
  a toy class is not an Elkies walk. The exponent is a fit across classes of
  different structure; only 11 classes exceed 1000 curves.
- `scaling_v2.json` was not replayed; ideal-KS comparisons use the closed form, not
  a simulation.

## Next step

(1) A transported-target pooled method in ecbench (`ecbench-extend`), so the
class experiment lands in a registered, replayable session, and an independent
host-class replay with the task card. (2) Path finding that does not enumerate: a
walk with Phi_l root-finding or a distance-halving heuristic, measured against
the same component-size axis. (3) Larger C and r in the batch half to separate the
finite-r overhead from the exponent.

## Note text (ready to paste into the consolidating section)

R7.6 (`research/isogeny_fibers_20261010/round7/r7_6_r62_batch/`) finishes R6.2.
The batch half, re-run with the corrected certified-edge rule (an l = 5, 7 edge
is in the class only where `SmallIsogeny::dual` certifies the reverse edge) at
four values of r (2^24.7 to 2^36.9, seven runs, 6 to 30 trials each), measures
the gain of pooling one target per class curve (C = L up to 256) over separate
rho runs on the same keys and seeds, with percentile-bootstrap intervals over
trials. The gain is above 1 with an interval excluding 1 at every C >= 4 in every
run; its exponent in C over 2..256 is 0.43 to 0.52 (the ideal Kuhn–Struik value
over that range is 0.489, inside the interval at every r >= 2^28), and at C = 256
the gain is 11.0 to 13.9, rising with r towards the ideal 14.2, against the 16
that "gain = sqrt(C)" would give. So the O(sqrt(C)·sqrt(N)) pooled total is
supported as a scaling law and the literal sqrt(C) gain is not: it is below
sqrt(C) by more than the interval from C = 32 in every run, by the Kuhn–Struik
constant at large r and by up to 29% more at r = 2^24.7. The transported arm B
and the same-curve control C differ by no consistent amount (5 of 56 intervals
exclude 1, in mixed directions; inside [0.95, 1.05] at C >= 128 in 13 of 14
cells), closing the open B/C item of R6.2; pricing the transport (61 to 163 M and
4 to 18 I per target) moves the C = 256 ratio by at most 5%. The ecbench session
`ECBS1h4b0bd52ebc3a` (L0, counts only; 640 records, `verify --replay-all`
reproduced 640 of 640, receipt SHA-256 e120c0cb...f783780, same host class, so
not an independent certificate) gives the same-curve pooled arm against
`rho.negation` at r = 2^19 to 2^32: pooled/separate 0.110 [0.098, 0.124] at
L = 256 (gain 9.1 [8.1, 10.3]); its single-run reference makes the per-curve
intervals wide, and it does not contain the transported arm.

The path half asks whether the corrected bidirectional search costs O(sqrt(C))
enumerations per pair. Over 40 toy classes with 32 distinct certified component
sizes from 8 to 16 702 curves (3.3 decades), the exponent of the cost per pair
(search + certification + validation enumerations) against component size is 1.13
[1.06, 1.21]; 0.5 lies outside every interval, so the sqrt(C) hypothesis is
refuted for breadth-first search over l in {2, 3, 5, 7} certified edges (about
0.44 of the component is expanded for C >= 100). This does not bear on an Elkies
walk. Limits: toy sizes (r <= 2^37), one base curve per run, C <= 256 in the batch
half, 5 to 6 pairs per class and a size cap in the path half, same-curve pooling
only in ecbench, no isolated or independent run and no admitted benchmark.
