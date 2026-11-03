# Prime-cover projected relation cost is unestablished

- **Status:** `open`
- **Kind:** implementation gap, independent-verification gap, and missing measurement
- **Affected work:** the [prime-field `C_d` study](../research/enge_primefield_cover_20261024/REPORT.md) and the [prime-cover Jacobian map gap](prime-cover-balanced-jacobian-and-induced-maps-missing.md)
- **Next action owner:** prime-field projected-relation workstream (unassigned)
- **Last updated:** 2026-10-25

## Problem

We cannot establish reusable projected rank, blind-target descent, or less
than `sqrt(ell)` total work for high-genus `C_{d,s}: y²=f(t^d+s)` over prime
fields. The current collector handles only fully rational zeros of two
restricted function families. It lacks arbitrary-divisor pushforward and a
complete, matched cost comparison with direct elliptic norms and one-target
rho. The intended setting has `g=Theta(log p)`, a prime-order subgroup of
`E(F_p)` of order `ell` with `ell` not dividing `d`, reusable setup priced
separately, and a previously unseen public target. `pi_* pi^*=[d]` establishes
subgroup transport in this setting; it does not supply a faster solver.

## Evidence and uncertainty

- **Observed:** the [deterministic `F_101` census](../research/enge_primefield_cover_20261024/reports/run-20261024T2315Z/REPORT.md) finds 30, 0, 0, 0, and 8 fully split public linear sections for `(d,s)=(2,0),(3,0),(5,0),(7,0),(2,6)` respectively. In `(2,6)`, target-only recovery takes 1,070 candidate sections, 108,070 cover point tests, and 22 accepted rows. All 202 accepted non-invariant target rows in its complete census match the same rational zero sets and parameters under the direct elliptic norm. A rho control takes 72 elliptic group operations; these units are not a speed ratio. The tested unshifted higher-genus target fibers are not fully rational, so that target collector does not apply.
- **Derived or proved:** the [study](../research/enge_primefield_cover_20261024/REPORT.md) derives the function-field norm identities and uses `pi_* div(h)=div(Norm(h))`. Projection alone creates no new elliptic relation. The Lean polynomial proof source is locally unverified; neither it nor the evaluated certificate formalizes the geometric divisor map.
- **Suspected explanation:** fully split degree-one zeros may become too rare as `d` grows. The observed zero counts support this only for the fixed field, covers, and sections; higher-degree primes and other functions remain untested.
- **Missing capability:** checked pushforward of arbitrary cover prime divisors and classes; degree-`>1` factor-base and descent handling; independent replay; rank after merging projected images; and a full one-target cost and memory record. The [existing prime-cover map problem](prime-cover-balanced-jacobian-and-induced-maps-missing.md) covers the degree-two even-model arithmetic dependency.
- **What this evidence does not establish:** no high-genus relation shortage, no lower bound against specialized projected families, no Enge-algorithm result, and no sub-rho or large-field security claim.

## What would unblock it

- [ ] Fix one exact prime-field curve, subgroup, `d`, shift-selection rule, function family, degree bound `B>1`, factor-base convention, and one-target protocol before collection. Count construction and failed attempts.
- [ ] Implement and independently replay norms and pushforward for every degree-`<=B` cover prime used, including nonsplit support, ramification, and infinity. Verify projected relations directly on `E` and merge duplicate images before rank accounting.
- [ ] Demonstrate reusable rank and blind-target descent for a target selected after setup, with independent scalar verification, or record the bounded failure and its exact limit.
- [ ] Measure cold one-target work and peak memory for the cover route, the same norm family handled directly on `E`, and strong one-target rho in matched units and hardware. A less-than-`sqrt(ell)` claim further needs a justified growing-size bound or predeclared scaling test.

Meeting these conditions permits a cost and mechanism assessment for the fixed
family. It would not establish a general asymptotic advantage or transfer to
other prime-field curves.

## Next discriminating step

- **Action or proof obligation:** implement degree-two cover-prime pushforward on one shifted small-field fixture, then compare projected rows with a direct elliptic norm collector under identical parameters and factor-base rules.
- **Inputs and controls:** retain the public `F_101`, `(d,s)=(2,6)` fixture, add a fixed degree-two factor base, and use direct elliptic norms and deterministic rho as controls. Choose the target after setup and measure rank after projection.
- **Execution limits and stop condition:** first run exhaustively on the fixed small field with every candidate counted. Stop on the first preserved pushforward discrepancy, unreplayed relation, or incomplete cost record. This correctness gate prescribes no large-field compute budget.
- **Outcomes and next actions:** agreement with useful new rows permits larger `d` and a matched cost study; no useful rows is a bounded negative for that family; a counterexample blocks rank claims until corrected; resource or implementation failure remains a gap.
- **Recheck trigger:** a checked arbitrary-prime norm map, higher-degree relation data, or a complete matched one-target cost record.

A planned step is not a completed experiment. Apply the repository's run,
review, and evidence rules before promoting a claim.

## Resolution and history

Unresolved. The 2026-10-24 study established a toy target-coupled relation
source and direct-norm equivalence for its tested family, while leaving the
high-genus and full-cost requirements open.
