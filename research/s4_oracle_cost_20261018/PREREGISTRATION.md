# Pre-registration: eliminated S4 against chained S3 as the m = 3 decomposition oracle

Registered 2026-10-18, before any registered cell was run. Stage study of the
decomposition oracle only (AGENTS.md success criterion: a stage improvement,
not a pipeline one).

## Question
On the same instances, does the decomposition oracle cost less per target when
m = 3 is solved as the eliminated S4 system than as the chained S3 system, with
every answer verified, and how does the gap move as l grows?

## Prior art (grep of `research/`, `docs/`, `src/`)
- `research/target_free_s4_20261018/` measured only the eliminated S4 (target
  and target-free), solving degree 7 at (16,4); no chained arm.
- `research/target_free_relations_20261018/` measured only the chained m = 3
  target system, degree 4, largest matrix ~25,000 x 24,000 at (16,4); its
  RESULTS.md says the eliminated presentation "was not tested here".
- `research/ic_rr_norm_ladder_20260930/RESULTS.md` (line ~89) reports the
  Riemann-Roch norm form refuting 2-3 degrees below "the direct S4" at about
  the chain's degree: a different instrument and protocol, not matched here.
- `weil_descend_s4` (`src/cryptanalysis/binary_semaev_s4.rs`, symmetrised S4)
  and `research/koblitz_symmetrised_e2e_20260927/`, `d4_symmetrised_s5_20261015/`
  concern the symmetrised variants and are not compared here.
No earlier matched chained-versus-eliminated comparison exists. The two studies
above may not be combined; this one runs both arms on the same instances.

## Arms (b = 1, Koblitz)
- chained: `build_decomposition_system(basis, x_r, b, 3, st)`: 3l+n unknowns, 2n eqs.
- eliminated: `build_eliminated_s4_system(basis, b, Some(x_r), [0;4], st)`: 3l unknowns, n eqs.
- Solver: `pq_f4_f2::groebner_basis_f4`, field equations included, 600 s budget,
  `F4_F2_MAX_MATRIX_WORDS = 2^26` (512 MiB step matrix), `prlimit --as=3000000000`,
  `RAYON_NUM_THREADS=1`. A timed-out, oversize or killed run is unmeasured.

## Cells (frozen)
(n, l) = (12,3), (13,4), (16,5), (18,6), (19,6), (21,7), (24,8). Chained
unknowns 21, 25, 31, 36, 37, 42, 48. The chained arm is known to hit a memory
wall near 40 unknowns: a cell where it is unmeasured is recorded unmeasured.
n varies with l (n is near 3l), so the l-trend is confounded with n; (18,6)
and (19,6) give one same-l pair.

## Draws and instances
Seed 20261018, per-instance rng `seed ^ n<<40 ^ l<<32 ^ draw*0x9E3779B9`. 4 draws
per cell: draws 0 and 2 random x_R, draws 1 and 3 planted satisfiable (random
x1,x2,x3 in V, u a root of S3(x1,x2,.), x_R a root of S3(u,x3,.) via
`S3LinkRoots`). Basis: `random_subspace_basis`. The same basis and x_R go to
both arms; each (cell, draw, arm) is one process (`examples/s4_oracle_cost.rs`).
Order alternates: even draws chained first, odd draws eliminated first.
Skip rule: if an arm's first 2 draws in a cell are unmeasured, its remaining
draws in that cell are not run and it is not attempted at larger cells; the
eliminated arm continues until a cell where its first 2 draws are unmeasured.

## Verification (per record)
Independent truth, computed before F4 and never from F4 output, by exhaustive
enumeration over V^3: E = {(x1,x2,x3): S4(x1,x2,x3,x_R)=0} (resultant quartic),
E_rat = members of E with a common root u in F_{2^n} of the two S3 links
(`S3LinkRoots`). The eliminated system is exactly E; the chained system exactly
E_rat (E minus E_rat is resultant zeros whose common root lies in the quadratic
extension; this is a semantic difference of the oracles and is reported, not
hidden). Solutions are read off the returned Groebner basis (DFS with constant
propagation, node cap 2e7), projected to (x1,x2,x3), each checked with
`binary_semaev_s4` over F2mElement, and the set compared with E (eliminated) or
E_rat (chained). Verdict `verified` needs: extraction complete, zero S4
failures, set equality. Planted summands must lie in E_rat. Anything else
(incomplete extraction, mismatch) makes the record unverified and it cannot
support a verdict.

## Cost metric, isolation tier, A/A
- Primary: `F4Stats.word_xors` (64-bit XORs of the row eliminations), a
  deterministic counted unit. Also recorded: solving degree, largest matrix,
  `matrix_rows_sum`, `word_xors_performed`, peak matrix bytes.
- Section 5a: the F4 unit omits `divisor_tests`, `pair_candidate_visits`,
  `pair_cover_lookups`, and the construction of the input polynomials. Priced
  cost C(w) = word_xors + w * (divisor_tests + pair_candidate_visits +
  pair_cover_lookups + input_terms), w in {0, 0.1, 1}, same for both arms.
  Equation construction beyond input term count has no counter: an
  implementation gap, so costs are lower bounds. Rho references are not
  involved (this is a stage comparison; the distance to rho is unchanged).
- Isolation: the container is shared with other agents and
  `tools/isolated_bench.py` was not used, so wall time is NOT earned
  (tier: uncontended not established; `/proc/loadavg` recorded before and
  after each F4 run). Wall seconds are a practicality note only.
- A/A: at (13,4) draw 0, each arm is run twice more (rep 2) with identical
  inputs; the spread of word_xors (expected exactly 0, single thread) and wall
  time is reported. A gap is only a result if it exceeds the A/A spread.

## Verdict rule
At a cell: **stage improvement** iff both arms have >= 3 measured and verified
draws, and the eliminated arm's median C(w) is below the chained arm's at all
three w, using only the draws measured on both arms (same instances). If the
order flips inside the band the cell is unresolved; if chained is below, the
cell is a loss (recorded). Cells where either arm lacks 3 verified draws get no
verdict (a chained-unmeasured cell with a verified eliminated result is
reported as "eliminated measured, chained beyond the cap", not as a win).
**Trend rule** over cells with a verdict: the least-squares slope of
log2(median C_chained / median C_eliminated) on l. Slope < 0 = advantage
shrinking; > 0 growing; any cell with ratio < 1 = crossover observed. Needs >= 3
cells to be stated; otherwise "not determined".

## Predictions (confidence)
- P1 Eliminated solving degree exceeds chained's (7 vs 4 at l=4) at every cell
  where both finish: 85%.
- P2 Eliminated median word_xors below chained at every cell where both finish
  (at least 10x at (13,4)): 80%.
- P3 Chained unmeasured at (21,7) and (24,8): 75%; already at (16,5): 55%.
- P4 Trend slope < 0 if >= 3 cells have a verdict: 65%.
- P5 Eliminated record verified = E in every finished draw: 90%; chained verified
  = E_rat: 90%.
- P6 E strictly larger than E_rat in at least one planted draw: 80%.
- P7 A/A word_xors spread exactly 0: 95%.

## Stop conditions
Skip rule above; the whole queue stops when the eliminated arm is unmeasured on
its first 2 draws of a cell. Process failures (nonzero rc) are recorded
as unmeasured with their rc. No registered cell is rerun except as the A/A.

## Smoke tests disclosed (unregistered cells, seed 7, records not kept)
(9,2), (10,3), (14,4), draws 0 and 1, both arms, 120 s budget. Seen: solving
degree chained 3-4, eliminated 4 / 7 / 7 at l = 2 / 3 / 4; at (14,4) chained
~1e9 word_xors (~30 s) against eliminated ~6e7 (~0.3 s); E larger than E_rat at
(10,3); all verified. The (13,4) cell is therefore close to a seen regime.

## Scope
Stage improvement at most; moves no end-to-end S; the distance to rho is
unchanged. Single host class (shared Linux x86-64 container); no external
solver comparison (AGENTS.md 15 owed before any speed claim).
