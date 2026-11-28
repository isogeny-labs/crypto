# Completion of the scoped Vieta 2–6× local goal

The declared local goal is complete at **word62/m16384**: the frozen Rust
results report more than 2× full construction-plus-query improvement over
Euclid/Newton in both separately reported target populations. Implementation
and original results landed in [PR4535](https://github.com/isogeny-labs/crypto/pull/4535),
merged November 17 at 15:13:45 UTC as
`06d77461111293a8d05e04e4e2815aa9ae0eadb5`. These are producer observations,
claim `PCC-20261117-b4ee50`, **unverified and below L2** [pcc:unverified].

| Word62 coset m16384 population | Retained targets | Newton cold median | Candidate cold median | Paired speedup median | Conditional 95% interval |
| --- | ---: | ---: | ---: | ---: | --- |
| Conditioned sum | 3 | 5.3044 s | 0.6830 s | 7.56× | [7.31, 8.24] |
| Random abscissa | 3 | 5.3917 s | 0.6941 s | 7.61× | [7.42, 8.20] |

These copy the frozen native analyzer's stored values; no new statistic was
estimated. Paired ratio medians differ from dividing the separate arm medians.
The intervals describe these small samples, without population significance
or exponent inference. The native stored control audit records exact A/A and
positive count controls, clock controls, complete reference pairs and the
predeclared local criterion as met for both groups [pcc:unverified].

Cold clocks include fresh native field setup, a certified descriptor, the
complete point factor base, any required lookup index and one query. Fixture
and oracle preparation, process launch, canonical output encoding and
independent replay are outside both worker intervals. The shared native fields,
NTT products, unchanged half-GCD and exact linear/quadratic roots are explicit
opt-in components. Schoolbook, Euclid/Newton, intermediate references and
optimized lookup remain. Lookup is cheaper in both populations. Query clocks,
construction clocks and primitive counts remain in the original native
[summary](../speedup-20261117/evidence/primary-summary.json). Native field
constructor internals are timed but unpriced in the counters, so this does
not establish a complete fast cold operation count [pcc:unverified].

The corrected primary-v2 protocol was published at `0b63f6e26e` before calls;
its exact bytes survive the final PR and merge. That pre-call commit also
contains the successful file-check of the seed/export qualification fixture.
The original final primary export's outcome-label defect and its 16 findings
remain immutable beside the metadata-only corrected derivative, whose file
check has zero findings. No raw seed, target, status, metric or hash was
relabeled. See the original [results](../speedup-20261117/RESULTS.md),
[report PDF](../speedup-20261117/report.pdf), registration addenda and
[completion audit](evidence/completion-audit.json) for the exact sources and
the contemporary journal chronology [pcc:unverified].

The first 192-call cohort remains inconclusive as a whole: 176 successful
calls replayed complete relation sets and 16 larger word32768 reference calls
hit the frozen operation cap. Those failures do not affect the complete
word16384 groups, and are not erased by this completion decision. A separate
fresh8b cohort replayed all 48 calls; it replaces no original call. All
replays are producer-owned BigUint scalar complete-set checks, not signed
independent execution. Profiles retain explicit timer undercoverage warnings;
they do not establish reliable CPU hotspot proportions. The word fixture is
synthetic, without a prime-order subgroup assertion [pcc:unverified].

The current official standing query reports no signed verdict for the data
claim. The full degree bound and general normal-form instantiation remain
unverified. This completion is not L2 admission, a fastest-external-tool result,
an ECDLP pipeline comparison, or a degree-bound theorem. The declared goal
requires these limits to be reported honestly; it does not require converting
the local observations into independent standing [pcc:unverified].

The entire original study tree is identical at the former checkpoint,
final PR head, merge and inspected current main. The merge retains both
parents' complete normalized journal blocks and the exact first-parent PCC
ledger prefix. Actual final-head size passed after merge; gate and artifacts
failed after merge. No clean-CI inference is made. Autonomy episode
`E-96bdfe264fbe` consequently remains partial/not-green, separately from the
local numerical goal; its intervention history is not reconstructed or
asserted empty. Earlier partial lesson outcomes remain unchanged. The new
scoped goal outcome is `O-fada519f4468` [pcc:unverified].

This administrative receipt is separate from PR4525's results, protocols and
censored record closure. It changes none of either study's scientific files
or benchmark records. No benchmark or replay rerun, new cloud experiment,
local S3 operation, gate edit or protected promotion occurred. Current AGENTS,
the newest journal, the report-evidence skill and the original frozen sources
were read. Conductor remains unavailable; the existing isolated administrative
lane PR4987 carries this addition. No new Rust or full-root qualification is
claimed. Further independent validation would be a separate scientific step.
