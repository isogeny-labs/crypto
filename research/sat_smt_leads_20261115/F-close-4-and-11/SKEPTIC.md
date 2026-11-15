# SKEPTIC review — F-close-4-and-11 (Part 1 idea 4, Part 2 idea 11)

Independent review, 2026-11-15. One process under `nice` on the shared 4-core box (load ~6.5),
about 2 minutes of CPU. Everything below is `[pcc:unverified]`: pilot-level checks, no pcc claim,
no replay certificate. Producer files were not edited; reruns were made in a scratch copy
(`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-F/`, scripts
`sk1.py`, `sk2.py` there; the scratch directory is not durable).

## Summary

| item | result |
|---|---|
| Reproduction of smallest cells | **yes** — `raw_l3.jsonl` rerun: 324/324 rows identical in every solver counter; Part 1 `results.json` identical; `defect_min.json` identical except `secs` |
| Part 1 verdict under its rule | **inconclusive**, and not evaluable as registered (the rule's "largest l" is l=5, which was not run). At l=4 the kill/inconclusive boundary is decided by a 0.0005 margin against an A/A interval that is not a valid floor |
| Part 1 "substantively no lead" | outside the rule (post hoc), but supported by a fact the README does not state: at l=4 UNSAT the best k is k=1 for both arms in 8/8 instances |
| Part 2 verdict under its rule | **inconclusive**. (a) holds and is in fact a theorem for prime n; (b) is a required conjunct of "nothing to synthesise" and was not run at power |
| Part 2 (a) | **verified** in 6 cells with independent code |

## 1. Reproduction and brute-force agreement (Part 1, l=3, n=11)

Own GF(2^11) arithmetic (carry-less multiply, modulus x^11+x^2+1, the one the producer's `Field`
selects), own affine group law for y^2+xy = x^3+x^2+1, points found by scanning y. Producer code
was used only to regenerate the same 12 targets and CNFs.

- All 12 targets: own group-law brute force over points with x in V\{0} gives 0 decompositions
  for the 8 UNSAT targets and 6 (= 3! orderings of one unordered triple) for each of the 4 SAT
  targets. This equals the producer's `nzeros`, the full model set of the `lit` CNF (blocking
  clauses), a plain CaDiCaL call, and the SAT/UNSAT status of every one of the 324 raw rows.
- Each planted triple sums to a point with x = x(R) for some sign choice; x(R) is never in V.
- Size caveat: only 3 to 5 of the 7 nonzero x in V lift to curve points at l=3. These instances
  are tiny (lit refutation about 450 conflicts).

Not rechecked: the `sym` CNF model set (the producer's `--validate` covers two instances), the
symbolic e-form, anything at l=4 beyond recomputing statistics from the stored rows, and l=5.

## 2. Part 1 statistics, recomputed from raw rows

Pooled best-k xor/lit and sym/lit ratios reproduce exactly (0.951 [0.896, 1.010]; 0.975
[0.955, 0.993]; SAT 1.600 and 2.094; sym 5.04, 2.15, 10.35, 6.47).

Problems found:

1. **The "best k" at l=4 UNSAT is k=1 for both arms in all 8 instances.** Total conflicts rise
   with k for both arms (lit median 3750 at k=1, 4257 at k=4). The optimum sits on the edge of
   the grid and k=0 (no cubes) was never run, so the headline 0.975 compares two 2-cube runs and
   says nothing about whether cubing helps. It does support "no lead": xor at k=4 costs about the
   same as lit at k=1.
2. **The fixed-k effect is larger than the headline and is not reported.** l=4 UNSAT xor/lit at
   fixed k: 0.975 (k=1), 0.938 (k=2), 0.887 (k=3), 0.890 (k=4), each with a tight interval
   (k=3: [0.872, 0.902]). l=3: 0.895, 0.927, 0.967. So XOR cubes do reduce conflicts by 3 to 11%
   at equal k, growing with k at l=4. That is a real effect, far from the 0.8 threshold and of
   no use while more cubes cost more in total. "~2.5%" understates it.
3. **The A/A floor is not computed fairly.**
   - It pools all ordered seed pairs (i/j and j/i), so its geometric mean is 1 by construction,
     and it bootstraps 48 pairs as if independent although there are 8 instances. The interval
     is the standard error of a symmetrised mean, not the null distribution of an 8-instance
     pooled ratio.
   - At k=1 there are only two cube orders. Seeds 101 and 102 give the same order ([1,0]), so
     16 of 48 pair ratios at l=4 UNSAT are exactly 1. The l=4 floor rests on two distinct
     orders, not three.
   - It compares single seeds while xor/lit compares 3-seed means.
   - A same-shape null (per instance one lit seed over another lit seed, pooled over 8 instances,
     20000 draws) gives [0.984, 1.016] at l=4 UNSAT and [0.921, 1.086] at l=3, against the
     producer's [0.993, 1.007] and [0.966, 1.034]. Scaled to 3-seed means, the l=4 band is about
     [0.991, 1.009].
4. **The verdict at l=4 is knife-edge.** The producer's code reads "inside A/A" as interval
   overlap. The gap between the xor/lit upper bound (0.99292) and the A/A lower bound (0.99345)
   is 0.0005. With either fairer band above the intervals overlap, which is "kill" by the same
   code. A t interval on the 8 log ratios is [0.950, 0.9997]. Picking k on one seed and scoring
   on the other two gives 0.973 [0.952, 0.994], the same again, and 0.987 [0.967, 1.005].
5. **"Basic bootstrap" is registered, percentile is implemented.** The difference is small here
   (basic [0.9565, 0.9943] at l=4). The code also takes the geometric mean over seeds where the
   protocol says mean; that changes nothing material (0.9745 vs 0.9746).
6. **In-sample best-k selection.** No bias at l=4 (k=1 everywhere). At l=3 lit always picks k=3
   while xor picks k in {1,2,3}, so the in-sample minimum favours xor slightly. Held-out
   estimates are 0.861, 0.921, 0.979, so there is no sign the headline is inflated, but the l=3
   interval including 1.0 is the more honest reading.
7. **SAT population, N=4.** A percentile bootstrap over 4 instances has 35 distinct resamples.
   At l=4 the per-instance ratios are 1.03, 3.60, 1.46, 3.55; the t interval is [0.76, 5.7] and
   the fixed-k=4 ratio is 0.999. "xor markedly worse on SAT, well outside the floor" holds at
   l=3 (t interval [1.18, 2.17]) and is weaker than stated at l=4. All cubes are solved with no
   early exit, so this is not a time-to-first-solution measure.
8. **`sym` arm: the direction is supported, the "censored (0)" column is misleading.** There are
   6 censored `sym` rows at l=3 and 9 at l=4 (UNSAT and SAT). The table shows 0 because best-k
   prefers uncensored k. The cap therefore cannot have produced the 2 to 10x figure; it makes it
   a lower bound at the censored k. Every instance has sym/lit above 1.6 and fixed-k ratios are
   2.0 to 15.6. The cost is heavy-tailed in cube order (median 2.5k to 13k, some seeds above
   50k), so the point estimates are unstable. Scope: one encoding (x-bits kept, e2/e3 defined
   from x-bits, one AND variable per e-bit pair). The claim is about this CNF, not about
   symmetrisation in general. The "1.45M conflicts" pilot solve is not in the raw data.
9. **Propagation claim.** "~1.7x propagations at equal conflicts" matches SAT targets (1.72,
   2.13) but not UNSAT at best k (1.29 at l=3, 1.05 at l=4).
10. **The l=5 corroboration is borrowed** from the 2026-11-14 skeptic's runs under a different
    protocol. It is not evidence from this directory.

**Does the decision follow from the rule?** As computed, "inconclusive" follows from the
producer's numbers. The rule names the largest l, which the protocol set at l=5; substituting
l=4 is a deviation the README states. My label: **inconclusive, incomplete** (kill by the letter
under a fair A/A floor at l=4, not evaluable at l=5). "Substantively no lead" is a judgment
outside the rule. I agree with its direction: the best conflict reduction seen is 11% at fixed
k, the alive threshold is 20% at the two largest l, and cubing raises total conflicts for both
arms at l=4.

## 3. Part 2 (a): minimum of dim span(V.V)

Own code, own moduli, no producer imports. Enumeration of **all** l-dimensional subspaces (no
scaling reduction), counts checked against the Gaussian binomial:

| cell | subspaces | histogram of w | min | 2l-1 |
|---|---|---|---|---|
| l=2, n=5 | 155 | {3: 155} | 3 | 3 |
| l=2, n=7 | 2667 | {3: 2667} | 3 | 3 |
| l=3, n=5 | 155 | {5: 155} | 5 | 5 |
| l=3, n=7 | 11811 | {5: 2667, 6: 9144} | 5 | 5 |
| l=4, n=7 | 11811 | {7: 11811} | 7 | 7 |
| l=3, n=11, containing 1 | 174251 | {5: 2387, 6: 171864} | 5 | 5 |

- The subsets containing 1 reproduce the producer's histograms exactly ({5: 147, 6: 504} at
  (3,7), 1395 at (4,7), 174251 at (3,11)).
- **Scaling reduction is valid.** (cV).(cV) = c^2 (V.V), and multiplication by c^2 is an
  F2-linear bijection; every V contains a nonzero v and v^-1 V contains 1. Numerically: 0
  violations in 6000 random (V, c) at (3,7), (3,11), (4,11). The full enumeration at (3,7)
  gives the same minimum as the reduced one.
- Detector control: composite n gives values below 2l-1 as it should (l=2: w=2 at n=4 and n=6;
  l=3, n=6: w=3), so my code can see a violation.
- At (3,7) the 2667 minimisers are exactly the 2667 geometric-progression subspaces c<1,t,t^2>.
  The minimum is attained only by the polynomial span there.

**The cited bound.** The linear Kneser theorem (Hou-Leung-Xiang 2002 for separable extensions,
Bachoc-Serra-Zemor in general) states dim(AB) >= dim A + dim B - dim H(AB), with H(AB) the
subfield stabilising AB. I state this from memory; I did not read the papers in this review.
Hypotheses needed for 2l-1:

- the stabiliser of V.V is F2. For prime n the only alternatives are F2 and the whole field, and
  the latter means V.V is everything, so the correct statement is **w >= min(n, 2l-1)**;
- hence **2l-1 <= n** for the bound to read 2l-1;
- separability, automatic over finite fields.

The protocol states both conditions correctly. For prime n claim (a) is therefore a theorem and
the enumeration is a sanity check of it, not the evidence.

Weaknesses in the presentation of (a):

1. Only (3,7) and (3,11) are informative. The l=2 cells are trivial (w=3 unless V is a scaled
   F4, impossible for odd n). (4,7) is saturated: 2l-1 = n = 7 and every V has w = 7.
2. The sampled cells carry almost no weight. At (4,13), (5,17), (6,19) the 5000 random draws do
   not even reach 2l-1 (minima 8, 13, 17). "No V below 2l-1" there rests on the theorem alone.
3. Scope. (a) concerns the proxy w = dim V.V. It does not show that no V improves the m=3
   descended system by another statistic; the README says so under "Not covered".
4. The task summary lists "l=2 n=3..13"; the cells are n = 3, 5, 7, 11, 13 only.

## 4. Part 2 (b) and the decision

`raw_sat_l4.jsonl` has 40 rows, none censored: S 2 bases, A 2 bases, B 1 base, l=4 only. That is
against a registered 16 bases per arm at l=4 and l=5. Per-base geometric means on
non-decomposable targets: S 65.8k and 74.7k; A 80.0k and 92.7k; B 76.4k.

- The producer's own `results.json` has `part_b_inside_aa: false`: its S/A interval
  [0.71, 0.93] is disjoint from its A/B interval [1.05, 1.21]. The README sentence "there is no
  SAT difference outside the A/A floor" contradicts that file. Neither reading means anything
  with 2, 2 and 1 bases; the point is that (b) shows nothing either way.
- The rule is a conjunction: "nothing to synthesise" needs (a) **and** (b). "Alive" needs a V
  below 2l-1 (excluded for prime n) or a 2x SAT gain (not tested at power). The registered
  outcome is the third branch: **inconclusive**.
- The README's verdict, "nothing to synthesise for the structural claim; (b) underpowered", is
  not one of the three registered labels. It describes the state honestly but relabels an
  inconclusive outcome as a partial negative.
- Borrowing the 2026-11-14 SAT numbers does not fill (b); that run was judged underpowered by
  its own skeptic.

Part (c) (linear 2+1 leaves and candidates) was not reviewed and not rerun.

## 5. Not done

- No rerun of l=4 Part 1, of Part 2 (b) or (c), or of `run_part1.py --validate`.
- No independent check of the `sym` CNF or the symbolic (e1,e2,e3) form.
- No exhaustive check of (a) beyond (3,11); the theorem was not re-derived or re-read.
- Reruns were on the same host and same solver build as the producer, so they test
  determinism, not independence of the instrument.

## 6. Verdict

- **Part 1: inconclusive (incomplete).** Weaker than stated on the formal verdict's footing
  (A/A floor invalid, margin 0.0005, l=5 missing) and on the SAT claim at l=4. The practical
  conclusion "no lead at these sizes" is supported: fixed-k gain at most 11%, best k = 1.
- **Part 2: inconclusive under the rule.** (a) is verified and is a theorem for prime n with
  2l-1 <= n, for the proxy w only. (b) is unresolved, so "nothing to synthesise" has not been
  established as registered.
