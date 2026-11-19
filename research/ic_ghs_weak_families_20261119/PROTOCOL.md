# PROTOCOL: what makes the GHS-weak rows weak, and which further curves are weak

Frozen 2026-11-19, before any measurement of this study and before `analysis/fit_criterion.py` was run.
Not edited afterwards; deviations go in README.md. Every number produced is `[pcc:unverified]`.
Single target, charged cold. No multi-target or batch method is used or priced.

## Question

The IC leaderboard "GHS-weak" rows are E0/F_{q^3} (q prime, q = 7 mod 12, 4 | #E0(K), special m = 2 form)
solved through the genus-3 plane quartic over F_q (study `research/quartic_n4_20261019`, v2 in
`research/quartic_n4v2_20261020`, latest configuration `research/ic_gw_nrm_lz_20261115`).
(1) What exactly makes them fall below rho (genus, field sizes, cost exponents)?
(2) Which further curves over F_{2^N} (composite N) are weak under the same mechanism, and how many per field?
(3) Does one newly predicted weak instance, measured end to end, land below its matched rho?

## Scope facts read before registering (not data of this study)

* The measured rows are in odd characteristic (prime q). The binary analogue is the GHS/Hess descent over
  F_{2^N}, N = n*l, whose cover is hyperelliptic (never a plane quartic), with the closed forms of
  `research/ghs_n3_genus3_census_20261024` (n = 3: class A genus 1, class B genus 3, class C genus 4).
* No solver exists in this checkout for binary GHS covers of genus >= 3 at a size where it competes with rho
  (`research/ghs_generalized_20261024` is toy-sized, N <= 12). The only end-to-end solver present is the
  odd-characteristic plane-quartic pipeline (`research/ic_gw_nrm_lz_20261115/src/v3.rs`).
  Therefore the binary census is a desk result (cost-model classification), and the measured new instance is
  in the odd-characteristic family. Both are labelled as such.

## Criterion C (the cost-model inequality)

A curve E/F_{q^n} (q = 2^l or prime) with a transfer to Jac(C)/F_q, C of genus g, is *weak* when the best
index calculus on Jac(C) is cheaper than rho on E.

* C-asym (exponents): e_IC(C) < n/2, with e_IC = 1 for a non-hyperelliptic genus-3 plane quartic (Diem),
  e_IC = 2 - 2/g for a hyperelliptic genus-g cover (Gaudry; double large primes change the constant, not
  the exponent used here), and n/2 the rho exponent on a group of order about q^n.
* C-fin (finite size, used for the binary census): weak iff kappa * g! * q^(2-2/g) < 0.63 * 2^(N/2)
  (rho on E: sqrt(pi r / 4) with r = #E / 2, constants rounded down), kappa in {1, 10, 100}. kappa = 1 is the
  most generous to IC, so its weak set is an upper bound on what is weak. Genus from the magic number m:
  type I (x+1 does not divide Ord_b) g = 2^(m-1) - 1, type II g = 2^(m-1), m = deg Ord_b + [type I]
  (MMT, LMS JCM 5, 2002, Thm 5; as stated in `examples/ghs_magic_spectrum.rs`).
* C-fit (measured family): a log-linear model ln cost = a + e ln q fitted per arm (IC arms B0, D, DL, DL1;
  rho arms plain, neg) by ordinary least squares in M-eq at w = 0 from the committed
  `research/quartic_n4v2_20261020/results/summary.json` (`ic_size`, `rho_size` rows); predicted
  IC/rho(q) = exp(a_IC - a_rho) q^(e_IC - e_rho). Weak at q iff predicted ratio < 1.

Not used as evidence: the three-significant-figure leaderboard tables (they are read only to know which
arms and sizes exist). The held-out design is therefore structural (fit window vs test window), not blind:
the author had read the leaderboard outcome before registering. This is stated here, not hidden.

## Predictions (falsifiable; decided by `analysis/fit_criterion.py` and `analysis/census.py`)

* **P1 (held-out classification).** Fit window q <= 4051 (k <= 12) only. For every held-out size q >= 8191
  with a frozen-valid rho row (v2 rule: `rho_excluded` sizes are not used) and for each of IC arms
  B0, D, DL, DL1 against rho plain and rho neg: the sign of ln(predicted ratio) equals the sign of
  ln(measured ratio of the means at w = 0). PASS if at least 90% of the pooled held-out (arm, rho, q) cells
  agree; the number of disagreements is reported per arm.
* **P2 (crossover).** The crossover q* (predicted ratio = 1) of B0 vs plain rho from the fit window q <= 4051
  lies within a factor of 4 of the crossover of the full-range fit (all sizes).
* **P3 (negative control: the mechanism is the factor base, not the cover alone).** On the same curves the
  baseline arm A0 (all q+1 rational points as factor base; `quartic_n4_20261019/results/summary_all.json`)
  has fitted exponent above rho's 3/2 and is never weak; B0, D, DL, DL1 have fitted exponent below 3/2.
  Expected exponent of every non-A0 arm in [1.0, 1.4] (Diem exponent 1 plus polylog drift).
* **P4 (second family, genus-3 hyperelliptic cubic GHS, `research/ghs3_vs_rho_20261023/TABLES.md`).** The
  criterion e_IC = 4/3 < 3/2 says weak only asymptotically and by a thin margin: the fitted slope of
  ln(IC / R_ideal) against ln q over log2 q = 14..20 is negative and above -0.25, and IC/R_ideal at every
  measured size stays at or above 0.5, i.e. no resolved win at q <= 2^20.
* **P5 (binary census closed forms).** For N = n*l with n odd, the number of b in F_{2^N}* with Ord_b a product
  of the irreducible factors in a set S of x^n - 1 is prod_{i in S}(2^{d_i l} - 1); the n = 3 values equal
  the exhaustive counts of the committed census (A: 2^l - 1, B: 2^{2l} - 1, C: (2^l - 1)(2^{2l} - 1)), and a
  brute-force recount in F_{2^N} at N = 6, 9, 12, 15 (odd n | N, l >= 2) gives zero deviations.
* **P6 (weak set under C-fin, kappa = 1).** For every N <= 96 the only (n, class) pairs that satisfy C-fin
  with b outside every proper subfield and n >= 3 odd are n = 3 class B (genus 3) from l >= some l0(kappa)
  and, for n >= 5 odd, none with g >= 7 at N <= 96. Class C (n = 3, genus 4) is never weak
  (24 q^(3/2) > 0.63 q^(3/2)). Reported as a count per N of weak b and its fraction of 2^N - 1.
* **P7 (measured new instance).** At three fields not in any earlier grid, q in {98299, 196579, 393187}
  (primes, q = 7 mod 12), three new instances each (seeds 1..3 of the standard instance stream) and the
  matched rho (plain and negation) on the same instances, the IC arms DL1 and B0 are below both rho arms
  at w = 0, 0.1, 1 in every one of the 9 instances, and the measured ratio lies within a factor 1.5 of the
  C-fit prediction (full-range fit) for the field-mean ratio. FAIL if any IC/rho field mean exceeds 1 or the
  factor-1.5 band is violated.
* **P8 (admissible fraction, new fields).** The standard instance search (`v3 instances`) needs on average
  between 2 and 60 candidates per instance at each of the three P7 fields (the density of the weak
  sub-family among m = 2 candidates is about 1/(small constant) and flat in q).

## Measurement plan and budget

Binary `analysis/..`: `rustc --edition 2021 -O v3.rs --cfg combo` from `research/ic_gw_nrm_lz_20261115/src/`
(already built to `bin/v3`, gitignored, sha256 recorded in README). One process at a time, single thread,
`nice`. IC: `v3 ic q seed D 3.0 1.7` and `v3 ic q seed B0`. rho: `v3 rho q seed 0 4 plain,neg` (4 targets per
instance). Estimated 20 to 25 CPU minutes in total. Raw JSONL under `raw/` (gitignored), SHA-256 in README.
Pricing: M-eq with the instrument's counters at w = 0, 0.1, 1 (unpriced native additions), rho charged by the
same instrument; the rho reference is the repository's `rho2.rs` (plain and negation), calibration rule of v2:
steps/sqrt(ell) in [1.0, 1.5] for plain and [0.75, 1.15] for neg over the pooled runs, else the arm is not used.
Decision rules are the P1..P8 rules above and nothing else.
