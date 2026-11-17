# Preregistered protocol (experiment.protocol/v1 style) — ghs_n7 nonclassical-W1 escape

Status: PREREGISTRATION ONLY. Not run here (Sage/Magma MISSING; no isolab/S3).
Author: audit session 2026-11-16. All producer-side numbers below labeled [pcc:unverified].

## Precise open question
Over K=F_(q^7), q=2^ell, with cubics p1=T^3+T+1, p2=T^3+T^2+1 and V_i=ker p_i(sigma):
the W1 locus has ~q^6 coefficients (reproduced: 56/3906/237839 union at q=2/4/8).
Exactly 2(q^3-1) of them (the classical slice b in V_1 u V_2) admit a genus-7
GHS target. The remaining ~q^6-2(q^3-1) "nonclassical" coefficients (42 at q=2,
3780 at q=4, 236817 at q=8 — all reproduced independently) have b outside V_1 u V_2.

QUESTION: Does there exist a k-rational, subgroup-preserving cover/correspondence
C' -> Y with g(Y) < 14 (ideally genus-7-or-lower GHS-tractable) that carries the
odd-order ECDLP subgroup of E for a NONCLASSICAL b, and is NOT reducible to the
classical genus-7 GHS target? I.e., is there a transfer that is NOT "classical
GHS in disguise"?

## What is already excluded (reproduced locally, this audit)
- Symmetric dihedral ansatz (gamma2 = c*gamma1 + d, c,d in k): collapses — beta in V_i,
  so b classical. Reach set == classical slice exactly; nonclassical ∩ reach = 0
  at q=2,4. [verified: recheck_collapse.py]
- Natural k-rational genus-7 quotient D: hyperelliptic ⟺ b in V_p. Nonclassical b
  gives genus-7 but non-hyperelliptic D (no GHS target). [producer THEOREM, numerics reproduced]
- Any DIRECT degree-d quotient C->Y retaining g(Y)>=7: Riemann-Hurwitz forces the
  unique profile (deg=2, g(Y)=7, R=2); the genus-7 case's norm is zero on the odd
  subgroup (Hess §8.3). [RH arithmetic reproduced: only solution is (2,7,2)]
- Deck-involution quotients land at genus 3 or (g=6) norm-zero.
So every CONCRETE route proposed to date is closed. The surviving sliver is:
NON-direct correspondences / different involutions, for which NO construction exists.

## Arms (gated; require Sage/Magma — NOT runnable here)
- ARM-COVER: for a fixed nonclassical witness, build the genus-14 Hess cover C and
  its full Jacobian/Prym decomposition; enumerate ALL k-rational quotients and
  degree<=3 correspondences; for each, compute (i) target genus, (ii) the
  norm/conorm map on the odd-order source subgroup, (iii) whether target reduces
  to the classical genus-7 target (F2-isomorphism / isogeny test).
- ARM-GHS (only if ARM-COVER yields a genus<=7, nonzero-norm, non-classical target):
  run the full GHS pipeline (cover construction -> function-field sieve / IC relation
  collection -> linear algebra -> individual log) on that target, counting ops in
  ecbench's counted unit.
- ARM-RHO (matched reference): Pollard rho on the SAME source odd-order subgroup of E,
  same counted unit, §5a band.

## Fixed witnesses (grounded this audit)
- q=2, K=F2[w]/(w^7+w+1) (0x83): gamma1=2, gamma2=5 -> z=10, b=68. CONFIRMED b not in
  V_1 and not in V_2 (nonclassical). #E not yet checked for this b here.
- q=4, modulus 0x4021: arm1 (gamma1=89, gamma2=187, b=4109); arm2 (gamma1=77, gamma2=430, b=440).
  (producer-reported witnesses; coefficient-level nonclassicality reproduced in aggregate, not per-witness here.)
- Larger cryptographic targets (q=2^k, k=16..40) chosen cold/public before any cost run.

## Counted unit and pricing (AGENTS.md §5a)
- Unit: ecbench group-operation equivalent, with field mul/sqr/inv behind each group op
  recorded. Price every arm's unpriced native work (GHS cover build, IC linear algebra,
  rho steps) at 0, 0.1, and 1 GAE per operation. Rank GHS vs matched rho ONLY where the
  order of the two holds across the whole 0/0.1/1 band.

## Falsification criteria (decide "just classical GHS in disguise" vs genuine)
CLOSED (lead dies) if EITHER holds for every enumerated correspondence of a nonclassical b:
  (F1) the only targets with g<=7 and nonzero subgroup norm are F2-isomorphic or isogenous
       to the classical genus-7 GHS target of some classical b'; OR
  (F2) every target that is NOT classical has zero norm on the odd-order subgroup
       (subgroup not preserved); OR
  (F3) every non-classical subgroup-preserving target has g>=14 (no genus advantage),
       so matched rho dominates across the full §5a band.
GENUINELY OPEN / ADVANCE only if some nonclassical b yields a k-rational,
subgroup-preserving target with g<14 that is provably not classical-equivalent AND
the measured GHS cost beats matched rho across the whole 0/0.1/1 GAE band.

## Seeds / determinism / order
- Cover/correspondence enumeration is deterministic (no seed). Rho and any randomized
  IC relation search: derive per-arm seeds from a single published master seed;
  interleave ARM-GHS and ARM-RHO; mask arm labels during cost analysis.
- Freeze this protocol (hash) before any data; bind run as experiment.export/v1.

## Tooling the gated step needs (currently MISSING here)
- Magma (preferred) or Sage with function-field + Jacobian/Prym packages:
  AlgorithmicFunctionField / RiemannRochSpace, Jacobian arithmetic, isogeny/Prym
  decomposition in char 2; plus the repo ecbench harness on isolab for the cost arms.
