# D4 — Does auxiliary data help? (preregistration-grade reality-check)

Design-only. No experiment run, no measurement produced here. Grounding reads:
`docs/ic/LEADERBOARD.md` §"Multi-target IC against multi-target rho" (record
`benchmarks/20261112-mt-fieldops-v5/VALIDATION.json`), `docs/bounds/FRONTIER.md`
(oracle-domain rule), `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` (factor-base log DB,
`ic logs`), AGENTS.md §5a price band and "sabotage is assumed".

## Two readings of "auxiliary data"

### (A) Auxiliary-INPUT (Cheon / static-DH oracle) — does NOT help the plain goal

FRONTIER.md is explicit: *"A domain with an oracle is the DLP with that
auxiliary-input oracle, not the plain ECDLP … a method whose ops are lower only
because it was handed the oracle's answers does not displace one that was not."*
The oracle-queries column (static-DH queries per √r, **zero** for a method that
queries nothing) is itself an axis of dominance.

Consequence for ECC2K-130 / the single-target index-calculus goal: a Cheon-type
auxiliary-input attack lives in a **different domain** (`ecdlp.* · oracle=…`). It
can never displace `rho.signed_frobenius` or an IC bound on the plain
`ecdlp.single_target · koblitz` domain, because the two domains "share no curve"
in the dominance sense — they are not comparable, and the non-oracle method is
never dominated by the oracle-fed one. So reading (A) cannot advance the stated
program goal (improve the plain-ECDLP IC baseline, close the gap to rho).

**The one legitimate case.** If an *actual deployment* exposes a static-DH
oracle (fixed secret d, attacker may query [d]·(·)), then Cheon's auxiliary-input
DLP (knowledge of [d]P, [d²]P, …, i.e. the order ± 1 having a divisor near √r)
is in-scope — but as a **separate result against a separate domain**, scored on
its own `oracle queries` axis, and it must still beat the best *oracle-aware*
reference on that domain, not the plain-ECDLP rho. For ECC2K-130 as posed (a bare
point, no oracle), this case does not apply. It is worth a one-line note in the
threat model, not program budget.

### (B) Auxiliary data = cross-target precomputation — already what IC is, already charged

The factor-base logarithm database (`ic logs` precomputes it;
ECDLP_RESEARCH_TECHNIQUES.md "Individual logarithm") is exactly amortized
auxiliary data: pay relation collection + linear algebra **once** to build the
log DB, then each new target is a cheap individual-logarithm descent.

Where it is already charged: the ledger-§23 one-target pipeline charges the full
precomputation to the single target (cold). The multi-target row
(`benchmarks/20261112-mt-fieldops-v5`) is the regime where precomp is amortized
over T targets, **and it already exists and is validated**. Its verdict: against
the matched multi-target rho (cheapest of Kuhn–Struik retention rho and
distinguished-point rho), IC is **dearer at h=0, about tied at h=0.1, cheaper at
h=1 in every cell — every §5a decision is UNRANKED.** So "auxiliary data helps"
is, at the measured sizes, entirely an artifact of how you price the hash /
canonicalization ops. It is not a clean win and it is not new.

For a single **cold** target, amortized precomp does nothing beyond the known
|F|/throughput amortization that the cold accounting already applies: C_pre is
charged in full to T=1.

## The crossover model (the one preregisterable question)

Per-target cost as a function of target count T (same curve, same subgroup):

    IC_pt(T)  = C_pre / T + c_solve                 (precomp amortized + per-target descent)
    Rho_pt(T) ≈ κ · sqrt( r / (A · T) )             (batch/VW DL amortization: total ~ 2√(rT), per-target ~ √(r/T))

- `C_pre` = relation-collection + linear-algebra cost to build the full-rank log DB (dominant, ~ the whole IC cost).
- `c_solve` = per-target individual-log descent (≈ constant, a few decompositions + verify).
- `κ` = fitted batch-rho constant; `A` = usable automorphisms (signed Frobenius + negation).

IC beats batch rho iff  `C_pre + c_solve·T < κ·sqrt(r·T/A)`.
LHS is **linear** in T, RHS grows as **√T**. Therefore:

- **small T:** C_pre dominates → rho wins (IC precomp not yet amortized);
- **large T:** c_solve·T dominates √T → rho wins again (its per-target → 0, IC's → the constant floor c_solve);
- **a middle band exists at all iff the discriminant is positive:**

      κ²·r/A  >  4·c_solve·C_pre

  giving the band  T ∈ [T₋, T₊],  T± = ( (κ√(r/A) ± √(κ²r/A − 4 c_solve C_pre)) / (2 c_solve) )².

So IC-with-shared-log-DB can only win in a **bounded window** of T, and only if
`κ²r/A > 4 c_solve C_pre`. Crucially the discriminant must hold across the **whole
§5a band** h ∈ {0, 0.1, 1}, because both c_solve and κ carry unpriced hash /
canonicalization ops. The measured multi-target row says the order flips inside
that band → the discriminant does **not** hold uniformly at today's sizes.

### Cheapest test to locate T (no new large run)

1. Read C_pre, c_solve from existing IC records (N83 holdout `ic1_rows_*`, the
   toy-ladder cells) — already on disk.
2. Fit κ, A from the existing matched batch-rho references (retention /
   distinguished-point rho) in the same records.
3. Plug into the discriminant at h = 0, 0.1, 1. If `κ²r/A ≤ 4 c_solve C_pre` at
   any h → **no winning band at that price → drop.** If positive at all three h,
   report [T₋, T₊] and, only then, run one interleaved paired cell at a T inside
   the predicted band vs matched batch rho to confirm.

This is a day of reading + one calibration cell, not a campaign.

## Baselines and controls (if the test runs)

- **Reference:** matched **BATCH** rho on the **same T targets** (signed
  Frobenius + negation; cheapest of retention / distinguished-point), one
  process, replayed and verified — the existing multi-target reference. **Never**
  single-target rho run T times (see sabotage flag).
- A/A arm for the noise floor; interleaved arm order; masked labels.
- §5a band h ∈ {0, 0.1, 1}; report per-target IC/rho as the distance remaining,
  ranked only where the order holds across the band.

## Honest verdict and ranking

**Confirm the prior assessment: do not spend program budget here for the
single-target goal.** Reading (A) is a different domain that cannot advance the
plain-ECDLP baseline. Reading (B) already exists as a validated multi-target row
whose verdict is *unranked across the §5a band* — amortization is already
charged, and the "win" is purely a hash-price artifact. The crossover analysis
shows IC can only win in a bounded T-window that may not exist at all, and the
measured row indicates it does not exist cleanly today.

**Ranking:** D4 **lowest** of the four. D1 (transport graph, the only angle with
a structural reason to escape the equal-envelope control) > D2 (shared-x twist
twin, cheap and binary-Koblitz-specific) > D3 (MaxSAT selection, cheap, low
ceiling) > **D4**. The single salvageable deliverable from D4 is the discriminant
check above — cheap, falsifiable, and most likely returns "no band → drop."

## Sabotage flags (repo "sabotage is assumed")

1. **Accounting asymmetry — the headline trap.** Charging IC precomp once and
   amortizing it over T, while comparing against rho run *per target* (not
   batched), manufactures a spurious ~√T "win" entirely from the reference
   choice. Any D4 row must use **matched batch rho** on the identical T targets.
2. **§5a free-lunch.** Pricing the precomp's hash / canonicalization / LA ops at
   h=0 (treating native work as free) is exactly the trap the band is designed to
   expose; the multi-target row is unranked precisely because h=0 flatters IC.
   A single-h claim is unverified.
3. **Cold-vs-warm substitution.** Quoting a warm (log-DB-present) per-target cost
   as if it were a cold single-target cost silently drops C_pre. The ledger-§23
   cold pipeline must charge C_pre to T=1.
4. **Oracle smuggling.** Importing any value derived from a static-DH / known-log
   oracle into the plain-ECDLP arm (even a single known factor-base log beyond
   the public generator) moves the result into the oracle domain; FRONTIER's
   oracle-queries axis must then be non-zero and the comparison re-based.

Any of these would make D4 look like a win that the matched-reference, banded
accounting does not support.
