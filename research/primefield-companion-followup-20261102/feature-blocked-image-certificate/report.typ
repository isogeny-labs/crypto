#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Shared blocks for adaptive image recovery
T-1821 · 2026-11-07 · derived algebra and unsigned producer evidence

The adaptive recurrence keeps its exact certificate C(B)=0 modulo G, where B=X^m mod G. This successor shares a short polynomial-power table between truncated trace projection and certificate evaluation. The table contains polynomial residues, not candidate scalar labels or group elements.

#image("flow.svg", width: 100%)

*Projection.* At prefix length L choose q=ceil(sqrt L). Convolutions produce the trace functional for each giant power; scalar pairings with the short powers produce only the requested moments. Initial trace weights through index 2n-2 are shared across all prefixes.

*Certificate.* Split C into coefficient blocks of length q. Form each block value from the same short powers and evaluate the blocks by Horner in H=B^q. This computes exactly C(B) modulo G, including polynomials above the modulus degree and incomplete terminal blocks.

*Derived work.* Geometric prefixes ending below 4t give O(sqrt(t) M(n)+nt) total projection and certificate work. Including initial powering and scalar recurrence synthesis gives O(M(n)(log m+sqrt(t))+nt+t²+t log r). Validation, envelope output and parent checks remain additional. Stored arrays use O(n sqrt(t)+n+t), plus backend temporaries. The table cap is explicit; no total-memory or timing claim is made.

*Exact checks.* 1,272 projected prefixes agree with direct root power sums. 5,088 complete certificate residues agree with coefficientwise Horner, at degrees 0,1,n,2n+1. Each prefix passes its exact table-word allowance and rejects one word less. The 6,522 small adaptive cases and three larger cases preserve image polynomials, envelopes, accepted moment vectors and rejected-prefix counts. All 63 public recovery routes and query fields agree, with 511 source calls. Original 4,563-case cohort unchanged.

*Failure retained.* validation-01 exposed an out-of-bounds read after convolution coefficients were trimmed. Its source and panic are preserved. Missing high coefficients now contribute zero; validation-02 passes. Compiler warnings, commands and input hashes remain in the receipts. Producer evidence has no independent signed verdict or complete formalization.

*Sources and next step.* Official FLINT modular-composition documentation, linked in DERIVATION.md; Brent-Kung PDF request timed out; KB unavailable. Polynomial blocking is established, and no fast matrix multiplication is implemented. Next choose among sequential, blocked and lcm construction using full operation accounting and finite workspace; source evaluation remains separately charged. No cloud experiments launched.
