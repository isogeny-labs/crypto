# Frozen #40 EC relation-oracle Grover simulation

Date: 2026-11-25. Conductor T-164 claimed before editing. Native source `EC_GROVER_040.rs` SHA-256 `b4a3b35980a35b29dd8eaf82d7e0dada7b2d27aafc2f54874d2d3914c51d6846`. Existing independent GP signed-point predicate `EC_SIGN_ENERGY_GP.gp` SHA-256 `2e9a1d0cd66a7e59dfd83200da44b3d58aed6b7ae89f802f7dd0681b037d9eb8` reports marked masks 5 and 14. The original #40 proposal concerns amplitude amplification for smoothness in Fp×; this test is an explicit E(Fp) adaptation to the already checked exact signed-point relation predicate.

Literature/atlas check: Grover, *A fast quantum mechanical algorithm for database search*, STOC 1996, [arXiv quant-ph/9605043](https://arxiv.org/abs/quant-ph/9605043), gives O(sqrt(N)) unordered search in the abstract oracle model. The repository's `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` describes relation filtering but supplies no reversible EC oracle gate circuit. The existing GP control above fixes the 16-candidate EC truth table. A quantum circuit cost estimate requires reversible field/point arithmetic synthesis, which is not supplied by this bounded state-vector simulation.

## Frozen input and method

E/F1009: y²=x³+x+14; Q=(0,425). Candidate index has four bits: bit 3 chooses one of triples [(4,456),(10,32),(29,35)] or [(5,12),(10,32),(25,23)], and bits 0–2 negate each point before addition. Predicate is exact group equality of their signed sum to Q. Verify all 16 predicate values with native EC arithmetic and require marks [5,14]. Encode the resulting EC point injectively in a work register. Simulate the reversible permutation `work ^= encode(sum(index))`, target equality into a flag, phase flip, flag undo, and work undo; assert cleared ancillas for every basis index. Apply diffusion on a 16-amplitude real state vector for j=0..5 iterations. Require probability normalization and agreement with sin²((2j+1)asin(sqrt(2/16))). Compare to uniform classical search without replacement at equal j logical oracle queries, with one final verification query reported separately.

Charge per Grover iteration one logical phase oracle, two coherent point-predicate computations (compute and uncompute), and 32 EC branch evaluations in this classical simulator. Initial 16-entry truth-table validation is separate. These are abstract costs: the source does not synthesize a fault-tolerant gate circuit, assign qubit/Toffoli/T-depth counts, or execute quantum hardware. The simulator's branch evaluations are not quantum gate counts. Do not infer a practical ECDLP advantage or extrapolate the 16-state result.

Checkpoint gate exited 2 because `.resume/RESUME.md` is missing. The user's explicit local bypass covers this bounded public-curve run. No cloud spend or third-party target. Compile and run once, cap 60 seconds, preserve stdout/stderr/exit. Any failure requires a distinct versioned repair.

`rustc -O research/prime_field_100_ideas_20261123/EC_GROVER_040.rs -o /tmp/ec_grover_040_20261125.bin`

`timeout 60 /tmp/ec_grover_040_20261125.bin > research/prime_field_100_ideas_20261123/EC_GROVER_040.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_GROVER_040.stderr.txt`

## Post-run receipt

The single native compile and capped run both exited 0. Stdout SHA-256 `09b499e2ccdba35e9a1b2ef0232a4dd08ef27850af324520a6775178299baec2`; stderr empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`; exit receipt SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`. The 16-entry native EC truth table marks exactly 5 and 14, matching the independent GP control. Compute/flag/phase/uncompute returned both ancillas to zero for every index. State-vector normalization and the closed-form success probability passed at iterations 0 through 5.

At two Grover iterations the simulated marked-measurement probability was 0.9453125, with two logical phase-oracle invocations, four abstract coherent predicate computations including uncomputation, and 64 EC branch evaluations by the classical simulator. Uniform classical sampling without replacement succeeds with probability 0.241666666667 after two predicate queries; including the final verification query on both sides, its three-query probability is 0.35. The corresponding values for other iteration counts are in stdout. This is a small abstract-oracle simulation. The EC arithmetic is evaluated classically in the simulator; no quantum circuit, gate count, hardware result or full relation-search cost follows.
