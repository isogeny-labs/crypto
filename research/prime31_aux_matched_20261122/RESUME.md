# Resume: matched auxiliary factor bases on ordinary Prime31

## Goal
Complete the controlled whole-fiber ordinary/SS/Dickson versus matched plain
comparison requested by the user and publish it in PR #4923.

## Lead / hypothesis
Separate representation effects (dense versus retained tower on identical V)
from usable-cardinality effects and construction/seed-selection costs. Use
natural public-target queries, signed identity charts and complete scalar
recovery rather than planted relation corpora or rank-only proxies.

## Spec
Study: research/prime31_aux_matched_20261122. Native panel.json and cells/*.json
are prepared. protocol.json is a draft, not canonically frozen. Read README.md,
PROTOCOL.md and RUN_STATUS.json before attempting execution.

## State
Rust implementation and development checks are in this PR. No Prime31 campaign
cell has launched. Exact target preparation is in target-fixture.json. The full
monolithic root library is outside the isolated native verification claim.

## Next steps
From the published repository root, run the exact cargo test command in
README.md and retain its receipt. Obtain readable current canonical protocol
tooling, validate and freeze the protocol, bind the published source/input
hashes, obtain source checkpoint and current budget headroom receipts, then use
the existing admitted IsoLab command-job route and run-cell.sh. Do not substitute
local performance measurements for the blocked campaign. Preserve all cell
exits and upload event streams with SHA256SUMS to the artifact store, then run
the independent native replay command and create the canonical export.

## Runs in flight
None for this study. Reconcile live backend jobs before starting anything; this
statement does not describe other contributors' concurrent work.

## Stop and success conditions
55 planned cells, serial, zero retries, proposed aggregate cost ceiling USD5,
two CPUs/two Rayon threads and 4096 MiB, 120-second hard cell deadline. Unknown
budget remainder blocks launch. A successful IC cell recovers d and independently
checks dG=Q; all setup failures, unknown query outcomes and caps remain data.
No speed comparison is admitted without a separate L2 and A/A qualification.

## Open questions
Access to the current canonical protocol and authenticated IsoLab/checkpoint/
budget route is absent in this session. Runtime F4 and setup native counters are
partial. The A=1 rho control is not the optimized negation-aware incumbent.
