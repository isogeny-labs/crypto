#set page(margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: false)
= R7.7 -- the trace pair as a coset label for a = 1 and for E/8E

Curves $y^2 + x y = x^3 + a x^2 + b$ over $FF_(2^n)$, $n$ odd, $b != 0$. Toy fields ($n <= 23$); no claim about any deployed curve. Run records: `exhaustive.json`, `exhaustive_alla.json`, `random.json`, `control*.json`, `cost.json`, `witness.json`, `search_level*.json`; experiment `examples/r7_7_coset_label.rs`; full text in `RESULTS.md`.

== Verdicts

#table(
  columns: (auto, 1fr, auto),
  inset: 4pt,
  [*Hypothesis*], [*Statement and outcome*], [*Verdict*],
  [H1 (as posed)],
  [For $"Tr"(a) = 1$, $E slash 4E$ has R6.3's structure and the trace pair labels it. False premise: $v_2(\#E) = 1$, $2E$ has odd order, $E slash 4E = E slash 8E = E slash 2E = ZZ slash 2$. The label is $"Tr"(x) + "Tr"(a)$ (0 wrong in 148,598,015 points). The second bit satisfies $b_2(-P) = b_2(P) + "Tr"(x)$, so on $2E$ it flips between $P$ and $-P$ in one coset: the pair is non-constant on 312,667 of 312,684 cosets (the 17 exceptions are tiny $n = 5$ cosets). 0 cross-coset collisions.],
  [*refuted as posed; corrected form confirmed*],
  [H1' (R6.3 for any $a$, $b$ with $"Tr"(a) = 0$)],
  [$("Tr"(x), b_2)$ is constant on every coset of $4E$ and equals $(0,0), (0,1), (1,beta), (1,1+beta)$ for $k = 0,2,1,3$, with $beta = "Tr"(b H(b))$ ($= (n+1)/2 mod 2$ at $b = 1$). 0 failures in 71.6 M, 68.0 M and 9.0 M points (exhaustive $n <= 13$; all $(a,b)$, $n <= 9$; random $n = 17..23$).],
  [*confirmed* (constancy measured, rest proved)],
  [H2 ($E slash 8E$)],
  [$ZZ slash 4 times ZZ slash 2$ is impossible ($E(F)[2] = {O, T}$, 2-part cyclic). The pair cannot label $E slash 8E$ (4 values, 8 cosets; smallest counterexample $n = 5$, $a = 0$, $b = 6$). With $b_3(R) = b_2(R slash 2)$ for even $R$ and $b_3(P) = b_3(P + Q_0)$ for odd $P$, the triple is a bijection $E slash 8E -> FF_2^3$; the same recursion labels $E slash 16E$, $E slash 32E$. 0 failures in 74.1 M / 37.6 M / 18.2 M points. No closed trace formula for odd points was found among 120 trace monomials.],
  [*confirmed; datum not cheap*],
  [CT bar ($<2 times$)],
  [$"Tr"(a) = 1$: one masked trace, ratio 1 by construction. $"Tr"(a) = 0$: no new half-trace algorithm, not re-timed; R6.3's $2.5$--$5.0 times$ wall, $2.0$--$3.7 times$ words stands.],
  [*met (a=1); not met (a=0)*],
)

== The label tables

#figure(image("fig1_labels.svg", width: 100%), caption: [Label of $E slash 2^J E$ by case; $k$ counts from a reference odd point $Q_0$ ($b_2(Q_0) = 0$ for $v >= 3$; $P_4$ for $v = 2$). `plot_data.json`.])

== Evidence

#figure(image("fig2_evidence.svg", width: 100%), caption: [Left: points tested per case (exhaustive $n <= 13$, all-$a$ $n <= 9$, random $n = 17..23$ summed, overlap at $n <= 9$) with failures; right: two deliberately broken labels fail in the same code (negative controls).])

Proved: structure ($P in 2E <=> "Tr"(x) = "Tr"(a)$, $v$ from $"Tr"(a), "Tr"(b)$), the $a = 1$ statements, the $4E$ criterion for all $a, b$, $beta = "Tr"(b H(b))$, x-only forms $"Tr"(b slash x^2) = "bit"_1$ and $b_2 = "Tr"(x H(b slash x^2))$ for even $P$. Measured only: constancy of $b_2$ on odd cosets for general $(a, b)$ (R6.3 proves $(0,1)$) and the $J >= 3$ tables that rest on it.

== Cost

#figure(image("fig3_cost.svg", width: 100%), caption: [Counted $FF_(2^n)$ operations per point, $n = 23$ (M product, S squaring/sqrt, I inversion, H half-trace, T trace); `cost.json`, 24,000 points per bar. Scalar-multiplication truth for comparison is about 60 M + 50 S + 30 I.])

The pair ($J = 2$) saves the point addition of the halving route on odd points ($I + 3M + 2S$) and $M + S$ on even points for one more trace. For $J >= 3$ the label is not cheaper than halving. Wall time was not measured.

== Limits

Toy fields, $n$ odd only; exhaustive to $n = 13$ ($a in {0,1}$) and $n = 9$ (all $a$), sampled to $n = 23$ (1500 points x 1000 curves per $n$ and $a$-class). The 120-monomial search is a finite family. Counted operations, not weights. Constancy for general $(a, b)$ is verified, not proved.
