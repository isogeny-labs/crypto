# Validation and actual invocation outcomes

The refined explicit-one-worker implementation passes the full library run:
4,407 passed, zero failed,118 ignored, exit0. Thirteen standalone
generalized-solver tests, strict Clippy and the example build/tests pass.
The earlier full sandbox invocation remains failed and is retained.
All original failing and interrupted invocations remain in `records/`.

The final full command uses network access, one test thread and one Rayon worker, with
`KIC_GF2_WRITE_TABLE_ONCE=1`, `KIC_GF2_VECTOR_TABLES=1`,
`KIC_GF2_SERIAL_CLEAR=1` and `KIC_ROOTFIND_BATCH_SETUP=1`:

```sh
cargo test --release --lib -- --test-threads=1 --color=never
```

The refined guard receipt is `records/boolean-guard-final-library.log`:
4,407 passed,0 failed,118 ignored, exit0 (1075.15seconds).

The preceding unrestricted-guard receipt is `records/boolean-serial-final-library.log`:4,397 passed,
10 failed,118 ignored, exit101. Nine failures were sandbox-denied local
network binds. The unchanged `gray_ffs_beats_on_step_full_enum_wall` test
checked equal root sets but failed its single-shot wall threshold during
the full run. It is not used to establish any native performance claim.

The affected network module was rerun with network access:

```sh
cargo test --release --lib cryptanalysis::pollard_collab:: -- --test-threads=1
cargo test --release --lib cryptanalysis::mq_fes::tests::gray_ffs_beats_on_step_full_enum_wall -- --exact --nocapture --test-threads=1
```

The same four switches and worker count are retained. The network suite
passes38/38; the exact wall test passes1/1, with equal outputs. Their logs
and zero exits are `boolean-final-network-rerun.*` and
`boolean-final-wall-rerun.*` in `records/`. No source changes were made to
these tests. The interrupted preceding post-merge run ended with signal 143
before completion; its cause is unknown and it supplies no full-suite result.

Other latest checks:

- `cargo test --release --locked --manifest-path tools/generalized-rootfinder/Cargo.toml`:
  13 tests pass; `cargo clippy` for all targets with `-D warnings` passes.
  Receipts: `records/boolean-guard-focused.*`; earlier SDK receipts are also retained.
- `cargo test --release --locked --example f4_f5_boolean_perf` compiles and
  passes (zero example unit tests); independent runtime output validation
  is supplied by the native jobs, not by that zero-test invocation.
- All 10 GF2 reference tests pass in separate processes at one/four workers
  with serial clearing disabled/enabled. All four configurations are repeated
  before timing in the refined native job.
- Latest standalone CLI fixtures complete with exact counts 2 and 8 for the
  Boolean and full eight-coordinate separator fixtures. Original-coordinate
  tuples and certificates are retained in `records/cli-*.json`.
- The native analyzer's two admission/noise tests pass; original optimizer
  analyzer tests and the nine workflow-path tests pass. Leaderboard path
  coverage, regeneration and browser checks have zero exits. The final
  generated checks are rerun after the last native result is added.

Earlier validation is retained separately: a pre-merge serial full run
passed 4,397 tests before the new table-map test was added; the later8-thread
full run had an unchanged Joux–Vitse time-budget failure, whose exact serial
rerun passed. Those receipts are not silently substituted for the latest
candidate invocation.

`records/final-source.sha256` and `records/validation-compiler.txt` identify
the final publication sources/compiler. Native job source and executable
hashes remain in each raw run; later merges and reports do not alter them.
The native performance gates use their own dedicated-host isolation, A/A,
five pairs, exact original outputs and bootstrap admission. Shared-host
unit-test elapsed times are never promoted into that evidence.

Final evidence checks: extraction and deterministic repacking reproduce the
archive SHA256 exactly; all nine derived JSON artifacts replay byte for byte
from the extracted receipts. Every report page is visually inspected (see
PDF_INSPECTION.md), and benchmark PDF/SVG/150dpi PNG copies match.
Leaderboards regenerate and check cleanly, all219 source paths are covered
for both PR/push workflows, nine path tests pass, browser data is current,
and the curve-name checker finds zero problems. All preexisting leaderboard
JSON fields compare equal after excluding the new component/source entry.

Scoped rustfmt checks pass on the modified runtime, SDK, example and native
analysis sources. Repository-wide cargo fmt --all --check fails on the
unchanged examples/n83_precomputed_rho_analyze.rs and
research/conductor_gap_cm_orbits_20261015/native/ecbench_next.rs; both compare
identically to origin/main. The failing global check and passing scoped check
have separate retained log/exit receipts. No unrelated formatting is changed.
