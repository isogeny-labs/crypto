# Certified relations

Shared native Rust implementation of exact polynomial membership, bounded
degree/closure traces, direct Semaev-pair consequences and certified quotient
bases. The repository's F4 adapter and the original study workers depend on
this crate. Plotting and campaign executables remain in the study.

The polynomial basis constructor accepts equations only. Public known-answer
fixture helpers are for development; witnesses never enter construction.
The field uses exact u64 residues/u128 products. Sparse certificate and matrix
budgets may return explicit incomplete/unsupported results. Final basis degree
is distinct from solving degree, top-form regularity and paid certificate degree.

This library is not published. Its fixture JSON and frozen historical numeric
table are byte-identical copies of the original study's development inputs.
No historical Python code is an active dependency. `count-field-ops` is a
separate diagnostic feature; sparse update counts omit dense quotient writes.

Development checks: `cargo test --release --locked` and strict all-target
clippy; also check the study crate and root `f4_fp::certified` adapter. Existing
producer observations and their frozen source receipts remain in
`research/relation_degree_trace_20261115`; moving the code creates no new
performance or independently signed correctness claim `[pcc:unverified]`.
