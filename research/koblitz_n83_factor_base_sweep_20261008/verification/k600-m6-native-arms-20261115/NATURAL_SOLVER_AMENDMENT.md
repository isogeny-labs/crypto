# K600 m6 natural solver admission and bounded result

Status: **implemented and locally tested; no real natural K600 solve run**.
This follows the [natural XCNF export gate](NATURAL_EXPORT_AMENDMENT.md)
and keeps the frozen fixture-0 target, full K600 base, m6 circuit, solver
parameters and arm order.

The Rust `solve-natural` command requires the exact source-bound natural
XCNF export receipt plus terminal passing pinned and unpinned native replay
controls. The natural export must bind the unpinned replay-stage SHA-256.
The command rechecks the K600 panel, preflight, source, image, binaries,
natural XCNF and both positive controls before and after the solver child.
It rejects an output directory inside any input or control directory.

The solver runs one CryptoMiniSat invocation in Docker with 4 GiB hard
memory, no additional swap, one pinned CPU, 64 PIDs, no network, read-only
root and input, seed `20261115`, one thread, 10,000 conflicts, model output,
an internal wall cap below the outer guard, and verified container cleanup.
Exit 10 is `SAT_PENDING_XCNF_REPLAY`; it is not an accepted relation until
the original XCNF and group equation pass native replay. Exit 20 is
`UNSAT_NATURAL_QUERY`; solver cap, wall cap, resource exit and producer or
cleanup failures retain separate statuses. The receipt includes raw stdout
and stderr hashes, the unpinned replay-stage digest, limits and wall fields.
It leaves total IC runtime and best-factor-base selection null.

Local macOS arm64 Rust 1.99.1 release example tests passed **12/12**; the
production release example build exited 0. The
[test](natural-solver-test.log.gz) and [build](natural-solver-build.log.gz)
logs retain exact output. The new negative test rejects a natural solve
without both constructed controls before creating output. The real Linux
Docker path and natural solver result remain unverified.

This step made no S3 request, image build, preflight, XCNF export or solver
invocation against the retained K600 input. The separate proposed
3,600-second charged K600 allowance and exact future astronomy-store writer
and destination remain pending. Native replay of a natural SAT model,
useful rank, target descent and full cold one-target runtime selection remain
open. No performance chart changed because this step produced no K600
measurement.
