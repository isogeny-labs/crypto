# Native signed-graph Ring word correspondence

2026-11-01. Source: three_leads.rs Ring::add/neg/sub/mul/signed at lines1491-1530. Seven main declarations and ten kernel controls passed with seventeen separate named axiom readbacks. No scientific execution.

## Derived result

For a positive UInt64 modulus r, canonical operands and r<=2^63, literal (a+b)%r returns a canonical residue and decodes to addition in ZMod r. The headroom proof establishes a.toNat+b.toNat<2^64, so both wrapping-value interpretation and checked-add no-overflow interpretation agree. This is a graph-specific bound, not reuse of the earlier field p<2^31 bound.

Native neg is (r-a)%r, unlike earlier field wordNeg=(r-a%r)%r. neg_literal_bridge proves they are equal under canonical a<r; then the established negation theorem applies. Native subtraction composes this exact neg with the new bounded addition theorem. The direct Ring multiplication is the previously checked widened BitVec128 product/remainder followed by UInt64 narrowing. mul_correct uses that literal wide model and supports every positive UInt64 modulus and all UInt64 operands, with no canonical-operand or 2^63 premise.

The signed branch is modeled with an Int sign after decoding native i8. For s=1 or s=-1 it yields canonical output and decodes to s*a. No theorem equates arbitrary native i8 input to a ring multiplier: source treats every sign unequal to1 as negation.

## Controls

Zero and maximal canonical negation; additive boundary r=2^63; overflow outside that bound at r=2^63+1; corresponding natural arithmetic value; noncanonical neg mismatch; wide product versus narrow UInt64 product; sign0 behavior; literal subtraction. Specifically at r=2^63+1 and a=b=2^63, the wrapping word sum is0 but natural modular sum is2^63-1. These controls use a composite modulus as a word-arithmetic boundary, not a claim about constructor-accepted native inputs. Neg17(18) differs from reduced-input wordNeg17(18), showing why canonicality is required by the literal bridge. Widened 2^63*2 modulo17 is1, whereas UInt64 multiplication wraps to0. These are kernel evaluations of exact word models, not compiled Rust experiments.

## Source mapping and remaining obligations

No primality assumption is required for these ring-operation decoding theorems. Ring::new asserts prime r>2 but does not imply the additive headroom bound. Native counters increment before operations and remain unmodeled. Field::neg is not silently substituted for Ring::neg; the canonical bridge is explicit. The wider multiplier is reused only because native code literally widens both inputs before multiplying.

Still required: preservation of canonical offsets and exact +/-1 i8 signs through mutable graph operations; actual modulus bounds for admitted jobs; native Int/i8 decoding correspondence; forest/index/counter invariants; Ring::inv and its nonzero/primality premises; constructor correctness; compiled Rust equivalence. These theorems can now be composed with signed graph algebra once those input invariants are established. Independent semantic review of this new packet is pending.

## Verification provenance

Fresh GraphRing, Controls and Readback native Lean exits0. Pre/post input maps agree; cached imported helper binaries and Mathlib remain the explicit trust boundary. The original first main-only check also passed; its receipt retained. No failed proof attempts in this packet. Source-first plan predates implementation. Primary existing references: native Ring source, WordAdd11/WordClassifier canonical arithmetic, Wide.wideMul value/decoding proof. No novelty claim or cost measurement.
