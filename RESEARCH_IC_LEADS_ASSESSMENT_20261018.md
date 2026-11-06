# ECDLP index calculus: assessment of every open and closed lead, with tracked follow-ups

**Date.** 2026-10-18. **Status.** Analysis over the committed record of
`aburan28/crypto`, `aburan28/cryptanalysis`, `aburan28/ml-cryptanalysis` and
`aburan28/crypto-autoresearcher`, plus four read-only code audits (solver
engines, IC pipeline, sibling repos, autoresearcher ledger). Nothing here is a
new measurement; every number cites the file it comes from. **No security claim
about any deployed curve is made or withdrawn.** Conductor was not installed in
this container (`conductor: command not found`); recorded, work proceeds.

Companion notes this one builds on: `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md`,
`RESEARCH_FREE_RELATIONS_CENSUS_20261012.md`, `docs/ic/GROEBNER_F4F5F6_AUDIT_20261010.md`,
`RESEARCH_G2_EQUIVARIANT_F4_SCOPE.md`, `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`,
`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` §2, §6–§9,
`research/20261016-n83-precomputed-rho-control/README.md`,
`research/dreg_closure_{mutantxl,scale,driver,specialise}_2026101{6,7}/RESULTS.md`,
`research/d4_symmetrised_s5_20261015/README.md`, `docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md`.

---

## 0. Bottom line

1. **The Koblitz online class is closed.** The N83 precomputed-rho control ran
   Bernstein–Lange rho at matched advice (time-matched and memory-matched). IC
   lost online in every arm, median IC/rho 3.74–395× with the 95% interval above
   1 throughout. Every pair-table IC is a generic algorithm, so `S·T² ≳ r/2n`
   binds it; the ladder's rungs sit 25–27 bits above that line.
2. **The solving-degree ladder was measured with the wrong instrument for three
   weeks.** Plain XL gave degrees 5–7 rising with `ℓ` and the ECC2K-130 cost rows
   (`ℓ + 4` fit, `2^+112`) were built on them. The MutantXL closure refutes at
   degree 3–4, flat through `m·ℓ ≈ 20`, rising at `ℓ = 7` for `m = 3` where one
   guessed variable brings it back to 4. This is the most consequential recent
   result and the clearest case of a lead ruled out by an instrument artefact.
3. **It still does not reach rho on ECC2K-130 at any degree.** The free-oracle
   floor of the chain model at `m = 4` is `2^56.46` against rho's `2^60.81`,
   leaving `2^5.71` group operations per oracle call; a degree-2 Macaulay matrix
   already costs `2^16` column touches. The `(m−2)·n` intermediate unknowns of
   the chain make `m = 5` no better. On the current formulations the ECC2K-130
   question is a scientific one (what is `D`?) and not an attack route.
4. **Three measured-positive Gröbner engines were lost to process**, not to
   evidence: Faugère–Svartz equivariant F4 (31–128× on rank at `n = 11–23`),
   Joux–Vitse trace reuse, BooleanSolve sparse Wiedemann. All three were
   "WIP, uncommitted worktree" on 2026-10-12 and are absent from `src/` today.
5. **The largest verified improvement available under the owner's criterion is
   engineering of the wide (`n > 64`) producer and a GPU 4-sum scan.** Tables A
   and C are 79–99.7% root table, rank scan and relations, all probe-rate bound;
   the wide path has never been profiled.

## 1. Rule-outs to reopen or relabel

| item | what the record says | reading |
|:--|:--|:--|
| XL-degree ladder (`⌈ℓ/2⌉+4`, `ℓ+4` fits; `2^+98`, `2^+112` rows) | an XL property, not an ideal property; closure is flat at 4 to `m·ℓ ≈ 20` (`dreg_closure_mutantxl`, `_scale`, `_driver`) | corrected 10-16; every other degree measured with plain Macaulay/XL in the program, including the autoresearcher `dreg-linear-law` extrapolation (`2^1194` vs rho), inherits the same doubt |
| G2 equivariant F4 | gauge-fixing argument predicted no gain; falsifier refuted it in G2's favour; projected ≈300× dense at `n = 131` | ruled in, then dropped; T07 |
| G1 trace reuse, G3 Wiedemann closure | WIP worktrees, never landed | T08; T06 carries the implicit-matrix half |
| 6-sum kernel B2 | 7.6–40× fewer solves, net 1–8× *more* work because one `S₄` root solve cost 20–60 `S₃` solves | an implementation constant: a char-2 quartic by resolvent cubic plus half-trace is ≈4–6 `S₃` equivalents; T04 |
| cyclotomic base A2 | falsifier tested yield only; yield is invariant by the `|B|^m/r` identity | the discriminating solver-degree clause was never run; low value at `n = 131` (no Frobenius-stable subspace) |
| Lead B free relations | relations real, homogeneous rank 49 of 599 at `n = 61` | correctly closed |
| H-TTN-001, H-EQJ-001, H-STR-002 (autoresearcher) | rejected on lossy truncation, a dropped 152σ row collapse, and missing evidence (`run_ids: []`) | never measured as stated; T16 |
| H-GFPN-9a29be, H-ICPERF Macaulay2 column | killed by `RLIMIT_AS` and a `.m2` parse bug | infrastructure artefacts recorded as negatives; T16 |
| yield-gated rejections (H-FB-001, H-FBG-001, H-ISO-001) | mean yield is conserved for every base of equal size | the null was guaranteed; decomposition *cost* per geometry was not measured |

## 2. Ranked angles

1. **Wide-producer engineering and a GPU scan** (T01, T02, T03, T09; GPU out of
   reach in this container). Expected 3–10× on the probe loop, 10–20× on the
   serial `build_index` (18,398 s at `n = 73`). Generic: moves `S`, not the
   operation-count distance to rho.
2. **A multigraded sparse closure engine.** After Weil descent, `S₃` on a
   Koblitz curve is squares of pairwise products (bilinear in the bits, since
   squaring is `F₂`-linear) plus one trilinear term; the Macaulay matrix is
   block-structured by `Zᵐ` multidegree and the degree-3 falls live in fixed
   multidegrees. Multihomogeneous regularity bounds (Faugère–Safey El
   Din–Spaenlehauer; Spaenlehauer 2012) predict a closure degree that stays
   constant until a block size is crossed, which is what the `ℓ = 7` rise looks
   like. Implicit rows (generator × multiplier) and block Wiedemann for
   `1 ∈ rowspace` would make degree 5 at `N = 40–50` and `k(ℓ)` at `ℓ = 8–10`
   fit in 16 GB. T06 builds the predictor; the engine is a follow-up.
3. **One symmetry-stacked Boolean engine.** The group acting on the system is
   `(Z/2)ᵐ ⋊ Sₘ × Cₙ` (2-torsion Möbius flips, summand permutations,
   Frobenius). The D4-weighted `S₅` study showed presentation alone was worth
   18× and the weighted invariant ring 850×; nothing combines hyperoctahedral
   invariants with Faugère–Svartz blocks on the Boolean Koblitz systems past
   `m = 3`. T05 (closure as a production engine), T07, T08 are the parts.
4. **Fix the `S₄` root solve and re-run B2** (T04).
5. **Prime fields: stop**, apart from invariant-presentation engineering. The
   falsifier (an `S₅` small-root method at `δ ≈ 1/5` against a reach of `1/80`)
   is sharp.

The one place an exponent story could still live is a chain-free formulation at
`m ≥ 5` on binary curves (floor ≈ `2^48–50`, oracle budget ≈ `2^15` group
operations per target). No known formulation fits; any work there is a search
for the formulation.

## 3. Tooling and code

- WIP dies with containers (item 4 above). T12 adds a hook that blocks a
  session stop on unpushed work.
- Memory and counters: the deciding cells need 70–150 GB on the dense engines
  against 14–16 GB here, and the hosts have no PMU. T06 is the software route;
  the backlog cards are the hardware route.
- Measurement integrity: `calibration_pins.pinned = []` on the `K₀` instances
  priced identical work at 0.22 and 8.7 GAE; 0 of 24 board curves have a
  `curves.yaml` record (T10, T11).
- No external Gröbner baseline in the crate (msolve once in a bench; no M4RI,
  no Magma); `performance/GATE.md` requires baselines that do not exist (T17).
- Two diverging IC stacks: 154 shared solver files between `crypto` and
  `cryptanalysis/suite` have diverged; the suite's rank-indexed `f4_gf2` kernel
  was never ported back (T13, T14).
- Hard caps: Boolean monomials are `u64`/`u128` masks (`N = 394` at `n = 131`
  is unbuildable); every fast `GF(2ⁿ)` path stops at `n = 127`; the producer's
  `r` is `u64` (T09).
- `ml-cryptanalysis` feeds nothing into IC; predicting closure degree or `k(ℓ)`
  from system structure, and learning WDSat variable orders (order alone gave a
  135× conflict spread), are untried (T15).
- Autoresearcher ledger: 83 IC proposals never designed; the confirmed
  `s4_expr` bug (KN-OPEN-5b3a08) not propagated to earlier rejections; corpus
  contradictions (BKK sparse 1.5–4× vs MV = Bézout; the monodromy closure) left
  standing (T16, Coordinator authority required).

Optimisation state: the `n ≤ 62` library path is tuned (AVX-512, VPCLMULQDQ,
Four Russians 7.5×, inherited F4 6.9×). The wide producer and every GPU path are
not. In the Gröbner engines the matrix build is ≈60× the reduction, the F5
criterion removes 4–8% of rows, and `insert` cascades are 27% of instructions.

## 4. Tracked follow-ups

Each task is a narrowly scoped change landed on this PR's branch from an
isolated worktree, with a same-binary control where it is engineering. Status is
updated in this table as commits land.

| id | task | scope | status |
|:--|:--|:--|:--|
| T01 | parallel, batch-inverted `build_index` in the wide producer | `examples/koblitz_orbit_dlp_fast.rs` | landed (`0f831d70b`, engineering): index build at n = 73 serial -> 4 threads + batch inversion, K = 300 14.60 s -> 5.56 s, K = 600 66.4 s -> 29.1 s (x0.44); index identical to serial (`KIC_VERIFY_INDEX`, unit test); the 18,398 s was the rank-stage wall, not the index build; `docs/ic/perf/WIDE_PRODUCER_ENGINEERING_20261018.md` |
| T02 | blocked Bloom filter, slot prefetch, single canonicalisation per hit in the wide probe path | same file | landed (`21dccf843`, engineering): wall ns per probe at n = 73, control -> all switches, K = 40 417 -> 217, K = 300 614 -> 226, K = 600 622 -> 235 (x0.52, x0.37, x0.38); lazy canonical key alone x0.50 to x0.62, prefetch alone x0.71 at K >= 300, blocked Bloom no gain (off by default); hits 0 in timed scans, e2e n = 69 identical to control; unisolated 4-vCPU guest; no end-to-end S, no rho claim |
| T03 | callgrind profile of `scan_state128` at `n = 73` on this host | `docs/ic/perf/WIDE_PRODUCER_PROFILE_20261018.md` | landed (`e06f51528`): n = 73, a = 1, K = 40, 40,000 states, 11.68M probes under callgrind (124.4G Ir; no PMU on this host, so no IPC). The scan is 31.8% of that run at 3,384 Ir per probe; inside it, canonicalisation 41.3% (1,397 Ir per probe), `RootBatch128::solve` 39.1% (field multiplication 26.0%, 5.5 per probe at 159 Ir), `Linear::apply` byte tables 13.7%, Bloom hashing about 1.1%. `build_index128` is 0.94% at K = 40 (scales about 225x to K = 600); `KoblitzCurve::new` -> `factorise` is 67% of the short run and is its own follow-up. Hits were 0, so the hit path is unprofiled |
| T04 | char-2 quartic root solve for `S₄`; B2 kernel re-measured in `S₃` units | `src/cryptanalysis/koblitz_sixsum_extract.rs` | landed (`33484a7e5`, `9df9bcaa2`, engineering): direct char-2 quartic root solve by resolvent cubic plus half-trace in the six-sum B/B2 kernel, re-measured in `S₃` units with report, graph, PDF, `benchmarks/20261018-sixsum-s4-resolvent/` and a `sixsum_resolvent_component` leaderboard section; a stage diagnostic, no end-to-end `S` |
| T05 | MutantXL closure as `SolverEngine::Closure` with verdict-agreement tests | `src/cryptanalysis/koblitz_groebner.rs` | landed (`39c28a992`): `SolverEngine::Closure` tested against `MatrixF4` and brute force (7 tests: refutes the constructed control at the root with stats (1, 0, 1), never refutes a satisfiable system, agrees on random systems, chained draws and every split rule); one assumption loosened to the contract `plain.splits >= closure.splits` |
| T06 | multigraded Hilbert-series predictor of the closure degree, compared with the 148 committed draws | `research/closure_degree_predictor_20261018/` | negative: frozen multigraded rule matches 8 of 148 draws (5.4%, all at ell = 7, same as the univariate control) against 124 of 148 (83.8%) for constant 4; Spearman 0.369 (draws), 0.489 (cell means); ell = 7 rise not reproduced; FSS bound applies to 0 of 37 cells; no PDF (no typst, pandoc has no engine) |
| T07 | equivariant F4 as a library module with inverse DFT and row-space agreement test | `src/cryptanalysis/equivariant_f4.rs` | landed (`3ca8d640a`): `CyclicAction` (Frobenius rotation of the n-blocks), `orbit_decompose` into the trivial block over F₂ plus one block over F_{2^d} per Galois orbit of characters (d = ord_n 2), blockwise RREF, true character inverse DFT, `row_space_agrees` by RREF; agreement on random invariant systems at n = 7, 11, 13 and the Koblitz S₃ barrel at n = 7, 11. Row ops at n = 11, degree 3 (17,281 x 34,606): plain 164.5M word XORs vs 1.72M (1.25M word + 0.48M field) at 1 word per F_{2^d} op, 6.0M at d. Limits: n prime, d <= 40 (so not n = 131), cyclic action only |
| T08 | Joux–Vitse trace record/replay in `matrix_f4_f2` | `src/cryptanalysis/koblitz_groebner.rs` | negative (`19087245d`): `matrix_f4_f2_record` / `_replay` with `F4Trace` (survivors per degree, no coefficients). A single F₂ recording is unsound (accepted strict subspaces); replay now audits 16 random dropped-row combinations (2^-16 miss bound) and merges 3+ recordings. Word XORs on 20 replayed draws at degree 4: generic dense 10-var, 11 of 20 accepted, -24.7% on those but refusals cost 820K, total +13% over always-full; Koblitz m = 3 (n = 5, 7) 2 and 6 of 20 accepted at -0.3% / -3.3%, net loss; specialised draws all refused. Not wired into the relation search |
| T09 | three-limb `GF(2ⁿ)` field for `n ≤ 191` with the ECC2K-130 polynomial pinned | `src/cryptanalysis/wide_gf2m.rs` | landed (`19a7c1fb1`): `WideGf2x3` (3 x u64, 2 <= n <= 191), Karatsuba over six carry-less multiplies with the module's hardware/portable kernels, general sparse reduction plus a hand-folded ECC2K-130 reduction (`z^131 + z^13 + z^2 + z + 1` pinned), Itoh-Tsujii inversion, batch inversion, half-trace; 12 tests (axioms at n = 131/163/191, agreement with the u128 type at n = 97/127 on 10,000 products, known answer against `F2mElement`). Dependent-chain ns per op on this host (pclmulqdq): n = 131 mul 24.0-33.9, sqr 23.1-28.4, against n = 127 u128 mul 13.4-13.9; about 1.8x the u128 multiply, squaring unoptimised (bit-spread) |
| T10 | leaderboard: precomputed-rho reference on retained-setup rows; unpinned-calibration flag | `scripts/build_ic_leaderboard.py`, `docs/ic/` | landed (def0e266f, f5cfe6e1d, ec11047cd): precomputed-rho control block beside the retained-setup rows (n61 and K1798 cells "not run"; K3300/K600 control 3.74–395×), GAE-unpinned flag path with tests, slug-regex fix |
| T11 | `curves.yaml` records for the 24 board curves | `docs/curves/ic/curves.yaml` | landed (7832011a9, d019bfdd1, 1aea01552): 24 of 24 board curves have a record, 0 reconciled; validator accepts the registry's decimal-string integers; peer mirror is `aburan28/cryptanalysis#959`, which must merge first for the `ic-semantic-metadata` job |
| T12 | hook that blocks a session stop with unpushed worktree commits | `.claude/hooks/`, `.claude/settings.json` | blocked: the auto-mode classifier refuses an agent editing `.claude/settings.json`; needs the owner to run it |
| T13 | divergence report of the 154 shared solver files against `cryptanalysis/suite` | `docs/ic/STACK_DIVERGENCE_20261018.md` | landed (1d3d03988): 154 shared names (52 identical, 29 identical up to import edits, 49 crypto-ahead, 21 both-changed, 3 suite-ahead, all non-IC), 106 crypto-only, 23 suite-only; ports recommended in order: `f4_gf2` (T14, ≈2,000 lines, suite-measured ≈10× on the PDP stage), `semaev_decomp/linearised_m4` (≈1,450 lines, 36× instructions on one cell), the GPU echelon path (needs a GPU). Note: the suite develops on `claude/cryptanalysis-repo-setup-p59kek`, not `main` |
| T14 | port the suite's `f4_gf2` rank-indexed kernel with a same-binary comparison | `src/cryptanalysis/f4_gf2.rs` | landed (`5a0db28a4`, engineering, kernel level): Koblitz m = 3, n = 15, ell = 5 (30 variables), degree 4, 20 draws, equal decisions (all undecided), median ms per draw: crate full readback 199.5, crate solver path (the engine's call) 396.7, port F5 on 35.6, port F5 off 40.1 (5.6-6.0x over full, 10.4-11.2x over the solver path, four runs); median word XORs 155.0 M / 64.3 M / 57.5 M / 68.7 M, so the gain is build and readback, not elimination; agreement with `matrix_f4_f2` on 160 Koblitz comparisons (37 refuted, 73 forced, 50 undecided) plus random dense systems at degrees 3 and 4; default `SolverEngine` unchanged; `docs/ic/perf/F4_GF2_PORT_COMPARISON_20261018.md`, leaderboard section `f4_gf2_port_component`; no end-to-end S, no rho claim |
| T15 | closure-degree predictor experiment from the committed draws | `ml-cryptanalysis/experiments/closure-degree-predictor/` | landed as `aburan28/ml-cryptanalysis#30`: only `ℓ` and `m·ℓ` are sufficient (148 draws, 37 cells, p = 1/666); the two rules are separated only by the cell m = 4, ℓ = 6 (2–5 unsatisfiable draws suffice); null result |
| T16 | autoresearcher ledger corrections (artefact rejections, `s4_expr` propagation, corpus contradictions) | needs `/coordinate`; not delegated here | pending |
| T17 | external Gröbner baselines (msolve, M4RI) wired as optional arms | `performance/`, `groebner_compare/` | pending; neither binary is installed here |
| T18 | wire `f4_gf2` as `SolverEngine::RankIndexed` and A/B it on the relation search; controlled A/B of the default engine's 24-variable build switch | `src/cryptanalysis/koblitz_groebner.rs`, `f4_gf2.rs` | pending |
| T19 | lift the equivariant F4 field cap (`d <= 40`) with `WideGf2x3` so the character blocks reach the (4,6), ℓ ≥ 7 and n = 131 cells; add the hyperoctahedral flips | `src/cryptanalysis/equivariant_f4.rs`, `wide_gf2m.rs` | pending |
| T20 | re-run the landed n = 73, K = 600 rank stage with the T01/T02 defaults under the frozen protocol, A/A control, replay certificate; move the row's `S` | `examples/koblitz_orbit_dlp_fast.rs`, `docs/ic/` | pending |
| T21 | wide producer arithmetic: basis-aware root solve (drop the per-probe `to_normal`/`to_poly` maps), n = 73 and 83 modulus-specialised reduction, closed-form Koblitz order in `KoblitzCurve::new` | `examples/koblitz_orbit_dlp_fast.rs`, `src/cryptanalysis/wide_gf2m.rs` | pending |
| T22 | rank-indexed elimination inside the equivariant blocks (T14 kernel per T07 block), counted against both | `src/cryptanalysis/equivariant_f4.rs` | pending |
| T23 | δ-census: for every IC arm on the board, the fraction of group elements whose structure the arm exploits, against the Ω(min(√q, 1/δ)) bound of ePrint 2026/384; a per-row theory floor beside the matched rho | `docs/ic/`, `scripts/build_ic_leaderboard.py` | pending |
| T24 | fall-space closure-degree predictor (the T06 next step): degree-3 fall space per multidegree block, frozen before `compare.py`, on the same 148 draws plus the (4,6) draws | `research/closure_degree_predictor_20261018/` | pending |
| T25 | B2 kernel end to end on the ladder rows only (n = 41, 53): relations per second with the resolvent solver against the 4-sum, same binary; stop at that measurement | `src/cryptanalysis/koblitz_sixsum_extract.rs` | pending |
| T26 | Joux–Vitse replay: cheap refusal by sketch rank before elimination; keep only if the generic-family total drops below always-full, else close | `src/cryptanalysis/f4_trace.rs` | pending |
| T27 | CI and worktree debt: refresh or retire the four frozen-lock workflows, pin research verifiers to their own `catalog_commit`, per-worktree Cargo targets in the agent rules, archive the 965 MB of unreferenced `research/boolean_*` data | `.github/workflows/`, `research/cover_transfer_assessment_20261013/`, `AGENTS.md` | pending |

## 5. Leads from the follow-up wave (2026-10-18, evening)

What changed since §2: fourteen of the seventeen follow-ups landed on this
branch (§4), and the owner's review of the same day
(`research/ic_leads_review_20261018/`, PR #2150) merged on `main` with its own
ranked opportunities (§3 there: the large-memory node for the (4,6) and ℓ ≥ 7
cells and the PKM tower; the baseline and bookkeeping debt; the linear algebra
as an object; the stacked D₄ + 2LP arm; the symmetry arms at m = 4; crossbred,
last-fall-degree and learned orderings; the ML and code loops). The leads
below are the ones the wave's own measurements opened and that review does
not list. Two external results frame them: ePrint 2026/384 (structure must
be exploited on at least a 1/√q fraction of elements to beat rho) and
ePrint 2025/783 (table-lookup hybrids are non-adaptive, hence Ω(√N) below
√N advice), so every wide-producer item here is engineering on the ladder
rows, never a route to ECC2K-130, and is labelled so.

Ranked by what each can move, then by cost.

1. **Make the ported kernel the engine (T18).** `f4_gf2` reproduces the
   crate kernel's rows bit for bit on 160 Koblitz comparisons and is 5.6×
   faster than the full-readback path and about 10× faster than the path the
   default engine actually calls, at the n = 15, m = 3, ℓ = 5, degree-4 cell
   (`docs/ic/perf/F4_GF2_PORT_COMPARISON_20261018.md`). Nothing on the board
   uses it yet. The same comparison found the default engine's own 24-variable
   build switch (`KIC_F4_*`) running 2× slower than its full path at 30
   variables on 3-draw probes: a possible regression in the default that a
   frozen A/B at 22–32 variables settles in an hour. Expected: a constant on
   every PDP-stage row that goes through F4; the first `S`-moving change from
   this wave if the relation search is kernel-bound at those widths.
   Kill: the search is not kernel-bound, or the engine variant changes a
   verdict on the agreement families.

2. **Reach the cells that matter with the equivariant blocks (T19, T22).**
   The (4,6) A1 cell refuted at degree 4 on both unsatisfiable draws today
   (`research/dreg_closure_m4l6_20261018/RESULTS.md`), so "degree 5 holds" is
   alive and the review's §3.5 symmetry arms are conditional no longer on
   that cell. T07's blocks give 95× fewer row operations on the n = 11
   barrel at the 1-word price (27× at d words) but stop at d ≤ 40, which
   excludes n = 131 (d = 130) and every cell with a large ord_n(2). T09's
   three-limb field removes the arithmetic reason for the cap. Lift it, add
   the (Z/2)ᵐ ⋊ Sₘ flips the note's §2 item 3 names, and run the rank-indexed
   elimination inside each block (T22): the 5.6× and the 95× are on
   different axes and should stack. Kill: the F_{2^d} multiply-add at
   d = 130 costs more than the rows it removes (measure at d words, not 1).

3. **Move one real row (T20).** The landed n = 73, K = 600 run spent
   18,398 s in the rank stage; the probe loop now costs 0.38× per probe at
   that K with the defaults T02 turned on, and the index build 0.44×
   (`docs/ic/perf/WIDE_PRODUCER_ENGINEERING_20261018.md`). Re-running that
   row under its frozen protocol with an A/A control and a replay certificate
   is the one measurement in this wave that can move an end-to-end `S` on the
   board rather than a component, and it is cheap (about two hours on this
   host). It stays engineering: the distance to the matched rho is reported,
   not closed.

4. **The next 2× in the probe loop (T21).** With canonicalisation gone, the
   per-probe cost is arithmetic: the root solve (5.5 field multiplications
   at 159 Ir each plus the batch-inversion control, 39% of the old scan) and
   the two byte-table basis maps `to_normal`/`to_poly` (13.7%). Three
   concrete cuts, each a same-binary A/B: a root solver that works in one
   basis so the maps leave the loop; a modulus-specialised reduction for the
   n = 73 and 83 pentanomials as T09 did for ECC2K-130 (the hand-folded path
   is the cheap part of that type); and the closed-form Koblitz group order
   in `KoblitzCurve::new`, whose trial-division `factorise` is 67% of every
   short run and about 9 s of every process start. Kill for the first two:
   under 1.2× on the scan at K = 600.

5. **A theory floor on every row (T23).** 2026/384 bounds any algorithm that
   exploits structure on a δ-fraction of elements by Ω(min(√q, 1/δ)). No row
   on the board states its δ. For each IC arm the fraction is computable
   from the committed relation records (decomposable targets over sampled
   targets, or the factor-base density the arm actually hits); placing
   1/δ beside the matched rho turns the "distance to rho" column into a
   two-sided statement and gives a kill test that needs no run: an arm
   whose 1/δ exceeds √q cannot beat rho in-model. Python over committed
   artefacts; a day.

6. **Fall-space predictor (T24).** The multigraded Hilbert-series rule is a
   recorded negative (8 of 148 draws, the same as a semi-regular control,
   no ℓ = 7 step; `research/closure_degree_predictor_20261018/REPORT.md`).
   Its own next step is a fall-space model (degree-3 fall space per
   multidegree block), frozen before comparison; the (4,6) draws now give
   it an out-of-sample cell. Cheap, and it is the only instrument that could
   say in advance which cells the large-memory node should buy.

7. **B2 once, then stop (T25).** The resolvent solver made one S₄ root solve
   1.7 S₃ instead of 27 (`resolvent_summary.json`), and on the n = 41 ladder
   B2 is now below the 4-sum at K = 10 and 20 on solver cost alone. The
   end-to-end relation rate was not measured. Measure it once on the ladder
   rows (n = 41, 53) with the same binary; the review's §3.8 and 2025/783
   close 6-sum designs at ECC2K-130, so this lead ends at that table.

8. **Replay only if refusal is free (T26).** Joux–Vitse replay is sound
   with the audit and a net loss because refused replays pay the full
   elimination first (`f4_trace` module docs). A sketch-rank refusal
   (random projection of the dropped rows before elimination) would make the
   generic family net positive if it costs under 10% of a full run; the
   Koblitz families gained 0.3–3.3% on accepted draws, so unless refusal
   becomes free the lead closes.

9. **Pay the CI and worktree debt (T27).** Four workflows copy a 2026-09-25
   `Cargo.lock` and build `--locked`, red for every PR since `main` added
   dependencies; one research verifier pins the 2,100-row catalog against a
   live 2,572-row registry; two parallel worktrees sharing one Cargo target
   overwrite each other's test binary (observed in this wave); and 965 MB of
   `research/boolean_*` data that no code embeds sits in every checkout.
   None of this is science, and all of it cost this wave hours.

Not leads: another pair-table or k-sum rung at n = 85–89 (2025/783), a
GPU IC kernel in this container (no device), and any claim from the (4,6)
cell before A2 and C1 run (`RESULTS.md` there: A2 deferred, C1 capped at
169 GB).
