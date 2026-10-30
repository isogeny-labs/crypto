# Results: the summand-swap invariant does not refute below degree 4 at ℓ = 6; degree 4 at ℓ = 7 is unresolved (chained S₃ descent)

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
  - **The two labels differ in strength.** At (22, 6), H_none is *measured*:
    Q does not refute at 3, and B refutes at 4, so D_Q ≥ D_B.
  - At (21, 7), H_none comes from the pre-registered convention that
    "unmeasured at 4" counts as H_none. **It is not evidence that Q fails at
    degree 4.** B does not refute at 4 there, so Q refuting at 4 would be a
    lower degree. The scorer reports this as "Q unmeasured below D_B on 4 of
    4 (not excluded)".
- **Predictions.** I gave H_lower at (22, 6) about 0.4 and at (21, 7) about
  0.25, with H_none as the central guess at (22, 6). The result is H_none in
  both cells.

## What it does and does not say

- **At (22, 6): same or higher degree, at a larger matrix (measured).**
  - Adding the invariant `p` makes every equation quadratic, but the closure
    does not refute at degree 3.
  - B refutes at 4, so Q's degree is at least B's, and Q's degree-4 matrix
    has 5.1–5.5 times B's columns.
  - In this cell the invariant arm costs more at every degree that could
    matter.
- **At (21, 7): unresolved at degree 4.** Neither Q nor R refutes at 3. Their
  degree 4 is unmeasured, so this study does not exclude a lower degree here.
  - B does not refute at 4, so its closure degree is at least 5, with
    C(42, ≤ 5) = 974,982 columns.
  - Q's degree-4 matrix has 637,393 columns.
  - So if Q refuted at 4, it would refute at a lower degree *and* a smaller
    matrix than B. This is the open question the study leaves.
- **At degree 3, quadratisation alone does not help either.** The matched
  control R is also quadratic but not invariant, and it behaves the same at
  degree 3.
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

- **Where the plain system refutes at 4 (ℓ ≤ 6), the pair invariant does not
  help.** Measured at (22, 6): no lower degree, and a larger matrix.
- **Where the plain system does not refute at 4 (ℓ = 7), it is not
  resolved.** Degree 4 of Q (and R) at (21, 7) is the measurement that
  decides it. It is the cell where degree-of-regularity costs rise, so a
  refutation there would matter. It is not closed by this study.
- **At m = 4** the second cubic link `e₁x₃e₂` has no summand-swap symmetry,
  and it would need an auxiliary like R's.
- **No row moves and none beats rho.** Against the success criterion this is
  a negative, not an improvement over the index-calculus baseline. Class:
  stage diagnostic. There is no `S`, no rho ratio, and no benchmark or
  leaderboard row.

## Successors (not run)

- **Degree 4 of Q and R at (21, 7).** This decides the open cell.
  - The dense matrix has 637,393 columns, which needs a large-memory host
    (the open lead `LEAD-ICE-descent-closure-degree-at-scale` already asks
    for ≥ 250 GB).
  - The sparse closure needs more than 10⁹ entries, which ran past 12 GB
    here.

- **The invariants as a *replacement*, not an addition.** Eliminate the
  summand coordinates of x₁ and x₂ in favour of `s` and `p` with the
  root-in-V relation, so N falls instead of rising. Whether that relation has
  a low-degree Boolean form over V is open.
- **The orbit-aware split rule** (`RESEARCH_EXOTIC_COORDINATES.md` §18,
  untried): guess along the Frobenius orbit of an unknown rather than raw
  coordinates. It connects with the guess-count studies (#2429).
