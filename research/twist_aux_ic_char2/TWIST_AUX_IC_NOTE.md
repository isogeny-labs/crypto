# Twists as auxiliary curves for ECDLP index calculus: audit, and lift-dense lines in odd characteristic

2026-11-05. Exact toy computations in PARI/GP 2.17.3, plus proofs. No ECDLP was solved and
no end-to-end cost was measured. The scripts and logs are in this folder
(`common.gp`, `exp1_n2.gp` … `exp5_generic_pair.gp`, `exp6_singular.gp`, `exp7_distinct.gp`, `results/*.log`).

## 0. Summary

1. **The prior programme has one live lever left.** That programme ran from Oct 14 to
   Nov 1 in `isogeny-labs/crypto` (`research/twist_*`, `quadratic_twist_*`, `lift_dense_*`,
   `c1_*`, `twist_relations_tr04_*`).
   - Everything aimed at prime-field targets is closed within the scope of the ledger's
     bounded-degree obstruction (KN-OPEN-020) and the one-line Frobenius twist lemma.
   - The live lever is the **lift-dense ("twisted all-lift line") factor base** of Oct 27.
     It was built for characteristic 2 with N composite. It gives up to 2^n more relations
     per decomposition trial, which is a constant factor, and its net cost exists only as a
     cost model.
   - **Update (Exp 12): the odd-characteristic analogue, case P, is closed at n = 4.**
     Its yield edge is a 2-descent concentration. Ordinary class-pinned lines get the
     same concentration under the same hypothesis, at the same per-trial solver cost.
     Against them case P wins only 4× with a whole-line factor base, and 1.02× in a
     balanced (double large prime) attack at q = 2⁶⁴.
   - **The char-2 original survives, but shrinks (Exp 13).** Here E/2E = Z/2 and a
     point's class is Tr(x) + Tr(a). So on u·F_Q the class is constant exactly when
     T(u) = 0, and lifting is decided by T(√b/u).
     - The lift-dense line has both T(u) = 0 and T(√b/u) = 0. It is therefore already
       class-pinned, with one class of Q − 1 x-values.
     - The best non-lift-dense line has exactly one of the two traces zero. It is
       pinned with (Q − 2)/2 x-values, half as many.
     - Gain against pinned lines: 8× with a whole-line factor base (q ≲ 2²⁰), and
       1.4–1.5× balanced at Q = 2⁶⁴ (analytic √2, i.e. 0.5 bit). Against random
       lines: 16× and 2×.
     - **Equal per-trial cost: verified for Singular (Exp 14, 14b).** The degrevlex GB
       has equal median cost across line types, with a mean up to 1.33× higher on D from
       degree-deficient systems. The change of order (Singular `stdfglm`, 32 systems)
       costs the same on every type: D/others = 0.98 in CPU time. Under Singular the
       change of order is about 1400× the GB, so the whole trial costs the same and
       Exp 13's 1.52× (balanced, against pinned, 2⁶⁴) stands as measured. Estimated,
       not measured: if the GB were 30% of the solve (its share under msolve in odd
       char, Exp 11h) and Singular's GB penalty carried over to F4, D would be about
       1.1× dearer, giving 1.44×. Worst case (1.33× on the whole trial): 1.29×. Decomposition counts match
       brute force 72/72.
     - **Squaring reduction and affine lines (Exp 15, 16).** Every char-2 system on a
       line through 0 has each solution counted 2^{m−1} times; substituting squares
       cuts the degree 8× at m = 4 on every line type. The evenness behind it is stated
       in FHJRV 2014; the proof via dx/x and the descent factor 2^{dim U} are worked out
       here (16b). **Twist dependence:** a lift-dense line through 0 exists only when
       Tr(a) = 0, i.e. on exactly one curve of each quadratic-twist pair. Both curves
       have affine lines in √b·F_{Q²} with all Q x-values lifting into one class, but
       those get only the 4× reduction (16c). Their trial costs about 7.7× a D or pinned
       trial (Singular stdfglm, same process, two curves; break-even is 2×), so pinned
       remains the baseline and the 1.29–1.52× bracket stands.
       D's ~32 doubled points cost nothing for an F_Q-root solver (16a).
2. **Classification theorem (new): which F_q-lines are all-lift or all-twist in odd
   characteristic.** Take q odd and above the Weil threshold, K = F_{q^n}, and
   f_L(w) = c₁⁻³f(c₀+c₁w). Then L is degenerate (all-lift or all-twist) exactly when, for
   every Frobenius orbit O of roots of f_L, the number n·k_O/|O| is even. Here k_O is the
   number of roots of f_L that lie in O. That condition comes in two shapes:
   - **(D) descent.** n is even and f_L ∈ F_{q^{n/2}}[w]. On these lines about half of the
     lifted points lie in the twist component and are dead columns for the ℓ-DLP. This is
     the old quadratic-twist obstruction, one level up. **Useless.**
   - **(P) conjugate root pair.** n is even and at least 4. Two roots of f satisfy
     (e_j−c₀)/c₁ = ((e_i−c₀)/c₁)^{q^s} with s ≠ n/2. The third root γ has
     v₂(deg_{F_q}γ) < v₂(n). This requires E(K)[2] = (Z/2)². **No descent, and the points
     are useful.**

     The construction is generic. A generic full-2-torsion curve over F_{q⁴} has 6 such
     lines, found by solving an F_q-linear equation, and about half of them are all-lift.
     The count differs when N(B/A^q) = 1; that case is not analysed. Measured:
     - 146 of 146 constructed lines are degenerate as predicted, and only 3 of ≈1,050 points
       on the all-lift lines are dead.
     - Distinct ℓ-columns match the chance-collision expectation (Exp 7), so there is no
       hidden 2-torsion collapse.
     - **n = 6, unsaturated (Exp 10).** Prop 5 generalises: s ∈ {1, 2}, γ ∈ F_{q³} \ F_q.
       Exp 10c covers 328 constructed lines on 28 curves at q = 11 and 13, both values of
       s. Every line follows the lift rule. None has f_L descending to F_{q²} or F_{q³},
       and j never lies in either subfield. Lemma 9 never fails. 6-term coverage of the targets is 0.18 (q = 11) and 0.17
       (q = 13), and matches the null drawn from H to within 0.995–0.999, so the n = 4
       shortfall does not appear here. Against the **median** random line, case P needs
       **14× (q = 11) and 31× (q = 13) fewer trials per relation**, rising toward the
       model's ≈64× limit. Against the **best of 12–24 random lines**, it is only 3.8×:
       at q ≤ 13 the Weil error term (≈(3n−1)√q) exceeds q, so some random lines lift
       almost fully. **Regime.** P/null ≈ 1, so coverage is fixed by F alone. Against a
       line found by searching k random lines (F_best ≈ q + √(2q ln k)), the gain per
       relation is about 2·(2q/F_best)^{m−1}. That approaches 2^m only when
       q ≫ (3n−1)². In the cryptographic regime, searching for lucky lines is hopeless,
       so case P is the only route to F = 2q. At sizes we can enumerate, a search
       recovers most of the advantage. The 14×/31× and 3.8× figures are small-q
       measurements of this formula, not separate effects.
     - **Equivalent view.** With c₁ square, the case-P factor base on E is exactly the
       standard subfield factor base {X ∈ F_q} on the K-isomorphic model Y² = f_L(X),
       whose cubic has two Frobenius-conjugate roots.
     - **Per-trial cost measured (Exp 11, n = m = 4, q = 11, msolve, 12 P + 12 R systems).**
       **Not yet settled for cryptographic q.** Gaudry–Diem descent of S₅, symmetrised
       to σ₁..σ₄, with field equations σᵢ¹¹ − σᵢ: the Gröbner basis takes 1.99 s
       (median) on case-P lines and 1.94 s on random lines. Decompositions read off the
       basis match brute force on 24/24 systems (P 21 = 21, R 3 = 3). Without field
       equations the ideal degree is ≤ 4096 for both. Case-P ideals are not in generic
       position (eliminant degree 4074 < 4096, non-squarefree) and carry 7–20
       F_q-rational σ-points against 0–3 for R. Most of those σ give non-split quartics.
       msolve's *default* solve path (rational parametrisation) therefore stalls on
       P: 10/12 time out at 600 s, against 2.4 s for R. That stall is in msolve's
       solving stage, not in F4. But field equations have degree q, so they are only
       usable at toy q. The cryptographic route is grevlex GB → parametrisation/FGLM →
       eliminant roots, and that is exactly where case P stalls.
     - **Default path at larger q (Exp 11d, n = 4, q = 31 and 43, 6 P + 6 R each, no field
       equations).** F4 cost is still equal: GB median 2.0 s against 1.7 s at q = 31, and
       1.5 s against 1.4 s at q = 43. Case P was in generic position on 1/12 systems,
       random lines on 12/12. Every case-P system that retried had eliminant degree below
       the ideal degree, i.e. the linear form did not separate. Only 5/12 case-P systems
       are provably non-radical (squarefree part of the eliminant smaller than the
       eliminant). One of those, q = 31 `c1_P_1`, was generic and fast (7.8 s). So the
       retries track separation failure, not non-radicality. This is suggestive, not
       proved.
       - **Solve time.** msolve retries linear forms, or adds a variable A, until one
         works.
         - q = 31: case P finished 3/6, in 8, 108 and 153 s. The other 3, on one curve
           with GB degree 4080–4088, timed out at 900 s after 61–70 retries. Random lines
           took 5–7 s.
         - q = 43: case P finished 6/6, median 35 s (15–153 s), against a median of 5.4 s
           for random lines.
       - **Cost model.** Time ≈ (retries + 1) × one attempt, and a case-P attempt costs
         about 1.5–2.5× a random-line attempt. Two curves per q cannot show a trend in q.
         The 61–70 retries all come from q = 31 curve 2, where q = 31 curve 1 had 0–10
         and q = 43 had 1–15. What does predict a timeout, at q = 11 and q = 31, is a GB
         degree below 4096 (4080–4088).
       - **Correctness.** Wherever it finished, recovery matches ground truth: P 1 = 1 and
         4 = 4, R 3 = 3 and 1 = 1, with 0 bad points. This needed a parser fix, because
         msolve permutes the variables or adds A.
       - **Cost at q = 43.** About 37 s per case-P trial against 7 s per random-line
         trial, a factor of about 5 *with msolve's default solver*.
       - **Trial gain at the same q (Exp 11e, 3 curves, 150 targets per line).** Case-P
         lines (F = 43) give 1.43 decompositions per target, close to the saturation
         density 2^{m+1}/m! = 1.33. They hit at least one decomposition on 74% of
         targets. Random lines (F = 20–27, 9 lines) give 0.083 per target, with a hit
         rate of 6.4%. That is 17× more relations per trial, or 11.5× by hit rate. The
         gain grew from the 4–6× seen at q ≤ 17, as the regime formula predicts.
       - **Net at q = 43, default solver: about 17 / 5 ≈ 3.4× fewer seconds per
         relation.** The gain survives. Index calculus needs about #FB relations, and the
         case-P base is about 1.9× larger (43 against 23), so the whole-attack
         relation-collection advantage over a *random* line is about 1.8×. A searched
         line would narrow this, as in Exp 10b.
       - **Block elimination (`msolve -e 3`) is not an alternative route.** It would
         avoid generic position, but it ran over 600 s on a single random-line system,
         against 5 s on the default path.
       - **Cause of the overhead: separation over a small field (Exp 11g, supported, not
         proved).**
         - **The mechanism.** A linear form over F_q fails on a pair of F_q-rational
           solutions with probability 1/q. Case-P systems carry 4–20 F_q-rational
           σ-points; random lines carry 0–4.
         - **The data.** At q = 31 and 43, every case-P system where under 3% of the
           F_q-linear forms separate its rational points timed out: 3/3, with 61–70
           retries. Systems where 20–87% separate finished, with 1–15 retries.
         - **Two exceptions.** On two q = 43 systems s₄ separates the rational points,
           yet msolve still retried (5 and 1 times). So points over small extension
           fields probably contribute as well.
         - **Same cost when generic.** The one case-P system that was generic on the
           first try (q = 31 `c1_P_1`) took 7.8 s, against 7.2 s for random lines on
           the same curve.
         - **msolve's knobs don't help.** `-d 3` and `-d 4` (more normal forms) and
           `-c 1` all took longer on the slowest q = 43 system (189–237 s against 153 s),
           with the same 14–15 retries.
         - **No structural cause found.** S₅ restricted to random lines in (w₃, w₄)
           shows no case-P-specific factorisation (Exp 11f).
         - **Consequence.** The number of rational σ-points stayed near 10 from q = 11 to
           q = 43, so the fraction of failing forms is about pairs/q. That tends to 0 at
           cryptographic q, and so should the overhead, leaving per-trial parity. At toy
           q, choosing the separating form over an extension field would avoid it; msolve
           doesn't offer that.
       - **Confirmed at q = 1009 (Exp 11h, n = 4, 6 P + 6 R, default path).** Case P is
         generic on 6/6 systems with 0 retries, even though each still carries 11–18
         F_q-rational σ-points. Solve time is 6.9 s median for both P and R (P 6.3–7.3 s,
         R 6.5–7.4 s), and GB time is 2.0 s against 1.9 s. Decompositions match the
         meet-in-the-middle ground truth on 12/12 systems (P 7 = 7, R 0 = 0), with 0 bad
         points. **So the case-P overhead was a small-field separation artefact. Per-trial
         cost is equal on the cryptographic route, and the full trial gain stands.**
     - **Gain against searched lines at q = 1009 (Exp 11i, n = m = 4, 3 curves, 2000 random
       lines per curve).**
       - **Method.** Yield is exact rather than sampled: the mean number of
         decompositions per random target in 2E is 64·N₀/#E. Here N₀ counts the
         4-multisets of factor-base x-values whose 2-descent classes sum to 0; this uses
         #E/2E = 4 and the fact that a class sum of 0 gives 8 sums, up to sign, in 2E.
         Against brute force on the same lines at q = 43 the ratio was 1.015 for P and
         1.09 for R, i.e. within noise.
       - **Yield per target.** Case P: 1.341 (saturation density) with F = q = 1009
         x-values. Median random line: 0.042 with F ≈ 505 ≈ q/2. Best of 2000:
         0.062–0.063 with F = 556–559, i.e. q/2 + 3.2·√q/2.
       - **Per-trial yield ratio.** 32× against a median line and 21–22× against the
         best of 2000. This equals 2·(q/F_x)⁴: the multiset count grows as F⁴, and
         Lemma 9 contributes the 2.
       - **Trials per full relation set, ∝ F/yield.** 16× against a median line and
         11.7–11.9× against the best of 2000. The regime formula 2·(2q/F_pts)^{m−1}
         predicts 11.9. The limit as q → ∞ is 2^m = 16: the search gains only
         O(√(q log k)) in F.
       - **Net, with equal per-trial cost (Exp 11h).** Relation collection is about 12×
         cheaper than with the best of 2000 searched lines at q = 1009, and about 16×
         asymptotically. **Superseded by Exp 12:** a class-pinned line, which is not
         case P, cuts this to 4×.
       - **Caveat.** The case-P factor base is twice as large (q against ≈ q/2), so
         sparse linear algebra costs about 4× more. At n = 4 relation collection
         dominates, since each trial is a 7 s solve against about q² for linear algebra,
         but in a balanced (double large prime) attack this would eat part of the
         gain. *(Exp 12 below costs this.)*
     - **Balanced cost model, linear algebra included, against the strongest non-case-P
       line (Exp 12, n = m = 4).** **Exp 11i compared case P with the wrong baseline.**
       - **The comparator: class-pinned lines.** Take any α ≠ β in F_{q²}, and set
         c₁ = (e₂−e₁)/(α−β) and c₀ = e₁ + c₁α. Then x−e₁ = c₁(w+α) and x−e₂ = c₁(w+β).
         For q odd, F_{q²}* lies in the squares of F_{q⁴}. So every x-value on the
         line has the *same* 2-descent class.
         - The third root is generic, so the line is half-lift and not case D.
         - These lines need exactly what case P needs, E(K)[2] = (Z/2)², and there are
           about q⁴ of them.
         - Measured at q = 1009 (12 lines, 3 curves): one class, 483–525 x-values. The
           whole-line yield is 0.14–0.20, against the predicted 64·C(q/2,4)/q⁴ ≈ 1/6.
           Brute force at q = 43 gives 0.16, 0.23 and 0.23, against predicted 0.20,
           0.24 and 0.17.
         - msolve cost per trial is the same as for case P: 7.1 s against 6.9 s at
           q ≈ 2²⁰, and 14.8 s against 15.0 s at q = 1009 in one clean run. The pinned
           systems were generic with 0 retries.
       - **Best subset for any line.** Four x-values of one class always sum into 2E. So a
         factor base B of S x-values inside one class has yield 64·C(S,4)/#E on every
         line type. At equal S, case P has no per-trial edge over a one-class subset of
         a random line, and none over a pinned line. Brute force at q = 43 agrees within
         finite-size error: measured 0.26 and 0.375 against predicted 0.33 and 0.445,
         the gap being repeated-point multisets at S ≈ 25.
       - **What separates the line types.** Only the class pools: how many x-values
         share B's class, which feeds large primes, and how many pairs of equal-class
         x-values exist, which feeds double large primes.
         - Case P (Lemma 9): two classes of about q/2 each, 476–533 at q = 1009.
         - Pinned line: one class of about q/2.
         - Random line: four classes of about q/8 (median largest class 134–137).
       - **Cost model.** Cost = N·T + LA(S).
         - T is the trial cost. It is flat in q (above) and equal across line types:
           at q ≈ 2²⁰, in one clean run, case P took 8.3 s and the random line 7.5 s. An
           earlier 2× random/P gap at 2²⁰ was CPU contention and has been discarded.
         - LA(S) = 2S²·(8·t_add + 3·t_mul), Lanczos/Wiedemann with row weight 8. The unit
           costs are measured with GMP mpn: at 256 bits, t_add = 6 ns and t_mul = 144 ns.
         - Each line type uses the cheapest of four strategies:
           - the whole line;
           - a single-class B with no large primes;
           - a single-class B with one large prime;
           - a single-class B with two large primes.
         - Relations are counted exactly by multiset. They are combined by cycle rank on
           a simulated large-prime graph (union–find), and that count matches
           random-graph theory to 4 digits (Exp 12d).
         - Every per-trial rate depends only on σ = S/q, so cost/q depends only on
           κq/T. A q = 2¹⁸–2²⁰ simulation therefore stands in for any q; the two sizes
           agree to 0.5%.
       - **Result: case-P gain at equal per-trial cost.**

         | q | 2¹⁶ | 2²⁰ | 2²⁴ | 2²⁸ | 2³² | 2⁴⁰ | 2⁴⁸ | 2⁵⁶ | 2⁶⁴ |
         |---|---|---|---|---|---|---|---|---|---|
         | against a random line | 15.9 | 14.7 | 11.0 | 4.6 | 3.4 | 2.19 | 2.09 | 2.07 | 2.05 |
         | **against a pinned line** | **3.99** | **3.69** | **2.94** | **1.50** | **1.20** | **1.07** | **1.04** | **1.02** | **1.02** |

         - Up to q ≈ 2²⁰ the whole line is optimal for every type. Trials per relation
           set are then (q/2)/(1/6) = 3q for a pinned line and q/(4/3) = 0.75q for case P.
           That is a gain of 4, not 2^m = 16.
         - Above about 2²⁴, double large primes take over.
         - Above 2⁴⁰ every type runs with S ≪ q, and linear algebra takes about half
           the cost.
       - **Why the edge goes to 1 (against pinned) or 2 (against random).** For S ≪ q,
         the double-large-prime graph of one class only has to reach its percolation
         threshold. That takes about q/(32σ²ℓ) trials, where ℓ is the largest class
         pool as a fraction of q. Balancing against κσ²q² gives a total cost of
         2q·√(Tκq/(32ℓ)). So the gain is √(ℓ_P/ℓ_other): √((1/2)/(1/8)) = 2 against a
         random line, and √((1/2)/(1/2)) = 1 against a pinned line. Case P's second
         class only adds lower-order cycles. The optimal σ ratio against a random line
         is predicted at 4^{1/4} = 1.41 and simulated at 1.49.
       - **Scope.**
         - Line search only shifts pool sizes by O(√q).
         - T was measured only to q ≈ 2²⁰, since the q = 2³¹ systems failed to generate.
         - The asymptotes do not depend on the unit costs (T, t_mul); those only move
           the crossover.
         - Not modelled: structured Gaussian elimination, three or more large primes,
           and the larger weight of long cycle-combined relations. All three change κ
           about equally for every line type.
     - **Yield now measured (Exp 8, n = m = 4, q ∈ {11, 13, 17}).** Every case-P point
       lies in one index-2 subgroup H ⊃ 2E(K) (Lemma 9), so 4-term sums cover 71–76% of
       2E(K), which contains every odd-order target. Random lines cover 6–9%. Per useful
       relation, that is 4.0×, 6.0× and 5.6× fewer decomposition trials at q = 11, 13, 17.
       Against a random set of the same size drawn from H, coverage is 0.96–0.97 of the
       null in every run (0.973, 0.960, 0.957, 0.958). That shortfall of about 4% always
       has the same sign and is **not explained**. Exp 9 rules out excess 4-term
       coincidences as the cause.
       The metric is coverage of the odd-order part. 2E(K) is a valid proxy only when
       v₂(#E) = 2. Of the two low outliers, one is a 2-adic artefact: v₂ = 6, the base sits
       in an index-8 subgroup, and odd-part coverage is 0.94. The other has an unusually high
       count of 4-term coincidences P₁ − P₂ = P₃ − P₄. Exp 8h suggested case P carries 1.7×
       the null rate, but the larger Exp 9 (157 case-P lines) gives 1.06× and 1.14× its
       class-matched null, so the 8h figure was noise. That line is the low tail of
       ordinary chance coincidences, not a case-P effect.
   - **Side finding on the repo's char-2 lead.** A valid u ∈ b^{1/4}·F_Q (which exists
     when T(b^{1/4}) = 0) makes the line invariant under translation by T = (0, √b), whose
     x-action is x ↦ √b/x. Columns then pair up: Exp 7 measures 90 distinct out of 186.
     Such lines give about 2× useful yield, not 2^n. A u-count used for cost should exclude
     them. Random valid u are unaffected (178 vs 181 expected).
3. **Consequences.**
   - Odd n: no lift-dense line exists. This covers F_{p^5} designs such as EcGFp5 and
     EcMasFp5.
   - n = 2, which includes every GLS curve: only the useless descent kind exists.
   - n ≥ 4 even: the 2^m lever exists if and only if E has full K-rational 2-torsion. This
     extends the char-2 result into odd characteristic. It is still a constant factor.
4. **Unifying view (interpretation, not a theorem).** The repo's char-2 lines and family (P) share one mechanism. The
   function that decides lifting, restricted to L, is a Galois coboundary:
   - char 2: u = z^Q + z, so T(u) = 0, which is additive Hilbert 90;
   - odd characteristic: (w−α)(w−α^q) = h·h^σ, so its norm is a square, which is
     multiplicative.

   Descent is the trivial coboundary (N_{K/K′}(f_L) = f_L²), and the one that only
   reshuffles the twist.

**Novelty.** A research-kb search for the (P) family (conjugate-root lines, norm square,
full 2-torsion lift-dense) found nothing in the ledger. External novelty has not been
checked.

**Correction log.** My first draft claimed that (D) was the only degenerate kind in odd
characteristic, and therefore that the lever did not exist there at all. That was false.
The proof treated each Frobenius orbit as holding a single root. The review pointed out the
conjugate-pair counterexample, and Exps 4–5 confirmed it. Nothing below relies on the
withdrawn claim.

## 1. Where the earlier work left off, read critically

| strand (dir, date) | what it claimed | assessment |
|---|---|---|
| `twist_index_calculus_20261020`, `twist_obstruction_20261026`, `twist_ecdlp_20261027` E0/E1/G3 | For a target R ∈ E(F_q), the twist part B of any twist-assisted decomposition satisfies 2B = O. | **Correct.** The proof is one line, π(B) = −B. Over 10⁸ sums and 5·10⁵ targets were spent re-confirming it, which adds little. It was already in the ledger as IDEA-20260906-476f20 and IDEA-20261004-31991f. |
| `MIXED_REPORT` (trace descent from F_{p²}) | Descent is valid; the traced factor base still needs to be measured. | The descent claim is correct. But for a prime-field target, any positive-dimensional algebraic factor base maps to Θ(p) columns, so the outcome was fixed in advance (KN-OPEN-020 scope). |
| E2, E4, G2 | Each reports a cost ratio ≥ 1. | These are **accounting identities**: (N0+N1)/N0 and 1 + W_T/W_D. The repo's own verifiers say so. They are not evidence. |
| E3 (F_{p²} eigen-route) | 53–2812× dearer. | The target design (B uniform) is not an ECDLP target. For real targets B = O, so the route is vacuous. |
| E5/G1 (Kummer x-only walk) | No gain over negation-map rho. | Not index calculus. This is a constant-factor walk comparison, and its direction is robust. |
| E6 (genus-2 gluing) | Construction verified. | No index calculus was run. It falls under the same prime-target column obstruction. |
| quartic twist24 (target on a twist over F_{p⁴}) | Order-24 folding; 33.8× dense-row gain. | Extension-field target. The folding gain is a constant factor. The dense-row gain is an implementation effect on an exact oracle, not a twist effect. The repo's own discriminator gives Θ(r^{2/3}) table cost, which is worse than rho's r^{1/2}. |
| TR-04 (Oct 29) | — | Never executed. It is self-declared ≤ √r and cannot beat rho. |
| `p2_lift_defect`, `p2_halving_carry` (Nov 1) | — | p-adic lifts, not twists. Both are scoped nulls. |
| **`lift_dense_factor_base_20261027`** + C1 + Oakley/c2pnb | Char 2, N = d·n: the line u·F_{2^d} with T(u) = T(√b/u) = 0 and Tr(a) = 0 is all-lift, giving 2^n yield. Prime N: impossible. | **The live lead.** The yield is real, and the per-trial cost was unchanged for C1's simplified solver. The gain is exactly what doubling the usable factor base gives. The net cost (+0.5 bits with 2LP, −2 bits without large primes) is a model. **It never asked about odd characteristic, or whether the extra points are useful columns.** Sections 2–5 answer both. |

I recomputed the load-bearing char-2 fact independently (Exp 2c):
- 248 of 248 points on valid-u lines lift at N = 15.
- 23 of 31 sampled curves had no valid u, which matches the repo's corrected u-count.

## 2. Odd characteristic: when is an F_q-line degenerate?

**Setting.**
- q odd, K = F_{q^n}.
- E: y² = f(x), with f monic squarefree cubic in K[x] and roots e₁, e₂, e₃.
- L = c₀ + c₁F_q.
- α_j = (e_j − c₀)/c₁, the roots of f_L.
- g_L(w) = N_{K/F_q}(f(c₀+c₁w)) ∈ F_q[w], of degree 3n.

**Lemma 1 (lift criterion).** Write χ_K = χ_q ∘ N_{K/F_q}. Let r be the number of w ∈ F_q
with g_L(w) = 0. Then

  #{w ∈ F_q : c₀+c₁w lifts to E(K)} = (q − r)/2 + ½Σ_w χ_q(g_L(w)).

**Proposition 2 (Weil).** Suppose g_L is not a constant times a square, and let it have
d ≤ 3n distinct roots. Then |Σ_w χ_q(g_L(w))| ≤ (d−1)√q, so the lift density is within
((3n−1)√q + 3n)/(2(q−3n)) of 1/2. Degenerate lines therefore exist only when g_L is a
constant times a square, provided q − 3n > (3n−1)√q. That means q ≥ 37, 83 and 149 for
n = 2, 3 and 4.

**Proposition 3 (exact criterion).**

  g_L(w) = N(c₁)³ · Π_O m_O(w)^{n·k_O/|O|},

where O runs over the F_q-Frobenius orbits that meet {α_j}, m_O is the minimal polynomial
of that orbit, and k_O = #(O ∩ {α_j}). So g_L is a constant times a square iff every
n·k_O/|O| is even.

*Proof.* g_L = N(c₁)³ Π_{i<n} Π_j (w − α_j^{q^i}). An element β of an orbit O appears as
α_j^{q^i} for exactly n·k_O/|O| pairs (i, j). ∎

When the criterion holds, the line is all-lift if c₁ is a square in K and all-twist
otherwise, because χ_q(N(c₁)³) = χ_K(c₁).

**Corollary 4 (the cases).**
- **n odd.** g_L has degree 3n, which is odd, so it cannot be a constant times a square.
  **No degenerate line exists.**
- **All k_O = 1.** The criterion becomes v₂(|O|) < v₂(n) for every orbit. Then
  Frob_{q^{n/2}} acts on each orbit as a power of Frob_{q^n}, so it permutes {α_j}. That
  gives **f_L ∈ F_{q^{n/2}}[w]**, which is case **(D)**. The converse is immediate.
- **A single orbit holds all three roots (k_O = 3).** For n ≤ 4 every such configuration
  descends. I checked this by enumerating |O| ∈ {1, 2, 3, 6}.
- **k_O = 2 for some orbit (case P).** The exponent 2n/|O| is even iff |O| divides n, that
  is, α_i ∈ K. So the pair α_i, α_j = α_i^{q^s} lies in K, and γ = α_k also lies in K, with
  v₂(deg γ) < v₂(n). That forces **E(K)[2] = (Z/2)²**. If s = n/2, the case always descends:
  deg γ divides n and v₂(deg γ) < v₂(n) together force γ ∈ F_{q^{n/2}}, so it is (D). The
  new cases are those with s ≠ n/2.
- **n = 2.** Every degenerate line is (D), because case (P) forces f_L ∈ F_q[w]. Exp 1
  confirms this exhaustively.

**Proposition 5 (constructing case P).** Fix an ordered pair (i, j), let k be the third
index, and put A = e_i − e_k, B = e_j − e_k. Write c₀ = e_k − c₁γ and z = 1/c₁. Then the
pair condition α_j = α_i^{q^s} becomes the F_q-linear equation

  B·z − A^{q^s}·z^{q^s} = γ^{q^s} − γ.

The linear map on the left is singular iff N_{K/F_q}(B/A^{q^s}) = 1. That happens with
probability about 1/(q−1); I checked it with 0 exceptions in 1,440 pairs. Otherwise each
γ ∈ F_{q^{n/2}} \ F_q gives a unique z.

The equivalences γ ↦ λγ + r (λ, r ∈ F_q) preserve the line, so the family of lines has
dimension n/2 − 2 for each (i, j, s), plus finitely many classes. For n = 4,
F_{q²} \ F_q is a single AGL₁(F_q)-orbit. So a generic full-2-torsion curve has exactly 6
such lines, one per ordered pair with s = 1. That count assumes N(B/A^q) ≠ 1 for every pair.
The singular case, which includes Exp 4's curves with γ ∈ F_q, is not analysed. Each is all-lift or all-twist according to
whether c₁ is a square.

## 3. Usefulness

**Theorem 6 (case D is useless).** Suppose L is all-lift of type (D). Put
K′ = F_{q^{n/2}} and E_L: v² = f_L(w) over K′. Let ℓ > 3 be a prime that divides #E(K)
exactly once and divides only one of #E_L(K′) and #E_L^{tw}(K′). Then:

1. E ≅_K E_L.
2. #E(K) = #E_L(K′) · #E_L^{tw}(K′).
3. A point over w has nonzero ℓ-projection iff χ_{K′}(f_L(w)) matches the side of ℓ.

*Proof.* The points with w ∈ K′ split into the eigenspaces σ = ±1 of Gal(K/K′). ℓ lives in
exactly one of them, so the points in the other eigenspace project to O. ∎

**Corollary 7.** Suppose n is odd, or n = 2, or E(K)[2] ≠ (Z/2)². Then every F_q-line,
and so every F_q-affine subspace, is useful on at most a fraction
1/2 + (3n−1)/(2√q) + O(n/q) of its points. To see this, recurse Theorem 6 down the 2-power
tower; Proposition 2 or Hasse applies at the bottom.

**Case P is useful.** There is no Frobenius symmetry on L: the curve does not descend, and
j ∉ F_{q^{n/2}} was checked. Measured dead fraction: 0 of 212 in Exp 4, and 3 of ≈1,050 in
Exp 5. Those 3 are consistent with points in the cofactor part.

**Lemma 8 (2-torsion collapse).** Full 2-torsion opens a second failure mode that the
dead-point test cannot see. P and P + T have the same ℓ-log. If x ↦ x(P+T) maps L to
itself, every column appears twice, and the yield gain shrinks to about 2.
- In w-coordinates, translation by (e_k, 0) acts as
  w ↦ α_k + (α_k−α_i)(α_k−α_j)/(w−α_k).
- That map preserves P¹(F_q) only if α_k ∈ F_q and the constant lies in F_q.
- Generic case-P lines (Prop 5, γ ∉ F_q) have no root in F_q, so they are safe.
- Exp 4's lines have γ ∈ F_q. There (γ−α)(γ−α^q) = (γ−α)^{q+1} ∉ F_q, because α has
  degree 4, so they are safe too.

Exp 7 confirms this numerically: distinct ℓ-columns on case-P lines are 234, 110 and 130,
against 234.7, 113.0 and 131.0 expected from chance collisions alone. No proof of "no
other hidden obstruction" is offered.

**Lemma 9 (index-2 concentration).** Let L be an all-lift case-P line, with conjugate
roots e_i, e_j (so (e_j−c₀)/c₁ = ((e_i−c₀)/c₁)^q) and third root e_k. Then x − e_k is a
nonzero square in K for every x ∈ L with f(x) ≠ 0. So the 2-descent image of every
factor-base point lies in {δ ∈ E(K)/2E(K) : δ_k = 1}, every sum of factor-base points
lies in the index-2 subgroup H it defines, and 2E(K) ⊂ H.

*Proof.* Write x = c₀ + c₁w and α = (e_i−c₀)/c₁. Then
(x−e_i)(x−e_j) = c₁²(w−α)(w−α^q) = c₁²(w−α)^{1+q}, which is a square because 1+q is even.
Since f(x) is a square (all-lift), x − e_k = f(x)/((x−e_i)(x−e_j)) is a square. ∎

*Consequence.* A factor base spread over all four classes sends about 1/4 of its m-sums
into the target coset 2E(K). A case-P base sends about 1/2. The sum density on the
targets doubles, on top of the ×2^m from lift density. On all of E(K) the coverage looks
about 25% *worse* than a random base of the same size (Exp 8b). That is only because no
sum ever lands outside H; it is not a collapse.

## 4. Characteristic 2 (the repo's construction, recast)

Lifting on y² + xy = x³ + ax² + b is decided by Tr_{K/F₂}(x + a + b/x²). On u·F_Q this is a
Kloosterman sum with parameters (T(u), T(√b/u)), which degenerates iff both vanish. That is
additive Hilbert 90: u = z^Q + z. Exp 2c measures 1 dead point in 248.

**T-invariant sub-case (new; affects the repo's lead).** For T = (0, √b), translation acts
on x as x ↦ √b/x (Exp 7 sanity check: 0 failures in 200). So u·F_Q is T-invariant iff
√b/u² ∈ F_Q, that is, iff u ∈ b^{1/4}·F_Q. Such u are automatically valid when
T(b^{1/4}) = 0, since then T(u) and T(√b/u) are F_Q-multiples of T(b^{1/4}). On those lines
the columns pair up: Exp 7 measures 90 distinct ℓ-columns out of 186. The useful gain there
is about 2×, not 2^n, so a cost model should exclude these u from the u-count. Random valid
u are unaffected (178 vs 180.8 expected).

Degeneracy of affine char-2 lines (c₀ ≠ 0) is not classified; their one-class lifting
pool is (Exp 16c).

**Exp 13: 2-descent classes and the pinned baseline in char 2.**
- **Classes.** E[2](K) = {O, T}, so E/2E = Z/2 and the 2-part of E(K) is cyclic. The
  halving criterion says P ∈ 2E iff Tr(x_P) = Tr(a). I tested it by checking
  (#E/2)·P = O: 0 mismatches over 28 lines at N = 20, 28 and 32.
- **Classes on u·F_Q.** Tr(uw) = tr_{F_Q/F₂}(w·T(u)) and Tr(√b/(uw)) =
  tr(w⁻¹·T(√b/u)). There are four cases:

  | T(u) | T(√b/u) | x-values that lift | classes |
  |---|---|---|---|
  | 0 | 0 | all Q − 1 (lift-dense; needs Tr(a) = 0, else none lift, Exp 16c) | one |
  | 0 | ≠ 0 | exactly (Q − 2)/2 | one, class Tr(a) (pinned) |
  | ≠ 0 | 0 | exactly (Q − 2)/2 | one, inside 2E (pinned) |
  | ≠ 0 | ≠ 0 | about Q/2 | two, about Q/4 each |

  Measured at Q = 32, 128 and 256 (11 curves), these hold exactly. For example at
  Q = 128: D 127/127 in one class; P1 and P2 63/127 in one class; R 53–71 split
  evenly.
- **Yield check.** Brute force at Q = 32 (4 curves, 150 targets per line) against
  32·N₀/#E:
  - D: 1.23 measured against 1.41 predicted. The gap is the same repeated-point
    finite-size effect as at q = 43.
  - Pinned: 0.085 against 0.093.
  - Random: 3–25 hits per run, which is too few to test.
- **Balanced sweep.** I used the Exp 12 model with constant 32 and pools [Q] for D,
  [Q/2] for pinned and [Q/4, Q/4] for random. Results are in
  `results/exp13_sweep_char2.log`.

  | Q | 2¹⁶ | 2²⁰ | 2²⁴ | 2²⁸ | 2³² | 2⁴⁰ | 2⁴⁸ | 2⁶⁴ |
  |---|---|---|---|---|---|---|---|---|
  | D against pinned | 7.97 | 7.36 | 4.69 | 2.34 | 1.70 | 1.57 | 1.50 | 1.52 |
  | D against random | 15.9 | 14.7 | 9.17 | 3.95 | 2.45 | 2.20 | 2.09 | 2.03 |

  - The asymptotes are √(pool ratio): √2 and √4. The extra 0.1 against pinned is
    the coarse S grid (steps of 0.7). At 2⁶⁴ the pinned optimum sits at an LA share of
    0.65 rather than 0.5, which accounts for about 6%. Read the balanced gain as
    1.4–1.5×.
  - **No concentration beyond E/2E (Exp 13b).** With a = 0, the 2E points of every
    line type split about 50/50 between 4E and 2E∖4E: for example D 123–137 of 255,
    pinned 58–73 of 127, at Q = 128 and 256 over 12 curves with v₂ from 2 to 9. So no
    deeper 2-power level favours lift-dense lines.
  - Unlike odd characteristic, the lift-dense line here keeps a genuine 2× pool
    advantage over every other line through 0. In odd characteristic, case P's pool
    equals a pinned line's.
**Exp 14: per-trial solver cost by line type in char 2.** Curve y² + xy = x³ + b over
F_{2^{4d}}, factor base u·F_Q*, S₅ built by resultants of S₃, symmetrised (s₁..s₄), Weil-
descended to 4 equations in 4 unknowns over F_Q. Solver: Singular `std` (degrevlex) through
Sage, not msolve/F4, so ratios may differ under F4. Rational points come from the GB plus
field equations, a route that only works at toy Q and is not timed as representative.
- **Validation.** Split-and-lift decompositions match a brute-force multiset count in
  72 of 72 systems (Q = 32, 64).
- **GB time (s), 3 curves × 3 targets per type (2 × 3 at Q = 128):**

  | Q | D median / mean | P1 | P2 | R | mean D / others |
  |---|---|---|---|---|---|
  | 32 | 0.38 / 0.54 | 0.39 / 0.42 | 0.39 / 0.40 | 0.39 / 0.39 | 1.33 |
  | 64 | 0.92 / 1.11 | 0.77 / 0.80 | 0.87 / 0.88 | 0.84 / 0.86 | 1.32 |
  | 128 | 0.73 / 0.88 | 0.81 / 0.89 | 0.82 / 0.81 | 0.71 / 0.88 | 1.02 |

  - The expected cost is the mean trial time, so the mean is the figure that counts.
  - The excess comes from degree-deficient systems (ideal degree 4064 or 4032 rather
    than 4096), which take 2–3× longer. D had 5 of 24, the others 2 of 72. D had 0 of 6
    at Q = 128, which at the small-Q rate happens about 14% of the time, so it does not
    show the penalty fading.
  - Six of the seven deficient systems are the first target on their curve, and each
    target is shared by all four line types. I have no mechanism for this; it is
    recorded, not explained.
- **Effect on Exp 13.** Re-running the balanced sweep with D trials 1.33× dearer
  (`exp14_sweep_char2_TD1.33`): against pinned 6.00 (2¹⁶), 3.90 (2²⁴), 1.50 (2³²), 1.29
  (2⁶⁴), analytic √(2/1.33) = 1.23. Against random 12.0 → 1.73. So the char-2 gain is
  bracketed at 1.29–1.52× balanced (0.37–0.6 bit).
- **Change of order (Exp 14b).** Singular `stdfglm` to lex at Q = 32, 4 curves × 2
  targets × 4 types (32 systems, four processes in parallel on a loaded machine, so
  CPU time is the figure used):

  | type | FGLM CPU mean (s) | sd | range | GB CPU mean (s) | deficient | lex GB ≠ 4 elements |
  |---|---|---|---|---|---|---|
  | D | 640.1 | 19.5 | 609–662 | 0.57 | 2/8 | 5/8 |
  | P1 | 646.7 | 6.5 | 637–659 | 0.45 | 0/8 | 1/8 |
  | P2 | 654.7 | 9.0 | 642–667 | 0.41 | 0/8 | 1/8 |
  | R | 650.2 | 8.3 | 642–663 | 0.48 | 1/8 | 1/8 |

  - D/others = 0.98. The change of order does not penalise D, and under Singular it is
    about 1400× the GB, so the whole trial costs the same on every type.
  - **No system is in shape position, on any line type (Exp 14c).** In the lex basis
    the s₄-eliminant has degree 512 = 4096/8 on all four types (one target, rerun with
    the lex structure printed), so the projection to s₄ is about 8-to-1 for every line
    type. Every type therefore needs a separating linear form or a different last
    variable before a RUR/shape-lemma solver can be used; this is common to all types,
    not a D cost. Where D differs: its eliminant is not squarefree (multiplicity-1
    part 446 of 508; the scripts' "sqfree-part" is h/gcd(h, h′), which in char 2 drops
    squared factors entirely, so it is the simple part, not the squarefree part; see
    Exp 16a), which points to a non-radical ideal, and its lex bases have more
    elements more often (5 of 8 against 3 of 24). Under Singular neither cost anything.
    I have no mechanism for the factor 8; it is recorded, not explained. Log:
    `exp14c_shape`.
  - Degree-deficient GBs appeared again (D 2/8, R 1/8), this time on both targets, so
    the first-target pattern in Exp 14 was not stable.
  - **Which bracket to use.** Singular's FGLM is slow; with msolve in odd char the GB
    was about 30% of the solve (Exp 11h: GB 2.0 s of 6.9 s). With those proportions
    the 1.33× GB penalty becomes about 1.1× per trial (`exp14_sweep_char2_TD1.10`:
    against pinned 7.25 (2¹⁶), 4.42 (2²⁴), 1.63 (2³²), 1.44 (2⁶⁴); against random 14.5 → 1.93). So the char-2 balanced gain against pinned at 2⁶⁴ is about 1.44×,
    within 1.29–1.52×. This is an estimate: Singular measured equal cost (1.52×), and
    neither the 30% share nor the carry-over of the GB penalty to F4 was tested in char 2. F_Q-rational solution counts (about 10 on D against 1–3) were
    not a cost at Q = 32 in this route; separation failure scales as C(k,2)/Q and is
    expected to vanish at large Q, as in Exp 11h, but that is not measured.

**Exp 15: the factor 8 is a squaring redundancy, and removing it shrinks every char-2
system on a line through 0 8×** (affine lines: 4× or none, Exp 16b). In all four descended equations (two lines checked), s₁, s₂, s₃ occur only
with even exponents; s₄ occurs with any exponent. In characteristic 2 squaring is a
bijection on F̄_Q and on F_Q, so t_i = s_i² (i = 1, 2, 3) loses nothing: the original
ideal is the reduced one with every point counted 8 times (Exp 14c's eliminant of degree
512 = 4096/8 is this). This is a property of the char-2 symmetrised summation polynomial,
common to every line type, not a twist effect. It is an inseparability redundancy
(each point has multiplicity 8, and the reduced system has 512 distinct points on
P1/P2/R; on D about 32 of them are doubled, Exp 16a), not
the 2-torsion group-action symmetrisation of FGHR or Galbraith–Gebregiyorgis, which
identifies distinct points. **Prior art: the evenness is known.** Faugère–Huot–Joux–
Renault–Vitse (Eurocrypt 2014, HAL hal-00935050, §5.1) state for the char-2 curve
y² + xy = x³ + ax² + b, in their ϕ-coordinates with λ ∈ {0, 1}, that the symmetrised
P_{ϕ,n} involves only even exponents of its first n − 1 invariants, "true in general"
(also stated for their e₁, s₂, …). They use it to speed up *computing* the polynomial; the
paper does not say whether they also substitute squares when solving (their "well-chosen
order" for the GB may be this). So the property is not new. What was not in any source
read (Petit–Quisquater and Shantz–Teske unread): a proof in the x-coordinate (Exp 16b,
via dx/x), the multiplicity accounting (4096 = 512 × 8), the descent factor 2^{dim U} on
affine lines (Exp 16b), and the measured solver effect. FHJRV note the evenness holds for
λ ∈ {0, 1}; the general shift behaviour is Exp 16b's Corollary 3. Galbraith–Gebregiyorgis
(eprint 2014/806, read in full) do not mention it; Kosters–Yeo (arXiv 1503.08001) neither.
Copies: `exp16b_papers/user_upload/`.
- **Reduced system** (Q = 32): degree 512 (508 on one degree-deficient D system,
  = 4064/8). On the two random lines checked: shape position, squarefree s₄-eliminant,
  lex basis of 4 elements. D lines are not in shape position: about 32 points of
  multiplicity 2 (Exp 16a).
- **Validation:** decompositions from the reduced system (field equations, then square
  roots) match brute force in 6/6 D systems, 4 of them with nonzero counts (1, 2, 2, 3).
- **Cost under Singular:** the plain degrevlex GB gets slower (2.3–2.5 s against 0.4 s),
  but the weighted order wdegrevlex(2, 2, 2, 1), which matches deg t_i = 2 deg s_i,
  restores it (0.36–0.77 s). The change of order drops from about 640 s to 81–126 s
  (std + FGLM together), 5–8×. An F4/FGLM implementation should gain more, as FGLM
  scales roughly with the cube of the degree.
- **Consequences.** Shape position removes the separating-form step (Exp 14c) on P1, P2
  and R; D is not in shape position (Exp 16a). The cost conclusions of Exp 13/14 are
  ratios between line types and do not change. Reach at m = 5: see Exp 16d. Logs:
  `exp15_reduced`, `exp15_reduced_validate`; scripts `exp15_reduced_shape.sage`,
  `exp15_reduced_validate.sage`.

**Exp 16: four follow-up leads, each checked by an independent verifier that reran a key
computation where disk space allowed.** All curves have a = 0 unless stated.

- **16a. D's doubled points: a small, Q-independent nuisance, not a lever.**
  - In the three D systems where s₄ generates the quotient algebra (Krylov rank =
    degree), each doubled factor of the s₄-eliminant is a Galois orbit of points of
    multiplicity 2, not two points sharing s₄. Nilradical dimension 32, 33 (Q = 32, two
    degree-512 systems) and 31 (Q = 64, degree 508). One further Q = 32 D system is not
    cyclic in s₄ (lex basis of 6 elements, s₄-minimal polynomial degree 509 < 512) and is
    unresolved.
  - About 32 doubled points out of 512 in every case, with no growth from Q = 32 to
    Q = 64. **Exp 16f confirms it out to Q = 128:** new D systems (one target each on
    fresh curves) give 32 doubled points at Q = 64 and 32 and 33 at Q = 128, all cyclic
    in s₄. One more Q = 64 D system is not cyclic (s₄-minimal polynomial degree 510,
    477 distinct s₄-values). The four pinned (P1) controls on the same curves and
    targets are all cyclic and squarefree (512 simple points). So the count is about 32
    = 2^{m+1} out of 512 at Q = 32, 64 and 128, constant in Q (7 cyclic D systems in
    all). Logs `exp16f_q64`, `exp16f_q128`; run `bash exp16f_run.sh`, which keeps the
    Sage cache and temp files on the working volume.
  - At Q = 64 the P1, P2 and R lines on the same curve and target are cyclic and
    squarefree of degree 512 (one target each). With Exp 14c and the two random lines
    of Exp 15, the doubling has been observed only on D.
  - On the 11 doubled factors of one Q = 32 system: no repeated w, no w = 0, no point
    or pair sum equal to T or O, no T-pair (x_a x_c = √b). Triple partial sums were not
    checked. Three doubled points are F_Q-rational in (t, s₄) but none is a
    decomposition (the w-quartic does not split). Mechanism not identified.
  - **Cost:** none for an F_Q-root solver. In shape form h need not be squarefree:
    gcd(h, X^Q − X) and back-substitution find every rational solution, and Exp 15's 6/6
    brute-force match shows nothing is lost. A RUR/radical-based solver needs the
    squarefree part, which in char 2 needs Yun's algorithm with the square-root step
    (h/gcd(h, h′) deletes doubled roots), about one gcd. Non-cyclic D systems need a
    separating form, as any non-shape system does. **The 1.29–1.52× bracket stands.**
  - Scripts `exp16a_doubled2.sage` (the earlier `exp16a_doubled.sage` had a ring
    mismatch that made its minpoly/radical output meaningless); logs
    `exp16a_q32_D_cls2`, `exp16a_q64_types`.
- **16b. Why the evenness holds, and when descent keeps it (proved).**
  - Lemma: on y² + xy = x³ + ax² + b the invariant differential is dx/x, so on the
    summation hypersurface x_j ∂F/∂x_j ≡ x₀ ∂F/∂x₀ (mod F). A Vandermonde argument then
    gives ∂G/∂e_k = 0 for k < m: the symmetrised G lies in K[e₁², …, e_{m−1}², e_m, x₀],
    and e_m does occur to odd powers. Exp 16d gives a second, independent proof by a
    (Z/2)-grading of the resultant chain. Measured for m = 2, 3, 4 (exact), and the
    identity on specialisations for m = 4, 5, 6. Proof: `exp16b_proof.md`.
  - Supersingular mirror (y² + y = x³ + …): e₂, …, e_m only squared, e₁ not.
  - **Descent (Corollary 3):** on x_i = c₀ + u w_i with r = c₀/u, the reduction factor is
    2^{dim U}, dim U = m − dim_{F_Q} span{1, r, …, r^{m−1}}. Every line through 0 (all of
    D, P1, P2, R) gets the full 2^{m−1} = 8. r ∈ F_Q gets 8 after an F_Q-linear change
    of variables. r ∈ F_{Q²} ∖ F_Q gets 4. A generic affine line gets none (degree 4096
    measured at Q = 16). Measured: dim U predicted correctly 12/12 (Q = 32) plus 5/5
    (Q = 16, verifier), and 10/10 at m = 3. Exception: the target R = T (x_R = 0) is
    even in every variable (dim U = m on every line), so "every target" statements
    exclude it.
  - Literature: FHJRV §5.1 (above) states the evenness without proof. No evenness,
    multiplicity or descent statement in the Gebregiyorgis thesis, FPPR 2012, Huang's
    thesis, Semaev 2015, Karabina 2015, Kosters–Yeo, or Galbraith–Gebregiyorgis.
    Petit–Quisquater and Shantz–Teske were not read. FHJRV §5.2 does a subfield descent
    where the same multiplicity must occur, with "a well-chosen order", so they may have
    used it implicitly.
- **16c. Affine lines: there is a denser baseline than D, but it costs more per trial.**
  - **Pool-Q lines exist.** Inside the plane √b·F_{Q²}, every lifting x is in 2E
    (Tr(√b/x) = 0 there), and the lines c₀ + u₀F_Q with u₀ = √b/Tr_{K/F_{Q²}}(√b) and
    Tr(c₀) = Tr(a) have all Q x-values lifting in one class, against D's Q − 1. There are
    Q/2 − 1 such lines (Tr(a) = 0) or Q/2 (Tr(a) = 1). Measured at Q = 32 to 512 on
    several curves.
  - **Nothing else comes close.** Exhaustive at Q = 32 (all 1.1·10⁹ affine lines per
    curve) and Q = 64 (all T(u) = 0 lines): the maximum pool Q occurs only on those plane
    lines; the best other line has pool 30 or 31 (Q = 32) and 52 (Q = 64). Proved: off
    the plane the pool is at most Q/2 + 3√Q + O(1) (Artin–Schreier curve of genus 3;
    assumes √b ∉ F_{Q²}).
  - **Tr(a) = 1 curves have no lift-dense line through 0** (on D, Tr(x + a + √b/x) =
    Tr(a)). The Exp 13 class table's D row assumes Tr(a) = 0. On those curves the only
    dense lines are the affine plane lines.
  - **Reduction:** plane lines have r = c₀/u₀ ∈ F_{Q²} ∖ F_Q, so they get the 4×
    reduction, not 8×. The verifier measured reduced degree 1020–1024 (original 4080 =
    4 × 1020), non-radical with multiplicity 4.
  - **Cost** (one curve and target, Q = 32): reduced GB, same process: 9.1 s on A (4×,
    degree 1020) against 0.40 s on a D line (8×, degree 512), about 21×. Unreduced GB:
    76.2 s on A (separate concurrent process) against 0.42 s on D, about 180×. FGLM on
    reduced A was not measured.
  - **Effect on the gain.** In the balanced model an affine plane line beats pinned as
    a baseline only if its trial costs less than twice a pinned trial (pool Q against
    Q/2). With the reduction used:
    - Under F4-like proportions (GB about 30% of a trial) the 21× GB gap alone puts A
      well above 2×: pinned binds and the bracket stands.
    - **Under Singular, measured (Exp 16e):** stdfglm (std + FGLM to lex) on reduced A
      against reduced D, same process and same target, two curves (seeds 23, 31):
      A 694.1 s / D 90.6 s = **7.60×** (lex eliminant degree 1020 against 508), and
      A 700.5 s / D 91.3 s = **7.73×** (1023 against 510). Doubling the degree costs
      about 7.7×, close to cubic. A trial on A costs about 7.7× a pinned trial, far
      above the 2× break-even, so **pinned stays the binding baseline and the
      1.29–1.52× bracket stands** under Singular as well. (An earlier version of this
      note guessed near-linear scaling from Exp 15 and called it near break-even; the
      measurement refutes that.) One target per curve; the second rep was stopped
      once both curves agreed.
    - Without the reduction, with Singular's FGLM dominating, A would cost about 1.1×
      D and D's edge over A would be about 1×; that regime is moot once the reduction
      is used.
    Not measured: yield on A. Script `exp16e_fglm_AD.sage` (run: `bash exp16e_run.sh`);
    logs `exp16e_fglm_s23`, `exp16e_fglm_s31`.
  - Scripts `exp16c_affine.c`, `exp16c_plane.gp`, `exp16c_plane256.gp`,
    `exp16c_affine_system.sage`, `exp16cv_reduce.sage`, `exp16cv_d0.sage`; logs
    `exp16c_affine_exh`, `exp16c_plane*`, `exp16cv_reduce`, `exp16cv_d0`. The original
    agent's claim that all affine lines stay at degree 4096 was refuted by the verifier
    (the 4× reduction).
- **16d. General m, and m = 5.**
  - The reduction holds for every m: original/reduced degree is exactly 2^{m−1}
    (freeness), and weighted Bézout bounds the reduced degree by 2^{(m−1)²}: 16, 512 and
    65536 at m = 3, 4, 5.
  - m = 3 (n = 3): reduced counts match brute force 48/48 (Q = 32, 64; D, P1, R), degree
    64 → 16.
  - m = 5: the symmetrised S₆ was built by evaluation/interpolation (1267–1281 terms,
    under a minute at Q = 2 and Q = 8, three direct checks). No GB finished: Singular std
    at Q = 8 (durations not logged), and msolve F4 over F₂ at Q = 2 (75 s CPU, stopped at
    degree 21 when the disk filled). 65536 is an upper bound only.
  - Tools: msolve 0.11 works over F₂ only (no F_{2^d}), and its solve mode loops on
    restarts there ("Ideal not in generic position"). Singular's FGLM scaled as about
    D³ on random dense 3-variable systems (D ≤ 512); extrapolated to D = 65536 that is
    about 2 CPU-years. This is a weak estimate (4 points, not summation systems, no
    sparse FGLM).
  - **Verdict:** m = 5 feasibility is undetermined, not ruled out. The GB was not
    obtained within budget, and dense FGLM looks out of reach. Cheapest next paths: a GB
    with field equations at Q ≤ 8, or sparse-FGLM/Wiedemann code over F_{2^d}.
- **Not covered.** The T-invariant u ∈ b^{1/4}F_Q are excluded, as before.

## 5. Numerical checks

Each check was written so that it could fail.

| exp | setting | prediction | result |
|---|---|---|---|
| 1 | n = 2, q ∈ {37, 41, 43, 47}. Every line on 12 curves per q: 6 generic and 6 affine images of F_q-curves. | degenerate ⇔ (D); all-lift ⇔ c₁ square | **0 mismatches in 87,312 (line, curve) pairs.** Generic curves: no degenerate line, maximum density 0.757. |
| 1b | n = 2, q ∈ {7, 11, 13}, below the threshold | sporadic degenerate lines possible | q = 7: 20; q = 11: 1. So the threshold in Prop 2 is needed. |
| 2 | n = 4, q ∈ {11, 13, 17}: 21 constructed (D) lines | all lift; dead set = the wrong K′-side | 276 of 276 lift; **128 dead, and the side rule predicts them with 0 mismatches.** |
| 2c | char 2, N = 15, 8 curves with ℓ > 1000 | twisted line all-lift and useful | 248 of 248 lift, 1 dead. Standard base: 112 of 248 lift, 0 dead. |
| 3 | n = 3, q ∈ {83, 89}, 12,000 random lines | no degenerate line | density range [0.265, 0.687]. Sampling only. |
| 4 | n = 4, roots {α, α^q, γ ∈ F_q}, 17 curves | (P): all-lift, no descent | 212 of 212 lift, **0 dead**, 0 descend, j ∉ F_{q²} on 16 of 17. The arm meant to run a same-curve (D) control did not finish within the time cap; Exp 2 is the (D) control. |
| 5 | n = 4, q ∈ {11, 13, 17, 19}, 27 generic full-2-torsion curves; Prop 5 construction | 6 lines per curve; degenerate; all-lift ⇔ c₁ square; no descent | 146 lines built (16 solver singularities, explained by the norm condition). **146 of 146 satisfy the pair relation. 73 have c₁ square and all 73 are all-lift. 73 are all-twist. 0 descend. 3 dead of ≈1,050.** Exp 5 tested descent only through f_L; Exp 7 adds j: j ∈ F_{q²} on 1 of 16 fresh curves. |
| 6 | singularity of the Prop 5 linear map, 1,440 pairs | singular ⇔ N(B/A^q) = 1 | 0 exceptions; rate 0.06–0.11, close to 1/(q−1) |
| 8 | 4-term sum coverage, n = m = 4, q ∈ {11, 13, 17}; sub-runs 8b, 8c, 8e and 8f use 14, 7, 12 and 13 fresh curves. One or more all-lift case-P line per curve; random lines (F ≥ 6) on the same curves; random symmetric sets of the same size F, drawn from all of E(K) and from H with the same class split. | Lemma 9: x − e_k always square. Yield on 2E(K) ≈ (lift density) × (index-2 concentration); otherwise the null. | **x − e_k nonsquare: 0 of 316 points.** Coverage of 2E(K): P 0.734, 0.713, 0.757; same-size random symmetric 0.53, 0.54, 0.52; random lines (mean F 11.3, 13.3, 18.4) 0.094, 0.061, 0.073. P against the null drawn from H: 0.973, 0.960, 0.957. Trials per needed relation (columns ÷ coverage): R 60, 109, 126; P 15, 18, 22, so **4.0×, 6.0×, 5.6× fewer**. Two low outliers. The first, 0.12 on all of E(K), has v₂(N) = 6, its base sits in an index-8 subgroup, and odd-part coverage is 0.938 (exp8g_repro). The second, 0.53 on 2E with v₂ = 2, is not 2-adic. **8h** (26 lines, q = 11, v₂ = 2) counts nontrivial 4-term coincidences P₁−P₂ = P₃−P₄. Case P averages 9.85 and the null in H 5.85, with 21 of 26 and 13 of 26 lines nonzero, about 2σ. Coverage drops with excess: 0.77 at zero excess, down to 0.67 at 18–24. **Superseded by Exp 9:** with 157 case-P lines, the excess is not significant (ratio 1.06–1.14, under 1σ). Coverage of the ℓ-subgroup itself tracks 2E(K). Logs exp8*.log. |
| 9 | 4-term coincidence excess by factor-base type. q = 11 (40 curves) and q = 13 (25 curves). Types: P = all-lift case-P lines; R = random lines; U = union of two random lines (size about 2q); Q = random quadratic curves x = c₀+c₁w+c₂w². Each is compared to a random symmetric set with the same split over the four 2-descent classes. | If the 8h excess is specific to P: P ratio ≈ 1.7 while U ≈ 1 | **P: 656/616 = 1.06 (93 lines) and 458/402 = 1.14 (64 lines), under 1σ combined. U: 1.20 and 0.90.** R and Q have only a few events each (36/48, 60/12, 30/12, 38/12), too few to read. So there is no case-P-specific excess; Exp 8h's 1.7× was a small-sample fluctuation. Log exp9_coinc_lines.log. |
| 10 | n = m = 6, K = F_{q⁶}, q = 11 (3 curves, 12 random lines) and q = 13 (3 curves, 24 random lines), v₂(N) = 2. Case-P line from B z − A^{q^s} z^{q^s} = γ^{q^s} − γ, γ ∈ F_{q³}\F_q. Distinct 6-sums in 2E(K), computed as class-matched pairs of 3-sums. | lift rule; no descent; Lemma 9; P matches null drawn from H; gain over random lines grows toward ≈64× | **Checks 5/5, descent 0, Lemma 9 failures 0.** P coverage 0.187 and 0.174; P/null 0.999 and 0.995. Trials per relation: P 59 and 75; random lines median 809→2340, best 221 (q = 11) and 284 (q = 13). **Ratio vs median 14× → 31×; vs best 3.8× at both q.** The multiset model overestimates every coverage by about 1.5–2×, for P and R alike (sums that include both P and −P collapse), so compare against the null rather than the model. Logs exp10_n6.log, exp10b_n6_q13.log. |
| 10c | n = 6 construction only: q = 11 (18 curves) and q = 13 (10 curves), s ∈ {1, 2}, all 6 ordered pairs, no v₂ filter | lift rule, no descent, j not in a subfield, Lemma 9 | **s = 1: 104 + 58 lines; s = 2: 108 + 58 lines. Lift rule holds on all 328. Descent 0, j in a subfield 0, Lemma 9 failures 0.** Singular solves: 8. |
| 11 | Per-trial algebraic cost: Weil descent of S₅ on case-P vs random lines, n = 4, q = 11, solved with msolve | ideal degree and solve time equal for P and R | **F4 cost equal; default solve path stalls on P (open whether this is small-q).** GB-only + field equations (Exp 11c): P 1.99 s, R 1.94 s median; decompositions match brute force on 24/24 systems. Without field equations: GB 1.4–2.4 s for both, ideal degree ≤ 4096. P ideals are not in generic position (eliminant 4074, squarefree 4065), so msolve's default parametrisation times out on P (10/12 at 600 s; R 2.4 s). Field equations only work at toy q, so equal cost holds only on that route. Run with `DYLD_LIBRARY_PATH=/opt/homebrew/Cellar/flint/3.6.0/lib`, because the `opt/flint` link points at 3.5.0. Logs: `exp11_solve`, `exp11b_solve`, `exp11c_gbfe`. |
| 11d | Default msolve path (no field equations), n = 4, q = 31 and 43 | is the case-P stall a small-q artefact? | **Not shown either way.** GB is equal. Case P is non-generic on 11/12 systems (separation failure) and provably non-radical on 5/12; random lines are generic on 12/12. A GB degree below 4096 predicts a timeout. Solve time is (retries + 1) × an attempt costing 1.5–2.5× a random-line attempt. q = 31: P 3/6 timed out at 900 s, the rest 8–153 s, R 5–7 s. q = 43: P median 35 s, R 5.4 s, about 5× per trial. Recovery matches ground truth on every finished system. Logs: `exp11d_default_path`, `exp11d_reparse`. Scripts: `exp11d_gen.gp` (meet-in-the-middle ground truth, matching the exhaustive count on 10/10) and `exp11d_run.py`. |
| 11e | Relation yield per trial at n = 4, q = 43: case-P line vs random lines, using the meet-in-the-middle ground truth | gain grows with q | **Yes.** Case P gives 1.43 decompositions per target (hit 74%), random lines 0.083 (hit 6.4%), a ratio of 17×. Against Exp 11d's ~5× cost per trial, that is about 3.4× net per relation; normalised for factor-base size against random lines, about 1.8×. Log: `exp11e_yield43`. |
| 11g | Why case P retries: F_q-rational σ-points (via field equations) and the fraction of F_q-linear forms separating them, against Exp 11d's retries | separation failure at small q | **Supported.** Under 3% separating → timeout (3/3, 61–70 retries); 20–87% → finished. Random lines have 0–4 rational points and are always generic. 2 exceptions point to small-extension points. msolve `-d` and `-c` don't help. No case-P factorisation of S₅ (Exp 11f). Logs: `exp11f_factor`, `exp11g_sep`. |
| 11h | Default msolve path at q = 1009, n = 4 (6 P + 6 R) | separation artefact vanishes at large q | **Yes.** P generic 6/6 with 0 retries; solve 6.9 s median for both P and R; GB 2.0 s against 1.9 s; decompositions match ground truth 12/12 (P 7 = 7). Logs: `exp11h_q1009`, `exp11h_truth1009.txt`. |
| 11i | Exact yield (64·N₀/#E) at q = 1009: case P against 2000 random lines per curve, best-of-k search | gain against searched lines approaches 2^m | **Yes.** P yield 1.341 against best-of-2000 0.062, a per-trial ratio of 21–22×. Trials per relation set: 11.7–11.9× against best-of-2000, 16× against a median line; the regime formula predicts 11.9 (F in points). The formula was validated against brute force at q = 43 (ratio 1.015 / 1.09). The `regime pred` column in the log uses F in x-values and is wrong by (2)³; ignore it. Logs: `exp11i_validate43`, `exp11i_q1009`. |
| 12 | Class-pinned lines (two roots shifted into F_{q²}); balanced cost model: one-class factor base, 0/1/2 large primes (union–find cycle rank), LA from measured GMP unit costs, scaled sweep q = 2¹⁶–2⁶⁴ | does the 12–16× survive linear algebra and the best non-case-P line? | **No.** Pinned lines put all ≈ q/2 x-values in one class at the same per-trial msolve cost (generic, 0 retries). Gain against pinned: 3.99 (2¹⁶), 2.94 (2²⁴), 1.20 (2³²), 1.02 (2⁶⁴). Against random: 15.9, 11.0, 3.4, 2.05. Analytic asymptotes √(ℓ_P/ℓ) = 1 and 2. T is flat to 2²⁰ and equal across line types. Logs: `exp12a_classes`, `exp12b_modops`, `exp12c_sweep3_qsim20` (+ `sweep_qsim18/20`), `exp12d_graphcheck`, `exp12e_pinned`, `exp12f_pinned_q*`, `exp12_trial_q1048573`. |
| 13 | Char 2, N = 4d (Q = 32, 128, 256): halving criterion, class counts on lift-dense / pinned / random lines through 0, brute-force yield at Q = 32, balanced sweep with char-2 pools | does the char-2 lever survive the pinned baseline? | **Partly.** The criterion has 0 mismatches. Lift-dense: one class of Q − 1. Best non-lift-dense (pinned): one class of exactly (Q − 2)/2. Gain against pinned: 7.97 (2¹⁶), 4.69 (2²⁴), 1.70 (2³²), 1.52 (2⁶⁴), analytic √2 (grid ±6%). No 4E-level concentration (Exp 13b). Against random: 15.9 → 2.03. Per-trial cost equality is assumed (repo C1), not re-measured. Logs: `exp13_char2`, `exp13_char2_bf`, `exp13b_4E`, `exp13_sweep_char2`. |
| 14 | Char 2, Q = 32, 64, 128: Singular `std` degrevlex GB time and ideal degree per line type (D, P1, P2, R; 96 systems), decompositions against brute force, sweep re-priced | equal per-trial cost (repo C1) | **Median yes, mean not quite.** 72/72 decompositions match brute force. Median GB time equal; mean D/others 1.33, 1.32, 1.02, all from degree-deficient systems (D 5/24, others 2/72). Re-priced gain against pinned at 2⁶⁴: 1.29 (from 1.52). Change of order not timed per type. Logs: `exp14_d5`, `exp14_d6`, `exp14_d7`, `exp14_sweep_char2_TD1.33`. |
| 14b | Char 2, Q = 32: Singular `stdfglm` change-of-order CPU time per line type (32 systems) | does the post-GB step penalise D? | **No.** D/others = 0.98 (D 640 s, others 647–655 s). Change of order ≈ 1400× GB, so per-trial cost is equal under Singular. With msolve-like proportions D ≈ 1.1× dearer, gain against pinned at 2⁶⁴ 1.44×. Logs: `exp14b_fglm_s2501..2504`, `exp14_sweep_char2_TD1.10`. |
| 14c | Char 2, Q = 32: lex basis structure of one target's four systems (D, P1, P2, R) | why shape is False | **Common to all types.** s₄-eliminant degree 512 = 4096/8 on every type, so every type needs a separating form. D's eliminant is not squarefree (446 of 508). One target only. Log: `exp14c_shape`. |
| 15 | Char 2, Q = 32: s₁..s₃ exponent parity; reduced system t_i = s_i²: degree, shape, decompositions against brute force, GB/FGLM time | is the factor 8 removable? | **Yes.** s₁..s₃ only squared; reduced degree 512, shape position; 6/6 match brute force (4 nonzero). GB equal with wdegrevlex(2,2,2,1); change of order 5–8× faster under Singular. Common to all line types; P1/P2/R in shape position, D not (16a). Evenness itself is in FHJRV 2014 §5.1. Log: `exp15_reduced`. |
| 16a, 16f | Char 2, Q = 32, 64, 128: structure of D's doubled s₄-roots | cost or lever? | **Neither.** About 32 points of multiplicity 2, constant from Q = 32 to 128 (7 cyclic D systems; P1 controls 0), not decompositions, observed only on D. No cost for an F_Q-root solver. |
| 16b | Proof of evenness; descent on affine lines | why, and where it survives | **Proved** via ω = dx/x. Factor 2^{dim U}: 8 on every line through 0, 4 for c₀/u ∈ F_{Q²}, 1 generically. 27/27 dim U predictions. |
| 16c | Char 2, Q = 32–512: one-class lifting pool of all affine lines | stronger baseline than pinned? | **Pool Q exists** (affine lines in √b·F_{Q²}), but those lines get only the 4× reduction: GB 21× dearer than D, same process; stdfglm 7.60× and 7.73× (16e, two curves). Pinned stays the baseline; bracket stands. No through-0 D line when Tr(a) = 1. |
| 16d | m = 3 and m = 5: evenness, degree, solvability | does it extend to m = 5? | **Reduction proved for all m** (factor 2^{m−1}); m = 3 matches brute force 48/48. m = 5 GB not obtained; feasibility undetermined. |
| 7 | distinct ℓ-columns (keyed by x of the ℓ-projection) on all-lift lines: odd case P (39 lines); char-2 random valid u (6); char-2 T-invariant u ∈ b^{1/4}F_Q (6) | P and random u: chance level; T-invariant: about half | P: 474 distinct vs 478.7 expected. Random u: 178 vs 180.8. **T-invariant: 90 of 186 (collapse detected).** |

## 6. Consequences and next steps

- **Prime-field ECDLP:** closed within the scope of KN-OPEN-020 and the twist lemma. I
  would stop spending runs here.
- **Odd n, and n = 2** (EcGFp5, EcMasFp5, all GLS curves over F_{p²}): no lift-dense lever
  exists (Cor 7).
- **Even n ≥ 4 with full 2-torsion (new):** the 2^m yield lever exists and is generic
  (Prop 5). This puts it in the same place as the char-2 c2pnb/Oakley result. It is a
  constant-factor change:
  - yield per trial: measured at n = m = 4, q ≤ 17 (Exp 8). The decomposition
    probability on targets is about 0.73, against 0.06–0.09 for random lines, which comes
    to 4–6× fewer trials per needed relation. **This is an n = 4 figure, not a small-q
    one.** The case-P sum density on targets is 2^{m+1}/m! = 32/24 > 1 at m = 4 for every
    q, so the probability is capped near 1 − e^{−4/3}. The full ×2^{m+1} density gain
    can only show where 2^{m+1}/m! < 1, i.e. m ≥ 6 (Exp 10);
  - the usable base doubles;
  - with no large primes, the relation phase gets ×2^{1−m} cheaper and the linear algebra
    ×4 dearer;
  - **against the right baseline the lever is gone (Exp 12).** Class-pinned lines exist
    under the same hypothesis and put about q/2 x-values in one class. Case P then wins
    only 4× with a whole-line factor base (q ≲ 2²⁰), and 1.02× in a balanced attack at
    q = 2⁶⁴. Against random lines the balanced gain is 2×.

  Full 2-torsion already invites symmetrised summation polynomials (Faugère–Huot–Joux–
  Renault–Vitse). **Whether the two gains stack is an open question.**
- **Next steps, cheapest first:**
  00. *(Partly done: Exp 11. F4 cost is equal at n = 4, q = 11, but the default
      parametrisation stalls on case P. Exp 11d: at q = 43 that overhead is about 5× per
      trial with msolve's default solver. Exp 11e: the trial gain at q = 43 is 17×, so
      the net is positive, about 3.4× per relation.)* Exp 11g: the overhead tracks
      failure to separate the ~10 F_q-rational σ-points, which should vanish as q grows.)*
      Exp 11h confirmed it at q = 1009: generic 6/6 and equal solve time.)* Per-trial cost
      is therefore no obstacle. Exp 11i: against the best of 2000 searched lines at
      q = 1009, relation collection is about 12× cheaper, matching the regime formula,
      with limit 2^m = 16.)* *(Exp 12: the comparator was wrong. Class-pinned lines, which
      are not case P, cut the gain to 4× with a whole-line factor base and to 1.02× in a
      balanced attack at q = 2⁶⁴.)* **The case-P lever is closed for n = 4.**
      *(Exp 13: the char-2 lift-dense lever keeps a 2× class-pool edge over the pinned
      baseline. That is worth 8× with a whole-line factor base, but only 1.4–1.5×
      (analytic √2 = 0.5 bit) in a balanced attack at Q = 2⁶⁴.)* *(Exp 14/14b: per-trial cost is
      equal under Singular; the change of order shows no D penalty (0.98×), and the GB
      penalty (mean up to 1.33×) gives about 1.1× under msolve-like proportions. Balanced
      gain 1.52× measured under Singular, about 1.44× estimated for msolve, 1.29× worst
      case.)* *(Exp 16/16e: the bracket survives D's doubled points and the dense affine
      lines; reduced affine-plane trials cost 7.6–7.7× reduced D under Singular.)*
      *(Exp 16f: D's doubled-point count is 32–33 at Q = 32, 64 and 128.)*
      Open run: an m = 5 GB with field equations at Q ≤ 8.
      Still open: whether one-class factor bases, which every curve with full
      2-torsion has, stack with the 2-torsion symmetrisation. That symmetrisation
      already uses the same 2-descent structure, so they may be the same lever.

      n = m = 6 cannot be run for any line type: the ideal degree is 2^30.
  0. *(Done: Exp 8 measured the yield.)* Still open: run at larger m/n where the base does
     not saturate (n = 6 needs the case-P count first). *(Exp 9 settled the 4-term
     coincidence question: there is no case-P-specific excess.)*
  1. Formalise Prop 3, Cor 4 and Lemma 9 in Lean. They are short and purely algebraic.
  2. Count case-P lines for general even n, and test the family dimension n/2 − 2 at n = 6.
  3. Test whether the case-P line and the 2-torsion symmetrisation are compatible in one
     descended S_{m+1} system (same q, one solver).
  4. Run the repo's still-open end-to-end toy solve, with both factor bases, in char 2 and
     in the new odd-char case.
