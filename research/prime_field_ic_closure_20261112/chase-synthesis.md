# Chase synthesis: five ideas, no improvement over the IC baseline

**Standing:** every number below is `[pcc:unverified]`. All of it comes from local pure-Python toy runs in `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/chase/`. No pcc claim was filed and no `bench.record/v1` record was written. Nothing ran on EKS, nothing was committed, and nothing passed the validation gate. None of the five workers queried research-kb or the lead atlas.

## Bottom line

- **No verified improvement over the current index-calculus baseline came out of this round.**
  - Two ideas give constant-factor gains only, and both were already built and measured in `crypto`: the CM-orbit fold and the paired translate.
  - Two ideas were closed by negative results that survived a re-run: Weil descent and the hybrid-collision family.
  - The equation-budget-A worker produced nothing.
- **The circling has a structural cause.** The hybrid-collision cost algebra shows why every explicit-table IC variant on a prime field costs at least about 1.6·√h × rho-neg, where h is the number of table hits needed. Tuning constants inside that family cannot reach rho.
- **Only the equation-budget gap could change the exponent.** That gap is a factor-base condition that adds equations without adding unknowns. Weil descent was the one route to it that got tested, and it is now closed. The other candidate conditions were never tested.

## What was measured, and what survived re-run

| Chase | What was measured | Verdict | How far the red-team re-ran it |
|---|---|---|---|
| **cm-orbit** (j=0 curves, base made of orbits under the order-6 automorphism) | m=2 IC at 16–22 bits, pair-table and Semaev oracles, against plain, negation and order-6 rho | Constant only | Partly. Regenerating the summary with `analyze.py` timed out after 300 s. The red-team checked that all 20,064 runs had no failures, that rho step counts match √(πn/2k), and found no math error in a code review. |
| **paired-translate** (x(R+T) and x(R−T) from one inversion) | Toy IC and rho at 18 and 20 bits, plus a projection onto the recorded incumbent at 20–32 bits | Constant only, and only at w=0 | Partly. The projection arithmetic and the reproduced w=1 S values were checked by hand. The toy itself was not re-run. |
| **equation-budget-A** (algebraic factor-base conditions) | Nothing. A safety check stopped the worker before any experiment. | No result | Nothing to re-run. The output folder does not exist. |
| **equation-budget-B** (Weil descent of E/GF(p) into GF(p^k)) | Descent decompositions for k=2 and 3, m=2 and 3, plus a subspace-base scaling test | Negative result survives | Yes, for 3 of 4 scripts. Output is identical to the stored runs. The long Groebner script was not re-run, and nothing depends on it. |
| **hybrid-collision** (IC sum tables combined with rho walks) | 20–24-bit toy runs, 40 paired instances per size, plus union theory | Negative result survives | Partly. The union theory and the 20-bit numbers reproduce. The 24-bit rows were not re-run. |

### Per-chase details

- **cm-orbit**
  - **Gain inside IC:** the CM-orbit base costs 2.2–2.6× less than the folded base under fermat pricing; batch pricing gives 2.06–2.36×.
  - **Net gain after the matched automorphism rho:** 1.35–1.59 under fermat pricing (overall range 1.25–1.62), below the predicted √3 ≈ 1.73. At 20 and 22 bits the 95% intervals exclude √3.
  - **Exponent:** IC cost grows about n^0.64, against n^0.51 for automorphism rho.
  - **Prior work:** this reproduces experiments E1, E6 and E12 in `/home/user/crypto/research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`, so it is not a new lead.
- **paired-translate**
  - **Prior work:** the trick already exists in `crypto` as sibling points, `sib`/`pool` (`CountedGroup::sibling_pair` in `src/cryptanalysis/ic_boundary.rs`).
  - **At w=0:** the projected IC/rho-neg ratio improves by a factor of 0.64–0.80. That gain comes from the second point and the probes being free, not from IC itself.
  - **At w=1:** no change.
  - **Fair reference:** a store-all rho with siblings gets the same gain of about 2× (0.50–0.52 measured), so the ratio does not move. Per §5a, the order is not stable across the 0–1 band, so it cannot be ranked. This matches the lab's existing verdict, "unresolved".
- **equation-budget-B**
  - **Route 1, base x ∈ GF(p):** these points all lie on E(GF(p)) or its quadratic twist. The extra k−1 descended equations only constrain the twist part, which carries no information about the DLP. Predicted and measured solution counts match on 12 of 12 targets.
  - **Route 2, subspace base outside GF(p):** this gives a genuine Gaudry/Diem descent, but in a group about p² in size. The cost then scales like p^1, against rho's p^{1/2}, which is an exponent loss.
  - **Unchecked:** the trace, norm and GHS remarks were written from memory.
- **hybrid-collision**
  - **IC side:** the only lever is stopping at the first dependency, and that is already the repo default (`incremental-gauss`, target-pinned).
  - **Collision union:** with an unknown-log table, 37–40 of 40 runs ended on a rho collision, so the union is rho plus table overhead. Calling it an IC gain would be relabelling.
  - **Known-log table:** this is BSGS plus rho. With a negation walk, the theoretical first-order gain is exactly zero, and the measured ratio was 0.98–1.14× rho-neg, with every interval including 1.

## Distance to rho

These rows come from different instruments, curve sizes and pricings. Do not compare them with each other or with the leaderboard.

| Source | Arm | Distance to matched rho |
|---|---|---|
| cm-orbit toy, 16→22 bits, w=0, fermat pricing | CM-orbit IC against order-6 rho | 4.6× → 7.9×, growing about n^{1/6} |
| hybrid toy, 20 / 22 / 24 bits, w=0 | Best real IC arm, first-dependency stop | 4.56× / 5.94× / 6.84× rho-neg |
| hybrid toy, 24 bits, w=1 | Same arm | 8.84–9.16× rho-neg |
| hybrid analytic bound | Any explicit-table member | At least 1.6·√h × rho-neg, with h growing with n |
| incumbent projection, 20→32 bits, w=0, with siblings | prime-abscissa, folded | 6.0× → 25.7× rho-neg (derived, not measured) |
| incumbent projection, 20→32 bits, w=1 | Same | 12.9× → 32.6× rho-neg, unchanged by siblings |

In every chase the distance grows with group size. Nothing here narrows it at scale.

## Why we keep circling

1. **Two of the five chases re-measured things the repo already had.** The CM-orbit fold is E1/E6/E12, and the paired translate is `sib`. Nobody checked research-kb or the lead atlas before spending compute.
2. **Three of the five are constant tweaks inside the generic explicit-table family.** The hybrid cost algebra and the S·T² ceiling in `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §1 put a hard floor under that whole family.
3. **The one direction that could change the exponent was barely worked on.** That direction is a cheap membership test, meaning a factor-base condition that adds equations without adding unknowns. Equation-budget-B closed one route to it. Equation-budget-A, which covered the other routes, never started.

## Single best next step

**Re-run equation-budget-A as a small, scoped pass with a kill criterion before any more constant-factor work.**

- **Scope:** the conditions B did not cover, which are the rational-map image and the fixed-degree root family. Galois-orbit restriction is in scope only where it differs from B's descent.
- **What to measure for each condition:**
  - Equations against unknowns per relation, counted on the E(GF(p)) component only. Twist or trivial components do not count.
  - Factor-base size as a power of p.
  - A degree-of-regularity check for m=3 and m=4 at 16–20-bit p, using sympy or python-flint. Both were already installed in that session.
- **Kill criterion:**
  - The condition must give more than one effective equation per relation, with a base of at most about p^{1/2} points, and decomposition cost must not blow up.
  - If no condition passes, record the equation-budget gap as closed for these routes and stop the prime-field IC constant-tuning line.
- **Precondition:**
  - Run `kb_status` and `search_knowledge` on these conditions, including the negative form ("not tried" / "fails"), and run the lead-atlas brief.
  - Log the negative results from this round (cm-orbit duplicates E1/E6/E12, siblings are band-dependent, Weil descent is closed, hybrid-collision is bounded) in the lead atlas so they are not chased again.
  - Find out what the safety check blocked in A's last step before reassigning it, so the new worker does not run into the same block.