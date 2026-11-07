# Independent native signed-collision preflight review

Scoped preflight source fidelity supported; candidate uncompiled and unexecuted. Exact published c531856dcb725a35e5b463d67cc1e37f73d8790e, candidated8c496084c1e0397a8e961b94b85575888502403dc99e0d4df13a093c009328a. Prior L1 and frozen native core were inspected before judging the correction. Manifest/Git bytes match; prefix before modified function and entire tail from chart construction are identical.

Opt-in metadata bounds and existing prime predicates precede constructor; canonical nonzero point/coordinate/coefficient guards use comparisons before source-field operations. Constructed coefficients then match, and valid G/Q plus rG=O are checked before transfer. Zero images return Err after transfer. The false branch retains original family/validity/coefficient/image assertions in their original order without new field arithmetic. This resolves the enumerated L1 late-check paths, not all possible panics.

Under existing group operations and prime predicate correctness, nonzero G with rG=O and prime r has exact orderr. Valid Q does not establish subgroup membership, correctly left as a premise. Nonzero transferred image has orderr only conditional on transfer homomorphism. Collision sign equation, zero-denominator handling and final source multiplication equality are unchanged.

Bounds are coherent: p<2^31 gives p²<2^62; canonical scalar residues below r<=2^63 add without overflowing u64; products use u128. Existing public is_prime and counted constructors remain source dependencies, not an independently proved primality certificate in this review.

Primality and comparison work remains unpriced outside field counters. Counted source validity/order checks are reclassified into signed_collision_source_preflight. Err carries no Outcome/phases, so external receipts must account failed work; unknown is not zero. Constructor/transfer/observer/allocation/counter and later arithmetic can still panic. No runtime equivalence, total-memory or cost comparison follows. Prospective native qualification and full controls remain a later authorized step.

The preflight phase mixes target-independent curve/G/order checks with target-dependent Q validation. Its label supplies no online, cold or reusable-cost boundary. Future measured accounting must define that split; this review assigns neither blanket online cost nor blanket amortization. Byte-identical tail and preserved false-path order are source facts, not runtime equivalence.
