# Canonical N83 source gate before the hosted pair

This evidence gates source commit
`cdaa0a96391084de881064e99ca7a8da0f527370`; it is not a relation-yield
measurement. The input was a `git archive` of tracked source and public
fixtures at that commit. The corrected archive was 35 MiB, SHA-256
`56c9d2cb62d8ad63355961c98a80843815f5447bd078da235e8ea5ce5a0f6bad`.
It contained `Cargo.toml`, `Cargo.lock`, `src`, `examples`, `tests`, the N83
study tree, the referenced curve/IC/ecbench documents and compile-time
research fixtures, plus the prior single-target and tool-program fixtures.
It contained no `.git` directory or untracked local files. The digest-pinned
`rust:1.98.1-bookworm` image ran on isolated Linux x86-64 with eight reserved
CPU cores, 16 GiB requested memory and Rust 1.98.1; see the raw v2 receipt.

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo test --release --locked --lib -- --test-threads=4` | **PASS**, 4,091 passed, 117 ignored, 0 failed; 205.37 s test execution, 355.87 s including clean build | `v2/receipt.json`, `v2/lib.log.gz` |
| `cargo test --release --locked --example koblitz_n83_mixed_orbit_worker` | **PASS**, 6 passed, 0 failed | `v2/receipt.json`, `v2/worker-example.log.gz` |
| N83 Python `unittest discover` in the complete local checkout | **PASS**, 63 passed in 64.126 s | `local-n83-python.log.gz` |
| `rustfmt --check`, `actionlint` and frozen-plan check | **PASS** locally | Source and workflow checks in the parent protocol |

The [corrected isolated run](https://modal.com/apps/a-buran28/main/ap-TU0fvr7C2IR7h6VyLS8n80)
also invoked the Python suite, but its trimmed archive omitted historical
files used by two tests and omitted `.git` metadata used by one test. That
invocation ran 63 tests and ended with two fixture failures and one Git-metadata
error. These are archive-preparation outcomes; the same suite passed in the
complete checkout. Its raw result is retained in `v2/n83-python.log.gz`.

The [first isolated run](https://modal.com/apps/a-buran28/main/ap-NNjLoNvTIaXrJsZKCDsHna)
used a smaller 8.1 MiB archive, SHA-256
`4afab774d1971e03158ecea7145320fff91f4f26adde533677129116820a90d5`.
Its library command stopped at compilation because four referenced public
fixtures were absent. Its worker example passed. Its Python invocation found
zero tests, so its zero exit code is **not** a Python gate. All v1 receipts
and raw outputs are retained separately.

A local full-library attempt on a host shared with multiple other release
builds reached four failures in unrelated `jv_cover` tests whose own 120-second
solver limits censored the cases. One isolated local reproduction completed
only 17 of the required 30 cases before that limit. The already-failing local
suite was interrupted; it is not reported as a pass. The complete isolated
Linux suite above passed all non-ignored library tests, including those cases.
Both new canonical-ordering library regressions also passed separately from
the local release binary, and the local worker example passed all six tests.

For every Modal case, `receipt.json` records the command, source archive hash,
image, exit status, elapsed wall time, raw-output SHA-256 and compressed-log
SHA-256. Independently checking a case means hashing its `.log.gz`,
decompressing it, checking the raw-output hash, and inspecting the final test
summary. The receipts and logs here contain only public-source validation;
they do not select a factor base. `selected_best_total_runtime` remains null.
