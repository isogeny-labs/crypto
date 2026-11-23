# PLAN lever 6 — binary batched PairTable for `rootfind-mitm`

Date: 2026-11-22. `[pcc:unverified]` (shared host). Public Koblitz fixtures
only. Engineering wall claim; no security / sub-rho / deployed-curve claim.

## Change

`rootfind-mitm` prepare on `BinaryGroup` now builds through
`PairTable::build_negation_folded_binary_batch` (via
`PairedAdd::build_negation_folded_table`) instead of serial
`build_negation_folded`. A word Bloom filter (same layout as
`rootfind-floor` / `rootfind-kmitm`) sits in front of every probe.

Prime curves keep the serial folded builder; their batched table build
already landed on floor/subtract (lever 5).

## Falsifying measurement

Cold prepare wall, Koblitz `n=23`, subspace dimension 8, 9 cold repeats,
median of wall ns:

| Builder | Median prepare wall (rep 1 / rep 2) |
|---|---:|
| `build_negation_folded` (serial) | 1 874 938 / 1 879 768 ns |
| `build_negation_folded_binary_batch` | 329 375 / 254 165 ns |
| **Ratio (serial / batch)** | **≈5.7× / ≈7.4×** |

Replay: `cargo test --release --lib binary_batch_prepare_wall_beats_serial_on_koblitz_n23 -- --nocapture`.

Agreement: `binary_rootfind_oracles_agree_with_mitm_on_a_koblitz_subspace_base`
asserts `setup_native` reports `binary_batch_table=1` and `probe_bloom=1`,
and that witnesses still match `mitm`.

## Claim hygiene

- Constant-factor engineering on prepare wall; charged additions unchanged
  by design (same first witnesses / insert attempts as the existing
  `binary_batch_table_matches_scalar_table_and_witnesses` test).
- No PROTOCOL / G3 / ledger promotion from this lever alone.
