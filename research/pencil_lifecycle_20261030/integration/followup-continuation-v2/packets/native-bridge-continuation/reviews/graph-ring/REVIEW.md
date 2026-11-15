# Independent graph Ring word review

Seven main declarations, ten producer controls and nine independent reviewer controls freshly compiled in an isolated target snapshot. Separate readback covers all26 declarations and only standard propext/Classical.choice/Quot.sound axioms. The prospective plan and source reading preceded proof-body inspection. No discrepancy was found within the stated conditional value semantics.

## Source-first findings

Native Ring (`three_leads.rs.txt:1492-1534`) implements add=(a+b)%r, neg=(r-a)%r, sub via add and neg, and mul by widening both operands to u128 before multiplying/reducing and then casting to u64. The definitions match those literal expressions. In particular neg is not silently replaced by neg(a%r): the bridge is proved only for canonical a<r.

The exact additive premise is canonical a,b<r and r<=2^63. Then a+b<=2r-2<=2^64-2, preventing overflow. Equality r=2^63 is allowed mathematically, even though this composite modulus is not admitted by native Ring::new. The constructor checks primality and r>2 but does not enforce r<=2^63, so constructor success alone cannot discharge the headroom premise. Outside-bound examples describe UInt64 wrapping; a Rust debug/overflow-checking build can panic instead. No compiled overflow equivalence is inferred from those negative controls.

Negation needs only positive r and canonical a<r: r-a does not underflow. The producer bridge uses the existing reduced-input neg theorem after proving a%r=a. The widened multiplication theorem correctly permits arbitrary u64 operands and any positive u64 modulus. Its dependency Wide.lean proves the product fits128bits and the remainder is below r<2^64, making narrowing exact. This review inspected that dependency source and hash-bound its retained object; it did not independently rebuild all imported arithmetic modules.

Native signed(sign,a) returns a exactly for i8 sign==1 and otherwise neg(a). The theorem models sign by Int and assumes sign=1 or-1. It neither implements nor proves an i8 decoding bridge or graph-state sign preservation. Sign2 returns neg(a), not2*a; sign1 passes a through and does not canonicalize it.

Inverse, prime-constructor correctness, scalar counter increments/overflow, graph potential preservation, compiler output and whole-machine execution are outside these declarations. Counters can fail independently of correct returned-value arithmetic; no claim about them follows here.

## Independent controls

Nine kernel controls cover maximum allowed additive sum; endpoint subtraction; negation at maximal u64 modulus; maximal u64 operands in widened multiplication (3364 modulo18446744073709551557) versus narrow multiplication (1); invalid sign2; sign1 retaining noncanonical18; literal noncanonical underflow; and positive modulus1 multiplication. These complement the ten producer controls. No native_decide was used.

## Evidence boundary

Fresh replay used Lean4.34.1 with new target oleans. Producer source and adjacent dependency .lean/.olean files were hash-bound around replay. Broader Mathlib caches were reused and not independently kernel-rechecked. Private input snapshots have no immutable publication commit attached in this review. Thus full formal-review workflow completion remains false despite the scoped replay/source assessment. No scientific execution, curve evaluation, new measurements or cohort results occurred. Full1260 scope and null cost ratio remain.

Audit correction: the first parser counted24 declarations because two axiom-free lines use a different readback form. The explicit expected-count check failed. Supporting both readback forms yields26 declarations, including two with empty axiom sets. Native Lean checks themselves passed first attempts.
