# Prime31 exact signed kSUM table frontier — a charged control, not a new index

Date 2026-11-23. Draft PR #5064. Source checkpoint `main da7777094392d3ccd37db78dcfc761c32e7e7813`; previous merged #4941/#4951/#4977/#4979/#5048. **PCC unverified. No native Rust test or performance measurement.**

## Selection and prior art

This note chooses one unclaimed *math/validation-only* gate: how expensive is an exact hash-lookup shortcut if it explicitly precomputes 2-, 3-, or 4-leaf point sums? It is the classical meet-in-the-middle/kSUM indexing baseline, **not a proposed novel S3 elimination or a proof against implicit indexing**. Read `iteration8/EXACT_CONTRACTION_AND_YIELD.md` (blob `20f9428434bf228c75de404bdb8c4fde4f7b3675`), `iteration9/PAIR_SPECTRUM_RESULTANT.md` (blob `e2348e91dc08141a663c5adb9798b6ca8f09424e`), `iteration10/parallel_review/REVIEW.md` (blob `0e295d7d5e6ef1d84f59c1889ae81d6cbcaa7fb9`) and `iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md` (blob `953b48e251bdbd6fce9b7748cd4b05c5d60afb7c`).

Primary literature: Semaev 2015, https://arxiv.org/abs/1504.01175 (rational auxiliary summation chains); Dinur–Golovnev, *Improved Time-Space Tradeoffs for 3SUM-Indexing*, ICALP 2026, https://drops.dagstuhl.de/storage/00lipics/lipics-vol374-icalp2026/html/LIPIcs.ICALP.2026.78/LIPIcs.ICALP.2026.78.html (generic integer kSUM indexing, different resource model). Their theorem does **not** furnish a free lossless additive integer hash or a free elliptic-to-small-domain map for this prime-order elliptic source. The shared KB MCP is not loaded, and its repository wrapper cannot run in the supplied environment, which has no source checkout. This is a documented prior-art limitation, not a novelty claim.

## Exact finite point-table semantics

Let G be the supplied **odd-prime-order** elliptic subgroup of order n, and A={+P_i,-P_i:1<=i<=B}, with B distinct abscissae, all rational lifts and nonzero ordinates. Let L=2B. O is not a leaf but is a valid intermediate sum. For k in {2,3,4}, precompute every sorted k-multiset of *signed leaf labels*. Store its exact group sum and all associated labels; retaining just one witness would be a decision-only structure. Number of canonical label multisets is

    M_k = binom(L+k-1,k).

The full point hash is exact and does not need a DLP. For a nonzero target R and k<4, enumerate all (4-k)-leaf signed multisets V and look up R-sum(V) in the k-table. Exact signed group replay and final canonicalization restore every four-leaf multiset. For k=4, directly look up R; every requested witness is retained under the full point key. Reconstruct valid ordered affine S3 chains only after replay, with nonzero-prefix checks and all permissible reorderings. Hash collision keys must be compared as full point encodings; if x alone is used, verify ±R and flip **all** signs when required. Handle O and zero subtotals separately. A dictionary miss after the entire enumeration is an exact relative-to-A miss.

With all leaf witnesses and exact hashing, source setup, query time on a miss, and storage (up to logs and curve costs) are:

| k precomputed | precompute tuples and memory | empty query probes | complete output |
| --- | --- | --- | --- |
| 2 | Theta(B^2) | Theta(B^2) | +O(h) signed replay |
| 3 | Theta(B^3) | Theta(B) | +O(h) signed replay |
| 4 | Theta(B^4) | expected O(1) full-point lookup | +O(h) signed replay |

Here h counts the emitted signed witnesses (including required orderings if requested), not just unique target abscissae. Sorting or deterministic trees add log factors; hashing has an explicit expected-time collision/rehash model. Source point-addition costs, construction of the verified factor base, proof transport, and empty targets must all be priced. **This is a time-space tradeoff:** a fast query was purchased by target-independent enumerative construction; it is not near-linear-in-D setup when D=Theta(B^2) and k>2.

## A restricted full-solver exponent gate

Assume fixed four-summand arity, uniform nonidentity targets in a prime-order group of size n asymptotic to p, **at most one useful retained independent relation per query**, a rank target of Theta(B) rows, and an explicit k-sum dictionary built by enumerating its Theta(B^k) tuples. No uncharged DLP or FFT labels. The elementary support bound gives

    Pr(target has four signed leaves) <= min(1, (2B)^4/n).

Under this restricted single-row protocol, at least Omega(max(B,n/B^3)) target queries are needed in expectation, where the n/B^3 term uses only that upper bound and the row requirement. A fixed-complement enumeration costs Theta(B^(4-k)) for an empty query, so a scope-specific work lower bound in candidate/lookup operations is

    W_k(B) = Omega(B^k + B^(4-k) max(B,n/B^3) + B^2).

This is **not** a lower bound for all exact algorithms: source-specific algebraic contraction, multiple independent rows per target, adaptive sampling, different rank architectures, fast bulk operations or non-enumerative indexes lie outside it. It also does not convert unlike field operations and group additions into equal wall-clock costs.

Minimizing the displayed exponent for B=n^beta and charging the saturated-query regime beta>=1/4 gives:

| k | optimal beta | necessary exponent under the protocol |
| --- | --- | --- |
| 2 | 1/4 | at least 3/4 (not 2/3: the unsaturated optimum beta=1/3 violates B^4 << n) |
| 3 | 1/5 | at least 3/5 |
| 4 | 1/7 | at least 4/7 |

For k=3,4, beta<1/4, so the sparse-support assumption is consistent at the balance point. The beta values fit the *selected-field* beta<5/12 availability window in the earlier conditional fibre argument, but this **does not establish a constructible prime-order curve family, actual useful rank, or new speedup**. A finite fixed-k dictionary alone does not cross exponent 1/2. A generic formula k/(2k-1) must not be applied when 1/(2k-1)>=1/4.

The stronger desideratum remains an *implicit*, source-specific exact index with near-linear-in-D setup and o(D) empty query work, ideally D^theta with theta<1/2 for the conditional pipeline. This note closes only the full enumerative lookup shortcut.

## Concrete inherited Prime31 control (not a performance result)

The inherited fixture has p=2147483647, prime subgroup n=2147446831 and B=31. Counts of canonical unordered signed tuples, allowing repeats and opposite signs, are:

    M_2=binom(63,2)=1953
    M_3=binom(64,3)=41664
    M_4=binom(65,4)=677040.

The merged cancellation-reduced **upper** bound of nonidentity four-sum abscissae is S<=309442. Thus for a uniformly sampled nonidentity subgroup target,

    Pr(nonempty) <= 309442/1073723415 ≈ 0.00028819526116
    Pr(empty) >= 0.99971180473884
    E[queries until first hit] >= 3469.87.

This is a bound, **not a census or measured success probability**. If the protocol retains at most one row per target and requires 31 useful successes, its expected query count is at least 107565.96; meaningful rank can only be harder. Do **not** combine the 42-row *planted/distinct-index* coupon-collector threshold from #5048 with this natural-target bound without an explicit distribution coupling. Similarly a positive planted trial cannot stand in for the missed-query workload.

At fixed B the 3-leaf dictionary has at most 41664 canonical signed label triples, so a 62-complement-probe natural target check is a concrete finite comparator to the pair-spectrum B^2 scan. It may improve a repeated-query **component** in practice but consumes extra setup and storage. Run no performance or cold solver comparison until source-freezing, reservation, matched-resource pricing and the current original-root validity gates permit it. The 4-leaf table's 677040 tuple records similarly give constant expected membership probes at a much higher setup cost; there is no claimed measured advantage or new asymptotic algorithm.

## Validation plan, separated from producer arithmetic

On an eligible small odd-prime-order curve, freeze the original point source and test the 2/3/4-sum group dictionaries against a separate exhaustive four-leaf oracle for **every** group target. Verify the full signed multiset set (not merely positive support), repeated labels, opposite/cancelled leaves, O subtotals, R=O, 2-torsion exclusions, key collisions, global sign flip, duplicate canonicalization and nonzero-prefix affine S3 reconstruction. For every recovered public relation independently replay the group sum and exact signed coefficient vector; compare useful rank over the actual subgroup field including RHS/target columns. Charge emitted witnesses, table construction, memory, proof bytes, all empty queries and rebuilds. The pair-spectrum source-signed baseline and the best preserved original IC frontier remain the controls.

Coordinator-only arithmetic review: the combinatorial M_k values were checked by multiplicative binomial formula, and the three exponent minima were checked independently by a 100000-point beta grid (k=2 -> about 0.7500005; k=3 -> 0.6000029; k=4 -> 0.5714319). This is no native agent, Rust run, independent signoff or PCC proof. No Rust worker was assigned implementation because the selected enumerative design does not pass the sought asymptotic admissibility gate and the live interactive Rust/workflow/state/report reservation remains binding.

### Next mathematical gate

Find an exact source-specific contraction exploiting the actual M(Z)-leaf algebra / signed swap quotient that prunes **misses** with comparable O-tilde(D) source setup; specify a full costed zero oracle and certificate including all target exceptions. Do not merely rename a degree-D resultant, a full 4SUM table or a 3SUM-indexing integer hash. [pcc:unverified]
