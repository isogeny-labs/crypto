# Full-cost analysis adapter contract

Execution clarification before implementation/calls, subordinate to the frozen
COMPACT_SPEED_PROTOCOL.md. Its 15x rule, counts, seed, controls, roster and
fresh-confirmation admission are unchanged. This step implements transcript
accounting/statistics only; a process/backend adapter and external execution
admission remain required. No benchmark is launched by the analyzer.

## Sources checked

KB unavailable: no MCP tools; repository wrapper reports no executable.
Fallback: COMPACT_SPEED_PROTOCOL.md, the original Rust campaign.rs interval
routine and the actual-root worker contract. Official SciPy bootstrap docs,
https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.bootstrap.html,
describe shared-index paired resampling and percentile intervals, and warn
that percentile coverage can be less accurate than other methods. Retain the
registered percentile calculation; do not claim exact frequentist coverage or
switch estimators after seeing samples. No Python execution is used.

## Input and accounting

Implement a standalone Rust library/CLI, without changing solver source.
Input is a bounded, typed, versioned single-case transcript: p31, n=3/4/6 and
one of exploratory seeds 101/307/911 only. Fresh seeds are rejected by this
adapter until a separate family-admission layer exists; no fixture is generated.
The transcript has a positive shared mature-validation setup cost, exactly one
warmup per arm, seven A/A pairs per arm and 21 ordered three-arm blocks.
Require complete status, positive worker/validation costs and matching
canonical native/external basis SHA256 strings throughout, including warmups.
These strings and statuses are producer data, not independent mathematical
or execution evidence. Unknown fields, invalid digests, malformed lengths,
roster/order changes, zero costs, overflow and incomplete data are rejected.

worker_wall_ns must span launch through output consumption/admission, not a
worker's internal phase clock. validation_wall_ns includes per-call external
conversion/checking. setup_wall_ns includes original-input mature reference
generation/conversion, charged without amortization to EVERY arm/sample:
full_cost = worker_wall_ns + validation_wall_ns + setup_wall_ns. Retain the
native-only component separately. All charged durations must fit the frozen
two-hour campaign bound. Raw input and process receipts remain required;
this calculator cannot certify their provenance, measurement scope, isolation,
CAS route choice, checkpoint admission or absence of omitted costs.

## Frozen numerical interpretation

Follow the existing campaign's median of paired per-block control/candidate
ratios. Resample complete block indices jointly for both controls and both
cost scopes, never arms independently. SplitMix64 seed 20261115, unbiased
index selection by rejection, 100000 draws. Reset that seed for each case and
each A/A calculation; this fixes determinism before observations. Percentiles
use nearest rank (ceil(N*q)-1, zero based), matching the old 10000-draw 95%
endpoint convention. Compute indices with integer arithmetic: adjusted lower
q=1/720; ordinary 95% endpoints q=1/40,39/40. No rounding before comparisons.
For A/A use the same complete per-pair ratios and include 1 in its 95% interval
for each arm, on full cost; retain native-only A/A diagnostics too.

An incomplete/mismatched/failed-A/A case supplies no main numeric comparison.
Missing remainder remains incomplete, never filled or replaced. An admitted
exploratory case may return a conditional numerical flag only when BOTH
full-cost adjusted lower endpoints are strictly greater than 15. Also require
both native-only A/A controls to include 1, so charging setup cannot conceal a
component-control failure. A/A failure is retained. A passing numeric flag is
not a family result or a signed performance verdict; output always states that
execution admission and independent verification have not been established.

## Development qualification

Use hand-authored synthetic transcript numbers/digests only, clearly labelled
synthetic, and do not call worker or CAS binaries. Check known constant ratios,
strict threshold behavior, paid-cost suppression of an apparent native win,
pairing/order/count/digest errors, capped controls, A/A failure, arithmetic
limits, deterministic resampling and rejection of fresh seeds. Check nearest
rank indices against known order statistics. Build/copy/hash immutable release
qualifiers with separate build/pre/post records. Unit elapsed times are not
performance samples. No fresh target or holdout is generated.
