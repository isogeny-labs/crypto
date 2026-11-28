# Amendment 3: charge the F4 oracle's repeated tower construction

Frozen before any new held-out cold target arm. The original protocol and
Amendments 1–2 remain immutable. The v1, v2 and v3 cold stores contain
headers and busy-host checks only; none launched a cold target. Keep their
raw receipts and binary/source identities separate from the new store.

The `pkm-semaev` oracle constructs the selected isogeny tower from the
public factor-base parameters before the pipeline separately builds the
factor base. The source-pinned development audit
`AUDIT_Z2XZ8_DEV_F4_DUPLICATE_SEARCH_20261128.json` (SHA-256
`1837fb30166ed726cc8383843b78533faea67568bb1f2084b5dae3f56ef9463f`)
showed that the old report charged only the factor-base construction.
Its whole-process wall included both constructions, but its group and
native setup ledger omitted the oracle's constructor work.

Use source commit `62908d5299c04c0b9bda686eba7f19e271b09786`. Its
clean full-checkout Rust 1.99.1 release build receipt is
`/Volumes/SSD990-2/pkm-oracle-rebuild-build-20261128/BUILD.json` (SHA-256
`21b70ef0dc1799f545efd257b5d858ff8bda82cc65d6a9f4b87f378280cf82cf`),
and the binary SHA-256 is
`de461514bcb5f6538781ee25785f11ac33940711a93c0b6cb6a5dfbe0a587002`.
The exact development input's one-trial diagnostic receipt is
`/Volumes/SSD990-2/pkm-oracle-rebuild-build-20261128/DIAGNOSTIC.json` (SHA-256
`b94484580da5b5aa5f8b3ebefff30303385a8e712ca07b335c6b433c25eb22aa`).
On that input the separately charged oracle setup has the same 415 additions,
538 doublings and two scalar multiplications as the factor-base search.
Positive native setup counters include 109224 square-root multiplications,
352 target Legendre tests and 10 Vélu evaluations, all still unpriced.
This is a development correctness check, not a qualified time measurement.

Run all six original arms with the exact `[0,32)` interval control under
`run_z2xz8_holdout_cold_v4.py` into a new store,
`/Volumes/SSD990-2/pkm-z2xz8-holdout-cold-rebuild-ledger-20261128`.
The source commit and binary change; the six curves, 96 targets, relation
seeds, arm order, process caps, source-independent map-preflight receipt,
and promotion rule do not. Pin the original protocol and all three
amendments in the new header. A failed, unavailable or censored arm stays
in the denominator. The newly exposed positive native work must be priced
before any complete cold GAE or matched-rho advantage is reported.
