# Exact sublinear queries on the Prime31 elliptic pair spectrum

- **Status:** open
- **Kind:** mathematical algorithm and complexity gap; separate native-build dependency
- **Affected work:** [Prime31 study](../research/prime31_summation_compression_20261122/README.md), [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977), and [PR #4979](https://github.com/isogeny-labs/crypto/pull/4979)
- **Next action owner:** coordinator of the existing summation-polynomial task and its available algebra/Rust/validation workers
- **Last updated:** 2026-11-23

## Problem

The rational quadratic and pair-spectrum representations are exact, but no new algorithm has a justified sublinear empty-inclusive query cost for the actual Dickson-derived pair spectrum with comparable charged setup and complete witness recovery. The fixed field p=2^31-1 supplies a concrete regression instance; asymptotic claims require a specified growing family. [pcc:unverified]

Let A={+P_i,-P_i} be the signed rational base with B distinct abscissae, X=x((A+A) minus O), and D=|X|<=B^2. The current exact baseline uses the pair spectrum and S3/complement search with quasi-linear-in-D target work, plus the requested output. Low ordinary rank of the S3 evaluation matrix, quadratic input equations, and fast evaluation of a supplied witness do not establish faster relation discovery. [pcc:unverified]

## Evidence and uncertainty

- **Written algebra:** the [pencil note](../research/prime31_summation_compression_20261122/iteration10/S3_PENCIL_CONTRACTION.md) and independent review provide rank at most three, a degree-D column product, exact formal target degree 2D-1 in the split quotient, and the limited dense-format costs. The source discriminants already split; a nonsquare test prunes no nondegenerate component. These are unverified written claims, not general lower bounds. [pcc:unverified]
- **Available parameters:** the [family correction](../research/prime31_summation_compression_20261122/iteration10/FAMILY_SIZE_CORRECTION.md) applies Chang's smooth-modulus theorem to chosen fields and supports any fixed 0<beta<5/12 for the stated dense full-fibre construction on a supplied eligible odd-order curve. Source construction, a required prime-order family, useful rank, and exact query solving remain separate. [pcc:unverified]
- **Follow-up:** the leaf-to-pair norm identity and F7 counterexample are in [the norm review](../research/prime31_summation_compression_20261122/iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md). The exact norm retains B^2 chart structure but the pair selector need not inherit a uniform normalized D2 tower. The [parallel-result review](../research/prime31_summation_compression_20261122/iteration10/parallel_review/REVIEW.md) retains the forced resultant multiplicities and cancellation-reduced support bound. [pcc:unverified]
- **Native execution:** the third dedicated original-root run [45113871392](https://github.com/isogeny-labs/crypto/actions/runs/45113871392) failed compilation in unchanged binary_hyperelliptic_index_calculus.rs before any new test or issuer ran. The [source audit](../research/prime31_summation_compression_20261122/iteration6/evidence/third-blocker-source-audit/AUDIT.md) identifies the introducing merge and both valid parents; a complete intended union was not reconstructed. No substituted crate is a full-root pass. [pcc:unverified]
- **Missing capability:** a new target-query algorithm and its proof; independently qualified native execution; any admitted performance run. The KB, Conductor, EKS credentials and signed verifier runtime are unavailable in this interactive environment. Native Space access has an independent unresolved service/authorization state.

This evidence does not establish that every compact index is impossible. Arbitrary-set linear-span counts and dense output allocations are restricted representation results. A small counterexample rejects one automatic folding rule; it does not reject all other coordinates or a Prime31-specific construction. [pcc:unverified]

## What would unblock it

- [ ] **Requested component advance:** give an exact O-tilde(D^theta+h) target-query algorithm for fixed theta<1 against the named quasi-linear-in-D component baseline, with comparable charged setup, all misses, signs, collisions, zero subtotals, rational recovery and requested output/replay included. A logarithmic or other precisely proved asymptotic improvement may also count at its actual component scope. [pcc:unverified]
- [ ] **Stronger conditional pipeline target:** if D=Theta(B^2), setup is O-tilde(D), matrix work is B^2 and the conventional uniform-target/full-independent-row model applies, establish theta<1/2 plus the remaining input-family and useful-rank assumptions before claiming that complete exponent target. This is not a requirement for recognizing every component improvement. [pcc:unverified]
- [ ] **Execution:** freeze the candidate source, independently check completeness and replay against the original arithmetic, and price target setup, unsuccessful queries, storage, certificate traffic and output. Repair the original root build through a separately reviewed preservation of both relevant parent implementations before quoting a full-root native pass.

## Next discriminating step

- **Proof obligation:** exploit the actual leaf algebra M and its two-factor swap symmetry in R_M J_M=kappa C_M^2 without materializing a target table of quadratic size in D. Give explicit intermediate dimensions and operation counts. Merely taking another norm or resultant is not a query algorithm. [pcc:unverified]
- **Controls:** retain the exact pair-spectrum scan, the fixed balanced-tree cancellation counterexample, the nonrational auxiliary ghost, and the F7 failure of automatic D2 folding. Include empty targets and repeated leaves, rather than only planted positive relations.
- **Execution limits:** continue written algebra and source review with the existing task resources. New native or performance work follows the repository's original-source, admission, artifact and budget rules; this record authorizes no extra spending or substitute benchmark.
- **Outcomes:** a valid component bound becomes a source-frozen candidate for independent implementation and tests. A counterexample closes only its stated mechanism. An inconclusive derivation remains a named gap and motivates a different representation. Infrastructure failure does not decide the mathematics.
- **Recheck trigger:** a new exact contraction lemma, a reviewed root-source reconstruction, or restored supported execution access.

## Resolution and history

Unresolved. Core notes are pinned at d6e1d03aff64ff168bd1d1d565a08c8f5032ef62. The current PR adds independent follow-up reviews and separates the component target from the stronger conditional complete-pipeline target. Publication of this record does not meet either algorithmic criterion. [pcc:unverified]
