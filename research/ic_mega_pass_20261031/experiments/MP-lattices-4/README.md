# MP-lattices-4: result = supports (engineering, reach unchanged)
- Identity (p=nextprime(2^32), 50 trials x 16 masks): Y(P+T)=-Y(P) 800/800; even flips keep the sum, odd flips add T. Negative control (wrong c): the identity fails 50/50. D4 rewrite support is 105 monomials.
- Reach (same JM componentwise code, all arms): D4 beats the S4 e(x) arm at matched l by 4.7x, 1.26x, 1.12x, 1.08x, 1.06x for l=1..5. That is a finite-l edge only, and it decays. The Monte Carlo asymptote is 0.0125 = 1/80 for x-box, S4e and D4 (ratio 1.0003). The pre-registered >10% lever rule fires at l<=3 only and does not hold asymptotically, so it is not a lever. It stays under 1/32 and well under the 0.169 needed.
- Dimension: D4 is 4.7-7.0x smaller than S4e and tends to 8 (free-orbit limit). That is the constant-factor engineering gain.
- Claim B: Mobius, x+k and 1/x reparametrisations give the same polytope, so reach is the same. This is trivial for degree-preserving maps only. Rank>=2 GAP substitutions were not tested.
- Caveats: the code uses the JM componentwise-lead strategy, not the programme's min-cut closure, so absolute finite-l values differ from 0.005253/0.0144. No fplll finite-p run, no log-det at 2^130/2^256.
- Next: cross-check against the min-cut code; run fplll l=1 dim-105 at p~2^256, 8 seeds, with a null xR arm.
