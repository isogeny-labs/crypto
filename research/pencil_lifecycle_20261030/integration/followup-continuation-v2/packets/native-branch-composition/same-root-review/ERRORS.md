# Retained review setup errors

1. Initial replay command guessed pencil-native-gate as the binary name; shell exited127 before any Lean run. Read Cargo package output, rebuilt native runner and used quotient-storage-native-verification. No replay data were produced by the rejected invocation.
2. Initial published Rust-source extraction guessed main.rs.txt; git reported nonexistent path and comparison rejected an empty output. The actual published path is native/src/main.rs. Re-extraction from the pinned commit and byte comparison passed. Mathematical published .lean.txt files all matched on the first extraction.
3. No reviewer Lean proof attempts failed. Fresh ExpectedFailure deliberately rejects 0=1 with native/runner exit1. All producer failures remain in the producer's sealed packet.
