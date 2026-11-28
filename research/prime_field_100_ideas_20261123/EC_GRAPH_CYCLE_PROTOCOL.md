# Frozen EC graph-cycle filter, #39/#76

Date: 2026-11-25. Conductor task T-112, claimed before edits. Local public-curve experiment under the user's explicit local-bypass authorization. `scripts/checkpoint.sh gate` exited 2: `.resume/RESUME.md` is missing. This is recorded rather than treated as an S3 checkpoint. No cloud request or third-party target is involved.

## Input and hypothesis

Use exactly the 152 `partial_filter` source rows embedded in `WS68_NATURAL_PARTIAL.stdout`, SHA-256 `922945c9a81568d23a504e2d296ef6c468f8dd47738e9b9e8a303ce12246f2b3`. Exclude the first 17 `full_only` duplicate lines. The frozen native source `EC_GRAPH_CYCLE_RS.rs` has SHA-256 `f194b411d3360f81ee7fc9fef3bbe7cf2f37a6aff59238a345cedd304ca49bb7`. Curve E/F1009: y²=x³+x+14, order 1013; G=(0,425), Q=(779,357), eight small and sixteen large columns. The producer generated 512 seeded candidate points; no new relation search occurs here.

Test whether a spanning-forest cycle filter extracts complete rows with less modular work than ordinary dense large-column elimination, preserving rank and target coverage. Graph edges are one-large rows from a hub and two-large rows between their two large vertices. A cycle candidate uses only its unique forest path; its large columns must cancel exactly modulo 1013. An odd or otherwise nonclosing cycle is retained in the failure count. Full rows pass through. Compare against ordinary elimination on exactly the same 152 source rows and against connected-component partition followed by ordinary elimination within each component. The component arm includes graph traversal cost.

## Checks and accounting

Independently rebuild the first 24 curve points by x/y scan and recheck every source identity `F_i+F_j=aG+bQ`. Recheck every complete output point identity and any extracted target scalar. Record complete rows, base and augmented ranks, target value or absence, graph tree edges, closing/nonclosing cycles, path length, modular add/multiply/inversion and cell/path scan counts, and the producer's setup/candidate costs as an external stage. Dense 26-cell row operations are charged. Rust `rustc -O` compile, one invocation capped at 60 s, stdout/stderr/exit recorded separately. Any failure remains in place; no silent retry. No speedup claim from this small matrix or from unpriced point field arithmetic. Frozen commands:

`rustc -O research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE_RS.rs -o /tmp/ec_graph_cycle_20261125.bin`

`timeout 60 /tmp/ec_graph_cycle_20261125.bin > research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GRAPH_CYCLE.stderr.txt`

The compiled binary is local-only. This protocol was written before the first compile/run. Any source repair requires a V2 source/protocol and distinct raw receipts.
