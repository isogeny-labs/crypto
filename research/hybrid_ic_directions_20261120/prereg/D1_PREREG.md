# D1 preregistration — Frobenius-transport relation graph (A* + frontier sharing)

**Status:** frozen-before-data draft. No data, no claim, `[pcc:unverified]`.
**Protocol:** [`D1_n31_sharing_null.protocol.json`](D1_n31_sharing_null.protocol.json) (`experiment.protocol/v1`, schema mirrored from `research/arity_completion_20261104/koblitz_m5_m6/protocol.json`).
**Design memo:** [`../D1_transport_graph.md`](../D1_transport_graph.md). **Synthesis:** [`../SYNTHESIS.md`](../SYNTHESIS.md). **Receipts:** [`../SOURCES.md`](../SOURCES.md).

## The question, in one line

Does ordering the decomposition search over a graph of canonicalised residual
points — with an algebraic A\* heuristic and small-norm `Z[τ]` multiplier edges —
cut forward node expansions ≥2× over uniform-cost batched MITM at matched yield,
and does the winning arm beat the **equal-envelope control** at all three §5a price
points, at `n=31` and `n=41` on K₀? And (separately) is cross-target frontier
**sharing** chance-level?

## What is being tested, honestly

Two of the three ideas that name this direction are preregistered to fail:

- **Sharing (`D1-SHARE`) is a preregistered NULL.** The forward-frontier overlap
  across a batch of `b=32` independent targets is predicted by the birthday rate
  `f_share = b·E·n/r` ≈ **0.18 at n=31** and ≈ **7.6·10⁻⁴ at n=41**. Sharing dies
  exponentially in `n`. The cheap `n=31` cell runs **first** to confirm this; it is
  where overlap would show if it existed anywhere.
- **The A\* heuristic (`D1-ASTAR`) is the one live sub-claim, at low confidence.**
  The heuristic (ONB Hamming weight of `x(ρ)` off the factor-base subspace) is the
  *exact* selector feature set that gave `SOLVER_SELECTION` **no routing headroom**
  on 288 queries. That prior is recorded as the main risk; the A\*-on vs A\*-off arm
  is the direct test of whether it carries.

The real, narrow question is therefore `D1-ASTAR` ∧ `D1-BEAT`. If both main effects
are null (`D1-COLLAPSE`), the hybrid reduces to **batched MITM + compact-orbit,
already on `main`**, and is recorded as a measured null so it is not reproposed.

## Design (2×2 factorial + the decisive control)

| arm | sharing | A\* | role |
|----|----|----|----|
| `A-S0H0` | off | off | baseline (= batched `mitm-frobenius` + CADO) / A/A anchor |
| `A-S0H1` | off | on | heuristic main effect |
| `A-S1H0` | on | off | sharing main effect (**n=31 cell = cheap gate, run first**) |
| `A-S1H1` | on | on | full hybrid / interaction |
| `C-equal-envelope` | off | off, **wider base `|F∪D|`** | **the control the candidate must beat** |
| `C-positive` | both | both, planted | witness-recovery control |
| `C-aa` | off | off, 2nd seed | noise floor |
| `R-rho` | — | `rho.signed_frobenius`, same process | matched reference (gap-to-rho) |

`C-equal-envelope` is the control that cut the double-large-prime graph from 23.8×
to **2.1×**: a win at fixed columns that loses to a wider base at matched budget is
**not** an improvement. D1 adds no columns, which is the one structural reason it
*might* survive this control where large-prime did not.

## Gates, thresholds, falsifiers

- **Phase 0 (cheap):** `n=31` `A-S1H0` vs `A-S0H0` + A/A. Confirm `f_share≈0.18`.
- **Phase 1 gate on the n=41 spend:** only commit the full `n=41` 2×2 + equal-envelope
  spend if the `n=31` A\*-on arm shows a point estimate ≥2× forward-expansion cut.
  Otherwise record `D1-ASTAR` not-supported at `n=31` and spend only on the `n=41`
  baseline + rho reference (for the gap record).
- **`D1-ASTAR` supported** iff A\*-on forward-expansions/relation ≤ 0.5× A\*-off at
  matched yield, 95% bootstrap CI excluding 1.0, at both rungs.
- **`D1-BEAT` supported** iff the best candidate beats `C-equal-envelope` on
  GAE/relation **and** cold one-target GAE at **all** of 0 / 0.1 / 1 GAE (§5a band).
- **`D1-COLLAPSE`** (verdict `reduces_to_known`) iff sharing null ∧ A\* not supported,
  arms within the A/A floor.

Every ratio is reported with **gap-to-rho** (best-arm GAE / matched-rho GAE) as the
distance remaining — never a pass/fail against rho. Counted unit: GAE, read across
the §5a band; wall time is diagnostic only.

## Scope limits (grounded, not optional)

- **No `n=131` path exists** (u64 `FastPoint` < `n=64`; `MAX_MODULUS_BITS=63`). The
  protocol tops out at `n≤41` toy rungs. No ECC2K-130 / exponent / sub-rho /
  leaderboard / `benchmarks/README.md` claim follows (`D1-OOS`).
- Expected best case: a constant-factor cut on the relation-search phase at one toy
  rung, partially absorbed by the equal-envelope control. **One rung, no exponent.**

## Prerequisite before anything runs

The candidate instrument **`examples/transport_graph_cell.rs` does not exist yet**.
It must implement the A\*/beam search over residual states, the shared-frontier mode,
the equal-envelope control, the planted control, and same-process
`rho.signed_frobenius`, all counting GAE identically, and be **sha256-frozen with a
recorded preflight before the freeze**. Then: research-kb/lead-atlas receipts →
Conductor task → `scripts/checkpoint.sh init` (S3 resume) → experiment-director
compiles a frozen task card → executor-mechanical runs it on isolab-eks (native L2).

This file and the protocol are design artifacts only. Nothing here authorizes a run.
