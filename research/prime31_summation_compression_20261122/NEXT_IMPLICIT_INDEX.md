# Fourth iteration — exact emptiness representations and the two-summand rank barrier

Checkpoint: 2026-11-22 (America/Los_Angeles). [pcc:unverified] This is a written mathematical analysis, not a new Rust execution, a signed PCC proof, an admitted timing experiment, or a claim of novelty. It follows [NEXT_ROOT_INTERSECTION.md](NEXT_ROOT_INTERSECTION.md). No complete ECDLP or asymptotic relation-solving improvement has been established.

## Fixed setting

Use the same ordinary Prime31 curve and signed-liftable Dickson fibre as REPORT.md: p=2147483647, group order n=2147446831 (inherited source record, not independently recounted), fully split squarefree F(X)=D_N(X)-c with N=2^k<p, and B distinct liftable x-coordinates W. Let R be a nonzero point with s != 0; S3(X,Y,x(R)) is as in REPORT.md. An exact relation is a signed identity P+Q=R on the *original* curve, with x(P),x(Q) in W. The number of projected x solutions z is not the same as the number of signed ordered pairs.

The previous note supplied an output-sensitive binary-tree **number of tests**, not the cost of an emptiness test. The experiments had already rejected dense norm and explicit dimension-charged block descent. We now examine a different exact algebraic zero-test, show where it fails, and pivot to an exact *group-sum index* that does achieve fast **online** emptiness at a charged, prohibitive preprocessing cost.

## A. Rejected auxiliary representation: Fermat idempotent in the split fibre algebra

For a tree node (j,a), put M=2^j and F_(j,a)(X)=D_M(X)-a. Assume all its M roots belong to the split squarefree full fibre. Define the etale algebra

    A_(j,a) = F_p[X] / (F_(j,a)(X)) ~= F_p^M.

Let G_R(X)=Res_Y(F_k(Y),S3(X,Y,x(R))) and g=G_R mod F_(j,a). Define the *Fermat zero projector* e=1-g^(p-1) in A_(j,a).

**Exact lemma.** e^2=e, and for the algebra trace of multiplication,

    z_(j,a) = M - Tr_A(g^(p-1)) = Tr_A(e) in F_p.

Since 0<=z_(j,a)<=M<p, the canonical field representative is the *integer* count of roots x in that node with G_R(x)=0. Hence E(j,a) is true if and only if this integer is positive. There are no probabilistic false negatives, provided splitness, squarefreeness and M<p are checked. Proof: Chinese remaindering identifies A with a product of M copies of F_p; at each root, a^(p-1) equals 0 for a=0 and 1 otherwise. The trace is the sum of the diagonal evaluations. Non-split roots or inseparable fibres are not covered by this formula.

**Complete explicit-vector recurrence.** Compute q=dY^2-tY+u with d,t,u as in REPORT.md, *inside A*. Set (Q_0,T_0,U_0)=(d,t,u) mod F_(j,a). For ell=0,...,k-1:

    Q_(ell+1) = Q_ell^2
    T_(ell+1) = T_ell^2 - 2 U_ell Q_ell - 4 Q_ell^2
    U_(ell+1) = U_ell^2 - 2 T_ell^2 + 4 U_ell Q_ell + 4 Q_ell^2

with each right side reduced modulo F_(j,a). Then g=U_k-c T_k+c^2 Q_k. Compute u_0=1,v_0=g and, for binary digits b_i of p-1,

    u_(i+1) = u_i * (b_i ? v_i : 1) mod F_(j,a)
    v_(i+1) = v_i^2 mod F_(j,a).

The final u is g^(p-1). Precompute power sums s_t=sum_(F_(j,a)(alpha)=0) alpha^t for 0<=t<M using Newton identities from F_(j,a), or any exact equivalent. If u=sum_(t=0)^(M-1) u_t X^t, then Tr_A(u)=sum_t u_t s_t. This is the complete zero decision, not just an equation-circuit construction.

With dense normal-form arrays and cost C(M) for one modular product, this specific implementation uses O((k+log p) C(M)+M) target arithmetic after the node modulus and trace-weight setup, which costs at most O(j C(M)+M^2) under straightforward squaring/Newton-sum algorithms. Its live state is O(M) coefficient slots plus multiplication scratch and output, with O(M) coefficient writes to instantiate dense state. For the empty root node M=N the setup/target state is already Omega(N); visiting many nodes is worse. Reusing trace weights across targets does not eliminate the target-dependent g and idempotent. This representation therefore **fails** the requested o(N) cold empty-root criterion. This is a bound on an explicitly materialized quotient-state algorithm, *not* on every possible symbolic compression of the projector.

## B. Different auxiliary representation: exact signed group-sum index

Instead of representing G_R, let A={P_x,-P_x:x in W}, where P_x is one rational point over x. In the present odd prime-order source group these are 2B distinct signed points, so L=|A|=2B. The point signs are part of the relation and every entry is in E(F_p). Use a collision-checked dictionary keyed by a *complete encoded group point*, not by x alone.

Define the full tuple index I_1={(P,(P)):P in A}; recursively,

    I_(h+1) = { (S+P, (P_1,...,P_h,P)):
                 (S,(P_1,...,P_h)) in I_h, P in A }.

Store all entries by their first coordinate S (an elliptic-curve point, including O). For an exact public target R the dictionary bucket I_m[R] contains **all** ordered signed m-summand decompositions, including repeated indices and exceptional identities. Nothing is inferred from merely sharing an x-coordinate. Hash collisions are settled by exact point equality; alternatively use deterministic sorted point keys. Every output is rechecked by original-curve addition before being treated as a relation.

An exact online empty-root test for m=2 is simply “is I_2[R] empty?”. To answer *every* Dickson-subtree predicate for that target, take returned tuples, project their first x coordinates, and insert their sequences D_(2^j)(x) into a target-local trie (or deduplicated level map). E(j,a) then tests existence of the node label in the trie. This is logically equivalent to the S3/full-fibre predicate under the earlier rational lifting theorem and its R-domain exceptions. It needs no G_R coefficients, no dense quotient vectors and no traversal of an empty fibre. The trie is only for a queried target; it cannot be credited as free universal precomputation.

Let r_R=|I_m[R]| count *ordered signed tuples*, distinct from z_R projected first coordinates. Complete cost accounting for this particular representation:

- Fibre generation, liftability, point signs and original-curve checks: C_base(N,B,p) >= Omega(B), and reusable only after charging.
- Tuple enumeration: exactly L^h records at level h, requiring sum_(h=2)^m L^h original-curve additions, plus O(m L^m) tuple bookkeeping and encodings for fixed m; hash insertion is expected O(L^m), a deterministic balanced index has O(L^m log(L^m)) comparison overhead.
- Stored index: Theta(m L^m) point/identifier words with full tuple storage; peak memory and temporary sorting buffers must be charged. A different compressed representation needs its own proof.
- Per-target exact lookup: expected O(1+r_R) indexed accesses with collision-checked hashing, or O(log(L^m)+r_R) comparisons via ordered keys; complete signed row verification O(m r_R) group additions. Optional trie build costs O(k r_R) Dickson evaluations and O(k r_R) words. No-root queries have O(1) expected online lookup but **not** zero preprocessing cost.
- For T uniform independently generated group targets, *exactly* E[sum_R r_R]=T L^m/n, by linearity of expectation: each of L^m fixed tuple sums equals a uniform target with probability 1/n. This expectation does not assert independence among tuple sums, full relation rank, or low memory.
- Total work over T targets is Omega(L^m+T+sum_R r_R) for the full table, before linear algebra; group-operation and bit costs are additional. Per-query amortized setup is Omega(L^m/T), so the advertised constant-time emptiness is a **batch-indexing tradeoff**, not a cold gain.

In the Prime31 depth-six component cell, B=31 gives L=62 and 3,844 ordered signed pair entries for I_2, before deduplication or index overhead. This is an exact size calculation, **not** an executed benchmark. A PR #4829/group-complement scan is still a mandatory incumbent. Full pair-sum indexing is classical meet-in-the-middle preprocessing, not a claimed novel ECDLP method.

## C. Conditional rank/work barrier: why a perfect two-summand online oracle is insufficient

Assume the conventional full-rank index-calculus architecture with B unknown factor-base logs, requiring at least B independent relation rows; assume each attempt uses a fresh uniform random R in a source group of size n; assume at most L^m signed ordered tuples can be reported for a target, and count all attempts and factor-base setup. (One-dependency methods, correlated target selection, reused challenge-specific knowledge and more structured relation constructions fall outside this model.)

The expected number of **candidate tuple rows** from T attempts is exactly T L^m/n. Requiring expected candidate output at least B (already weaker than independent rank) implies

    T >= B n / L^m = n / (2^m B^(m-1))   when L=2B.

For m=2 even an ideal O(1) exact query, with only Omega(B) factor-base setup and Omega(T) target handling, costs

    Omega(B+n/B) = Omega(sqrt(n))

in the stated model. This is a **conditional full-rank collection barrier**, not a black-box ECDLP lower bound. Dependencies, repeated tuples and rank loss only make the necessary collection harder. It explains why accelerating only the two-summand emptiness predicate cannot, under these assumptions, yield a strictly better cold exponent than optimized Pollard rho.

For the *explicit complete m-tuple index*, preprocessing alone is Omega(B^m). Combining with the necessary attempts yields

    Omega(B^m + n/B^(m-1))
      >= Omega(n^(m/(2m-1)))   for fixed m>=2,

with the minimum near B=n^(1/(2m-1)). The exponents are 2/3 for m=2, 3/5 for m=3, 4/7 for m=4, all **above** 1/2. This conclusion is confined to fully materialized m-sum dictionaries, uniform target attempts and a B-row rank objective. It does not exclude a genuinely subquadratic *implicit* m-sum index, higher-arity Semaev chains with cheap online extraction, or one-dependency index calculus. It also excludes all linear-algebra and sign-validation overhead from the displayed optimistic lower bounds; adding them cannot improve the bounds.

## D. What to pursue next, without recycling the failed mechanism

The most useful successor question is now an **implicit signed m-sum index** or higher-arity S3 chain for m>=3, with a proved preprocessing/query/storage tradeoff and exact signed extraction, not merely a small polynomial or conditional branch count.

For a claimed growing prime-field family define:

    P_m(B,p) = charged setup and index work,
    Q_m(B,p) = expected exact cost per fresh natural target including misses,
    W_m(B,p) = verified output and group work,
    L_m(B,p) = independent rank and sparse linear algebra.

The optimistic full-rank collection model gives the *necessary* scale T=Omega(n/B^(m-1)), so the total is P_m+T Q_m+W_m+L_m, with duplicate rows and rank defects explicitly measured. A candidate must specify how it outperforms both O(B) group-complement scanning and maintained quotient/GCD/product-tree controls. One-dependency methods require a separate end-to-end accounting; the B-row assumption must not be imported into that architecture.

For k increasing, one can define primes p_k congruent to -1 mod 2^k (Dirichlet guarantees an individual prime for each k), but **no bound here** proves 2^k >= p_k^delta for any fixed delta>0, or supplies ordinary prime-order curves with sufficiently many liftable fibre points. Those are explicit *additional* asymptotic family obligations, not facts inferred from the fixed Mersenne prime 2^31-1. No assumption of infinitely many Mersenne primes is used.

## Provenance, controls and standing

- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004): https://eprint.iacr.org/2004/031
- Faugere–Huot–Joux–Renault–Vitse, *Symmetrized Summation Polynomials*, EUROCRYPT 2014, DOI 10.1007/978-3-642-55220-5_3: existing symmetry prior art.
- Repository: research/prime31_ic_20261115/native/examples/prime31_s3_quotient_gcd.rs, PR #4829 group-complement/point-origin roots, PR #4924 monic symmetric pair work, PRs #4692/#4812/#4822/#4901 active solver/rank controls. The shared research-KB interface and wrapper were not available; the repository and public primary sources were checked directly. This is not an exhaustive novelty audit.
- Companion **new source-bound CI receipt found during this iteration**: [run 45089472283](https://github.com/isogeny-labs/crypto/actions/runs/45089472283), job 137415511884, checked out 96c9101315d9f0c7ca5fee2af18cdb10d16c645d and passed its original-Rust tests, including 2 monic-oracle tests. The monic oracle blob ec949fa765aee1686ba01a74f420f8f2f0a77df3 at that commit is identical to PR #4924 current head 70e04851fae933675dbc4476f22dcea25090551e; the intervening commit changes only B677_MONIC_SCALING_PROTOCOL_20261123.md. This **supersedes the stale-source concern for those exact monic bytes**, but does not imply signed independent validation, a complete solver result or an admitted benchmark. The log prints B=677 online 28.9–32.0 ms and reference scan 4.8–5.5 ms in the inspected cases, not a speedup over that printed control; protocol/admission were not established for these timings. [pcc:unverified]
- All mathematical statements above are written derivations only, [pcc:unverified]. No new Rust run, natural-target corpus, EKS job, Lean proof, signed PCC certificate, or performance leaderboard entry was produced in this iteration.

**Next obligation:** derive and source-audit a non-enumerative, exact higher-arity/implicit sumset representation with a full P/Q/memory recurrence that is sublinear per cold emptiness query after setup; otherwise retain the explicit barriers. Then implement the already preregistered byte-exact natural-target generator before any new matched performance run, obtain original-source correctness CI, repository checkpoint and EKS admission, and measure genuine signed rank and complete recovery.
