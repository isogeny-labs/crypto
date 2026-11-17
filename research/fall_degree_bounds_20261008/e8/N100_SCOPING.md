# N ≈ 100: what a sparse closure engine would have to do (scoping only, nothing measured)

**Size.** At `N = 100` the degree-4 Macaulay space has 4,087,976 columns. The dense
echelon basis the current engine keeps is about `columns² / 16` bytes ≈ 1.0 TB. At
`N = 54` the measured peak was 12.6 GB against the 7.3 GB that formula gives for 342,541
columns, so the real figure is about 1.7× the formula. The current engine does not reach
`N = 100` on any single host.

**What the closure needs.** `1 ∈ V_{F,4}` is a statement about the span of the degree-4
rows, plus the feedback of every basis row of degree `< 4` multiplied by variables.
- The feedback only involves rows of degree ≤ 3. That space has 166,751 columns at
  `N = 100`, and a dense basis for it is about 1.7 GB.
- The degree-4 columns (3.92M) are the problem. They would have to be eliminated away,
  that is, a Schur complement onto degree ≤ 3, without ever storing a dense degree-4 basis.

**Candidate method, not built.**
1. Keep the Macaulay rows `m·f` sparse (each has at most the equation's term count times
   `|m|` terms, a few hundred).
2. Eliminate the degree-4 columns with Markowitz-ordered structured Gaussian elimination
   (as in F4-style sparse linear algebra), spilling to disk.
3. Whenever a row's degree-4 part cancels, hand its degree-≤3 residue to the dense
   low-degree basis, and queue its variable multiples.
4. Stop at `1`.

**What is unknown, and decides feasibility.** The fill-in during step 2. The E7 draws show
that the whole degree-4 space is spanned (`dim = columns` on every `d_F = 4` draw), so the
elimination is essentially full rank. Fill-in on a full-rank elimination of ~4M columns is
the known failure mode of sparse methods. Block Wiedemann gives rank without storing
the matrix, but does not give the degree-≤3 rows the feedback needs, so it would answer a
different (plain, not mutant) question.

**Therefore** no number at `N ≈ 100` is claimed or extrapolated. The tier that is
measurable with the current engine, given enough RAM, is `N = 58–60`
(`e7/aws_large_tier.sh`). A cheaper proxy that this engine can do at `N = 100` is only the
degree-3 closure (166,751 columns); it says whether `d_F ≤ 3`, not whether `d_F = 4`, so it
is not a substitute.
