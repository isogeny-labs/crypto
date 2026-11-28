# FD-BINARY-CLOSURE-EXT-20261029

**Question.** For the binary Weil descent of Semaev's S_3 (m=2, k=ceil(n/2)), measured by a second, independent closure implementation: (a) does d_F >= 5 hold on random-subspace instances at n=41-51 and on Semaev's polynomial subspace at n=47-51; and (b) does the last fall degree reach 6 on any of the 8 instances certified d_F >= 5 at n=33-37?

**Instrument.** crypto:examples/fd_m2_degrees.rs (native Rust; replaces the draft's lfdclose2 because crypto forbids Python research tooling and the 120 GiB runner is a crypto runner)

**Runner.** crypto:.github/workflows/fd-paper-degrees.yml on crypto-eks-highmem; crypto:research/fd_paper_data_request_20261029/run.sh

**Pilot.** 2026-10-30 local 4-vCPU container: native closure reproduced the recorded d_F on R11a, R13a, R15a, R19a, P17a, P21a, P21c, U33a (exact) and T33a (>=5, W_4 rank 48068 = recorded); D=4 at n=33 in 34-44 s; D=5 at n=21 (35443 columns) 117-137 s; planted controls never refuted to D=5

**Claims.**
- C0 (exact_computation): On every recorded m=2 instance, the native closure gives the recorded last fall degree (exact, or >= 5 where the record says >= 5), and at D=4 the same closure rank as lfdclose2's W_4.
- C1 (exact_computation): For each new instance, the native closure decides whether 1 lies in W_4 (d_F <= 4) or not (d_F >= 5, reported 'closure_only' until a checked certificate exists).
- C2 (exact_computation): For each instance already certified d_F >= 5 at n=33-37 (T33a/b, R35a/b, U35a/b, R37a/b) the degree-5 closure decides 1 in W_5 (d_F = 5) or 1 not in W_5 (d_F >= 6, closure_only until certified).
- C3 (out_of_scope): Checked certificates (lfdclose2 + cert_check.py) for R39a/b, C21a/b and the new d_F >= 5 instances: a separate crypto-autoresearcher run, not this protocol.
- C4 (heuristic_estimate): Per (family, n) cell, the fraction of instances with d_F >= 5 is reported with a Clopper-Pearson interval; no trend in n is claimed from these counts.

**Arms.**
- `closureA` (candidate): fd_m2_degrees --closure: builds the m=2 descent (build_decomposition_system, b = a6, x_R = z) and the full MutantXL closure at each D (= W_D); D=3..4 on new draws, D=5 on the 8 named instances.
- `lfdclose2-record` (reference): lfdclose2's recorded outcomes for the 93 recorded instances (independent C/M4RI implementation, different monomial order and descent code): last fall degree and W_4 closure dimension.
- `closureA-aa` (control_aa): closureA rerun on R21a, R25a, R29a, R33a with RAYON_NUM_THREADS=4: closure ranks must be identical (determinism floor).
- `pos-known-refuting` (control_positive): Every recorded instance with 1 in W_3 or W_4 (the calibrate stage): closureA must refute at the recorded degree.
- `neg-satisfiable` (control_negative): Planted satisfiable instances (fd_m2_degrees --planted: z a root of S_3(a, c, z) for a, c in V), one per n=33-51: 1 can never lie in W_D, so a refutation here is an instrument fault.

**Decision rule.** C0 must hold before any new row is read. C1: per instance, d_F <= 4 if 1 in W_4, else >= 5 (closure_only). C2: d_F = 5 if 1 in W_5, >= 6 (closure_only) if not. P1 holds if every admissible R instance at n=41-51 has 1 not in W_4; P2 holds if any named instance has 1 not in W_5.

**Stopping rules.**
- fixed design: run every listed cell once; D=5 closures only on the 8 named instances
- a closure that exceeds its memory or 48 h limit is recorded as censored (no verdict) and not retried beyond the infrastructure limit
- any positive-control miss or negative-control refutation stops the study: the instrument is faulty

**Inadmissible.**
- redrawing an instance because its d_F is inconvenient (only decomposable targets are redrawn, and every redraw is reported)
- calling a closure_only bound 'certified'
- reading any new row before C0 passes
- reading the cell fractions as a growth law in n

## Run

`../../run.sh target/release/examples/fd_m2_degrees <stage>` through `.github/workflows/fd-paper-degrees.yml` (stages calibrate, closure-d4, closure-d5, f4). The calibrate stage must pass C0 before any other stage is read.
