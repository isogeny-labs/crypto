# experiment.protocol/v1-style note — volcano ell=2 floor IC-yield: settle-or-drop
Lead: volcano_floor (research/isogeny-volcano-cost-v2_20261017)
Author: parallel-sweep executor. Status: PREREGISTERED (pre-data). Produced-side [pcc:unverified].
Date: 2026-11-16. Scope: GATED isolab measurement; NOT run here (no isolab/S3/EKS).

## Decision this settles
The v2.1 study found: ell=2 FLOOR gives 5.8% [4.4,7.2] fewer IC relations PER TRIAL
than the crater, and IC cost S is 4.5% [2.3,6.7] HIGHER on the floor (floor/crater
S = 1.045). Reframed as a strategy: given a floor-curve target, walk UP the ell=2
volcano (depth 3) and run IC on a crater curve for a ~4.5% S saving.
The study did NOT charge the walk cost, nor the cost of orienting the volcano
(deciding a given curve is on an ell=2 floor and which neighbour ascends). The
mechanism of the per-trial deficit is unidentified: the ONE pre-registered
mechanism hypothesis (ell-power torsion changes between levels) was falsified by
l3-s1011 (ell=3 analogue showed NO deficit; rel/trial floor/crater = 1.0185).

Question: once the full floor->crater exploitation cost is charged end-to-end and
counted in the same unit as the matched rho, is the net IC-S on the floor-target
still strictly lower than running IC in place, by a margin whose CI excludes 0,
and does it persist beyond 24-bit? If NO -> DROP (log closed). If YES -> it is a
charged constant-factor IC-baseline diagnostic, still 17-170x from rho.

## Arms (one counted unit: ecbench GAE S; all arms same binary/host, interleaved)
A0 (baseline): IC (ic.pipeline, prime-abscissa size=32, mitm oracle) run IN PLACE
    on the given floor target curve. Counted: full IC S.
A1 (candidate): orient + ascend + solve. Counted end-to-end, no free steps:
    (a) volcano orientation: modular-poly Phi_2 root-find + depth probe to decide
        the curve is on an ell=2 floor and pick the ascending 2-isogeny (charged);
    (b) depth-3 ell=2 ascending walk by Vélu, pushing G and Q each step (charged);
    (c) IC on the reached crater curve (charged), + transport the recovered log back.
A_rho (reference): rho.negation on the SAME target, same binary — matched-rho.
A/A: A0 vs A0 on model 0, for the noise floor.
Positive control: planted IC factor-base size 33 vs 32 (predicted ~1.063), as v2.1.

## Matched-rho reference & pricing band (AGENTS.md §5a)
Counted unit = ecbench GAE S. Price every arm's unpriced native work (orientation
field ops, Vélu muls/sqrs/invs, IC canonicalisations/sqrts/lookups/row-ops, rho
canonicalisations) at w ∈ {0, 0.1, 1} GAE/op. Report A0/A_rho and A1/A_rho at all
three weights; rank A0 vs A1 ONLY where the sign of (A0 - A1) holds across the
whole 0/0.1/1 band (§5a). A1 "wins" only if A1_S < A0_S with the paired-bootstrap
95% CI of (A0 - A1)/A0 excluding 0 at ALL three weights.

## Counted unit, falsification, seeds
- Primary statistic: net S ratio A1/A0, class-stratified, paired per target.
- FALSIFY (drop the lead) if ANY holds:
   (i) A1/A0 95% CI includes 1.0 at any weight in {0,0.1,1}; or
   (ii) the orientation+walk charge (a)+(b) alone >= the in-place 4.5% S margin
        (i.e. charging the walk erases the gain); or
   (iii) the net advantage shrinks monotonically with bit-size across {24,28,32}
        (consistent with a 24-bit artifact / vanishing constant factor).
- CONFIRM (bank as charged IC-baseline diagnostic, NOT a rho result) only if
   A1/A0 CI excludes 1 across the full band AND does not shrink toward 1 with bits.
- Population: selection rule of v2.1 §2 continued from seed 1015; >=10 fresh ell=2
  classes per bit-size in {24,28,32}; depth 3, split craters; 8 targets/curve;
  warm-up discarded. Seeds: class seeds 1015+; algorithm_seed derived per run;
  analysis seed 20261017. Freeze spec digest before any measured run.
- Admission: native L2/replay/A-A; replay certificate + validator VALIDATION.json
  + matched-rho band, per the validation gate, before any row.

## Cost / priority
Lowest-value lead. Effect is a few % constant factor on an arm 17-170x (median
46.5x at w=0) from rho; isogenous curves share group order so vertical moves are
rho-cost-invariant (confirmed: H3 rho floor/crater 0.989, ±5% equivalent) and
theory caps the vertical IC change at a constant factor. A constant factor cannot
iterate toward beating rho. Recommend: run ONLY if isolab capacity is otherwise
idle; otherwise log CLOSED as a diagnostic. This protocol is the single decisive
measurement that would settle it either way.
