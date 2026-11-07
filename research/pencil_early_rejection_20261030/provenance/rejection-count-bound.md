# Static bound on root-count rejections

This is a source-bound mathematical argument, not an execution result or a speed claim.

Source: research/pencil_large_prime_20261029/src/native_solver.rs, SHA256 e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df, in /home/ec2-user/crypto-pencil-signed-graph-20261029.

Assume an odd-characteristic field, a smooth quintic y²=h(x), monic degree-two u, degree(v)<2, and u dividing v²-h. Field operations and source bucket construction are assumed faithful; the existing bounded-arithmetic qualification remains necessary for Rust equivalence.

Define q_lambda=((v+lambda*u)²-h)/u=q_0+2lambda*v+lambda²u. Each q_lambda is a nonzero cubic with leading coefficient -h_5. The coefficient is -1 only when h is monic; nonzero h_5 suffices.

Every ordinary bucket entry has u(x) nonzero and lambda=(y-v(x))/u(x), hence q_lambda(x)=0. At fixed x and lambda, y is unique. The rational-y table enumerates each y once, including y=0 once. Thus ordinary entries in a bucket are distinct x roots; the source also checks distinctness for ordinary collision candidates.

Exceptional entries have u(x)=0, so they cannot duplicate ordinary entries. The x scan visits each support root once, including repeated roots of u. The all-lambda case would require u(x)=v(x)=q_0(x)=0 (odd characteristic makes slope 2v=0 equivalent to v=0). Evaluating v²-h=u*q_0 and its derivative then forces h(x)=h'(x)=0, contradicting smoothness. Therefore that case is absent under the hypotheses.

At each of the at most two distinct roots of u, the remaining exceptional branch has nonzero slope 2v(x). The equation q_lambda(x)=q_0(x)+2lambda*v(x)=0 supplies exactly one lambda. Candidate-set deduplication can only reduce the number of exceptional lambdas, so there are at most two.

An ordinary collision candidate starts with at least three distinct roots of its nonzero cubic, hence exactly three. Any subsequently appended exceptional point would be a distinct fourth root, impossible. Its final bucket therefore has exactly three entries and passes the root-count guard. Every rejected candidate must consequently belong to the exceptional candidate set.

Conclusion: at most two rejected candidate-verification calls per sieve, and at most 32 across the caller's at-most-16-walk loop. Non-degree-two walks are skipped before the sieve. This bounds the number of complement calls the proposed reorder can avoid. It does not bound their fraction of total work or establish any timing or complete-cost improvement.

Reviewed source sites: complement at804; point_sieve_candidates at850, lambda-zero validation858, exceptional classification862 onward and finalized bucket additions near970; caller's16-walk loop and degree guard near1311; candidate guard near1330. Source line numbers are subordinate to the pinned file hash.

No patch application, compilation, tests, solver execution, reanalysis or cloud calls were performed for this argument.
