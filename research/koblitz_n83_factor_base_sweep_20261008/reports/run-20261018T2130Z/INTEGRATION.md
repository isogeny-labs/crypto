# Post-merge native integration validation

Runtime source: merge `943acd489aa413608c226558e3c56b3bdd1bad97`.
The runtime paths (`src`, `examples`, Cargo manifests/lockfile,
`isogeny_algos`, `tools/volcano-cli`) match upstream PR head
`08c306906144c2cee56297ae6820bc176dd681ff`; subsequent report commits
change documentation only. This validation does **not** supply timings or
counters for the older frozen experiment worker.

The serial native library suite completed with **4397 passed, 0 failed,
118 ignored**, in the runner's recorded **960.43 s**. All **19 exporter
tests passed**, 0 failed/ignored, recorded **0.86 s**. The outer tool returned
**143** despite both complete passing summaries. Its cause is not established;
we do not claim a captured zero exit for that combined launcher. A short
independent exporter confirmation captured exit **0**, with all 19 tests
passing. There were no assertion failures reported in the completed suites.

Commands (scratch target prevents replacing the experiment worker):

```sh
env CARGO_TARGET_DIR=/home/ec2-user/scratch/n83-docker-sweep-20261018/integration-target \
  RAYON_NUM_THREADS=1 cargo test --offline --locked --release --lib \
  --example koblitz_n83_factor_base_export -j 2 -- --test-threads=1
```

The first combined attempt started at 22:24:40 UTC with a 1200-second
build-and-test limit. Compilation consumed the recorded `15m 11s`; it was
intentionally stopped (143) and preserved so tests could have their own
allowance. The cached restart began 22:41:01 UTC with a 1800-second limit,
completed compilation in 0.28 s, and retained both full passing summaries.
Serial execution follows the existing documented parallel JV deadline issue;
assertions and ignored-test settings were not changed.

Logs remain local under
`/home/ec2-user/scratch/n83-docker-sweep-20261018/validation/`:
`integration-lib-and-example.log`, `integration-native-serial.log`,
`integration-exporter-confirm.log`, `integration-exporter-confirm.exit`.
Hashes are in [integration-receipt.json](integration-receipt.json).
These later integration logs are outside the unchanged, pending-approval
raw experiment ZIP. No broader raw upload occurred.

The frozen experiment-source validation remains independently catalogued in
`verification/cloud-matched-k2048-k4096-20261018/VALIDATION.md` and its sealed
manifest. No historical Python research suites were rerun after the updated
native-only policy arrived. New Python use was limited to the explicitly
permitted plotting/rendering/PDF-inspection exception.
