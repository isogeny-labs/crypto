#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= Every isogeny class over $FF_p$, $p = 2^31 - 1$: an exact census of ECDLP structure

Study `research/fp31_whole_field_census_20261111`, 2026-11-11. Base commit `24690cfeef`.
Tool: `tool/` (native Rust, no dependencies), `fp31-class-census census | bruteforce | summary`.
Host: 4-vCPU cloud container, no isolation; the run time (211 s) is incidental and not a performance claim.
Every number below is an exact computation over the complete field unless it is labelled _model_.
No claim here has a pcc claim id: all are *[pcc:unverified]*.

== Question and scope

The request was a statistical sweep over a whole field: every curve, every isogeny, every transfer, to see
whether ECDLP behaves differently across an isogeny class. Three parts of that question already have
answers in this repository or by theorem, so this study measures only the part that has not been done:
the structure of *every* isogeny class of one field.

- *Inside one class* (already measured): rho across 38 isogenous curves × 100 seeds,
  `research/isogeny-class-rho-n45/` (Kruskal–Wallis p = 0.38, 0 of 38 curves flagged); IC relation
  yield on every curve of full classes at 24, 28 and 32 bits, `research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md`
  (null supported in 9 of 12 cells, one flag not replicated). By theorem a generic algorithm sees only the
  group and $"Aut"(E)$, and isogenous curves have the same order (`research/isogeny_class_difficulty_20261008/`).
- *Lifting to $FF_(p^2)$* (theorem): $\#E(FF_(p^2)) = (p+1-t)(p+1+t) = \#E dot \#E^"tw"$, and the Weil restriction of
  $E slash FF_(p^2)$ to $FF_p$ is isogenous to $E times E^"tw"$. The lift contains the same prime subgroup
  and adds the twist's; it cannot shrink $r$. The census confirms it: $r_(p^2) = max(r, r^"tw") >= r$ in all
  185,363 classes (0 exceptions).
- *Covers over a prime field* (theorem): GHS/Weil descent needs a subfield to descend to; $FF_p$ has none.
  A cover $C -> E$ over $FF_p$ moves the DLP into $"Jac"(C)$ over the same $FF_p$, where index calculus for
  genus $g >= 3$ costs $tilde(O)(p^(2 - 2/g))$, more than rho's $p^(1/2)$.

== Method

By Tate's theorem each trace $t$ with $|t| <= 2 sqrt(p)$ is one $FF_p$-isogeny class. For every one of the
185,363 traces $t in [-92681, 92681]$ the tool computes: $N = p + 1 - t$ and its factorisation; $r$, the largest
prime of $N$; the twist order $p+1+t$ and its largest prime; $D = t^2 - 4p = f^2 D_K$; $h(D_K)$, exactly, by
counting reduced forms (`classno.rs`, copied unchanged from `research/isogeny_class_trio_20261017/selector/`);
the Hurwitz class number $H(4p - t^2) = sum_(d | f) h(d^2 D_K) dot 2 slash w$, which is the number of
$FF_p$-isomorphism classes of curves with trace $t$ (weighted $2 slash |"Aut"|$); the embedding degree
$k = "ord"_r(p)$; and the volcano depths $v_ell(f)$. A random curve $y^2 = x^3 + a x + b$ lies in class $t$ with
probability $H(4p - t^2) slash 2p$, which is how "share of curves" is weighted below.

*Checks, all passed:*
+ $sum_t H(4p - t^2) = 2p$ exactly: $sum 6H = 25,769,803,764 = 12p$ (`data/census_p2147483647.jsonl.check.json`).
+ Brute force: every $(a, b)$ over $FF_p$ point-counted naively and binned by trace equals $(p-1)/2 dot H$ for every
  trace at $p in {101, 103, 107, 109, 113, 1009, 1013}$, which covers $p equiv 1, 5, 7, 11 space (mod 12)$ (and so the
  $j = 0, 1728$ weights); 0 mismatches, up to 1,025,156 curves per prime.
+ Independent implementation: a PARI prototype (`qfbclassno`, `factor`, `znorder`, via cypari2; not part of the
  result path, AGENTS.md "no Python") gave the same values in all 185,363 rows on all 17 integer fields.

== Results

#figure(image("figures/class_size_vs_trace.svg", width: 100%),
  caption: [Number of curves $H(4p - t^2)$ in each of the 185,363 isogeny classes against the trace, coloured by $log_2 r$.
  The envelope is the Sato–Tate semicircle; the scatter under it is the class number of $t^2 - 4p$. Data: `data/census_p2147483647.jsonl.gz`.])

#table(columns: (auto, auto), stroke: 0.4pt,
  [*quantity (exact)*], [*value*],
  [isogeny classes / curves (weighted)], [185,363 / 4,294,967,294 = 2p],
  [classes of prime order], [8,646 (4.66 % of classes, 2.64 % of curves)],
  [median $log_2 r$: per curve / per class], [18.65 / 19.31],
  [$log_2 r$ at the 1st percentile of curves], [8.07],
  [smallest $r$], [11 ($t = -47584$, $N = 2^5 dot 3 dot 7^5 dot 11^3$, 39,424 curves)],
  [anomalous class ($t = 1$, $N = p$; Smart's attack is polynomial)], [17,184 curves],
  [supersingular class ($t = 0$, $N = 2^31$)], [39,730 curves],
  [smallest embedding degree with $r >= 2^20$], [$k = 31$ ($t = -1$, $r = (2^31+1) slash 3$)],
  [classes where the $FF_(p^2)$ lift has smaller $r$], [0 of 185,363],
  [largest class], [171,048 curves ($t = 11752$, $f = 6$)],
  [most endomorphism-ring strata], [24 orders ($t = 80012$, $f = 1386$)],
)

#figure(image("figures/log2r_hist.svg", width: 100%),
  caption: [Distribution of $log_2 r$ over classes (each trace once) and over curves (weighted by $H$). Recorded bins: `data/summary_p2147483647.json`, `log2_r_hist_quarter_bits`.])

=== The one cross-class difference that exists, and its cause

Class size and smoothness of $N$ are correlated across classes. Spearman correlation over the 185,362 ordinary classes
of $log_2 r$ against: $log_2 H$, $-0.141$; number of endomorphism rings, $-0.134$; $log_2 f$, $-0.122$; $log_2 |D_K|$, $+0.104$;
$|t|$, $-0.0002$. Curve-weighted mean $log_2 r$ falls from 20.34 in classes with one stratum (crater only) to 18.96 with two
and 16.35 with eight or more.

This is a known theorem, not a new weakness. If a prime $ell$ divides $N$ then $t equiv p + 1 space (mod ell)$, so
$t^2 - 4p equiv (p-1)^2$ is a square mod $ell$; small split primes raise $L(1, chi_D)$ and with it the class number.
Equivalently, Frobenius mod $ell$ is equidistributed among matrices of determinant $p$ in $"GL"_2(FF_ell)$, so a random
curve has $ell | N$ with probability $ell slash (ell^2 - 1)$ when $p equiv 1 space (mod ell)$ and $1 slash (ell - 1)$ otherwise
(Lenstra 1987; Howe 1993), while a random trace has $ell | N$ with probability $1 slash ell$. The census reproduces both
for every prime $ell <= 47$ with relative error at most $1.3 times 10^(-4)$ (figure below, `lenstra_divisibility`).

#figure(image("figures/lenstra.svg", width: 100%),
  caption: [Fraction of curves (circles) and of classes (squares) with $ell | \#E$, times $ell$, against the $"GL"_2(FF_ell)$ prediction (bars). $p = 2^31 - 1 equiv 1$ mod 2, 3, 7, 11, 31.])

#figure(image("figures/generic_cost_cdf.svg", width: 100%),
  caption: [_Model, not measured:_ expected generic cost $sum_(q^e || N) e sqrt(pi q slash 2)$ (Pohlig–Hellman with rho per prime), cumulative over curves and over classes.
  The median curve needs about $2^9.7$ group operations, against $2^15.8$ for a curve of prime order. The rows: `rho_cost_ph`.])

== What this does and does not establish

- Established (exact): the full distribution of group structure, class size, conductor, embedding degree and twist
  over every isogeny class of $FF_(2^31 - 1)$, and that the only systematic cross-class effect in these invariants is the
  Lenstra divisibility bias. Every weak class found here is weak for a textbook reason: a small largest prime
  (Pohlig–Hellman), $t = 1$ (anomalous) or $t = 0$ (supersingular). No class with $r >= 2^20$ has embedding degree below 31.
- Not established: anything about running time. The cost column is a model. A measured check that rho cost is
  $sqrt(pi r slash 2)$ in every class and depends on nothing else (class size, conductor, $D_K$) needs a preregistered ecbench
  run with a benchmark record; it is the proposed next step (README). This is not an improvement over the index-calculus
  baseline and adds no leaderboard row.
