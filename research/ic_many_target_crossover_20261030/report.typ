#set page(margin: 1.6cm, paper: "a4")
#set text(size: 9.5pt)
#set table(stroke: 0.4pt + gray)

= Many-target crossover: amortised compact-orbit IC against batch rho (2026-10-30)

*Verdict: dead where it can be ranked ($n = 41..61$); undecidable at $n = 73, 83$ only for want of counts, and the $n = 83$ wall-clock
evidence points the same way.* Amortising relation collection over $k$ targets never makes IC cheaper than batch rho. With
$G = min_k "IC"(k) / "batch rho"(k)$: clock-priced `ic price` units give $G = 3.0$ to $4.1$ at $n = 41..61$ (best swept recipe); no size has $G < 1$;
$G$ is flat to rising above $2^36$. The §5a count band (IC at its emitted additions, a lower bound; rho at 0, 0.1, 1 GAE per canonicalisation) keeps
$G > 1$ at $n = 41, 43, 53, 61$ ($3.1$ to $4.8$ at $w = 0$, $1.6$ to $2.4$ at $w = 1$); at $n <= 45$ and $n = 47$ IC's descent and construction counts are not
emitted, so those sizes are unresolved by counts. $n = 83$ (wall seconds, practicality only): $G = 4.8$ to $5.1$, and a precomputation-rho
table with the same setup time has a smaller per-target cost than IC at every $k$. Class: accounting. Not an improvement to the IC baseline.

*Model.* $"IC"(k) = S_0 + k d$; $"batch rho"(k) = rho_1 K(k)$ with $K(k) = sum_(i<k) binom(2i,i) 4^(-i) approx 2 sqrt(k/pi)$
(Kuhn--Struik law as implemented in `src/bin/ic/rho.rs`, $K(32)/32 = 0.19869$). A window exists iff
$sigma delta < 0.318$ with $sigma = S_0 / rho_1$, $delta = d / rho_1$; $G approx 1.773 sqrt(sigma delta)$. The product $S_0 d$ must fall by $G^2$
(9 to 17x at $n = 41..61$, about 23x at $n = 83$).

#figure(image("g_vs_log2r.svg", width: 85%), caption: [Gap $G$ against $log_2 r$. Blue: k=32 recipe with 95% bootstrap; orange: best swept recipe; green: n=83 wall (K3300). The red line $G=1$ is where amortised IC would win.])

#table(columns: 7, align: (left, right, right, right, left, right, right),
  [curve], [log2 r], [S0/rho1], [d/rho1], [G [95%]], [k_opt], [G best recipe],
  [K1/2^19], [18.0], [95.1], [1.903], [23.90 [19.74, 28.10]], [50], [23.64],
  [K1/2^23], [22.0], [51.0], [0.904], [12.06 [7.38, 18.62]], [57], [12.00],
  [K1/2^45], [24.8], [124.5], [0.784], [17.52 [14.15, 21.48]], [159], [17.35],
  [K0/2^37], [27.8], [40.1], [0.587], [8.62 [6.70, 10.73]], [69], [8.50],
  [K1/2^43], [32.1], [36.3], [0.477], [7.39 [5.91, 9.09]], [76], [6.11],
  [K1/2^47], [36.6], [33.6], [0.261], [5.25 [4.34, 6.21]], [129], [3.53],
  [K0/2^41], [39.0], [25.1], [0.170], [3.66 [2.84, 4.85]], [148], [2.98],
  [K0/2^53], [44.3], [55.7], [0.089], [3.95 [2.22, 5.81]], [626], [3.52],
  [K0/2^61], [47.2], [80.8], [0.182], [6.80 [5.36, 9.49]], [444], [4.10],
)

*Measured vs extrapolated.* Measured: $S_0$, $d$, $rho_1$ for $n=19..61$ (clock-priced `ic price` units and emitted counts, same process, 4 target sets x 9 sizes
plus the recipe sweep); $n=83$ $S_0$, $d$ (16 targets), $rho_1$ (16 plain-rho runs) in CPU seconds on one runner per row.
Ledger only: $n=73$ (one target, $rho_1$ scaled from $n=83$): $G = 15$ to $22$ for the untuned K600 recipe. Extrapolated: the trend fit
(slope $-0.054$ per bit, $G(73) = 2.3 [1.6, 3.0]$, $G(83) = 2.7 [2.0, 3.5]$), which is not relied on: the top four sizes are flat to rising.

*n = 83, total CPU seconds for $k$ targets (run 2, K3300).* IC 825 + 0.049 k; BL table (a): 651 + 0.0042 k with a 0.3 GB table; batch rho
2.375 K(k): batch rho is cheapest at every $k$ from 1 to $10^7$ and BL(a) dominates IC at every $k$.

*Not run:* the equal-advice instrument (needs 24 to 48 GiB, 40 to 100 core-hours); counted $S_0$ at $n = 73/83$ is not emitted by the producer.
See README.md for the model, uncertainty method, section 5a band, literature check (one gap: Kuhn--Struik text unreadable here), and limits.
