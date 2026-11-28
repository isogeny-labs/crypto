# Amendment 1: selected-tower F4 oracle parity

This amendment is frozen before any corrected-binary held-out cold target arm.
It supplements `PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md` without changing its
six curves, 96 target scalars, relation seeds, arm order, limits, cost gates,
or promotion rule. Preserve the original protocol, map-preflight receipts,
release binary, and initial cold-store header and busy-host receipt.

The original source `a0642f42e205aba4951dcf9732721d62211d7c33` constructs
the selected equal-size `X1(2,8)` factor base, but its F4 oracle reconstructs
a generic seeded auxiliary tower. A development-curve one-trial invocation
was skipped before relation collection because selected base abscissa 392 was
not in that other tower. Its F4 candidate arm is invalid as a relation
measurement. The original six-curve map preflight remains a map-construction
check; its source and binary identity are retained for replay.

Use source commit `5f8ddfd0ff43d151d7b35c5d81003905c59d12fc`, which makes
the F4 oracle follow the factor base's `family` and `selection` parameters.
The clean full-checkout Rust 1.99.1 release build receipt is
`/Volumes/SSD990-2/pkm-oracle-parity-build-20261128/BUILD.json`, SHA-256
`ad0de809d158e476800d9d29d7ffde9048833e502a5e0f8b05c9233ed8823152`.
The binary SHA-256 is
`efe776236bea16b4f0bb31cbdaa2c8e83216b5914a337b4054a8f317106b9bf8`.
The development-only correctness receipt is
`/Volumes/SSD990-2/pkm-oracle-parity-build-20261128/DIAGNOSTIC.json`, SHA-256
`cc907600d2272d3abd9bf31470a469ff505bf29e2c78d774b75eac2055f06d4d`:
one configuration ran, one trial and one solver call were counted, and zero
relations were found under the one-trial cap. It is not a timed comparison.

Run all six original arms again under `run_z2xz8_holdout_cold_v2.py` into a new
store, `/Volumes/SSD990-2/pkm-z2xz8-holdout-cold-parity-20261128`. Pin this
amendment, the original protocol, manifest, old map-preflight receipt, new
source, new build receipt and binary in its header. Independently replay the
original map preflight against its original binary before first launch. Keep
old and new case receipts distinct. The quiet-host, 120-second and 2-GiB
limits still apply. A failed or censored arm remains in the denominator.
The corrected binary still lacks complete native field and Vélu prices, so
reported GAE remains a lower bound until that accounting gate is closed.
