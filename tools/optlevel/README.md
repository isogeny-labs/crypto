# optlevel

How close does an implementation run to what its machine can do? Definition,
formulas, tiers and failure modes: [`docs/OPTIMIZATION_LEVEL.md`](../../docs/OPTIMIZATION_LEVEL.md).
How to use it on a kernel: the `optlevel` skill
([`.agents/skills/optlevel/SKILL.md`](../../.agents/skills/optlevel/SKILL.md)).

A standalone Rust crate (not a member of the repository's root crate). No
Python, at run time or build time.

```sh
tools/optlevel/run.sh probe            # builds on first use, then runs the binary
cargo build --release --manifest-path tools/optlevel/Cargo.toml
cargo test  --release --manifest-path tools/optlevel/Cargo.toml   # 64 tests
```

| path | what |
|:--|:--|
| `src/score.rs` | the scoring arithmetic, tier rules and the schema validator (pure functions over JSON) |
| `src/kernels.rs` | the calibration and control kernels, one variant per instruction set (`avx512`, `avx2`, `scalar`, `base`), selected at run time; run as `optlevel kernel NAME` |
| `src/host.rs` | host identity, usable CPUs (affinity mask and cgroup quota), PMU probe, load conditions |
| `src/run.rs` | running a command or k pinned copies of it; the `optlevel.ops/v1` protocol; `perf stat` output |
| `src/cmd.rs`, `src/cli.rs` | the subcommands and the command line |
| `src/plot.rs` | the roofline SVG |
| `kernels/optk.c` | the C reference kernels the Rust ones were ported from; built only by `calibrate --c-kernels` |
| `kernels/gpu_calib.cu` | the CUDA device ceilings; built with `nvcc` at first use by `gpu calibrate` and `gpu probe` |
| `schema/optlevel.record.v1.schema.json` | the record schema (embedded in the binary) |
| `registry.json` | hot kernels across the three repositories and how each counts operations |
| `tests/` | `scoring.rs` (the 33 tests of the first, Python, version), `golden.rs` (committed measurements re-scored to the committed records), `fixes.rs` (CPU quota, GPU memory roof, `gpu probe`, the command line end to end on stub tools), `kernels.rs`, `formats.rs` |

Dependencies, each for one reason: `serde`/`serde_json` (every input and
output is JSON; `float_roundtrip` so that re-scoring is bit-reproducible),
`sha2` (file and source hashes, the host fingerprint), `regex` (`--ops-regex`
is a user-supplied pattern), `libc` (CPU affinity, child CPU time, signals).
The command line is parsed by hand.

The golden tests read `research/optlevel_20261117/` and
`research/hky_pod_runs_20261117/results/`; a sparse checkout must include them.

History and evidence for the port from Python:
[`research/optlevel_rs_20261117/README.md`](../../research/optlevel_rs_20261117/README.md).
