# K4-CHAR2-REPLICATION-20261029

**Question.** On k=4 characteristic-two curves E/F_{2^{4m'}} (Gaudry D4, binary oracle), is index calculus below the strongest legitimate rho in counted F_q multiplications at m=16 and m=17, across the pricing band, with 8 fresh targets per arm?

**Claims.**
- C1 (exact_computation): Every row recovers the planted discrete logarithm ([k]G = Q checked by the harness).
- C2 (comparative): At m=16 and at m=17, summed counted cost of ic-wied over rho-strong has a 95% paired-bootstrap interval entirely below 1 at every weight w in {0, 0.1, 1} GAE per unpriced operation.
- C3 (comparative): rho-neg reproduces the old records within the A/A spread (regression control).

**Arms.**
- `ic-wied` (candidate): `ic.pipeline` + `bin-t-line` + `gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1`, `linalg=wiedemann`
- `rho-strong` (reference): `rho.negation_strong` (`lanes=64`, `dp_bits=auto`): negation-map rho, distinguished points, Montgomery-batched inversion, canonicalisation charged; every phase in counted F_q multiplications / `fq_muls_per_add`
- `rho-neg` (baseline): the existing rho.negation, regression control against old records
- `rho-neg-aa` (control_aa): rho.negation under a second arm name: the A/A noise floor

**Decision rule.** C2 supported at m when the Holm-corrected 95% paired-bootstrap interval of ic_over_rho lies below 1 at w=0, 0.1 and 1; 'trades' if the band straddles; 'loses' if above 1 at every w.

**Stopping rules.**
- fixed design: 4 arms x 4 m x 8 targets (+1 warmup per arm); preflight ecbench verify --replay on one old clean session first
- a killed or partial run is never recorded; *.attemptN-killed stays out of analysis

**Inadmissible.**
- choosing the pricing unit after the data
- reusing the old targets
- quoting the m=17 killed-run ratio
- reading a weak-family crossover as a statement about prime-order or deployed curves

## Instrument and files

- Method: `rho.negation_strong`, registered in `src/cryptanalysis/ecbench/methods.rs`, walk in `src/cryptanalysis/ecbench/negation_strong.rs`; charge documented in `docs/ecbench/README.md` §7.
- Spec: [`spec.json`](spec.json): 32 `f2m4_torsion` curves (the listed seeds, `max_cofactor` 16), one target each, four arms, one round, shuffled arm order, no warm-up round (an ecbench warm-up round repeats every workload).
- Analysis: `examples/k4_replication_decide.rs` (sha256 in `protocol.json`): `cargo run --release --example k4_replication_decide -- SESSION_DIR...`. `estimate.py` in the red-team study is a wall-time estimate, not the decision rule, so it is not the analysis.

## Open prerequisite

The frozen instrument refuses m = 16 and 17: `f2m4_torsion` and `gaudry_quartic_bin` take m <= 15 (u16 F_q elements, points packed in 4m <= 60 bits). The red team's m = 17 records came from commit `e403626c`, which is not in this repository. Widening the tower is an instrument change filed as an amendment before any m >= 16 row (invalidation rule in `protocol.json`).

## Before the run

- `python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate protocol.json`
- Fill `.resume/RESUME.md` (`scripts/checkpoint.sh init`); the experiment director compiles the frozen record into a card
