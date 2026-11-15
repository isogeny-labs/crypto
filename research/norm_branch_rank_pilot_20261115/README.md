# Prime-field ECDLP: signed norm-branch rank pilot (2026-11-15)

**Scope:** toy diagnostic of a proposed non-generic relation finder, *not* a cheaper complete DLP solver. All observations are [pcc:unverified]; no signed verdict, independent run, cost measurement, or leaderboard promotion. Motivated by [PR #4077](https://github.com/isogeny-labs/crypto/pull/4077), especially its prospective part-B protocol; this is a **retrospective pilot**, not its held-out B1 campaign.

## Exact coefficient identity

For distinct abscissae x_i, nonzero y_i, d=k+1 points on a nonsingular curve y²=f(x)=x³+ax+b over odd F_p, set u=floor(d/2)+1, v=floor((d-1)/2). The fixed-target sign is +1. Define the d-by-d signed interpolation matrix M(epsilon)=[U | diag(epsilon)W], U_ij=x_i^j (0≤j<u), W_ij=y_i*x_i^j (0≤j<v). For every v-element row subset S, Laplace expansion gives

    c_S=(-1)^(sum(S)+u*v+v*(v-1)/2) * Delta(x_(S^c))*Delta(x_S)*product_(i in S)y_i.

All row indices in this formula are zero-based. Let F(X)=product_i(X-x_i) and Delta(x_I)=product_(i<j in I)(x_j-x_i). The **same coefficient** obeys

    c_S=(-1)^(v*(v-1)/2) * Delta(x_all) * Delta(x_S)^2
        * product_(i in S)(y_i/F'(x_i)).

Proof: expand Delta(x_all) into within-S, within-S^c, and cross terms. In product_(i in S)F'(x_i), the within-S terms are (-1)^(v*(v-1)/2)*Delta(x_S)^2; the cross terms cancel those from Delta(x_all). The remaining permutation parity equals the Laplace row-selection sign. The Python script compares both closed forms term by term; the mathematical argument is field-independent under the stated guards.

**Consequence:** in each fixed-cardinality coefficient block, replacing y_i by nonzero multiples rescales rows and columns by invertible diagonal matrices, including the common target factor. Its rank depends only on the abscissae, not the ordinates. Thus ordinate randomization is an exact invariance control, not an independent null model. Squared-Vandermonde coefficients can be evaluated efficiently, but that does not imply a cheap *zero-sign assignment search*.

## Reproduce

From the repository root, on Python 3.11+ with no external packages:

    python3 research/norm_branch_rank_pilot_20261115/run.py 18 > /tmp/norm-pilot-18.json

The deterministic seed is 19. B0 semantic control runs over E: y²=x³+2x+3 on F_1019, k=8, exhausting all 256 variable sign assignments for a planted public target. It compares three independent predicates: elliptic-curve group sum O, signed interpolation matrix singularity, and Laplace coefficient-polynomial zero. Accepted signs give (A,B) null vectors, followed by coefficientwise norm checks:

    A(x)^2 - f(x)*B(x)^2 = c * product_i(x-x_i), c != 0.

The target-sign flip maps models to their coordinatewise negatives. Duplicate abscissae fail closed, and changing the curve b parameter fails point membership.

B1 diagnostic over F_10007 samples curve-liftable, distinct abscissae for k=6,8,10,12,14,16,18. Each balanced split includes both adjacent middle layers and every supported cardinality block. The script computes exact F_p Gaussian ranks for natural coefficients, ordinate-rescaled coefficients, independent nonzero support-matched random coefficients (null), and synthetic rank-one matrices (positive control). The entire deterministic census is regenerated and checked equal (A/A). All data rows print in the stdout JSON.

## Local observations [pcc:unverified]

| k | blocks | largest square block | natural rank deficits | natural below coefficient-null |
|---:|---:|---:|---:|---:|
| 6 | 7 | 3×3 | 0 | 0 |
| 8 | 9 | 6×6 | 0 | 0 |
| 10 | 11 | 10×10 | 0 | 0 |
| 12 | 13 | 20×20 | 0 | 0 |
| 14 | 15 | 35×35 | 0 | 0 |
| 16 | 17 | 70×70 | 0 | 0 |
| 18 | 19 | 126×126 | 0 | 0 |
| **Total** | **91** | | **0** | **0** |

Every ordinate-invariance and rank-one control passed locally; the B0 toy fixture had one identical accepted model under all three predicates and a checked norm certificate. The sample did **not** exhibit a linear-message rank-compression gap, but does **not** establish an all-size rank theorem. The rank pilot is a producer-local test, **not** a native EKS experiment or a preregistered holdout.

SHA-256 of the locally run Python script: 9b66c469e7c2a94a6edd86333badc60765e85d4905060edccfd939597d7d419d. SHA-256 of deterministic stdout JSON: 0ffb8ab8f5b9268fbf8d11bc239004a68ec85c84be66ca3200846ada159bb91f. Neither is a PCC validation signature.

## Complete-solver research decisions

**Sparse PKM m=8 remains the strongest named algebraic cost-changing lead.** The optimistic full-output single-target model from the prior audit requires per-call work B^(gamma+o(1)) with gamma < 3.5; a one-useful-output model requires gamma < 3. Natural positive sparse solves, usable independent rows, setup, witness extraction, failures, linear algebra and final scalar verification must be priced before ranking against the strongest matched rho. The D(8,3) dense closure by itself is only a solving-degree diagnostic. [pcc:unverified]

**Norm-branch B1:** no gap observed on 91 small blocks. Attempt a proof/refutation of the compound-Vandermonde full-rank statement for guarded x-families; if absent, pre-register distinct held-out x-families and model-count strata. Only advance to SAT B2 after B1 shows its required reproducible rank deficit and scaling margin. The present pilot does not justify SAT tuning. [pcc:unverified]

**Chained Jacobian/cross-characteristic routes:** [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877) documents constructive toy transfers but not yet a complete scalar-blind map into a demonstrably cheaper DLP domain. Require a public-input map, defined treatment of exceptional points, exact scalar preservation, priced arithmetic of every hop, and a complete final scalar recovery. [pcc:unverified]

**Generic decomposition / accelerated rho:** component and batched optimizations can lower engineering cost, but a fair single-cold comparison must charge both complete pipelines on identical inputs. Amortized multi-target ratios are separate from the requested cold solve. The code in this directory does **not** solve arbitrary ECDLP instances.

## Literature and evidence

Repository evidence: [five-lead report](../prime_leads_chase_20261113/README.md), [continuation protocol](../prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md), [PKM native implementation #4091](https://github.com/isogeny-labs/crypto/pull/4091), [chained transfers #3877](https://github.com/isogeny-labs/crypto/pull/3877).

Primary literature checked: [Shoup 1997](https://doi.org/10.1007/3-540-69053-0_18) (generic group, not a universal lower bound); [Petit–Kosters–Messeng 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and [Amadori–Pintore–Sala 2018](https://ora.ox.ac.uk/objects/uuid:c99fdb39-f5b1-428d-999c-770b63363e25) (prime-field algebraic approaches); and [Corrigan-Gibbs–Henzinger–Wu, EUROCRYPT 2026](https://people.eecs.berkeley.edu/~henrycg/pubs/structured-generic-groups/) (bounded structural generic model, not a blanket algebraic obstruction).

Research-KB MCP tools are not exposed in this runtime; the newest session log also reports the wrapper catalog and encrypted atlas unavailable. GitHub repository search and primary papers were the fallback; **no novelty or exhaustive-prior-art assertion**. Conductor, EKS Spot, an independent host, and a PCC signing verifier were unavailable here. No execution, speed, complexity or security standing is claimed beyond [pcc:unverified].


## Follow-up: exact guarded rank-deficiency certificates and a cubic selector

The initial single-seed random census above remains intact, but **it does not support a universal full-rank conjecture**. A targeted counterexample was subsequently found on the same day. The following exact checked examples refute full rank under *only* the distinct-abscissa/nonzero-ordinate/odd-characteristic guards, although they do **not** satisfy the proposed B1 advancement criterion of rank <= half the support-matched null on held-out growing sizes. All results below remain [pcc:unverified].

### First finite counterexample (F_19)

The script `counterexample.py` uses E: y²=x³+5 (mod 19) and the nine valid distinct-x points

    [(0,9),(1,5),(5,4),(7,5),(10,6),(12,2),(15,6),(17,4),(18,2)].

For the eight variable signs, take the layer t=4 and balanced 4+4 split with row subsets of size 2 and column subsets of size 2. The resulting 6×6 coefficient flattening has rank **4**, not 6. All 36 possible 5×5 minors vanish, and an exact nonzero 4×4 minor witnesses rank at least 4. A support-matched seeded independent nonzero-coefficient null is rank 6. The exhaustive 256-sign equivalence test finds nine signed elliptic identities, and *each* one has a validated A/B norm-polynomial certificate. This is a small-characteristic structural exception, not a cryptographic-size attack.

### Stronger counterexample (F_1019; single positive target)

The script `counterexample_1019.py` gives E: y²=x³+46x+158 (mod 1019) and the nine distinct, nonzero-y points

    [(944,936),(243,525),(432,632),(225,527),(627,518),
     (387,878),(914,704),(764,816),(429,379)].

The last point is the fixed signed target. The first eight yield the same layer t=4, split 4+4, (a,b)=(2,2) 6×6 coefficient matrix. Its exact rank is **4**. All 36 5×5 determinants vanish; the leading 4×4 minor equals **126 mod 1019**. An independent seeded nonzero-coefficient 6×6 null has determinant **602 mod 1019**. Exhaustive 256-sign comparison of EC group addition, interpolation singularity, and sign polynomial has precisely **one** positive sign model, `(+,+,+,+,+,+,+,-)`; its full A/B norm certificate verifies. This refutes the naive claim that distinct x and characteristic exceeding k suffice for full block rank.

### Why this is algorithmically testable: a one-variable determinant locus

After removing invertible row/column scales, each balanced 2+2 entry equals

    K_(A,B)=Delta(x_(A union B))².

Fix seven abscissae `[944,243,432,225,627,387,914]`, and let the eighth `z` vary. Over F_1019, **the exact determinant polynomial for this 6×6 squared-Vandermonde matrix** interpolates to

    det K(z) =
      163 * (z-627)^4 * (z-387)^4 * (z-914)^4
          * (z³ - 2*z² + 79*z + 609)^2   (mod 1019).

The noncollision root `z=764` produces the rank-four counterexample, with y=816 on the specified curve. It is selected by a cubic equation rather than by scanning random factor bases. The target x=429 is **not** one of the eight variables used in this layer; this specific flattening deficiency is target-independent. This structural fact does not by itself accelerate sign search, relation finding, or scalar recovery.

The self-contained `locus_selector.py` interpolates det K(z) from 19 evaluations, verifies the predicted degree-18 polynomial on 32 untouched z values, divides the three collision factors to fourth power, and reconstructs the **square of a monic cubic** from the quotient. It reproduces the F_1019 example and five independently seeded seven-tuples over F_1019 and F_10007. All six trials match the factorization form; noncollision cubic roots in the six cases number **1, 1, 1, 0, 0, 3**, and all selected roots yield rank 4. The general square-cubic factorization is **an experimental conjecture**, not a theorem; its proof and characteristic restrictions remain open. The root enumeration in the toy script is O(p), not a claimed cryptographic-size selector. A future implementation could instead factor the resulting cubic over F_p.

Reproduction:

    python3 research/norm_branch_rank_pilot_20261115/counterexample.py
    python3 research/norm_branch_rank_pilot_20261115/counterexample_1019.py
    python3 research/norm_branch_rank_pilot_20261115/locus_selector.py

### Updated experiment decision

1. **Prove or refute the cubic-square identity** for arbitrary distinct abscissae and p>2. Check the determinant interpolation against a separately derived symbolic or exact fraction-free elimination identity, including characteristic 3 and collisions. A six-fixture finite-field success is not a proof.
2. **Targeted factor-base B1 campaign:** construct liftable eight-abscissa bases by cubic root selection, match random eight-abscissa controls and random-coefficient nulls, and measure ranks in *both* adjacent target-sign layers. Extend to growing k and multiple held-out primes; quantify rank/nulllity, target-model counts, witnesses, preprocessing cost and rank slope. A fixed 6×6 rank-4 block does not meet the preregistered rank-at-most-half gate.
3. **Only if compression scales, measure whole-query complexity:** exact model extraction and verified positive/negative certificates, amortized vs cold setup, elimination proof width and final public DLP solve. Compare to the strongest batched rho and sparse PKM under identical conditions. No proposed algorithm has yet shown a cheaper complete single-target solve.

The new scripts are **small deterministic exact-arithmetic test artifacts**, not an independently certified research claim. No pcc record/signature or external CI rerun is implied by checking these files into GitHub.
