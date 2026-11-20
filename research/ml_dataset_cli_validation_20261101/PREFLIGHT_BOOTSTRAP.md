# Preflight gate bootstrap without circular preparation

Read-only review found an existing matching plan/frozen protocol pair:

- `/var/tmp/ml-wide-prime-m3-cli-v4-210e8ea/plan.json` SHA256 `91cdea638aad7d961c1d8c386866d63ef0a096a66b5f71c6d38f2ce37c590665`
- `/var/tmp/ml-wide-prime-m3-cli-v4-210e8ea/protocol.json` SHA256 `fe717e2b8758c15b2c64cca656eb8fd90d23e62902e57ac580a7cad22810b765`
- Canonical protocol body SHA256 independently recomputed: `9a048836d3ff5b17d3a6b578dd09f16860578a62af58ea08f4e30e9385ddf5d3`
- Original freeze time: 2026-10-30T03:34:31Z, filed_before_affected_data true. Saved preflight.exit is zero, and historical preflight.stdout binds both exact input hashes and reports generated_cells=0/scientific_computation_performed=false. Historical binary/source metadata are not evidence of the new binary.

`dataset preflight` validates plan bounds and protocol file/body hashes, then independently observes its compiled source identity and current executable digest. It does not require the protocol instrument commit to equal the current binary. Therefore these historical inputs can serve strictly as parser/preflight inputs for observing current binary provenance. This does not reopen their original experiment or authorize any new generation. `finalize` removes ISOLAB_RESULTS_URI and issues no generate command.

The dependency is acyclic: completed current full-library validation -> adapted library receipt -> current release binary build -> preflight using the existing pair -> finalized gate -> driver.prepare binds and freezes the actual new 46-case software protocol. Retain the historical protocol unchanged. Do not use it for the actual new software generation.

Root can issue the following only after the separate build and library gates succeed. Set LIBRARY_RECEIPT, BUILD_DIR and NEW_GATE_DIR to the real current adapter outputs (the last must not exist); these are explicit parameters, not inferred successes:

```sh
python3 /home/ec2-user/ml-dataset-resume-20261029/cli-gate-receipt-adapter/adapter.py finalize \
  --root /home/ec2-user/crypto-ml-koblitz-m3-adapter-20261030 \
  --library "$LIBRARY_RECEIPT" \
  --build-dir "$BUILD_DIR" \
  --plan /var/tmp/ml-wide-prime-m3-cli-v4-210e8ea/plan.json \
  --protocol /var/tmp/ml-wide-prime-m3-cli-v4-210e8ea/protocol.json \
  --out "$NEW_GATE_DIR"
```

The exact inner command is the newly retained BUILD_DIR/crypto dataset preflight --plan NEW_GATE_DIR/plan.json --protocol NEW_GATE_DIR/protocol.json. No command above was executed in this review. No source write/build/crypto execution occurred.
