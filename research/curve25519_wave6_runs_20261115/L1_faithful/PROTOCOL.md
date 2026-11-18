# L1: the exponent study's cheap cells on a Curve25519-faithful miniature (frozen before data, 2026-11-15)

Question. Do the exponent studies' per-cell costs depend on the family rule? The lab's k = 20 member
(p = 2^20 − 3, A = 926) has p − 1 = 4·3³·7·19·73; the faithful member found in wave 6 has p = 1,048,261 = 2^20 − 315,
A = 383, p − 1 = 4·3·5·17471, #E = 8·130,969, twist 4·262,193 (checks/faithful_family.out.txt of wave 6).

Instrument. `curve25519_family f4sub p=1048261,A=383 sub D 0 mont M sym N SEED CAP` (new explicit-miniature
form; otherwise the exponent studies' command), N = 6 targets, SEED = 20261108 + 1000·M + D (the parent's rule),
CAP = 1,800 s per target, `ulimit -v 6000000`, RAYON_NUM_THREADS=1, one process. Cells, cheapest first:
m = 2: D ∈ {3, 4, 5, 6, 10, 12, 15, 20, 30, 60}; m = 3: the same D ascending, stopping the m = 3 chain at its first
censored cell (the parent's chain rule). Even D are allowed here as in the extension study.

Comparators (recorded on the smooth family, same (m, D), binary dc2c30a4): m = 2: D = 3: 1.65e3, D = 5: 5.34e3
(k = 22); m = 3: D = 3: 1.90e4, D = 5: 2.02e5 (k = 22) field ops; D = 12 at m = 3: 7.75e8 (k = 20).

Predictions. P1: at D = 3 and 5 (prime), mean field_ops on the faithful member lie within a factor 2 of the
comparators at both m (the theory lane's "T depends on (m, D), not on k or p"). P2: SD = D + 1 at D = 3, 5 for
both m. P3: the m = 3 chain censors at or before D = 60 (D = 30 = 2·3·5 may complete; D = 60 = 2·2·3·5 has four
chain variables per summand, more than the censored k = 20 D = 36 cell), so the preregistered m = 3 estimator
(≥ 4 complete cells with D ≥ 9 spanning 4×) does not apply on the faithful member either. P4: at D = 12 (2·2·3) and
m = 3, field_ops lie within a factor 2 of 7.75e8.

Decision. P1 or P4 failing means the family rule is a confounder for the exponent studies' cell costs, and every
cross-family comparison must be redone on a matched family. SD and mean field_ops are the recorded quantities;
wall time is not claimed.

## Pre-data amendment (2026-11-15, before any L1 cell ran)

The member named above (p = 1,048,261, A = 383) is invalid under the lab's family rule, which the wave-6 search had
not applied in full: the rule requires A ≡ 2 (mod 4) and r > 2^(k−3) besides the cofactor conditions, and the
engine's explicit-curve path enforces it. Under the full rule no k = 20 prime of Curve25519's shape
(p ≡ 5 mod 8, p − 1 = 4·3·q·C with q < 2^(k/8), so q = 5, and C prime; v₂(p + 1) = 1) admits a curve with
A < 40,000 among the seven such primes below 2^20 − 17,115 that were searched (`checks/faithful_full.py` output in
`L1_faithful/faithful_search.txt`). The k = 24 prime p = 16,777,141 = 2^24 − 75 (p − 1 = 4·3·5·279,619) admits
A = 7,842 with #E = 8·2,098,009 and twist 4·4,192,553. L1 therefore runs on `p=16777141,A=7842`; the cell list,
seeds, limits, predictions and comparators are unchanged (the comparators are per (m, D) and the theory lane's
claim under test is that T does not depend on k or p). P4's comparator stays the k = 20 value 7.75e8 at D = 12.
