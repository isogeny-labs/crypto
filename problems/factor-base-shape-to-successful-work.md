# Factor-base geometry has not established cheap successful decomposition

- **Status:** open
- **Kind:** implementation and measurement gap; formal-proof gap for a regularity claim
- **Affected work:** [chart-complement screen](../research/factor_base_chart_complements_20261019/README.md), [completed shape study](../research/factor_base_shape_leads_20261018/README.md)
- **Next action owner:** native factor-base CLI lane; new follow-up remains draft/unfunded
- **Last updated:** 2026-10-19

## Problem

We cannot infer cheaper successful decomposition or certified degree of
regularity from a factor base's finite subgroup support, cofactor fibres,
bounded refutation measurements or relation counts.

The concrete candidate `B027+B033` on `icv1-f2m13-t181-515ee569` has 14
columns and exact support 242/2002, versus 226/2002 for 15-column GP seed 17.
This does not price the extra chart branches, setup, membership or witness
selection, nor prove how a solver scales. The degree 31 model
`icv1-f2m31-tm90707-c95f16f5` also has almost no useful subgroup support
in the finite screen. Neither fixture establishes83/130-bit applicability.

## Evidence and uncertainty

- **Observed:** source 6811c867 frozen before native screening;355 structural
  cases with all 62,986 unique pair sums directly checked. All cases and
  witnesses are in the study's raw archive and compact summary. The graft
  contributes42 targets requiring mixed-chart witnesses; its useful
  same-target-difference rank 3 equals GP seed 17's3.
- **Derived or proved:** finite counts checked by native arithmetic and
  agreement with prior singleton coverage. No kernel proof of the full
  census, polynomial identity certificate or certified regularity result.
- **Suspected explanation:** a tiny auxiliary chart could make some new
  decompositions accessible through a small number of point-subtraction and
  membership operations. Its charged contribution to independent relations
  is unknown. Cofactor-fibre concentration may help select aligned shapes;
  projecting points can remove the inexpensive membership representation.
- **Missing capability:** matched fresh-target native implementation and
  complete counters for the graft fast path/general chart solver and
  rank-aware admission; formal algebraic analysis for any regularity claim.
- **Not established:** significance, solver speed, an exponent change,
  complete one-target ECDLP cost, a rho win or a large-field mechanism.

## What would unblock it

- [ ] Freeze a new native protocol with exact inventories, matched fresh
  public targets, positive subgroup controls and a separate holdout.
- [ ] Implement and verify main-main, mixed and auxiliary-only branches;
  charge setup, failed attempts, point/membership operations, root/witness
  enumeration and incremental rank. Compare against unchanged references.
- [ ] Admit a cost improvement only with complete symmetric native pricing;
  include fastest matched external implementations for a speed claim.
- [ ] For a regularity claim, provide the actual ideal/system/order and a
  faithful certificate/proof; measured successful degree is insufficient.

The first three conditions can establish an engineering improvement in the
measured stage without beating rho. They do not by themselves certify
regularity or end-to-end ECDLP advantage. A full-pipeline claim still needs
all phases, matched rho and independent replay under repository rules.

## Next discriminating step

- **Action:** draft [chart-graft follow-up](../backlog/XP-20261019-factor-base-chart-grafts/card.json), not dispatched.
- **Controls:** the exact14-column graft,12-column main component,15-column
  GP17 reference and 16-column coordinate reference on the same exact model.
- **Limits/stop:** no runnable protocol or funded execution envelope exists;
  keep the card draft/unrun/zero-funded/unleased until those are frozen.
- **Outcomes:** lower verified complete native cost per independent relation
  would support a stage improvement; extra support erased by branching or
  low rank is a negative result. Timeouts or missing accounting are
  inconclusive, not mathematical impossibility. Preserve every arm.
- **Recheck trigger:** implementation plus preregistered matched counters
  and verification, or a concrete relevant algebraic certificate.

## Resolution and history

2026-10-19: finite chart-union screen supplies a specific candidate and
cofactor/rank diagnostics. Successful solver work and regularity remain
unresolved. This record and its PR do not discharge those obligations.
