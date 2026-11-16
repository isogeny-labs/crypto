# ECC2K-130 new leads survey, 2026-10-23

Class: ideation and cross-checking. No measurement, no claim. Every number
below is copied from a cited repository file or is marked derived. Literature
recalled from memory is marked "recalled, unverified". Visuals:
`research/ecc2k130_leads_survey_20261023/` (leads.json, plot.py, SVG, PNG,
report.typ, report.pdf).

## What was asked

Find leads on the descent route beyond what is established: chained S3
systems with m summands, N = (m-2)n + m*l, refutation degree growing with l,
MutantXL closure at degree 3-4 for l <= 6 (m=3) and l <= 5 (m=4) (the m=4,
l=6 cell also refutes at 4, `research/dreg_closure_m4l6_20261018/RESULTS.md`),
a rise past 4 at m=3, l=7, k <= 4 guesses at l=8, free-oracle floor binding
every row. Success criterion (AGENTS.md): a verified measured improvement over
the index-calculus baseline counts; relabelling and accounting do not; rho
rows are priced across the section 5a band (0, 0.1, 1 GAE per unpriced
operation). The user's current plan is the three-guess test at l=8
(`research/dreg_closure_guess3_20261023/PREREGISTRATION.md`) with the large
cells (l=9, m=4 l=6/7, degree 5 past l=7) deferred to a larger machine. Memory
note: any run that is killed or empty is unmeasured, never negative.

## Method

Four angles (rho, floor, oracle, structure), each lead stated object-first
with lossy-projection test, null and positive controls and a cheapest native
falsifier, then one skeptic verdict per lead. Nine leads, none dropped by a
skeptic (eight "rescope", one "keep"). Ranking here is by expected value per
cost and by directness to a measured improvement over the IC baseline. Three
leads (D1-D3) concern the rho reference, not the IC baseline; under the
success criterion they cannot be improvements, so they are not ranked. That
is a prioritisation, not a refutation, and it is flagged where weak.

Gap to rho throughout is the model's unconverted figure (touches against
group operations, K unmeasured per `research/k_conversion_20261018/README.md`):
+7.62 bits (m=4 degree 4, w=1), +10.32 bits (degree 5), free-oracle floor
2^56.46 (m=4) from `RESEARCH_ECC2K130_DESCENT_DEGREE.md` sections 2-3. No lead
below moves a priced row under rho.

## Ranked leads

### 1. Matched null N0 and shape arms for the guess effect (skeptic: keep)

- Object: unguessed linear-subspace system (m=3, n=22, l=7), N=43, 44
  equations, 136,698 columns: the exact matched null for G3 (same N, same
  equations, same columns; coset against subspace). Shape arms GB (three
  variables in one summand) and GE (three intermediate coordinates fixed)
  are secondary; GE is not equivalent to a guess on t.
- Why now: the G3 pre-registration controls (N7 at n=19, l=7) have different
  n and 3 fewer equations, so they cannot separate a guess effect from a
  surplus or l*(n) effect. N0 decides whether "k <= 4 at l=8" is a guess count
  or a boundary in (n, l).
- First experiment: 4 unsatisfiable draws at (22,7) (about 60 percent of
  draws, estimate), `dreg_closure --to 4 --max-cols 140000` under the G3
  prlimit; peak 10.6-13 GB, so serial after the 12 GB run and after G3. A
  cheaper pilot is (n=20, l=7), about 102-113k columns. Controls: N7 reproduces
  committed ranks 2027 / 94,230; G6 (six zeros) refutes on at least 3 of 4;
  a 1-guess arm at l=8 must fail (else the instrument cannot discriminate);
  satisfiable draws never refute.
- Number that refutes the "pure l-reduction" reading: N0 fails on at least 3
  of 4 draws while G3 refutes on at least 3 of 4. "Pure surplus" confirmed if
  N0 refutes on at least 3 of 4.
- Falsifier of the lead's value: both arms agree and the (22,7) cell sits
  above the memory cap (an unmeasured run is not a negative).
- Gap to rho: unchanged (+7.62 / +10.32). If N0 refutes, the degree-4 row is
  best-case with k=0 and the hybrid price 2^k C(N-k,<=4)^w no longer applies.
  Doubt: surplus +1 and n=22 are confounded in one cell; 4 draws are few.

### 2. Free leaf-count rescoring of G3/G4 (skeptic: rescope, accepted)

- Object: guess tree with G3 leaves as parents of G4 leaves. From stored
  records report f (failed G3 branches), leaves L = 8+f, and total closures
  8+2f against 16 flat; k_eff = log2 L is an upper bound for this guess set
  only (skeptic correction: the failing internal nodes were full closures).
- Cost: zero beyond the G3 run. Do it the moment G3 ends.
- Refutation: f = 0 makes k <= 3 flat and the adaptive question moot; f = 8
  is a loss (24 against 16). Step 2 (lookahead selection at l <= 6, ladder
  analogue; null = shuffled scores, positive = G6 as a refuting leaf, charged
  at 0/0.1/1 GAE) runs only if f > 0 on some draw. The specialisation study
  (`research/dreg_closure_specialise_20261017/RESULTS.md`) says no simple
  score predicts which fix works, a weak prior against lookahead.
- Gap to rho: at most about 1.5 bits (k 4 to 2.5) before charging failures;
  +7.62 stays a best case.

### 3. E-capped MutantXL closure, m=3 only (skeptic: rescope)

- Object: closure over monomials with e-degree <= c (c = 0, 1, 2), total
  degree <= D; sound by construction (subspace of the full degree-D part).
  Exact at m=3 where the system is linear in the single e; for m>=4 the
  middle link has e1^2 e2^2 so c=1 discards generators. The m=4 savings
  quoted by the colleague (3.15 / 4.4 / 6.3 bits) are withdrawn.
- First experiment: `--edeg c` filter in `examples/dreg_closure.rs`; c=0 (must
  refute nothing), c=infinity (must reproduce 2027 / 94,230 on n19l7 draw 0),
  on committed unsatisfiable m=3 draws (n16l6, n18l6, n15l5, n13l5), then
  n19l7, n21l7, (22,8). Soundness on satisfiable draws. Deliberate bad cap
  (t-degree <= 1) as structural contrast. Needs a small column-mask engine
  change.
- Refutes: at c=2, degree <= 5, fewer than half the uncapped degree-4
  refutations; alive if same or +1 degree with column ratio >= 2. Prior of
  keeping the degree is about 0.3 (colleague's estimate); the trace result
  (RESEARCH_ECC2K130_ROUTE_TARGETS.md X5''') is a reason to expect failure.
- Gap to rho: a column reduction at unchanged degree would be a measured
  oracle improvement (m=3 column ratio about 2.8 at (22,8), derived); the
  m=4 distance stays +7.62 to +10.32 until a separate m=4 c=2 screen exists.

### 4. Tree against chain encoding at m=4 (skeptic: rescope)

- Object: S3(x1,x2,u), S3(x3,x4,v), S3(u,v,xR) against the chain. Same
  variety, different ideal; at m=3 the tree is the chain (null control).
  The Vieta arm is dropped from round one (it adds an n-bit unknown, N =
  3n+4l, and is a consequence of the same quadratic).
- First experiment: m=4, l=3 and 4, at least 8 paired unsatisfiable draws
  per cell, lowest refuting degree and closure rank. Controls: m=3 tree =
  chain draw for draw; harness reproduces the committed m=4 l=6 degree-4
  refutation in both topologies; positive control unsplit S4 above chain at
  (13,4) (F4 degree 7 against 4, `research/pdp_split_dreg_20261011`).
- Honest limit: the chain does not rise at m=4, l<=6, so small cells can show
  at most a degree 4 to 3 shift. Success counts only as a measured drop on
  paired draws; the 78.2x / +10.32 to +7.62 extrapolation is not claimed.
  The decisive cell (m=4, l=7) needs a large machine.

### 5. Unequal-arity closure-degree probe (skeptic: rescope of the large-prime lead)

- Object: m=3 or 4 with summands in V (l=4..5) and one in V' (l'=8..10).
  Question: does the refutation degree track max l_i, sum l_i, or N? The
  large-prime pairing arm (floors 2^45.6 and 2^53) is not claimed: E4 in
  `RESEARCH_ECC2K130_DECOMPOSITION_TARGETS.md` closed it as relabelling
  under memory (about 2^35-2^42 partials), and paired rows have weight
  2(m-1).
- Refutes: degree rises by 2 or more over the equal-dimension case at equal
  N. Null: l'=l. Only if the degree stays at 4: add a memory-charged
  large-prime arm to `examples/descent_degree_cost.rs`, with the m>=3
  rank/sign check from `RESEARCH_ECC2K130_DECOMPOSITION_RUNS.md` E4.
- Gap to rho: hand arithmetic gives about -1.7 bits at m=4 degree 4, ignoring
  memory; not a result.

### 6. Measured algebra constant on uniform-column matrices (skeptic: rescope)

- Object: s(c), the dense-core fraction at oversampling c, and the fitted
  constant and exponent of filtering plus block Wiedemann against m*2^(2l).
  Prior for s(c) near 1 is about 85 percent (colleague); for uniform columns
  above the core thresholds (Pittel-Spencer-Wormald, Molloy, recalled,
  unverified) singletons and the clique rule find little.
- First experiment: weight m=3,4 matrices at 2^12, 2^14, 2^16 columns,
  c in {1.05, 2, 6}, existing `filter_relations` and
  `block_wiedemann_kernel`. Positive control: Zipf matrix reproduces the
  316/3000 row; null c=1.05. Closed if s(c) >= 0.8 everywhere; counts as an
  improvement only if s(c) < 0.5 at some c <= 10 at both weights.
- Value: replaces a derived constant by a measured one; the algebra term
  binds the m=4 degree-4 and degree-5 rows.

## Dropped from the ranking (deprioritised; skeptics did not drop them)

- D1 Bailey sigma arm as an ecbench rho method. Skeptic: rescope; real and
  unclosed (scoreboard panel koblitz-references-20260923), but the 1.9x / 4x
  headline is stale: canonicalisation is at most 42.5 percent of the
  optimised strong rho's Ir (`research/ecbench_rho_optimized_20261014`), so
  the ceiling is about 1.74x before the arm's own uncharged work. Effect: it
  makes the matched rho cheaper, widening the gap to rho. Stage 1 (re-price
  only, w = 0, 0.1, 1) is cheap and should be done by whoever owns Table A.
  Reason for not ranking: reference correction, not an IC improvement.
- D2 Wide-path (n=65-127) strong rho: attribution first. Skeptic: the port
  targets the probably-minor term (clmul inlining, tens of Ir per product
  against 6,665 Ir per step); do the Callgrind attribution against the
  selected `fold`/64-lane/DP6 config. Engineering; no IC row moves.
- D3 Ir per step of the n=131 client. Skeptic: rescope to portable-path Ir
  per lane-iteration, never labelled K; Callgrind may not emulate AVX-512
  (recalled, unverified). The committed note says a wider step makes K
  smaller, opposite to the lead's projection. Accounting. Weakly supported
  drop: it is cheap (about 1 h) and the only way to give the n=131 step any
  number; run it whenever the machine is free of the 12 GB job.

## Caveats

- No experiment, build or cargo run was done; the 12 GB measurement was left
  alone. All bit figures are derived from committed cost-model rows.
- Cost tiers and effect classes in the graph are planning judgements.
- Skeptic notes are one pass per lead; two cross-lead facts matter: the chain
  already refutes at m=4, l=6 (kills the motivation of lead 4's m=4 gain), and
  n=131 K is unmeasured (all distances unconverted).
- Several leads share the (22,7)-(22,8) cells and the 16 GB limit; run
  serially after G3.

## Successor

1. G3 finishes: do lead 2 (free). 2. Run lead 1 pilot (n=20, l=7), then
(22,7). 3. In parallel (small memory) leads 3 and 5 probes. 4. Lead 4 and
lead 6 as time allows. The large cells (l=9, m=4 l=6/7, degree 5 past l=7)
need the larger machine already requested; leads 3 and 4 are the ones that
could reduce that need.

## Status update after the three-guess result (2026-10-23, 06:10 UTC)

The survey above was written while the three-guess study was running. Its
result is now in (`research/dreg_closure_guess3_20261023/RESULTS.md`), and
three statements above change. The ranking is left as written.

- **Memory.** The three-guess runs peaked at 7.0 GB at 136,698 columns, not
  the 10.6–13 GB assumed in lead 1; the earlier 10.6 GB was one outlier
  (median 7.0 GB over 68 runs). Lead 1's (22, 7) cell fits this container, and
  the unspecialised (22, 8) system (179,447 columns) may too.
- **Lead 1 is better supported.** Three guesses (variables 7, 15, 23) did not
  suffice on 3 of 4 draws. That 3-set leaves every summand at dimension 7 with
  N = 43, the shape of the unspecialised (22, 7) cell, so the matched null is
  the direct test of whether this is "just ℓ = 7". The (22, 7) cell has
  surplus +1, so unsatisfiable draws should be easier to find than at the
  negative-surplus cells.
- **Lead 2 is not free.** The registered stop rule skipped 7 of the 8 G3
  branches on each failing draw (0, 3, 5), so the number of failed branches f
  is unmeasured there. Measuring it means 21 more runs, about 4 minutes each.
  Only draw 6 has f = 0.
- **m = 4, ℓ = 6 is measured.** The note's opening already cites it
  (`research/dreg_closure_m4l6_20261018/RESULTS.md`): refutes at degree 4,
  which refutes the "m·ℓ ≥ 21" explanation and leaves "ℓ = 7".
