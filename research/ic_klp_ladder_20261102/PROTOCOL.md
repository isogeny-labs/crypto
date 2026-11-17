# k-large-prime relation collection (k = 1, 2, 3, 4) vs rho: preregistered protocol (before data)

Date: 2026-11-02. Branch `claude/klp-ladder-20261102`. Owner request: run triple and quadruple large-prime variants.

## Method (scripts/klp.py, stdlib Python, exact counts)
Prime-order curves E/F_p, p = 3 mod 4, generated deterministically at 18, 20 and 22 bits (seed = bits * 7919, as in the multi-target prototype).
Large-prime set L = points with x < X (membership by x, so +-L share a column); unknowns = logs of the canonical points of L touched, plus the target log h (last column).
Targets R_i = R_{i-1} + S with S = sG + tQ (one addition each), R_0 = aG + bQ, so R_i = a_i G + b_i Q is known.
A relation is R_i = sum of k signed elements of L, giving a_i + b_i h = sum s_j log(L_j) over GF(r):
- k = 1: x(R) < X (no group operation).
- k = 2: for each signed L1: R - L1 (one addition), test x < X.
- k = 3: table T2 of all signed pair sums of L (setup additions counted); for each signed L1: R - L1 (one addition), look x up in T2.
- k = 4: table T2; for each entry E of T2: R - E (one addition), look x up in T2.
Linear algebra: incremental sparse Gaussian elimination over GF(r), h ordered last; stop when column h receives a pivot; recover h; assert h equals the planted value (a run whose answer differs is a failure, reported).
Counted: group additions and doublings (1 GAE each), including setup (scalar multiplications by double-and-add, table builds). Native uncharged: x comparisons, hash probes, row operations. Priced band w in {0, 0.1, 1} GAE per native operation.
Large-prime set sizes (fixed before data), X chosen so that |L| ~ r^theta:
k = 1: theta in {0.5, 0.75, 1.0}; k = 2: {0.25, 0.33, 0.4, 0.5}; k = 3: {0.2, 0.25, 0.33}; k = 4: {0.15, 0.2, 0.25}.
Reference: Pollard rho with the negation map on the same curve (32-jump adding walk on canonical classes, all points stored, fruitless cycles restart the walk), same counting.
Trials: 8 per (size, k, theta) cell and 16 for rho, distinct seeds; cap 3e7 GAE per run (a capped run is reported as censored).

## Measures and claims
- C1 (descriptive): mean GAE per solved DLP for each cell, the best theta per (k, size), and its ratio to the matched rho at w = 0, 0.1, 1.
- C2 (descriptive): fitted exponent of log2(cost) on log2(r) for each k's best cell across the three sizes, against rho; compared with the generic prediction (k = 1: 1/2 only at theta = 1; k = 2: about 1; k = 3, 4: about 2/3).
- Any k-LP cell below the matched rho across the band would be reported as such and sent for independent replay before any claim.
No threshold is tuned after data. Scope: stdlib Python prototype outside ecbench, toy sizes; not a leaderboard row.

## Amendment 1 (2026-11-02, before any data under this protocol): multi-target batch, k = 1
Owner: run multi-target first, then return to k = 2..4. Script `scripts/mt.py` (the shared-table prototype of
`research/ic_largeprime_multitarget_20261102`, unchanged except an added seed offset argument). T targets (T in 1, 2, 4, 8, 16, 32, 64) share one point table and one GF(r) linear system;
walks step by multiples of G only, so trails of different targets merge; arms B = 0 (multi-target rho) and B = 1 (one unknown base point subtracted every step, the residual collector's structure).
Counted: group additions per solved target (walk steps + 2 per walk start + scalar-mult starts); jump/base setup not counted (equal across arms). Every recovered log asserted equal to the planted one.
Sizes 18, 20, 22 bits (curves as in the main protocol), 16 trials per cell, seed offset 500000 (fresh, distinct from the earlier 20-bit exploratory run).
Measures: (M1) per-target cost vs T for each B and size; (M2) B=1/B=0 ratio vs T; (M3) per-target cost at T relative to T = 1 (the multi-target gain), against the sqrt(T) ideal.
Reference for any "parity" statement: B = 0 at the same T (multi-target rho), never single-target rho. No negation map, no distinguished points (table memory reported, not charged).

## Amendment 2 (2026-11-02, before any data under it): triple large primes with the existing Gaudry engine on E/F_{p^3}
Reason: the prime-field k-LP script (scripts/klp.py) could not be written in this session. The repository already has a tested
Gaudry index calculus with an arbitrary-count large-prime eliminator (`src/cryptanalysis/gaudry_cubic.rs`, `GaudryOptions.max_large_primes`,
`small_base`; relations are 3-point decompositions, so at most 3 large primes per relation: quadruple is not expressible there).
**Different setting from the main protocol:** curves over the cubic extension F_{p^3} (where a subfield factor base exists), not prime fields.
Driver `examples/klp_cubic.rs` (calls only existing library functions): `generate_instance3(p, seed)`, `SubspaceBase::build`,
`run_gaudry_opts` with max_large_primes in {0, 1, 2, 3}, and `run_rho3` as the matched rho on the same instance.
Small base |F'| = ceil(|F|^e) with e in {2/3, 3/4} for max_large_primes >= 1 (Gaudry's |F|^(1-1/n) is 2/3 at n = 3); e = 1 (no large primes) for max_large_primes = 0.
p in {61, 103, 151} (p = 1 mod 3), 8 instances (seeds) each, max_residuals 2e6. Every IC and rho answer checked (report `correct`).
Measure: the engine's own S (total_ops / sqrt(n), F_p multiplications converted with its fp_muls_per_add) per arm, ratio to rho's S on the same instance, and
the fitted exponent of log2(total_ops) on log2(n) per arm. Unpriced native work as the engine reports it; no new pricing introduced.

## Amendment 3 (2026-11-02, before any data under it): quadruple large primes (engine extension)
`src/cryptanalysis/gaudry_cubic.rs` gains `GaudryOptions.arity` (default 3, unchanged behaviour) and `subspace_quad_oracle`:
for each signed base point P_k, the existing 3-point oracle on R -/+ P_k, keeping quadruples with k above the other indices
(each once). Meet-in-the-middle only; about 4|F|^2 pair tests per residual, F_p multiplications counted by the engine as before.
The large-prime histogram widens to 0, 1, 2, 3, >=4. A unit test checks recovery with arity 4 and four large primes.
New arms in `examples/klp_cubic.rs`, same instances, seeds, rho reference and measures as Amendment 2:
arity 4 with no large primes (full base), and arity 4 with max_large_primes = 4 at |F|^(2/3) and |F|^(3/4).
p in {61, 103, 151}, 8 instances each, max_residuals 2e6. Every answer checked.

## Amendment 4 (2026-11-02, before any data under it): batch parity with tighter intervals
Owner: "reach parity with rho". Single-target parity is not reachable without relabelling (a known-log factor-base point turns the
collector into rho itself, which section 6 does not count). Parity is tested where Amendment 1 found it: the batch.
`scripts/mt.py` (unchanged except an optional comma list of T), 22 bits (same curve as Amendment 1), T in {64, 128, 256}, 32 trials per cell, seed offset 900000.
Claim P (frozen): parity holds at T if the 95% interval of the B=1 / B=0 per-target cost ratio (delta method from the two means' standard errors) contains 1 and lies inside [0.9, 1.1].
Reference: B=0 at the same T (multi-target rho). The ratio of B=0 at T to single-target B=0 (Amendment 1, T=1) is also reported.

## Amendment 5 (2026-11-02, after Amendment 4's data, before any replication data): replication of the T = 256 cell
Amendment 4 found B=1/B=0 = 0.935 [0.898, 0.971] at T = 256 (rule P fails low: B=1 cheaper). Replication, unchanged code and curve:
T = 256, 32 trials, fresh seed offset 1300000. Reading frozen now: if the replication interval contains 1, the Amendment 4 cell is treated as a
chance deviation (one of three cells); if it again lies below 1, the B=1 advantage is reported as replicated and unexplained (candidate cause:
the two arms use different step sets, J vs J - P0), with no claim of beating rho until a step-set-matched control is run.
