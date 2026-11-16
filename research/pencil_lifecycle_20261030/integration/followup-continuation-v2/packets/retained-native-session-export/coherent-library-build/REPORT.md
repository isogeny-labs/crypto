# Coherent native library build

The exact native crypto_lib library compiled successfully in 132.534783 seconds using 16 code-generation units, no embedded bitcode, and a real inherited jobserver with zero additional tokens. The earlier single-unit build failed in LLVM under the same 8 GiB address-space cap; its source, outputs and receipt remain preserved in the parent failure packet. This is a successful unoptimized software build, not a performance comparison.

The compiler and outer wrapper exited 0, the compiler was reaped, and parent descriptors 3/4 closed. All 1103 source entries, 506 reused dependencies including the rebuilt graph, and 145 compiler/sysroot/runtime support artifacts passed before/after hash checks. Source membership matched with no symlinks. No signal or launch uncertainty remained. The jobserver limits managed work slots; it is not a hard CPU quota or thread-count claim.

The resulting rlib has 637987072 bytes and SHA256 71753af3fbab712659cbf04e2d4d8cc718c2c31631812cffc5d889cb89b88e3f. Its metadata file has 165091518 bytes and hash 6a3d5dd3ad3dc744fe267f278672f931f624249205f289353d2346511c63548f. The retained graph rlib is eda0cd547d4524c16950245b3821bc185f6571f25ed665acd318006a2106a2e1. Reused third-party compiled artifacts have byte pins, not fresh source attestation.

The frozen allowance was one 300-second attempt with hard 8 GiB AS and 1 GiB perfile. The sampled output maximum was 803401272 bytes; minimum free space 18669445120 bytes; observed 4 GiB threshold overshoot zero. The aggregate monitor may miss intersample peaks and is not a filesystem quota. Exact owned outputs are retained for archive and separately authorized exporter compilation.

A non-author reviewer accepted the prospective recipe and then independently rehashed all three outputs and inspected native receipts/outer exit. Final review 4c6a94d4a0844ad2f5e0e12250ab558a2db44c01069d494a0f3c48c6463b7347 involved no reviewer binary execution. Prior advisory familiarity is disclosed.

The exporter and its 12 controls have not executed. A subsequent source review identified private helper imports in the original controls; root authorized fixture-only correction using public primitives, while exporter and actual validators remain unchanged. That separate control plan is outside this build result.
