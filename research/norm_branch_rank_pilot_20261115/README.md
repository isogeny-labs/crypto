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

## 2026-11-15 follow-up: scalable cubic factorization and complete toy DLP [pcc:unverified]

The local cubic-square pattern is **still a conjecture** for arbitrary eight abscissae. The new `cubic_factor_fast.py` replaces the original `locus_selector.py`'s linear scan through all `p` field elements with a bounded Las Vegas factorization of the degree-three quotient. It first computes `gcd(cubic, X^p-X)` to count exactly the distinct `F_p` roots (including when the original cubic has repeated factors), splits the linear factors by randomized quadratic characters, checks each returned root against the cubic, and checks the rank of the original 6x6 matrix. Exhaustion of the splitting cap is an explicit error, never a partial root list. It does **not** verify primality of input `p`. The interpolation and square-cubic factorization remain guarded hypotheses and cause a failure if contradicted.

```sh
python3 research/norm_branch_rank_pilot_20261115/cubic_factor_fast.py --selftest
python3 research/norm_branch_rank_pilot_20261115/signed_support.py
python3 research/norm_branch_rank_pilot_20261115/toy_complete.py
```

Producer-local deterministic arithmetic tests, not signed independent validation: the original `F_1019` seven-tuple returns cubic `z^3-2z^2+79z+609`, unique noncollision root `764`, and rank `4/6`. In addition to `F_10007`, the factorization code was locally exercised on algebraic residue tuples at the **61-, 127- and 256-bit prime-field sizes** `2^61-1`, `2^127-1`, and `2^256-2^32-977`. These latter tuples are not validated elliptic-curve factor bases and cannot establish any ECDLP speed advantage. At each chosen input, the determinant polynomial is built from 19 evaluations, checked on 32 further inputs, and candidate ranks are rechecked. The computation of *roots* no longer loops over `p`; expected splitting work is polynomial in `log p` for fixed degree, conditional on the factorization hypothesis holding.

### Exact fixed-base hit ceiling (mathematical argument)

Let `H = <G>` have prime order `r` and let `P_1,...,P_8 in H` be **fixed before** sampling the uniform target `R in H`. Define `S = {sum_i epsilon_i P_i : epsilon_i in {-1,+1}}`. A single candidate signed-sum test has exactly

```text
Pr[R in S] = |S|/r <= 256/r.
```

This is a direct counting identity, not a conjecture: there are at most `2^8` signed sums and all `r` target points have equal probability. A cubic-selected rank-deficient base that depends only on seven fixed abscissae therefore cannot by itself increase the hit probability above `256/r`. The statement does **not** apply unchanged to a base chosen adaptively after seeing `R`, to a larger changing collection of bases, or to a scheme that exposes additional outputs per query. It is not an index-calculus lower bound.

On the guarded `E/F_1019: y^2=x^3+46x+158` fixture, `signed_support.py` counts exactly `#E(F_1019)=977`, a prime, and **256 distinct** signed sums from the selected eight points. Thus a *uniform* full-group target hits this fixed support with probability exactly `256/977`. This counting experiment is [pcc:unverified] until independently replayed.

### End-to-end toy result and cost boundary

`toy_complete.py` uses `P_1` as generator, independently recomputes all eight factor-base logarithms by standard baby-step/giant-step, creates the signed-sum table with 255 Gray-code updates, and recovers a scalar for **each of the 977 possible target points** by adding `G` until it finds the table. Every answer is re-multiplied to the target. Producer-local run: 977/977 correct, 32 baby table entries, 111 giant steps across eight base-log queries, 256 signed-table entries, 3958 total online shift additions (mean 4.051177, maximum 23). All figures are [pcc:unverified].

This is a **complete toy ECDLP solver with reusable generic preprocessing**, not a new asymptotic algorithm, not a speed comparison against a matched rho, and not evidence of a cheaper cold prime-field solver. The setup, including finding eight factor-base logs and building/storing 256 entries, must be charged for one cold target; the low per-target online step count cannot be substituted for that full cost.

### Next decisive tests

First prove or refute the general square-cubic determinant identity (including exceptional characteristic and collision strata). Then carry a preregistered held-out growing-`k` rank census into the norm-branch B1 gate, and allow SAT B2 only after a substantial rank compression survives controls. Independently execute the already specified sparse PKM `m=8` positive full-solver-call panel with extracted relations and its `gamma<3.5` whole-call gate; this remains the stronger direct cost-changing ECDLP lead. All claims here are [pcc:unverified].

**Literature and prior-work check:** This extends the current PR #4188 counterexample and protocol, not a claim of novelty in polynomial factorization. Root splitting uses the standard Cantor–Zassenhaus idea, and the toy preprocessing belongs to ordinary generic-group time/memory tradeoffs, as in the repository's `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`. Review also considered Shoup 1997 (generic model) and Yokoyama–Yasuda–Takahashi–Kogure 2020 (naive Semaev IC limits, DOI 10.1515/jmc-2019-0029). Research-KB MCP/catalog remained unavailable in this execution context; the repository lead heat map and primary sources served as fallback. No signed PCC standing, EKS benchmark, conductor scope lock, full Rust suite or independent host replay is claimed.

## 2026-11-15: exact integer one-parameter locus and a second cold-solver control [pcc:unverified]

This follow-up is deliberately **not** a proof of the conjectured square-cubic factorization for arbitrary seven fixed abscissae. It supplies a stronger, exact **infinite one-parameter specialization**, valid as an identity in `Z[z]`, plus an independent prime-order elliptic-curve fixture that quantifies the cold-cost penalty. No PCC-signed independent verification, EKS run, or cryptographic-size benchmark was obtained.

### Exact polynomial identity for a fixed seven-tuple

Let `K(z)` be the 6-by-6 matrix from `locus_selector.matrix([0,1,2,3,4,5,6,z],p)`, but formed first over the integers (before reduction modulo any prime). The script `exact_integer_locus.py` gives a deterministic coefficient-free proof certificate for

```text
det K(z) = -[41472 (z-4)^2 (z-5)^2 (z-6)^2
               (11879 z^3 - 58033 z^2 + 78934 z - 25880)]^2 .
```

**Why 19 evaluations suffice:** only the three columns indexed by right-hand pairs containing the eighth point depend on `z`. Each such column has polynomial degree at most six, so the determinant has degree at most 18. The displayed right-hand side has degree 18. Exact integer equality at the 19 distinct values `z=0,...,18` therefore proves the identity in `Q[z]`, and both expressions lie in `Z[z]`. Consequently the same **specialized** identity holds after reduction modulo *every* prime. This argument does not extrapolate to variable first seven abscissae. The script evaluates determinants by fraction-free Bareiss and separately cross-checks Leibniz expansions at two values.

Modulo the prime `1031`, the noncollision cubic root `z=338` gives rank **4**. All 36 minors of size 5 vanish; the leading 4-by-4 minor is `188 mod 1031`. The comparison at `z=7` has rank 6. These are exact local arithmetic checks [pcc:unverified]. They falsify a universal guarded full-rank assertion but do **not** satisfy the prospective B1 threshold `rank <= (null rank)/2` (4/6 > 1/2).

### Actual prime-order EC fixture and matched cold-cost accounting

The separate `prime_curve_fixture_1031.py` constructs

```text
p = 1031; E: y^2 = x^3 + 447 x + 997; #E(F_p) = 991 (prime).
P_1,...,P_8 =
 [(0,911),(1,528),(2,620),(3,731),
  (4,555),(5,518),(6,687),(338,592)].
```

The script checks the discriminant guard, each nonzero ordinate and curve equation, curve cardinality by direct point counting, primality of the cardinality by trial division, and the coefficient-matrix rank. With `P_1` as generator, it independently recovers and checks all 991 public toy target logarithms by ordinary BSGS and by the sign-table-and-shift algorithm. The eight-point signed support contains **252 distinct elements** out of 991, below the combinatorial ceiling 256.

The script counts calls to the same elliptic-curve addition routine, with a **fresh cold setup per target**, and averages the target-dependent query cost over all 991 known-answer points. It treats the generator's own logarithm as free to the signed solver; it omits curve/locus discovery, field-inversion weights, full verification expenses and memory costs. These omissions favor the signed alternative.

| Stage, group-addition calls | Ordinary BSGS | Signed table / shift |
| --- | ---: | ---: |
| Shared BSGS baby/giant-stride setup | 39 | 39 |
| Additional seven base-point logarithms | 0 | 90 |
| Build 256 sign assignments (252 unique entries) | 0 | 271 |
| Mean online target calls | 14.984864 | 4.896065 |
| **Mean one-target cold total** | **53.984864** | **404.896065** |

The 991-target census is a correctness sweep, **not** a justification for amortizing its setup. At this exact toy protocol the signed approach reduces online calls but **increases cold calls about 7.5-fold**, prior to charging locus selection. [pcc:unverified] This controls the interpretation of the previous 977-target demonstration; it is not a universal performance claim.

For fixed signed support `S` of size `s` in a prime-order group of size `r`, and a fixed target-independent set of `T` shifts `D`, the union `S-D` has size at most `sT`. Hence a uniformly sampled target has success probability **at most** `min(1,sT/r)`. Guaranteed coverage by such a fixed-shift strategy requires `T >= ceil(r/s)`. This elementary bound does not cover a target-adaptive algebraic relation algorithm and is not a lower bound on arbitrary prime-field ECDLP.

Reproduce from the repository root (Python 3.11+, standard library only):

```sh
python3 research/norm_branch_rank_pilot_20261115/exact_integer_locus.py
python3 research/norm_branch_rank_pilot_20261115/prime_curve_fixture_1031.py
```

**Next gated experiments:** (a) seek an arbitrary-seven symbolic determinant formula or a counterexample to the universal square-cubic conjecture; (b) compare genuine conditioned rank defects on preregistered growing-`k` holdouts with independent nonzero-coefficient nulls, both adjacent middle layers and exact positive-relation extraction; (c) put the native sparse PKM `m=8` natural-positive whole-call relation/rank panel on authorized EKS Spot and report full cold setup + relation finding + linear algebra + verified scalar recovery against matched rho. Do not promote a model from stage-only online costs.

Literature scope: the prime-field algebraic context was cross-checked against Petit–Kosters–Messeng, *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1), and Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves* (Finite Fields Appl. 51, 2018, DOI 10.1016/j.ffa.2018.01.009). No priority or novel general theorem is claimed. Research-KB MCP, repo wrapper, Conductor and EKS are not available in this execution environment; code and sources were reviewed via the GitHub connector and public primary records. All numerical claims are producer-local [pcc:unverified].

## 2026-11-15: unconditional collision-factor lemma and 7-point quotient interpolation [pcc:unverified]

An additional **symbolic argument, valid for arbitrary eight independent abscissae over `Z`**, establishes the universal part of the earlier finite-field factorization that had been treated only as an observation.

Write `K_(A,B)=Delta(x_(A union B))^2`, with `A` ranging over 2-subsets of indices 0..3 and `B` over 2-subsets of 4..7. Fix `j in {4,5,6}`, put `t=x_7-x_j`, and let `k,l` denote the other two right indices. The column for pair `{j,7}` is divisible by `t^2`, since every entry contains the factor `(x_7-x_j)^2`. Subtract column `{j,k}` from column `{7,k}` and, independently, column `{j,l}` from column `{7,l}`: both differences are divisible by `t` because the corresponding columns become equal upon specializing `x_7=x_j`. Thus the 6x6 determinant is divisible by `t^4`. The three monic irreducible differences `x_7-x_4`, `x_7-x_5`, `x_7-x_6` are pairwise relatively prime in `Z[x_0,...,x_7]`, so

```text
prod_(j=4..6) (x_7-x_j)^4  |  det K(x_0,...,x_7)
                                      in Z[x_0,...,x_7].
```

Only the three columns for right pairs containing index 7 depend on `x_7`, and each entry has degree at most six in `x_7`. Therefore `deg_(x_7)(det K) <= 18`, and the **quotient by the proven degree-12 collision factor has degree at most six**. This argument holds in every characteristic after reduction and requires neither a nonzero determinant nor an elliptic curve. It does **not** prove that the degree-six quotient is the square of a cubic for arbitrary other seven abscissae; that stronger statement remains conjectural.

The new `quotient_interpolate.py` determines the quotient from **seven** distinct noncollision field evaluations instead of 19 determinant samples. It retains the same 32 independent held-out determinant checks and compares its output with the original degree-18 interpolation/division path in a separate execution mode. `cubic_factor_fast.py` now uses the new routine in its selector, retains the exact polynomial-square assertion as a fail-closed gate, and reports `interpolation_points=7`, `quotient_degree` and the unchanged 32 holdouts. The determinant-evaluation count for interpolation + 32 checks drops from **51 to 39** (a source-level reduction in calls, not a measured runtime speedup or ECDLP cost change).

A local standalone helper-equivalent replay reproduced the original quotient coefficients exactly on four fixed test families over F_1019, F_10007, F_(2^61-1), and F_(2^127-1), with degree six in all four, and exercised the new quotient at the 256-bit NIST prime modulus. These are [pcc:unverified] producer-local arithmetic checks; the exact checked-in integration still needs independent CI/repository replay.

Run from repository root:

```sh
python3 research/norm_branch_rank_pilot_20261115/quotient_interpolate.py
python3 research/norm_branch_rank_pilot_20261115/cubic_factor_fast.py --selftest
```

**Experiment decision:** the collision quartic is now a mathematically established factor, so future work should no longer spend time fitting or testing that component. The remaining novel algebraic question is whether the **degree-six quotient is generically a square of a cubic**, why noncollision roots yield corank two rather than just one, and whether either structure produces **positive algebraic ECDLP relations** on a growing-`k` held-out campaign. The fixed eight-sum support and cold-cost boundaries from the preceding sections are unchanged.


## 2026-11-15: universal eight-variable negative-square identity [pcc:unverified]

**This section supersedes the historically preserved conjecture** about the balanced 6-by-6 determinant's square-cubic quotient. The following is an exact **computer-assisted polynomial identity certificate** in the ring of integer polynomials, not an accepted Lean theorem or a signed independent PCC verdict. Its mathematical degree argument explains why the finite tests establish an infinite identity if the implementation is correct.

Let L={0,1,2,3}, R={4,5,6,7}, and K have rows I in binom(L,2), columns J in binom(R,2), with K[I,J]=Delta(x_(I union J))^2. Put V_L=prod_(i<j in L)(x_j-x_i), and V_R similarly. Define C[i,j]=(x_(j+4)-x_i)^2 (0<=i,j<4) and N[I,J]=prod_(i in I,j in J) C[i,j] (right pairs reindexed 0..3). Then

    det K = V_L^2 V_R^2 det N .

The previous universal **collision quartic** argument works for every pair on each four-element side by exchanging rows or columns. Thus V_L^4 V_R^4 divides det K over Z[x0,...,x7], and Q=det N/(V_L^2 V_R^2) is an integer polynomial. Each x_i occurs in precisely three rows or columns of N, each with degree <=4 in that variable. Therefore deg_(x_i) det N <=12, and removal of the three internal squared differences yields **deg_(x_i) Q <=6**. Q is translation-invariant and homogeneous of total degree 24.

The executable [universal_square_certificate.py](universal_square_certificate.py) does:

1. Normalize x0=0, x1=1 by an affine action over a rational-function field; set (x2,...,x7)=(10+t0,20+t1,30+t2,40+t3,50+t4,60+t5).
2. Evaluate 4^6=4,096 exact integer values of Q at t_i in {0,1,2,3}; at each, -Q is a positive integer square. Choose the positive roots and tensor-interpolate the unique H of degree <=3 separately in each t_i.
3. At all 7^6=117,649 grid points t_i in {0,...,6}, evaluate the determinant Q independently and check Q=-H^2 using the *previously interpolated* polynomial. Both sides have degree <=6 in each variable, so successive univariate root bounds prove the identity in Q[t0,...,t5].
4. Convert H to ordinary monomial coefficients with exact fractions. All **3,326 nonzero coefficients are integers** and the **total degree is 12**. Coefficient-stream SHA-256: **3a3bbd39a96a7e185eb84126bcff98812aaca6290325398dece037ee7c8b8ada**. Also cross-check original K, factored N, and a separate 720-term Leibniz determinant at two integer inputs.
5. Let d=x1-x0. Because H has integer coefficients and total degree <=12, the explicit homogenization d^12 H((x2-x0)/d-10,...,(x7-x0)/d-60) is an integer polynomial. Affine covariance and degree-24 homogeneity of Q extend the normalized identity to eight formal variables; polynomial continuation covers collisions.

**Result (exact producer-local computer-assisted identity):**

    det K = -[ V_L^2 V_R^2 S(x0,...,x7) ]^2

for some S in Z[x0,...,x7] of total degree 12. The identity remains valid after reduction into any field. For seven fixed abscissae, the degree-six noncollision quotient in the eighth variable is minus the square of a polynomial of degree <=3 (possibly degenerate); normalization to a monic cubic is valid in odd prime characteristic when the leading coefficient is nonzero. This resolves the k=8 algebraic square-cubic conjecture, not higher-k rank growth, sign positivity, exact corank at a cubic root, coverage beyond the fixed 256 signed sums, or cheaper ECDLP.

**Reproduce:** python3 research/norm_branch_rank_pilot_20261115/universal_square_certificate.py

Local output reported seed_checks=4096, polynomial_identity_checks=117649, nonzero_integer_coefficients=3326, max_total_degree=12 and independent_leibniz_checks=2. Those are producer-local **[pcc:unverified]** observations pending independent replay and signed PCC. The original one-parameter examples and 991-order cold-cost control remain preserved.

### Lead assessment and next registered tests

- **Sparse PKM m=8** (highest direct cost priority, #4077): run a natural-positive relation extractor and record failures, complete invocation costs, useful independent relation/rank increments, setup, linear algebra and individual log recovery against the strongest matched rho. Optimistic full-output whole-call exponent gamma must be below 3.5 (below 3 for one-output). Cheap stages or algebraic rank alone cannot pass.
- **Norm-branch rank-to-positivity**: preregister held-out growing-k distinct-x/characteristic panels, both sign layers and support-matched independent-coefficient nulls. Separately measure how often noncollision k=8 cubic roots yield *positive* EC sign witnesses, conditioned vs unconditioned hit rates, target-independent sampling, curve/point selection costs, and actual recovered relations. Do not confuse this universal structural square with a growing-k compression law.
- **Transfer chains** (#3877): require scalar-blind public-point transport and an explicit cheaper final complete solve. A changed prime-field fixture or a torsion lift presuming the original log does not meet that gate.
- **Whole-solver accounting**: record memory, all failed candidates, field operations, useful relation ranks, generator/factor-base precomputation, cold single-target and amortized multi-target costs separately, and post-verified scalar answers.

**Sources checked:** root AGENTS.md; #4077 prospective PKM/tensor protocol; #4161 304-lead heat map; #4188 polynomial and toy controls; #3877 transfer status; Petit–Kosters–Messeng, *Algebraic Approaches for the ECDLP over Prime Fields*, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Shoup, *Lower Bounds for Discrete Logarithms and Related Problems*, EUROCRYPT 1997, DOI 10.1007/3-540-69053-0_18. Web search for an exact rank-three compound determinant-square precedent was inconclusive: **no novelty/priority claim**. Research-KB MCP tools and wrapper were unavailable; GitHub primary sources and the repository atlas were the fallback. No Conductor/EKS benchmark, full Rust workspace, independent PCC signature or leaderboard promotion is claimed.
