# Double-LP calibration session, 2026-11-15

## Implementation and ODE gate
### What was done
Protocol3331b61abapreceded main.rs/core.rs/ode.rs. Native RK4step.25agrees with
closedu/f/Vin48cases, plus a zero-FPseeded-root control. C0andB0edges avoid
division byzero. Existing Fixture/target rule copied from heldoutsource.
Weighted full/rooted forest paths preserve public D1/D2columns, raw split
decomposition and every returnedscalar are group-checked. Raw/full/unique/
empty rows and exact active core rank remain separate counters.
### Decisions made
Fit only first2048trials, keep holdoutrules and fixed32768trialbudget. No
oneLPbirthday or independent-row inference. Echelon skips additional insertion
only after fullcolumnrank; graph/weight counters continue after recovery.
### Open threads
Commit/checkpoint/buildsource, execute cohort once, report all M1/M2/M3outcomes.
### Errors
No numerical-gate failure. Unused historical fixture fields produce warnings
only. No scientific stream has run before this source checkpoint.

## Real stream terminal
### What was done
Source353a85552epre-run checkpoint20261115T113606Z-353a85552e4dpreceded the
589824trialcohort. All18full/rooted targets recover matching scalars; all raw
decomposition and snapshot public-label gates pass.72snapshots/36holdouts.
Native jq M2fails exactly one holdout: g2p16381seed16,t32768,744root Full rows
versus367.5945predicted; known/depth forecasts pass. The other35holdouts pass.
Source/ODEgate/raw output/summary hashes retained; no retry or rate refit.
### Decisions made
M1passes/M2fails at registered tolerance. This is a model calibration outcome,
not a timing result. Rooted and full forests remain distinct: g2p16381median
raw outputs673vs8442at32768, while target determination occurs earlier and
rank is a separate quantity. No raw-count ratio becomes a speed claim.
### Open threads
Publish report with actual row weights/rank and the failed forecast; derive
the genus2zero-core boundary as an exact partition-invariance question before
any new performance pilot. All count workers terminal.
### Errors
No scientific gate failed except the declared M2forecast miss. A stray empty
patch request was rejected before modifying any file; corrected path used.

## Assessment and report
### What was done
RESULTS.mdrecords35/36heldout passes but registered M2overall failure, all
raw/count/rank distinctions and final three-target medians. Derived exact
genus2partition invariance for identical raw streams: degree2rows always
have<=2off-base entries, so H0full weighted elimination leaves only target.
The rooted ODE cannot seed that boundary. No H0implementation run claimed.
Native median SVG/TypstPDFbuilt; source/fixture/binary hashes accompany raw.
### Decisions made
Do not use raw emission advantage as10xruntime result or count full rank as
necessary for target determination. A separate H0prefix gate precedes timing;
genus3/4remain in full objective. Root-weight denominator standardized to ALL
emissions before publication; zero empty counts mean numeric values unchanged.
Original derived summary retained. No scientific stream was rerun or refitted.
### Open threads
Inspect PDF, finish verification, commit/update PR/checkpoint and fresh CI.
### Errors
One registered forecast miss retained. An initial scope refresh reported
overlap with the same T-6113task; its expired lease was reacquired at epoch3
for final publication. No competing task owner observed and no old record
or default was changed.

## Final checks
### What was done
Inspected the single-page PDFand native diagram;12release library tests and
scoped formatting pass. Scientific main/core/ODEsources unchanged from frozen
353a85552e. Native reducer retains all forecasts and the exact failed case.
Raw-check totals and raw/summary SHA256files added. Previous count/model
studies remain untouched. No scientific worker live.
### Decisions made
Full/rooted raw row ratios are counts, not10xruntime evidence. The genus2H0
claim is a degree<=2Gaudry-stream partition theorem; it does not apply to the
degree3principal rows of the earlier genus2point sieve or automatically to g3/4.
### Open threads
Commit/publish/checkpoint and follow current CI; then gate that narrowly stated
partition theorem before pricing any implementation against strongest controls.
### Errors
No new scientific error; retained M2miss blocks an all-holdout calibration pass.
