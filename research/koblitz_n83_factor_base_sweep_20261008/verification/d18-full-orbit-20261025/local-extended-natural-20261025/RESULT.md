# Full-D18 extended local natural screen

The [frozen protocol](PROTOCOL.md) ran from clean source commit
`a79536d6605216d258771dffba9cee94072363a5` and one release executable
with SHA-256
`ee2fd534ed952f512577440ea6b2adb0c0fab8a35a25ea0da6d9991a83a6e645`.
A fresh local S3 GET passed provider SHA-256 checksums, byte lengths,
server-side-encryption metadata, and the worker's full signed-Frobenius
point-row import. The complete D18 object has 32,643 signed-Frobenius columns,
5,418,738 point rows, 160,747,529 compressed bytes and SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`.
The hash-bound S3 wrapper SHA-256 is
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`.

| Arm | Audited status | Import | Model | Solver or post-model window | Worker wall | Maximum RSS |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Constructed control | `PASS_CONSTRUCTED_CONTROL` | 88.978 s | 0.150 s | 0.793 s solver, zero conflicts | 90.410 s | 3,295,657,984 bytes |
| One public natural query | `UNKNOWN_wall_cap` | 79.071 s | 0.294 s | 1,120.361 s observed after model; no solver return | 1,200.004 s internal cap | 3,028,795,392 bytes |

Both arms built the free-choice shared-root `m=4` model. The natural shape
was 329,726 variables, 2,807,254 clauses and 3,901 XOR rows. The
[independent audit](final-audit.json) replayed the planted four-point sum in
the original field, its signed-orbit coefficient row and rank one, then
rejected altered-target and altered-row-digest controls. It checked the
natural worker's exact source and binary, S3 receipts, model shape, internal
1,200-second cap, exit 124, and non-triggered 1,210-second supervisor. The
supervisor measured 1,201.105 seconds of outer wall for that arm. A changed
cap value was rejected by the [negative control](negative-controls.json).
Rerunning the auditor against the retained raw files reproduced the saved
audit byte for byte.

The first local constructed attempt is retained in
[`attempt-0-invalid-wrapper`](attempt-0-invalid-wrapper): the worker returned
a checked result, but macOS `/usr/bin/time -l` could not read
`kern.clockrate` inside the sandbox. Its supervisor exited 1 without RSS,
so it was not used as the gate. The source-matched pair above ran with a
working host resource wrapper in a fresh directory. This L0 host had no
hard memory cgroup or CPU isolation; stage walls are exact charges for this
run, not an isolated throughput comparison against the separate 8 GiB Linux
protocol.

The [validation receipt](validation.json) binds compressed raw logs:
`cargo test --release --lib -- --test-threads=2` passed 4,165 tests with 117
ignored; the touched example passed 19 tests; the N83 and boundary Python
suites passed 63 and 52 tests; formatting and Python compilation passed. An
initial sandboxed library invocation had nine existing localhost socket
tests denied by the sandbox; the socket-enabled full rerun above passed.
The complete raw arm and provider metadata receipts are retained beside this
file with a SHA-256 inventory.

The public natural query reached its whole-process cap before the solver
returned, so this is a censored single-query observation. It supplies no
natural relation-rate or rank estimate. Factor-base logs, unseen-target
descent, complete cold one-target cost and matched same-target rho still
need measurement before choosing a total-runtime winner.
`selected_best_total_runtime` remains `null`.
