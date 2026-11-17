# Frozen first full-D18 four-summand access gate

This protocol is committed with the opt-in worker before its first retained
natural-query run. It uses only the primary Koblitz `a=0` GF(2^83) curve and
the complete, S3-custodied `direct_subgroup_x_d18_full` archive in
[`manifest.json`](manifest.json). The exact object and producer/source hashes
are pinned in [`AWS_RESUME.md`](AWS_RESUME.md). The worker rejects any changed
manifest or object before constructing a model and generically rechecks all
5,418,738 stored point rows, labels and signed-Frobenius orbit maps.

Use fixture `(a=0, fixture=0)` from
`research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json`,
raw SHA-256
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`.
Its public point is
`(1834913121178197242856238, 8797964190732846213981377)`.
The natural worker never reads the known-answer scalar. Retain the exact
S3 GET response, source commit, binary SHA-256, host architecture and image,
memory/cgroup and CPU limits, process stdout/stderr, exit code, UTC interval,
phase/shape/result/cap receipts and raw-file SHA-256 inventory.

Run `primary-full-d18-adapter-check BASE_DIR NEW_JSON` first. Require its
`PASS` result and a fresh independent S3 GET/replay according to
[`AWS_RESUME.md`](AWS_RESUME.md). For the solver, use `m=4`, identity mask 0,
all 32,643 signed-Frobenius representatives, the existing per-representative
orbit-domain encoding, at most 400,000 SAT variables and 12,000,000 domain
clauses. The worker allows at most one SAT model and 100,000 conflicts, with
a 1,200-second internal whole-process cap. The supervisor must impose an
independent 1,210-second outer cap, exactly 32 GiB of hard cgroup memory,
zero swap, one CPU, read-only inputs and no network inside the worker.
Record actual total CNF clauses, XOR rows, model wall and memory peak; the
analytic preflight bound is 329,726 variables and 11,238,656 domain clauses.

Run the constructed `positive` arm first. Its target is the sum of the first
four distinct orbit representatives. Check every partial group sum is affine,
the signed-orbit row replays to that target, and changing the target rejects
the row. Install the same full domain circuit and pin the four summand,
two intermediate and final target x blocks to this checked witness. Require
`PASS_CONSTRUCTED_CONTROL` with a SAT model, original-field S3 check, exact
group lift and rank increment one before interpreting a natural run. Its
solve time is a control, not a natural-yield sample.

Then run `natural` on the public point with the same binary and caps, without
any witness pins. On SAT, verify the original model, all stored x lifts,
original group sum, signed-Frobenius row and rank increment; retain
`PENDING_INDEPENDENT_NATURAL_REPLAY` until a separately built verifier
replays it. `UNKNOWN_search_cap`, `UNKNOWN_wall_cap`, `MODEL_GROUP_REJECTED`,
`UNVERIFIED_UNSAT`, OOM, missing or malformed receipts are censored or
invalid observations, never a zero-yield estimate. Preserve a failed
positive control and do not interpret its dependent natural arm. Keep
`selected_best_total_runtime=null`: this first gate cannot establish a
best base without checked disjoint natural targets, useful-rank growth,
full factor-base log/linear algebra, unseen-target descent, cold one-target
accounting and matched same-target rho.
