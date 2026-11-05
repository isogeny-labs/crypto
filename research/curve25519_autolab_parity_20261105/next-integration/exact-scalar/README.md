# Exact-scalar transport candidate

This standalone Rust module is a candidate for the next ecbench integration. It is not registered in the library and does not change the current running suite. It represents unsigned values through 256 bits using four limbs, provides checked u64/u128 narrowing, and preserves the existing JSON wire rule: number through u64, decimal string above u64. Arithmetic and target sampling are not implemented here. [pcc:unverified]

The native test command reports four tests passed: boundary round trips/checked narrowing and bit access, ordering against BigUint, malformed/oversized input rejection, and nullable wide scalars. Fixtures include the actual field prime and subgroup order. This is producer-reported standalone validation, not ecbench integration or signed execution verification. [pcc:unverified]

Built with rustc --edition=2021 --test -O exact_scalar.rs -L dependency=TARGET/release/deps and --extern entries for num_bigint, serde and serde_json. The recorded dependency identities are the locally reused artifacts; a normal Cargo integration should use the repository's locked dependencies and add actual legacy-codec compatibility/record replay checks. [pcc:unverified]

Before dispatch uses this module, widen workload facts, exact identity matching, public/planted target construction, solver result transport, replay and pricing consumers. Preserve historical byte encodings and target laws. Do not use failed narrowing as a zero value. [pcc:unverified]
