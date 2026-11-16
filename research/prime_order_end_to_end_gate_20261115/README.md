# Complete prime-field ECDLP controls: smooth-order positive, prime-order negative

**Date:** 2026-11-15 (UTC) · **Standing:** [pcc:unverified] producer-local exact arithmetic and reproducibility. This directory does **not** establish a faster solver for general prime-order ECDLP or NIST curves. It adds two different, scoped, end-to-end tests to PR #4188.

## 1. Restricted positive result: known smooth-order prime-field ECDLP

On the *prime fields* `F_p`, with `p=2^k-1` prime and `k=31,61`, choose the fixed supersingular curve

    E: y^2 = x^3+x.

Because `p = 3 mod 4`, the quadratic character satisfies `chi(-1)=-1`. Since `f(-x)=-f(x)`, all nonzero character terms pair and cancel, yielding

    #E(F_p) = p+1 = 2^k.

Moreover `x^3+x=x(x^2+1)` has only `x=0` as a root over `F_p`. Thus E(F_p) has a single nonidentity 2-torsion point, `(0,0)`. A finite abelian 2-group with a unique involution is cyclic. The script checks `p` by Lucas–Lehmer and finds an explicit generator `G` with `[2^(k-1)]G != O`, `[2^k]G=O`.

**Complete algorithm (known Pohlig–Hellman, not novel):** let `Q=[d]G` with unknown `d`. After digits `d_0,...,d_(i-1)` are recovered, put `R_i=Q-[sum_(j<i) d_j 2^j]G`. Then

    [2^(k-1-i)] R_i = d_i [2^(k-1)]G,

which is either O or the unique order-two point and therefore yields `d_i` exactly. The invariant proves correct recovery for **every** group point, using `O(k^2)` operations and constant memory for this implementation. It is a complete scalar-blind ECDLP solver in this deliberately weak, smooth-order curve family.

Local counted operations (nontrivial affine point additions/doublings; both methods share the same point primitive, include per-target verification, same public group order; excludes curve/generator selection):

| Family | PH target cases | Mean PH point ops | Mean BSGS point ops | BSGS/PH |
|---|---:|---:|---:|---:|
| `p=2^31-1`, `#E=2^31` | 16/16 | 406.9375 | 57289.375 | **140.78** |
| `p=2^61-1`, `#E=2^61` | 16/16 | 1372.5625 | not run (memory) | not measured |

**Limits:** PH is a textbook smooth-order algorithm, not a discovery. The 31-bit positive control is cheaper than **our matched BSGS implementation by counted point operations**, not a measured optimized rho GPU benchmark. It does not help with a prime-order subgroup of cryptographic size. The two algorithms receive the same explicitly known curve, subgroup order, and generator. The 61-bit general completeness statement is mathematical; the run samples 16 points rather than enumerating 2^61 points. No claims about modern NIST security follow.

## 2. Prime-order, end-to-end signed-relation control

The *prime-order* curve

    E/F_1019: y^2=x^3+46x+158,  #E=977 (prime)
    G=(944,936)

is exhaustively checked by character counting, and every nonidentity point is in the order-977 subgroup. The fixed eight-point factor base excludes G. A deterministic Gray walk constructs the 256 sign-branch sums (250 distinct points for the chosen base).

The **public-input-only** solver receives G and Q (not the target scalar), and:

1. Scans the sequence `aG` for hits in the precomputed signed table, recording each exact equation `sum_i epsilon_i log_G(P_i)=a mod 977`.
2. Keeps only independent equations until the coefficient matrix has full **rank 8**. Recovers all eight *previously unknown* basis logarithms by elimination and verifies `[log_G(P_i)]G=P_i`.
3. Scans `Q+aG` until a signed witness is found. It recovers `log_G(Q)=sum_i epsilon_i log_G(P_i)-a`, followed by direct point verification.

This is **not sparse PKM m=8**: it enumerates all 256 sign sums and then scans group shifts. It is the needed complete black-box relation economics control, not a successful algebraic relation oracle.

Local fully cold per-target test: **977/977** distinct scalars recovered and replayed in each solver, including O. Relation stage had 12 exact signed rows, rank increment to **8** after 45 candidate shifts, and 250 distinct table entries. The same curve and scalar-multiplication primitive were used for BSGS and elementary three-way Floyd rho.

| Prime-order cold method | Mean nontrivial point ops/solve | Scope |
|---|---:|---|
| BSGS | **64.609** | complete, 977/977 |
| Three-way Floyd rho | **147.382** | complete, 977/977; not negation-map optimized |
| Eight-sign relation table + linear algebra | **430.874** | complete, 977/977; field elimination uncharged |

The eight-sign route costs **6.67×** the BSGS baseline and **2.92×** this basic rho. Matrix row reduction/inversions are **not** priced, so the signed pipeline is already favored by this comparison. These producer-local results do not establish a scaling exponent; the BSGS control is favorable at this tiny size, and the rho implementation is *not* the strongest external reference. The full table and fixed-shift structure still fall under the restricted support/coverage lower bound recorded in [STRIDE_COST_GATE.md](../norm_branch_rank_pilot_20261115/STRIDE_COST_GATE.md).

## 3. What this eliminates and what remains open

The restricted positive control validates that our end-to-end gates **can** recognize a genuinely cheap complete solver when the group order has exploitable smoothness. However the mechanism relies explicitly on composite group order and provides no method for arbitrary prime-order ECDLP. The prime-order relation experiment confirms that a high-yield signed table and independently verified matrix relations do not automatically reduce total cold cost. A **prime-field** curve can have a **composite-order** point group; those are mathematically different security cases.

**Priority 1 — natural-positive sparse PKM m=8** (the actual remaining candidate). Implement the native system and witness extraction from #4091's tower engine, *without* using these 256-sum tables as an oracle. Use #4077's frozen natural targets at `log_B p` near 7, 8 and 9, preserve every failure, exact matrix rank increment, entire invocation work, peak memory, linear algebra and final scalar recovery. Fit both full-call exponent gamma and useful independent-output exponent delta on two held-out uncensored sizes. Necessary optimistic gate from [PKM_PARTIAL_OUTPUT_GATE.md](../norm_branch_rank_pilot_20261115/PKM_PARTIAL_OUTPUT_GATE.md):

    gamma < 3 + delta/2.

**Priority 2 — target-adaptive rank-locus.** Construct a scalar-blind public-target-dependent selector and **natural positive** witness extraction, then measure rank of useful output and full cold setup. The universal k=8 determinant-square identity from #4188 alone supplies neither.

**Priority 3 — transfers and implementation controls.** Continue #3877 only after a complete scalar-blind public map and arithmetic costs at every hop; separately optimize rho and projective/affine field inversion on matched hardware. These are not substitutes for the sparse algebraic feasibility experiment.

## Reproduce and provenance

On a complete checkout, standard-library Python 3.11+:

    python3 research/prime_order_end_to_end_gate_20261115/prime_signed_relation_pipeline.py
    python3 research/prime_order_end_to_end_gate_20261115/rho_prime_order_control.py
    python3 research/prime_order_end_to_end_gate_20261115/smooth_order_control.py

For each script, `python3 -O` produced byte-identical output, confirming measured group-operation verification does not vanish when assertions are disabled. `python3 -m py_compile` passed. All source Git blob IDs were checked against the local exact files; reproducibility SHA-256 digests and compact result records are in [RESULTS.json](RESULTS.json). These are **producer-local** tests, not independent PCC signatures; no leaderboard entry, comparative field-op timing, EKS Spot run, complete Rust suite or GitHub CI acceptance is claimed.

**Source audit:** the existing repo [Pohlig–Hellman explainer](https://isogenylabs.io/cryptanalysis/pohlig-hellman.html), the elliptic-curve Mersenne prime/cyclicity argument in [Savin's text, §3](https://www.math.utah.edu/~hacon/4400/Savin-book08_jun.pdf), [Pohlig–Hellman algorithm reference](https://flintlib.org/doc/algorithms/number-theory/discrete-log.html), [PKM prime-field algebraic approaches (PKC 2016)](https://doi.org/10.1007/978-3-662-49387-8_1), [Shoup generic lower bounds (1997)](https://doi.org/10.1007/3-540-69053-0_18), internal #4077, #4091, #4188, #3877. KB MCP/Conductor/native EKS were unavailable in this runtime, so do not infer priority or novelty.
