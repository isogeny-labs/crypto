# Full-width rejection-sampling candidate

The candidate derives counter-tagged SHA-256 draws from canonical JSON and rejects values outside the requested bound after masking to its bit width. It returns the accepted counter or an explicit finite-attempt error. Existing target laws are unchanged; this source is not registered in ecbench yet. [pcc:unverified]

The standalone native harness reports 32 passing tests, including five sampler regressions, the exact-scalar compatibility tests, canonical encoding tests and the actual SHA-256 implementation tests. Exhaustive one-byte draws check equal accepted multiplicities for every bound1..256. Other regressions force rejection/exhaustion, exercise 256-bit boundaries, and check deterministic domain/input separation. These are unsigned development observations, not a statistical experiment or signed PCC verdict. [pcc:unverified]

Integration needs a new target-law identifier, exact order/field transport, actual Curve25519 dispatch, and preserved historical seed laws. `native-harness.rs` pins this checkout path for the observation; remap the paths to matching source hashes from `manifest.json` for replay. [pcc:unverified]

Replay with `python3 research/curve25519_autolab_parity_20261105/next-integration/exact-sampling/replay.py --target /path/to/locked-cargo-target --out /fresh/output`. The helper checks source hashes first, records the selected dependency hashes and compiler version, then saves build/test exits and artifact hashes. The retained replay reports32 passed/0 failed. [pcc:unverified]
