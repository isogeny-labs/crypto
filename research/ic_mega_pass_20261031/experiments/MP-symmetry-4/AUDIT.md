# AUDIT MP-symmetry-4 (2026-10-31/11-01)
audit: partially_valid | claimed outcome: supports | corrected outcome: supports (no-excess falsifier did not fire), with one README claim refuted and several statistics overstated.
reran: partial (n=11 curves 0 and 2 only, l=4 exact + 7 fresh shuffles each; n=7 brute-force l=3; ~4 CPU-min). Full run.py NOT rerun (22 min).

## Code vs protocol (verified by reading + checks)
- Math correct: y=xz gives z^2+z=x+a+b/x^2, so lift iff Tr(x)+Tr(beta/x)=Tr(a). Identity arm uses an independent image table of z^2+z (does not use TR), so it also validates the trace table. Polys/seeds as in PROTOCOL. No off-by-one, wrong field, or wrong subgroup found.
- count_exact is a correct unique-RREF enumeration (pivots strictly increasing, new vector zero on old pivots, M &= M[idx^v] accumulates all span translates, Z[0]=False excludes dependence). Independently brute-forced l=3 at n=7: 162/162 and 63/63 match (README only validated l=2).
- Rerun n=11 curve0 l4 obs=14025, curve2 l4 obs=14686, identical to RESULTS.json; identity mismatch 0.
- count_sis is an unbiased importance estimator (weight = product of candidate counts).
- Null E=Gauss*prod(k-i)/(2^n-1-i) is the right hypergeometric mean per curve.
- Positive control real: planted dim-4 V into a shuffled Z, count 15285->17529.

## Findings
1. README claim REFUTED: "shuffled sets also sit below the hypergeometric null, so part of the l=4 deficit comes from the pairing". RESULTS shuffles: [14443,15255] vs null 15186; [40372,43384] vs 40252; [15080,14900] vs 15656. 3 of 6 are at/above null; pooled shuffle mean 71717 vs null 71094 (+0.9%) vs obs 67103 (-5.6% vs null, -6.4% vs shuffle). My 7 fresh shuffles: curve0 mean 15212 sd 528 (null 15186, obs 14025 = -2.2 sd); curve2 mean 15620 sd 786 (null 15656, obs 14686 = -1.2 sd). So the pair-preserving shuffle reproduces NO deficit; the ~8% l=4 deficit is specific to the true Z (not explained by orbit pairing or density). Class/outcome unchanged but the attribution in README and "next_step: attribute the deficit to pairing" are misdirected.
2. z-scores (-7.3, -35) use Poisson sd=sqrt(null) on counts of heavily overlapping subspaces; real dispersion is ~3-5% per curve at l=4 (shuffle sd 528/15212, 786/15620) versus Poisson 0.8%. z=-35 is invalid; the proper figure is ~-1 to -2 sd per curve, consistent sign on all 9 curves (0.89-0.95). Under the protocol's literal rule (Poisson 2-sigma CI of 1) l=3 and l=4 fail; outcome rests only on the "or matches 0.92x prior census" escape clause.
3. n=13 "no growth" is weak: SIS l4 ratio 0.966+-0.032 vs n=11 0.92 (exact) is 1.4 se apart; per-curve SIS se ~10-15%. Excludes the 1.5x falsifier easily, does not show the deficit vanishes. SIS l3 n=13 0.975+-0.011 is 2.3 se below 1 while exact on 2 curves is 0.9991, suggesting se is underestimated (heavy-tailed weights). n=13 l=3 exact only on 2 of 9 curves (documented).
4. sigma check is a tautology, not a test: for l>=2 sum of Tr(x) over V is 0, so all-lift forces Tr(beta*sigma_V)=Tr(a); 100% is guaranteed. Also all-lift V with Tr(a)=1 forces sigma_V!=0 (above data: sigma_zero=0 on all four Tr_a=1 curves; ~9e-4 = 2^-(n-1) on Tr_a=0 curves). README's "4.1e-4 matches 2^-n" is a pooled coincidence of that mixture. beta/V-subspace=0 is the expected null for a nonlinear map, uninformative.
5. Arm D deviates from PROTOCOL undisclosed in the amendment: protocol 2e5 nodes per dim target, code uses 2e4 total, 64-branch cap, 3 curves per n, one shuffle replicate, l* only ranks +-1 from randomized DFS (RNG-noisy: true 3 vs shuffled 4 at n=7, true 4 vs shuffled 5 at n=11). At n=17,19 both arms stop at 5 vs formula 6, so the arm has no power there; "never exceeds shuffled" is a non-test at n>=17. README admits budget limit but not the protocol deviation.
6. Runtime 1333 s exceeds 20-min cap (disclosed). Scale: l<=5 vs needed l~n/2 at n=131, so no bearing on lever 7 (disclosed).
7. Novelty/outcome class relabelling: consistent with brief. No security claim made. Good.

## Corrected verdict
Falsifiers (excess >=1.5 growing with n; l* >= null+2; beta/V-closed families) did not occur, so "supports" (no structure beyond a deficit) stands. But: (a) a real, orbit-shuffle-irreproducible ~8% l=4 deficit exists at n=11 (consistent with the 0.921x census), (b) significance numbers should not use Poisson z, (c) n=13 and l* arms are low power. Power of the whole experiment to detect a modest (<1.5x) excess is low at n>=13.
