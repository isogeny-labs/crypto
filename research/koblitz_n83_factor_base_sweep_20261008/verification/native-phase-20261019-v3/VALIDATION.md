# Native validation scope

The measured-core full release command uses the preserved executable
`crypto_lib-565bcb20cb4d5bb4`, SHA256
`b0c7f64f77fbbb59534864255521778055cadb9a3675f93ad9f53f36eebd3e03`.
It exits 0 explicitly: 4397 library tests pass,118 ignored; 19 exporter tests pass;
fixed-witness example 0 tests. The selector example's later locator test
passes separately, and the final native runner/card suite has 13 passing tests.
The full command is:

```sh
CARGO_TARGET_DIR=<scratch>/integration-target RAYON_NUM_THREADS=1 \
 cargo test --offline --locked --release --lib \
 --example koblitz_n83_factor_base_export \
 --example koblitz_n83_fixed_witness_control -j2 -- --test-threads=1
```

**Metadata qualification:** `validation.json`'s `source_commit` is the
27d17bd applicability snapshot, not a captured whole-checkout HEAD at the
start of that long compile. The exact start HEAD was not recorded. The
library core's last change is 27ac11a75; native Git comparison confirms that
core, primary adapter and exporter source are identical through 27d17bd.
The frozen raw JSON remains unchanged; this addendum makes that distinction
explicit. Later native runner/control edits have their own tests. Do not
attribute the original full run to the subsequently merged core.

Integration 0dbd1fdf has additional upstream query progress events and replay
assertions. It gets a fresh, separate full native release run with all three
relevant examples, `--test-threads=4`, an explicit exit file and separate log.
See INTEGRATION.md for the final disposition. No measured worker binary is
replaced. Python research suites/harnesses/auditors were not run here under
the native-only rule; historical upstream logs remain separately preserved.
