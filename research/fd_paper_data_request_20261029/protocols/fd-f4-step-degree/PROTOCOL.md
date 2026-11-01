# FD-F4-STEP-DEGREE-20261029 (draft)

**Question.** On the same binary S_3 descent systems whose last fall degree d_F is known, what maximal step degree D_reg does an F4 run reach, and does it reproduce Kousidis and Wiemers' published D_reg (4 at n=34-36, 5 at n=37-48) on Semaev's polynomial subspace?

**Claims.**
- C1 (exact_computation): On every instance, D_reg >= d_F (no F4 run can refute below the closure degree). A violation is an instrument or closure fault and stops the study.
- C2 (comparative): On polynomial-subspace instances at n=34-48, our D_reg equals Kousidis-Wiemers' Table 1 value at the same n.
- C3 (heuristic_estimate): The gap D_reg - d_F per (family, n) is reported descriptively.

**Arms.**
- `f4` (candidate): Boolean-ring F4 (crypto cryptanalysis::pq_f4_f2, normal selection strategy) on each .sys instance with the field equations; records every step degree.
- `kw-published` (reference): Kousidis and Wiemers 2019, Table 1, m=2 rows (published values, not rerun; transcription checked against the PDF before freeze).
- `f4-aa` (control_aa): f4 rerun on every instance with a different input variable order (reverse): D_reg must be unchanged, or the dependence on order is reported.
- `pos-small` (control_positive): Instances with d_F = 3 (P17, R11, R13): F4 must refute with D_reg >= 3 and return 1.

**Decision rule.** C2 is supported at n when our P-instance D_reg equals the published value at n; it is reported per n, with no pooling.

**Stopping rules.**
- fixed design over the listed instances; n is run in increasing order and a cell beyond the memory or 72 h limit is censored
- any C1 violation (D_reg < d_F) stops the study

**Inadmissible.**
- changing the selection strategy after seeing a mismatch
- pooling P and R instances in C2
- reading wall time or memory as comparable to Magma's

## Before freeze

- Prerequisite: examples/f4_f2_sysfile_degree.rs (to be written before freeze): reads a .sys file, runs cryptanalysis::pq_f4_f2::groebner_basis_f4, prints the degree of every F4 step, the maximal step degree D_reg, the step at which 1 appears, and the counted units
- `python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate protocol.json` (passes with 0 findings as drafted, 2026-10-29)
- `... freeze protocol.json --commit auto --filed-before-affected-data`, commit, then `seeds --all`
- Fill `.resume/RESUME.md` (`scripts/checkpoint.sh init`); the experiment director compiles the frozen record into a card
