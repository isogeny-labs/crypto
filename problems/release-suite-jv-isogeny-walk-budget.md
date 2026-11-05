# Release-library validation stalls at the unchanged JV isogeny-walk case

- **Status:** investigating
- **Kind:** software-validation resource limit; cause unknown
- **Affected work:** [PKM execution prerequisites](../research/prime_ecdlp_three_leads_20261029/execution-20261030/README.md), [earlier validation limit](../research/prime_ecdlp_three_leads_20261029/PROBLEM.md)
- **Next action owner:** native library/JV validation maintainer
- **Last updated:** 2026-10-30

## Problem

A complete release-library pass is not established: the unchanged
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`
case remains active at the local40minute execution budget. The cause of its
runtime is unknown. No assertion failure or mathematical obstruction is inferred.

## Evidence and uncertainty

- **Observed (2026-10-29):** parallel5356-case run capped at40minutes with5232passed,0failed,123ignored and that one case unfinished; historical receipt is preserved.
- **Observed (2026-10-30):** serial5553-case run capped at40minutes with2173passed,0failed,47ignored,one case active at the cap and3332cases not reached. The timeout wrapper exits124; no final full-suite summary exists.
- **Source check:** src/cryptanalysis/jv_isogeny_walk.rs is byte-identical between8baeee2d446968936882adc503570fb957bb1429 and the new PKM baseedf3db4457bc286099d08365f12c9b1300949d6c. The new PKM work does not modify library source or that test.
- **Observed:**12new PKM tests and8legacy regressions pass; this does not substitute for the full library gate.
- **Missing capability:** a diagnosis/complete passing unmodified focused case and final-source full suite under an adequate stated validation envelope.
- **Not established:** deadlock, nontermination, a change-caused regression or any negative research conclusion about PKM.

## What would unblock it

- [ ] Reproduce/profile the exact unmodified case with source, compiler, inputs, seed and process/resource telemetry retained; identify the cause rather than weakening assertions or introducing an ignore.
- [ ] Complete the focused case with original correctness checks and a documented validation envelope.
- [ ] Complete the full release-library suite on the final relevant source, reconciling the entire declared count.

## Next discriminating step

Run a controlled focused software-validation diagnosis of the unchanged case,
retaining progress and resource telemetry. Keep it separate from PKM scientific
panels. A focused pass resolves only that reproduction; the full-suite obligation
remains. A cap preserves the resource limit and supplies no pass or impossibility
claim. No test has been skipped, disabled or waived. Recheck on a diagnosis,
source/runtime change, or adequate validation host/budget.

## Resolution and history

Unresolved. The two raw logs remain at the exact local paths/hashes in their
respective validation receipts; required artifact upload is unavailable. Current
raw-log digest/size and bounded excerpt are retained beside the PKM execution
report. Completing this problem record does not complete the validation gate.


## Integrated validation at source merge 2acf618dd (2026-10-30)

The new 5646-case serial release-library execution capped at 40 minutes:
2208 observed passes, zero observed failures, 47 ignored, the same JV case
active and 3390 cases not reached. The timeout wrapper and supervisor exit
124; Cargo's direct exit is unknown. No final suite summary/full pass exists.
The JV source Git blob remains 568a67fb3a7e4067036c7b5985648c477a46290d,
identical to the original 8baeee2d snapshot. No test or assertion was changed.

The new 214615-byte raw log is NOT_UPLOADED, SHA256
`d41f2749c59315e4f1c11d5315c212e67489ab983662dd94633aabc39c0977ca`.
Exact command/source/binary bindings, reconciliation and bounded excerpt are
in [the final integration receipt](../research/prime_ecdlp_three_leads_20261029/execution-20261030/integration-validation/final-receipt.json).
The scoped 20-example suite, strict lint/formatting, CLI checkpoint refusal,
backlog validation and library build pass on that source. They do not replace
a full-library pass. All earlier receipts/logs remain separate and unchanged.
This repeated validation limit remains unresolved; no duplicate attempt is
launched merely to repeat the same cap. Scientific G1–G6 is also incomplete.
