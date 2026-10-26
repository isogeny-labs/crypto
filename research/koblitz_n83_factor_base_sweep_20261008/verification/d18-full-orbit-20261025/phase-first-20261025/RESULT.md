# Complete-D18 free-choice planted control with phase-first search

The [preregistered phase-first policy](../PHASE_FIRST_PROTOCOL.md) was built
from clean commit `57c20963924ab0097111b08fc6cb157400e87e53` with worker
SHA-256 `064699dfd6fcf19592f7f0de7b82fb816426bb3be27bc2766757296d9a58222d`.
A fresh GET rehashed the same complete 32,643-column, 5,418,738-point S3
archive and verified its provider checksum and producer-manifest binding.
The public primary `a=0` curve, four planted point indices `[0,166,332,498]`,
summand/intermediate x pins and 100,000-conflict budget match the prior
shared-root free-choice constructed control. No representative selector or
Frobenius phase was fixed. The source change prioritizes the 28 phase wires
and initializes their saved decision polarities to false; it adds no formula
constraint. Both runs reported the same 329,726 variables, 2,807,752 total
CNF clauses and 3,901 XOR rows as the earlier shared-root control.

| Retained run | Worker status | Full import | SAT search | Conflicts | Worker wall | Supervisor | Max RSS |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: |
| First | `PASS_CONSTRUCTED_CONTROL` | 74.162 s | 1.121 s | 0 | 76.200 s | exit 1, time-wrapper error | unavailable |
| Corrected repeat | `PASS_CONSTRUCTED_CONTROL` | 77.410 s | 1.584 s | 0 | 79.903 s | exit 0, outer cap not reached | 3,861,577,728 B |

The first worker wrote a complete checked result, but sandboxed macOS
`/usr/bin/time -l` failed when reading `kern.clockrate`, so its supervisor
record is invalid. The [contemporaneous correction note](supervisor-correction.txt)
froze a repeat with the same binary, S3 input, fixture, mode and caps using
local permission for the time wrapper. Both raw `first/` and `repeat/`
directories are retained; their times are reported separately rather than
pooled or reduced to the faster value. The corrected repeat recorded 81.55 s
supervisor real time and 5,311,271,088 B peak memory footprint on the
[shared L0 host](host.json), without a hard cgroup.

The [independent auditor](audit.py) rehashed the S3 object and executable,
checked provider and manifest bindings, recomputed the four archived points'
subgroup membership and target sum with separate Python GF(2^83) arithmetic,
rehashed the sparse relation row, and checked both worker/source/phase/shape
and supervisor records. Its [output](audit.json) is
`PASS_TWO_WORKER_GROUP_REPLAYS_ONE_SUPERVISOR_VALID`. Four altered-receipt
[controls](negative-controls.json) were rejected by
[`negative_controls.py`](negative_controls.py). The [SHA-256 inventory](sha256-inventory.json)
binds every retained small file. To replay, follow [AWS_RESUME.md](../AWS_RESUME.md)
for the exact S3 GET, build the example at the pinned source commit, install
`blake3==1.0.8`, and run `PYTHONPATH=<dependency-directory> python3 audit.py
OBJECT BINARY`.

The previous free-choice shared-root planted control reached its 300-second
cap. This phase-first result clears a constructed-control search gate for an
opt-in encoding and search policy. It is a single public planted relation;
the search times do not measure natural relation yield, useful rank across
targets, factor-base logs, descent or matched rho. A Linux hard-cgroup
positive and a separately frozen natural-query panel remain the next gates.
`selected_best_total_runtime` remains `null`.
