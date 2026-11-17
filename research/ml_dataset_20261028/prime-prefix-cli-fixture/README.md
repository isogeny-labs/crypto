# Native prefix CLI validation fixture

Run `bash run.sh /absolute/path/crypto /new/output/directory` after compiling the
frozen source. The driver invokes only the Rust CLI and checks output identities,
artifact inventories, joins and immutable retries. It consumes the existing n7,
m3 curve/twist fixture whose receipt hash is frozen in protocol.json. It performs
no new generation, timing comparison or model training. Algebraic certificate
verification occurs in Rust; this shell driver is not an independent mathematical
verifier. Original native full-trace and instruction-count claims remain false.
