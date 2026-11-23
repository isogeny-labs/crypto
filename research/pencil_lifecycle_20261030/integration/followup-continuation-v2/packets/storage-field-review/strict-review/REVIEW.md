# Strict storage endpoint: independent technical review

Joint A, reviewer `/root/strict_storage_reviewer`; 2026-11-01. Reviewed immutable producer commit `e45227d1b8f420414c71de1cbecd787b887f69e7`, dispatched under the prospective review plan at `660bbb72be`. Verdict: **validated_scoped** for the four strengthened endpoint declarations. Machine status: **passed**. Source fidelity: **faithful_scoped**. This is an ordinary independent source review and fresh replay, with inherited conversation context, not blind re-derivation.

## Claims and source wiring

| Declaration | Checked scope |
|---|---|
| `ReductionWord.step_exists_strict` | Executes the existing residual exact division, monic normalization and remainder of negative ordinate. Exports canonical and normal output words, strict remainder length, nonempty support, array capacities and decoded validity. |
| `ReductionWord.initialized_run_strict` | Uses the original `ReductionWord.Run`, with its original stop and step constructors. The short branch returns both input normality assumptions and strict input length; the long branch constructs one step followed by stop and propagates the strict step outputs. |
| `ReductionWord.reduction_exists_strict` | Performs entry monic normalization and initial remainder, obtains normalized short ordinate from remainder totality, and invokes the same initialized Run. |
| `ReductionWord.fallback_reduced_strict` | Propagates the strengthened endpoint through the existing two-EEA fallback Chain and Divided relation, its first remainder, entry remainder and original Run. |

At the pinned native source, `Jacobian::reduce` first applies `pmonic`, then `prem`, and loops while `u.len()>3`. Its body takes the exact residual quotient, normalizes it, then takes `prem(-v,new_u)`. This agrees with the original Run constructors; no new weakened execution relation was introduced. The native `Divisor::from` assertion is precisely nonempty support, support length at most three, and ordinate length strictly smaller than support length. All three are now exported explicitly, with normality of both lists. The bound `b.length<=2` additionally follows from those storage bounds. The canonicality and validity outputs remain separately present.

## Exact assumptions

All four declarations use a UInt64 modulus whose natural value is prime, greater than one and strictly below `2^31`. Canonicality means every supplied coefficient is below that modulus; Normal means the final stored coefficient is not zero, with empty list accepted as normalized zero. The decoder is the existing prime-field polynomial decoder.

The step theorem assumes canonical f,u,v, normalized u, degree f exactly five, degree u between three and four, degree v strictly below degree u, and divisibility of f-v² by u. It does not assume normalized v; this is unnecessary for the step's decoded degree hypothesis and output remainder normalization.

The initialized theorem additionally assumes normalized v, monic support (last stored coefficient one), strict input list length and degree u at most four. Entry reduction instead assumes nonempty normalized support, canonical words, degree u at most four and residual divisibility; it constructs the initial normalization and short remainder. The fallback theorem assumes canonical ua,ub,va,vb,f; normalized and decoded-nonzero supports; support degrees at most two; degree f five; and both input residual divisibilities. It needs no ordinate normalization or support monicity at that interface. Smoothness is not an assumption and is not needed for these storage and polynomial-divisibility conclusions.

These assumptions are nonvacuous: p=2, ua=ub=[1], va=vb=[], and f=[0,0,0,0,0,1] meet the fallback conditions. This is a direct mathematical satisfiability witness, not an additional machine-replayed full-endpoint fixture.

## Adversarial boundaries and controls

The pinned producer control demonstrates U=[1], V=[0] fits both arrays but violates strict length. The identity control demonstrates normalized U=[1], V=[] satisfies strict length. The reviewer compiled four further declarations: empty support admits no strictly shorter list; [0] is not Normal; [0] decodes to zero but is not an empty list; and a degree-five f cannot have zero residual f-v². The last follows the existing quintic residual theorem: a nonzero square has even degree, whereas f has degree five. Thus the quotient nonzero argument is substantive rather than an accidental equation with zero support. The false theorem fixture exited 1. No unexpected Lean failure occurred in this replay.

## Fresh machine evidence and custody

Seven exact pinned `.lean` inputs were extracted using `git show` into a new private directory. Six modules (four theorem modules, producer controls and readback) compiled freshly with the existing pinned Lean executable, `-j1 -M4096`, per-module timeout 120 seconds. All six named producer readbacks passed an exact ordered inventory audit allowing only propext, Classical.choice and Quot.sound; no-axiom output is also accepted. A reviewer-built Rust checker rejects forbidden-axiom, empty-output and wrong-name control inputs. Four additional reviewer controls passed their own exact inventory and axiom audit.

Existing imported `.olean` caches are reused; this review does not recompile every helper, independently recheck the kernel, or prove that a dependency binary corresponds to its source. Non-union helper files were privately copied; library/dependency union trees remain reused read-only inputs and are recursively content-hashed before and after replay. `before.sha256` and `bindings-check.txt` bind the resulting input boundary. Pre/post hashes establish stable observed bytes, not protection against hypothetical transient shared-cache edits. No such edit was observed. The Lean executable is hashed; the entire installed toolchain was not rebuilt.

216 of 217 copied helper source files match archived Git blobs at the producer commit. The unmatched file is `deps/44/Readback.lean`, Git blob `0052ce362db2bbd98e372d5358228148f637f4cd`; it contains readbacks for SameSupportClassifier. It is preserved and hashed, and the source import traversal confirms it is outside this replay's local import closure. Therefore it is not silently represented as a pinned proof dependency. The traversal records eleven external library source modules unavailable at these source roots; their compiled dependency boundary is retained, not claimed fully source-rebuilt.

Current main was separately inspected through the GitHub tree API at `a8f8a46c84472caccad0d964c35e213d9f001c35`. The repository has no root `ledger` directory. The producer branch was not merged or mutated for this read-only technical review, and no official scientific ledger transition is made.

## Remaining bridges

1. This is the generic fallback relation, not the optimized nonzero-norm/slope branch.
2. Native source fidelity is manual comparison to the extracted Rust source; there is no compiled Rust-to-Lean equivalence theorem or executable Rust replay in this review.
3. The hypotheses concern canonical prime-field word arithmetic (n=1, no extension/tables), not every possible native Field configuration.
4. Decoded Mumford polynomial validity and storage assertions do not prove divisor-class addition semantics, Kummer closure/sign recovery, collector invariants or relation-information metrics.
5. Native event counters, allocations, time and matched cohort costs were not measured.

## Reviewer attestation

I did not author the four strict declarations. I read the published plan, review-evidence skill and Lean reference, all seven strict sources, producer HANDOFF and inventory, original ReductionRun, ReductionQuinticBound, FallbackWordExact, FallbackWordRemainder, Normalized definitions and the native reduce/add/Divisor sections; exact input and helper source lists are in the accompanying manifests. Dependency source hashing and import traversal are mechanical custody checks, not a claim of line-by-line semantic review of every helper. I read no sibling reviewer report or new sibling output. No blind re-derivation is claimed. Joint A holds at the stated scope. No scientific workload, cloud action or campaign authorization change occurred. Publication, PDF/diagram rendering and any official promotion remain with the parent coordinator.
