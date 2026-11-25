# Reproduce the fixed correctness controls

Use the54-input source inventory and the exact three archive hashes in
SOURCE_QUALIFICATION.json. The immutable protocol is PROTOCOL.md. These are
development correctness controls, not a performance/solver experiment.

```
cargo test --release --manifest-path tools/torus-section-compiler/Cargo.toml
cargo fmt --check --manifest-path tools/torus-section-compiler/Cargo.toml
cargo clippy --release --all-targets --manifest-path tools/torus-section-compiler/Cargo.toml -- -D warnings
cargo build --release --manifest-path tools/torus-section-compiler/Cargo.toml
```

Copy the executable and qualifier to new immutable paths; check all54 source
and ten compiled-in input hashes. Write distinct create-new BUILD/PRECALL/
POSTCALL files and use separate stdout/stderr and raw paths. Before dispatch,
exercise transport success and collision refusal through the actual wrapper.
Do not overwrite the retained v1 executable, raw stream or exports.

```
bash tools/torus-section-compiler/run-control.sh IMMUTABLE_BINARY --controls NEW_RAW NEW_EXPORT_DIR NEW_LOG_PREFIX
```

The wrapper enforces600seconds and2GiB virtual memory. A cap/timeout is an
incomplete call, not an absence proof. The expected native flow is15 fixed
arms plus completion; coefficients in FIXTURES.json are imported known controls,
not discovered roots. The compiler's arguments are only public Spec,R,Q and
limits. [pcc:unverified]

TCG1 exports contain magic,32-byte little-endian modulus, four u64 counts
(inputs,constants,gates,output constraints),32-byte constants,12-byte gates
(op,a,b u32), then8-byte constraints(wire,want u32). Opcodes0..4 are constant,
input,add,negate,multiply. Gate index is its output wire. Witnesses contain one
canonical32-byte little-endian Fp value per wire. The raw JSON record binds
semantic coefficient slots, input IDs, S/H IDs, guards, public Spec/R/Q, and
exact hashes/sizes. Sparse equations are streamed by Program::equations;
no input equation is silently added or output/guard omitted.

Inspect full raw and program/assignment bytes through native-v1/ARCHIVE.json.
They are currently local and unpublished; restore needs those bytes or a later
genuine trusted receipt. A local digest does not establish remote availability.
