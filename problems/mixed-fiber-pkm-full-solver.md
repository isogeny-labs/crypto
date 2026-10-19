# Complete algebraic mixed-fiber PKM solver and DLP pipeline

- **Status:** `open`
- **Kind:** implementation and measurement gap
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L3
- **Next action owner:** unassigned follow-up
- **Last updated:** 2026-10-19

## Problem

The current small mixed-fiber census uses a complete pair-sum table; it does
not build the requested heterogeneous `m=3` polynomial systems or time a
complete F4/msolve root-and-lift solver, so it cannot establish a PKM algebraic
solver or end-to-end DLP advantage.

## Evidence and uncertainty

- **Observed:** [540 complete oracle records](../research/five_leads_e2e_20261019/correctness/mixed-fiber-census-v1.jsonl) on nine public small prime-field curves and 180 targets agree with an independent exhaustive triple enumerator in the same native source. [Cost summary](../research/five_leads_e2e_20261019/results/mixed-fiber-summary.json) counts cold field operations and logical table work. At the declared `M+0.75S+10I` price, mixed/homogeneous pair-table cost is 1.005–1.019 by field-size cell.
- **Derived or proved:** Only the exact finite solution sets of these small pair-table systems are established by enumeration.
- **Suspected explanation:** Unknown whether heterogeneous fibers change complete algebraic solving degree or root cost; no F4 run was made.
- **Missing capability:** polynomial-system generator, all-roots F4/msolve adapter, sign-orbit lift checker, matched strongest external solver run, and relation/linear-algebra DLP pipeline. On the execution host, `msolve`, `sage` and `Magma` were absent from `PATH`; this is an access/tool limit in addition to the missing implementation.
- **What this evidence does not establish:** A pair-table cost ratio is not a Gröbner cost ratio or a complete IC/rho ratio; the tested primes/fiber cardinalities do not imply behavior at N=21.

## What would unblock it

- [ ] Generate equivalent `m=3` polynomial presentations for each frozen homogeneous, mixed, random and scrambled control, then certify their complete rational solution sets against the enumerator.
- [ ] Compare total root-and-lift cost, including setup, rejected lifts and failures, against msolve/full-F4 and the fastest available implementation on the same inputs and host.
- [ ] If the oracle improves, run complete one-target DLPs with source scalar checks and matched rho, retaining all costs and misses.

The first two conditions establish a solver result only; the third establishes
a complete DLP result.

## Next discriminating step

- **Action or proof obligation:** Implement the three-summand polynomial generator and all-roots solver on the existing 540 small cells.
- **Inputs and controls:** Exact fibers, targets and enumerated solution sets in the retained census; homogeneous, random point-count/sign-orbit matched and scrambled controls.
- **Execution limits and stop condition:** Freeze solver version, host, caps and target split before timing. Stop promotion on any missing or spurious root or unmatched cost interval.
- **Outcomes and next actions:** A complete solver win enables IC testing; a loss or equal cost is retained for these cells; an external-tool failure is an access/implementation limit.
- **Recheck trigger:** A checked polynomial all-roots adapter and external solver access.

## Resolution and history

2026-10-19: Exact small pair-table census completed; polynomial solving and
end-to-end DLP remain unresolved.
