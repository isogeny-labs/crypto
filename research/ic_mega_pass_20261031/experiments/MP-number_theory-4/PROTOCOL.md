# MP-number_theory-4 PROTOCOL (pre-registered 2026-10-31, written before any run)
Hypothesis: degree-d (d<=6) polynomial filters f on x(kG), bucket [0,p/256), show no structured excess (Weil-type bilinear bound).
Scope: prime field F_p only. KN-OPEN-91d4f6 (char 2, F_2-linear functionals) is NOT tested. Toy evidence only.
Setup: p = next prime above 2^k, k in {14,16,18,20,22}; random E: y^2=x^3+ax+b of prime order N (PARI ellcard); enumerate x(kG), k=0..N-1 (baby/giant numpy).
Statistic: F[k]=1[f(x_k) mod p < p/256] (k>=1; x_0 := inf -> 0); lambda_f = max_{j!=0}|FFT(F)_j| / sqrt(N*mu_hat), mu_hat = mean(F).
Arms: REAL; NULL-1 = x permuted randomly (marginal kept, DL order killed); NULL-2 = iid Bernoulli(mu_hat).
Search: random-restart hill climber over integer coefficients of f (deg d), budget 40 evaluations per (p,d,arm) at k<=18, 16 at k>=20; same budget on REAL and NULL-1. NULL-2: 16 fresh masks, report max.
Held-out: best REAL f re-evaluated on a second independent curve at same p (seed+1).
Controls: positive = planted: degree-2 f mask with 5% of bucket members re-assigned according to cos(2pi j0 k/N)>0 (correlation planted at frequency j0); must give lambda >> null. d=1 f(x)=x is the known-pseudorandom control.
Theta sweep (p=2^20): A,B random / interval / adversarial-top-s (A = top |A| rows by hit count out of a 4|A| pool) of size p^theta, theta in {0.25,0.35,0.45,0.55}; report bucket mass ratio = hits/(|A||B| mu) and z-score; same on NULL-1.
Seeds: numpy default_rng(20261031 + k*100 + d).
Pre-registered decision:
 SUPPORTS (prediction_if_true): REAL lambda (and held-out) within 1.5x of NULL-2 for all d<=6, flat/falling in p.
 REFUTES (prediction_if_false): REAL/NULL-2 >= 3x persistently on held-out curves, not shrinking over 2^14..2^22.
 otherwise INCONCLUSIVE. Planted control must exceed 3x null or the run is invalid.
Not tested (declared): coding_comb-2 even-character / 16|#E / M=2 merge-gain arm (requires prime order N incompatible with 16|#E; deferred).
Budget: <20 CPU-min.
