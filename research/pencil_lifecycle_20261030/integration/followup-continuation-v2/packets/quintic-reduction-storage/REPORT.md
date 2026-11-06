# Quintic normalization and final divisor storage

Producer proof packet, 2026-11-01. Independent source-fidelity review pending.

Result. Literal word-list monic normalization preserves canonical normalized storage and length, produces leading coefficient one and decodes to a nonzero scalar multiple. Canonical normalized nonempty word lists have polynomial degree equal to length minus one. Normalization preserves the residual divisibility and polynomial degree.

Quintic bound. Over a field, if degree(f)=5, degree(u) is three or four, degree(v)<degree(u), and f-v*v=q*u, then q is nonzero and degree(q) is at most two. The nonzero residual follows because a nonzero polynomial square has even degree. For degree(u)=3 the residual has degree five. For degree(u)=4 the residual degree is at most six. The product-degree identity gives the quotient bound in each case.

Storage endpoint. For a canonical normalized stored quotient q and a supplied remainder r shorter than monic(q), the quintic bound implies that monic(q) is nonempty and has at most three coefficients, and r has at most two. With the explicitly supplied residual divisibility, the decoded pair satisfies the formal Mumford-validity predicate. This establishes the final fixed-array size preconditions for the modeled pair. The theorem does not construct the reduction remainder or claim a complete execution of the native loop.

Checks. Six modules freshly compiled; ten main declarations, four fixed controls and fourteen exact axiom readbacks. The native checker accepts only the listed names and standard axioms propext, Classical.choice and Quot.sound. A deliberately false theorem exits one. Three checker controls reject a forbidden axiom, empty output and a wrong theorem name. Every successful fresh module check records unchanged bound inputs. Imported dependency caches are reused; independent binary/kernel rechecking is not included.

Assumptions. Word-level results use prime modulus 1 < p < 2^31 and canonical normalized input storage. The field-level quotient theorem requires degree(f)=5 and the displayed exact quotient identity. The storage theorem separately assumes the short remainder and residual divisibility. The native curve constructor checked here requires polynomial length six; the degree-five condition is not inferred for every possible public Jacobian value.

Controls and failures. Fixed controls check empty normalization, the already-monic branch, a vanishing even-degree residual and a trailing-zero list which fits storage but is not normalized. Early producer attempts needed explicit unfolding of Normal and the empty list polynomial, and explicit branch rewrites in normalization. Those failed attempts remain archived. The final fresh replay passed. No trial, curve enumeration, timing measurement or scientific cohort was launched.

Source and prior work. Native src/cryptanalysis/three_leads.rs at fe447429798e2936a86bda36e83fe4bc960ebab3: pmonic, reduce, Divisor::from and the quintic constructor. Prior fallback-geometric-invariant and pegcd-six-word-endpoint packets supply the congruence and exact-word arithmetic context. Pinned Mathlib polynomial degree lemmas in Degree/Defs, Operations and Domain supply standard product and subtraction degree identities. This is a source-correspondence construction, with no novelty claim.

Next joint. Compose the actual remainder generation and normalization calls with these storage lemmas, and establish the composed input degree bound from the two degree-at-most-two input divisors. Divisor-class interpretation, compiled source fidelity and independent review remain separate obligations. Campaign scope and the original 1,260 targets are unchanged; fresh costs are unknown.
