# Prime-field ECDLP: chase of the five open leads (2026-11-13)

**Standing.** Every verdict, number and argument in this note and its subdirectories is
`[pcc:unverified]`. No pcc claim was filed, no `bench.record/v1` was written, nothing reached a
leaderboard or the validation gate, and nothing here is a cost, security or speed result. This is a
record of exploratory toy/desk work.

**Task.** The owner's question, relayed as "chase all leads": the best non-generic, sub-√p (ideally
subexponential) leads for ECDLP over prime fields, from
[`../notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md`](../notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md)
and [`../../RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`](../../RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md).

**How the work ran.** Five parallel subagent labs (L1–L5), each freezing a `PROTOCOL.md` (hash in
`PROTOCOL.sha256`) with predictions and a KILL/SURVIVE rule before any run. L1 and L3 ran their
large cells on a RunPod CPU pod (32 vCPU / 256 GB, msolve from the Ubuntu archive); L2, L4, L5 ran
locally. L3 and L4's new proofs were each re-checked by an independent adversarial reviewer
(`L3-check/`, `L4-check/`). research-kb and the lead atlas were unavailable in the container; the
labs grepped the repositories instead. The account hit its weekly model limit mid-run and the L1
agent terminated; its detached pod jobs continued and the main session drove them to the end.

## Headline

**No lead produced a sub-√p prime-field ECDLP method `[pcc:unverified]`.** Four of the five are
closed at toy/desk scale; the fifth (PKM tower at m = 8) is inconclusive and leans to closure. The
whole question now narrows to one quantity: whether the PKM / digit-presentation solving degree keeps
growing fast enough at m ≥ 8 `[pcc:unverified]`.

| Lead | Sub-dir | Verdict `[pcc:unverified]` | Distance to matched rho (exponent 1/2) |
|---|---|---|---|
| L1 PKM Kummer tower, aux. unknowns | `L1-pkm/` | m ≤ 7 not viable; **m = 8 inconclusive, leaning closure** | hinges on D(8,3)=9 (E≈0.43, would beat) vs ≥10 (E≥0.65, worse) |
| L2 small roots beyond Coppersmith | `L2-lattice/` | not viable at desk/toy scale; 9 ideas, none moves the exponent | S3 reach δ≈0.1–0.16 → IC exponent 0.84–0.90 (+0.34–0.40) |
| L3 digit presentation ("lever 2") | `L3-digit/` | not viable at toy scale; a new solving-degree lower bound | E₄ ≥ 0.70 (ω=1); open only at m ≥ 8 / >10⁴-bit primes |
| L4 lifting with growing rank | `L4-lifting/` | no attack; a new per-trial bound (scope narrowed on review) | no finite gap quotable; route not closed |
| L5 two-isogeny fibre base | `L5-fibre/` | a relabelling of IC on the 2-isogenous curve (row A11) | (m−1)/(2m−3) > 1/2 always |

## L1 — PKM tower, m ≥ 8 (`L1-pkm/`)

Certified polynomial-calculus / solving degrees D(m,t) for the Kummer tower, each by an exact sparse
closure (`1 ∉ W_{D−1}`), cross-checked by a second implementation, a second seed (9/9), a planted
control, and msolve as an upper bound `[pcc:unverified]`:

| m | t=1 | t=2 | t=3 | t=4 | t=5 |
|---|---|---|---|---|---|
| 4 | 5 | 6 | 7 | ≥7 | ≥8 |
| 5 | 5 | 6 | ≥8 | | |
| 6 | 6 | 7 | ≥8 | | |
| 7 | 6 | ≥8 | | | |
| 8 | 7 | 8 (exact) | ≥9 (see below) | | |

**Decisive cell D(8,3)** (p = 16801793, 43.7M-column degree-9 system; `D83_VERDICT.md`,
`decisive/`): the exact closure certified `1 ∉ W_8` → **D(8,3) ≥ 9**; its degree-9 level did not
finish (censored). msolve cleared all of degree 9 and advanced into a degree-10 round before
crashing at the memory cap → **uncertified evidence D(8,3) ≥ 10**. Under amendment A3 this is
**INCONCLUSIVE, leaning KILL** `[pcc:unverified]`: m = 8 beats rho only if D(8,3) = 9 exactly
(σ = 0.125 < α*(8,1) = 0.156, E ≈ 0.43); D(8,3) ≥ 10 gives σ ≥ 0.25 and E ≥ 0.65. Certifying it
needs ~140 GB of closure or a completed large-memory msolve run — an EKS cell, not this pod.

**Theory (`THEORY.md`, paper only):** with all but one block fixed, the solving degree is bounded
independent of t, so one-free-block (Lemma-X-type) arguments cannot give a t-growing lower bound
`[pcc:unverified]`. No t-growing lower bound is proved — the single missing theorem behind the whole
PKM question.

## L2 — small roots beyond Coppersmith (`L2-lattice/`)

Nine structurally distinct ideas (several shared-root equations incl. a "line/function through R"
parametrisation; many targets in one lattice; other coordinate models; 2-adic/Hensel/ACD; amortised
basis; S3 chains; large-prime; Vallée generation). A 405-cell instrument (`nu.py`) that lower-bounds
the reach of any such lattice found none below the plain-S3 value; a planted control fired and
random coefficients on the same supports reproduced it, so curve structure contributes nothing
`[pcc:unverified]`. S4 is 1.5× short at δ = 1/4, S5 is 1.9–2.0× short at δ = 1/5. No fpylll run was
triggered — the protocol's gate never opened. Not viable `[pcc:unverified]`.

## L3 — digit presentation (`L3-digit/`, checked in `L3-check/`)

Solving degree D grows linearly in the digit count k for Boolean, base-3, base-4 and mixed-radix
presentations; msolve's max F4 degree matched the certified D on 34/34 paired systems; digits are
worse than the one-variable base, not better `[pcc:unverified]`. **New lower bound (Theorem F,
`THEORY.md`):** below the first-fall degree the closure adds nothing usable, giving
D ≥ ffd = m·2^{m−1} + ⌊(k−2^{m−1})/2⌋ + 1 for the unsplit presentation, met with equality on all 166
certified records `[pcc:unverified]`. It settles hypothesis G clause (ii) for these presentations
(7,835/7,835 targets). **Checker (`L3-check/CHECK.md`): sound with gaps** — the bound is on the
unsplit axiom's degree, not on solver cost or the decomposition problem (the split S3 chain solves
lower: 8 vs 13 at m = 3); the corrected m = 4 exponent is ≥ 0.79, not 0.70; the asymptotic crossover
is m ≥ 8, not m ≥ 6. Not viable at m = 4; open at m ≥ 8 `[pcc:unverified]`.

## L4 — lifting with growing rank (`L4-lifting/`, checked in `L4-check/`)

No attack. A new short-relation bound (Theorem X, `PROOF.md`, paper only): per-trial success ≤
|M|/q, independent of lift construction, number-field degree and height normalisation, which closes
the closure note's Prop 3 sketch `[pcc:unverified]`. The xedni bound stops excluding rho-beating
rates at rank ≈ √(log p / log log p); the fixed-lift (Silverman–Suzuki) bound at ≈ log p / log ρ.
**Checker (`L4-check/CHECK.md`): sound with gaps** — the headline "ranks 7–8 excluded at 256 bits"
is unsupported, because the height ratios were measured only on lifts that carry no relation; the
number-field step is Q-only. Growing-rank lifting stays open, blocked in practice by an unproved
step (no known fast way to find a small-height preimage of the reduction map) `[pcc:unverified]`.

## L5 — two-isogeny fibre base (`L5-fibre/`)

Exact relabelling: a sum in the fibre base F on E equals a sum in an x-coordinate base S′ on the
2-isogenous curve E′ (N_m^{E,F}(R) = N_m^{E′,S′}(φ(R))); full 2:1 fibres give 2^{m−1}·N′, i.e.
closure-note row A11 `[pcc:unverified]`. Measured partial-sum widths match random bases to within
0.02 at ≥18 bits (a structured positive control is caught at 0.012), so the summation message stays
generic. Best achievable is an explicit (m−1)-sum table, exponent (m−1)/(2m−3) > 1/2 `[pcc:unverified]`.

## What is NOT shown

- No sub-√p method; no proof that index calculus (or lifting) cannot beat rho `[pcc:unverified]`.
- **D(8,3) is not certified** (9 vs ≥10) — the one open numerical question, needing a large-memory host.
- L1's t-growing solving-degree lower bound, and L3's bound for the *split* chain, are unproved.
- L4's exclusion holds only for the quantity measured (unsuccessful-lift height ratios), not for
  successful lifts; growing rank and r ≥ 7–9 at 256 bits stay open.
- All proofs are paper-only except where a `MANIFEST.sha256` names a machine-checked part; cited
  theorems were recalled, not re-read; research-kb novelty checks were not run.

## Next action (single, if compute allows)

Certify D(8,3) on a ≥140 GB host (EKS cell): degree-9 closure to completion, or a finished msolve
run, deciding m = 8. Everything else here is closed or desk-bound.

## Evidence index

Per-lab `PROTOCOL.md` (+ `.sha256`), `AMENDMENTS.md`, `MANIFEST.sha256`; reports in `REPORT.md`
(L2, L4, L5) or, where the harness blocked the file, transcribed into this note and the lab's
`THEORY.md` / `D83_VERDICT.md` (L1, L3). Checker reports in `L3-check/CHECK.md`, `L4-check/CHECK.md`.
Large solver inputs, compiled binaries and cargo targets were left in the session scratchpad and are
not committed.
