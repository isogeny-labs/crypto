# Exact sublinear queries on the Prime31 elliptic pair spectrum

- **Status:** investigating
- **Kind:** mathematical algorithm and complexity gap; separate native-build dependency
- **Affected work:** [Prime31 study](../research/prime31_summation_compression_20261122/README.md), [direct Dickson/S5 structural gate](../research/pplus_dickson_s5_structure_20261127/REPORT.md), [PR #4977](https://github.com/isogeny-labs/crypto/pull/4977), [PR #4979](https://github.com/isogeny-labs/crypto/pull/4979), and [PR #5287](https://github.com/isogeny-labs/crypto/pull/5287)
- **Next action owner:** coordinator of the direct Dickson/Semaev workstream
- **Last updated:** 2026-11-27

## Problem

The rational quadratic and pair-spectrum representations are exact, but no new
algorithm has a justified sublinear empty-inclusive query cost for the actual
Dickson-derived pair spectrum with comparable charged setup and complete
witness recovery. The fixed field `p=2^31-1` supplies a concrete regression
instance; asymptotic claims require a specified growing family.
[pcc:unverified]

Let `A={+P_i,-P_i}` be the signed rational base with `B` distinct abscissae,
`X=x((A+A) minus O)`, and `D=|X|<=B^2`. The current exact baseline uses the
pair spectrum and `S_3`/complement search with quasi-linear-in-`D` target work,
plus the requested output. Low ordinary rank of an `S_3` or `S_5` evaluation
matrix, quadratic input equations, and fast evaluation of a supplied witness
do not establish faster relation discovery. [pcc:unverified]

The direct four-leaf gate strengthens the obstruction: for `n=8,16`, its
pair-flattened `S_5` kernels saturate the algebraic bounds
`rank(K)=15` and `rank(Delta_s)=rank(Delta_p)=30`, but rank deficiency predicts
a hit on six of six public control cells whose exact zero count is zero. The
compact degree-four divisor circuit still represents `binom(n+3,4)` states.
[pcc:unverified]

## Evidence and uncertainty

- **Written algebra:** the [pencil note](../research/prime31_summation_compression_20261122/iteration10/S3_PENCIL_CONTRACTION.md) and independent review provide rank at most three, a degree-`D` column product, exact formal target degree `2D-1` in the split quotient, and the limited dense-format costs. The source discriminants already split; a nonsquare test prunes no nondegenerate component. These are unverified written claims, not general lower bounds. [pcc:unverified]
- **Direct S5 packet:** [v2 analysis](../research/pplus_dickson_s5_structure_20261127/evidence-v2/analysis.txt) records nine structurally exact cells, six singularity-predicate disagreements, no exact zero oracle and `STRUCTURE_ONLY_STOP`. The [verifier](../research/pplus_dickson_s5_structure_20261127/evidence-v2/verifier.txt) independently reports zero implemented mismatches; the [proof model](../research/pplus_dickson_s5_structure_20261127/PROOF_MODEL.md) states its non-ZK and non-PCC scope. [pcc:unverified]
- **Available parameters:** the [family correction](../research/prime31_summation_compression_20261122/iteration10/FAMILY_SIZE_CORRECTION.md) applies Chang's smooth-modulus theorem to chosen fields and supports any fixed `0<beta<5/12` for the stated dense full-fibre construction on a supplied eligible odd-order curve. Source construction, a required prime-order family, useful rank, and exact query solving remain separate. [pcc:unverified]
- **Follow-up:** the leaf-to-pair norm identity and F7 counterexample are in [the norm review](../research/prime31_summation_compression_20261122/iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md). The exact norm retains `B^2` chart structure but the pair selector need not inherit a uniform normalized `D_2` tower. The [parallel-result review](../research/prime31_summation_compression_20261122/iteration10/parallel_review/REVIEW.md) retains the forced resultant multiplicities and cancellation-reduced support bound. [pcc:unverified]
- **Native execution:** the third dedicated original-root run [45113871392](https://github.com/isogeny-labs/crypto/actions/runs/45113871392) failed compilation in unchanged `binary_hyperelliptic_index_calculus.rs` before any new test or issuer ran. The [source audit](../research/prime31_summation_compression_20261122/iteration6/evidence/third-blocker-source-audit/AUDIT.md) identifies the introducing merge and both valid parents; a complete intended union was not reconstructed. No substituted crate is a full-root pass. [pcc:unverified]
- **Missing capability:** an implicit target-query contraction and its proof; independently qualified native execution; any admitted performance run. The shared KB and Conductor were unavailable during the direct-S5 study. The local deterministic Rust replay required no cloud or object store.

This evidence does not establish that every compact index is impossible.
Arbitrary-set linear-span counts and dense output allocations are restricted
representation results. A small counterexample rejects one automatic folding
rule; it does not reject all other coordinates or a Prime31-specific
construction. [pcc:unverified]

## What would unblock it

- [ ] **Requested component advance:** give an exact `O-tilde(D^theta+h)` target-query algorithm for fixed `theta<1` against the named quasi-linear-in-`D` component baseline, with comparable charged setup, all misses, signs, collisions, zero subtotals, rational recovery and requested output/replay included. A logarithmic or other precisely proved asymptotic improvement may also count at its actual component scope. [pcc:unverified]
- [ ] **Current direct-S5 gate:** instantiate the degree-two pair-divisor transfer with fewer than `n^2` target-specific field slots and operations, a fitted slope below `1.9` on four growing sizes, and exact diagonal/off-diagonal, lift, sign, infinity, witness and miss handling. It may not list elliptic `A+A`, scan `n^2` leaf pairs, materialize the `N^2` kernel or retain the `binom(n+3,4)` divisor algebra. [pcc:unverified]
- [ ] **Stronger conditional pipeline target:** if `D=Theta(B^2)`, setup is `O-tilde(D)`, matrix work is `B^2` and the conventional uniform-target/full-independent-row model applies, establish `theta<1/2` plus the remaining input-family and useful-rank assumptions before claiming that complete exponent target. This is not a requirement for recognizing every component improvement. [pcc:unverified]
- [ ] **Execution:** freeze the candidate source, independently check completeness and replay against the original arithmetic, and price target setup, unsuccessful queries, storage, certificate traffic and output. Repair the original root build through a separately reviewed preservation of both relevant parent implementations before quoting a full-root native pass.

## Next discriminating step

- **Action or proof obligation:** for `q_(s,p)(T)=T^2-sT+p`, propagate the exact remainder recurrence `u' = u^2 s+2uv`, `v'=v^2-u^2p-2` for `D_(2m) mod q`. Combine two copies with the rank-15 target polar form and prove a backwards transfer or subresultant representation whose live state is `o(n^2)` before implementing a larger cell. [pcc:unverified]
- **Inputs and controls:** retain the exact pair-spectrum scan, the fixed balanced-tree cancellation counterexample, the nonrational auxiliary ghost, the F7 failure of automatic `D_2` folding, the v2 planted/control targets, every multiplicity partition, nonlifting roots and target infinity.
- **Execution limits and stop condition:** first inspect symbolic state number and degree through small doubling depths. Stop this arm without another census if either one is already `Theta(n^2)` before second-pair elimination. Any executable follow-up retains the frozen no-amortization and source-custody rules.
- **Outcomes and next actions:** a proved subquadratic transfer becomes a separately preregistered source candidate. Quadratic state growth closes only this transfer and moves to the torus-binomial formulation. An inconclusive derivation remains this named gap; infrastructure failure does not decide the mathematics.
- **Recheck trigger:** a new exact contraction lemma, a reviewed root-source reconstruction, or restored supported execution access.

## Resolution and history

Unresolved. Core pair-spectrum notes are pinned at
`d6e1d03aff64ff168bd1d1d565a08c8f5032ef62`. On 2026-11-27, the direct
Dickson/S5 gate added exact public evidence for the constant-rank kernel and
the singularity/zero-entry separation, then selected the degree-two
pair-divisor transfer as the next proof obligation. Publication of this record
does not meet the component or pipeline criterion. [pcc:unverified]
