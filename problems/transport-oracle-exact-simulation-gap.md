# Transport-oracle exact simulation gap

- **Status:** open
- **Kind:** proof gap, with a counterexample to the stated lemma
- **Affected work:** [R6.8 report](../research/isogeny_fibers_20261010/round6/r6_8_transport_oracle/RESULTS.md), Proposition 5-prime and the proposed closure of Proposition 5(c)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-20

## Problem

The stated simulation does not preserve all fixed-length decompositions because
it removes the identity from the transported set. Its arbitrary-preimage
lifting step and claim of challenge-independent adaptive paths also require
additional justification and accounting.

## Evidence and uncertainty

- **Observed and derived:** the checked [finite witnesses](../research/isogeny_fibers_20261010/round6/r6_8_transport_oracle/review-counterexample.json)
  show that, in C5 with the identity map, S = {0, 1} and arity 2, removing 0
  loses the nonzero target 1. They also show that choosing one arbitrary lift
  under C5 x C2 -> C5 can omit a valid mixed-fiber decomposition.
- **Missing proof:** a complete corrected simulation, including identity and
  kernel treatment, all lifts, adaptive path dependence and total overhead.
- **Limits:** these witnesses refute the stated steps; they do not refute
  every possible corrected theorem or establish a transport advantage.
  The original eight census files and 16 measured cells remain unchanged.

## What would unblock it

- [ ] Specify fixed versus at-most-m decomposition and prove the image-set
  relation with identity and kernel points handled explicitly.
- [ ] Return exactly the original decompositions while charging all lift
  enumeration and filtering work.
- [ ] Prove the required challenge independence under explicit adaptive
  attacker rules, or narrow the statement to a non-adaptive model.
- [ ] Account for path evaluations, base construction and scaling parameters
  before restoring any cost or asymptotic conclusion.

Meeting these conditions would repair the proposed theorem. It would not
independently establish an end-to-end performance improvement.

## Next discriminating step

Rewrite Lemma 5 for an image set that retains identity, replay the two finite
witnesses, and obtain an independent proof review of the remaining simulator.
This is a bounded proof/code review; no additional cloud campaign is required.
A corrected local example is necessary but not sufficient for the theorem.
Recheck when a revised statement and complete accounting are available.

## Resolution and history

Unresolved. On 2026-10-20 the closure claim was withdrawn, an executable
counterexample was added, and the original report/source/PDF were preserved
byte for byte with a SHA-256 manifest. Merging the correction does not resolve
the mathematical problem.
