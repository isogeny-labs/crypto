# Plan: SAT-solver speed for the point decomposition problem (PDP), with a re-measured baseline

**Date:** 2026-11-15. **Status:** plan only. No data has been taken under it, no
protocol is frozen yet, and nothing here is a result. Every number quoted from an
earlier study is an unverified assertion `[pcc:unverified]` until it is
re-measured under §2.

## 0. Where the repository already stands (read before the plan)

The question "what is the most exotic way to speed SAT up on PDP" has a long
history in this tree, and the plan is built on it rather than beside it.

| fact | where it is recorded | standing |
|:--|:--|:--|
| Every SAT, SMT and WDSat back end measured here spends about one conflict per candidate factor-base tuple; native CDCL+XOR at 0.97–1.02 conflicts per triple for ℓ = 5…8, n = 24 | `research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` §1–3, `RESEARCH_SAT_SEMAEV.md` | `[pcc:unverified]` |
| The exact oracle (pairs-and-solve, `semaev_decomp.rs`) is the floor at `T_enum = C(2^ℓ, 2)` candidates; everything else is "enumeration in disguise" | same survey, §2 ladder row 1 | `[pcc:unverified]` |
| The survey's own falsification target: a per-target count `< 0.5 · T_enum` on uniform targets, m = 3, at two consecutive rungs ℓ ≥ 8, each verdict checked against the exact oracle | survey, §1 "Falsification target" | rule, not a number |
| The sub-rho SAT reading (S/rho 0.854 at n = 17) does not survive pricing conflicts: 7.34× rho at 0.1 GAE/conflict, 51× at 1 GAE | `research/sat_pdp_pricing_band_20261111/README.md` | `[pcc:unverified]` |
| Items still open, with frozen protocols: XOR-learning CDCL (Xorricane/CDXCL), WDSat `-x` audit + SATIC, Bosphorus preprocessing, GPU pairs-and-solve leaf. All four were **blocked** on 2026-10-11 because upstream sources are unreachable under the network policy | `research/pdp_followups_20261010/blocked/PROTOCOLS.md` | pending |
| The one SAT thread that met a "lead alive" rule: clause reuse across targets (REUSE/FRESH conflict ratio 0.003–0.005 at n = 4…7 on the x-only ladder circuit). Its prescribed replication (second solver, 40 targets) has not been run | `research/sat_ladder_scaling_20261110/README.md` | `[pcc:unverified]`, replication pending |
| Closed, do not re-run: a third CDCL solver on the same descended system; Riemann–Roch encodings; symmetry breaking inside the pipeline oracle; one-hot Frobenius variables; Crossbred; ML-guided SAT; cube-and-conquer on the descended system (its cubes are the m − 1 summand prefixes, i.e. pairs-and-solve); in-memory/GPU/analog SAT hardware | survey §6 "Not worth a round" and §7 | closed |

Of the six "exotic" directions named in chat (XOR-native + symmetry; automorphism
quotienting; algebraic learning; cube-and-conquer; analog/annealing; learned
branching), four are measured-and-closed or dismissed with a reason above. Two
survive in some form: **learning stronger than clauses** (open item 1, blocked)
and **amortisation across targets** (the pending replication). The plan spends
its budget there and adds one new angle (§3.2) and one closure track (§3.5).

### What changed since the October block

Checked in this session (2026-11-15, this container):

- `github.com` returns 403 for Xorricane, Bosphorus, CryptoMiniSat, WDSat and
  Kissat (network policy: package registries only).
- `index.crates.io` is reachable and serves `cryptominisat 5.8.0-rust1`
  (bundles the C++ source, builds with `cmake`) and `cadical-sys` (already used
  by `research/sat_ladder_scaling_20261110/tool`). `kissat-sys` and
  `cryptominisat-sys` do not exist on the index (404).
- So CryptoMiniSat (native XOR clauses, Gauss–Jordan) and CaDiCaL (plain CDCL)
  can both be linked from Rust without a GitHub fetch. Xorricane, SATIC and
  Bosphorus still cannot; an XOR-learning solver has to be written in
  `src/cryptanalysis/sat.rs` (§3.1) or the sources vendored by the owner.
- **Build check (this session, scratch crate, `cargo build --release`) `[pcc:unverified]`:**
  `cadical-sys 0.8.0` builds unchanged. `cryptominisat 5.8.0-rust1` needs two
  local fixes: its tarball omits `scripts/xxd-alike.py` and
  `cmsat_tablestructure.sql`, which the CMake build only touches when it finds
  sqlite3, so add `.define("NOSQLITE", "ON")` to the crate's `build.rs` through
  a `[patch.crates-io]` path override; and the 2020 C++ source lacks
  `<cstdint>` includes for current GCC, so build with
  `CXXFLAGS="-include cstdint"`. With both, a 3-variable XOR instance
  (`a ⊕ b ⊕ c = 1`, `¬a`, `¬b`) returns SAT with `c = true` through
  `add_xor_literal_clause`. Build time ≈ 85 s. The patched crate copy goes in
  `third_party/` when B0 is implemented, beside the existing `m4gb-patches`.

Sources checked (KB): **KB unavailable** in this container (`research-kb` MCP not
loaded; `scripts/research-kb` has no executable; lead atlas in
crypto-autoresearcher is vault-encrypted with `KEY` unset — same state as the
2026-11-13/14/15 session logs). Used `rg` over both repositories and read the
files named above directly. Lead ids known from those files:
`LEAD-ICE-sat-xor-pdp-oracle`, `LEAD-ICB-decomposition-encoding-and-sat`,
`LEAD-ICP-icperf-binary-pdp-solver`. The atlas cards must be re-read once `KEY`
is available, before the first PR that lands data.

## 1. The quantity that decides the question

The decisive metric is dimensionless and counted, not a clock:

```
R(arm, ℓ) = median over uniform targets of (solver work per target) / T_enum,   T_enum = C(2^ℓ, 2)
```

with "solver work" in the solver's own counter (conflicts for CDCL; nodes for
WDSat; candidates for the exact oracle, where R = 1 by construction). A SAT
route can matter for IC only if R falls below 1 and keeps falling with ℓ; the
survey's falsifier is `R < 0.5` at two consecutive ℓ ≥ 8. Wall time per target
and per relation is recorded beside R, on an isolated host, and the §5a band
(each arm's unpriced operations at 0, 0.1 and 1 GAE) is applied before any
comparison with matched rho is written.

Three things are not improvements under AGENTS.md "Success criterion" and are
excluded by design: a lower wall time on the same R (relabelling), a planted-target
result (positive controls only), and any arm whose verdicts are not checked
against the exact oracle.

## 2. Phase 0 — re-establish the baseline (B0)

Purpose: a frozen, replayable, this-session measurement of R for the existing
arms, so every later candidate is read against a number taken with the same
instrument on the same host class, not against October prose.

**Instance family** (identical to the frozen item-1 inputs, so results are
comparable with the blocked protocol): curve `y² + xy = x³ + x² + 1` over
`F_{2^n}`, `n = 3ℓ − 1`, `ℓ ∈ {8, 9, 10}` (11 if the floor arm finishes it in
budget), `m = 3`, symmetrised `S′₄`, subspace factor base. Per rung: 16 uniform
targets (seed derived from 20261115 and the rung) and 4 planted targets
(positive control, known decomposition). Corpus from `semaev_corpus.rs`; ANF
from `semaev_sat.rs`.

**Arms** (same corpus, same targets, same thread count = 1, interleaved order,
arm labels masked at analysis):

| arm | what | role |
|:--|:--|:--|
| A | pairs-and-solve exact oracle (`semaev_decomp.rs`) | floor; supplies the verdict for every target; R ≡ 1 |
| B | native CDCL + Gauss–Jordan XOR (`sat.rs`, `sat_xor_encoding=native`) | incumbent SAT arm; prediction R = 1.0 ± 0.1 |
| B′ | arm B, second seed and variable order | A/A noise floor; prediction spread < 5 % |
| C | CryptoMiniSat 5.8 via the `cryptominisat` crate, XOR clauses on | external XOR-aware reference; prediction R ≥ 1 |
| D | CaDiCaL 3.0.1 via `cadical-sys`, CNF with Tseitin XOR chains | plain-CDCL control; prediction R ≫ 1 |
| ρ | matched Pollard rho on the same curve at n = 23 from ecbench (`count-qualified-main.json` pattern) | distance-remaining reference, priced on the §5a band |

Arms C and D are not "a third solver on the same system" in the sense of §7:
they are not candidates, they are the reference instruments every later
candidate is compared to, and C is the first XOR-aware external solver run on
this exact corpus.

**Recorded per target:** verdict (and agreement with A), conflicts, decisions,
propagations, XOR-elimination rows, counted field operations where the arm
reports them, wall ns, peak RSS, binary SHA-256, commit, `rustc -Vv`, host tier.

**Controls and ledger** (`controlled-experiment` skill): negative population =
uniform targets; positive control = planted targets, expected decomposed by
every arm; A/A = B vs B′; same instrument on every arm; variable ledger with
instrument, host, order and seeds fixed or randomised as stated; frozen
`PROTOCOL.md` (`experiment.protocol/v1`) committed before the first row.

**Predictions written before data:** B at R = 1.0 ± 0.1 on every rung (the
survey's claim, re-tested); C within 0.5–3× of B; D above C; no arm below 0.9 at
ℓ ≥ 8; A/A spread under 5 %. If B lands outside 0.8–1.3 the instrument differs
from October's and the difference is explained before anything else runs.

**Execution route:** `isolab-eks` (default for x86 CPU benchmarks), L2
admission for the wall-time columns; `scripts/checkpoint.sh init` before launch;
`tools/benchrec.py init|run|close` around the run; `experiment-director`
compiles the task card, `executor-mechanical` runs it, director assesses;
validator agent replays ℓ = 8 and 9 with its own build; pcc claim filed; report,
SVG graph and PDF per `research-visuals`; row in `benchmarks/README.md` marked
pre-gate until the validator report and replay certificate exist.

**Budget:** build and dry run locally at ℓ = 5…6 (minutes); EKS run ≈ 2–4 h
single-core per arm at ℓ = 10 (arm D may censor; censored runs are reported
as censored, never imputed).

## 3. Phase 1 — candidates, ranked by (value of the answer) / (cost), each with a kill rule

Each candidate is a separate protocol on the B0 corpus, run only after B0 is
frozen, and stopped by its own rule. The order is the order of execution.

### 3.1 XOR-learning CDCL inside `sat.rs` (open item 1, unblocked by writing it)

Mechanism: today's native solver learns clauses from conflicts but keeps the
parity rows as a static Gauss–Jordan system. XLIN / RES(⊕)-style learning
derives a new *XOR* constraint from each conflict that involves the parity
system, so a conflict excludes a linear subspace of assignments rather than a
single cube. This is the one SAT route with a proof system stronger than
DPLL + Gaussian elimination, and the question Trimoska's thesis leaves open.

- Work: a learning rule for conflicts whose analysis crosses a Gauss–Jordan
  pivot (derive the implied linear combination, add it as a learned parity row,
  with deletion policy), plus an ANF → 2-XNF writer so the same corpus can be
  fed to an external XLIN solver later. Rust only.
- Falsifier of the survey verdict: `R < 0.5` at two consecutive ℓ ≥ 8.
- Abandon: `R ≥ 0.5 · (2^{3ℓ}/3!) / T_enum` at ℓ = 9 (no pruning above the last
  level), or learned parity rows per conflict below 0.01 after 10⁵ conflicts (the
  rule never fires on this structure).
- Expected (survey §3): engineering at best; the "one conflict per full tuple"
  argument predicts the learned rows are the Macaulay degree-2 rows already
  injected.
- Cost: about one week of implementation; B0-sized runs.

### 3.2 Extension-field conflict learning (new angle; cheapest test first)

Mechanism: once m − 1 summands are assigned, the residual `S′₄` is a univariate
polynomial over `F_{2^n}` and the conflict is "no root in the factor base". The
certificate of that refutation (the gcd with the factor-base vanishing
polynomial, or the resultant) is an `F_{2^n}`-algebraic object, and on a
Frobenius-orbit factor base (`koblitz-orbit`) it is invariant under the orbit.
A learned nogood of that form excludes the whole orbit of partial assignments
at once, which a GF(2) clause cannot express. This is "conflict-driven Gröbner"
in the survey's language, and the survey predicts a native solver in that
direction becomes Gröbner, which is enumeration in disguise — so the test must
be able to show exactly that.

- Discriminating test (no solver change): instrument arm B to dump every
  conflict's partial assignment on B0 at ℓ = 8; compute offline, for each
  conflict, the number of distinct candidate tuples its algebraic certificate
  would exclude on a Frobenius-orbit base of the same size. Report the median
  **generalisation factor** G.
- Kill: median G ≤ 1.5 (nogoods do not generalise; each still costs about one
  tuple). Proceed to an implementation only if median G ≥ 4 and the certificate
  cost per conflict is below G field multiplications (otherwise it is a slower
  enumerator).
- Cost: 2–3 days for the trace and the offline count; implementation only on a
  pass.

### 3.3 Amortisation across targets: the pending replication, then PDP

Mechanism: one incremental solver per factor base; each target enters as
assumptions; learned constraints persist. The ladder study found REUSE/FRESH
far below 0.2 and its decision rule prescribes a replication that nobody ran.

- Step 1 (prescribed, unchanged): replicate `sat_ladder_scaling_20261110` with
  a second solver (CryptoMiniSat via crate) and 40 targets on its own frozen
  grid; the ladder circuit is not PDP, so this decides the thread, not the IC
  question.
- Step 2: the same FRESH vs REUSE design on the B0 corpus (arm B and arm C),
  targets in seeded random order, 16 + 16 per rung.
- Falsifier: REUSE/FRESH median conflicts `< 0.5` at ℓ = 8 and 9 **and** the
  amortised total over a relation batch stays under the preprocessing bound the
  ladder study names (`S · T² ≳ n`); otherwise closed. The pair table already
  amortises a factor-base precomputation, so REUSE is also read against arm A's
  table build + query cost, not only against FRESH.
- Cost: 1–2 days; the ladder tool exists.

### 3.4 Native ANF preprocessing (Bosphorus-style), as a control not a candidate

Reimplement the two steps that matter (linear/XL-lite elimination on the
degree-2 rows; ANF→CNF with Karnaugh-map cuts) in Rust in front of arms C and
D. Falsifier `R` before/after `< 0.5` at ℓ = 8; expected: it rediscovers the
Macaulay rows arm B already injects (accounting). One day; run only if 3.1 or
3.2 proceeds, to separate "the preprocessing did it" from "the learning did it".

### 3.5 Closure track (theory, in parallel): a lower bound for RES(⊕) on `S′₄`

If 3.1 and 3.2 both close, the honest end state is a theorem, not another
measurement: a size lower bound for RES(⊕) / XLIN refutations of the
symmetrised `S′₄` system under a subspace factor base, of the form
`2^{Ω(mℓ)}`. That would close the SAT route rigorously rather than by
exhaustion of solvers. Scope with the literature on RES(⊕) lower bounds for
structured XOR systems; statement first in prose, Lean only if the statement
stabilises. No compute; frontier-model time only.

### 3.6 Explicitly not funded

Analog / Ising / annealing / memcomputing SAT: the corpus is XOR-dense, which
is the worst case for every such device, and the survey already lists SAT
hardware as not worth a round. GNN branching: `pdp_ml_sat_ceiling_20261027`
("the ceiling is real, the signal is not"). Both reopen only if 3.1 or 3.2
fires.

## 3.7 Status after the first session (2026-11-15, all `[pcc:unverified]` until the rows and validator land)

| item | status | evidence |
|:--|:--|:--|
| B0 instrument | built; ℓ = 5, 6 checks clean (0 disagreements, 0 unverified models); protocol frozen at dec928b6; rows ℓ = 7…10 running | `b0/PROTOCOL.md`, `b0/protocol.json`, session log |
| incumbent | `R₃ ≈ 1.06–1.16` (one conflict per triple) at ℓ = 5, 6, 7 — the survey's reading reproduces | ℓ = 7 check: R₃ 1.062 |
| repository chain encoder (`chain-p12`) | 15–25× worse than the incumbent at ℓ = 5 (R 160–262): AND-gate products hide the linear structure | ℓ = 5 run |
| **new solver `lin`** (field-product gates, §3.1-adjacent, built instead of XOR-learning) | `R = 1.20 / 1.46 / 1.27` at ℓ = 5 / 6 / 7 vs incumbent 10.9 / 21.9 / 43.9; 35× fewer conflicts and 11× less wall at ℓ = 7; every verdict matches the oracle | ℓ = 5–7 checks |
| 3.3 REUSE | plateau `R → 1.00` at ℓ = 6 (from 1.46); learns the root-less pairs; ℓ = 7, 8 running for the falsifier | 64-target runs |
| 3.2 certificate generalisation | answered negatively by B3 data: no refutation below the pair count in any arm | learnt-width histograms |
| 3.1 XOR-learning in `sat.rs` | superseded: the product-gate solver realises the linear structure directly; XLIN learning would act on the same residual | — |
| ceiling | per-target pair count is the floor of any CDCL formulation here (inverse lookup `t ↦ (x₁,x₂)` is not expressible as propagation); the pair table stays 2^ℓ ahead | argument in the session log; to be written in the B0 report |

## 4. Decision rules for the programme as a whole

1. **Baseline first.** No candidate run starts until B0 is frozen, validated at
   ℓ = 8–9 by an independent build, and its pcc claim filed.
2. **The survey's falsifier is the bar.** A candidate is "alive" only on
   `R < 0.5` at two consecutive ℓ ≥ 8, uniform targets, verdicts matched to arm
   A. Anything else is engineering or accounting and is labelled so.
3. **Distance, not pass/fail.** Every report states each arm's S against
   matched rho at w = 0, 0.1 and 1 GAE per unpriced operation and ranks arms only
   where the order holds across the band (AGENTS.md §5a).
4. **One candidate at a time**, stopped by its own kill rule; results, graph
   and PDF per `research-visuals`; leaderboard untouched unless a validated row
   moves.
5. **Stop the programme** when 3.1–3.3 have each hit a kill rule; hand the
   remainder to 3.5.

## 5. Model tiers and delegation

- Frontier (this session / Fable): §2 protocol text, §3.1 learning rule design,
  §3.2 certificate definition, §3.5 statement.
- Opus-tier: analysis scripts for R and the §5a band, conflict-trace analysis
  for §3.2.
- Sonnet (`experiment-director`): task cards for B0 and each candidate run,
  assessment of executor returns.
- Haiku (`executor-mechanical`): runs the cards verbatim.
- Validator agent: independent build and replay of ℓ = 8, 9 for B0 and for any
  arm that crosses `R < 0.8`.

## 6. Process checklist for the first PR that lands data

- [ ] Conductor task (`conductor check`; absent in this container — hooks warn).
- [ ] Lead atlas cards re-read once `KEY` is set; proposal file otherwise.
- [ ] KB queries (positive and negative form) once `research-kb` is reachable.
- [ ] `PROTOCOL.md` frozen (commit hash recorded) before any B0 row.
- [ ] `.resume/RESUME.md` filled; `scripts/checkpoint.sh` gate passed.
- [ ] `tools/benchrec.py` record open → run → close; raw data in S3 by SHA-256.
- [ ] isolab-eks receipt: commit, binary hash, host tier, L2 admission.
- [ ] pcc claim per measured statement; prose tagged otherwise.
- [ ] Validator report; replay certificate; matched-rho reference → `VALIDATION.json`.
- [ ] Report, SVG, PDF; `benchmarks/<date>-<slug>/` row (pre-gate until validated).
- [ ] Session log entry; PR ready for review with auto-merge.

## 7. Immediate next steps (in order)

1. Add `cryptominisat` (patched copy under `third_party/`, `NOSQLITE=ON`,
   `-include cstdint`) and `cadical-sys` as optional, feature-gated
   dependencies. Both are confirmed to build and run through the crates.io
   path in this container (§0, "Build check").
2. Write `PROTOCOL.md` for B0 from §2 and commit it (freeze).
3. Dry-run all five arms at ℓ = 5, 6 locally; check verdict agreement with arm A
   on every target; fix the instrument before any rung ≥ 8 runs.
4. Director card → executor → EKS run → validator → report → PR.
5. Only then: §3.2's trace dump (cheapest discriminating test), then §3.1.
