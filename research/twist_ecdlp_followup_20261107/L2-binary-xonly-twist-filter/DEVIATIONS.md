# DEVIATIONS from PREREGISTRATION.md (frozen; not edited)

Each entry: what the registered text says, what was done, why, and the effect on interpretation. Interpretations that fill gaps in the
registered text are listed separately at the end. Numbers quoted from development runs are not relied upon anywhere in RESULTS.md; the
evidence for each deviation is a mechanism that can be re-observed with the shipped code, as stated.

## D-01 Rho: distinguished-point and branch rule (registered: "r = 16 branches chosen by the low 4 bits of x, DP = x low bits zero")
* In the polynomial basis of the registered irreducibles, the x-coordinates of the odd-order subgroup G = 2E(F_q) satisfy
  Tr(x) = Tr(a) = 1 (a = 1, k odd). GP (checked: the indices i with Tr(t^i) = 1) gives Tr(x) = x_0 for k = 17, 23, 31, and
  Tr(x) = x_0 + x_17 (k = 19), Tr(x) = x_0 + x_27 (k = 29). So for k = 17, 23, 31 bit 0 of x is pinned to 1 for every point of G and the
  registered rule "x low bits zero" can never hold; for k = 19, 29 bit 0 is tied to a high bit and the low-bit statistics of G are
  not uniform either.
  Observed while developing (terminal output, not archived, no number relied on): at k = 17 the walks never produced a distinguished point
  and the instance never terminated; the low-bit histogram of x over walk points contained only odd values. Reproduce by switching the two
  hx() calls in src/rho.rs to r.x.
* Done: branch index = bits 0..3 and DP test = bits 4..(4+d_dp-1) of h(x) = (x * 0x9E3779B97F4A7C15) >> 32 (one 64-bit integer multiply).
  Counted inside the registered native operations "branch index" and "DP test" (one native operation each). d_dp, theta and the 20/theta cap
  are as registered.

## D-02 Rho: setup, starts and fruitless-cycle handling (registered: r = 16, starts a P + b Q by scalar multiplication, walks capped at 20/theta steps)
* With r = 16 and the negation map, a 2-cycle is entered with probability about 1/(2r) per step, so most walks end at the cap and the
  registered implementation fails the registered sanity band F8 (mean G / (sqrt(pi n)/2) far above 1.6), mostly by cap waste and by the cost
  of full-length scalar-multiplication starts and table construction at small k. Section 5 (F8) says to fix the baseline before ranking.
* Done (all counted, none free): (i) table scalars alpha_j, beta_j are 12-bit (16 table points = 16 x (2 short scalar multiplications + 1
  addition), counted in G); (ii) a start is the sum of 6 random table points (5 additions, counted in G) instead of a P + b Q;
  (iii) after each step the canonical x is compared with the x two and four steps back (2 native operations per step, so 5 native operations per
  step instead of the registered 3); on a match the walk is moved by one doubling (1 G, counted). The registered cap 20/theta is kept.
  The instance Q = dP itself is not counted (it is the instance, not part of the attack).
* Effect: rho is priced at 5 native operations per step instead of 3 (higher rho cost at c > 0, i.e. in rho's disfavour). F8 is still
  evaluated exactly as registered on the resulting mean G; sizes outside [0.8, 1.6] are flagged in RESULTS.md and no ranking against rho is
  reported for them.

## D-03 Full solves: k = 29, m = 3 not run to completion
* Registered: complete solves for m = 3 at k in {17, 19, 23, 29}. Not run at k = 29 (about 1e9 pair attempts per 1000 targets in arm U,
  several hours of CPU per instance on the shared host). It is reported as "extrapolated from measured per-relation cost" like the
  registered k = 29, 31 (m = 2) cells, with the registered formula model for linear algebra. All other registered full-solve cells
  (5 curves x 2 instances) were run.

## D-04 Registered stop rule executed literally; additional exploratory stop rule
* Registered rule: after N_need = |V_E| + 11 relations run elimination; if rank < number of unknowns (|V_E| + 1) collect 10% more targets,
  up to 5 rounds, else FAIL. It was executed exactly as written (incremental dense zero-skipping elimination mod the prime n, so every
  round's rank is available; LA row operations and entry multiply-adds counted over the whole incremental run).
* Because unknown columns that no relation touches make full column rank unreachable, the registered rule is expected to FAIL for many
  cells. As an addition (labelled exploratory everywhere) the d-column is placed last and the first relation count at which a pivot exists in
  the d-column is recorded ("d determined"), with all counters snapshotted there, and d is checked against dP = Q. The IC-vs-rho gap
  is reported for both rules, never mixing them.

## D-05 Prime-field control details left open by the registration
* Targets are random x with a quadratic-residue f(x) (so R in E(F_p) = G since the order is prime), not points of a walk; the prime
  relation check lifts x(R) canonically and tests sign patterns against +-R. Prime arms' group-operation counts are therefore not
  comparable to the binary ones and no prime IC-vs-rho ranking is made.
* H2 census for prime fields uses 50 random factor bases per curve (registered only for binary: 200).
* Prime curves: p = the largest prime below 2^k with p = 3 mod 4 (p = 2^k - 1 for k = 17, 31), A, B drawn from seeded GP streams until the
  order is prime (ellcard).

## D-06 S arm accounting
* Registered: S adds m T-operations per raw solution. Implemented: S needs the per-column E/twist bit, so it pays the same per-column
  precompute as F once per (curve, V) (counted in S's extra counters) plus m T lookups per raw solution. S total = U + S extra.
  S/U is therefore at least 1 by construction; the question is only whether it stays below 1.05.

## D-07 Operation-count model notes (no deviation from the registered formulas)
* Instrumented inv(k) = (k-1) S + (floor(log2(k-1)) + popcount(k-1) - 1) M exactly (Itoh-Tsujii), and the measured failed-attempt cost is
  a = 27 at k = 17, equal to the registered table. Membership in V is counted as k-l parity tests (X) per test as registered, even though
  the implementation uses byte-table syndromes for speed (implementation speed, not counted cost).

## Interpretations of under-specified registered wording (not deviations)
* "Call ratios within 2 sd (H2 band) of 2.0 / 4.0": per curve z_c = (U/F stage-1 call ratio - ref) / sd_pred with sd_pred obtained by
  the delta method from the H2 sd of f (4 sd_f for m = 2, 16 sd_f for m = 3); a cell passes if |mean z_c| <= 2.576 sd(z_c)/sqrt(#curves).
* "Cost ratio R_hat (native ops, precompute amortised)" = (sum over curves of solver native operations of U) / (same for F including its
  precompute). The "CI contains 1.0 at c >= 0.1" test is applied to the cost ratio (G + c N), where G is the common group-operation
  count (target walk, setup, sign checks), 10,000 cluster-bootstrap resamples over curves with seed 20261107.
* "Waste" in PC-B = fraction of unique solutions that contain at least one non-E-liftable coordinate.
* Flagged targets (x(R) in V or sqrt(b)/x(R) in V) are solved normally; their degenerate-family solutions are counted in the D classes,
  everything else with a twist coordinate would count as a violation regardless of flag.
* PC-D skips (and replaces by the next walk target) the rare targets with sqrt(b)/x(R) in V, which would add the second D family.
* Rho: Q = dP is built per instance without counting; d_dp = round(log2(n)/4).

## D-08 Changes made after the main run started (disclosed; none alters a counter or solution)
* A column-alignment bug in the full-solve TSV writer (rows with a failed registered stop rule lacked the `reg_*` columns, so rows of one file
  could have different column sets) was found while reading the first analysis. Fixed (all columns always emitted); the 6 full-solve files were
  regenerated with the same commands and seeds; the first versions were discarded and never reported. The `quick` same-seed replay confirms
  the corrected files are reproducible.
* Added after the main run: environment variable `L2SEED_XOR` (default 0 = registered master seed; used only for the independent replay in
  `VERIFICATION.json`), a "known-sd variant" column for the call-ratio check, the H6 summary line, removal of a placeholder column, relabelling of the
  wall-time column. `run.sh analyze` regenerates analysis, plots and RESULTS.md from the stored TSVs.
* Call-ratio check: the declared rule (normal critical value, sd estimated from the curves) is used for the F6 decision. The stricter variant
  with the known sd of 1 fails for one prime cell (m = 2, k = 31) in the main run; both are shown.
