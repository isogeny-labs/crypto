# S56 development checks

Public synthetic fixture: `icv1-fp56-tm132906245-e876a6ea`.
The S56 implementation is unchanged by the two ordinary main merges.
These local checks have unverified PCC standing (`PCC-20261107-d43ebf`);
they do not assert signed execution or mathematical admission. [pcc:unverified]

Command: `cargo test --release --lib s56 --no-default-features`.
Rust toolchain: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; Linux x86-64.
The filter runs all tests added for this fixture, not the entire library suite.
The final command exited 0. [pcc:unverified]

```text
running 6 tests
test cryptanalysis::ecbench::workload::tests::s56_exact_prime_workload_uses_supplied_curve ... ok
test ecc::s56::tests::s56_four_rational_three_isogeny_kernels ... ok
test ecc::s56::tests::s56_prime_shape_and_exact_order_witness ... ok
test ecc::s56::tests::s56_frobenius_discriminant_has_conductor_three ... ok
test ecc::s56::tests::s56_twist_has_a_52_bit_prime_factor ... ok
test cryptanalysis::curve_catalog::tests::s56_catalog_alias_and_native_generator ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 6243 filtered out; finished in 0.00s

```

The paired cryptanalysis release library ran all five S56 tests successfully.
The exact-workload test is specific to crypto's ecbench integration. Thirteen
metadata negative controls, paired semantic validation, dashboard coverage,
metadata-only generator checks and local historical benchrec gates passed.
The JSON preservation check retained all pre-existing registry entries and
measured result payloads. [pcc:unverified]

Initial full-library attempts failed on sparse-excluded compile-time fixtures;
restoring the referenced Git inputs allowed these checks to run. The
historical registry raw pin was retained and read from its exact Git bytes in
the length/hash-checked local archive cache; remote S3 custody was not tested.
No timed IC measurement was run. [pcc:unverified]
