# Results: the summand-swap invariant does not lower the closure degree of the chained S₃ descent

- **Scoring.** Natively scored by `dreg_invariant_score runs/` (`score-output.txt`)
  against `PREREGISTRATION.md`: commit `d708ad2ee`, Amendment 1 made after the
  first secondary run capped and before scoring.
- **Records.** Every value comes from a committed record in `runs/`.
- **Run, 2026-10-28 (UTC):**
  - control 05:51–06:15;
  - primary and soundness 09:30–09:31;
  - secondary 15:21–17:11, one sparse run, capped.
- **Processes.** 30 measurement processes exited 0. Two more were stopped by
  hand, and both are disclosed in `runs/queue.log`:
  - one secondary run started under a 10-minute tool cap
    (`runs/killed-by-session-cap/`);
  - the second secondary run, stopped by Amendment 1
    (`runs/stopped-by-amendment-1/`).
  Both left empty outputs.
- **Binaries.** One `dreg_invariant` build (`runs/binary.sha256`) for every
  scored run. The scorer's hash is in `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated.
  - Peak RSS: 5.1–5.8 GB (B at degree 4), ≤ 0.13 GB (Q and R at degree 3),
    and 12.0 GB (the sparse run).
  - Iso Lab EKS (AGENTS.md §9a) is not reachable from here: there is no `aws`
    CLI and no credentials. Closure outcomes are exact algebra and do not
    depend on the host; timings are indicative only.
- **Report** with graph and PDF: `reports/run-20261028T0551Z/`.

## The measurement

| cell | draw | B (plain, N = 40/42) | Q (invariant p, N = 60–63) | R (control r, N = 62/63) |
|---|--:|---|---|---|
| (22, 6) | 0 | refutes at 4 | > 3 (degree 4 capped) | > 3 |
| (22, 6) | 1, 2, 3 | refutes at 4 | > 3 | > 3 |
| (21, 7) | 0, 2, 3, 6 | > 4 (saturated) | > 3 | > 3 |

- **Degree 3.** Q and R refute on 0 of 8 unsatisfiable draws. Their closures
  saturate degree 3 without reaching `1`:
  - Q: closure rank 14,393–15,934 of 34,751–36,612 columns at (22, 6), and
    10,889–10,897 of 38,655–39,369 at (21, 7);
  - R: 10,481–10,484 of 36,994–37,009, and 10,136 of 39,579–39,621.
- **Degree 4.** Unmeasured for Q and R. The one sparse run hit its 10⁹-entry
  cap after 6,553 s, and Amendment 1 stopped the rest. A dense degree-4 matrix
  needs C(N, ≤ 4) = 523,686–637,393 columns for Q and R, against B's 102,091
  (22, 6) and 124,314 (21, 7).
- **Validity: valid.**
  - V1: the identity check is true on all 22 Q and R systems.
  - V2: B reproduces its committed records, refuting at 4 (not 3) on 4 of 4
    at (22, 6) and saturating 4 without refuting on 4 of 4 at (21, 7).
  - V3: no satisfiable draw is refuted.
- **Registered verdicts: H_none at (22, 6) and H_none at (21, 7).** D_Q < D_B
  holds on 0 of 4 draws in each cell. The mechanism reading (Q against R) is
  not read under H_none.
- **Predictions.** I gave H_lower at (22, 6) about 0.4 and at (21, 7) about
  0.25, with H_none as the central guess at (22, 6). The result is H_none in
  both cells.

## What it does and does not say

- **Same or higher degree, at a larger matrix.** Adding the invariant `p`
  makes every equation quadratic, but the closure does not refute at degree
  3. At (22, 6), where B refutes at 4, Q's degree is therefore at least B's,
  and its degree-4 matrix has 5.1–5.5 times B's columns. So the invariant arm
  costs more at every degree that could matter. At (21, 7) Q's degree 4 is
  unmeasured, and it would need 637,393 columns, 5.1 times B's.
- **Quadratisation alone does not help either.** The matched control R,
  which is also quadratic but not invariant, behaves the same at degree 3.
- **Unregistered descriptive reading.** At (22, 6), Q closes a larger share
  of its degree-3 matrix (rank / columns 0.41–0.44) than B (0.37–0.38) or R
  (0.28). The invariant does produce more degree-3 relations than the
  non-invariant auxiliary, but not enough to refute. Treat this as a
  pointer, not a finding.
- **Scope.** m = 3, two cells, 4 unsatisfiable draws each (plus 3
  satisfiable), and one form of the invariant (p as new unknowns over
  `span{v_i v_j}`, with `s` left implicit in the summand coordinates). The
  64-variable mask allows no larger ℓ = 7 cell.

## What this means for ECC2K-130 (derived)

- **This closes the pair-invariant lever for the chain at m = 3.** It does
  not lower the closure degree, and it raises the matrix size at the degree
  the plain system already needs.
  - At m = 4 the second cubic link `e₁x₃e₂` has no summand-swap symmetry and
    would need the R construction, which also did not help at m = 3.
  - Neither auxiliary is a route to a cheaper degree-of-regularity row.
- **No row moves and none beats rho.** Against the success criterion this is
  a negative, not an improvement over the index-calculus baseline. Class:
  stage diagnostic. There is no `S`, no rho ratio, and no benchmark or
  leaderboard row.

## Successors (not run)

- **The invariants as a *replacement*, not an addition.** Eliminate the
  summand coordinates of x₁ and x₂ in favour of `s` and `p` with the
  root-in-V relation, so N falls instead of rising. Whether that relation has
  a low-degree Boolean form over V is open.
- **The orbit-aware split rule** (`RESEARCH_EXOTIC_COORDINATES.md` §18,
  untried): guess along the Frobenius orbit of an unknown rather than raw
  coordinates. It connects with the guess-count studies (#2429).
