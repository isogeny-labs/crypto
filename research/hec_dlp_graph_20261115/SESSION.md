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
