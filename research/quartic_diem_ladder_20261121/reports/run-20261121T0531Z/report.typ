#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt, inset: 3.5pt)
= Lead \#1 ladder: Diem 2LP index calculus on prime-order plane quartics, sampled factor base, against matched rho

Run report, study `research/quartic_diem_ladder_20261121`, runs 2026-11-21 05:31 to 09:05 UTC. *Every number is \[pcc:unverified\]*: one shared host, planted targets, no replay certificate, no VALIDATION.json. Not a leaderboard, benchmarks or tier row.

*Binding.* PROTOCOL.md sha256 `96311c59…2ec2`, frozen before any measured run (with analyze.py and the run scripts). Binary `genus_ladder.qsfb` sha256 `f0243cf8…e38b`. Raw data in `raw/`, hashed in `data.sha256`.

*What ran.* 8 primes, 1031 to 131101 (2#super[10] to 2#super[17]). 5 prime-order quartics per prime, 2 solver seeds, arms `2lp-new` (enumerated factor base) and `2lp-sfb` (sampled factor base, step A). 160 IC runs, all certified with the recovered logarithm recorded. 50 measured negation-rho runs at p ≤ 16411, all certified. Host: shared 4-core VM, at most 2 threads.

*Reference.* Ideal negation rho $sqrt(pi r slash 4)$ Pic additions, nothing unpriced (L10-corrected). The primary metric is GAE#sub[w] = group_ops + w · (all counted native operations), with w ∈ {0, 0.1, 1}.

== Verdicts (frozen rules)
- *Band, every prime, both arms: UNRESOLVED.* IC is far below rho at w = 0 and above it at w = 1.
- *Gap closing at every w.* Every upper 95% bound of the ln(IC/rho) slope is below 0.
- *Step A accepted.* Paired FPM mid sfb/new is 0.683 \[0.640, 0.729\] and all sfb runs certify.
- *Lead not stopped.* FPM-mid exponent: sfb 0.863 \[0.834, 0.891\], new 0.924 \[0.893, 0.953\].

#figure(image("fig_ladder.svg", width: 100%), caption: [Left: IC / ideal negation rho at the three weights, geometric mean over 5 curves with cluster-bootstrap 95% intervals (n = 10 runs per arm and prime). Right: counted F#sub[p] multiplications per solve, and the ideal rho priced at the measured Pic cost. Study quartic_diem_ladder_20261121.])

== IC / rho (primary band, geo-mean over 5 curves, boot95)
#table(columns: 7,
[p], [new w=0], [new w=0.1], [new w=1], [sfb w=0], [sfb w=0.1], [sfb w=1],
[1031], [0.0125], [7.37], [73.5], [0.0089], [4.58], [45.7],
[4099], [0.0028], [3.97], [39.7], [0.0027], [2.38], [23.8],
[16411], [0.0010], [2.63], [26.3], [0.0009], [1.76], [17.6],
[65537], [0.0003], [1.53 \[1.33, 1.81\]], [15.3], [0.0003], [0.90 \[0.83, 1.005\]], [8.96],
[131101], [0.0002], [1.23 \[1.12, 1.35\]], [12.3], [0.0002], [0.77 \[0.68, 0.88\]], [7.75 \[6.75, 8.85\]],
)
The w = 1 reading is driven by `la_add`: graph, DSU and AᵀDA additions, 85% of sfb's native operations at 131101.

Sensitivity at 131101, sfb (the verdict does not change):
- Mul-only band at w = 1: 0.913 \[0.834, 0.999\].
- FPM against the ideal rho in FPM: 0.0010.
- Count-calibrated, at 10 w#sub[cnt]: 0.0195. At w = 0, w#sub[cnt] and 10 w#sub[cnt], IC is below rho at every prime in both arms. This calibration is count-based, not timed.

== Exponents (8 primes, boot95)
#table(columns: 3,
[metric], [2lp-new], [2lp-sfb],
[FPM mid], [0.924 \[0.893, 0.953\]], [0.863 \[0.834, 0.891\]],
[rel_mul], [1.086], [0.971],
[rel_enum_mul (set-up)], [1.113], [0.616],
[la_mul], [0.975], [0.968],
[all native ops], [1.142], [1.135],
[group_ops], [0.687], [0.672],
[decomp_trials], [0.791], [0.793],
[ln(IC/rho), w = 1], [-0.359], [-0.365 \[-0.390, -0.342\]],
)
No sub-1 slope is extrapolated. The extrapolated crossover at w = 1 (a prediction) for sfb is log#sub[2] p\* = 24.9 \[24.0, 25.8\].

== Step A, paired sfb/new (pooled)
- FPM mid: 0.683.
- rel_mul: 0.287.
- rel_enum_mul: 0.028.
- la_mul: 0.918.
- All native operations: 0.606.
- group_ops (w = 0): 0.846.
- Relations: 0.995.

Sampling about 3√p fibres (1,097 at 131101) replaces the enumeration of all p points. The tests show identical relations when the sampled arm is given the same base. An exhaustive check on tiny p shows that membership agrees with the enumerated index.

== Calibration, deviations and scope
- *Calibration.* The executed negation rho costs 2.1 to 2.3 times the plain Pic operation per step (canonicalisation; L10 gives 2.45). Its step counts are 0.70 to 1.19 times the ideal.
- *Deviations.* The container restarted before freeze (no data lost). The 110-minute search cap was not enforced for one 131101 process during a session pause; the same curves would have been used. The budget was 3.6 h wall and about 6 core-hours against about 3 h, almost all of it curve admission. The disk filled on 2026-11-23; every file was re-verified by hash and by reproduction.
- *Scope.* Planted, warm, one session. The exponents are local (2#super[10] to 2#super[17]). Before any below-rho statement, the 131101 rows need the validation gate.
