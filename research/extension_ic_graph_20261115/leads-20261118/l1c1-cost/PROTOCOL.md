# L1 x C1 descent cost: a preregistered 2x2 on shared descent streams

Lane key `l1c1-cost`, written 2026-11-20. This is a **preregistered protocol,
written before any data exists**. No target was generated, no stream was run
and no result exists. Every number below is a model, a design computation or
an earlier producer observation, and is labelled [pcc:unverified]. No pcc claim
is registered. The commit that first adds `protocol.json` at this path is the
freeze. A separate session runs it with `run.sh`, after drand round R (at least
one hour after the freeze), and changes nothing listed in `SOURCES.sha256`. Any
change after the freeze is a numbered amendment committed **before** the data
it affects. A silent re-run is inadmissible.

Machine-readable form: [`protocol.json`](protocol.json) (`experiment.protocol/v1`).
Where this file and `protocol.json` disagree, the runner stops and asks. The
runner does not choose.

## 1. Question, arms and endpoint

**Background** (all [pcc:unverified]).

- L1 ([`../l1-paired/results/README.md`](../l1-paired/results/README.md)): on 96
  fresh p1009 targets, accepting `(1^4,2)`, `(1^2,2^2)` and `(2^3)` solutions
  whose quadratic classes are in `D_exact` gave ρ = M/F = 1.42 (95% 1.27–1.59).
  That was INCONCLUSIVE against its 1.5 line. Arm B (full splits plus L1 rows)
  had 2.42x the verified successes per solution of arm A. Censored mean
  residuals to the first success were 705.8 (A) and 275.5 (B).
- C1 ([`../descent-finder/`](../descent-finder/README.md),
  [`../descent-walltime/`](../descent-walltime/README.md)): F4 trace replay cuts
  F4 multiplications 2.29x on identical streams. The complete counted descent
  ratio is 1.68x on seed 389 (linear stage counted, learning RREF at its upper
  bound). Local wall time is 1.26–1.29x (indicative).
- Complete accounting ([`../complete-accounting/`](../complete-accounting/README.md))
  supplies the counted linear stage. Without it, descent was undercounted by
  about 28%.
- L5 ([`../l5-kernel-split/`](../l5-kernel-split/README.md)): Krylov is 17.4% of descent.

L1's result is an acceptance rate, not a cost. C1's is a cost on a fixed stream,
not an end-to-end stop. Critic F6 ([`../CRITIC_ROUND2.md`](../CRITIC_ROUND2.md))
forbids multiplying the two (the scout's "L1xC1 ≈ 0.69e12").

**Question.** On fresh targets and identical descent streams:

1. Does L1 lower the **actual cost of descent**, all of L1's own work included?
2. Does C1?
3. Do they compose? Is the cost of L1+C1 the product of the two single effects?

**Arms** (2x2 factorial; every arm on the SAME target, collection, factor base,
`D_exact`, certified quotient and residual stream):

| Arm | Solver | Accepts | Notes |
|---|---|---|---|
| **base** | production staircase F4 + linear stage | `(1^6)` materializer decompositions | Run50's descent as frozen |
| **l1** | production | base rows **or** L1 rows | counted L1 matcher (§1.2) |
| **c1** | C1 traced F4 (one learned trace per process) | base rows | fallback to plain F4 on rejection, both charged |
| **l1c1** | C1 traced | base rows **or** L1 rows | |
| **base-aa** (wall only) | production | base rows | A/A repeat of base, in its own process and position |

An arm runs as production would. Residuals are drawn in stream order. For each
residual the arm:

1. solves its Weil system and materializes the `(1^6)` decompositions;
2. if L1 is on, runs the counted L1 matcher on every `F_p`-solution;
3. projects every row that passes production checks, and accepts it **if it
   raises the rank** of the arm's two-column affine system;
4. stops after the residual in which the rank reaches **K = 2**, which is what
   Run50 needs.

It then solves for `d` and checks `[d]G = Q`. The cap is NCAP = 8,192 residuals,
the pipeline's own descent cap. A base arm that has not reached K by then is
**censored**. Its cost to NCAP is then a lower bound, which biases the result
against L1.

**C1 does not change acceptance**: the solution sets must be identical. So `c1`
stops at base's τ, and `l1c1` stops at l1's τ. **L1 can only stop earlier**:
arm B's success set contains arm A's.

### 1.1 Endpoint and metrics

**Endpoint**: for each target t and arm X, the cost from the start of the stream
to the first K = 2 rank-raising verified successes, ending with the scalar check.

**(i) Counted (primary)**: `C_X(t)`, the bitop/v1 price (b = bits(p) = 10,
r = bits(l)) of every hooked operation in the arm's window. The window includes:

- the stream set-up and advances;
- the Weil forms, the F4 or trace replay, and the **counted linear stage**
  (complete-accounting's `zero_dim_la.rs`, SHA-256 `f0c50f5f…`);
- the materializer, `verify_dec` and the projection of every row;
- for C1, the rejected replay plus the fallback, both charged;
- for C1, the learning run's tracked RREF, charged at its upper bound
  (descent-finder's bound, as `fp_mul + fp_add` per multiply-add);
- **all of L1's own work**:
  - the `D_exact` table build;
  - recomputing the Weil forms;
  - the sextic, its squarefree test and the distinct-degree pattern test;
  - equal-degree splitting of the linear and quadratic parts;
  - the factor-base probes and ordinate checks;
  - per class: the resultant tests, `V = −A/C mod U`, `V² ≡ H mod U`, the
    canonical key and the `D_exact` probe;
  - the principal-divisor identity in the Jacobian;
  - the anchor identity, once per class per target (the extra replay);
  - the substituted row and its projection;
- the rank test, the two-column solve and `[d]G = Q`.

The window **excludes** the work that every arm shares: collection, linear
algebra, the pipeline's own descent, and process set-up.

The metric is labelled a **counted lower bound**. Not priced:

- F4-replay bookkeeping beyond `f4_fp`'s hooks;
- symbolic preprocessing, hashing outside the hooks, and allocation;
- containers;
- the dense row buffer.

Uncounted replay bookkeeping is exactly what shrinks C1 from 1.68x counted to
about 1.28x in wall time. So no counted verdict is a §5a ranking (critic F5).
The counted verdict says only "reduces counted descent cost".

**(ii) Wall (indicative)**: `W_X(t)`, the thread CPU time
(`/proc/thread-self/schedstat`) of the same window, measured as follows:

- the frozen linear stage, with hooks off;
- one process per arm, on one thread;
- the five wall processes of a target (base, l1, c1, l1c1, base-aa) in a
  permutation derived from that target's `l1c1-order` seed.

The local host is shared and not isolated, so every wall verdict is labelled
**INDICATIVE**. It becomes "isolated host" only if `run.sh` is edited by an
amendment that declares an isolated host.

**Summary statistics** (one cluster per target):

- `R_L1 = ΣC_l1 / ΣC_base`, `R_C1 = ΣC_c1 / ΣC_base`, `R_L1C1 = ΣC_l1c1 / ΣC_base`;
- the interaction `I = R_L1C1 / (R_L1 · R_C1)`, which is 1 under multiplicative
  composition;
- the simple effects `S_C1|L1 = ΣC_l1c1 / ΣC_l1` and `S_L1|C1 = ΣC_l1c1 / ΣC_c1`.

These are ratios of sums, so they estimate ratios of **expected** cost to K.
Residual counts to K are negative-binomial draws (critic: descent residual
counts). A per-target ratio is heavy-tailed, so it is secondary only.
Intervals come from a target-cluster percentile bootstrap: B = 20,000, indices
500/19,499 (95%) and 100/19,899 (99%), SplitMix64 with exact rejection
sampling, seeded by `l1c1-bootstrap`.

### 1.2 The counted L1 matcher, and why there is an audit

The l1-paired instrument found quadratic factors by brute force over all of
`F_{p^2}` (about 5·10^5 evaluations per candidate solution). That is fine for
counting rows, but it is not a production cost. This protocol therefore uses
**two independent implementations**:

- **`l1_match`** (in `l1c1_cost.rs`) is the production path that the arms run
  and charge. It uses:
  - DDF by `x^p mod S` and `gcd`;
  - `x^{p²} ≡ x mod S/g₁` for the quadratic part;
  - Cantor–Zassenhaus equal-degree splitting with deterministic shifts;
  - resultants `Res(U, ·) = g0² − u1·g0·g1 + u0·g1²` for the root conditions;
  - `(x+μ)^{-1} ≡ −(x + u1 − μ)/U(−μ) mod U` for `V`.

  Every `F_p` operation goes through the `jv_bitops` hooks.
- **The audit** is the l1_paired instrument: `l1_paired.rs` lines 60–839,
  byte-copied, with one marked insertion. It is never charged. On residuals
  `0..τ_base` it classifies every solution and gives every row, of both kinds:
  - the affine path;
  - the certified projection;
  - the elliptic replay `[d]G = Q`;
  - the sign-flip negative control.

`collect` requires the arm's production-checked rows to **equal** the audit's
rows up to the arm's τ, in order. This covers kind, residual and projected
`(k, t, rhs)`. It also requires every accepted row to be audit-replayed. A
disagreement between the two implementations is a gate failure, not a judgement call.

## 2. Decision rules (frozen; implemented in `l1c1_analyze.rs`)

Order of evaluation. The first rule that applies gives the verdict.

1. **INVALID** (all effects). This applies if:
   - any per-target gate fails (§5);
   - PC1, PC1b, PC1c, PC2, DC or the freshness check fails;
   - or the counted A/A fails.
2. **KILL-CORRECTNESS (L1)** if any L1 row that passed the production checks
   fails the audit's elliptic replay (`replay_fail_M > 0`).
   **KILL-CORRECTNESS (C1)** applies in either of these cases:
   - a traced arm's solution chain differs from the production audit's;
   - a C1 replay was accepted whose solutions differ.

   The interaction and the composition verdicts then read KILL-CORRECTNESS. The
   other factor's main effect is still decided.
3. **INCONCLUSIVE-UNDERPOWERED** if fewer than T_MIN = 50 targets are measured.
   That is fewer than 100 accepted successes per arm.
4. **Main effects** (L1: `R_L1`; C1: `R_C1`; 95% interval `[L, U]`):
   - **MATERIAL-REDUCTION** if `U < 0.8`;
   - else **REDUCES** if `U < 1.0`;
   - else **INCREASES** if `L > 1.0`;
   - else **INCONCLUSIVE**.
5. **Interaction** (`I`, 95% interval). The verdict has two independent parts:
   - **Direction:**
     - SUB-MULTIPLICATIVE if `L > 1`: the combination saves less than the
       product of the two single effects;
     - SUPER-MULTIPLICATIVE if `U < 1`;
     - NOT-DISTINGUISHED-FROM-MULTIPLICATIVE otherwise.
   - **Equivalence:** MULTIPLICATIVE-WITHIN-10% if `[L, U] ⊂ [0.90, 1.10]`,
     else NOT-SHOWN-WITHIN-10%.
6. **Composition** (does L1+C1 beat each factor alone?):
   - **COMPOSES** if `U(S_C1|L1) < 1` and `U(S_L1|C1) < 1`;
   - **DOES-NOT-COMPOSE** if either lower bound exceeds 1;
   - else **INCONCLUSIVE**.

**Wall.** The same rules 4–6 apply to `W`, with two additional gates:

- the wall A/A (`ΣW_base / ΣW_base-aa`, 99%) must contain 1, else
  INVALID-WALL-AA;
- the wall A/A 99% interval must lie within [0.95, 1.05], else
  NO-WALL-VERDICT-NOISE-FLOOR.

Every wall verdict is labelled INDICATIVE (non-isolated host).

**Counted A/A** (first 32 measured targets, independent `l1c1-aa-stream` seed,
same collection). Two 99% intervals must contain 1:

- `ΣC_base^aa / ΣC_base`;
- `R_L1^aa / R_L1`.

C1's A/A is not gated: on a shared stream its ratio is nearly deterministic, so
its interval is too narrow to be an A/A.

**Always reported**, whatever the verdict:

- every point estimate and both intervals;
- the no-learning-bound counted ratios (`Cn`), which bracket C1 from the
  favourable side;
- the K = 1 (first success) ratios;
- per-target geometric means;
- mean residuals to K per arm;
- L1's added-work share;
- audit F, M and ρ̂, a descriptive cross-check against l1-paired;
- trace rejections;
- the censored count.

**No distance to rho** is computed. This is a descent-phase component
measurement and produces no leaderboard row (critics F3/F5). Under the success
criterion, a verified counted reduction of descent cost is an improvement of the
IC baseline's descent component. It is not a relabel: the arms differ in
algorithm, not in accounting, and every arm carries the complete counted linear
stage.

### 2.1 Pre-data model predictions (design only, [pcc:unverified])

The model behind these predictions: costs are counted in production-residual
units, with the following parameters.

| Parameter | Value | Source |
|---|---|---|
| r (traced/production cost per residual) | 0.5697 | 2,483,262,669,096 / 4,358,996,976,408 |
| λ (learning bound, in residual-equivalents) | 88.4 | |
| A rate per residual | 554/393,216 | l1-paired |
| L1 gain g (B rate / A rate) | 2.42 | l1-paired |
| δ (L1 overhead per residual) | 0.01 | |

Under this model, the expected ratios are:

| Quantity | Expected value |
|---|---|
| `R_L1` | ≈ 0.42 |
| `R_C1` | ≈ 0.63 |
| `I` | ≈ 1.14 (SUB-MULTIPLICATIVE, not within 10%) |

The interaction comes from three things that do not shrink in proportion:

- C1's per-process learning charge is fixed;
- that charge is amortized over the fewer residuals L1 needs;
- L1's own work is not reduced by C1.

The interaction verdict therefore partly reflects the learning-bound
convention. The secondary `Cn` ratios bracket it. These predictions exist so
that the data can refute them. They decide nothing.

## 3. Fresh public targets

The family, generator and admission are those of l1-paired. The
generator is [`../fresh-scaling/gen`](../fresh-scaling/gen), unchanged and
hash-pinned. The family is the `jv_cover` weak cover over `F_{q³}`,
`q = p²`, with `p = 1009`, `#E = 4l`, `l` prime and `l < 2^62`. Public `Q` is
hashed to the curve, so nobody knows `log_G Q`.

**Nonce.** It is computed as
`fresh-scaling-gen nonce --protocol-sha256 H --commit C --drand-round R --drand-randomness X`:

- `H`: the SHA-256 of `protocol.json` at the freeze commit;
- `C`: the freeze commit, the first commit that adds `protocol.json` at this path;
- `R`: the first drand mainnet round (default chain `8990e7a9…51b2ce`, genesis
  1595431050, period 30 s) whose time is at least the freeze committer time
  plus 3,600 s;
- `X`: that round's `randomness`. `run.sh` refuses to start before round `R` is
  published.

The disclosed fallback, used only if no drand host answers, is `R = 0` and
`X` = the freeze tree id. It is grindable by whoever made the freeze, and it is
recorded as such.

**Admission.** Candidates are indices 0, 1, 2, … (first T + 16, then 16 at a
time, cap 192). Each candidate must pass both of these:

- `fresh-scaling-gen check`;
- this lane's `check_unseen.py`. It refuses any candidate that equals, or
  shares a curve `(p, α)` with, any of the following:
  - a git-tracked `*public*.json`;
  - **any line of a git-tracked `*publics*.jsonl`**, which includes the 96
    l1-paired targets (the check stops if fewer than 96 such lines are found);
  - a dry-run tarball input;
  - an earlier candidate.

The pipeline's own public transfer is the admission. Exit code 10 means
refused and not counted. Exit code 11 (collection not rank-ready) counts
toward T and contributes no rows.

**Seeds** are derived by `fresh-scaling-gen seed --nonce N --p 1009 --index i --role ROLE`:

| Role | Use |
|---|---|
| `l1c1-collect` | the collection seed, which is also the primary stream seed (the pipeline's own stream) |
| `l1c1-aa-stream` | the counted A/A stream seed |
| `l1c1-order` | the arm order (Fisher–Yates, `l1c1_analyze order`) |
| `l1c1-bootstrap` | index 0 |

**Planted targets** (PC2 only) come from
`fresh-scaling-gen pilot-nonce --protocol-sha256 H`, indices 0 and 1. The owner
answer is used only as the comparison value.

## 4. Sample size, budget and stop rules

**Design.**

- T = **128** measured targets carry the counted metric, all four arms.
- The first **64** also run the five wall processes.
- The first **32** also run the counted A/A stream.
- Each counted arm contributes K = 2 accepted successes per uncensored target,
  so **256 per arm**. That meets the ≥ 100 successes per arm at T ≥ 50, which
  is T_MIN.

**Power** (`power-design.csv`, from `l1c1_analyze power 1000 2000`; a design
computation under the §2.1 model, [pcc:unverified]):

| True L1 gain g | 1.0 (none) | 1.15 | 1.3 | 1.6 | 2.42 (l1-paired) |
|---|---:|---:|---:|---:|---:|
| T=128: P(L1 REDUCES) | 0.000 | 1.000 | 1.000 | 1.000 | 1.000 |
| T=128: P(L1 MATERIAL) | 0.000 | 0.000 | 0.086 | 1.000 | 1.000 |
| T=50: P(L1 REDUCES) / P(MATERIAL) | 0 / 0 | 0.981 / 0 | 1.0 / 0.05 | 1.0 / 0.865 | 1.0 / 1.0 |
| T=128: P(I within ±10%) | 1.000 | 1.000 | 1.000 | 0.969 | 0.000 (Î ≈ 1.14) |

The other entries of the power table:

- **L1 false positives.** With no true gain (g = 1), P(L1 REDUCES) is 0. L1
  then reads INCREASES with probability 1, because its added work is pure cost.
- **C1 power.** P(C1 MATERIAL) is 1.0 in every scenario. C1's ratio is nearly
  deterministic on a shared stream, so its power is not in question; its label
  is.

The design is powered for every decision it makes, at T = 128 and, by the
budget floor, at T = 50.

**CPU model** ([pcc:unverified], from these sources:

- l1-paired `results/cpu.tsv`: 93.6 s per target including a 4,096-residual
  audit, and 6.4 s per generation;
- descent-walltime: production 3.07 ms per residual, traced 2.41 ms;
- PC1's 9.3 ms per audit residual.

E[τ_base+1] ≈ 1,420 and E[τ_l1+1] ≈ 590 residuals.)

| Step | CPU-s |
|---|---:|
| Build (two lib builds at `-j 1`, generator, analysis) | ~200 |
| PC1 (Run50 prepare + 30,000-residual audit), PC1c (2 fixed + 4 K-stop counted arms), PC1b (l1-paired target, 4,096 audit) | ~490 |
| PC2 (2 planted, full procedure with wall) | ~230 |
| Generation (~144–160 candidates) and freshness | ~1,000 |
| 128 targets × (prepare ~50 + 4 counted arms ~20 + audit to τ_base ~15 + collect) | ~11,000 |
| 64 wall targets × 5 processes (~26) | ~1,660 |
| 32 counted A/A streams × (4 arms + audit, ~35) | ~1,120 |
| DC rerun of the first target (counted procedure) | ~90 |
| Analysis (B = 20,000) and packing | ~30 |
| **Total** | **≈ 15,800 (4.4 CPU-h)** |

At 1.5x slower collection or arm set-up, the total is about 6 CPU-h.

**Hard budget.**

- **8 CPU-hours**, measured as user+sys of every step (bash `time`) in
  `log/cpu.tsv`.
- Local only, one thread, `-j 1` builds.
- No paid compute, Runpod, EKS or remote host.
- The only network use is the drand fetch.

**Stop rules** (in `run.sh`):

1. **Budget.** Before each target, the run stops if spent CPU + 1.5 × (the
   costliest target so far) > 8 h − 900 s. The analysis then runs on the
   completed targets with `budget_truncated=1`. This reads CPU time only. Below
   T_MIN = 50 targets the verdict is INCONCLUSIVE-UNDERPOWERED.
2. **Control failure.** If PC1, PC1b, PC1c, PC2 or DC fails, the run stops with
   no verdict, and an amendment is required.
3. **Runner error.** Any runner exit other than 0, 10 or 11 stops the run.
4. **No adaptive stop.** No stop reads a cost, a τ or a ratio. The arms' own K
   stop is part of the endpoint, not of the sample.

## 5. Controls and per-target gates

| Id | Control | Pass condition |
|---|---|---|
| **PC1** | Run50 `public.json`, seeds 389/389. Frozen-build `prepare` + `audit` with N 30,000 and PRIMARY_N 3,415. | `../l1-paired/compare_pc1.py`, unchanged: every Run50 / `run50-control.json` number, both checked in full:<br>• phase subtotal 5,913,875,255,822 and descent phase 3,426,280,619,436;<br>• the `D_exact` SHA-256 (`d7246223…`) and the rows-file SHA-256 (`61ced7ab…`).<br>Also: F = 31 and M = 57, all replayed, and 57/57 flips refused. |
| **PC1c** | Counted build on the same stream: `base` and `c1` with FIXED_N 3,415, plus the four K-stop arms | Exact counted totals:<br>• production `nagao` component 4,358,996,976,408;<br>• traced 2,483,262,669,096;<br>• learning bound 112,775,271,840.<br>Trace tally (learned, accepted, after mismatch, rejected) = (1, 3,414, 33, 0). Full-split residuals {263, 3,414}. Identical solution chains. τ_base = 3,414. The four K-stop arms pass every collect gate, which means the counted L1 matcher reproduces the audit's rows on Run50. |
| **PC1b** | l1-paired's first measured target (`publics.jsonl` line 1 with its recorded `collect_seed`). Frozen `prepare` + `audit` FIXED 4,096. | Every column of `target.primary.tsv` equals the committed `../l1-paired/results/targets.tsv` row, except `public_sha256`. That one differs because the committed line had its newlines removed. |
| **PC2** | Two planted fresh targets, full procedure including wall | Every collect gate passes. Every arm's solved `d` equals the owner answer, and so does the audit's `owner_check`. Not censored. |
| **NC** | Sign-flipped L1 rows (`D_key → −D_key` for the first class) | The audit refuses every flipped row by elliptic replay (gate `flips`). |
| **A/A** | Counted: independent stream, 32 targets. Wall: base twice per target in derived positions, 64 targets. | §2 |
| **DC** | The first measured target's counted procedure is run twice | These files are byte-identical in both runs: `dexact.jsonl`, `quotient.json`, `rows.primary.jsonl`, `accepted.primary.jsonl`, `soldig.primary.txt` and the four `arm-*.counted.det.json`. The collect line is identical except for its timing columns. |
| **C1 correctness** | Every traced residual | Solution chain equal to the production audit's. Accepted replays are certified: not timed out, no pair above the bound, and `1 ∈ J` or staircase ≤ 64. Rejected replays and the fallback F4 are both charged. Rejections are reported. |
| **Freshness** | `check_unseen.py` + `fresh-scaling-gen check` | A seen or failing candidate is never measured. |

**Per-target gates** (`l1c1_cost collect`; any failure means INVALID):

- **Audit gates, as l1-paired:**
  - `group_bad`, `anchor_bad`, `replay_fail_F` and `affine_inconsistent` are 0;
  - flips refused equals flips total;
  - unreplayable rows are at most 1%;
  - `pipeline_rows_match` and `pipeline_consistency` hold;
  - the owner check holds where there is one.
- **Per counted arm:**
  - the audit covers its residuals;
  - its solution chain equals the audit's (for C1 arms a difference is a C1
    correctness KILL instead);
  - its production-checked rows equal the audit's rows up to its τ, in order;
  - every accepted row is audit-replayed;
  - the solved `d` verifies and equals the pipeline's verified scalar (and the
    owner answer);
  - no foreign hook calls;
  - the components sum to the window total;
  - zero unattributed work.
- `τ_c1 = τ_base` and `τ_l1c1 = τ_l1`; `τ_l1 ≤ τ_base`.
- **Per wall process:** τ, solution chain, accepted rows and scalar equal the
  counted arm's.

## 6. Variable ledger

| Variable | Handling |
|---|---|
| Arm | 2x2 on the same target, collection, `D_exact`, quotient, factor base and stream. Each arm runs in its own process. |
| Instrument | One source file, two builds that differ only in `zero_dim_la.rs`:<br>• frozen (`5ae85b34…`, Run50's bytes): prepare, audit, wall arms;<br>• counted (`f0c50f5f…`): counted arms.<br>Outputs are build-independent: complete-accounting checked this with 108 differential tests and the 3,415-residual stream, and PC1c and the wall-vs-counted gate re-check it. Rust 1.98, `-j 1`, offline, locked. |
| Library | The 11c9a031 deferred-pair modules over the Root41-equivalent fcda1e22 closure, with the nagao-e2e `jv_cover` overlay (`efc554a4…`), manifest-checked by `../l1-steps-2-3/build/fetch.sh` |
| Target / curve | Fresh and nonce-derived. The cluster bootstrap carries the between-target variance. |
| Stream | Derived per target and role. The primary stream is the pipeline's own. The A/A stream is independent. |
| Order | Counted arms are deterministic, so their order is irrelevant (DC checks it). Wall processes run in a derived permutation of five, with the A/A repeat. |
| Host | Shared and not isolated. Counts are host-independent. Wall is INDICATIVE, with an A/A noise-floor gate. |
| Trace state | One trace per process. Learning happens on residual 0 of every C1 arm. Its RREF is charged at its bound. |
| Censoring | At NCAP = 8,192. Censoring is reported, and it biases against L1. |
| Analyst | The frozen Rust analysis program. There is no manual inspection before it runs. |

## 7. What is recorded, and where

**Scratch** (`$S`):

- per target `out/tNNNN/`:
  - `pipeline.summary.json`, `dexact.jsonl` (about 1.5 MB), `quotient.json`;
  - the audit's `rows/accepted.{primary,aa}.jsonl`, `stream.*.summary.json`,
    `soldig.*.txt` and `target.*.tsv`;
  - `arm-*.{primary,aa}.counted.{det,time}.json`;
  - `arm-*.primary.wall.{det,time}.json`;
  - `line.{primary,aa}.tsv`;
- `pc1/`, `pc1b/`, `pc2-*/` and `dc/`;
- `targets/`, `planted/` and `log/`.

Large files are gzipped with `-n`, and their SHA-256 values are in `$S/RAW.sha256`.

**Committed** (`run.sh` stages `$S/commit/`; the runner copies it unchanged
into `runs/`, splitting any file over 64 KiB into `.partNN` pieces of at most
60,000 bytes):

- `targets.tsv` (128 lines, about 60 KB, so possibly split);
- `aa.tsv`;
- `analysis.json`;
- `controls.kv`;
- `pc1.json` and `pc1c.json`;
- `refused.tsv`;
- `publics.jsonl` (about 60 KB);
- `nonce.txt` and `freeze.txt`;
- `cpu.tsv`;
- `build.log`;
- `RAW.sha256`;
- `COMMIT.sha256`.

The results `README.md` reports the verdicts exactly as the analysis program
printed them, plus every gate and the deviations. Because this is a timing
comparison, the results PR also adds a `bench.record/v1` and a research-visuals
report. Those are the runner's and orchestrator's tasks after the data. They
are not inputs to any verdict.

## 8. Exact commands and source hashes

```
export S=<scratchpad>/l1c1-cost-run && mkdir -p "$S"     # must be empty
cd research/extension_ic_graph_20261115/leads-20261118/l1c1-cost
sha256sum -c SOURCES.sha256            # must print OK for every line (run.sh repeats it)
bash run.sh                            # build, nonce, PC1/PC1c/PC1b, PC2, targets, DC, analysis
```

Per target, `run.sh` runs:

1. `l1c1_cost_frozen prepare PUBLIC CS DIR`. The pipeline runs with Run50's
   options: `b_count 1018081`, descent cap 8192, accepted cap 32, split cap
   524288, deferred-edge cap 262144, one thread, `JV_ADDS_INC=1`,
   `JV_ADDS_SHARE=1`.
2. Four counted arms, each `l1c1_cost_counted arm PUBLIC CS CS 8192 - DIR ARM 1 DIR/arm-ARM.primary.counted`.
3. The audit: `l1c1_cost_frozen audit PUBLIC CS CS N N DIR primary -`, with
   `N = τ_base + 1` (or NCAP if base is censored).
4. If wall: five `l1c1_cost_frozen arm … ARM 0 …` processes in the derived order.
5. If A/A: the same counted arms and audit on the `l1c1-aa-stream` seed.
6. `l1c1_cost_frozen collect DIR TAG 8192 - DIR/line.TAG.tsv`.

The analysis runs as
`l1c1_analyze analyze targets.tsv aa.tsv BOOT_SEED controls.kv analysis.json`.

Source SHA-256 values are in `SOURCES.sha256` (and `protocol.json`
`sources_sha256`). `run.sh` refuses to run on any mismatch. **Freeze-commit requirement.** Two pinned files are matched by the
repository's `.gitignore` (`**/Cargo.lock`):

- `../l1-steps-2-3/build/Cargo.lock`;
- `../fresh-scaling/gen/Cargo.lock`.

The l1-paired run found them only because it ran on the host that made them.
The freeze commit must force-add both, with `git add -f`. A runner on a fresh
checkout otherwise stops at `sha256sum -c`, which is safe but gives no data.

The pinned files
include:

- this lane's sources;
- the audit's origin (`../l1-paired/l1_paired.rs`);
- `../l1-paired/compare_pc1.py`;
- the l1-steps-2-3 build;
- the generator;
- the counted linear stage;
- the PC1b and PC1c reference files;
- Run50's `public.json`;
- the overlay;
- both validation manifests.

## 9. Responses to the critic, and what this does not show

- **F6 (stacked models).** L1+C1 is measured directly, and the interaction is a
  tested quantity, not an assumption.
- **Pricing labels (F2/F3).** "Counted lower bound" names what is unpriced.
  Nothing here is "conservative" or "complete".
- **Negative-binomial residual counts.** The design uses ratios of sums over
  targets, a cluster bootstrap, a power model with negative-binomial τ and
  between-target heterogeneity, and reports censoring.
- **F5 (no ranking of lower bounds).** The counted verdicts are effects on
  counted cost, not §5a rankings. The wall metric is reported separately and
  labelled INDICATIVE.
- **Not shown:**
  - any size other than p1009;
  - isolated timing;
  - an end-to-end pipeline cost: collection and linear algebra are shared and
    excluded;
  - a rho comparison;
  - a pcc-registered claim;
  - an S-pair-certified C1 in Amendment 13's sense (descent-walltime F2);
  - the learning RREF's true count.
- **Knowledge base.** `scripts/research-kb` has no executable on this host
  (2026-11-20), and no research-kb MCP tools are loaded. No literature,
  novelty or "not tried" claim is made.
