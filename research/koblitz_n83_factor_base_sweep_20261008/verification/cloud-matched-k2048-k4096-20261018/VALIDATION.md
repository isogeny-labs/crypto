# Native continuation validation

- Complete release library suite: **4,091 passed**, zero failed, 117 ignored;
  test execution 324.65 seconds.
- Release exporter example: **19 passed**, zero failed; 1.14 seconds.
- N83 Python discovery: **63 passed**, zero failed.
- Boundary Python suite: **46 passed**, zero failed.
- Independent stage custody/count/resource audits: **six passed**.
- Final forged-count/forged-success controls with matching recomputed hashes:
  **six rejected as required**.
- Exact K4096 reconstruction, every-point generic replay, fresh publication
  round trip and both fresh input downloads: PASS.

Rust validation used `cargo test --offline --locked --release --lib --example
koblitz_n83_factor_base_export -j 2 -- --test-threads=4` with loopback access,
a dedicated target directory and a 600-second outer guard. The guard did not
expire. Runtime source is unchanged from the tested native worker build;
experiment commits add protocols, evidence tooling and receipts only.

The retained [validation receipt](validation.json) records full local log
hashes and paths. Logs remain in the prepared local ZIP; automatic approval
review rejected its broader S3 upload. No complete external-host replay is
claimed while those raw artifacts await approved transfer.

Three harness issues were preserved and corrected without changing actual
experiment evidence: a historical K2048 rank-history width in a forged K4096
template, a short/long control report-name collision, and a final replay that
omitted the task-local sudo Docker wrapper. The malformed template was
rejected before the intended scalar/rank check; output-name failures preserved
prior reports; the default Docker client was denied socket access. Corrected
final controls all pass with their expected rejection reasons.

No algorithm, throughput, full cold-runtime, fastest-tool or IC-vs-rho gain
is claimed. The source model/protocol and validation facts are distinct from
the still-missing natural-rank and full-cost measurements.
