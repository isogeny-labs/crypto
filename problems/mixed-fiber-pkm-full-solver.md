# Complete algebraic mixed-fiber PKM solver and DLP pipeline

- **Status:** `open`
- **Kind:** implementation and measurement gap
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L3
- **Next action owner:** unassigned follow-up
- **Last updated:** 2026-10-20

## Problem

The current small mixed-fiber experiment now includes a complete exhaustive
Semaev-S4 polynomial all-roots and sign-lift DLP solver. It does not time a
complete F4/msolve root-and-lift solver or compare the fastest external
tool, so it cannot establish a PKM algebraic-solver advantage.

## Evidence and uncertainty

- **Observed:** [540 complete oracle records](../research/five_leads_e2e_20261019/correctness/mixed-fiber-census-v1.jsonl) on nine public small prime-field curves and 180 targets agree with an independent exhaustive triple enumerator in the same native source. [Cost summary](../research/five_leads_e2e_20261019/results/mixed-fiber-summary.json) counts cold field operations and logical table work. At the declared `M+0.75S+10I` price, mixed/homogeneous pair-table cost is 1.005–1.019 by field-size cell.
- **Derived or proved:** The exact finite solution sets of the initial pair-table systems agree with independent enumeration; the later Semaev-S4 solver computes the same target DLPs by exhaustive polynomial evaluation and checked lifts.
- **Observed (2026-10-20):** [540 exhaustive polynomial DLP runs](../research/five_leads_e2e_20261019/correctness/mixed-fiber-polynomial-dlp-v1.jsonl) on the same 180 targets all recover and source-verify the scalar. The [native full price grid](../research/five_leads_e2e_20261019/results/mixed-fiber-polynomial-dlp-price-grid.csv) gives mixed/rho 32.2975× and mixed/pair-table 10.0233× at M+0.5S+10I and 0.1M per native logical operation; all 36 pooled cells lose to both. Every polynomial root was sign-lifted or rejected, and all 540 targets/fibers/rho records match the prior pair-table cohort.
- **Suspected explanation:** Unknown whether heterogeneous fibers change F4 solving degree or root cost; no F4 run was made.
- **Missing capability:** complete F4/msolve all-roots adapter, matched fastest external solver run, and larger-size validation. The Semaev-S4 generator, exhaustive all-roots checker, sign-lift checker, and relation/linear-algebra DLP pipeline are implemented for the small cohort. On the execution host, `msolve`, `sage` and `Magma` were absent from `PATH`; this is an access/tool limit in addition to the F4 implementation gap.
- **What this evidence does not establish:** Neither a pair-table nor exhaustive-polynomial cost ratio is a Gröbner/F4 cost ratio; the tested primes and fiber cardinalities do not imply behavior at N=21 or qualified wall-speed performance.

## What would unblock it

- [x] Generate the Semaev-S4 polynomial presentation for the frozen homogeneous, mixed and matched-random DLP controls, enumerate its rational zeros and independently sign-lift every accepted relation. Scrambled control remains open.
- [ ] Compare total root-and-lift cost, including setup, rejected lifts and failures, against msolve/full-F4 and the fastest available implementation on the same inputs and host.
- [x] Run complete one-target DLPs with source scalar checks and matched rho for the exhaustive polynomial solver, retaining all costs and misses. This does not substitute for the F4 comparison above.

The small exhaustive-polynomial DLP result is complete within its stated
operation model; the external F4 comparison and larger-size question remain.

## Next discriminating step

- **Action or proof obligation:** Implement the three-summand polynomial generator and all-roots solver on the existing 540 small cells.
- **Inputs and controls:** Exact fibers, targets and enumerated solution sets in the retained census; homogeneous, random point-count/sign-orbit matched and scrambled controls.
- **Execution limits and stop condition:** Freeze solver version, host, caps and target split before timing. Stop promotion on any missing or spurious root or unmatched cost interval.
- **Outcomes and next actions:** A complete solver win enables IC testing; a loss or equal cost is retained for these cells; an external-tool failure is an access/implementation limit.
- **Recheck trigger:** A checked polynomial all-roots adapter and external solver access.

## Resolution and history

2026-10-19: Exact small pair-table census completed; polynomial solving and
end-to-end DLP unresolved then.

2026-10-20: Exhaustive Semaev-S4 polynomial all-roots and sign-lift solver
completed all 540 DLP arms on the matched cohort, losing to source rho and
the earlier pair table in every pooled price cell. F4/msolve and external
comparison remain unresolved.
