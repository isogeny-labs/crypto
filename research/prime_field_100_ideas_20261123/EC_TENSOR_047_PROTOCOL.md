# Frozen exact EC tensor-train check, #47

Date 2026-11-25. Conductor check was clear; task T-12461 claimed before edits. The earlier short task ID T-184 was observed as unrelated after the coordination service resumed; it is not used for this experiment. Native source `EC_TENSOR_047.rs` SHA-256 `ee46ff7dceb45a7daf12fccd0d3390882e8f7fdf06d26d211e708f0c97587d21`. Preserved natural EC input `WS68_NATURAL_PARTIAL.stdout` SHA-256 `922945c9a81568d23a504e2d296ef6c468f8dd47738e9b9e8a303ce12246f2b3`. No relation search is run. The SSD capacity pause before this protocol left only a copied, unrun source; disk headroom was restored before this freeze.

Literature/prior API check: Oseledets, *Tensor-Train Decomposition*, SIAM J. Sci. Comput. 33(5), 2011, DOI [10.1137/090752286](https://epubs.siam.org/doi/10.1137/090752286), defines a chain of low-rank unfolding factors. The shared KB's `KN-LIT-032` summarizes this construction; repo search found no existing MPS/TT solver for the EC relation matrix. The source proposal #47 seeks kernel information from MPS or PEPS representation of a sparse exponent matrix. This run tests a concrete exact tensor train and contraction, not a tensor-based kernel solver.

## Frozen matrix and controls

Replay all 152 original source EC point witnesses on E/F1009 y²=x³+x+14, order 1013. Ordinary modular elimination of the 16 large columns yields 136 complete rows; recheck each complete row as an EC point identity. Select the first nine coefficient-independent complete rows in source order. Check all nine point identities, base rank 8, augmented rank 9, and scalar 701 against Q=(779,357). Form their 9x9 coefficient matrix (eight small columns and Q), pad with zeros to 16x16, then reshape in row-major order to a four-way 4x4x4x4 tensor.

Build a genuine exact tensor train over F1013 with three successive unfolding rank factorizations. Each stage computes RREF without truncation and verifies A=UV. Verify all 256 tensor entries after four-core contraction. Contract one frozen vector x_j=(17j+3) mod1013 for j<9, zero on padded columns, and compare all 16 outputs with direct modular matvec on the same original 9x9 coefficients. Record bond ranks, storage entries, factorization modular adds/muls/inverses/scans, tensor contraction adds/muls, and direct 9x9 adds/muls. Row-selection rank work and prior relation filtering are separate costs. These are integer operation counts on a tiny selected submatrix; no wall-time or full kernel advantage is inferred.

Checkpoint gate exited 2 for missing `.resume/RESUME.md`; the user's explicit local bypass applies to this bounded public-curve run. No cloud spend or third-party target. Run once under 60 seconds, compiled binary in `/tmp` to protect SSD headroom, preserve stdout/stderr/exit. Any failure remains and requires a distinct versioned repair.

`rustc -O research/prime_field_100_ideas_20261123/EC_TENSOR_047.rs -o /tmp/ec_tensor_047_20261125.bin`

`timeout 60 /tmp/ec_tensor_047_20261125.bin > research/prime_field_100_ideas_20261123/EC_TENSOR_047.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_TENSOR_047.stderr.txt`

## Post-run receipt

Compile exited 0 with unused-code and style warnings from the copied control helpers; the single bounded run exited 0. Stdout SHA-256 `f3b5b4690c34f5ed7ebf213edd7256a390b8e6d6b3e5300cba4e68d2d9607a74`; stderr empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; exit receipt SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`.

All 152 source, 136 complete, and nine selected EC point identities passed. Selected rows had base rank 8, augmented rank 9 and scalar 701, independently checked against Q. The four-core tensor train had exact bond ranks `[1,3,9,4,1]`; all 256 entries reconstructed. It stored 280 F1013 elements, compared with 256 padded dense entries and 81 original unpadded coefficients. Three factorization stages counted 708 modular adds, 1,060 multiplies, 16 inversions and 132 pivot scans; selecting nine independent rows by repeated rank checks cost 16,978 adds, 24,362 multiplies and 284 inversions. The frozen-vector contraction matched all direct outputs. Contraction used 364 modular adds and 364 multiplies versus 81 of each for direct 9x9 matvec. The 136-row large-column filter and original candidate producer costs are additional. The middle rank of 9 is a concrete gate against compression in this four-way reshape; it does not rule out another tensorization or a separate kernel algorithm.
