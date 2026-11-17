# Legacy-codec and retained-record checks

The native compatibility harness includes an exact extraction of `canonical.rs::compat_u128` and the unchanged exact-scalar candidate. Its output reports seven tests passed, including the four original candidate tests and three additional compatibility tests. [pcc:unverified]

Constructed boundary cases through u128::MAX compare serialized bytes and deserialize each codec's output through the other, with both present and null optional scalars. Accepted and malformed input cases compare the actual old parser with the candidate. Full-width values above u128 remain an intentional extension covered by the candidate's original tests. [pcc:unverified]

All records from four retained fixture files were read: 4 work-v1 records, 1280 prime records, 1664 Koblitz records and 256 r24 records. Their 9612 subgroup-order, group-order and cofactor fields retain their original JSON values and match the legacy codec's serialized bytes. The records contain zero fields above u64, so the archived-record check does not establish historical u128-record coverage; larger widths are covered by constructed boundary cases. [pcc:unverified]

To replay, compile compatibility_tests.rs with the same --extern dependencies as the initial module, then set CRYPTO_TEST_ROOT to the repository checkout containing the pinned fixture files and execute the test binary with --test-threads=1 --nocapture. The fixture files were read, not rewritten. This remains standalone validation, not integrated workload execution or a signed verdict. [pcc:unverified]
