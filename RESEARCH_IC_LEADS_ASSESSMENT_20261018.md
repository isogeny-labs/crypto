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
| T14 | port the suite's `f4_gf2` rank-indexed kernel with a same-binary comparison | `src/cryptanalysis/f4_gf2.rs` | pending |
| T15 | closure-degree predictor experiment from the committed draws | `ml-cryptanalysis/experiments/closure-degree-predictor/` | landed as `aburan28/ml-cryptanalysis#30`: only `ℓ` and `m·ℓ` are sufficient (148 draws, 37 cells, p = 1/666); the two rules are separated only by the cell m = 4, ℓ = 6 (2–5 unsatisfiable draws suffice); null result |
| T16 | autoresearcher ledger corrections (artefact rejections, `s4_expr` propagation, corpus contradictions) | needs `/coordinate`; not delegated here | pending |
| T17 | external Gröbner baselines (msolve, M4RI) wired as optional arms | `performance/`, `groebner_compare/` | pending; neither binary is installed here |
