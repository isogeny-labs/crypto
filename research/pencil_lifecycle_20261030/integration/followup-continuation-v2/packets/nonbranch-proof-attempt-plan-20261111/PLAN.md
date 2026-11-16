# Prospective nonbranch proof replay — approval pending

**Status:** [pcc:unverified] plan only. No new compiler invocation, attempt, deadline, budget debit, proof verdict, or scientific promotion is authorized by this file. Conductor planning task: T-2115. The original campaign ended at attempt 3 of 3 with deadline 2026-11-06 04:00 UTC; its receipts stay immutable.

## Fixed question and inputs

Can the proposed `IndependentControls.lean` correction be compiled and its 20 named claims audited alongside the ten claims in `Nonbranch.lean`, while the retained false-case control fails as expected? A successful compiler exit alone cannot establish source fidelity, independent semantic review, or the historical experiment.

The immutable source base is crypto commit `4f8d8bf375348180cf1518ecf9d913a2ddca7ad2`. The current tracked source closure was unchanged through gate commit `cce79de90322deb6e598c28a63b0f3cb75e2fb02`; the preflight import map is SHA-256 `6463b80ee54d1dcdafc80ec30f8c39617f858866489fb5ff66b95b669bde0ee7` and records 41 sources and 53 internal edges.

| Input | SHA-256 |
| --- | --- |
| Archived `Nonbranch.lean` | `ef72bcbfc9ea3643ae126dfeba644c2a34a999cb34254f8b17357ddd31d6f9e1` |
| Archived original `IndependentControls.lean` | `6dda751467447446311323393a0a985ae93f2440e700289914bcd0037e1476d7` |
| Proposed corrected `IndependentControls.lean` | `448d5ab601e955b93a9071d6b5f932b38850f5728c9cf2c3fc3eb5b3742bb35d` |
| Archived `Readback.lean` | `1cf95c5e60a634a8817b2dbb948c887bd7f1044630676f1089eb5dac5f14c8b2` |
| Archived `ExpectedFailure.lean` | `d734382f5c0fdc1d3846ec0cc07d2c9ea1eb951cd47576fef6d67cf410059165` |
| Projective-law `lake-manifest.json` | `ebc6643e694eaa4a60ab2b727c96253f3d88694e7ecb404f2176a548eedbe98d` |

The projective-law workspace pins Lean `leanprover/lean4:v4.34.1` and Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`. The archived pencil roots have no nearest toolchain file; the retained native commands explicitly used Lean 4.34.1. The correction overlays one file in a private copy only. No source, manifest, or archived receipt is edited in place.

## Proposed one-attempt envelope

Execution requires a separate owner authorization recorded after this plan. The new campaign would have one local attempt, no retries, an absolute 40-minute wall deadline from admission, no EKS Job, no new instance, and no quota change. The current user limit is USD 25/day for Spot agents and experiments. Admission must read current spend and remaining allowance; unknown or exhausted allowance prevents launch. Count failed preparation, dependency work, compiler work, and cleanup against the same 40 minutes and cost ceiling.

Before launch, verify the source commit and every input hash, the installed Lean/Mathlib pins, exclusive Conductor and build ownership, no competing Lean/Rust build, RAM and RAM-backed temporary storage, and disk headroom. Reserve at least 16 GiB RAM for the system and other agents after estimated peak. Use one Lean compiler at a time with the retained `-j1 -M3500 -s16384` settings. Use a bounded private workspace and stop before disk exhaustion. Failure to establish ownership or headroom is a no-run result, not a retry.

## Frozen check order and stopping rule

1. Materialize the 41-source import closure in a private workspace from the pinned tree. Verify all source hashes, toolchain and manifest before and after. Record exact command argv, process IDs, exit codes, elapsed time, resource peaks, and input/output hashes.
2. Build the three direct projective-law imports of `Nonbranch` (`WeightedProjNonbranchDVR`, `WeightedProjQuinticHead`, `Common22SourceHead`) and their transitive imports at the pinned Lake revision. If dependency materialization or any build fails, stop and retain the diagnostics.
3. Compile the archived `Nonbranch.lean`, then the proposed corrected `IndependentControls.lean` using the private dependency artifacts. Any nonzero exit stops the positive path. Do not edit the candidate within this attempt.
4. Compile the archived `Readback.lean`; require all 30 named `#print axioms` outputs, check each theorem name and transitive axiom set against the repository's fail-closed auditor, and reject an empty or truncated audit.
5. Compile `ExpectedFailure.lean` against the same candidate and require a nonzero exit at its false equality. A missing, skipped, or unexpected passing control prevents a validated result.
6. Rehash the full local source and verifier inputs, preserve stdout/stderr and native receipts, and perform independent source-fidelity and semantic review before any claim promotion.

The single attempt ends at the first failed stage or at the deadline. A failed build, missing import, timeout, OOM, mismatched hash, absent readback, unexpected axiom, or wrong control outcome is retained with its own diagnosis. No automatically altered proof script or second attempt is allowed.

## Review plan before dispatch

The prior is **inconclusive**: the original `Nonbranch` module compiled, the original `IndependentControls` module failed, and the correction has zero compiler invocations. Reviewer A owns a blind re-derivation of the algebraic nonbranch and selected-order joints from only the mathematical statement and parameters. Reviewer B owns source fidelity: compare all 30 theorem statements, hypotheses, and coefficient constants against the archived source and correction diff, including the `n.eval 1 = b*(-1:ℚ)` negative case. Reviewer C owns the verifier boundary: exact module compilation, import closure, axiom outputs, and failure controls. Each reviewer records allowed inputs, actual reads, independence, and a separate verdict; the Coordinator composes them only after all reports exist.

The attack cases are: a hidden assumption that makes a theorem vacuous; a changed coefficient or theorem statement masquerading as a proof-only edit; an imported module absent from the audited workspace; a custom axiom or `sorryAx` hidden behind an imported helper; and the false-case control unexpectedly elaborating. The review must not treat the original native compiler failure, the hosted standalone import failure, or the correction's proposed status as a passing proof.

## Current state

The source-only preflight is merged in PR #3856. Hosted proof-gate run 42928041253 passed batch 148 and recorded `FAIL_build` for the five pencil roots in batches 180 and 184 at unresolved import prefixes. Those hosted failures locate the gate workspace gap; they do not consume this proposed attempt or change the original native verdict.
