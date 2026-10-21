# Bridge cloud run repair

[Run 39091079969](https://github.com/aburan28/crypto/actions/runs/39091079969)
is retained as a failed first attempt. Three strata stopped in the broad
library suite when two unrelated `jv_cover` wall-clock-limited tests timed out
on shared runners. The fourth built the focused examples but stopped in host
preflight because the runner did not expose `/sys/fs/cgroup/memory.max`.
None of the four strata produced a completed IC/rho pair.

The revised workflow runs the focused IC, rho, fixture, and analysis tests
without repeating the repository-wide library suite in each stratum. It
creates the artifact directory before building and records the visible cgroup
paths. The native producer writes a `REJECTED_CGROUP_TELEMETRY` preflight
receipt if the CPU or memory cgroup files needed for the Tier 1 gate are
absent. The analyzer runs only when the producer wrote its complete event
ledger. A rejected runner remains an infrastructure result and does not
enter a timing aggregate. The frozen public points, candidate definitions,
IC source digest, caps, and same-point pairing remain as in `protocol.json`.

The next [cloud attempt](https://github.com/aburan28/crypto/actions/runs/39094407077)
exposed a narrower infrastructure issue in stratum 2. Its recorded
`/proc/self/cgroup` member was
`/system.slice/hosted-compute-agent.service`; the revised probe still read
only the cgroup mount root, where `memory.max` and `memory.events` were absent.
Its `preflight.json` correctly recorded `REJECTED_CGROUP_TELEMETRY`, with zero
completed pairs in that stratum. Strata 0 and 3 later recorded the same
rejection, and stratum 1 was cancelled before runner allocation to release
the matrix's concurrency slot. The [second-attempt receipt](second_run_39094407077.json)
pins all three preflight hashes and artifact names. The producer now resolves
the process's actual cgroup member before reading CPU and memory counters.
