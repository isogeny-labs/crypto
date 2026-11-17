# Same-host N83 K2048/K4096 bounded stage control

The frozen [ABBA protocol](PROTOCOL.md) completed on one native x86-64
`c7i.xlarge` host with one release binary build and four sequential Docker
arms. The two signed-Frobenius factor-base archives passed SHA-256 checks
against their frozen S3 objects. All four arms independently replayed the
**same** public fixture-zero relation query, entered SAT, and returned
`UNKNOWN_solver_cap` at exactly 1,000 conflicts. Each arm had zero SAT
models, zero relation rows, and observed matrix rank zero. These four capped
queries measure bounded stage cost; they do not estimate natural relation
yield or complete index-calculus runtime. `selected_best_total_runtime` is
null.

| Arm | Base | Base import | Model ready after decomposition entry | SAT interval | Container wall | Peak cgroup memory |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| A1 | K2048 | 3.604 s | 309.275 ms | 55.517 s | 61.026 s | 484,773,888 B |
| B1 | K4096 | 7.469 s | 603.725 ms | 127.103 s | 137.953 s | 752,508,928 B |
| B2 | K4096 | 7.460 s | 600.394 ms | 115.414 s | 126.215 s | 752,574,464 B |
| A2 | K2048 | 3.628 s | 309.180 ms | 55.778 s | 61.180 s | 484,630,528 B |

The two-run mean container wall was 61.103 seconds for K2048 and 132.084
seconds for K4096, a 2.162 ratio **for this one public query and 1,000-conflict
cap**. The K2048 repeat spread was 0.154 seconds; K4096's was 11.738 seconds.
The two-run mean SAT interval was 55.647 seconds and 121.258 seconds,
respectively. These are descriptive paired observations, not a speed claim
for finding sufficient relations. Both bases had zero observed useful rank
under this narrow cap.

The whole runner job took 866 seconds from recorded start to done, including
checkout, input staging, the shared compilation, Docker setup, all four
containers, and output upload. The runner's separately recorded command
wall was 12:50.12, and the four container walls summed to 386.374 seconds.
The job used a two-hour TTL, one CPU/4 GiB/zero-swap/no-network per container,
and a pinned image ID `sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63`.
The job exited 0 and instance `i-0fa3bc013cdf16219` was verified
terminated. The exact source commit was
`2b6bbb94998e562dd9bc0b623e5a9c803892cf71`; the compiled binary SHA-256
was `5809186cfccdffd516d90372cc6e9da501b0e923289e7bab08a51d637bb41f4b`.

The independent [audit](completed-run-audit.json) verifies the pushed source
snapshot, both archives and all panel/fixture sidecars, image identity,
resource limits, exact model dimensions, four terminal phase streams,
same-query group replay, attempt counters, and wrapper wall receipts. It
rejects altered public-query x, terminal conflict count, and archive byte
([controls](mutation-controls.json)). Compact [native receipts](native/) and
their SHA-256 manifest are committed; the full raw runner output, including
both 10–20 MB factor-base archives and source snapshot, remains at
`s3://ephemeral-runs-104186811093-us-east-1/jobs/20261019-023945-n83-k2048-k4096-abba-5fcee8/`.
The long-lived factor-base originals remain in the
`crypto-autoresearcher/factor-bases/icv1/etc` S3 prefix.

This comparison does not charge the earlier factor-base constructions or
measure enough natural queries to estimate relation yield, useful rank,
linear algebra, column logs, individual target logs, scalar verification,
or matched rho. Those gates remain necessary before selecting the base that
minimizes total index-calculus runtime.

Final branch verification passed `cargo test --release --lib --locked`
(4,391 passed, 118 ignored), the touched N83 example suite (19 passed),
the N83 Python suite (63 passed), and the boundary Python suite (52 passed).
The first library attempt passed 4,382 tests but nine unrelated loopback
network tests failed with sandbox `Operation not permitted`; the identical
suite passed when rerun with local socket access. Both exact raw attempt logs
are retained here as deterministic gzip files.
