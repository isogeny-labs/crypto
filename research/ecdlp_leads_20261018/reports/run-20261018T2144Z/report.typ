#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#show link: underline
#align(center)[#text(size: 15pt, weight: "bold")[Run report: genus-4 screen for étale covers at K-283, K-409 and K-571 (lead L8)]\ run-20261018T2144Z · study `research/ecdlp_leads_20261018` · commit `4369a50a`]

*Kind.* This is an exact enumeration with a screen on a necessary condition, run with the native Rust
crate `etale_search` (`screen.rs`, `genus4.rs`). It is not a timing or statistical measurement.
Host: shared cloud container (Xeon 2.80 GHz, 4 cores, rustc 1.97.0), isolation L0. It builds on
run-20261018T2129Z, which closed the étale route for bases of genus $<= 3$.

*Why genus 4.* The L(1/2) budget (derived) allows base genus $g_0 <= 4$ for K-283, $<= 5$ for
K-409 and $<= 7$ for K-571, so genus 4 decides K-283. An $FF_2$-rational, geometrically
$ZZ\/n$-Galois cover with Frobenius acting by $lambda$ exists iff $n | P_B (lambda)$, and $T_n$ in the
Prym needs $op("ord") lambda <= g_0 - 1 = 3$. The run tests $lambda$ of order 1, 2 and 3.

*What ran.*
- *Hyperelliptic models:* $y^2 + h(x) y = f(x)$ with $deg h <= 5$ and $deg f <= 10$.
- *Canonical models:* $X = Q inter C$ in $PP^3$, with $Q$ one of three normal forms (hyperbolic,
  elliptic, cone; Xarles, Lemma 9), and the cubic $C$ taken modulo $Q dot L$ over the 16 monomials
  not divisible by $x_0 x_1$.
- *Smoothness:* tested over $PP^3(FF_16)$, $PP^3(FF_32)$ and $PP^3(FF_64)$. Every singular point has
  degree $<= 6$ (derived).
- *$L$-polynomials:* from the counts over $FF_2, ..., FF_16$, checked against the Weil bounds and,
  for canonical curves, against out-of-sample counts $N_5$ and $N_6$.

#table(columns: 4, stroke: 0.4pt + luma(170), inset: 4pt,
  [*quantity*], [*hyperelliptic*], [*canonical (trigonal)*], [*all genus 4*],
  [smooth models], [49,152], [55,368], [104,520],
  [distinct $L$-polynomials], [217], [521], [620 (118 shared)],
  [largest $\#J = L(1)$], [139], [260], [*260*],
  [largest $|L(-1)|$], [139], [144], [*144*],
  [largest $\#X(FF_2)$], [6], [8], [8],
  [$N_5, N_6$ checked / failed], [–], [110,736 / 0], [],
  [$n | L(plus.minus 1)$, $n in {283, 409, 571}$], [none], [none], [*none*],
  [$n | P(lambda)$, $op("ord") lambda = 3$], [283, 409], [283, 409], [],
)

*Census check (sourced).* Xarles (arXiv:2007.07822, Theorem 10 and Tables 1 and 5) gives 620
$L$-polynomials for genus-4 curves over $FF_2$. Of these, 403 do not occur for hyperelliptic curves
and 99 do not occur for trigonal curves. That leaves 217 hyperelliptic, 521 trigonal and 118 shared,
all matched here. The largest numbers of points, 6 and 8, also match. The hyperelliptic model count
is $384 dot 2^7$, the mass of $H_4(FF_2)$.

#figure(image("reach.svg", width: 100%), caption: [Odd primes dividing $L_B (plus.minus 1)$ over every genus-4 curve over $FF_2$, and the $n$ with $n | P_B (lambda)$ for $lambda$ of order 3, against the deployed sizes whose budget allows $g_0 = 4$. Recorded values from `reach.csv`.])

*Point-count lemma (derived).* Suppose Frobenius acts by $lambda$ of order $k = g_0 - 1$ and $T_n$ is in
the Prym.
- The Prym's eigenvalues are closed under $mu_k$ and contain those of $T_n$. A dimension count then
  shows they are exactly $omega zeta^j pi^plus.minus$.
- So $\#D(FF_(2^k)) = \#B(FF_(2^k)) + k t_k$.
- The cover is untwisted over $FF_(2^k)$, so this number is $equiv 0 mod n$.
- For $n$ above the Hasse–Weil range this forces $\#B(FF_(2^k)) = -k t_k$.

For the $a = 0$ Koblitz curve, $t_3 = 5$, $t_4 = 1$ and $t_6 = 9$ are positive. So the maximal twist
is impossible at $g_0 = 4, 5, 7$, and every order-3 candidate above is excluded.

*What this closes.* K-283 is closed: genus 2 and 3 fail the Weil bound, and genus 4 fails by
measurement plus the lemma. Genus 4 is also closed for K-409 and K-571. These two remain open at
$g_0 = 5$ ($k <= 3$), and K-571 also at $g_0 = 6, 7$. Together with the earlier runs, the étale route
is closed for ECC2K-130, K-163, K-233 and K-283.

*Not covered.* Genus $>= 5$ bases, and ramified (L7) and non-cyclic covers.

*Records.* `RUN.txt` (commit, host, SHA-256), `genus4.jsonl`, `reach.csv` (via `tables.awk`),
`reach.plot.awk`.
