# Resume: native prime31 algebraic IC continuation of #4188

## Goal
Establish a cheaper complete prime-field non-generic or index-calculus ECDLP
solver on the ordinary exact F_(2^31-1) fixture, reusing Rust and prior leads.
No complete prime31 cost advantage is yet established.

## Lead / hypothesis
Reuse the existing tower-F4 solver with factor-base membership inside S3, native
rational-root extraction and signed EC replay. This supplies a two-summand
baseline; the priority m=8 sparse test needs a representation supporting six
free abscissae. PKM/Dickson construction is established prior work.

## Spec
`research/prime31_ic_20261115/native/EXPERIMENTS.md` EXP-R31-A/B/C.
Prime31 exact coefficients, subgroup and generator in native/README.md and
lib.rs. Matched strongest rho and BSGS, all cold phases, public targets and
full native counters are required before cross-method comparison.

## State
Focused source-reusing crate implemented; all 40 development release-lib tests
passed locally, including prime31 relation controls and one complete p1031
public-target recovery. `development-tests.txt` retains output. All observations
are producer-local, unverified. No research benchmark has launched. Full root
suite status is in the session log. No secret or process-memory state is needed.

## Next steps
1. Read the existing PR #4188 head, this report, today's log and the retained
   test output before changing source.
2. `cargo test --locked --release --lib --manifest-path research/prime31_ic_20261115/native/Cargo.toml`.
3. Resolve the EKS/checkpoint authenticated submission and complete ecbench
   integration/accounting. Freeze actual executable spec/input hashes and PCC
   preregistration at a published source revision.
4. Run EXP-R31-A only after budget/capacity/image admission and checkpoint upload;
   retain all censored attempts, useful row rank, scalar checks and matched controls.

## Runs in flight
No EKS or research benchmark is running. Development build/test sessions, if
still active, are recorded in the root task's session log; they do not constitute
queued prime31 experiments. No paid resources have been allocated.

## Stop and success conditions
No run before the checkpoint upload and live supported-backend admission.
EXP-R31-A: proposed USD 2 total, 4 CPUs, 8 GiB, 900 s, zero automatic retries;
the other distinct envelopes are in EXPERIMENTS.md. These are proposed limits,
not recorded balance or spending authorization. Report censoring honestly.
Success requires correct full prime31 recovery plus complete matched cold cost,
independent replay and the named improvement criterion; unit tests alone do not
satisfy the research goal.
