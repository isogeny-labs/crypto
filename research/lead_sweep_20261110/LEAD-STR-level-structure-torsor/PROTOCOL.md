# LEAD-STR-level-structure-torsor
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition open_next_step; family structure; title empty; cost "cheap-medium (exact checks N=5..47)"; why_it_might_beat_sqrt_p: "Expected closure; notes say dead by size".
- Excerpt: "Run the exact checks of the draft design (diamond action, Hecke forced zeros, separating-functio[n ...]" (truncated).
- Mechanism: X_1(N) diamond-operator recast: a point P of order N on E(F_p) is an F_p-point of Y_1(N); diamonds <a>:(E,P)->(E,aP) make the set of order-N points a (Z/N)^*/+-1 torsor; the DLP Q=kP asks which diamond relates (E,P),(E,Q).
- Atlas note: "genus ~ N^2/24, so it is dead by size"; one proof obligation (separating-function degree over F_p when N ~ p), no record yet.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 55,105,261); README.md (same dir, ~226); docs/session-logs/2026-11-01.md (~3617). Cryptanalysis repo not searched.
## 2. Hypothesis
For prime-order ECDLP, N ~ p, the recorded size obstruction holds exactly: g(X_1(N)) = (N-5)(N-7)/24 (prime N) ~ N^2/24, so for every N > p/2 in the Hasse range the Hasse-Weil bound p+1+2g*sqrt(p) is >p, i.e. no point-count or function-field certificate on X_1(N)/F_p can discriminate diamond translates at cost below p; the observed #Y_1(N)(F_p) is far below that bound.
## 3. Preregistered prediction and falsification criterion
- Statistics: (a) r(N)=g(X_1(N))/(N^2/24) for prime N=29..47 (exact formula); (b) for toy p in {31,53,79,101,127} and prime N in the Hasse interval, check 2*g*sqrt(p) > p whenever N>p/2; (c) exact #Y_1(N)(F_p) by brute force.
- Prediction: 0.45 < r(N) < 1.0 for all prime N in 29..47, and (b) holds for every row. consistent_with_prediction if both; falsified if either fails (a failure of (b) would mean a Hasse-Weil route is not vacuous at toy size and would reopen the lead); inconclusive if controls fail.
- Positive control: genus formula reproduces known genera g(X_1(N)) for N=5,7,10,11,13,17,19,23,29,37 (0,0,0,1,2,5,7,12,22,40). Negative control: no order-N point curves exist for N outside the Hasse interval (checked at p=31, N=5, 11). A/A: the brute-force count for the first (p,N) is recomputed and must match. No randomness, so no seeds.
## 4. Method
Pure stdlib. genus via g=1+mu/12-cusps/2 with mu=N^2 prod(1-q^-2)/2 and cusps from sum phi(d)phi(N/d). #Y_1(N)(F_p) = #{(a,b): #E_{a,b}(F_p)=N}*(N-1)/(p-1) (N>=5 prime, fine moduli space, so the (E,P) classes are counted as (a,b,P)/(p-1)). Brute force over all (a,b) in F_p^2 with a Legendre-symbol table: about p^3 operations, p<=127 gives ~2e6 per N, a few primes N per p; total well under 1 minute, <50 MB. `--smoke` uses p in {31,53}.
## 5. Classification: runnable_local
Pure integer/formula computation, stdlib only, deterministic, seconds to a minute.
## 6. What a run cannot establish
It checks only the genus-size arithmetic and toy point counts; it does not verify the Hecke forced-zeros or separating-function degree for N~p at cryptographic size, does not test the diamond action beyond the (a,b,P) counting identity, and does not exclude non-Hasse-Weil algorithms on X_1(N) (e.g. Jacobian methods are covered by the genus-g floors elsewhere in the atlas). Results are labelled [pcc:unverified].
