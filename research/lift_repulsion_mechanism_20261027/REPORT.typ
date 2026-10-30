#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Why lift sets repel linear structure: Kloosterman moments

Study `research/lift_repulsion_mechanism_20261027/`, 2026-10-27.
- Run: `reports/run-20261027T1459Z/` (748 sets, exact counts), master seed
  `0x11F7_2026_1027_C` (the parent's, so the `orig` population is the parent's).
- Protocol: `PROTOCOL.md`, frozen before the run; sha256 of protocol, source and
  `Cargo.toml` in `FREEZE.sha256` (verified after the run).
- Harness: `src/main.rs`, std-only Rust. Field arithmetic, lift set, null generator and the
  count-mode subspace DFS are copied from `research/lift_dense_factor_base_20261027/src/main.rs`;
  the parent's 104 `orig` ratios are reproduced with max abs difference 0.0.
- Plotting: `plot.py`, layout only.

Each statement is tagged *theorem*, *exact computation*, *measured* or *not established*.

== Question and short answer

Parent finding (measured): on $E: y^2 + x y = x^3 + a x^2 + b$ over $FF_(2^N)$, $N$ prime, the set
$L$ of lifting abscissae contains fewer all-lift $k$-dimensional $FF_2$-subspaces than a uniform
random set of equal size, and its count scatters less. At $k = 4$ the normalised counts are
0.68, 0.92, 0.98 at $N = 9, 11, 13$.

Hypothesis tested: the cause is the Kloosterman (Sato--Tate) fourth moment of the Walsh spectrum,
2 instead of the Gaussian 3, which gives fewer additive quadruples and fewer all-lift subspaces.

*Verdict.* The premise is *a theorem*: every curve has the same Walsh power sums
($sum W^4 = 2 q^3 - 2 q^2 - 3 q$, normalised 4th moment $2 - O(1\/q)$), so curves have exactly
the same, far-below-random, number of additive quadruples. A first-order expansion of the subspace
counts in the 3-point and 4-point correlations is exact at $k = 2$ and explains about 70 % of the
$k = 4$ curve deficit. The mean shift comes from the 4th moment, the Tr$(a)$-dependent offset from
the 3rd, and the smaller scatter from the 3rd moment (not the 4th). About 30 % of the odd-$N$
deficit is *not established*. Composite $N = 12$ is not explained by subfield cancellation.

#figure(table(columns: 4,
  [id], [rule], [verdict], [numbers],
  [I1, I2], [exact Walsh 4th and 3rd power sums on every curve], [*pass*], [120/120 curves],
  [P1], [curves have lower Walsh 4th moment], [*pass*], [2.0010 vs 2.980 at N = 11; d = 13.7; p = 1e-5],
  [P2], [ratio#sub[4] rises with the 4th moment, pooled and within nulls], [*fail*], [within nulls: 0.195 / 0.029 / −0.118 (N = 9 / 11 / 13)],
  [P3a], [$A_2 = E_2 + D_"line"$ exactly], [*pass*], [exact for all 748 sets],
  [P3b], [first-order $k = 4$ within factor 2, $R^2 >= 0.5$], [*pass*], [pred/obs 0.885 / 0.705 / 0.683; $R^2$ 0.61 / 0.92 / 0.96],
  [PC], [planted Walsh bias raises the count], [*pass*], [Spearman 0.944 / 0.998 / 0.994]),
  caption: [Preregistered verdicts, computed by the program (`summary.json` → `verdicts`). P2 fails by the rule, as predicted in the protocol.])

== Derivation <sec-derivation>

Throughout, $q = 2^N$, $u = q - 1$, $c = sqrt(b) != 0$, $epsilon = (-1)^("Tr"(a))$.
Put $psi(x) = (-1)^("Tr"(x + a + b\/x^2))$ for $x != 0$ and $psi(0) = 0$. Then
$psi = +1$ exactly on the lift set $L$ and $-1$ on $FF_q^* without L$. Write
$W(w) = sum_x psi(x) (-1)^(w dot x)$ (the bit dot product is a non-degenerate
form, so every power sum below equals the one for $"Tr"(w x)$).

*Tag: theorem (proved here by hand; each identity is also checked as exact
integer equality on every curve of the run).*

=== Walsh spectrum is a Kloosterman spectrum

Since $"Tr"(b\/x^2) = "Tr"(c\/x)$,
$ psi(x) = epsilon (-1)^("Tr"(x + c\/x)), quad
  W(w) = epsilon K(1 + w; c), quad K(t; c) = sum_(x != 0) (-1)^("Tr"(t x + c\/x)). $
The substitution $x -> x\/t$ gives $K(t; c) = K(1; t c)$, so as $t$ runs over
$FF_q^*$ the pair $(t, c)$ runs over every Kloosterman sum $K(lambda) := K(1; lambda)$
exactly once. $K(0; c) = -1$. All curves with the same $N$ therefore share one
Walsh spectrum up to translation $w -> w + 1$ and the global sign $epsilon$.

=== Power sums of the Walsh spectrum

*Proposition 1.* For every $N$, $a$ and $b != 0$:
+ $sum_w W(w)^2 = q u$ (Parseval; true for every $psi$ with $psi^2 = 1_(x != 0)$).
+ $sum_w W(w)^3 = q C_3$ with $C_3 = -epsilon (q - 2)$ for $N$ odd and
  $C_3 = epsilon (q + 2)$ for $N$ even.
+ $sum_w W(w)^4 = q (2 q^2 - 2 q - 3)$.

_Proof of 2._ $sum_w W^3 = q sum_(x + y + z = 0) psi(x) psi(y) psi(z)$. For
$x, y != 0$, $x != y$ and $z = x + y$ the linear traces cancel,
$"Tr"(x) + "Tr"(y) + "Tr"(x + y) = 0$, so the product is
$epsilon (-1)^("Tr"(c (1\/x + 1\/y + 1\/(x + y))))$. Put $y = t x$ with
$t in FF_q without {0, 1}$: the exponent is $"Tr"(c h(t)\/x)$ with
$h(t) = (t^2 + t + 1)\/(t (t + 1))$. Summing over $x != 0$ gives $-1$ when $h(t) != 0$
and $q - 1$ when $h(t) = 0$. $h$ vanishes exactly at the primitive cube roots of
unity, which lie in $FF_q$ iff $N$ is even. Odd $N$: $sum_t = -(q - 2)$. Even
$N$: $2 (q - 1) - (q - 4) = q + 2$. $square$

_Proof of 3._ $K(t; c)^2 = sum_s (-1)^("Tr"(t s)) G(s)$ with
$G(s) = sum_(x + y = s, x y != 0) (-1)^("Tr"(c (1\/x + 1\/y)))$. Then
$G(0) = u$. For $s != 0$ put $x = s z$, $z in.not {0, 1}$:
$G(s) = sum_z (-1)^("Tr"(lambda\/(z^2 + z)))$ with $lambda = c\/s$. The map
$z |-> z^2 + z$ is two-to-one from $FF_q without {0, 1}$ onto
${v != 0 : "Tr"(v) = 0}$, and $2 [ "Tr"(v) = 0 ] = 1 + (-1)^("Tr"(v))$, so
$ G(s) = sum_(v != 0) (-1)^("Tr"(lambda\/v)) + sum_(v != 0) (-1)^("Tr"(v + lambda\/v)) = -1 + K(lambda). $
Parseval on $t in FF_q$ gives $sum_t K(t; c)^4 = q sum_s G(s)^2 = q (u^2 + sum_(lambda != 0) (K(lambda) - 1)^2)$.
Now $sum_(lambda != 0) K(lambda) = sum_(v != 0) (-1)^("Tr"(v)) sum_(lambda != 0) (-1)^("Tr"(lambda\/v)) = 1$
and $sum_(lambda != 0) K(lambda)^2 = q u - 1$ (part 1 with the $t = 0$ term removed).
Hence $sum_(lambda != 0) (K - 1)^2 = q u - 1 - 2 + u = q^2 - 4$ and
$sum_w W^4 = q (u^2 + q^2 - 4) = q (2 q^2 - 2 q - 3)$. $square$

*Consequences.* For a curve the normalised 4th moment is
$kappa = sum W^4\/(q u^2) = 2 - O(1\/q)$, identical on every curve: the Sato--Tate
value 2, with no scatter. A random $plus.minus 1$ function has $kappa = 3$ (the
Gaussian value; exactly $3 - O(1\/q)$ for a uniform balanced one). The same
power sums, written as correlations of $psi$ over linear relations
($x + y + z + w = 0$ etc.), are:
$ C_3 = sum_(x + y + z = 0) psi psi psi = -epsilon (q - 2), quad
  R_4 = sum_(x + y + z + w = 0) psi psi psi psi = 2 q^2 - 2 q - 3. $
The $R_4$ sum contains degenerate solutions ($x = y$ and $z = w$, and the two
other pairings): $3 u^2 - 2 u$ of them, each contributing $+1$. The sum over
four distinct nonzero elements is therefore
$ C_4' = R_4 - 3 u^2 + 2 u = -(q - 2)(q - 4). $
A random sign pattern has $E C_4' = 0$ and sd $tilde q^(3\/2)$. So a curve has
$C_4' tilde -q^2$, which is $tilde 9$ standard deviations below random at $N = 11$
(§Results, $z_"plane"$), and it is the same for every curve.

=== Exact expansion of the subspace counts

Let $L$ have $m$ elements, $p = m\/u$, $e(x) = 1_L(x) - p$ on $x != 0$, $e(0) = 0$
($sum e = 0$). For a $k$-dimensional $V$ with $n_k = 2^k - 1$ nonzero points,
$product_(x in V^*) (p + e(x)) = sum_(T subset.eq V^*) p^(n_k - |T|) e(T)$, with
$e(T) = product_(x in T) e(x)$. Summing over $V$ and grouping the sets $T$ of
distinct nonzero elements by size $t$ and rank $r$ (the number of $k$-spaces
containing $T$ is $N_k(r) = binom(N - r, k - r)_2$):
$ A_k = sum_t p^(n_k - t) sum_(T, |T| = t) e(T) N_k("rank" T). $
For a uniform random $m$-subset the same identity holds with $e(T)$ replaced by
its expectation $epsilon_t$ (it depends on $t$ only), giving the exact $E_k$.
Subtract, and use that $sum_(|T| = t) (e(T) - epsilon_t) = 0$ for every $t$ (the
elementary symmetric polynomial of the multiset of $e$-values is the same for the
observed set and for a random one). Sets of $t$ distinct nonzero vectors are
independent for $t <= 2$, so with $D_c = sum_(T in c) (e(T) - epsilon_t)$ over
the dependent classes $c$ (rank $r_c < t$):

*Proposition 2 (exact).*
$ A_k - E_k = sum_(t >= 3) p^(n_k - t) sum_(c "dependent, size" t) D_c (N_k(r_c) - N_k(t)). $

The smallest dependent classes are the *lines* $T = {x, y, x + y}$ ($t = 3$,
$r = 2$) and the *planes* $T = {x, y, z, x + y + z}$ ($t = 4$, $r = 3$).
Their centred correlations are (Fourier inversion, $hat(e)$ the Walsh transform of $e$)
$ D_"line" = frac(1, 6 q) sum_w hat(e)(w)^3 - frac(u (u - 1), 6) epsilon_3, quad
  D_"plane" = frac(1, 24) [ frac(1, q) sum_w hat(e)(w)^4 - 3 P_2^2 + 2 P_4 ] - frac(u (u - 1)(u - 3), 24) epsilon_4, $
with $P_j = sum e^j$ (the bracket removes the degenerate solutions as above).
Since $hat(e)(w) = (W(w) + sigma)\/2$ for $w != 0$, $hat(e)(0) = 0$, $sigma = s\/u$,
$s = W(0) = 2 m - u$, these reduce to the power sums of Proposition 1 plus
terms of relative size $O(s^2\/q)$:
$ D_"line" = frac(C_3 + 3 s - s^3\/q, 48) + O(sigma), quad
  D_"plane" = frac(R_4 - 3 u^2, 384) + O(q) = - frac(q^2, 384) (1 + O(1\/q)) "for a curve". $

*(i) k = 2: exact, one term.* There are no classes with $t >= 4$ when $n_2 = 3$, and
$N_2(2) - N_2(3) = 1 - 0$, so
$ A_2 = E_2 + D_"line", quad 48 A_2 = u(u - 1) + 3 s (u - 1) + 3 (s^2 - u) + C_3. $
The term that carries the deviation from random is the 3-point correlation
$C_3 = (1\/q) sum W^3$ alone. For a curve $C_3 = -epsilon (q - 2)$, so
$A_2\/E_2 - 1 approx -epsilon\/q$: a part in $10^(-3)$ at $N = 11$, sign set by
$"Tr"(a)$. This is the alternating $1 plus.minus 0.0005$ seen in the parent
data. It is not the 8 % effect.

*(ii) k = 3.* Two classes with the next largest coefficients,
$ A_3 - E_3 approx p^4 D_"line" (N_3(2) - 1) + p^3 D_"plane" $
(the omitted classes have $t >= 5$). For $p = 1\/2$ and a curve this gives
$A_3\/E_3 - 1 approx -(7 + 7 epsilon)\/q$ (odd $N$).

*(iii) k = 4.* The same two classes, with $N_4(3) approx q\/8$ and
$N_4(2) approx q^2\/96$:
$ A_4 - E_4 approx p^(12) D_"line" (N_4(2) - N_4(3)) + p^(11) D_"plane" (N_4(3) - 1). $
With $p = 1\/2$, $E_4 = binom(N, 4)_2 2^(-15)$, $binom(N, 4)_2 approx q^4\/20160$:
$ frac(A_4, E_4) - 1 approx underbrace(-frac(105, q), "plane: 4th moment") underbrace(- frac(35 epsilon, q), "line: 3rd moment") quad (N "odd"), $
and $-(105 - 35 epsilon)\/q$ for $N$ even, where $C_3$ changes sign.
The 4-point (plane) term is the mean shift and is the same for every curve; the
3-point (line) term moves the curve by $plus.minus 35\/q$ with $"Tr"(a)$ and,
for random sets, is the largest source of scatter: $z_"line"$ is $N(0, 1)$ for a
null, but for a curve it is the constant $plus.minus 0.41$.

== Results <sec-results>

All numbers are copied from `summary.json` / `rows.jsonl`. Populations: `orig` (the parent's
26 + 26 sets at $N = 9, 11$, 6 + 6 at $N = 13$), `rep` (fresh seed: 24 + 24, 4 + 4), `extra` nulls
(150, 150, 40) and planted sets. $N = 12$ is a descriptive contrast.

=== P1: the 4th moment (exact)

Every curve at $N = 9, 11, 12, 13$ has exactly $sum W^4 = 2 q^3 - 2 q^2 - 3 q$ (one distinct value
per $N$: 267909632, 17171474432, 137405386752, 1099377385472) and the matching $sum W^3$.

#table(columns: 6,
  [$N$], [curves, kurt], [paired nulls, kurt (sd)], [difference, 95 % CI], [Cohen $d$], [perm. $p$],
  [9], [2.003902405398264], [3.017898522141077 (0.259)], [1.01400 [0.9449, 1.0883]], [5.54], [1.0e-5],
  [11], [2.0009763236153613], [2.979949697019886 (0.101)], [0.97897 [0.9506, 1.0065]], [13.65], [1.0e-5],
  [13], [2.0002441257165615], [2.9997707820642376 (0.051)], [0.99953 [0.9700, 1.0300]], [27.69], [1.0e-5],
  [12], [2.0004882215871227], [3.00965630381015 (0.064)], [1.00917 [0.9705, 1.0443]], [22.42], [1.0e-5])

The curve value has sd $tilde 10^(-16)$: all curves coincide. The Cliff's $delta$ is 1 at every $N$.

=== Standardised correlations: how far below random

With $z_"plane" = D_"plane" \/ (sqrt(N_"planes") (p(1-p))^2)$ and the analogous $z_"line"$:

#table(columns: 5,
  [$N$], [$z_"plane"$ curves], [$z_"plane"$ nulls (sd)], [$z_"line"$ curves (range)], [$z_"line"$ nulls (sd)],
  [9], [−4.604], [0.101 (1.014)], [−0.442 to 0.445], [1.009],
  [11], [−9.230], [−0.084 (0.987)], [−0.427 to 0.426], [1.061],
  [13], [−18.472], [−0.106 (0.986)], [−0.407 to 0.417], [1.093],
  [12], [−13.059], [0.343 (1.280)], [0.398 to 0.422], [1.102])

The curves' plane correlation is $tilde 0.204 sqrt(q)$ standard deviations below random (4.6, 9.2,
18.5) and varies by $< 0.013$ across curves. The line correlation is two-valued, $plus.minus 0.41$,
with sign $-epsilon$ (odd $N$).

=== The ratios and the first-order formula

#table(columns: 7,
  [$N$], [curves ratio#sub[4]], [paired nulls ratio#sub[4]], [observed deficit × q], [first-order × q], [residual × q], [pred/obs],
  [9], [0.7013], [1.0190], [152.9], [135.4], [17.5], [0.885],
  [11], [0.9223], [1.0164], [159.1], [112.2], [46.9], [0.705],
  [13], [0.9812], [0.9942], [153.9], [105.1], [48.9], [0.683],
  [12], [0.9885], [0.9983], [47.2], [67.6], [−20.4], [1.432])

- The observed curve deficit is $tilde 155\/q$ at all three prime $N$: it is a $1\/q$ effect.
- The first-order prediction approaches its asymptote $105\/q$ (the plane term; the line term
  averages to zero over Tr$(a)$): 135, 112, 105.
- The residual is $tilde 48\/q$ at $N = 11, 13$ (not $O(1\/q^2)$). It is the same for both Tr$(a)$
  classes. Splitting by Tr$(a)$ at $N = 11$: observed deficit 0.0939 ($a = 0$) and 0.0614 ($a = 1$),
  predicted 0.0722 and 0.0373. The split 0.0325 matches the predicted 0.0349.
- $k = 2$: $A_2\/E_2 - 1 = D_"line"\/E_2$ exactly. Curves: $1 plus.minus 0.0005$ at $N = 11$
  (sd 0.0005), $+1\/q = +0.000242$ at $N = 12$ (all $epsilon = +1$ there).
- $k = 3$: curve deficit observed / first-order: 0.015141 / 0.015081 ($N = 9$), 0.003566 /
  0.003517 ($N = 11$), 0.000853 / 0.000844 ($N = 13$); all-set $R^2$ 0.965, 0.993, 0.998.
- Over nulls the $k = 4$ fit of observed on predicted has slope 0.947, $R^2 = 0.924$ ($N = 11$)
  and slope 0.986, $R^2 = 0.983$ ($N = 13$). Over curves alone: $R^2 = 0.903$ and 0.968. Figure 3 shows
  nulls on the diagonal and odd-$N$ curves $tilde 48\/q$ below it.
- The exploratory A#sub[3]-aware version (rank-3 block replaced by the measured $A_3$ deviation)
  raises the null $R^2$ to 0.986 ($N = 11$) and 0.999 ($N = 13$) but the curve deficit only from
  0.0548 to 0.0555: the rank-3 classes are not what is missing. What is missing lies in rank-4
  dependent classes ($t >= 5$), which were not computed.

=== P2: correlation with the 4th moment, and what carries the scatter

#table(columns: 5,
  [$N$], [pooled Spearman (95 % CI), perm. $p$], [pooled Pearson], [within nulls Spearman, $p$], [within curves],
  [9], [0.444 (0.332, 0.544), 1.0e-5], [0.480], [0.195, 0.0029], [undefined (sd 0)],
  [11], [0.435 (0.322, 0.536), 1.0e-5], [0.589], [0.029, 0.342], [undefined],
  [13], [0.302 (0.036, 0.527), 0.0098], [0.553], [−0.118, 0.794], [undefined],
  [12], [0.048 (−0.223, 0.313), 0.355], [0.146], [−0.061, 0.664], [undefined])

The pooled correlation is the arm difference. Inside the nulls the 4th moment does not move
ratio#sub[4] (except weakly at $N = 9$): within-null $R^2$ of ratio#sub[4] on kurt is 0.0016
($N = 11$), 0.0054 ($N = 13$). Instead the 3-point correlation does:

#table(columns: 4,
  [$N$], [$R^2$ on $z_"line"$ (nulls)], [$R^2$ on $z_"plane"$ (nulls)], [sd ratio#sub[4]: curves / paired nulls],
  [9], [0.493], [0.045], [0.1456 / 0.2853 (0.51)],
  [11], [0.915], [0.0016], [0.0182 / 0.0474 (0.38)],
  [13], [0.976], [0.0056], [0.0048 / 0.0120 (0.40)])

The curve/null scatter ratio (0.38, 0.40) equals the ratio of the $z_"line"$ standard
deviations (0.413 / 1.061 = 0.39; 0.426 / 1.093 = 0.39). *Reading:* the rigidity that reduces
scatter is that of the Kloosterman 3rd moment, $C_3 = -epsilon (q - 2)$, not of the 4th.

=== Positive control

A rank-one Walsh bias: $L = {x != 0 : ell(x) = 0}$, each membership flipped with probability
$eta$ (density stays 1/2). At $N = 11$, $eta = 0.46, 0.42, 0.38, 0.34, 0.30$:

#table(columns: 4,
  [$eta$], [kurt (mean)], [ratio#sub[4] (mean)], [first-order prediction (mean)],
  [0.46], [3.008], [0.981], [0.978],
  [0.42], [4.725], [1.260], [1.238],
  [0.38], [8.467], [1.856], [1.698],
  [0.34], [23.12], [3.906], [3.041],
  [0.30], [51.76], [9.648], [5.969])

Spearman of kurt with ratio#sub[4] over the 20 planted sets: 0.9444, 0.9985, 0.9940 at
$N = 9, 11, 13$ (permutation $p = 1.0 dot 10^(-5)$). Direction as predicted. The first-order formula
underestimates strong planting, as expected (non-perturbative regime). Unions of cosets of a
subspace (exploratory, not gated) do not give a monotone relation: their counts are
non-perturbative and the raw 4th moment alone does not determine the count.

=== Composite $N = 12$

The 4th moment is identical (2.0005), but $C_3 = +(q + 2)$ flips the sign of the line term, so the
first-order deficit is $-(105 - 35)\/q approx 0.0165$ against $0.0115$ observed (curves 0.9885
against paired nulls 0.9983, $p = 0.0715$, 10 vs 10; null sd 0.020). The parent's "no difference"
(6 vs 6) is consistent with a 1--2 % effect that the sample cannot resolve. The all-lift
$FF_4$-subspaces of $FF_4$-dimension 2 (the subfield structure) number 85--131 per curve against
a null mean of 3.22 (max 9), but are at most 0.026 % of $A_4$. Removing them changes the Koblitz
ratio from 0.9875974815932574 to 0.9873476161452932. *Subfield cancellation is not supported;
the first-order formula, with its sign flip, already accounts for the small $N = 12$ effect.*

== Figures

#figure(image("reports/run-20261027T1459Z/fig1_ratio4_vs_kurtosis.png", width: 100%), caption: [A#sub[4]/E#sub[4] against the Walsh 4th moment. Curves (blue squares) share one 4th moment; nulls scatter around 3.])
#figure(image("reports/run-20261027T1459Z/fig2_positive_control.png", width: 100%), caption: [Positive control: planted rank-one Walsh bias raises both axes together (log--log).])
#figure(image("reports/run-20261027T1459Z/fig3_first_order_vs_exact.png", width: 100%), caption: [First-order prediction against the exact ratio. Nulls lie on the diagonal; curves at odd $N$ lie $tilde 48\/q$ below.])
#figure(image("reports/run-20261027T1459Z/fig4_standardised_correlations.png", width: 100%), caption: [Standardised plane and line correlations. Curves: plane $z$ far below random and constant, line $z$ two-valued.])

== What is and is not established

- *Theorem* (proved above, checked on 120 curves): all Walsh power sums of a curve are curve-independent;
  $C_3 = -epsilon (q - 2)$ ($N$ odd), $C_4' = -(q - 2)(q - 4)$.
- *Exact computation:* $A_2 = E_2 + D_"line"$, the exact expansion of $A_k - E_k$ (Prop. 2).
- *Measured:* the first-order $k = 4$ formula explains 68--89 % of the odd-$N$ curve deficit and the null
  scatter (R² 0.92--0.98); line term confirmed by the Tr$(a)$ split; scatter reduction carried by $z_"line"$.
- *Not established:* the remaining $tilde 48\/q$ (rank-4 dependent classes, $t >= 5$; the 5- and
  6-point correlations are also curve-independent but were not evaluated); behaviour for $N > 13$,
  $k >= 5$; any ECDLP cost.
- *Failed:* P2 within nulls (1 of 3 sets of $N$ significant). The 4th moment alone does not drive the
  count of random sets.
- No ECDLP was solved or priced; no leaderboard, benchmark or rho row.

== Reproduce

`cargo build --release && ./target/release/lift-repulsion-mechanism run reports/run-<UTC>/ --threads 3`
(about 18 minutes, 748 sets), `python3 plot.py reports/run-<UTC>/`, then
`python3 -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"`.
