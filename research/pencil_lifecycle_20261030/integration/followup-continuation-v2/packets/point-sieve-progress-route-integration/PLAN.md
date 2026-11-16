# Diagnostic progress-route finite-difference integration

T-1554: prospective ordinary source validation; no compiler release or scientific execution.

## Source and prior work

pencil.rs None dispatches to native_solver::solve_one; Some(sink) dispatches to progress_native::solve_one_with_progress. The former has the reviewed finite-difference sieve, while the latter still evaluates u and v with Horner. This is source evidence, with no runtime bottleneck or performance inference. Exact source hashes are in source-observation.json.

Reviewed prior point-sieve-finite-difference-plan, point-sieve-producer-integration, and point-sieve-word-iteration-independent-review. NIST DLMF https://dlmf.nist.gov/18.1 defines forward and repeated differences. For quadratic U, delta U(x)=u1+u2(2x+1) and delta squared U=2u2; linear V has constant delta v1. Existing checked ring/word composition supplies the arithmetic justification under canonical coefficients and 0<p<=2^63. No novelty claim.

## Proposed change

The packet candidate copies exactly the canonical initialization and simultaneous old-value capture/update sequence into the progress sieve. It advances before every continue. Active slices alone supply coefficients; inactive tails stay ignored. All remaining source bytes, including every diagnostic emission and solver branch, remain unchanged. Canonical source is untouched pending validation and root patch review.

## Proposed single ordinary validation envelope

One 240-second total wrapper (235 seconds TERM plus 5 seconds kill grace), 4 GiB address space, offline locked Cargo, one build job, one test thread, no retry. Freeze recursive source/include dependencies, vendor/toolchain bytes, wrapper, command and test before requesting execution. Use a fresh private copy of the already checked pencil-symbolic-graph producer crate and its actual five-leads-study dependency; add test-only module wiring for the COMPLETE actual progress_native and progress sink sources. This checks the actual progress module with real engine/kernel/measurement dependencies, not complete root ecbench adapter integration. Prior full root-source test limitations remain disclosed.

Exactly one named test must run: progress_native::finite_difference_integration_tests::finite_difference_progress_route_pairs. Port the existing 6398 full SieveResult pairs (all bucket order and counters), 20207 sequences/134067 samples, 32 canonical boundary transitions, and three known-wrong controls. Compare actual Ops deltas, never synthetic baseline charges; include degree-zero/one regressions and poisoned inactive tails. Add real progress Sink/Noop milestone and emission-helper controls, and a static exact unchanged diagnostic-site/source-body check. These tests do not invoke solve_one or a scientific collector. Require nonzero exact test inventory, all expected pair IDs/counts, unchanged pre/post input pins, process exit zero, group cleanup, bounded output, and retained failure evidence.

## Scope after validation

No candidate binary, image, matched cohort, or new protocol is admitted by this source test. Final ecbench diagnostic-route binary/image/protocol bindings must incorporate this exact source and reviewed pricing. Earlier qualified image and native binary remain historical bindings.
