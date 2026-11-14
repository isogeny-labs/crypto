# LEAD-ICB-ecc2k130-jacobian-carrier

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-30). Family: ic-binary.
- Title: ECC2K-130 via a Jacobian carrying the trace-zero abelian variety A (Torelli rarity heuristic).
- Next-step excerpt (truncated in leads.json): "Run the toy ladder at the first faithful rungs n = 11 and n = 13 (2 primitive mod n, A simple, n..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (one-line table row only).
  No lead-specific note was found; the other hits are tally/inputs JSON. Mechanism below is from my own knowledge, not from a recorded derivation.

## 2. Hypothesis
Let E be a Koblitz curve y^2+xy=x^3+ax^2+1 over F_2 (a in {0,1}) and A the trace-zero part of the Weil restriction of E from F_{2^n} to F_2 (dimension n-1, Frobenius eigenvalues the alpha with alpha^n = pi^n, alpha != pi). For the faithful rungs n = 11 and 13 (2 primitive mod n), the point counts a genus-(n-1) curve C with Jac(C) ~ A would need (N_k = 2^k+1+t_k for k<n, N_n = 2^n+1-(n-1)t_n, t_k = tr(pi^k)) pass the cheap necessary conditions: every degree-d place count B_d (Moebius inversion of N_e, d<=n) is a non-negative integer and the Weil bound holds. I.e. no cheap point-count obstruction excludes a carrier Jacobian at these rungs, so the Torelli rarity heuristic (expected carrier curves ~ 2^(3g-3 - g(g+1)/2), 2^-28 at g=10, 2^-45 at g=12) is the only obstruction.

## 3. Preregistered prediction and falsification criterion
- Statistic: boolean "places_ok and weil_ok" for (a in {0,1}) x (n in {11,13}); four cells.
- Prediction: all four cells pass. Falsified if any cell fails (that rung has a rigorous point-count obstruction, a stronger and cheaper reason than rarity). Passing decides only "not cheaply excluded", never existence.
- Negative control (null): random count vectors N_k drawn uniformly within the Weil band, same n; pass rate must be < 5% for every n (shows the test discriminates).
- Positive control: random genus-2 curves y^2+hy=f over F_2 counted over F_2 and F_4 by brute force; the test must accept at least some (smooth ones always pass; singular draws may not, so only "> 0 accepted" is required).
- A/A arm: the negative control repeated with two seeds (20261110, 20261111) at n=11; pass rates must differ by < 0.01, else verdict inconclusive.
- Verdict "inconclusive" if any control misbehaves.

## 4. Method
Pure stdlib, deterministic. For n in {5,7,11,13,17,19,23}, a in {0,1}: tau-recurrence t_k = mu t_{k-1} - 2 t_{k-2} (mu = +1 for a=1, -1 for a=0), build N_k, check B_d >= 0 integral for d = 1..n and |N_k - 2^k - 1| <= 2g 2^(k/2). Also record log2 Torelli ratio (3g-3) - g(g+1)/2 and whether 2 is primitive mod n. Seeds 20261110, 20261111, 7000+n, 424242. 20000 null trials per n. Cost: well under 1 minute, < 20 MB.

## 5. Classification: runnable_local
A stdlib integer computation decides the necessary-condition question exactly; no GPU, Groebner or builds needed.

## 6. What a run cannot establish
- Passing necessary point-count conditions does not show a genus-(n-1) curve with Jacobian isogenous to A exists (A need not even be principally polarised, nor a Jacobian); failing would exclude it rigorously.
- Says nothing about ECC2K-130 at n=131 itself, cover/decomposition cost, or any speedup over rho; the Torelli figure is a heuristic dimension count, not a theorem.
- The rungs are toy proxies; no matched rho, so no leaderboard relevance. Results are [pcc:unverified].
