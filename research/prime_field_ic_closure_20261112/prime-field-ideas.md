# Prime-field ECDLP techniques to try: ranked synthesis

**Standing.** Nothing in this report has been measured. Every claim, number and verdict below is `[pcc:unverified]`. All of it comes from desk reviews, plus one toy script that was not recorded in pcc (`/tmp/claude-0/s.py`). research-kb and the lead atlas were unavailable to every proposer and reviewer, so no novelty claim has been checked against the knowledge base.

## Bottom line

None of the 15 ideas is likely to beat Pollard rho. The reviewers sorted them as follows:

- **Broken:** 8.
- **Known prior art:** 4. One of these, batched normalisation, is already in the code at `/home/user/crypto/src/cryptanalysis/ic_framework/plugins.rs:3324-3343`.
- **Weak:** 3.

None of them changes an exponent. The cited gaps are 5.4–7.3x rho-neg for the k=5 sum table and 5–13x for Table B rows, both `[pcc:unverified]`. A constant-factor lever cannot close a gap that size.

Only one idea has a credible, measurable gain over the current IC baseline. That is the **CM-orbit factor base on j=0/1728 curves**. Expect about 3x on relation-phase cost and about 9x on linear algebra against the folded incumbent. Against a matched rho that also uses the order-6 automorphism, the net gain is about √3. It is known technique (Faugère–Gaudry–Huot–Renault 2014), so it would count as a verified IC-baseline improvement, not a novel result.

Several reviews point to the same structural gap. Prime fields have no algebraic factor-base condition that adds equations without adding unknowns (the "equation budget"). Only an m≥4 decomposition oracle with such a condition leaves room for an exponent change in the cost model. None of the ideas here addresses that gap.

## Ranked list

| Rank | Idea | Verdict | Best-case gain over IC baseline | Distance to matched rho after the gain |
|---|---|---|---|---|
| 1 | CM-orbit (automorphism-closed) factor base, j=0/1728 | known | ~3x relation phase, ~9x LA vs folded incumbent | net ~√3 vs automorphism-rho; gap stays ~3–8x |
| 2 | Paired x(R+T), x(R−T) from one shared inversion | known | ≤~1.5x on candidate generation only | unchanged if rho gets the same trick |
| 3 | Isogeny-class shopping (rescoped to incumbent oracle) | weak | expected null, <5% | unchanged |
| 4 | Legendre-symbol pre-filter before normalisation | weak | expected null; batch inversion dominates | unchanged |
| 5 | DP = decomposable point, rho/IC dual exit | weak | none; cost ≥ rho·(1+ε) | ≥ rho by construction |
| 6 | Möbius-chart base tuned for fall degree | known | null by degree invariance | unchanged |
| 7 | BKK / sparse-resultant presentation choice | broken | p-independent constant on a non-bottleneck phase | unchanged |
| 8–15 | Remaining ideas | broken / known | none | — |

## Ideas worth a run or an analytic closure

### 1. CM-orbit factor base (j=0/1728) `[pcc:unverified]`

- **Mechanism.** Build the factor base as a union of orbits of the order-6 automorphism (order 4 for j=1728). Since log(αP) = λ·log P, each orbit needs only one unknown. For j=0 this is the cube base {P : x(P)³ ∈ S}, with one unknown per orbit. It generalises negation folding.
- **Red-team verdict: known.** Prior art is Faugère–Gaudry–Huot–Renault (J. Cryptology 2014) and Galbraith–Gebregiyorgis (2014). The proposal's k^{m−1} claim is wrong in two ways:
  - The "free relations" are α-multiples of the same relation and add no rank.
  - KN-FIND-007 fixes yield per point.

  The real effect is k/2 fewer unknowns than the folded base. That gives ~3x on relations needed and ~9x on LA. Rho-neg gains √3 from the same automorphism (Duursma–Gaudry–Morain), so the net is ~√3. The exponent does not change. The idea does not apply to Curve25519.
- **Cheapest test.** Reuse the cube-base oracle with one unknown per order-6 orbit, at m=2 and 20 bits, on secp256k1-shaped j=0 curves. Compare against the folded incumbent and against rho using the order-6 automorphism, not plain rho-neg. Price both across w ∈ {0, 0.1, 1} and include an A/A arm.
- **Falsifier.** The total GAE gain over the folded incumbent is less than √3, measured against rho with the same automorphism, or the paired CI includes 1 at any band weight.
- **Why it ranks first.** It is the only lever with a mechanism that survives review. It acts on the relation count rather than the per-operation cost, so it is not an accounting change, and it can be measured in the existing harness.

### 2. Paired-translate candidates, x(R±T) from one inversion `[pcc:unverified]`

- **Mechanism.** One inversion of (x_R − x_T) gives both λ± and so both x(R+T) and x(R−T). That yields two relation candidates per inversion.
- **Red-team verdict: known.** It is the differential-addition and Montgomery-ladder identity, and simultaneous ±T is standard in negation rho and in BSGS variants. Under Montgomery batching it costs ~3.5–4M per candidate against ~6M, so at most ~1.5x, and only on the candidate-generation phase. T_dec is table probes, not generation.
  - A walk with a fixed T makes R−T a duplicate candidate.
  - If the matched rho-neg arm gets the same shared-inversion trick, the IC/rho ratio does not move. That makes it a probable relabelling.
- **Cheapest test.** Profile the k=5 incumbent at 20–32 bits and get the phase split between candidate generation and the decomposition test. Drop the idea if generation is under ~30%. Otherwise use a translate set R ± T_j with batched inversion, give rho-neg the same batching, and report counted field mults per phase.
- **Falsifier.** The counted-GAE drop per candidate is under 1.2x once y-recovery and the dependence check are included, or more than 5% of paired relations are dependent, or the gain disappears once rho is batched the same way.

### 3. Isogeny-class shopping, rescoped `[pcc:unverified]`

- **Mechanism.** Move the DLP by a Vélu isogeny to the curve in its 2- and 3-isogeny class with the cheapest decomposition, ideally an a=−3 or a=0 model.
- **Red-team verdict: weak.**
  - The proposal measures the S3 root-finder, which is not the incumbent. The m=2 and k=5 oracles evaluate no a,b-dependent polynomial in the hot loop.
  - a=0 needs j=0, which a generic class essentially never contains.
  - Model selection by isogeny is prior art (Brier–Joye 2003).
  - Any arithmetic saving also goes to the matched rho on the same curve.
- **Cheapest test.** Do a symbolic count first: are the coefficient-dependent ops more than 5% of T_dec in the folded pair-table or k=5 oracle? If not, which is expected, close the lead without a run. If so, measure relations per GAE on about 5 class members against an A/A arm, with rho priced on the same curves.
- **Falsifier.** The IC/rho ratio spread across the class sits inside the A/A band.

### 4. Legendre-symbol pre-filter before affine normalisation `[pcc:unverified]`

- **Mechanism.** Compute χ(X·Z) on projective candidates to reject about half the non-members before normalising and probing.
- **Red-team verdict: weak.**
  - A Jacobi symbol costs O(log p), the same order as the inversion it avoids.
  - Batch inversion already normalises at about 3M per candidate, so the filter loses to it.
  - The 2-bit variant needs the affine x anyway.
  - The oracles may have no projective step to skip.
- **Cheapest test.** First confirm that `ic_framework` has any per-candidate inversion; `plugins.rs` suggests it does not. If it does, microbenchmark batch inversion against Jacobi-then-invert-survivors at a 30-bit p.
- **Falsifier.** The Jacobi-to-inversion cost ratio is at least 0.8, or ΔS sits inside the A/A floor.

### 5. DP-as-decomposable-point dual exit `[pcc:unverified]`

- **Red-team verdict: weak.** The cost is at least min(rho, IC)·(1+ε). With IC at about 6x rho, the walk almost always exits through rho with overhead. The overhead comes from three places:
  - Every step needs an affine x.
  - Every step needs a table probe, priced at w>0.
  - The DP density is forced to ~p^{−1/3}, which delays detection.

  It cannot even improve on the IC baseline.
- **Cheapest test.** Analytic. Write the inequality into the lead note and close the lead. Skip the 200-instance toy.
- **Falsifier.** Mean GAE is at least the matched rho-neg figure at every w. This is expected.

### 6. Möbius-chart base for fall degree `[pcc:unverified]`

- **Red-team verdict: known.** It is the degree-1 case of the rational-map-image bases of Petit–Kosters–Messeng (PKC 2016). A PGL2 substitution keeps the multidegree 2^{m−2}, so the first-fall degree and the S3 root-find cost do not depend on the chart.
- **Cheapest test.** A one-off sanity check rather than the 20-chart sweep: substitute two random charts into S3 at 16 bits, confirm the multidegree and the msolve first-fall degree are identical, and record the null.
- **Falsifier.** The cross-chart variance is at or below the A/A spread.

### 7. BKK / sparse-resultant presentation choice `[pcc:unverified]`

- **Red-team verdict: broken as an exponent lead.** The root count over the algebraic closure is the fibre degree D(m) whatever the presentation, so BKK ≥ D(m). Over F_p the solve costs poly(D(m))·polylog p. The real obstruction is that the factor-base condition is not algebraic. That leaves e unchanged, and the method only touches m≥4, where no oracle exists yet.
- **Cheapest test.** A one-line analytic check: compute D(m) and the Semaev Bezout bound for m=2..5. Their ratio bounds the saving of every presentation and does not depend on p.
- **Falsifier.** The ratio does not grow with m, which is expected.

## Broken or already known, in brief

All entries below are `[pcc:unverified]`.

- **Doubling-fibred base with an x([2]P) pre-filter.** Broken.
  - The filter order is backwards: the free x-range test already rejects first.
  - Matching |B| forces X0 to grow, which pays back the claimed |X1|/p gain in priced doublings.
  - x([2]P) is a function of x(P), so it adds no hash information.
- **Isogeny pushforward with a kernel-closed base.** Broken.
  - The cofactor map collapses P + ker φ to {hP}, so the l^m "variants" are one relation.
  - If the kernel points are not rational, the base cannot be built at all.
  - Otherwise it reduces to IC on E′ with base |B|/l, which is isogeny transport plus a base-size relabel.
- **Guessing slopes λ_i.** Broken. A toy at p=65521 with B=120 gave 6,776 distinct slopes over 7,140 pairs (0.949), above the 0.9 kill threshold. Guessing a slope from a factor-base pair is the pair/k-sum table under another name, so it is bounded by Shoup and Corrigan-Gibbs–Kogan. Slope-as-unknown formulations are prior art (Semaev 2015; Karabina; Amadori–Pintore–Sala).
- **Resultant against V(X) with subproduct trees.** Broken.
  - The baseline is B² quadratic solves plus a hash lookup, not general root-finds.
  - The resultant replaces an O(1) membership test with Õ(B).
  - The claimed Õ(Bk+B²) is below the size of the enumerated space.
  - The technique is textbook: Semaev resultants plus fast multipoint evaluation.
- **Batched x^p over a product tree.** Broken. M(kd) ≥ k·M(d) for every multiplication algorithm, so the shared powering costs at least as much as the unbatched version. Nothing is shared except the exponent.
- **Richelot-glued (2,2)-split Jacobian.** Broken.
  - The Mumford coordinates add as many unknowns as equations.
  - The split-u base is the standard Gaudry base, cost p^{2−2/g} = p.
  - Projecting to E × E′ makes it a relabelled E decomposition.
  - Consistent with chained_prime_transfer_20261111 (9.8–42.9x rho).
- **Early abort of hopeless targets.** Broken. Each walked target is one deterministic O(1) lookup, the pipeline already re-randomises every step, and there is no per-target yield state to abort on. Pomerance-style early abort needs a multi-stage attempt cost.
- **Batched affine normalisation (Montgomery's trick).** Known and already implemented: `decompose()` calls `f.batch_inv` once per sweep at `/home/user/crypto/src/cryptanalysis/ic_framework/plugins.rs:3324-3343`. The k=5 sum-table builder from crypto#2852 is the only site not yet checked.

## Recommended next actions

1. **CM-orbit base on j=0 curves.** Run it as a frozen paired ecbench spec at 20 bits. Include an A/A arm, an automorphism-matched rho reference and §5a band pricing. Report it as a known constant-factor improvement over the IC baseline. The remaining distance to rho is the measured IC/rho ratio after the gain, expected around 3–8x.
2. **Paired-translate.** Profile the k=5 incumbent's phase split first. Pursue the idea only if generation is at least 30% of cost, and only with rho batched symmetrically.
3. **Analytic closures, no runs.** Record closure notes for ideas 3–7 and the broken list, each with the falsifying argument above and the prior-art citations. The citations come from reviewer memory and are `[pcc:unverified]` until checked in research-kb.
4. **Exponent-level work.** Put research effort into the open gap: a prime-field factor-base condition that is algebraic and adds equations, as needed for an m≥4 oracle with T_dec ≪ p^{1/4}. None of the fifteen proposals targets it.