# Cost-per-relation reduction: grounded ledger, a corrected invariance argument, and the live lever

Date 2026-11-23. Desk analysis, nothing run. All `[pcc:unverified]`; figures are
quoted from named records, not re-measured. research-kb unavailable in-container;
grounding by direct reads. Independently verified (Sonnet Explore) and
adversarially red-teamed (Opus); corrections from both are folded in and the
places this memo was wrong in its first draft are kept visible.

## 0. The quantity (corrected attribution)

End-to-end IC cost is dominated by relation COLLECTION. The exact share depends
on the study and setup — it is **not** a single clean number:
- 84–89% is *oracle setup* share of wall in the `ic_linalg_pipeline_20261019`
  benchmark (`docs/ic/ECDLP_RESEARCH_TECHNIQUES.md:1151-1154`);
- 0.11–1.9% is *linear-algebra* share of a real `ic logs` run
  (`research/la_engines_20261014/`);
- other records put collection at 90% / ~95% / 97–99% (`canon_laned_keys_20261008`,
  `ecbench_rho_lanes_20261023`, `session-logs/2026-11-14`).
Honest statement: **collection is 84–99% of cost; LA is a small single-digit %.**
The three multiplicative factors of collection cost:

    C_collect ≈ |F| × (1/yield) × (cost per attempt)

## 1. Yield — a hard invariant (stronger than "capped")

**KN-FIND-007 (established, Lean-checked `research/factor_base_designs_20261015/lean/`;
`docs/ic/factor-base-encyclopedia/README.md:631-637`).** `Σ_r c_D(r) = C(B+m−1,m)`
is a **double-count identity**: mean decomposition yield over *uniform* residuals
is **exactly** base-geometry-invariant (measured ratio 1.000000, 288 cells).
- The **1.582** figure is the **coverage** (fraction-of-targets-hit) headroom,
  **not** a yield multiplier. A Sidon B₃ base reaches coverage 1.107.
  [First-draft error, now fixed: I wrote "yield capped ≤1.582×"; mean yield is
  exactly fixed, and 1.582 bounds coverage.]
- Orbit bases are **yield-neutral** (cyclotomic falsifier, `research/cyclotomic_falsifier_20261015/`).
- Redistribution toward *concentration* is possible but trades coverage: a `{kP}`
  base reaches max `B^{m−1}` on some targets while lowering coverage
  (`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md:333-338`).

So the Sidon / sumset-coverage / additive-energy family cannot raise mean yield at
all, and can raise coverage by at most 1.582× (measured ~1.0–1.1×).

### 1a. The redistribution escape — corrected statement (red-teamed)

KN-FIND-007 bounds only the mean over *uniform* residuals; it says nothing about a
*biased* sampler. The question: can you sample residuals `R=[a]G+[b]Q` with **known**
`(a,b)`, cheaply and with `o(|mD|)` memory, so independent hits arrive at a rate
above the uniform baseline? First draft asserted a "Lemma (MITM is the maximal exploitation)".
The red-team **broke the literal statement** and salvaged the operational core.
Corrected:

**Claim (conditional, independent-relations metric).** Conditional on ECDLP
hardness, and measured in **rank-contributing (independent) relations** — not raw
membership — no cheap `o(|mD|)`-memory sampler of known-coefficient residuals
raises the independent-hit rate above the uniform baseline by a **super-constant**
factor. The structural
reason (the real content): a move that keeps coefficients known, `[Δa]G+[Δb]Q`,
re-hits `mD` only at the uniform rate; the **only** cheap maps that are *both*
log-known *and* `mD`-preserving are **Frobenius (scalar λ) and negation (−1)**, and
these produce the constant **|Aut|-fold** of linearly *dependent* rows — exactly the
yield-neutral automorphism fold the repo already measures. Any other `mD`-preserving
move needs an unknown base-point log (≥√r to obtain).

Three corrections this bakes in (from the red-team, `REDTEAM.md`):
1. The old "a biased sampler would be a decomposition oracle" is a tautology, not a
   proof; the real claim is conditional on ECDLP-hardness and only excludes
   *super-constant* bias.
2. **MITM is orthogonal** — it is per-attempt *check-cost*, not a redistribution
   exploit (it leaves the hit probability `|2D|/r` unchanged). Dropping the
   "MITM-maximal-exploitation" phrasing.
3. The literal "membership probability" version is **false**: the Frobenius/negation
   orbit `{±λ^j R₀}` of one hit is in `mD` with known coefficients at O(1) memory —
   it just yields *dependent* rows, which is why the metric must be independent
   relations.

Candidates that all **support** the corrected claim (each fails ≥1 bar): small-norm
Z[τ]/lattice `(a,b)` (pseudo-random map, no bias); near-miss/base-swap navigation
(loses known `(a,b)` without a ≥√r base-point log); kangaroo/DP (uniform-rate walk);
Wagner/generalized-birthday (memory ≫ `|mD|`); partial-join/slope buckets (measured
to decay to ambient density, `single_relation_reduction_20261115`).

Status: this corrected claim is **conjecture, not theorem** — but it is a genuine
sharpening of KN-FIND-007 (it closes the biased-sampler escape on the
independent-relations metric), and the red-team could not break its operational
core. Worth a Lean-style write-up IF pursued; not a speed claim.

## 2. Cost per attempt — toy-measured, unrun at real sizes (corrected)

The root-finding thread owns this (`rootfind_baseline_100x_20261117`).
- The **~10×** prototype figure is a **probe-throughput ratio on a 32-bit *prime*
  toy, mostly refutations, `[pcc:unverified]`, one unpinned VM, no replay**; it
  excludes table-build and witness recovery, and handles only primes <32 bits. It is
  **not** a cost-per-relation or end-to-end number. [First draft called it a
  cost-per-relation win "toward the real ECC2K-130 oracle" — both overstated.]
- The **binary port** (`rootfind_negfold_binary_batch_20261115`) is an **n=13
  Koblitz toy**, not ECC2K-130; it is fully specified (7 before-data amendments) but
  **unrun**, blocked by the lab **monthly budget being over limit** ($1,982 vs
  $1,300 as of 2026-11-15, now stale — recheck) plus L2/replay admission gates.
- The binary oracle **already batches** target additions ("more than half its
  per-point time is the probe", RESULTS L64-67), so the prime prototype's
  inverse-batching gains do **not** transfer; the open binary component is the
  still-serial **table build** (86–122 ns/pair vs 11–12 batched).

Net: per-attempt is engineering, demonstrated only on a prime toy, and its binary
realization is designed-but-unrun and budget-blocked.

## 3. Exponent (m / relation graph) — harvested
Double-large-prime graph 2.1× over the equal-envelope control, higher-arity 6-sum
break-even (`IC_TECHNIQUE_INTEGRATION_20261012.md §1`; not re-verified here).
Constant factors, one rung, no exponent.

## 4. Verdict and the live lever

**Cost-per-relation's two tunable factors are closed or claimed:**
- yield is an exact invariant (KN-FIND-007), and its biased-sampler escape is closed
  on the independent-relations metric (§1a, conjecture, red-teamed);
- per-attempt is engineering, toy-measured, unrun at real sizes.

**The live adjacent lever the repo itself points to is NOT cost-per-relation — it is
the SOLVING DEGREE of the algebraic presentation.** KN-FIND-007 "closes proposals
that rely on a table oracle" (`factor-base-encyclopedia/README.md:1567-1570`), but a
**tower / auxiliary-variable presentation escapes the Bézout bound**
(`:1587-1589`) — the PKM-tower and digit-presentation leads. That changes the
*solving degree* (and hence the real cost of a hit), which the yield identity does
not govern. This is where the repo locates the remaining room, and it is a different
research question from cost-per-relation.

**Ranked options:**
1. **Pivot to solving-degree / tower-presentation leads** — the one direction with
   un-closed structural room per the repo's own findings. Desk-designable next.
2. **Execute the designed binary per-attempt port** when budget reopens — engineering
   win, one rung, needs EKS budget + L2/replay, no new idea.
3. **Desk note now (no run):** write up the corrected §1a as a conjecture that
   sharpens KN-FIND-007. Strengthens an established finding; no speed claim.

Scientific headline: **cost-per-relation is closed — yield by an exact invariance
theorem (whose biased-sampler escape reduces to the constant automorphism fold),
per-attempt by toy-measured engineering — and the repo's own open door is
solving-degree reduction via tower presentations, not relation-cost tuning.**
