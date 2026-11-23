# Prime31 S9: orientation fibres and exceptional repeated-root controls

**Protocol frozen 2026-11-16 before the accompanying Rust implementation or CI.** Status: mathematical derivation plus prospective Rust implementation, not a measured ECDLP speedup or a non-generic root extractor. [pcc:unverified]

## Prior evidence and avoidance of duplicated work

Read repo `AGENTS.md`, newest PR #4391 session log, `M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`, `S9_B31_MULTIPLICITY_PROTOCOL_20261116.md`, and earlier B4/B8 and Dickson31 signed-support census. See also crypto #4228/#4349 and published prime-field index-calculus work: Petit--Kosters--Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1) and Amadori--Pintore--Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009, https://ora.ox.ac.uk/objects/uuid:c99fdb39-f5b1-428d-999c-770b63363e25). Their methods predate this work. Research-KB MCP/wrapper and Conductor are unavailable in this connector-only runtime; GitHub original sources and publisher repositories are fallback. These formulae are not asserted novel.

The existing `prime31_s9_b31_signature_multiplicity.rs` separately examines **different net vectors mapping to the same target point** by a complete generic 4+4 table. This new experiment instead examines the orthogonal algebraic question: for a **fixed net vector**, how many distinct degree-eight *x-multiset polynomials* and oriented signed words represent it? No 4+4 table or complete logarithm solver is built here.

## Exact algebraic obligations (over any odd-order elliptic-curve group)

Fix B distinct rational x values and a signed lift P_i for each. An eight-token signed multiset is specified by nonnegative counts a_i,b_i of +P_i,-P_i with sum_i(a_i+b_i)=8. Define d_i=a_i-b_i and m_i=a_i+b_i. The x-only candidate polynomial is f(T)=prod_i(T-x_i)^{m_i}; the net point is sum_i d_i P_i.

- Every d satisfies ||d||_1=k in {0,2,4,6,8}. Conversely every such d is realizable by exactly C(B+t-1,t) distinct x-only f, where t=(8-k)/2: set m_i=|d_i|+2c_i for any weak composition c_1+...+c_B=t.
- Every fixed x-only f with multiplicities (m_i) admits exactly prod_i(m_i+1) distinct orientation *net vectors* d (select a_i in 0..m_i). This counts sign vectors, **not** necessarily distinct group points.
- Double counting gives sum_{||d||_1<=8,even} C(B+(8-||d||_1)/2-1,(8-||d||_1)/2) = C(2B+7,8), and sum_{sum m_i=8} prod_i(m_i+1)=C(2B+7,8).
- There are C(B+7,8) distinct degree-eight x-multiset polynomials. The net-signature count is [even coefficients up to degree eight] of ((1+z)/(1-z))^B. The three counts measure DIFFERENT spaces. A given polynomial can match different target Q after sign changes; S9 in x_Q alone also cannot distinguish Q from -Q. The full (x_Q,y_Q) orientation replay is mandatory.
- For Q=O, cancellation orientations (d=0) give C(B+3,4) x-only polynomials, not one. Infinity and same-x intermediate cases must not be silently dropped by an affine denominator saturation.

## Frozen executable controls

Reuse only `prime31_algebraic_ic::{Arithmetic, Base, check_curve, PRIME31}` and Rust 1.100.0. Check exact combinatorial counts for B4, B8, B31 and BOTH double-count identities. On the actual Dickson-depth-six 31-point fibre, take the **first eight distinct selected points**, freeze the same degree-eight squarefree x polynomial, compute the group sum for **all 256 sign masks** independently in existing Rust group arithmetic, and report group-point collisions without assuming none. Check that flipping the first sign changes Q and that all masks correspond to the identical f. Separately take f=(T-x_0)^8 and enumerate the nine net choices d_0=-8,-6,...,8, reporting the distinct target count, the identity at d=0 and original-group-point checks.

No hidden ECDLP scalar enters any test. Tests MUST assert exact B31 counts, cross-count equality, two distinct Q from a single f, repeated-root identity behavior and true prime31 point validity. An actual completed CI test/run may support only an orientation/exception-branch diagnostic. Failures/queue/caps remain labeled. Separate extractor goal: target-conditioned f-from-Q without enumerating x-multisets or generic point-sum tables; certify its original ideal, orientation, repeated-root and infinity solutions on B4/B8 and Dickson31 controls, before rank32 unknown-log recovery and strongest matched rho/external PKM-APS-CAS timing.

**Previously frozen full-cold reference, not rerun here:** 48 Q APS 109.734280 ms versus unfolded rho64 99.473873 ms, unisolated producer results [pcc:unverified]. An orientation check is not a competing algorithm or a speedup.
