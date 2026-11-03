# Status report: index-calculus tournaments, prime-field factor bases, other threads

As of 2026-10-31. Compiled from three read-only surveys of the cloned repositories (`crypto`, `cryptanalysis`, `crypto-autoresearcher`, `cryptanalysis-rl`, `ml-cryptanalysis`). This report records what the documents state. It makes no assessment of the work. Status words are quoted from the source files. Paths are relative to the repository named in each heading.

**Coverage limits.** Not read in full: the 9.5 MB `crypto/docs/ic/leaderboard.json`, `ic-autolab/SKILL.md` (skimmed), `n83_final_tournament_20261015/admission.json`, `docs/ic/{FRAMEWORK,FIXED_PARAMETERS,README}.md`. No open-PR list was obtained (`gh pr list` returned 403). The survey agents' summaries were not independently re-verified.

## 1. Repositories

| Repo | Purpose (its README) | Latest on `main` |
|---|---|---|
| crypto | Root-finding decomposition engine in ecbench; Bochum v0.2 hashed factor base; Hexagon cs.CC review | 4b01aee9 (PR #3069, session log only) |
| cryptanalysis | C11 discrete-log library, Rust/Go/Python bindings, Rust attack suite | PR #1332 (GLV-table confirmation protocol) |
| crypto-autoresearcher | Multi-agent orchestration for ECDLP experimentation | PR #2853 (PFDR design freeze) |
| cryptanalysis-rl | RL fine-tuning starter on Caesar/affine ciphers; ECDLP tool-policy experiment E7 | PR #8 (E7 stage 0b) |
| ml-cryptanalysis | Curve Structural Discovery (CSD) | PR #56 (wire agents to ecdb) |

Governing rules (crypto/CLAUDE.md): success criterion of 2026-10-15 (every verified, measured improvement over the current IC baseline counts; gap to matched rho is reported as distance remaining); validation gate of 2026-10-19 (replay certificate, validator report, matched rho reference with §5a band at 0 / 0.1 / 1 GAE, named by `VALIDATION.json`).

## 2. Index-calculus tournament

### 2.1 N83 final tournament (`research/n83_final_tournament_20261015/`)
- Described as a "cross-panel final factor-base tournament"; class "cloud-instance exploratory many-point selection panel".
- Curve `EC1N83Ckb1a1haacb69ee9f37` (GF(2^83), y²+xy=x³+x²+1, prime subgroup order 8569786107849059).
- Admission: two standing controls (K3300/root11/wide3, K3596/root4/Bloom-off) plus the lowest-mean-online arm from each of four source runs. 2–6 candidates.
- Measurement: 4,000 fresh public points, each candidate run in list order and reverse order; 80 blocks of 50, 10,000 bootstrap resamples, 99% interval. Winner declared only if its upper endpoint against every other candidate is below zero; otherwise `BOUNDED_UNKNOWN`.

Result (`research/n83_final_tournament_results_20261015/RESULT.md`, run 38793258972; 8 arms passed independent replay):

| Candidate | Usable base points | Mirrored mean online / target | Peak RSS |
|---|---:|---:|---:|
| K3425/root4/standard | 568,550 | 28.688 ms | 14.425 GiB |
| K3450/root11/wide3 | 572,700 | 28.808 ms | 11.504 GiB |
| K3300/root11/wide3 | 547,800 | 30.673 ms | 11.052 GiB |
| K3596/root4/off | 596,936 | 35.384 ms | 13.959 GiB |

- K3425 vs K3300: advantage 1.985 ms, 99% interval [0.287, 3.722] ms.
- K3425 vs K3450: 0.120 ms, 99% interval [-1.545, 1.804] ms. Stated decision: `BOUNDED_UNKNOWN`; both admitted to fresh-point confirmation.
- Fresh-point confirmation: K3425 ahead by 0.399 ms, 99% interval [-2.104, +1.323] ms, `BOUNDED_UNKNOWN`.
- Earlier: K3300/root11 vs K3100/root22 on 4,000 fresh points, 37.035 vs 42.160 ms, difference 5.126 ms, 95% interval [3.386, 6.916].

### 2.2 Same-point IC vs plain rho (Tier 1; `research/n83_unresolved_tier1_results_20261015/RESULT.md`, run 38818664674)
One previously unseen point, GitHub-hosted AMD EPYC 7763:

| Pipeline | IC online | Matched rho online | Rho/IC | IC setup |
|---|---:|---:|---:|---:|
| K3425/root4/standard | 10.793 ms | 4,461.115 ms | 413.34× | 614.7 s |
| K3450/root11/wide3 | 35.624 ms | 2,276.915 ms | 63.92× | 552.9 s |

The file states one point cannot resolve the ranking (`BOUNDED_UNKNOWN`). That rho starts at its first target-dependent walk (no precomputed table).

### 2.3 Precomputed-table rho controls
- `research/20261016-n83-precomputed-rho-control/README.md` (2026-10-16): "Neither N83 'IC beats rho' online reading survives a rho given matched precomputation." K3300 vs Bernstein–Lange table: median IC/rho 3.74 (2.65–11.05) time-matched, 40.61 (24.27–71.72) memory-matched. K600: 81.5–214.7 time-matched, 395.5 memory-matched. Positive control (empty-table rho / IC): 70.9× for K3300.
- `research/n83_k1107_bl_reproduction_20261028/RESULT.md` (run 4, K1107/root11, 16 targets): IC online median 115.0 ms; IC/rho 40.9 (650 s table), 4.10 (149.6 s table), 33.1 (memory-matched). File states "precomputed rho is faster online in all three matched arms" and that this is not an improvement, gate result or leaderboard row.
- `docs/ic/LEADERBOARD.md` (2026-10-20 corrections) strikes empty-table online rows as "Superseded (rho had an empty table); not an improvement" (including n83 dense K1798, 18.53×).
- Atlas `LEAD-ICB-retained-setup-online`: `closed_dominated`; best_measured "None admitted… precomputed rho / IC online is 3.74x-396x in rho's favour at N83".

### 2.4 Cold-base and 16-point panels (`docs/ic/LEADERBOARD.md`, N83 Tier 1)
- K1120 five-point cold holdout (2026-10-16): selected `null`, `BOUNDED_UNKNOWN`; cold IC/rho online 55.4×.
- 16-point frontier (2026-10-17): 8 configs; online and cold selectors both `BOUNDED_UNKNOWN`. Example 1107/r11: online median 79.8 ms, cold mean 104.8 s, paired rho/IC median 18.5, IC faster 16/16.
- `n83_cold_base_confirm_20261021`: K1107/root11 beats K1103 by 1.195 s cold on 16 points, 95% interval [0.957, 1.433] s; `COLD_WINNER_CONFIRMED_WITHIN_PAIR`.

### 2.5 Native candidate tournament (`research/ic_candidate_tournament_20260915/`)
- Round 0026 (2026-10-28): 10.7× over matched rho in Valgrind Ir. Decision: batch16 reduces complete cold instruction cost on a bounded panel; wall-time condition "unqualified"; promotion awaits isolated confirmation; class "Engineering"; selected configuration "remains above the paired rho on all five curves". 1,632 trial receipts VERIFIED; holdout n19a1.
- Native `icprog tournament prepare|run|status|verify|propose` port (PR #2753) reproduces the frozen audit for rounds 0019, 0020, 0023, 0024, 0025 (session log 2026-10-28).

### 2.6 Related studies
- `ic_solver_selection_20260915`: "implemented, no routing gain in the initial portfolio". 288 natural queries + 16 planted controls, 912 policy outcomes; validation-loss improvement over constant 0%; hindsight oracle and best constant identical cost; 72/72 processes, 144/144 targets, 420 relations verified; adapter "not admitted to that performance tournament".
- `ic_tree_split_cutout_20260930`: registered verdict "CLOSED, for both T2 and Z" (gap 0 in 40 T2 and 8 Z cells; median slope 0.85 vs ALIVE threshold ≤ 0.15). Scope: two Koblitz curves, n 13–19, l 4–8; "No statement about n≈83 or n=131."
- `cryptanalysis/experiments/ic-candidate-catalog`: 1,000 "design proposals, not 1,000 measured attacks".
- `n83_factor_base_designs_20261031`: "research design and certificate tooling; no new factor-base measurements"; sixteen candidate constructions, execution pending; target `icv1-f2m83-tm6151469093347-debefd74` (order 2417851639230796216685689, cofactor 4).

### 2.7 Leaderboard / validation-gate status
- Tables A–C: no row stated below its reference. Table C (aimed m=3, 32 targets) rows "unvalidated (pre-gate)", ratios to matched rho 3.62× (n41) to 14.9× (n19).
- N83 Tier 1 sections are separate from Tables A–C. No `VALIDATION.json` was found in the n83 tournament / tier1 / bloom directories (search limited to depth 2).
- Validated with caveats: prime P20 N=45 baseline (`prime_base_oracle_tuning_20261029/VALIDATION.json`); second-host replay 1,008/1,008 records.
- Open regression (leaderboard line 96, PR #2290): current-binary root-finder replay-all 72/168 identical.
- GHS-weak E_0/F_{q^3} family: domain-restricted cold one-target IC win, reported as its own problem class (session log 2026-10-31).
- Atlas `LEAD-ICB-multi-target-and-portfolio`: `closed_obstructed` (restart portfolio of IC and rho cannot beat pure rho when IC cost is at a floor ratio ≥ 1).

## 3. Prime-field factor bases

Primary source: `docs/ic/factor-base-encyclopedia/README.md` (2026-10-15, updated 10-16), which "compiles results that are already recorded and runs nothing new".

### 3.1 Families and status
- **Implemented (sweeper/ecbench):** small-abscissa interval (A1, control); automorphism orbit `glv-orbit` (A3; j=0: 6 points/column, j=1728: 4); Kummer tower `prime-kummer:t=T`; isogeny tower `prime-isogeny:t=T` (PR #1889; supersingular aux since #1947); Dickson torus tower (A5c, #1893). Factory: intervals, arithmetic progressions, seeded x-subsets.
- **Built/measured, not a counted IC base:** A5b P-256 Dickson torus (24 rounds, 2026-10-04..06); A6 bit-box; A7 PGL₂ root orbit; A8 Eisenstein-smooth; A9 AP and mod-ℓ partitions; A10 ALPF; A11 augmented isogeny preimage, conductor-gap, ML yield-label study; A13 coset/low-x/low-digit/random comparison (Sage/Python/C; native rerun required).
- **Proposed only (A12):** height box "specified"; degree-d map bases "keep proposed and do not approve"; CM-factor-base intakes "unapproved"; xedni lifting, embedding into F_{p^n}, quasi-subfield at n=1, Weil-descent analogues "closed on paper" (last "structurally hopeless").
- **Not applicable to F_p:** Gaudry–Diem subfield base (B1), Frobenius-orbit and trace-kernel families. `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §0: F_p has "no subfield, no Frobenius and no nontrivial automorphism (for generic j)".
- **Algebraic prime solver:** `docs/ic/SWEEPER.md` (2026-10-29): "No algebraic prime space… `prime-table` is reference only". Factory delivery table: prime-field regularity "Partial… algebraic encoding and solver remain unimplemented". Prime solver path m=2 only, p ≤ 1,000,000. No msolve build known sound above p=2¹⁶. Reference arm: "No code exists" (`prime_ic_program_20261029` §2.2).

### 3.2 Measurements
- **Generic small-abscissa (A1):** at r=2^23.4, |F|=512, relative to rho: S₃ roots 1,107×, subtraction 436×, mitm m=2 59.1×, mitm m=3 14.5×; fitted exponents 0.905 / 0.850 / 0.662 / 0.601 vs rho 0.274. Yield/ceiling 0.75–1.25 ("behaves like a random set of its size").
- **Table B (leaderboard L154–171):** eight prime rows fp10–fp24 at 5.47×–13.5× matched negation-map rho, m=2 pair-table; `pricing_ok=False`, `protocol_frozen_ok=partial`; tracks README: IC/rho 4.66–16.30 across the pricing band.
- **Base-size tuning (20-bit):** N=32→45: S ratio 0.818 [0.773, 0.866] (w=0), 0.726 [0.677, 0.778] (w=1); gap to rho 9.47× → 7.75×. Tracks README: the 24- and 28-bit selections are "relabelling, not improvement".
- **Frontier campaign (PR #1837, 2026-10-15):** prime pair table 9.8–95.2× rho (20–40 bits, w=0); Kummer tower 30.3–141.3× (20–28 bits, w=0), 31.0–238.7× (w=1). Quote: "No index-calculus arm beats generic rho on any curve at any band weight; no index-calculus arm has a fitted cost exponent below rho's." Family law S = 0.75·r^{1/6}.
- **Matched-size geometry (A2):** mean yield ratio exactly 1.000000 over 288/288 cells (KN-FIND-007: yield depends only on |F|); `rejected_scoped`.
- **Kummer tower vs plain base of equal size:** 0.999–1.012× (896 runs, 20–28 bits). **Isogeny tower:** 1.002–1.331× (1,080 runs); aux search 0.8–25% of S; 21.0–136.5× rho. A13 coset ÷ random throughput 1.020 [0.959, 1.083] ("does not establish a coset throughput advantage").
- **Solving degree D:** Kummer m=2: 5 5 5 5 6 6; isogeny m=2: 5 to N=18, 6 at N=20; m=3: 7 at N=18; m=4: ≥ 8 at N=20 ("NEGATIVE"). Raising 2-adicity to v₂=96 does not change D.
- **Orbit fold (A3):** column ratio exactly 3 (j=0), 2 (j=1728); full-rank ratio 2.83–5.48 and 2.22–2.96.
- **Free relations:** the one census (2026-10-12) is Koblitz: homogeneous rank 49/599 (n=61, K=600), 6/599 (n=83); "Lead B is closed in both directions". No prime-field free-relation measurement found.
- **Fast prime engine (2026-10-08):** IC-2 at 20 bits 352 s → 0.50 s; IC-3 reaches 32 bits in 20.5 s; rho 0.5 s at 32 bits ("No vs_rho crossover"); rho reaches 56 bits in 20 s.
- **Root finding vs Gröbner (2026-10-12):** per decomposition ~35,000–47,000× vs `groebner_f4` at 16–24 bits; 3,021–5,802× on NIST P-curves; end-to-end vs msolve 100× (16 b), 105× (20 b), 50× (24 b). "No security claim"; on the "generic" track, "cannot beat rho" (SWEEPER.md).
- **Preprocessing-rho control (2026-10-08):** 11–60× fewer per-target steps than plain rho at 32–44 bits; needs 1–116 targets to break even.
- **Small-root lattices:** Howgrave-Graham condition never holds beyond δ=0.10; need δ ≥ 1/2 (S₃), 1/4 (S₄), ≈1/5 (S₅); lattice reach 1/6, 1/24, 1/80. `smallroot_optimal_reach_20261026`: max 0.3293 per S₃ block vs demand 0.8889.
- **Isogeny/volcano on prime fields:** augmented F ∪ φ⁻¹(F) null (Δ < 0.004); conductor-gap floor/crater ratios ≈ 1 up to gap 2,033,609; ML study 106,594 nodes / 254 classes, held-out R² 0.021. Transfer-aware note: conservation lemma — a homomorphism transfer cannot change decomposition counts.

### 3.3 Recorded blockers / open items
- Decisive PKM cell (m=4, N=20) needs 240–380 GB (derived) vs 128 GiB pool cap (O12). Relaunch of m=3 N=21 and larger-t cells staged; blocked on harness-host route and Mac-only raw data (O7).
- Fall-degree medians d_F = 3,4,4,4,4,5,5,5,5,6 (t ≤ 10); Amendment 6 predicts 4+⌊(t−2)/4⌋ (7 at t=14, "the decisive cell"). `LEAD-ICP-pkm-tower-algebraic-oracle`: `open_blocked`; Coppersmith small-root lead `closed_obstructed`.
- Reference arm (Semaev-naive + msolve) needs pinned msolve, F_p solver plug point, EKS identity; approved ≈ 49 CPU-hours (derived; 15–150).
- Prefetch patch "uncompiled and untested"; expected relation-phase gain < 5% (derived).
- No relation phase run at P-224/P-192 size; P-192 t=48 aux curve derivation costs 11.2 core-years (derived).
- Open question (encyclopedia): whether any F_p base has membership degree far below |F|; Bezout gives B ≤ 3d; only auxiliary-variable towers escape. Proposition 1 "excludes towers".
- Open decisions O3 (what counts as algebraically defined base), O4, O9 (Table B pricing caveat), O11. `Track::Generic` relabel not yet in code.

## 4. Other threads

- **Lead atlas** (`crypto-autoresearcher/knowledge/leads/ecdlp/`, reviewed 2026-10-30): 154 leads (REVIEW_STATE.json tally of a slightly earlier snapshot: 153): 74 closed with named obstruction, 54 open with decided next step, 17 blocked, 9 unexamined; 2178/2185 ledger records mapped; later commits in crypto (154 paths), cryptanalysis (18), crypto-autoresearcher (53) not yet reviewed.
- **crypto/RESEARCH_*.md** (status words quoted):
  - ECC2K130 IC feasibility: "Status 2026-10-16: the n=131 figures below are illustrative, not a bound"; crossover "withdrawn as a number".
  - Koblitz IC: "No IC efficiency verdict is supplied".
  - Isogeny fibers: "Theorem audit + proofs at the level of propositions"; rounds 4–6 toy/synthetic only.
  - Isogeny loop GLV: "Tool landed and validated; sweeps measured on public curves."
  - PDP speedup: measured on public synthetic small-field instances.
  - G2 equivariant F4: "Scoping with one measured falsifier."
  - GPU rootfind: host-emulated kernel validated against CPU oracle; "The CUDA path has NOT been executed on a GPU"; throughput "ESTIMATE".
  - EndRing PKE: survey + design + SageMath toy; figures are "derived estimates, not measurements".
  - P-256 speculation: "No weakness in P-256 is claimed or found."
- **ecbench** (`crypto/docs/ecbench/README.md`): rho variants, BSGS, kangaroo, IC in one counted unit; pipeline run → verify → compare → table → db sql. `ic.trace_line_2lp`: "GAE is a lower bound, not complete C1 cost".
- **ecdb:** PostgreSQL curve catalogue with HTTP API/MCP; AWS deployment.
- **ECC2K130 campaign** (`crypto/ecc2k130/README.md`): G7 (g7.2xlarge, RTX PRO 4500) 12.499 B complete scalar updates/s vs control 11.218 (ratio 1.1211, CI [1.1057, 1.1367]); "The 12 B/s G7 milestone is reached. DP34 collection throughput for this map and G7e throughput remain unmeasured."
- **cryptanalysis:** PR #1332 GLV-table protocol frozen (64/128-entry table at 42–46 bits, seed 20261102, before data); Lean receipt/corpus PRs #1323–#1333; Xedni lattice obstruction #1288; hashed-ecdl phase accounting #1325; Claude/Codex relay #1326.
- **cryptanalysis-rl E7** (status 2026-10-27): protocol "drafted… not frozen. No model has been trained or evaluated." Stage 0b (1200 instances × 5 arms): bluff control passes; oracle detection 7/8 cells pass (singular 32-bit g=1: median 0.118 vs 0.1 threshold, "unchanged pending an owner decision"); textbook triage solves 93–94%, triage-census 100%; oracle margin over triage-census ≤ 0.019; "at most about 0.02 reward of room above the scripted triage, under this reward and population."
- **ml-cryptanalysis:** milestones 1 (planted automorphism) and 2 (isogeny volcano exact-rule mining, "zero violations on complete enumerations") "done and reproducible"; other experiment directories listed but outcomes not read.
- **crypto-autoresearcher:** ROADMAP.md "Initial ECDLP Autoresearch Roadmap"; PUBLICATION-CANDIDATES.md (AES, 2026-08-02) states novelty "cannot be adjudicated here in either direction".

## 5. In-flight items recorded in the logs (2026-10-31)
- Frontier review: nine matched E2E specs drafted, `experiment.protocol/v1` passes validator, status `draft`; blocked on ECR image digest and authenticated EKS route; GitHub credentials rejected in that session so no push/PR; local timing deliberately not substituted.
- Log of PR #3069: "Lean proof gate not executing on main; pin change unverified".
- Open PRs not enumerated (API 403).
