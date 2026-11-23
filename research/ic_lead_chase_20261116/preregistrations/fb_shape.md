# experiment.protocol/v1 (preregistration) — GP factor-base total-cost measurement

leadkey: fb_shape
lead: trace-zero geometric-progression (GP) factor bases
evidence_dir: research/factor_base_shape_leads_20261018
author: parallel-sweep executor (re-check session, 2026-11-16)
status: PREREGISTERED — gated; NOT runnable in this environment (no ecbench/isolab/S3)

## Question
Does the higher equal-size decomposition YIELD of trace-zero GP x-bases translate
into a lower TOTAL counted cost (relation collection + linear algebra) for a complete
index-calculus DLP solve, versus the Frobenius-orbit incumbent base and versus matched
Pollard rho — given that the observed solving degree is unchanged (=3) and the single
emitted F4 counter already shows GP is MORE expensive per relation on the deg-13 fixture?

## Why this is the decisive missing measurement
The committed evidence measures only: coverage (exact census), holdout yield, train rank,
and a single INCOMPLETE cost component (F4 field-reduction multiplications). It records
`total_cost_winner: null`, `full_pipeline_speedup: null`, and
`pricing: "Incomplete on all arms; no symmetric-band total ranking"`. No relation-phase +
LA total, no GAE accounting, no matched rho. A yield gain is only a constant-factor gain in
ONE sub-phase and can be erased by (a) higher per-trial reduction cost and (b) LA cost.

## Arms (identical instrument = ecbench ic.pipeline on every arm; interleaved order; masked labels)
- A_coord   : coordinate / binary-subspace base (incumbent control)
- A_frob    : Frobenius-orbit base (the stated incumbent to beat for Koblitz/subfield)
- A_gp      : trace-zero GP base, seeds {17,29,43}, matched dimension
- A_rho     : matched Pollard rho reference on the same curve/subgroup (AGENTS.md §5a band)
- A_A       : A/A null arm (A_coord run twice, independent derived seeds) for the noise floor
Factor-base COLUMN COUNT (not signed-point count) matched across A_coord/A_frob/A_gp to
within +/-1 column; record the exact column count per arm (GP must not win by being larger —
cf. seed 43 at 21 vs 16 columns).

## Curves (cold, public, preregistered; escalate, do not cherry-pick)
1. icv1-f2m13-t181-515ee569 (deg 13, r=2003) — toy, reproduces existing fixture end-to-end.
2. icv1-f2m31-tm90707-c95f16f5 (deg 31, r=1439393) — intermediate.
3. GF(2^83) Koblitz target (deg 83) — the real subfield/Koblitz target where the recorded
   gap to rho is ~1.3x. This is the arm that settles the lead. Requires native L2/replay/A-A
   admission per CLAUDE.md.

## Counted unit + pricing band (AGENTS.md §5a)
Unit: GAE (group-arithmetic-equivalent) over the WHOLE pipeline:
  relation phase (point generation + Semaev/F4 reduction, counting field mul/sqr/inv behind
  each group op) + linear-algebra phase (sparse solve over F_r, GAE-costed).
Price every arm's unpriced native work, rho included, at 0, 0.1, and 1 GAE per operation.
Rank two arms ONLY where the order holds across all three price points (§5a).
Report each arm's gap to the matched rho as distance remaining, never pass/fail.

## Metrics (per arm, per curve)
- total_GAE_relation, total_GAE_LA, total_GAE (sum), at each price point
- relations_collected, trials_needed, columns, matrix dims
- observed solving degree distribution (confirm whether still 3)
- complete-solve certificate (recovered discrete log verified) — required for a full-cost row

## Falsification criteria (preregistered, symmetric)
- H_lead CONFIRMED (tier toward T0 gap-narrowing) iff total_GAE(A_gp) < total_GAE(A_frob)
  AND < total_GAE(A_coord) at ALL three price points, with a complete-solve certificate,
  and the gap to matched rho strictly narrows vs the incumbent arm.
- H_lead REJECTED iff total_GAE(A_gp) >= total_GAE(incumbent) at any price point, OR GP needs
  >= the same solving degree AND higher per-relation GAE such that the yield gain is erased.
- INCONCLUSIVE iff A_A noise floor exceeds the A_gp-vs-incumbent gap.
Prior from this re-check: per-relation F4 cost ratio GP/coord = 1.55-1.66x on curve 1 already
points to REJECTION unless the LA-phase saving from fewer columns dominates; the protocol must
emit the LA term to decide.

## Seeds
Base construction seeds {17,29,43} (as recorded). Target/probe seed 2026101802 (as recorded).
Arm-order and A/A seeds derived by SHA256(protocol_id || arm || curve || "order"/"aa").

## Admission / provenance
isolab-eks default route; verify tested commit, image identity, S3 result hashes; native
L2/replay/A-A admission for the deg-83 arm; resume via CRIU/EBS contract. Validation gate:
VALIDATION.json with replay certificate + validator report + matched-rho (§5a band) before any
leaderboard or benchmarks/ row.
