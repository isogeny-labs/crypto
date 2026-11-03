# Complete algebraic mixed-fiber PKM solver and DLP pipeline

- **Status:** `open`
- **Kind:** implementation and measurement gap
- **Affected work:** [five-leads follow-up](../research/five_leads_e2e_20261019/STATUS.md), L3
- **Next action owner:** unassigned follow-up
- **Last updated:** 2026-10-20

## Problem

The current small mixed-fiber experiment now includes a complete exhaustive
Semaev-S4 polynomial all-roots and sign-lift DLP solver, plus a native F4
oracle replay for the same one-target cohort. Its F4 counter omits symbolic
preprocessing and substitution arithmetic, and no fastest external solver
has run, so a complete F4 cost or external ranking is not established.

## Evidence and uncertainty

- **Observed:** [540 complete oracle records](../research/five_leads_e2e_20261019/correctness/mixed-fiber-census-v1.jsonl) on nine public small prime-field curves and 180 targets agree with an independent exhaustive triple enumerator in the same native source. [Cost summary](../research/five_leads_e2e_20261019/results/mixed-fiber-summary.json) counts cold field operations and logical table work. At the declared `M+0.75S+10I` price, mixed/homogeneous pair-table cost is 1.005–1.019 by field-size cell.
- **Derived or proved:** The exact finite solution sets of the initial pair-table systems agree with independent enumeration; the later Semaev-S4 solver computes the same target DLPs by exhaustive polynomial evaluation and checked lifts.
- **Observed (2026-10-20):** [540 exhaustive polynomial DLP runs](../research/five_leads_e2e_20261019/correctness/mixed-fiber-polynomial-dlp-v1.jsonl) on the same 180 targets all recover and source-verify the scalar. The [native full price grid](../research/five_leads_e2e_20261019/results/mixed-fiber-polynomial-dlp-price-grid.csv) gives mixed/rho 32.2975× and mixed/pair-table 10.0233× at M+0.5S+10I and 0.1M per native logical operation; all 36 pooled cells lose to both. Every polynomial root was sign-lifted or rejected, and all 540 targets/fibers/rho records match the prior pair-table cohort.
- **Observed (2026-10-20, F4):** The native [F4 query receipt](../research/five_leads_e2e_20261019/correctness/mixed-fiber-f4-dlp-oracle-v1.jsonl) matches every rational root set for all 2,290 polynomial systems among 2,316 relation trials; 26 identity right-hand sides required no polynomial system. The [F4-backed DLP replay](../research/five_leads_e2e_20261019/correctness/mixed-fiber-f4-dlp-v1.jsonl) recovers and source-verifies all 540 scalars, with no timeout or root mismatch. F4 reports 467,482,105,353 row-reduction field multiplications; even counting only that subset plus the collector's recorded costs, mixed/rho is 232,110× at the representative price and loses all 36 pooled cells. The mixed/homogeneous counted lower bound is 1.44258×, reversing the exhaustive evaluator's ranking. [Analysis](../research/five_leads_e2e_20261019/results/mixed-fiber-f4-dlp-lower-bound-summary.json) labels the omitted operations.
- **Suspected explanation:** Different matrix shapes or omitted F4 work may affect the full mixed/homogeneous ranking; the counted lower bound alone does not prove a complete F4 cost ordering.
- **Missing capability:** complete F4 primitive accounting, matched fastest external solver run, and larger-size validation. On the execution host, `msolve`, `sage` and `Magma` were absent from `PATH`; this is an access/tool limit in addition to the accounting gap.
- **What this evidence does not establish:** The counted F4 lower bound is not a complete F4 cost or qualified wall-speed comparison; the tested primes and fiber cardinalities do not imply behavior at N=21.

## What would unblock it

- [x] Generate the Semaev-S4 polynomial presentation for the frozen homogeneous, mixed and matched-random DLP controls, enumerate its rational zeros and independently sign-lift every accepted relation. Scrambled control remains open.
- [ ] Count all native F4 root-and-lift costs, including symbolic preprocessing and substitution, then compare with external msolve/full-F4 and the fastest available implementation on the same inputs and host. The native F4 root-set and DLP cohort have run, but their operation count is a lower bound.
- [x] Run complete one-target DLPs with source scalar checks and matched rho for both exhaustive-polynomial and F4-backed solvers, retaining all costs and misses. Full F4 cost accounting remains open.

The small exhaustive-polynomial DLP result is complete within its stated
operation model; the external F4 comparison and larger-size question remain.

## Next discriminating step

- **Action or proof obligation:** Instrument the native F4 solver's symbolic and substitution operations, rerun the 2,290-system cohort, and compare complete cold costs with an external msolve all-roots adapter on identical systems.
- **Inputs and controls:** Exact fibers, targets and enumerated solution sets in the retained census; homogeneous, random point-count/sign-orbit matched and scrambled controls.
- **Execution limits and stop condition:** Freeze solver version, host, caps and target split before timing. Stop promotion on any missing or spurious root or unmatched cost interval.
- **Outcomes and next actions:** The counted lower bound already loses to rho; complete counts can rank fiber choices and quantify the full gap. An external-tool failure is an access or implementation limit.
- **Recheck trigger:** Instrumented F4 counters or external solver access.

## Resolution and history

2026-10-19: Exact small pair-table census completed; polynomial solving and
end-to-end DLP unresolved then.

2026-10-20: Exhaustive Semaev-S4 polynomial all-roots and sign-lift solver
completed all 540 DLP arms on the matched cohort, losing to source rho and
the earlier pair table in every pooled price cell. F4 and external
comparison were unresolved at that point.

Later 2026-10-20: Native F4 returned exact roots on all 2,290 nonidentity DLP
query systems and an F4-backed collector replay recovered all 540 scalars. Its
counted row-reduction lower bound alone loses to rho and the pair table in
every pooled price cell. Complete F4 primitive counts and external msolve
comparison remain unresolved.
