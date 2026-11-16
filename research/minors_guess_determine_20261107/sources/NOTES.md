# Source notes: the Mahalanobis minors / guess-and-determine line

Read in full from the arXiv PDFs saved in this directory (fetched 2026-11-07 with `curl`
through the agent proxy, no fetch was blocked; text extracted with `pdftotext -layout`
to the `.txt` files beside each PDF; abs pages saved as `*.abs.html`).
All page/section locators below are to the arXiv PDF as saved here. Every number quoted
is a number the papers state, `[pcc:unverified]` (cited, not re-measured by us).

| file | paper |
|---|---|
| `1703.07544.pdf` | A. Mahalanobis, V. Mallick, "A Las Vegas algorithm to solve the elliptic curve discrete logarithm problem" (v3, 12 pp.; INDOCRYPT 2018) |
| `2005.05039.pdf` | A. Abdullah, A. Mahalanobis, V. Mallick, "A new method for solving the elliptic curve discrete logarithm problem" (J. Groups, Complexity, Cryptology 12(2), 2021) |
| `2310.04132.pdf` | A. Abdullah, A. Mahalanobis, "Minors solve the elliptic curve discrete logarithm problem" (v1, Oct 2023, 10 pp.) |
| `2607.09814.pdf` | A. Mahalanobis, "A Guess and Determine Attack on the Elliptic Curve Discrete Logarithm Problem" (v2, 6 Aug 2026, 15 pp.) |

## Common core (all four papers)

* Group `E = <P>` of prime order p (the group order; the papers also call the field size q).
  Given `Q = mP`, find `m`.
* Theorem (1703.07544 Thm 2.1, sec. 2): points `P_1..P_k` on `E`, `k = 3n'`, satisfy `sum P_i = O`
  iff a plane curve of degree `n'` not containing `E` passes through them (Bezout +
  Abel-Jacobi; over `F_q` by a Galois-trace argument).
* Points are `P_i = n_i P` and `Q_j = -n'_j Q` for distinct random integers in `[1,p)`
  (1703.07544 sec. 2.1; Algorithm 1 sec. 3). If `sum_{i in S} P_i + sum_{j in T} Q_j = O`
  then `sum n_i = m sum n'_j (mod p)`, so `m` follows (2607.09814 eq. (1), sec. 2).
* Matrix `M`: one row per point, the row is the vector of all degree-`n'` monomials
  `x^a y^b z^c` evaluated at the point (1703.07544 sec. 2.1; 2310.04132 sec. 2 "all possible
  monomials ... ordered"). `n' ~ log2 p` (1703.07544 sec. 3.0.2: `l` solves `l = O(1)+log l+log p`;
  2310.04132 sec. 2: "we take n' = log2 p"; 2607.09814 sec. 2: `n'` approximately `log2 p`).
* Extra-points theorem (1703.07544 Thm 2.3; 2005.05039 Thm 2; 2310.04132 Thm 2.2): with `l`
  extra points (`3n' + l` rows) the left kernel `K` of `M` has dimension `l`; a vector of `K`
  with `l` zeros certifies a degree-`n'` curve through the `3n'` points where it is non-zero.
  The optimum is `l = 3n'` (the binomial `C(3n'+l, l)` is largest there), so `K` is `l x 2l`
  (2005.05039 sec. 2.1, 2310.04132 Remark 2). Rows: `l-1` P-points and `l+1` Q-points
  (1703.07544 sec. 3.1.2, 2310.04132 Remark 2), or `l` and `l` (2607.09814 sec. 2).
* Row-reducing `K` to `[A | anti-identity]`, `A` is `l x l` (the "dense part"). A zero entry of
  `A` solves the ECDLP; more generally a zero `s x s` minor `A[alpha|beta]` gives a vector of
  `K` with exactly `l` zeros, at positions `beta` and the complement of `alpha` in the sparse
  part (2005.05039 Thm 3 and Lemma 1; 2310.04132 Thm 3.1: zero minors of `A` <-> maximal zero
  minors of `K`).
* Success per kernel is a counting heuristic, not a bound: a vector with `l` zeros exists iff
  some `3n'`-subset of the `3n'+l` points sums to `O`; each subset does so with probability
  about `1/p` (1703.07544 Thm 3.1, sec. 3.0.2), and `C(3n'+l, l) ~ p` subsets are covered.

## 1703.07544 (Las Vegas algorithm)

* Algorithm 1 (sec. 3, p. 7): choose `n'`, `l = 3n'`; draw `3n'-1` P-points and `l+1` Q-points
  with distinct random multipliers; compute `K = leftker(M)`; repeat until `K` has a vector with
  `l` zeros ("Problem L"); then `m = (sum of n_i at non-zero P positions) / (sum of n'_j at
  non-zero Q positions)`.
* Stated complexity: the reduction to Problem L has success probability `~0.6321` and is
  polynomial in time and space (Thm 3.2: matrix size `O(log p)`). Cubic Gaussian elimination
  for the kernel (p. 9). Problem L is "the current bottleneck": Algorithm 2 (multiple Gaussian
  elimination, p. 10) tests only `l^2`-many zero positions out of `C(3n'+l, l) ~ p`, so the
  overall success probability is `0.6 (log p)^2 / p` (sec. 5.1, p. 11): a factor
  `p / (log p)^2` more kernels than success events, i.e. "only little better than exhaustive
  search" (sec. 5.2, p. 11, the paper's own words).
* Claimed experiments: implemented in Sage; no timings reported (sec. 5.2).
* Vague: what "polynomial time" means here (the polynomial multiplies a `p/(log p)^2` expected
  repetition count); Problem L has no complexity bound.

## 2005.05039 (initial minors)

* Same reduction; Problem L is recast as "find a zero minor of the `l x l` matrix `A`"
  (sec. 3, Thm 3). The number of minors is exponential, so the paper tests a sub-family:
  - "All 2-minors" (sec. 4, Table 1): scan all `2 x 2` minors of `A`; in characteristic 2
    implemented as "ratios of corresponding elements of two rows; equal entries in the ratio
    vector" (sec. 6.1, p. 11).
  - "Gaussian-elimination Schur complement" (sec. 4.1, pp. 7-9): eliminate `k'` leading
    columns for `k' = 1..floor(l/2)`, scan all `2 x 2` minors of each Schur complement
    `H' = A/E` (eq. 4.1-4.2: a zero minor of `H'` lifts to a zero minor of `A` with the leading
    block adjoined).
* Measured (Tables 1, 2; `F_{2^m}`, binary-field curves, group order about `2^m`; 40 repeats
  for Table 1, 10 for Table 2): all-2-minors average kernels 113.4 / 716.3 / 4406.26 at
  `m = 40 / 43 / 46` (`l` = 360/387/414 rows listed as "No. of rows of A"; note `l = 3 log2 p`
  gives about 120-138, the table's 360-414 is about `3x` that: "the kernel size was amplified
  three times" in 2310.04132 sec. 5.2); Schur complement 2.8 -> 887.5 kernels from 39 to 49
  bits, order `2^50` reached ("groups of order up to `2^50`", abstract). The text itself says
  "the number of kernels ... approximately doubles at every step ... exponential ... this attack
  cannot be extended to much higher values of q" (sec. 4.1, p. 9).
* Work comparison the paper itself gives: Schur complement scans about 120x more 2-minors than
  all-2-minors at `q = 2^46` (area sum `k(7k^2-9k+2)/24`) and that factor matches the speed-up
  (sec. 4.1, p. 8-9): i.e. more minors tested per kernel, not a cheaper way to find a zero.
* Conjecture A (initial minors, sec. 5): a sub-exponential (stronger: polynomial) set of
  "initial minors" detects whether any minor of `A` is zero. Based on the above experiments
  only. No proof; the "minors tested" accounting never enters the reported cost.
* Code: C++/NTL/MPI at `bitbucket.org/abdullah0096/lasvegas-ecdlp` (sec. 6); not fetched here.

## 2310.04132 (minors solve the ECDLP)

* Self-contained restatement (secs. 2-3) plus the "almost principal minor" (APM) family
  (sec. 5, p. 6-8): a `k x k` minor with rows `alpha = {1..k-2} U {a1,a2}`, columns
  `beta = {1..k-2} U {b1,b2}` generalised to a contiguous principal block (found experimentally
  with dimension two) plus `n` "deviations" (extra arbitrary rows and columns), tested for `n = 2`
  then `n = 3` (sec. 5, p. 7; Table 4: from about 31 bits on the zero minor is found almost
  always at 3 deviations).
* Stated complexity: `O(l^5)` for two deviations, `O(l^7)` for three, `O(l^{2n+1})` for `n`
  (sec. 5.1, p. 8), per kernel.
* Measured (Table 3; binary fields `F_{2^m}`, `m = 28..47`, 10 attempts each): average kernels
  = 1 for `m <= 44`, 1.2 / 1.6 / 1.8 at 45 / 46 / 47 bits (one instance needed 6). Table 6 sets
  this against the Schur-complement numbers (1.0 at 30-38 bits, 126 at 47 bits). Claim
  (sec. 5.2): "the almost principal minor algorithm has a polynomial time complexity in size
  of the matrix A". Nowhere is the total work per solve compared with rho or baby-step
  giant-step, and "kernels used" is the only reported cost.
* The number of tested minors per kernel is `~ l * C(l,n)^2`, which at `l = 141` (47 bits),
  `n = 3` is about `2.7e13`, comparable to `p = 1.4e14`: this is consistent with the uniform
  heuristic "each tested minor is zero with probability `~1/p`" (our derivation, not stated in
  the paper), and with `l^7 ~ 1e15` operations per kernel by the paper's own `O(l^{2n+1})`.
* Sec. 6 (p. 9-10): after one zero minor is found many related ones exist (66 principal 2-minors
  of size 2 in `F_{2^25}`: mean 1 / 230 / 70498 zero minors with 2 / 3 / 4 deviations); "no pattern".
* Vague: the definition of "principal minor position" in Table 5; the kernel amplification
  factor ("the kernel size was amplified three times", sec. 5.2); the instance generator
  (curve, point and RNG choices) is not given in the paper.

## 2607.09814 (guess and determine)

* Reduction as above, `l = 3n'` even, `l = 2 l'`, `K` is `l x 2l` in anti-diagonal form (sec. 2).
  The target is a maximal (`l x l`) zero minor of `K` (sec. 1.1).
* Guess (sec. 3, Thm 1 and "finding a mate", Algorithm 1 lines 3-8): choose a set `a` of `l'`
  columns of the dense part, form `K' = K[a]^T` (`l' x l`), row-reduce it to anti-diagonal form.
  A zero `l' x l'` minor of `K'` <-> a zero maximal minor of `K` containing the guessed columns.
  Recursion is used exactly once: "if the number of steps in the recursion is too many, we will
  end up with a matrix with no zero minors" (sec. 3, p. 4).
* Determine (secs. 4-5, Algorithm 1 lines 9-22; Algorithm 2): defect `d` (small positive
  integer). For every subset `b` of `l' - d` dense columns of `K'` (there are `C(l', d)`):
  `M = K'[b]`, `T = leftker(M)` of dimension `d`; every remaining column of `K'` (the other
  `l'+d` columns, dense and sparse) is replaced by its "signature" `s_c` in `F^d`
  (Thm 3; signatures of sparse columns are free, only `d` need an RREF);
  for every `(d-1)`-subset of the `l'+d` signature rows compute the one-dimensional right
  kernel, normalise, hash; a repeated hash value gives `d` dependent signature rows, i.e. a zero
  minor (Thms 4, 5; post-processing Algorithm 2 requires the union of the two index sets to
  have cardinality `d`).
* Stated complexity: "For a fixed `d`, our algorithm is polynomial in `log2 p`. However, when we
  take the probability into account and select the `d` appropriately, the algorithm is no
  better than the exhaustive search, see Table 2" (sec. 1.1, p. 2). Cost drivers: `C(l', d)`
  choices of `b` and `C(l'+d, d-1)` kernels of `(d-1) x d` matrices each (sec. 5, p. 8-9;
  optimisations suggested: Gray codes and rank-one row replacement, parallelism).
  Success probability for one guess `a`: `1 - Lambda^{C(l',d)}` with
  `Lambda = (1 - (p-2d+1)/(p(p-d+1)))^{C(l'+d,d)}` (eq. (4), sec. 6); Table 3 lists the minimal
  `d` for success probability 0.25/0.5/0.75/0.99 at `floor(log2 p)` = 150..530 (e.g. `d = 15` at
  150 bits, `d = 29` at 300 bits) and Table 2 lists, for `floor(log2 p)` = 100..125,
  `log2 C(l'+d, d-1)` and `log2 C(l', d)` (e.g. 100 bits: `d = 17`, 53 and 48, probability
  0.98464), summing to about `log2 p` — i.e. work of order the group order.
* Table 1 (sec. 5, p. 8): compares `log2 C(2l, l-1)` with `log2 p` for success probability
  at least 0.9, e.g. `log2 p = 102` vs `104`; the text calls this "Pollard's rho on penultimate
  intersections ... comparable to the original Pollard's rho", but the table shows the *binomial
  size* tracks `log2 p`, not `(log2 p)/2`. No such rho-type collision algorithm is actually
  specified or run (the text: "we could not find a way to define the collision that reveals a
  central hyperplane intersection", sec. 5.2, p. 11).
* Claimed experiments (sec. 7, p. 13): implementation in Sage/FLINT/Python multiprocessing for
  prime fields, "a proof of concept ... we do not have much data to share". No timings or
  group orders are reported in the paper. (The atlas lead's "order-2^50 toy solve" belongs to
  2005.05039 / 2310.04132, binary fields, not to this paper.) The abstract and introduction
  state "our attack poses no security risk to elliptic curve cryptography at this time, see
  Tables 1 & 2".
* Vague: how `ell` and `n'` are chosen in Table 1 ("reduced by dividing by 5+tt"); whether
  Table 2's `l'` is `3 log2 p / 2` or smaller; the Gray-code speed-up is described, not
  implemented; the "two level parallelisation" is not done.

## What this means for the reconstruction the atlas lead warned about

The earlier draft described the algorithm as "a zero n x n minor of a Riemann-Roch evaluation
matrix with n-1 guessed columns and a cofactor scan for the last". The full texts say:

1. The matrix is the left kernel `K` (`l x 2l`, `l = 3 n'`) of the evaluation matrix of `2l` random
   multiples of `P` and `-Q` on all degree-`n'` plane monomials; its minors encode subset sums.
   Our implementation evaluates on the Riemann-Roch basis of `L(l O)` (`l` functions instead of
   `(n'+1)(n'+2)/2`); the left kernel is identical, so the matrix `A` (hence every minor) is
   identical and only the cost of building it differs, in the method's favour.
2. The 2026 paper is not "n-1 guessed columns + cofactor scan"; it guesses `l/2` columns (one
   recursion), then enumerates `C(l', d)` column subsets and finds the remaining `d` columns by
   hashing penultimate hyperplane intersections.
3. None of the four papers states an asymptotic bound for the total cost; the 2026 paper
   concludes "no better than exhaustive search" for the parameters that give constant success
   probability; the 2020/2023 papers count "kernels", not operations, and report
   exponential growth for the cheaper families.
