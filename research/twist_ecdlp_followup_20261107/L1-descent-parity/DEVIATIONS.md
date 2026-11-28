# DEVIATIONS from PREREGISTRATION.md (not edited)

Each entry says what differs from, or fills a gap in, the frozen text, and why. Entries D1 to D4 and D6 to D12
were fixed before the official run; D4 and D5 were provoked by development runs at s = 20 with 2 attempt streams
per instance (same seeds as the official run, outputs discarded, not used); this is disclosed here because the
choices were made after seeing those development outputs.

D1. Degenerate draws. A target with x(R) in F_p lies outside the hypothesis of L1(ii) (R in F) and makes the
Weil-restricted system degenerate (a positive-dimensional solution set). It is detected by one comparison (1 NAT),
not given to the solver, not counted as an attempt, and not entered in the D(R) census; its comb cost
(2W - 1 GAE) is charged in the run total, and the number of such draws is reported (`ndeg`, census column
`ndegenerate`). Frequency about 1/p. Without this H1 ("any target") and the L1 hypothesis contradict each other.

D2. Projection by the eigenspace class. L2's multiplication by c is implemented by classifying each of the two
abscissas of a valid relation by the Legendre symbol of f(x) in F_p (class of l*), dropping the column of the other
class (whose logarithm modulo l* is 0 because c kills that eigenspace) and keeping weight-one rows. Charged
1.5 ceil(log2 p) + 2 NAT per symbol, 2 symbols per valid relation. Factor-base membership lookups are charged 2 NAT
per found pair; y recovery is charged 2 x the frozen square-root cost per pair that passes the lookup.

D3. Arm C (C-fix). The output of the one solver call per (x1, sign) is produced by an exact table oracle (all
sums +-P2 +-P3 of factor-base points, keyed by abscissa) instead of the Weil restriction. UT3 checks it against
exhaustive evaluation of S3 on all pairs of V (controls_<s>.csv). The solver NAT is charged with frozen-style
formulas per call: expansion 27 (three F_{p^3} products), two Sylvester resultants 2 x 104, one GCD(8,8) = 64, i.e.
299 NAT, plus root finding ROOT(d) = 3 ceil(log2 p) d^2 with d = number of distinct abscissas found and 2 square
roots per found solution. Extension-field roots of the gcd are not charged, so C's NAT is a lower bound and its gap
to rho a lower bound. 2M GAE per attempt (every R -+ P1) are charged as frozen. C-S4 was not run (section 4.C
probe, infeasible); no result is labelled S4.

D4. Rho details the freeze leaves open. The stored-point table is kept across restarts (a walk ends and restarts
only after a trivial collision, i.e. a closed cycle longer than 2 or O). The development run that cleared the table at
each restart was miscalibrated on D (mean steps / 0.886 sqrt(l*) = 3.75 from cycle restarts); the table is now kept.
Priced rho cost is steps + doublings GAE and 5 NAT per step as frozen; the 16 step-table points, start points and, for
the composite-order full-N information runs, candidate tests (up to 64) are reported separately (`GAEsetup`) and are
not priced, which favours rho. 5 matched rho runs per instance (seeds from stream a=5,u=4), 1 random-target run
(NC3: Q* replaced by a random multiple of G*), and for F_p curves 5 whole-group runs on N (information only).

D5. NC2. The literal criterion (accepted recoveries above 1/l* + 3 sigma over about 1000 runs) is reported as
`NC2_permuted_targets` and fails in the development run (14 accepted against a bound of 5.57): with few relations the
shuffle leaves the colliding pair unchanged, or swaps it, and both return the true q. A second line,
`NC2b_...`, applies the same bound to the runs where the shuffle gave at least one colliding row a foreign (a,b). Both
lines are reported; the verifier itself (q_hat G* = Q*) is exact.

D6. Replicates. 10 attempt streams per instance for A, B, B', D at every size, D-half/D-quarter at s <= 28; for cost:
D-half/D-quarter at s = 32: 3 streams, C: 10 streams at s = 20, 24, 5 at s = 28, 2 at s = 32 (C s = 32 runs about a
minute each). Rho: 5 matched seeds per instance. Census: 20000 targets per cell pooled over the instances of the cell
(ceil(20000/n) per instance). Unspecified seeds: B' attempt streams a=2,u=12; D-half/quarter factor-base halves
a=6,u=3 and u=13, streams a=6,u=2 and u=12; census a=6,u=21 (A/B) and u=22 (D); unit tests a=6,u=24,25,26; stopping
check a=6,u=23; NC2 a=7,u=2. Per-run seeds are drawn from the stream seed and written in the CSVs.

D7. Comb precomputation is charged per run as 2 W 262 GAE (W = ceil(bits(N)/8); per window 254 additions
and 8 doublings, two tables) as the frozen "precomputed once per instance and counted"; the no-precomputation
variant is a labelled sensitivity row. Factor-base construction is not charged (reported in RESULTS.md).

D8. One factor-base subset (B, B', D-half, D-quarter) per curve, shared by its instances, drawn in order of first
appearance from the seeded stream (partial Fisher-Yates).

D9. Curve selection. A,B: p = first prime >= 2^(s/2); b drawn from the seed, a increasing from a drawn start; the
cofactors h1, h2 <= 15 apply to N1 = h1 l1, N2 = h2 l2; "h" in the CSV is N / l*. C, D: a, b random with
a, b, j(E) outside F_p, cyclic group (ellgroup), N = h l* with h <= 8, not supersingular (ellissupersingular), N != q,
distinct j-invariants; C primes: first prime >= 101, 256, 645, 1625.

D10. H2 uses the pooled B rate against 1/4 +- 0.01 (per-curve exact (|S_E|/M_E)(|S_W|/M_W) is not stored); census
"valid" counts relations (valid unordered pairs per target, i.e. the mean relations per attempt). PC1 is evaluated
for four statistics (rate half/D, rate quarter/D, attempts half/D, attempts quarter/D), each must pass at
>= 2 sizes. P3 model value uses 1/4 for the B rate.

D11. Gates. The pilot of 2 instances x 1 stream is timed by wall time in a single process (equal to CPU within the
accuracy needed) against 1,200,000 ms; the result is in `out/gate_<arm>_<s>.txt`.

D12. Hardware: shared 4-core host, wall times are recorded but never ranked or used.

D13. UT3 (C oracle versus exhaustive S3 on all pairs of V) runs at s = 20, 24 (p = 101, 257; 6 attempts each) and
s = 28 (p = 647; 2 attempts, about 20 minutes) and is not run at s = 32 (p = 1627, hours). The oracle is the same code
at every size.

D14. Analysis-time clarifications (no data re-run): PC1 and the "inside the +-25% window" test use the bootstrap 95%
interval of each of four statistics at each size; NC3 is evaluated per cell as the ratio of total steps of the random
target runs to that of the matched runs, with an instance bootstrap; two of twelve cells exclude 1 (reported in
RESULTS.md section 7, not explained away). The A / matched rho and similar ratios use the rho priced without the
step-table setup (D4). `plot_summary.py` only plots values already in results.json.
