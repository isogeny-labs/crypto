# K4-CHAR2-REPLICATION-20261029 (draft)

**Question.** On k=4 characteristic-two curves E/F_{2^{4m'}} (Gaudry D4, binary oracle), is index calculus below the strongest legitimate rho in counted F_q multiplications at m=16 and m=17, across the pricing band, with 8 fresh targets per arm?

**Claims.**
- C1 (exact_computation): Every row recovers the planted discrete logarithm ([k]G = Q checked by the harness).
- C2 (comparative): At m=16 and at m=17, summed counted cost of ic-wied over rho-strong has a 95% paired-bootstrap interval entirely below 1 at every weight w in {0, 0.1, 1} GAE per unpriced operation.
- C3 (comparative): rho-neg reproduces the old records within the A/A spread (regression control).

**Arms.**
- `ic-wied` (candidate): gaudry-d4-bin, plain Wiedemann, macaulay=11, frobenius=1, sparsest=1, replay=1, unchanged binary
- `rho-strong` (reference): negation-map rho, distinguished points, batched inversion, canonicalisation charged (new; ecbench-extend)
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

## Before freeze

- Prerequisite: rho-strong: negation-map rho with distinguished points, batched (Montgomery) inversion and charged canonicalisation, added with the ecbench-extend skill before freeze
- `python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate protocol.json` (passes with 0 findings as drafted, 2026-10-29)
- `... freeze protocol.json --commit auto --filed-before-affected-data`, commit, then `seeds --all`
- Fill `.resume/RESUME.md` (`scripts/checkpoint.sh init`); the experiment director compiles the frozen record into a card
