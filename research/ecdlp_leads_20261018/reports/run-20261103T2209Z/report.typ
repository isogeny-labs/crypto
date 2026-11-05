#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#show link: underline
#align(center)[#text(size: 15pt, weight: "bold")[Run report: genus-5 screen for étale covers at K-409 and K-571 (lead L8)]\ run-20261103T2209Z · study `research/ecdlp_leads_20261018` · commit `579d02b3a`]

*Kind.* Exact enumeration of two families, an exact recount of a published list for the third, and
a screen on a necessary condition, run with the native Rust crate `etale_search` (`genus5.rs`). It
is not a timing or statistical measurement. Host: shared cloud container (Xeon 2.80 GHz, 4 cores,
rustc 1.97.0), isolation L0. It builds on run-20261018T2144Z, which closed K-283 at genus 4.

*Why genus 5.* The L(1/2) budget (derived) allows $g_0 <= 5$ for K-409 and $<= 7$ for K-571, and
genus $<= 4$ is closed for both. At genus 5 the Weil bound $(1+sqrt(2))^10 approx 6726$ on $\#J_B (FF_2)$
exceeds 409 and 571, so the bound no longer closes it and the screen has to. An $FF_2$-rational,
geometrically $ZZ\/n$-Galois cover with Frobenius acting by $lambda$ exists iff $n | P_B (lambda)$,
and $T_n$ in the Prym needs $op("ord") lambda <= g_0 - 1 = 4$. The run tests $lambda$ of order 1, 2, 3
and 4 (for 409 all four occur; for 571 orders 1, 2, 3).

*What ran.* Every genus-5 curve over $FF_2$ is hyperelliptic, trigonal (a plane quintic with one
singular point of delta invariant 1) or a complete intersection of three quadrics in $PP^4$
(sourced: Dragutinović, arXiv:2202.07809). That paper lists all three families with point counts and
automorphism sizes. The lists were read as untrusted data; none of their code was run.
- *Hyperelliptic, independent:* every model $y^2 + q y = p$, $deg q <= 6$, $deg p <= 12$ ($2^20$ models).
- *Trigonal, independent:* every quintic singular at $(0:0:1)$ ($2^18$ forms), one node or cusp, no other
  singular point over $FF_(2^k)$, $k <= 6$.
- *Sourced recount:* every published model recounted over $FF_2, ..., FF_64$ (hyperelliptic and trigonal
  also $FF_128$); $L$ from $N_1..N_5$; compared with the published counts; Weil bounds; $N_6$ ($N_7$) out of sample.
- *Not re-tested:* smoothness of the complete intersections (carried by the published Sage check).

#table(columns: 5, stroke: 0.4pt + luma(170), inset: 4pt,
  [*quantity*], [*hyperelliptic*], [*trigonal*], [*complete int.*], [*all genus 5*],
  [curves (published)], [1,070], [2,854], [3,905], [*7,829*],
  [counts = published], [1,070], [2,854], [3,905], [all],
  [Weil / $N_6$ failures], [0], [0], [0], [0],
  [distinct $L$], [841], [2,036], [2,827], [*4,339*],
  [$sum 1\/|op("Aut")|$ (published)], [512], [2,817], [3,584], [*6,913*],
  [independent enumeration], [393,216 models], [67,608 forms at one point], [–], [],
  [divided by group order], [$393216\/768 = 512$], [$7 dot 67608\/168 = 2817$], [–], [],
  [$L$-set = published], [841 = 841], [2,036 = 2,036], [–], [],
  [largest $L(1)$], [342], [710], [975], [975],
  [largest $|L(-1)|$], [342], [176], [392], [392],
  [$n | L(plus.minus 1)$, 409, 571], [none], [none], [none], [*none*],
)

*Census check (sourced and measured).* The union has 4,339 distinct $L$-polynomials, the count the paper
gives. The stack counts $512 = 2^9$, $2817 = 2^11 + 2^10 - 2^8 + 1$ and $3584 = 2^12 - 2^9$ add to
$|M_5 (FF_2)| = 6913$. The hyperelliptic and trigonal families are measured independently here. The
complete-intersection family is sourced: its completeness is the paper's Theorem 4.1 plus the mass
3,584; this run checks its counts, Weil bounds and $N_6$, not its smoothness or isomorphism classes.

#table(columns: 6, stroke: 0.4pt + luma(170), inset: 4pt,
  [*n*], [*ord $lambda$*], [*$L$-polys*], [*curves*], [*$\#J(FF_2)$*], [*status*],
  [409], [1, 2], [0], [0], [–], [*closed*],
  [409], [3], [20], [40], [10 – 166], [*open*],
  [409], [4], [23], [44], [6 – 249], [excluded (lemma)],
  [571], [1, 2], [0], [0], [–], [*closed*],
  [571], [3], [12], [19], [8 – 122], [*open*],
)

#figure(image("reach.svg", width: 100%), caption: [Odd primes dividing $L_B (plus.minus 1)$ over every genus-5 curve over $FF_2$, and the $n$ with $n | P_B (lambda)$ for $lambda$ of order 3 and 4, against the deployed sizes whose budget allows $g_0 >= 5$. Recorded values from `reach.csv`.])

*Order 4 (derived).* The point-count lemma of the genus-4 run applies at $k = g_0 - 1 = 4$: if $T_n$ is in
the Prym, $\#D(FF_16) = \#B(FF_16) + 4 t_4 equiv 0 mod 409$, and $409 > 2^4 + 1 + 10 dot 2^2 + 4|t_4| = 61$
forces $\#B(FF_16) = -4 t_4$. For $a = 0$, $t_4 = 1 > 0$, so $\#B(FF_16) < 0$: impossible.

*Order 3 is not excluded.* At $g_0 = 5$ the Prym's eigenvalues are only known to contain the $mu_3$-closure
of $T_n$'s, which fills $3(n-1)$ of $4(n-1)$ dimensions. The remaining $n-1$ dimensions are free, so
$\#D(FF_8) = \#B(FF_8) + 3 t_3 - 3 s'$ with $s'$ an unknown integer, and no point count rules the
candidates out. $n | \#J(FF_8)$ holds for all 32 order-3 classes, as it must.

*What this closes.* No genus-5 curve over $FF_2$ has an étale $ZZ\/409$ or $ZZ\/571$ cover with $lambda = plus.minus 1$, and
the order-4 twist is excluded at $n = 409$. K-409 stays open at $g_0 = 5$ through 20 order-3 classes
(40 curves), and K-571 through 12 classes (19 curves); K-571 is also open at $g_0 = 6, 7$. No security
claim changes and no index-calculus cost moves.

*Next.* For the 59 candidate curves compute the twisted $L$-function over $FF_8$ (degree 8) for a
character of $J_B (FF_8)[n]$ and test for $(1 - c pi^3 X)(1 - c overline(pi)^3 X)$.

*Records.* `RUN.txt` (commits, host, SHA-256 of outputs and inputs), `g5_hyp_enum.jsonl`,
`g5_trig_enum.jsonl`, `g5_sourced.jsonl`, `candidates.csv`, `reach.csv` (via `tables.awk`), `reach.plot.awk`.
