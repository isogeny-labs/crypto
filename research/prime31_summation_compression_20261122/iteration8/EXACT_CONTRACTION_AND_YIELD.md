# Iteration 8 — exact signed four-summand contraction, misses, and yield gate

Research continuation after merged #4928 and #4924, dated 2026-11-22 America/Los_Angeles (2026-11-23 UTC).
**Standing: independent written algebra and source-bound CI audit, [pcc:unverified].** No Rust was run, no independent signed certificate issued, no benchmark made, and no faster root solver or ECDLP exponent proved. This is a precise **existing-method control**, not a novelty claim for meet-in-the-middle, S3 chains, group algebra, or Fourier analysis.

## 1. Inputs and exact contraction

Let G be a finite abelian group of **odd prime order** n, with known canonical point encoding and group arithmetic but **without** a discrete-log coordinate map. Let W be B different rational x-coordinates on a supplied nonsingular curve over Fp, each with both rational ordinates. Put

    A = { P(x), -P(x) : x in W },  L = |A| = 2B.

Identity O is not a leaf, but **is permitted as a subtotal**. The full Dickson fibre construction of iteration7 can generate W for an eligible chosen c, with its separate expected setup C_fibre(q,p); this contraction is independent of the way W is chosen.

Work in the nonnegative-integer group semiring N[G]. Let

    F_0(g) = 1 if g=O, otherwise 0,
    F_(j+1)(g) = sum_(a in A) F_j(g-a).

Then F_j(g) is exactly the number of ordered, **signed** j-tuples in A^j summing to g, including repeated indices and cancellations. Induction on j proves the invariant. For m=4,

    F_4(R) = sum_(s in G) F_2(s) F_2(R-s).                 (1)

This is a genuine contraction along the intermediate group sum s: there is no expansion of S5, no Frobenius-closure root, and no guessed sign. It is also an exact alternative semantic elimination for the freely ordered left-associated rational S3/Q1-Q3 tree: for R != O and 4 <= n, iteration6's backward ordering lemma supplies an admissible permutation of every signed multiset; compute its nonzero prefixes, then the unique oriented rational x/y/slopes. To enumerate **every ordered affine-chain solution**, filter all at most 4! permutations for nonzero prefixes and reconstruct slopes. To enumerate group tuples, retain the order as given. The balanced fixed-tree counterexample P,P,P,-P remains valid and must **not** be erased by (1).

### Fully specified algorithm, including an empty query

Preparation:
1. Construct and verify all L distinct signed curve points, square-rooting f(x) and checking curve equations. Include every negative sign and reject any nonrational root. Charge fibre selection, rejected trials and all square roots.
2. For each ordered (i,j) in [L]^2, compute s=A_i+A_j with the native exact group law. Store the pair label in H[s], keyed by the **canonical entire group point**, including O. Let D=|supp F_2|.
3. Optionally store multiplicity F_2(s)=|H[s]|. Avoid fixed-width overflow; it is at most L^2.

Query(R), even on a miss:
1. For every distinct s in H, compute t=R-s and look up H[t]. If absent, continue. All D group additions/lookups are charged even when no witness exists.
2. For each ordered pair of labels from H[s] x H[t], emit (i,j,k,l), check literal A_i+A_j+A_k+A_l=R and retain the signed row. No ordered tuple can appear under two s keys, because its first pair determines s.
3. If no pair matched, return **certified empty** relative to the supplied, verified A. For an affine-chain assignment, reconstruct and verify nonzero prefixes and Q1-Q3 on admissible orderings. Repetitions and O subtotals are not silently dropped.

Correctness follows directly from the unique split s=A_i+A_j in (1). In particular, the algorithm captures all R, all misses, repeated leaves, the two possible leaf signs, zero subtotals and the tuple P,P,P,-P. It is complete for this exact finite signed base, not for an unspecified polynomial closure or an unverified proper subgroup. For odd-order groups no rational leaf has y=0; point at infinity must still be handled in the sum dictionary.

### Charged complexity

Let C_add and C_key be costs of native exact group addition and canonical lookup/insertion, and let Y_4(R)=F_4(R) be the **ordered** output count.

    C_setup = C_fibre(q,p) + Theta(L^2(C_add+C_key)) + O(L) point lifts.
    M_setup = Theta(q+L^2) point/pair labels, D<=min(n,L^2).
    C_query(R) = Theta(D(C_add+C_key)) + O(Y_4(R) C_verify).
    M_query_extra = O(Y_4(R)) if materializing outputs, O(1) streaming.
    C_affine_reconstruct = O(4! * 4 * Y_4(R) C_add), fixed-arity overhead.

Expected-time hashing requires its own collision/rehash assumption; an exact balanced tree gives O(D log D + Y_4(R)) ordered comparisons and no probabilistic lookup correctness. An economical **decision/one-witness** table can retain one labelled pair per s and use O(D) memory, but cannot claim the full output contract. An all-output table stores L^2 pair labels. If only multiplicities are stored, the integer counters have up to O(log L) bits here; larger j has O(j log L) bits.

For general j, if only support/counts are stored, the group-semiring recurrence requires at most O(L D_j) point updates, with D_j<=min(n,L^j), and O(D_(j+1)) next-layer states; retaining all ordered j-tuple labels may cost L^j storage. This recurrence counts **actual intermediate dimensions** rather than input equation degrees. It is a control implementation strategy; it is not a low-degree algebraic-ideal elimination-cost bound.

## 2. Proof of exact empty-target yield and a rigorous rank-attempt gate

For any fixed A and a uniformly random target R in G,

    sum_(R in G) Y_4(R) = L^4,
    E_R[Y_4(R)] = L^4/n,
    Pr_R[Y_4(R)>0] <= min(1,L^4/n).                   (2)

The first equality counts each ordered tuple once at its unique sum; the other claims are linearity and Markov. They require **no independent-output, pseudorandom-base, Poisson or rank heuristic**.

For odd-order G and A=-A without identity, three disjoint-by-pattern families with possible intersections of zero-sum tuples are

    (a,-a,b,-b),  (a,b,-a,-b),  (a,b,-b,-a).

Each has L^2 members; each pairwise intersection has L members; the triple intersection is empty (it would force a=-a). Hence **at least 3L^2-3L distinct zero-sum quadruples** exist, giving for a uniform *nonzero* target

    Pr_R!=O[Y_4(R)>0] <= (L^4 - 3L^2 + 3L)/(n-1).   (3)

This is an upper bound, not an exact probability or a randomness assertion. Some additional nontrivial zero sums may exist.

For the inherited ordinary Prime31 fixture (not newly certified here),

    p=2147483647, n=2147446831, B=31, L=62,
    L^4=14776336,
    E_uniform_group[Y_4]=14776336/2147446831 = 0.006880885611...,
    Pr_uniform_group[empty] >= 0.993119114389...,
    Pr_uniform_nonzero[empty] >= 0.993124397869....

Thus a **uniform blind four-summand target at B31 is at least 99.31% likely to be empty**, regardless of the algebraic structure of the signed base; planted positive chains are not representative of this task. No target sample or Rust experiment is claimed here.

Suppose a protocol needs B independent rows to determine B factor-base logarithms. If it makes T uniformly distributed targets (each marginally uniform, not necessarily independent), and returns all ordered tuples for each, then rank is at most the number of returned tuples. Consequently

    E[rank] <= T L^4/n,
    Pr[rank>=B] <= T L^4/(n B),
    T >= eta * B*n/L^4                                  (4)

is **necessary** for probability at least eta of reaching rank B. At the B31 fixture and eta=0.9 this necessary lower bound is T>=4055 integer queries. Dependence of row vectors, failed signed replay, duplicate rows, and the target-coefficient column can only increase the required count. This is a bound within **uniform-target** sampling; adaptive or nonuniform one-dependency architectures are not excluded. Every query in (4), including a miss, costs the entire query charge.

### Optional distribution diagnostic, not a DLP-free Fourier oracle

For the complex characters chi of G let Ahat(chi)=sum_(a in A) chi(a). Orthogonality and Parseval yield

    sum_R Y_4(R)^2 = (1/n) sum_chi |Ahat(chi)|^8,
    |supp Y_4| >= n L^8 / ( L^8 + sum_(chi != 1) |Ahat(chi)|^8 ).   (5)

The support inequality is Cauchy-Schwarz. This tests how much additive structure concentrates the target relation counts and why the first moment alone cannot **lower-bound** the success fraction. It is a mathematical identity, **not** a computable fast oracle from point encodings: using scalar character labels generally requires elliptic discrete logs. For small validation groups the native pair/triple histograms, not unknown character exponents, are the permissible exact controls.

## 3. What this says about the quadratic circuit and conventional complexity

For the signed four-summand contraction in (1), if D=Theta(B^2), the empty-inclusive query has tau=2 with pair-table setup sigma=2. This **does not** satisfy the recorded conditional full-row criterion

    alpha=max(sigma,2),  alpha+tau < m-1=3.

Assuming the conventional uniform-target independent-row model *and* usable factor-base range, its crude cold cost is

    C_cold(B) = Omega(B^2 + (n/B^3) B^2)
              = Omega(B^2+n/B),

up to constants, signed verification and field pricing. Minimizing this expression freely gives B~n^(1/3) and exponent 2/3. This is a **scoped accounting obstruction for this explicit scan**, not a lower bound for all algorithms, all Semaev elimination, or all reporting indexes. D could be smaller if the base has high additive collision energy; measure D and the exact pair-sum histogram rather than substituting D~B^2 without a receipt.

General static split a+(4-a): precomputing all b=4-a sums costs O(B^b) preparation, while scanning all a-sums at query time costs O(B^a+output), if that a-list is also prepared. The b=3,a=1 option has (sigma,tau)=(3,1). Fully materializing all four-sums has (4,0+output). In the simple split-table model sigma+tau>=4, and none passes alpha+tau<3. More advanced additive indexing and nonenumerative algebraic elimination are **outside** this statement.

For an arbitrary source base A, put its pair-sum collision energy E_2(A)=sum_s F_2(s)^2. Cauchy-Schwarz gives D>=L^4/E_2(A). If E_2(A)=O(L^2), then D=Omega(L^2), and the full-scan query indeed costs Omega(B^2) within the specified algorithm. This does **not** establish a lower bound for other indexes, nor does iteration7's dense fibre prove low energy.

A genuinely improved rational-Q1-Q3 circuit solver must therefore specify a different, exact elimination procedure and prove its **empty-instance** cost, finite-field root isolation, memory, all source setup, authentic signs, full rank/target recovery, and matched baselines. Quadratic input degree, compact Dickson ladders, first-fall degree, and supplied-chain checks do not resolve these terms.

## 4. Growing-family exponent admissibility (separate from the solver)

The iteration7 least-prime construction chooses Q=2^k, p congruent to -1 modulo 4Q, and p<=C(4Q)^Lambda for some *justified* Linnik exponent Lambda. For any fixed beta<min(1/Lambda,1/2), sufficiently large p admit a smaller power-of-two q' of size Theta(p^beta) dividing Q and with 4q'|p+1 and q'<=sqrt(p)/16. On a supplied odd-order source curve, the fixed-source fibre-density theorem then constructs B=Theta(q')=Theta(p^beta), after expected O(q' log p) field work, for a selectable fibre c'. This avoids taking an arbitrary subset that would invalidate the full-Dickson membership predicate. A large prime source subgroup and its signed-base intersection are **additional unresolved prerequisites**.

If setup exponent sigma, matrix exponent 2, empty-inclusive query exponent tau and uniform full-rank/yield model are actually supported, write alpha=max(sigma,2) and use B=p^beta. A **sufficient exponent budget in that model**, subject to the listed assumptions, is

    alpha*beta < 1/2    and    1 - beta*(3-tau) < 1/2,
    i.e.  1/[2(3-tau)] < beta < 1/(2alpha), tau<3.     (6)

For alpha=2 this interval is nonempty exactly when tau<1. For an **unconditionally guaranteed selectable beta below 1/Lambda**, condition (6) additionally requires

    1/[2(3-tau)] < min(1/4,1/Lambda).                 (7)

The historically established Xylouris exponent Lambda=5 only guarantees beta<1/5 by this route, so the present **field-availability guarantee** supports the model's rho-beating exponent budget only for tau<1/2 (strict) at alpha=2. This is a guarantee limitation, not evidence that larger fibres never occur. A 2026 Naslund preprint reports Lambda=3.99; because its complete proof and global verification remain disputed/incomplete, using it here is expressly **conditional on validation**. If established, (7) would remove this particular base-size gap for any tau<1, but it would still supply no source prime-order family or target solver. No growing-family breakthrough is inferred.

At beta<1/4, the first moment (2) gives E[Y_4]=O(p^(4beta-1))=o(1) when n=Theta(p). Thus **almost all uniform queries are empty** throughout the alpha=2 sub-square-root candidate range. A planted-positive-only pilot is necessarily an inadequate cost measurement.

## 5. Exact-source evidence and retained failures (read-only observation)

- #4928 was observed **merged** 2026-11-23 02:32:58 UTC, last PR head fc184b03d47e13ec922c9b489f3b3f8a976a8bd9. #4924 was observed **merged** 2026-11-23 02:29:37 UTC, last PR head 84659f850676ea01aebfe9a90747b5000c6215b0. Neither merged PR should be maintained as an open lane. This independent follow-up changes no source owned by those completed PRs.
- Original rational-chain gate #45096187034/job137436389923 failed before candidate tests on the inherited ecbench/spec.rs parsing issue. Subsequent restoration commit a4ddc035... gate #45097885135/job137441679059 also **failed** at the inherited regression stage: the log at 2026-11-23 02:10:54 UTC reports an unclosed delimiter in src/cryptanalysis/ic_framework/mod.rs:4528, no rational-chain test reached. Preserve both distinct failure reasons.
- Later original-lock Dickson gate run #45100829948/job137451053331 also **failed** before candidate steps at Cargo metadata, because root Cargo.toml:132 has duplicate key name="curve-transfer-sweep". A success for the iteration6 or merged-head native verifier is **not** claimed from those runs. Earlier #45089569480 (iteration1) and #45089472283 (companion at its historical source) are not interchangeable with these source identities.
- Companion monic-t source changed after the historical 96c910... CI; the old printed reference_scan_ms was unordered quadratic S3-pair enumeration, not the proper O(B) group-complement baseline. Do not promote timings or binary-source claims for later bytes without a matching checkout receipt.
- The connected Space research Page remains page_729b0b80bdb4819193f457c80ecabe84. This runtime's attempted read returned “User input required ... non-interactive mode.” No Page mutation or recreated Space occurred.

## 6. Next falsifiable solver gate

Freeze a **specific exact algebraic contraction** for the existing Q1-Q3 + Dickson system on the original Prime31 curve, with a designated rational-root isolation and negative certificate. Match its signed tuples/empty results against the present DLP-free group-semiring contraction (1), first on every F101 target and then on natural uniformly sampled Prime31 targets with B=31, 64 and larger eligible fibres; include all zero-subtotal, repeated-leaf and P,P,P,-P controls. Count actual state dimensions D_j, multiplication/inversion, failure branches, setup, memory, and all outputs. Check native source/lock CI actually reaches the candidate after unrelated shared parser/manifest failures are repaired by their owning lanes. Only after pre-registration/admission under EKS rules compare complete signed-rank and cold target recovery with the strongest maintained IC and negation-map rho.

## Sources and scope boundaries

- Existing exact quadratic derivation, rational comb ordering and sign-ghost: iteration6/MATHEMATICAL_NOTE.md; iteration6/ROOT_REVIEW.md; existing state, source freezes and iteration7/FIXED_CURVE_FIBRE_DENSITY.md from merged #4928.
- Existing monic and Newton pilots, independently verified at their older source commits: merged #4924; old quadratic reference-scan warning in iteration6/SOURCE_AUDIT.md.
- Semaev 2015 rational-auxiliary S3 chains: https://eprint.iacr.org/2015/310 ; Faugere et al. EUROCRYPT 2014 symmetric Semaev prior, DOI 10.1007/978-3-642-55220-5_3. No novelty for those mechanisms.
- Newer integer-indexing and generic reporting caveats already audited in iteration6/LITERATURE_AUDIT.md: https://arxiv.org/abs/2610.42012 ; https://arxiv.org/abs/2611.09656 ; https://arxiv.org/abs/2611.13294 . Integer scalar hashing is **not** a free DLP-free elliptic reduction; reporting constructions over L^m inputs retain their preprocessing charge.
- Established Lambda<=5 reference: https://arxiv.org/abs/2401.17570 (Xylouris bound reported in introduction); **reported but not fully independently validated** Lambda<=3.99: https://hexagonmath.org/users/user_78af3893-4515-4d2b-a21b-f3e8f0649bbc .
- CI exact run URLs: https://github.com/isogeny-labs/crypto/actions/runs/45096187034 ; https://github.com/isogeny-labs/crypto/actions/runs/45097885135 ; https://github.com/isogeny-labs/crypto/actions/runs/45100829948 .

**Claims ledger for this note:** I8-C1 exact group-semiring contraction and reconstruction; I8-C2 uniform empty/yield and rank gate; I8-C3 split-table scoped exponent obstruction; I8-C4 conditional growing-family beta budget; I8-C5 CI source observation. All are author-written, unverified research claims [pcc:unverified], **not** signed PCC certificates, certified novel results, or measured ECDLP gains.