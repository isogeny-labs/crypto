# FD-BINARY-CLOSURE-EXT-20261029 (draft)

**Question.** For the binary Weil descent of Semaev's S_3 (m=2, k=ceil(n/2)), (a) does the certified bound d_F >= 5 hold on every random-subspace instance and on Semaev's polynomial subspace from n=45 upward, for n up to 51; and (b) does the last fall degree reach 6 on any instance where W_5 is computable (n=33-37)?

**Claims.**
- C1 (exact_computation): For each new instance with 1 not in W_4(F) by closure A, an independent certificate check (cert_check.py, own descent) accepts 1 not in W_4(F), so d_F >= 5 on that instance.
- C2 (exact_computation): For each instance already certified d_F >= 5 at n=33-37 (T33a/b, R35a/b, U35a/b, R37a/b) the degree-5 closure decides 1 in W_5 (d_F = 5) or 1 not in W_5 (d_F >= 6, certified as in C1).
- C3 (exact_computation): The existing uncertified d_F >= 5 instances R39a, R39b, C21a, C21b receive checked certificates, or the record says which check failed.
- C4 (heuristic_estimate): Per (family, n) cell, the fraction of instances with d_F >= 5 is reported with a Clopper-Pearson interval; no trend in n is claimed from these counts.

**Arms.**
- `closureA` (candidate): lfdclose2 (M4RI dense echelon, degree-descending colex order) computes W_D for D=4 (n=37-51) and D=5 (n=33-37); writes the quotient map as a certificate when 1 is not in W_D.
- `certcheck` (reference): cert_check.py rebuilds the equations from the curve parameters with its own descent and checks the certificate (Proposition 1.5); independent of closureA's code path.
- `closureA-aa` (control_aa): closureA rerun on a 10% subset with OMP_NUM_THREADS=4 instead of 8: W_D dimensions must be identical (determinism floor).
- `pos-known-refuting` (control_positive): Instances with recorded 1 in W_4 (P35a, P41a, S40a, U33a): closureA must report 1 in W_4 again.
- `neg-satisfiable` (control_negative): Satisfiable instances (Boolean solutions planted by drawing the target as a sum of two base points), one per n: 1 can never lie in W_D, so a refutation here is an instrument fault.

**Decision rule.** C1/C3: per instance, certified when certcheck accepts. C2: d_F = 5 if 1 in W_5, d_F >= 6 if 1 not in W_5 and the degree-5 certificate checks. P1 holds if every admissible R instance at n=41-51 is certified >= 5; P2 holds if any named instance is certified >= 6.

**Stopping rules.**
- fixed design: run every listed cell once; D=5 closures only on the 8 named instances
- a closure that exceeds its memory or 48 h limit is recorded as censored (no verdict) and not retried beyond the infrastructure limit
- any positive-control miss or negative-control refutation stops the study: the instrument is faulty

**Inadmissible.**
- redrawing an instance because its d_F is inconvenient (only decomposable targets are redrawn, and every redraw is reported)
- calling d_F >= 6 without an accepted degree-5 certificate
- reading the cell fractions as a growth law in n

## Before freeze

- Prerequisite: see ../../DATA_REQUEST.md
- `python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate protocol.json` (passes with 0 findings as drafted, 2026-10-29)
- `... freeze protocol.json --commit auto --filed-before-affected-data`, commit, then `seeds --all`
- Fill `.resume/RESUME.md` (`scripts/checkpoint.sh init`); the experiment director compiles the frozen record into a card
