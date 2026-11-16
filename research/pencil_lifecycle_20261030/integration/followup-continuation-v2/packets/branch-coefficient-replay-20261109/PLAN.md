# Branch coefficient kernel replay — prospective

Compile the unchanged candidate source `4e8cfd2ff984359b4fbd92e8fd0432db04b82e08fd19223eb5dac9d718995b76`. The prior independent source-fidelity review is retained. The import chain requires compiling ten canonical branch modules absent from the previous norm closure, followed by Nonbranch, BranchConjugateOrders, BranchNormTransport, BranchRootMultiplicity and the candidate. `import-bindings.json` records sources and order. Existing cached imports retain their recovered norm baseline bindings; generated modules stay in a fresh private workspace.

## One bounded attempt

The proposed ceiling is 3,600 active worker-seconds: 900 for custody before compilation, 1,800 for compilation and 900 for custody afterward. Retain one CPU, 8 GiB RAM, no swap, 128 tasks, one Lean thread, `-M3500`, 8 GiB address space, 16 MiB per file and a sampled 256 MiB aggregate output bound. The campaign deadline remains 2026-11-10 05:11:39 UTC. Actual remaining allowance and release fields stay unset until the coordinator records them. Execution is serial after the current payload terminates; there is one attempt and no automatic retry.

The runner hashes all 16,736 inherited dependency files (3,111,892,318 bytes), current runtime support and new module sources before and after compilation. This retains the existing limitation that a transient shared-cache mutation restored between checks may escape detection. Owned phase cleanup is required.

## Controls and readback

The independent paper witness is formalized over the rationals: f = X^5-X, branch point 0, supports X^2+1 and X^2-1, and zero ordinates. The intended coefficient tuple is (-4,0,0,-16,0), with residual -16X. The new control source proves the theorem premises, including the explicit Bezout certificate (-25X^3)f + (5X^4-4)f′ = 4, and instantiates the actual candidate theorem. Negative controls reject equal-support nonzero determinant and branch point 2. A separate ExpectedFailure process attempts the false branch claim; it must fail with the intended single unsolved False goal, not a syntax, environment or resource error.

These controls remain uncompiled. A separate readback process audits one producer and twelve control declarations, rejecting missing or duplicate names, sorry and unexpected axioms. Only propext, Classical.choice and Quot.sound are permitted. The main theorem's transitive axiom readback covers its dependencies. The prior thirty norm readbacks remain evidence of their earlier attempt; this continuation audits the new composition and controls.

## Publication and release

Before execution, review the controls, import coverage and runner differences; publish the exact source, configuration, input bindings and launch command; bind the actual remaining allowance, fresh lease, serial-terminal receipt and publication receipt; then finalize the freeze and coordinator release. No payload has run during preparation.

The future command is `python3 -B launch.py --execute --freeze-sha256 ACTUAL --release-record ABSOLUTE --release-sha256 ACTUAL`. Placeholders are not valid execution inputs.

Sources checked were the original DERIVATION and producer source, its independent REVIEW, predecessor branch sources, current norm freeze/runtime/runner and baseline import identities. The research KB was unavailable, so these exact primary sources provided the fallback. This packet makes no novelty claim.

## Compiler-envelope rationale

The coordinator's current norm receipts show approximately 217 seconds for four Lean calls, including approximately 101.5 seconds for Nonbranch and 35–40 seconds for each dependent import. This continuation requires eighteen calls: ten canonical prerequisites, Nonbranch, three branch parents, the candidate, controls, readback and expected failure. A copied 600-second compiler allowance would be predictably tight. The prospective allocation is therefore 1,800 compiler-seconds plus 900 seconds for each full custody phase, with no pilot or automatic retry. These observations inform the allocation; they do not predict a measured elapsed time.
