# N83 Colima bootstrap compilation — run report

**Run ID:** `run-20261116T0036Z`. **Status:** bootstrap compiler gate passed; the charged build, worker, relation search and cold one-target comparison remain pending. The [frozen protocol](../../verification/k600-colima-bootstrap-20261116/PROTOCOL.md) and [launcher](../../verification/k600-colima-bootstrap-20261116/launch.sh) were committed at `0f558a0f7998416a9db0924ba9425d69e9c0a7aa` before execution. The source commit remained clean and unchanged during the run.

## Observed execution

| Quantity | Observation | Evidence |
| --- | ---: | --- |
| Outer interval, UTC | 2026-11-16 00:36:41–01:00:47 | [outer receipt](outer.receipt) |
| Derived whole interval | 1,446 s = 24 min 6 s | End minus start, both second-resolution timestamps |
| Cargo-reported build interval | 23 min 19 s | [raw Cargo log](bootstrap.log) |
| Child / Docker exit | 0 / 0 | [child receipt](child.receipt), [container inspect](container.inspect.json) |
| Docker `OOMKilled` | `false` | [container inspect](container.inspect.json) |
| Hard memory / swap / CPU / PIDs | 12 GiB / zero swap / 4 CPUs / 512 | Launcher checks cgroup values; Docker inspect records 12 GiB memory and swap, 4 CPU quota, 512 PIDs |
| Cargo command | `cargo build --release --locked --no-default-features -j 1 --example n83_k600_m6_linux_build` | [launcher](../../verification/k600-colima-bootstrap-20261116/launch.sh), [raw log](bootstrap.log) |
| Source commit | `0f558a0f7998416a9db0924ba9425d69e9c0a7aa` | [source record](source.commit), [outer receipt](outer.receipt) |
| Toolchain image ID | `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573` | [launcher](../../verification/k600-colima-bootstrap-20261116/launch.sh) |
| Output producer SHA-256 | `5d7dea0ae6afea06528b5680652f83e1a7f3309a9ce3241c607976a20213edb7` | Raw log and independent host `shasum -a 256` |
| Output producer format | ELF 64-bit, Linux ARM aarch64 | Host `file` inspection |
| Raw log | 16,367 bytes, SHA-256 `9c49b9b8c78ff976d3112f93d4082e0fb71358240fd4f267282fa8e1a798089c` | [child receipt](child.receipt) |

The source was mounted read-only at `/Volumes/SSD990-2/tmp/n83-native-rank-query-wt`, alongside its actual Git common directory and alternate object store. The producer binary contains that manifest path, confirmed with `strings -a` and exact path extraction. `git status --short` was empty after the run, and `docker ps -aq --filter name=^/n83-colima-bootstrap-20261116-attempt-1$` returned no container. The output and target directory remain at `/Volumes/SSD990-2/tmp/n83-colima-output/bootstrap-20261116-attempt-1`; this absolute path is a local build cache, not durable custody. The committed source, lockfile, image ID and raw log permit rebuilding the binary. No AWS credentials or Docker socket were mounted for this compile.

The log contains four Rust library warnings and no compiler error. The source-linked [diagram](bootstrap-flow.svg) shows the measured setup step and the still-gated charged build. [DOT source](bootstrap-flow.dot) and the [inspected PDF](run-report.pdf) are retained with this report. There is one completed compiler run and no comparable second arm, so the diagram replaces a comparison chart. No canonical IC graph changes: no relation yield, rank, target descent, matched rho, or full cold `S` was measured here.

## Cost and decision

The measured 1,446-second bootstrap interval is setup, separate from the planned charged five-binary/solver build. It must be accounted for in a cold one-target pipeline unless a frozen comparison protocol explicitly prices reusable setup. This bootstrap result clears only the native producer compilation gate. The next gate is the plan/archive/container admission followed by the charged build under its unchanged stage caps; failure there blocks worker execution. The original factor-base sweep and runtime minimization remain unresolved.
