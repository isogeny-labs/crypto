# Prospective matched complete-selector protocol

Version 1; frozen before any new CAS/time/yield experiment. This protocol is
not an experiment receipt. The only emitted input so far is a known-answer
F17 development fixture. Unit correctness tests are not timed observations.

## Scope and identities

Use the unchanged curve `icv1-fp5-tm1-a77cc6af`,
`y^2=x^3+2*x+2` over F17, and unchanged factor base
`FB1h29cc024e755c`. Its eight x columns are
`[0,3,5,6,7,9,10,16]`; sixteen signed points. Reuse the issuer and FB1 dump
in `../pencil_search_20261101/fixture/`; do not reissue an identity from labels.
The inherited order19/cofactor1/registered=false metadata is not a new theorem.

Arms, with all variables counted:

| Arm | Variables | Equations | Complete choices |
|---|---:|---:|---|
| dense control | 2 | 3 | all recovered signs/lifts |
| linear-bit control | 7 | 8 | four sign pairs, four bits |
| XOR recoding | 7 | 8 | same four signs, invertible bits |
| one-hot | 9 | 16 | six Boolean bits, six support products |
| symmetric chord | 6 | 7 | d explicit, no pivot discarded |
| determinant expanded | 13 | 14 | unit kernel coordinate, all witnesses |

For each finite target the control is one dense system and four sign branches
of each other arm: 21 systems. Folded duplicates stay in cost totals; usable
relations are deduplicated by exact signed group points.

## Instances and completeness

`emit-fixture` exports only target (13,10). `emit-panel` will export all18 finite
F17 targets sorted by SHA256 of the fixed domain `fourtracks-v1|icv1-fp5-tm1-a77cc6af|x|y`
and coordinates (see exact native source encoding). All are public; no ECDLP
secret is generated. No target is chosen after inspecting solver output.
The identity target has a separate exact inverse-point chart and is excluded
from this finite-target CAS panel, explicitly, not charged as an easy solver win.

References enumerate the complete fixed signed base and replay every ordered
pair with exact group addition. Compare the decoded union of each arm's
branches to that reference, including empty targets and repeated coordinates.
Never equate a planted subset with a complete root set. Require zero false
positive and false negative relations for every fully completed target.
Timeouts and crashes remain records with missing completeness, not zero roots.

## Execution and accounting

Default route: EKS Spot, one immutable container digest/source tree and pinned
CAS build on the same host, one thread. Before submission freeze the admitted
host, isolation tier, resource requests, limits, wall-clock budget, CAS version,
monomial order and exact command line. No budget has been spent by this study.
Use a short development panel before any larger fields. Do not extrapolate
from this single field size or fit a meaningless exponent to presentation arms.

Use the unchanged native exporter for identical polynomial inputs. Compare our
complete adapter with the fastest available matching CAS (msolve open F4;
Magma F4/F5 if licensed), same inputs/order/host. Include A/A controls, fixed
interleaved order, at least five fresh processes/arm/target. Keep warm-only
measurements separate from cold setup; charge generation, parsing, every sign
branch, elimination, reconstruction, group replay, failed attempts and writes.
Check exact decoded root sets, not only leading monomials or determinant values.
Record every F4 degree reached, rows/columns/nonzeros per matrix, peak live
monomials, rank/zero reductions, field arithmetic and group operations, RAM,
bytes read/written, process/cold wall time, and unavailable counters as null.
Bytes measure storage/traffic and are never silently converted to operations.

Canonical first-vanishing degree, observed affine F4 solving degree, saturated
chart degree and block/multihomogeneous bounds are distinct fields. Attribute a
finite canonical degree only to the actual complete generator set with its
variable inventory. A Schur-eliminated arm is a separately priced arm.

## Admission and stop rules

No timing result is admitted without complete sources/raw logs, source hashes,
root references, exact replay, independent number recomputation and required
controls. No local algebraic bound alone promotes a leaderboard performance row.
Do not discard a chart or probabilistic branch and label its complement searched.
If correctness fails, preserve the input/output and stop that arm pending a fix.
Stop expanding if complete branch-summed work increases against the frozen
linear-bit control. Preserve null/negative results with the obstruction.

Larger-field or 17-term/P-256 protocols require new exact fixtures, growth
parameters, rank accounting and budgets. Original full-depth gates are not
evaluated from this two-summand fixture.

## Actual integration status

No new CAS panel has been launched. The connected tool catalog has no arbitrary
EKS submission tool; the managed session has no ready AWS identity. The current
GitHub isolab workflow builds ecbench/fresh-owner binaries and has no adapter
for these standalone CAS files. This is an integration gap, not a failed job or
a measured infrastructure failure. Next step: connect the fixed exporter and
complete root decoder to the admitted EKS/CAS runner, freeze the missing host
and budget fields, then submit and independently replay the panel. No previous
study's narrow local exception applies here.
