# Cayley chord extraction — frozen development correctness protocol

Date: 2026-11-25. Scope: bounded, public/synthetic M31 correctness checks. No timing or relation-yield benchmark, paid experiment, repository publication, or private-key target.

## Component

For a public affine target R=(w,y), y != 0, on y²=x³+a*x+b over M31, recover all affine ordered pairs P+Q=R whose Cayley-coordinate product is a supplied torus value. Represent nonidentity torus values by lambda=i(1+c)/(1-c); represent c=1 separately. The production candidate uses a chord-slope quadratic and the reference uses the symmetric Semaev quadratic plus separate ordinate recovery. Compare complete deduplicated signed point sets, not just polynomial roots.

## Frozen curves

Primary existing source fixture: p=2147483647, a=2147483644, b=21, n=2147487637, G=(1391443431,177979044), cofactor 1, from crypto::cryptanalysis::ic_boundary::fp_p31_subexp_mersenne_instance.

Secondary existing fixture: p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251), from research/prime31_ic_20261115/native/lib.rs::PRIME31. Both curves use M31; they are distinct fixtures and results must not be pooled as identical instances. Verify field/order primality, nonsingularity, the generator equation and unreduced [n]G=O. With prime n and 2n above Hasse's upper bound this certifies the full prime group order.

## Reproducibility

Protocol: isogeny-labs/experiment-seeds/v1. Study ID cayley-chord-m31-correctness-v1. Master seed 0x4341594c45593231. Explicit replicate keys 0x5245504c00000001, 0x5245504c00000002, 0x5245504c00000003, 0x5245504c00000004. PRNG: standalone Rust SplitMix64 with fixed published constants, unsigned wrapping 64-bit operations. No language hashes, clocks, host identifiers or entropy inputs.

Domain separation: SplitMix64 finalizer(master XOR replicate_key XOR fixture_tag XOR stream_tag). Fixture tags 0x4355525600000001 and 0x4355525600000002. Target stream tag 0x5441524745540001; supplied-lambda stream tag 0x4c414d4244410001; known-positive-control witness stream tag 0x5749544e45535301. Streams depend on logical identity, never worker identity. Modulo sampling is used only for deterministic development fixtures; these are not unbiased statistical experiments. The two implementations receive byte-identical public (curve,R,selector) inputs. No algorithm-side randomness.

For each curve and each of 4 replicate keys: 16 public targets, each with 16 unplanted supplied lambdas; separately c=1, lambda=-w, lambda=0, a known-positive distinct-pair selector, a doubling selector, and a tangent-through-minus-R selector. Target-generation scalars and positive-control witness points stay in the evaluator; extractor arguments contain only public curve, R and selector. Positive controls establish correctness only, not natural relation yield.

## Acceptance

- Exactly matching complete signed affine ordered solution sets from candidate and independent reference on every case.
- Every returned point lies on the supplied curve, every pair sums to R, and every selector equation holds. Independently check the Cayley product in Fp², not only its symmetric formula.
- At most four ordered affine solutions per selector.
- Each known-positive control returns its withheld pair. Exercise repeated summands, c=1 and lambda=-w; reject malformed targets and noncanonical lambda inputs.
- Verify the symbolic factorization and discriminant identities through exact algebra and finite fixtures. No claim of exhaustive enumeration of Fp or of a formal Lean proof.
- Preserve every failure and correction. Run the finalized corpus twice with byte-identical non-timing outputs. Retain raw case records, compiler version, source hashes and commands.

## Bounds and stopping

One local Rust compile/run at a time; at most 2 fixture curves, 128 targets, 2,816 planned selector cases plus deterministic edge cases. Maximum 60 seconds per development run, 1 CPU, 512 MiB virtual memory for the test process; compile cap 120 seconds and 1 GiB virtual memory. No wall-clock speedup is inferred. Stop optional checks once the stated component gate is satisfied. [pcc:unverified]

## Next experiment (not executed here)

Compare Gaussian-smoothness-conditioned versus uniform torus candidates against a frozen, negation-matched factor base using the existing public-target stream, with all rejected norm samples and candidate-generation work charged. Useful independent relation rank and target-determining coefficients, not root counts, decide promotion. Existing benchmark isolation/publication/resource rules apply before a scientific timing campaign.

## Pre-execution amendment 1.1

No fixture generation or test execution had occurred when this correction was recorded. Direct XOR of low-valued replicate and fixture IDs can cancel, reusing a child seed across different fixtures. Replace only that derivation with hierarchical mixing:

`keyed = mix64(mix64(master XOR fixture_tag) XOR replicate_key)`

`child = mix64(keyed XOR stream_tag)`

Retain all master/replicate/tag values and the planned case panel. Verify the 24 derived child seeds are pairwise distinct. This records the superseded expression transparently rather than silently editing frozen seed history.

The new exact average-yield bound also changes the scientific follow-up: a target-independent product-label policy has expected unordered B+B candidates at most |B|/(r-1) for uniform nonzero R, versus (|B|-1)/(r-1) success for direct P-in-B sampling. Consequently any subsequent smooth-label performance proposal must identify target-dependent enrichment or a cheaper arithmetic/membership mechanism; a comparison only against uniform torus labels is insufficient. The bounded development checks remain unchanged.
