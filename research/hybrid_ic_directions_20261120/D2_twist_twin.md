# D2 — Shared-x twist twin collector for binary Koblitz curves

**Status:** preregistration-grade design. No measurements. Design-only.

> **Prior-art reconciliation (see `SOURCES.md`):** the binary **m=2** case of this
> idea was **already run** in `research/twist_ecdlp_20261027/E4-koblitz-twist-pair/`
> (verdict `prediction_confirmed`: "shared base is a pure cost increase, accounting
> not a gain"). Only **m≥3** is unmeasured. This memo's design is retained for the
> m≥3 question and the field-op-sharing framing; it is not an untried lead.
**Date:** 2026-11-20. Novelty **unverified** (no literature search run from this fork).
**Grounding read:** `src/cryptanalysis/binary_semaev.rs` (S₃ definition and its
documented `a`-independence), `research/twist_index_calculus_20261020` (the measured
twist study whose arms produced "no priced row moves").

---

## 0. The fact that reframes the whole idea

The parent's framing assumed the twin's win is "one x-only solve yields a candidate
for both curves." The repo's own binary Semaev module settles the algebra and makes
the premise both **stronger and narrower** than stated:

For `E_a : y² + xy = x³ + a x² + b` over `F_{2^m}`, three points sum to `O` iff their
abscissae are the three roots of
`x³ + (λ²+λ+a)x² + μx + (μ²+b) = 0`, giving
`S₃(x₁,x₂,x₃) = (x₁+x₂)²x₃² + x₁x₂x₃ + (x₁x₂)² + b`.

**`S₃` is identical for `a=0` and `a=1`** (binary_semaev.rs:36–39). The entire
`a`-dependence is the Artin–Schreier side condition `Tr(s₁ + a) = 0` for the slope
`λ` (`t²+t = s₁+a`), plus the single-point membership test `x ∈ E_a ⇔ Tr(x + a + 1/x²)=0`.

So the "two curves need genuinely different Semaev systems" risk (item 9) is **refuted
by construction**: there is only *one* polynomial system. The arithmetic is not merely
*shareable* — it is *the same object*. The honest consequence, developed below, is that
this removes the leverage rather than creating it: the only thing that differs between
the twins is **one trace bit**, which is free, and a disjoint factor-base membership,
which is the real cost driver.

## 1. Exact algebra: shared vs duplicated

Let `m` be odd (ECC2K-130: `m=131`). `b=1` for both. `E_0` (a=0) and `E_1` (a=1) are
the two curves; `E_1` is the quadratic twist of `E_0`.

- **x-line partition.** For `x≠0`, `x` is an abscissa on `E_a` iff `Tr(x + a + 1/x²)=0`.
  Since `Tr(1)=1` for odd `m`, `Tr(x+1/x²)` and `Tr(x+1+1/x²)` differ by 1, so **every
  `x` lies on exactly one of `E_0, E_1`** (the finite affine x-line splits in half;
  `x=0` and the 2-torsion handled separately). The two factor bases are **disjoint**.
- **Shared field ops (amortized if a single unrestricted search is run):**
  evaluation of `S₃`/`S₄` monomials, the pair-table / MITM key arithmetic over
  x-coordinates, resultant / root-finding structure, F4/Macaulay rows — all
  `a`-independent, hence byte-identical for the twins.
- **Duplicated / per-curve (cheap):** the trace side-condition `Tr(s₁+a)` (one
  conditional trace, ~`m` squarings via the half-trace `H`, but the repo caches `H`),
  the membership classification of each x (one trace), and the **group lift/verify**
  of an accepted relation (done in the curve group — genuinely separate, but only on
  accepted rows).

**Net:** the *expensive* stage (the polynomial relation search) is literally one
computation; the *per-curve* part is O(trace bits) on candidates plus group lifts on
the (few) accepted rows.

## 2. Exact falsifiable claim (relations-per-field-op)

Define `Y_pair = (R_0 + R_1) / W`, where `R_a` = verified relations collected on `E_a`,
`W` = total counted field ops (muls+sqrs+invs behind GAE). Claim:

> **C2.** Running one unrestricted `a`-independent decomposition search over the *full*
> x-line and dispatching each `S=0` solution to whichever curve admits all its abscissae
> yields `Y_pair` strictly greater (beyond A/A noise) than the equal-field-op control that
> runs two single-curve searches, **because the dominant polynomial-solve ops are counted
> once instead of twice.**

**Quantitative prediction (the honest arithmetic that most likely sinks it).** A random
`S₃=0` solution has 3 abscissae, each independently ~½ on `E_0`; it is a *usable* relation
for `E_0` only if all 3 are `E_0`-admissible (prob `≈(1/2)³=1/8`), likewise `1/8` for `E_1`,
so **≈3/4 of raw solutions are mixed and wasted.** The shared search therefore pays ~1 solve
but discards 3/4 of its output; the two restricted searches pay ~2 solves with **no** waste
because each pre-filters its base to one curve. Predicted shared-arithmetic fraction of a
solve that is `a`-independent: `f ≳ 0.95` (only the trace side-condition is not). Net pair
speed-up prediction: `Y_pair^share / Y_pair^control ≈ 1 / (useful-fraction penalty)`. With
`k=3` the waste penalty (4/3 extra solve attempts to recover the same usable yield) roughly
cancels the 2× sharing, so **predicted net ≈ 1.0–1.3×**, and **< 1 is plausible**.

## 3. Cheapest discriminating test

Toy binary Koblitz twin pair at a rung where both subgroups are tractable (e.g. a
`GF(2^n)` Koblitz pair at `n≈23,31`), plus a `GF(2^83)` a=0/a=1 confirmation cell for the
membership-partition fact only. Counted unit: field muls/sqrs/invs behind GAE; report at
`h = 0, 0.1, 1` GAE per unpriced native op (§5a band). Instrument = the existing
`binary_semaev` S₃ pair-table collector, one build, one host, interleaved arms, masked
labels.

Arms (four, interleaved, derived seeds):
- **S** (shared): one unrestricted full-x-line search; dispatch each solution to E₀/E₁/mixed; count `R_0, R_1, W`.
- **C** (control, the one that must be beaten): two restricted single-curve searches, base pre-filtered per curve, at **equal total `W`**.
- **A/A**: two seeds of C for the noise floor.
- **ρ**: `rho.signed_frobenius` on each curve's subgroup, same process, same unit — report `Y_pair` gap to each matched rho (distance remaining), never pass/fail.

## 4. Baselines and references

- Primary control **C** (equal field-op envelope) — the arm that killed large-prime (§1 of
  `IC_TECHNIQUE_INTEGRATION_20261012.md`). S must beat C, not plain per-curve IC.
- A/A noise floor.
- Matched rho **per curve** (never one rho for the pair — see confounder b).
- Gap to rho reported for each curve separately and for the pair aggregate.

## 5. Confounders and variable ledger

| # | Confounder | Control |
|---|---|---|
| a | Gain is really "more columns" | base sizes frozen equal across S and C; S adds **no** columns, only removes pre-filtering; verify column counts identical |
| b | Two independent subgroups counted as one DLP | each curve's relations solve **that curve's** DLP only; two separate rho references; `Y_pair` is an explicit *pair* metric, never claimed as single-target speed-up |
| c | Orbit/Frobenius savings are generic (the measured-study failure mode) | subtract the generic signed-Frobenius orbit baseline already in `main`; the only credited delta is the shared polynomial solve |
| d | CSE a compiler already does | C must be the *restricted* two-search baseline the pipeline actually runs, not two naive un-CSE'd solves |
| e | Mixed-solution bookkeeping hides cost | count every discarded mixed solution's ops in `W` for arm S |

Ledger fields: instrument build hash, host, base spec, seed derivation, arm order,
curve pair, rung, analyst-masked labels.

## 6. Falsification threshold

Reject C2 if either holds at both rungs:
- `Y_pair^S / Y_pair^C ≤ 1 + 2·(A/A half-width)` (no separation beyond noise), **or**
- the mixed-solution waste makes arm S's usable yield per `W` ≤ arm C's.

## 7. Expected gain and honest ceiling

Binary-Koblitz-only; constant factor; **pair metric only.** Best plausible: ~1.3× on
`Y_pair`. Realistic: ~1.0×, possibly < 1 once mixed-solution waste is charged. **No
exponent change; no help to a single ECC2K-130 target** (for one curve you simply use its
smaller pre-filtered base — the sharing has nothing to amortize). This is a *multi-instance
across the twist pair* method, structurally in the same restricted regime as D4
(amortized precomputation): it pays only when DLP is genuinely wanted on **both** `E_0`
and `E_1`.

## 8. Novelty vs prior art

- vs `twist_index_calculus_20261020` (measured): that study used the twist as **extra
  orbit columns on one curve** and priced it as generic. The twin collector is the
  opposite move — **one shared `a`-independent search feeding two disjoint bases** — and
  reports a *pair* throughput, not a single-curve column gain. **Not measured.**
- vs the `GF(2^83)` a=0/a=1 check: that check tests the membership/partition fact; it does
  **not** run a shared collector or price `Y_pair`. **Not measured.**
- The credited mechanism (S₃ `a`-independence ⇒ one polynomial search for the twin pair)
  is documented in `binary_semaev.rs` but has **never been turned into a dual-dispatch
  collector or measured against the equal-envelope control.**

## 9. Sharpest risk

Not that the curves need *different* systems — they need the **same** system
(`S₃` is `a`-independent). The real risk is that this very sameness means **there is no
two-system cost to amortize for a single instance**, and for the pair the mixed-solution
waste (≈3/4 of S₃ solutions) plus the rarity of needing both twins' DLPs makes the win
situational and small. Expected outcome: a **clean, publishable negative/near-null
result** that converts the twist lever from "maybe" to "measured ≈1× on a pair metric,
nothing for single-target," which is itself worth banking. One scenario escapes the
partition — **even `m`**, where `Tr(1)=0` and the twins share x-coordinates — but
ECC2K-130 has odd `m`, so it does not help the target; and the `F_{2^{2m}}` trace-
decomposition view (twist relations as a Frobenius `(-1)`-eigenspace in `E_0(F_{2^{2m}})`)
is a different, GHS-flavored method, out of scope here.
