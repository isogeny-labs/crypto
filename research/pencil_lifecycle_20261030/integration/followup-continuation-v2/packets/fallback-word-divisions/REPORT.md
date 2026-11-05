# Word fallback numerator and exact divisions

Producer proof packet, 2026-11-01. Independent source-fidelity review pending.

Result. Two completed six-register word-list EEA runs supply the fallback coefficients. Their stored outputs are composed with the numerator operations in native source order. Both exact divisions succeed with canonical normalized stored quotients. The first quotient is nonzero, supplying the divisor condition needed by the subsequent remainder step. Quotient witnesses are derived from the input premises.

Stored identities. The returned U and W satisfy Ua*Ub = U*(D*D) and N = W*D after polynomial decoding. The model records both completed EEA traces, their empty second remainders, and their returned coefficient lists. The exact-division wrapper returns a quotient only when the computed remainder list is empty; the successful case corresponds to excluding the native inexact-division assertion.

Checks. Seven local modules freshly compiled; eleven main declarations, six named fixed controls and seventeen exact named axiom readbacks. Seven private control helpers establish intermediate product and addition values; their dependencies are included in the final readbacks. The false theorem 0 = 1 is rejected and three native checker rejection controls pass. Bound source/dependency hashes are checked per module. Imported helper and Mathlib caches are reused.

Assumptions. Prime modulus 1 < p < 2^31; canonical Ua, Ub, Va, Vb and f; normalized, nonzero decoded Ua and Ub; and Ua dividing f minus Va squared. The latter premise suffices for these exact divisions; the full geometric output requires further properties. No nonzero denominator or fallback quotient witness is assumed separately.

Fixed controls. In the F17 cancellation numerator fixture, Ua=Ub=X-1, Va=1, Vb=-1, f=1, c1=1, c2=0, e1=0 and e2=1 yield N=X-1. Both tested quotients equal 1. Nonmonic constant division (3+6X)/3 returns 1+2X. The model rejects 1/X as inexact and rejects an empty divisor. These are numerator/division controls; they are not a new cohort of genus-2 runs.

Proof boundary. The exact-division model represents an assertion failure by no result; it does not claim equivalence of failure behavior. The successful word-list operations and their mathematical identities are the scope checked. Compiled-Rust correspondence, physical allocation and stack bounds, subsequent remainder processing, and geometric reduction remain separate interfaces. No scientific cohort, timing or cost ratio is reported.

Failures retained. The quotient-normalization proof initially rewrote inside an unreduced Option match, then included a redundant branch already eliminated by simplification. A control helper initially treated an impossible empty-list branch with reflexivity instead of eliminating its contradiction. All failed attempt receipts are preserved. The final chain explicitly retains stopping evidence for both EEA runs.

Source and prior work. Native src/cryptanalysis/three_leads.rs at dafce9c7458c24b28d26c45ea061044e72517725, fallback construction and pexact. Sibling pegcd-six-word-endpoint supplies the EEA word interface; pegcd-fallback-composition supplies the polynomial divisibility result; next-interface-derivations contains the reviewed written construction and literature context. No novelty or performance claim is made.

Next review. Independently check both EEA stopping states, coefficient wiring, exact multiplication/addition order, quotient normalization and the zero-denominator exclusions. Then compose remainder processing and the geometric output invariant. Assigned reviewer agents remain unavailable after usage-limit errors; this packet is producer-checked only.
