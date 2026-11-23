# Qualified runtime versus new library-validation source

Source-only comparison under T-1962. No tests, builds, executable invocation, image operation or scientific job occurred. Exact Git blob hashes and complete selected source diffs are retained in bindings.json and source-differences.patch. [pcc:unverified]

## Finding and exact route

The qualified runtime is source3c592af96f3916fef3403e4cf173f22b6f7ef536, ecbenchd0e2d1426115f5efecced07c298d691e1bcc7d5378015f992edc615dc9384f5b, image3a2bb98af4743c888d969c9df00f0769a64e3704e57e97e937769d384e3c8409. New library validation is source29861c9a55f9ae5a33e1e09b3866aa397fbde605 with candidate-pencil555a5c62339a927579599d46c7e94265e52497dc56bacf0b13bb53c2dcd133c4.

pencil.rs is byte-identical between the two Git revisions. The candidate-pencil patch adds only the cfg(test) actualSome/None control; all bytes before cfg(test) match the pinned base. This does NOT make the full new instrument equivalent to the old runtime: transitive solver and timing sources changed.

Exact pencil.rs solve_with_progress match dispatch: None calls native_solver::solve_one; Some(sink) calls progress_native::solve_one_with_progress. Both native_solver.rs and pencil_progress_native.rs replace Horner peval(u,x),peval(v,x) in point_sieve_candidates with a degree<=2 finite-difference recurrence. Each route calls that sieve inside its measured sieve phase after degree2 walk transfer. Both routes now have the recurrence; the positiveSome/None control is not comparing one old and one new sieve.

For active coefficient counts k=2d+1, original Horner performs k field multiplications and k additions plus k collection entry visits per x. New setup charges k collection visits,2field additions; every x advances u/delta/v using3field additions, including the unused final advance. Thus local candidate-minus-original delta is M=-kp, A=3p+2-kp, collection entry visits=-k(p-1). Other branch bodies in the progress-route patch are unchanged. Output-value correspondence is distinct from operation-ledger equality. Degree0/1 addition increases and final advance are retained. There is no speed measurement in this comparison.

Additional source changes: native_solver adds optional relation tracing/capture and related controls; lib.rs exposes trace modules; three_leads moves applicability checks to a callable preflight/backstop; ic_measurement replaces Linux Instant bookkeeping with CLOCK_MONOTONIC nanosecond reads and integer differences. Ecbench methods, workload, pricing/comparison, CLI and registry also differ. Their full selected diffs are preserved. This packet does not claim whole-program semantic or counter equivalence for all these changes.

## Existing finite-difference coverage, retained not rerun

point-sieve-progress-route-integration/result.json and root-postreview.json report6398 full ordered SieveResult pairs;20207 coefficient sequences/134067 samples;32boundary transitions;3known-wrong controls. Actual controls source enumerates p3,5,7 and degree0..2 with q0/1; explicit inactive tails; recurrence samples p2,3,5,7; boundary p=2^63 with four coefficient vectors and eight x positions. Wrong controls use incorrect initial delta, premature second-difference update, and no-advance linear sequence. Five exception/ordinary branch classes are nonzero. Diagnostic helper checks threeevents at1/4/16, livepayload,disabledsink,Noop and eight milestones.

The independent retained review parsed all6398 counterpairs and checked the stated deltas/otherfields. Passing component test compiled actual progress module and sink against standalone dependencies, with78unrelated tests filtered. It did not compile full root adapter or execute complete solve_one; that is the purpose of pending root-library validation. Both original failed wrapper and successful successor receipts remain separate. These records support their existing component scopes only.

## Minimum next binding/build path

First complete new root validation and candidateSome/None positive control/full library. Then explicitly amend the scientific candidate instrument: old known-C1 diagnostic-only algorithm-unchanged wording is insufficient for the finite-difference source changes. Freeze new source/operation mapping, retain original failed rows and qualified runtime as historical baseline, and require independent source/counter review.

If the intended scientific run uses new29861source, build the new intended musl ecbench executable once with published exactsource/toolchain/flags/input closure. Do not substitute oldd0e2binary. Reuse byte-identical controller/progress/fresh-owner artifacts where proven; no reason here to rebuild unchanged helpers. Assemble a new exact base/runtime identity with that executable and build-info; qualify the changed image/binary using the existing reviewed Q0-Q4/runtime lineage with prospective bindings and bounded authorization. Existing old-imageQ0-Q4 is retained and does not qualify a different binary. Independent review must decide the exact necessary changed-image qualification scope; this packet does not authorize builds or tests.

Then bind scientific plan/source/binary/image/request/analysis and fresh native custody; complete the prospective known-C1 gate amendment before affected data. No frozen capped row is rerun by this source comparison.

## Sources checked

KB unavailable in current runtime; exact Git blobs, candidate patch, retained runtime qualification binding, actual progress controls source and its root review were read. Prior component PLAN cites recurrence/word proofs and NIST forward differences; this packet does not introduce a new mathematical mechanism or novelty claim.
