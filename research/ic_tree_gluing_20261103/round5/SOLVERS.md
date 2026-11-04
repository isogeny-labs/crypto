# Round 5: the solver side — what a faster solver would have to do, and why the obvious ones cannot

2026-11-04 · follow-up to [`../round4/README.md`](../round4/README.md) and
[`../round4/CROSSOVER.md`](../round4/CROSSOVER.md). Question: rounds 1–4 show
that no encoding change lowers the tree's cost. Is there a solver whose cost is
not the dense Macaulay size: SAT, faster or sparse Macaulay linear algebra,
tensor networks, geometric resolution, hybrids? Tests H and L were
pre-registered in `.resume/RESUME.md` (commits `72dde4fd1e` and the next one)
before their runs. **Class: derived bounds plus two stage diagnostics.** No
pipeline is priced.

## 1. What "faster Macaulay" would have to achieve (derived)

This is the round-4 cost model, with the linear-algebra exponent ω swept from
the information floor (1, linear in matrix size) up. Cost is log₂ cost / log₂ √p
at the best m; below 1 beats rho.

| ω | β = 0 (D flat in t) | β = 0.5 (measured t-slope) |
|--:|--|--|
| 1.0 | 2^128: 1.14 · **2^256: 0.86** · 2^512: 0.64 | 2^256: 1.35 · 2^512: 1.06 · 2^1024: 0.82 |
| 1.2 | 2^256: 0.95 | 2^256: 1.54 |
| 1.5 | 2^256: 1.04 | 2^256: 1.84 |
| 2.0 | 2^256: 1.22 | 2^256: 2.33 |

At 256 bits, beating rho needs **both** near-linear linear algebra (ω ≲ 1.3)
**and** a solving degree flat in the tower depth (β ≈ 0). With the measured
depth slope, even ω = 1, linear in the size of the matrix, does not get there.
Then the size of the matrix itself is the obstacle, not the elimination.

**Sparsity does not supply ω ≈ 1.** The recorded msolve matrices are not
sparse enough. Block Wiedemann costs about cols² · (nonzeros per row), which
gives an effective exponent log(cols²·w)/log(cols) between 2.41 and 2.72:

| matrix | widest | nonzeros per row | exponent |
|--:|--:|--:|--:|
| T at m4 t4 | 12,782 × 25,742 | 713 | 2.65 |
| F at m4 t4 | 15,086 × 13,503 | 50 | 2.41 |
| T at m3 t5 | 26,344 × 23,720 | 1,464 | 2.72 |
| T at m5 t4 | 372,489 × 654,028 | 654 | 2.48 |

(These come from the logs' density column, by awk.)

## 2. Three barriers for solvers that avoid Macaulay matrices

**(a) Enumeration-type solvers: SAT/CDCL branching, meet-in-the-middle,
dynamic programming over the tree, exact tensor-network contraction.**
*Counting bound (proved).* In any decomposition tree, the root's two children
carry partial sums from sets A and B. Covering a random target R requires
R ∈ A + B, hence |A|·|B| ≳ p up to the symmetry factor, and therefore
max(|A|, |B|) ≳ √p.

A solver that decides a separator by its value has to range over at least one
of these sets. Branching on its bits, tabulating it, or contracting a bond
indexed by it all count. So it pays at least √p, the same as rho, and batching
over targets does not escape this. Here is the batching calculation. A table
of k-fold sums costs |V|^k. Each target scans |V|^{m−k}. There are about
|V|·p/|V|^m targets per |F| relations. Keeping both terms below √p would
require |V|^k ≤ √p and |V|^{k−1} ≥ √p, which contradict each other. These
solvers are generic in effect.

*Rank version, for tensor networks.* The root coupling S₃(X₁, X₂, x_R) = 0 is
a 2-to-2 correspondence, a curve isomorphic to E. Its incidence matrix on
A × B has at most 2 ones in each row and column, so its rank is at least
|A|/2. No change of basis compresses the bond, which settles exact
contraction. Approximate low-rank contraction would need a different argument.

**(b) Geometric resolution, i.e. the Kronecker solver (cost ≈ δ², with δ the
largest intermediate degree).** If the membership equations come before the
coupling, the variety cut out by all m membership constraints already has
|V|^m ≈ p points. In the other order, the coupling variety with k leaves
restricted has degree ≈ |V|^k · c, so δ ≳ p^{1−1/m} at the last step. Either
way the cost is above p. *Heuristic count, with no ordering found that avoids
it.*

**(c) Character and rationality branching (test H, measured).** The idea is to
branch on a property of a separator cheaper than its value: its quadratic
character, imposed as X₁ = n·z² with n ∈ {1, a non-square}, one bit per branch.
- At m3 t4 both branches reproduce T exactly: D = 6, widest 7,077, the same
  sum of rows×cols, in 3.6–3.7 s.
- Each branch has quotient dimension 6, against T's 3, because z ↦ −z.
- Gröbner bases count solutions over F̄_p, where every X is c·z². So
  "X ∈ F_p is a square" carries no algebraic information. Making F_p-rationality
  algebraic takes degree p (z^p = z).
- The same holds for every multiplicative character, power-residue class, or
  "bit" of a separator: algebraic solvers cannot use them.
- The m3 t5 branches tracked T's matrices exactly until they were stopped at
  42 s. m4 t4 was not run.

## 3. The one structural lever left, and test L

Fixing the separators gives D = 4 in every cell (rounds 2–3). So the excess
degree is the cost of the couplings. A natural solver-structure question is
whether *fewer* couplings give a smaller excess.

Test L, the fat leaf, takes m = 4 as one triple plus one leaf: the triple is
encoded symmetrically (S₄ in e₁, e₂, e₃ with output X₁), giving 1 separator
instead of the pair tree's 2.
- **Prediction falsified:** D_L ≥ 8 at t = 4 and 5, against D_T = 7.
- **Cause:** the triple's node equation has total degree 8, against 4 for the
  pair tree's S₃-type nodes, so the input degree alone gives D ≥ 8.
- **Lesson:** the excess comes from the number of couplings times the degree
  of each node relation. S₃'s biquadratic, degree 4, is the floor for an x-only
  addition relation, and the pair tree already uses it everywhere.
- The final L numbers are in `README.md`.

## 4. Where that leaves the solver side

| solver family | cost driver | verdict at these laws |
|---|---|---|
| Gröbner/F4, dense or sparse | Macaulay size C(N + D, D), with N ≈ log₂ p | asymptotically subexponential if D = O(m), D flat in t and ω ≲ 1.3. Measured: the sparse exponent is 2.4–2.7 and the t-slope is about 0.5 |
| SAT, MITM, DP, exact tensor networks | separator domain size | ≥ √p (proved counting bound) |
| geometric resolution (Kronecker) | intermediate degree δ | ≳ p^{1−1/m} (heuristic) |
| hybrid on separator characters | none: invisible over F̄_p | measured no-op |

**Reading.** At 256 bits every solver family is held at or above rho by a
different mechanism. Compression is possible only in Gröbner, where it shows
up as D being far below the separators' domain sizes. Its price is the
Macaulay size, which grows because each added summand adds about 1.5 to D and
each tower level about 0.5.

So a real improvement has to change those two slopes. The candidates are:
- a node relation of degree < 4, which x-only coordinates cannot give;
- a coupling that is a function rather than a 2-to-2 correspondence (y-coordinate
  or slope encodings), which doubles the solution count and has not been
  measured here;
- leaving the summation-polynomial framework altogether.

No idea tried in rounds 1–5 changes either slope.
