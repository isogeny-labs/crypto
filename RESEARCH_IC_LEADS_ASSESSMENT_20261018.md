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
| T01 | parallel, batch-inverted `build_index` in the wide producer | `examples/koblitz_orbit_dlp_fast.rs` | pending |
| T02 | blocked Bloom filter, slot prefetch, single canonicalisation per hit in the wide probe path | same file | pending |
| T03 | callgrind profile of `scan_state128` at `n = 73` on this host | `docs/ic/perf/WIDE_PRODUCER_PROFILE_20261018.md` | pending |
| T04 | char-2 quartic root solve for `S₄`; B2 kernel re-measured in `S₃` units | `src/cryptanalysis/koblitz_sixsum_extract.rs` | pending |
| T05 | MutantXL closure as `SolverEngine::Closure` with verdict-agreement tests | `src/cryptanalysis/koblitz_groebner.rs` | pending |
| T06 | multigraded Hilbert-series predictor of the closure degree, compared with the 148 committed draws | `research/closure_degree_predictor_20261018/` | pending |
| T07 | equivariant F4 as a library module with inverse DFT and row-space agreement test | `src/cryptanalysis/equivariant_f4.rs` | pending |
| T08 | Joux–Vitse trace record/replay in `matrix_f4_f2` | `src/cryptanalysis/koblitz_groebner.rs` | pending |
| T09 | three-limb `GF(2ⁿ)` field for `n ≤ 191` with the ECC2K-130 polynomial pinned | `src/cryptanalysis/wide_gf2m.rs` | pending |
| T10 | leaderboard: precomputed-rho reference on retained-setup rows; unpinned-calibration flag | `scripts/build_ic_leaderboard.py`, `docs/ic/` | pending |
| T11 | `curves.yaml` records for the 24 board curves | `docs/curves/ic/curves.yaml` | pending |
| T12 | hook that blocks a session stop with unpushed worktree commits | `.claude/hooks/`, `.claude/settings.json` | pending |
| T13 | divergence report of the 154 shared solver files against `cryptanalysis/suite` | `docs/ic/STACK_DIVERGENCE_20261018.md` | pending |
| T14 | port the suite's `f4_gf2` rank-indexed kernel with a same-binary comparison | `src/cryptanalysis/f4_gf2.rs` | pending |
| T15 | closure-degree predictor experiment from the committed draws | `ml-cryptanalysis/experiments/closure-degree-predictor/` | pending |
| T16 | autoresearcher ledger corrections (artefact rejections, `s4_expr` propagation, corpus contradictions) | needs `/coordinate`; not delegated here | pending |
| T17 | external Gröbner baselines (msolve, M4RI) wired as optional arms | `performance/`, `groebner_compare/` | pending; neither binary is installed here |
