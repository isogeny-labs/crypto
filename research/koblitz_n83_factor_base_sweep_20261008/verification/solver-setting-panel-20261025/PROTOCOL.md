# Primary N83 exact-XCNF solver-setting screen

This frozen screen tests whether a two-axis CryptoMiniSat setting change
finds a model on the already audited primary `a=0` Koblitz GF(2^83)
natural-query formulas. It is a continuation of the
[24-arm panel](../natural-yield-panel-20261025/RESULT.md), whose every arm
reached its 300-second time cap. The exact eight inputs cover public trials
1 and 2, mask 0, hash and Gray residual pools, and 5+1 and 4+2 splits.
The separate [local input preflight](fetch-preflight.json) re-extracted all
eight XCNFs from the two immutable, independently audited provider ZIPs and
matched their frozen SHA-256 digests and export metadata. The hosted
[`fetch.py`](fetch.py) repeats those checks after downloading each ZIP on
its Linux ARM host. The original source artifacts were freshly fetched from
S3 during the 24-arm producer run; this screen reuses the byte-pinned XCNFs
to isolate solver settings. It does not charge a fresh factor-base build or
claim a complete cold runtime.

[`protocol.json`](protocol.json) fixes the 2×2 settings:

| Setting | Branch strategy | Maximum Gaussian matrices |
| --- | --- | ---: |
| `base` | `vmtf+vsids` | 10 |
| `vsids` | `vsids` | 10 |
| `matrix48` | `vmtf+vsids` | 48 |
| `vsids_matrix48` | `vsids` | 48 |
| `base_repeat` | `vmtf+vsids` | 10 |

Each exact formula is one host stratum. Its five arms run sequentially in
the displayed order, so the baseline is repeated at the end as a host-drift
control. The solver is the same pinned CryptoMiniSat 5.11.15 Linux ARM
binary as the earlier panel. Each arm has one thread, fixed random seed 0,
Gaussian auto-disable off, 25,000 maximum matrix columns, 2,000,000
conflicts, 600 solver seconds, a 630-second process watchdog, and an 8 GiB
address-space cap. The workflow repeats a positive wide-XOR solver smoke
before search. Its receipt records exact input/solver hashes, raw stdout and
stderr, status, conflicts, CPU, RSS, wall, OOM delta and watchdog state.

The outcome of interest is a model that independently replays against the
original curve-group equation. All five named settings run even if a solver
returns `SATISFIABLE` or `UNSATISFIABLE`, preserving the finite factorial
comparison. A retained SAT model stays `PENDING_INDEPENDENT_GROUP_REPLAY`;
it must be checked using a separately built pinned worker and a fresh S3
archive fetch before it counts as a relation. An UNSAT result is pending an
independent certificate. A malformed stream, watchdog or OOM stops the
stratum as such. If all five arms reach `INDETERMINATE`, that stratum is a
censored screen.
Conflict throughput and solver wall under censoring are diagnostics only;
they do not rank factor bases or predict time to a verified relation. The
eight formulas have different domains and run on different hosts, so any
cross-formula timing comparison is descriptive. The baseline repeat exposes
some within-host drift but is not a variance estimate.

The finite 2×2 setting cross is exhaustive only for these two named axes and
their named values. It does not cover other solver versions, schedules,
symmetry compositions, WDSat/FES adapters or the wider N83 factor-base
grid. A total-runtime winner still requires observed natural relation yield,
graph closure and rank, verified factor-base logarithms, unseen-target
descent, fully charged one-target cold accounting, and matched same-target
rho. Until then `selected_best_total_runtime` remains `null`.
