# Continuation instruction

Continue the user's prime-field elliptic-curve relation-solving research, starting with p=2^31-1, toward a correctly supported asymptotic speedup. Keep the objective open until that criterion is met or the user changes it. Treat this as open research, not a promised breakthrough. Complete concrete work at each iteration. [pcc:unverified]

Read current repository instructions, ACTIVE, newest session logs, both live PR heads, STATE.json and actual source-bound CI receipts before editing. Reuse isogeny-labs/crypto PR4928, branch lane/prime31-dickson-quotient-gate, and companion PR4924, branch research/symmetric-pair-quotient-prime31-20261122. Preserve other active lanes. Guard branch updates and reconcile concurrent changes; never overwrite a newer iteration with a stale state.

## Current checkpoint

Iteration6/MATHEMATICAL_NOTE.md gives full-Dickson membership equality for rational affine S3 trees, a bijective all-node quadratic augmentation, exact counts, explicit Frobenius guards, and free-order comb completeness for nonzero-total signed multisets with m<=n in a prime-order group. Completeness does not extend to arbitrary fixed balanced trees: retain P,P,P,-P. Fp coefficients alone do not force rational auxiliary roots. Preserve the explicit nonrational S4 ghost and distinguish witnesses from closure roots and multiplicities.

The supplied-chain verifier is examples/prime31_rational_chain_guard.rs, initially published at bf56de345c02ef6f1c55bfa047087b0e880cb09e. It reuses original arithmetic and has fixed correctness controls; it is not a relation finder. Original-root-lock CI is run45096187034/job137436389923; read the retained receipt and any superseding source repair before quoting an outcome. The cubic-residual identity issuer produces evaluated-tier records with proof:null. Read iteration6/ROOT_REVIEW.md, INDEPENDENT_REVIEW.md, LITERATURE_AUDIT.md and SOURCE_AUDIT.md; the last is a historical pre-implementation audit. SOURCE_FREEZE.json and execution receipts govern current implementation status. All claims retain their actual PCC standing.

Semaev2015 already uses rational-auxiliary S3 chains; the native m8 oriented example already orients a leaf pair. Do not claim either mechanism itself as new.

## Next discriminating step

Specify an exact elimination or contraction for the rational quadratic system, including misses. Derive recurrences for actual intermediate dimensions, field work, setup, memory, rational-root isolation and output. The missing term is solving, not evaluating an assigned tuple. Quadratic input, small circuits, first fall degree alone and surviving-branch counts do not establish that cost.

In the declared conventional uniform-target/full-independent-row model, fixed arity m=4 with setup exponent sigma<=2 and matrix exponent2 needs an empty-inclusive query exponent tau<1 for a sub-square-root balance. The general necessary budget is max(sigma,2)+tau<m-1. Justify yield/rank/target assumptions; other one-dependency architectures are not excluded. Preserve the dense quotient and explicit-witness-list obstructions. If a candidate only renames their work, record the obstruction and pursue a different representation.

Current literature includes Chan-Itakura-Ma-Yu2610.42012 with subquadratic-preprocessing integer indexing, and Vannimmen2611.09656 plus Dinur-Golovnev-Yan-Zhu2611.13294 with generic reporting, including current-output-sensitive bounds. Do not repeat blanket quadratic-preprocessing or global-output-size-only assertions. Generic reporting applies to encoded sums under its range assumptions, but retains construction over L^m inputs. Integer arithmetic hashes need a charged lossless DLP-free elliptic reduction. Exact low scalar-residue oracles are not free.

## Evidence and execution

Use Rust for cryptographic computations/tests. Preserve original curve, signs, failures, complete outputs and strongest maintained controls. Companion monic source blobec949fa765aee1686ba01a74f420f8f2f0a77df3 is covered by CI45089472283 at96c9101315d9f0c7ca5fee2af18cdb10d16c645d. Its printed reference_scan_ms is quadratic unordered S3-pair enumeration, not O(B) group-complement scanning. Balanced construction is a separate equality check; extract uses Newton construction. The logged timings are not admitted complete-solver benchmarks.

Retain iteration1's adverse17923 versus8816 field-multiplication diagnostic and original-lock CI45089569480 at109fc024e33f122fd678d2c93a18b2c0438a8668. Bind later claims to their own sources. Actual performance work requires repository checkpoint/admission and applicable EKS rules; first freeze the specified solver and exact native natural-target generator. No complete ECDLP gain without signed rows, authentic rank/dependency, target recovery, failures, setup, storage, linear algebra and strongest baselines charged. If compute is unavailable, continue useful proofs/source work without inventing measurements.

For asymptotics specify a growing prime-field family and quantitative smooth p+1 structure. Do not silently assume infinitely many Mersenne primes or a prime-order curve family. Fixed31-bit data establish finite-instance behavior only. Use citations and required claim records, signed independent verification and formal proofs at their actual scopes.

## Durable progress

Save derivations, code, failures, commands, hashes, standing and next obligations in the existing PR, STATE.json, session log and illustrated report. The confirmed Space is page_space_038e4361a33c8191af04e7a127104c0f; its root is page_6b0398c4e92481918fd28c1f69875748. Update https://chatgpt.com/space/page_729b0b80bdb4819193f457c80ecabe84 using guarded edits and readback. The historical uncertain creation is resolved; do not recreate Space/Page. Existing task6b0356b62c58819095775adf491e4492 is attached there. Preserve its schedule; do not create another task or send messages to other people.

## Concurrent growing-family checkpoint and build restoration

Preserve iteration7/SMOOTH_TORUS_FAMILY.md from50081b81e8eb3addccf5add80d6bd8f7683218a9. Its Linnik argument constructs growing primes p=-1 mod2^(k+2), giving a q=2^k split Dickson fibre with q=Omega(p^(1/L)); it supplies field/fibre availability only. The ordinary source curve, large prime subgroup, adequate liftable base, exact solver and full complexity obligations remain separate. Independent review is in progress. [pcc:unverified]

The initial iteration6 build failed on inherited spec.rs merge damage. The preserved prerequisite audit restores289 lines from both merge parents with zero deletions; original arithmetic examples, manifest and lock are unchanged. SOURCE_FREEZE in iteration6/evidence/inherited-spec-restoration records commands before rerun, including restored spec regression tests. Read actual new CI status before asserting success. Root interactive session is completing this repair; avoid duplicate source/CI writes while it is active.

Companion #4924 has advanced to observed c4bfab5de0af0e8f0a4e5bea1be056859d0ec273 and changed monic source after the old matched96c910/70e048 result. The earlier paragraph is a historical comparator audit, not evidence for the newest bytes. Obtain matching receipts for current source.
