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
