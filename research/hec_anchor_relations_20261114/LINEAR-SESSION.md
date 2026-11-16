# Linear-denominator working log, 2026-11-15

## Protocol freeze

### What was done (commands, files, features)
Read shared dated log and primitive study log, verified current dirty state
(only older shared resume edit), reviewed the generic normalization and checked
primary interpolation source. KB receipt and scope are in LINEAR-PROTOCOL.md.
Claimed T-6042for this study before editing; shared log/resume remain untouched.

### Decisions made (and why)
Previous turn was progress: a complete target solver, finite normalization/rank
improvement and replay. Eliminate generic GCD/full norm division in d2 through
the short denominator and exact high-coefficient recurrence, preserving every
tuple/sign/row decision. Benchmark only after this equality gate passes.

### Open threads (unfinished, pick up next time)
Implement/gate the specialization; then run a separately frozen fully cold
same-target comparison. Goal remains mathematical>=10x for genus2+.

### Errors (what went wrong, how it was handled)
No new error. KB lacks a directly relevant retrieved mechanism source; primary
paper was read directly. No inference of novelty follows from search results.

## Exact specialization and complete-solver gate

### What was done (commands, files, features)
Implemented linear.rs: a=b*H-S0^2*A, b=S0*x-S1; Gray-updated anchor probes
classify common factors without generic GCD, and high norm coefficients give
Qwithout full polynomial norm division. All2688curve/sign cases agree exactly
with subset a,b and generic normalized a,b,Q/masks;118common-factor and13
constant-denominator cases are covered, plus forced zero/constant controls.
An explicit solve_linear wrapper preserves all deterministic counts in the
12full generic solver runs, with both raw/projected group gates passing.

### Decisions made (and why)
The specialization changes arithmetic while preserving relation streams and
complete solver work. It is now eligible for a same-target cold screen. It
remains an application of established interpolation, without a novelty claim.

### Open threads (unfinished, pick up next time)
Freeze the cold protocol, implement a separate worker/stage without changing
old ARMS15/cohort/receipts, then measure all cold controls and candidate phases.

### Errors (what went wrong, how it was handled)
No mismatch occurred. These gate clocks are diagnostic, not performance evidence.

## Cold-worker preparation

### What was done (commands, files, features)
Implemented separate --linear-cold stage and rational worker arms while keeping
old ARMS15and its cohort unchanged. All25tests in the heldout example pass,
including exact generic/linear full-solver counts. New native collector pins
source, args and fixture SHA for the six-curve/18target/288cell protocol.

### Decisions made (and why)
Commit scientific source before building the timed binary. Keep collection/
reduction/plot generation native. Shared bench registry/custody tools handle
only generic metadata; they will not launch scientific workers or compute the
study's statistics. Existing progress metadata is separate from the study's
native numerical result path. Scoped metadata taskT-6049was checked/claimed.

### Open threads (unfinished, pick up next time)
Pin source/library/binary, open record, checkpoint, execute capped child stage,
collect/reduce all outputs and evaluate E6/E7. No result is known yet.

### Errors (what went wrong, how it was handled)
No new error. Old frozen raw data and closed records remain unchanged.

## Complete cold screen result

### What was done (commands, files, features)
Source ed0fb2c5f982was committed before build/run. Pre-run S3checkpoint
20261115T093410Z-ed0fb2c5f982captures source/protocol; record opened at
benchmarks/records/20261115T0930Z-hec-linear-cold.json before execution.
Native worker caps/custody controls pass. All18full gates and288timed cells
exit0with correct scalars. Gates check17064raw and600projected identities.
Native Rust collector validates source/args/input SHA; native jq computes
summary. All18generic/linear/A-Acount signatures and target identities agree.
Stage wall8.18s; summed owned child wall7.976461035s. No experimental cap/error.

### Decisions made (and why)
E6passes. E7fails: six three-target primary medians against the fastest of
14existing baselines (including compact and generic) are1.4341,0.7504,
0.8390,0.6960,0.9046,0.7551. Generic/linear full-runtime median gains are
2.0988,1.4557,2.1562,2.5965,2.3880,3.0202, with exact work/count equality.
Do not multiply these by old gains. A/Amax2.5462limits confidence in small
positive ratios. This implementation misses the10xpilot gate; no universal
negative claim about rational interpolation follows. Relation generation
remains66..78% of candidate time. Next mechanism should change useful-relation
supply/selection, rather than another equivalent kernel arithmetic rewrite.

### Open threads (unfinished, pick up next time)
Close raw custody/record, generate native report/PDF, preserve every target/loss,
regenerate generic progress metadata, publish and follow current-head CI.
Then literature-screen a structural relation-generation successor. Full objective
stays active; no scientific process is running.

### Errors (what went wrong, how it was handled)
No scientific cell failed. Runner's changed-input and timeout controls are
intentional diagnostics in separate artifacts, never experimental failures.
All new statistical reduction and plot generation remain native; shared ledger
metadata is separate. The reused six curves are not a fresh curve holdout.

## Report, custody and successor screen

### What was done (commands, files, features)
Built and inspected both native cold report pages plus the affected global
metadata PDF pages23/24. Closed record20261115T0930Z-hec-linear-cold has12native-
computed ratio observations and one hashed166699byte raw archive. Generic
registry/progress metadata is current. Added metadata-only daily exporter that
copies the committed recorded values, without executing/reducing a study.
Primary-source successor review: Improved sieving on algebraic curves,
https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf,
Sections2.2/2.3read. Its point-parameter buckets and at leastg-1small-base hits
before residual-quadratic testing are known Sarkar–Singh/Joux–Vitse prior art.
Queries: hyperelliptic sieving Flassenberg Paulus; "pencil" "index calculus".
KB hits were unrelated to this mechanism; no novelty inference follows.

### Decisions made (and why)
Keep the measured negative pilot and exact-work gain at their proper scope.
No admitted benchmark row/leaderboard/default promotion; README retains an
unvalidated evidence link. Next cost case is structural harvesting/selection,
with bucket construction, useful yield, correlation/dedup, forest/rank and cold
setup charged against compact. No new sieve experiment was run.

### Open threads (unfinished, pick up next time)
Publish results, follow CI upload and verify exact raw object/head; fast-forward
only automatic artifact pointers and checkpoint. Continue from LINEAR-HANDOFF.md.

### Errors (what went wrong, how it was handled)
Local raw PutObject was AccessDenied. Do not represent that as successful upload;
use the existing trusted credentialed CI path for the Git-retained archive.
A coordination CLI --next accidentally claimed unrelatedT-28; released it
immediately, changed no files in that task, then claimed the explicitT-6056.
Initial Typst rendering needed a space in b H; corrected and visually inspected.
Shared resume stays untouched. Local broad sparse-ledger diagnostics do not
replace current-head complete-checkout CI. Closed historical records unchanged.

## Final publication checks

### What was done (commands, files, features)
Closed record has no metadata errors; canonical generic progress report is
current. Daily metadata exporter emits all12recorded native ratio values when
run against committed HEAD with --since2026-10-01, in under one second. New
benchmark report directory was materialized in sparse-checkout patterns and
committed after an initial git-add refusal; allfourentryfiles are now tracked.
Scientific run/source/raw archive remain unchanged. Prepared PR scope update.

### Decisions made (and why)
Use complete-checkout current-head CI as the S3/canonical admission authority.
Local whole-ledger gate reports1862old missing-file/manifest issues in this
sparse checkout; targeted new-record metadata validation passes. Keep exact
failed upload and sparse diagnostics in the log, without editing historical
records or claiming a local global pass. No merge or admitted-row promotion.

### Open threads (unfinished, pick up next time)
Push and verify current-head CI/S3 raw custody, FF any artifact-only bot commit,
checkpoint. Then execute the structural cost screen in LINEAR-HANDOFF.md.

### Errors (what went wrong, how it was handled)
Initial benchmark entry files were outside sparse patterns; added the directory
permanently and committed it before push. The first metadata smoke used Git's
same-day --since cutoff and saw no new record; the required historical window
emits all12unchangedrecorded values. Shared resume was not reapplied or edited.

## Exploratory joint-cost diagnostic

### What was done (commands, files, features)
From the closed raw observations, native harvest_bound.jq subtracts compact's
relation time per target, holds all other observed costs fixed, then divides
best-existing total by that remainder. Replay matches HARVEST-ONLY-BOUND.json.
Maximum conditional ratio3.6210831112x; fixture medians1.53..3.26. Updated the
results/handoff/report and inspected the modified PDF page; raw archive, closed
record metrics and E6/E7criteria are unchanged.

### Decisions made (and why)
A harvesting-only same-work replacement cannot supply the10xcase under this
fixed-cost assumption. The next structural cost case must change useful
harvesting AND downstream bookkeeping/recombination/setup/algebra, or show that
its changed relation distribution reduces those costs. This is exploratory
post-run analysis, not a universal impossibility theorem or an observed speedup.

### Open threads (unfinished, pick up next time)
Publish this derived diagnostic with the measured screen, finish CI/S3custody,
then cost-screen a joint known-sieving/sparse-recombination successor.

### Errors (what went wrong, how it was handled)
The completed linear-kernel coordination task exhausted its lease/attempt
budget; the distinct post-run cost-analysis taskT-6066was checked/claimed before
source updates. No scientific run was retried, no closed record edited, and
no shared resume content changed.
