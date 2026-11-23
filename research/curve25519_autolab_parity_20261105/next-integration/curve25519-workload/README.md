# Actual Curve25519 workload candidate

This candidate reuses Curve25519Wide and Fp256. It represents points in the backend's short Weierstrass coordinates, uses exact scalars, checks field ranges before parsing into the backend, checks subgroup membership, and rejects mutations of the canonical curve/generator. Named planted/public target laws use the separate bounded rejection-sampling candidate; historical target laws are unchanged. Public targets are lifted from field abscissas with a domain-separated sign, cofactor-cleared and identity-rejected, without a known logarithm. [pcc:unverified]

Five standalone native tests passed: full-width scalar/point round trips, malformed/out-of-field/torsion rejection, mutated-curve rejection, deterministic planted construction, and deterministic public construction with subgroup checks. The harness imports the actual curve and Fp256 sources, an extracted ledger/trait, and an unused panic-only Fp64 primality shim. It is not actual-crate integration or a performance measurement. [pcc:unverified]

The first harness build failed because automatic cache selection chose num-bigint without the rand feature needed by the imported field tests. Attempt2 selected the unique rlib with the repository's rand feature; both build commands/logs are retained. The filtered adapter tests do not execute the Fp64 shim. [pcc:unverified]

Next: wire these functions into a dedicated full-size CurveSpec/Instance variant, widen CurveFacts and workload scalars, preserve legacy record bytes/target laws, and connect exact solver dispatch. No full-size workload is registered by this candidate alone. [pcc:unverified]

Replay with `python3 research/curve25519_autolab_parity_20261105/next-integration/curve25519-workload/replay.py --target /path/to/locked-cargo-target --out /fresh/output`. The helper verifies source and extraction hashes, selects the num-bigint rand-feature artifact, and records dependency hashes, commands and terminal exits. Its fresh replay reported5 passed/0 failed. [pcc:unverified]
