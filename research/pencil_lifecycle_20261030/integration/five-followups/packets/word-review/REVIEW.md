# Independent UInt64 residue correspondence review

Verdict: faithful at the five stated primitive-theorem scopes. Fresh compile of CanonicalResidue and UInt64Residue and separate declaration/axiom readback all exited zero. This is an independent fresh target-object build using existing pinned dependencies, not a new independent build of Lean or all dependencies.

Review plan SHA256 a7990508444b4f21b1a7ca45627282b3f0ebc55ab4dd093a74f1f01950c56e4f was written after reading statements and producer status, before proof bodies. Candidate SHA256 27e4193572a3fbb5d89db68528cf9f0887f87db2e31693cc951ff3f07f50c8fd matches the requested identity. Producer manifest SHA256 3ce85e913e980c7c48fdfdda4c80ded72afadb16335501c9f090db8e201f0754 matches; all 41 listed files match their hashes. Native producer receipts for candidate compile/readback have exit zero. Fresh commands and stdout are retained.

## Statement fidelity
1. canonical_product_bound: from canonical a,b<p<2^31 derives a*b<2^64 in Nat. The bound is conservative but sufficient, not asserted optimal.
2. uint64_mul_mod: actual Lean UInt64 multiplication followed by UInt64 remainder decodes to unbounded multiplication modulo p. Canonical input assumptions imply p>0, excluding division by zero.
3. uint64_mul_decode: the same result interpreted in ZMod p equals the product of the decoded inputs. No primality is required; ZMod is used as a ring, not silently as a field.
4. uint64_square_decode: the diagonal specialization, with exactly the same range restriction.
5. uint64_mul_canonical: positive modulus alone ensures the remainder is canonical, even when multiplication wrapped. This does not imply correct field multiplication without the other theorem's hypotheses.

Proof bodies apply UInt64.toNat_mul, which exposes reduction modulo 2^64, then remove that reduction using the product bound. UInt64.toNat_mod relates word remainder to natural remainder. Source Init/Data/UInt/Basic.lean lines596 and632 defines these using BitVec multiplication and unsigned remainder; Lemmas.lean lines141-143 supplies the generated decoding theorems. The checked core source is the pinned mirror under /var/tmp/pencil-same-support-lean-local-v1-wco_em6r/sources. Rust helper bm at three_leads.rs:190-193 and bs:194-197 use the matching arithmetic expressions, preceded by counter increments. Public Field dispatch reaches these helpers for the odd-characteristic n=1 path with no lookup table; other paths are not covered by this correspondence.

Fresh named readback: first four theorems depend only on propext and Quot.sound; canonical-output theorem only on propext. No sorryAx or extra axiom is present. The producer's list also includes Classical.choice because it describes 11 readbacks including imported Nat results; that is conservative but less precise for these five.

## Scope and controls
The explicit wrapping counterexample uses a noncanonical input for modulus5: 2^63 * 2 wraps to zero, while the natural product modulo5 is1. It demonstrates arbitrary-input overflow but does not isolate dropping only the p bound while preserving canonical operands. A useful additional review control is p=2^63+1, a=b=2^63 (canonical): word product is0, mathematical residue is1. This identity follows from a=p-1 and 2^126 divisible by2^64; no new control execution was performed.

Theorem sources do not prove Rust compilation correctness, counter headroom, word negation/subtraction, cross-scaled branch composition, or invariant preservation. Counts may overflow before arithmetic if their headroom assumptions fail. Lean's modulo-zero behavior differs from Rust panic behavior, but p>0 is supplied or implied on every relevant theorem domain. These boundaries match producer status and do not invalidate the scoped result.

## Next action
Use these lemmas as dependencies for literal word expressions after carrying canonicality and no-overflow hypotheses through each intermediate. Independently qualify the Nat classifier import repair; it is outside this candidate and remains recorded as failed. No scientific campaign, benchmark, or cost comparison was run in this review.
