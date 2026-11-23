# PROTOCOL: KM-ART-DEGEN (lab C, wave 2, 2026-11-15)

All numbers `[pcc:unverified]`. Local, 1 core, own Python implementation (`hky_eps.py`); no Rust touched.

Question. In the ring R_eps = F_p[z1,z2]/(z_i^K - eps), K = 2^t (tower rule y_{t-1}^2 = eps), measure the HKY last fall
degree d_F^eps and first fall degree ffd^eps of the m = 2 generator f, for t = 2..8.
Arms: eps = 1 (reference, real tower), eps = 0 (Artinian degeneration), eps = 5 (generic constant control).

Prediction (lead KM-ART-DEGEN, C1): d_F^0 = d_F^1 on every cell (point prediction: equality for t >= 4).
Declared alternative: d_F^0 constant in t while d_F^1 grows.

Instances.
- Primes p in {40961, 12289} (both 1 mod 2^12). Draws 0,1,2 per prime: curve a,b, base g, x_R from a seeded RNG
  (sha256 of "kmad|p|draw|try"); a draw is accepted if f = S3(g z1, g z2, x_R) has no zero on mu_K^2 at t = 8
  (hence at every t <= 8). Same (a,b,g,x_R) for every t and every eps.
- Families: tower (f = normal form of S3, support {0,1,2}^2) and W = 1 box (random coefficients in {0,1}^2, same acceptance).
- eps = 0: record whether f is a unit (f(0) != 0); skip the cell otherwise.
- Closure: V_{F,c} smallest space containing f and closed under g -> y*g for deg g < c; d_F = least c with 1 in V.
  c = 2,3,... up to cmax = 7 (columns of pop-sum <= c; t = 8 limited to cmax = 6 for memory). Censored cells reported as ">= cmax+1".
- ffd = least c >= 3 with dim(V_c cap R_{<=c-1}) > dim V_{c-1}.

Controls.
- Positive: eps = 1 reproduces the committed m = 2 values: tower d_F = 4,4,4,4,5,5,5,5,6,6 (t = 2..11), ffd = 4;
  W = 1 box d_F = 3,3,4,4,4,4,5,5,5,5, ffd = 3 (growth_law README sec. 5, devcheck).
- Exact twist identity: for eps = s^K (s random), d_F^{eps}(f) = d_F^{1}(f(s z)) for t <= 5 (isomorphism z = s w).
- Negative: planted consistent target (f vanishes at a grid point; x_R a root of S3 at a grid pair) never reaches 1 at eps = 1
  up to cmax, t <= 5.

Kill / survive rule (from lead). C1 survives if equality on >= 90% of completed cells and |d_F^0 - d_F^1| <= 1 on all.
C1 dies if |d_F^0 - d_F^1| >= 2 anywhere, or they differ on > 10% of cells. eps = 5: must match the generic value; the
mismatch is an engine defect not a result (checked by the twist identity, and by comparing d_F^5 with d_F^1 where f is a unit
in R_5). Cells where f is not a unit in R_5 (zero on the eps-grid over the closure field) never reach 1; reported as such.
