# FD-F4-STEP-DEGREE-20261029

**Question.** On the same binary S_3 descent systems whose last fall degree d_F is known, what maximal step degree D_reg does an F4 run reach, and does it reproduce Kousidis and Wiemers' published D_reg (4 at n=34-36, 5 at n=37-48) on Semaev's polynomial subspace?

**Instrument.** crypto:examples/fd_m2_degrees.rs --f4 (builds the descent natively and reports F4Stats.degree_reached = D_reg, solving_degree, refuted, counted word XORs)

**Runner.** crypto:.github/workflows/fd-paper-degrees.yml stage f4

**Pilot.** 2026-10-30: on R11a, R13a, R15a, R19a, P17a, P21a, P21c the F4 basis is {1} with degree_reached equal to the recorded d_F

**Claims.**
- C1 (exact_computation): On every instance, D_reg >= d_F (no F4 run can refute below the closure degree). A violation is an instrument or closure fault and stops the study.
- C2 (comparative): On polynomial-subspace instances at n=34-48, our D_reg equals Kousidis-Wiemers' Table 1 value at the same n.
- C3 (heuristic_estimate): The gap D_reg - d_F per (family, n) is reported descriptively.

**Arms.**
- `f4` (candidate): Boolean-ring F4 (crypto cryptanalysis::pq_f4_f2, normal selection strategy) on each .sys instance with the field equations; records every step degree.
- `kw-published` (reference): Kousidis and Wiemers 2019, Table 1, m=2 rows (published values, not rerun; transcription checked against the PDF before freeze).
- `pos-small` (control_positive): Instances with d_F = 3 (P17, R11, R13): F4 must refute with D_reg >= 3 and return 1.

**Decision rule.** C2 is supported at n when our P-instance D_reg equals the published value at n; it is reported per n, with no pooling.

**Stopping rules.**
- fixed design over the listed instances; n is run in increasing order and a cell beyond the memory or 72 h limit is censored
- any C1 violation (D_reg < d_F) stops the study

**Inadmissible.**
- changing the selection strategy after seeing a mismatch
- pooling P and R instances in C2
- reading wall time or memory as comparable to Magma's

## Run

`../../run.sh target/release/examples/fd_m2_degrees <stage>` through `.github/workflows/fd-paper-degrees.yml` (stages calibrate, closure-d4, closure-d5, f4). The calibrate stage must pass C0 before any other stage is read.
