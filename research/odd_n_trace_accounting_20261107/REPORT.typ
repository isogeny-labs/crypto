#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
= Odd-$n$ trace accounting: who owns the factor 2 on $2E$

2026-11-07. A follow-up to the subgroup-trap study (PR \#3646), which derived, without measuring, that for odd $n$ the twisted/standard yield ratio on $2E$ targets is $2^(n+1)$, not $2^n$. The protocol was frozen before any study data at `3516d8a885`. The run is `reports/run-20261107T1/`. Every statement is an unverified assertion of the producing session [pcc:unverified].

== Setup

$E: y^2 + x y = x^3 + a x^2 + b$ over $FF_(2^N)$, $N = d n$, $q = 2^d$, with $deg b = N$. For $P != O$, $P in 2E$ iff $"Tr"(x_P) = "Tr"(a)$. A line $u FF_q$ is trace-homogeneous iff $"Tr"_(N\/d)(u) = 0$, since $"Tr"_N (u t) = "Tr"_d (t "Tr"_(N\/d)(u))$. The plain $FF_q$ has $"Tr"_(N\/d)(1) = n mod 2$, so it is trace-mixed for odd $n$.

Arms: `std` ($FF_q$); `tz` (its $2E$ part); `tw` (all-lift line: $"Tr"_(N\/d)(u) = "Tr"_(N\/d)(sqrt(b)\/u) = 0$); `twh` (the half of `tw` with $"Tr"_d (t) = 0$); `hz` (a trace-homogeneous line that is not all-lift, so it has the standard size); and `mix` (a generic trace-mixed line). Cells: $n = 3$ ($d = 3, 4, 5$), $n = 5$ ($d = 3, 4, 5$), and $n = 4$ ($d = 3, 4$) as the even control. Three curves with $"Tr"(a) = 0$ per cell, plus one with $"Tr"(a) = 1$. Targets: every point when $N <= 15$, otherwise 2,000 per class, in the classes $2E$, off $2E$, and odd order.

*Exact coset identity.* $sum_(R in 2E) N(R) = ((m_0+m_1)^n + (m_0-m_1)^n)\/2$, where $m_0$ and $m_1$ count base points in and off $2E$. Among bases of size $m$, the $2E$ mean is largest, $2 m^n \/ hash E$, exactly for single-coset bases. Write $kappa = "mean" N dot hash E \/ m^n$ (1 for a uniform base, 2 for a single-coset base on its coset) and $eta = "mean" N \/ c$ (relations per trial per column).

== Results

#figure(image("fig_odd_n_trace.png", width: 100%), caption: [(a) $kappa$ on $2E$ targets for every arm and every $"Tr"(a) = 0$ curve. Trace-homogeneous arms sit at 2; `std` sits at 1 for odd $n$ and at 2 for $n = 4$. (b) Per-curve ratios for odd $n$, with 99 % bootstrap intervals and the composition-exact predictions.])

*Exact checks (P8, PASS).*
- The trace criterion agrees with $[hash E\/2]R = O$ on every target, with 0 mismatches.
- MITM counts agree with brute force on 2,293,041 (arm, target) pairs.
- The coset identity holds exactly on all 120 census arm-units.
- `tw` has exactly $2(q-1)$ points, and `tw`, `twh`, `hz` and `tz` lie in $2E$.
- For odd $n$, all 469,256 off-$2E$ counts of `tz`, `tw`, `twh` and `hz` are 0.

*Registered verdicts.*
- P2 (all targets) PASS. P4 ($n = 4$) PASS. P7 ($"Tr"(a) = 1$) PASS. P8 PASS.
- P1 FAIL. The pooled $kappa_"tw"\/kappa_"std"$ on $2E$ is 1.96 for odd $n$ (inside the band [1.7, 2.3]) and 0.996 for even $n$. Only 11 of 18 per-curve intervals exceed 1.25, where 17 were needed.
- P3, P5 and P6 FAIL, and the interpretation rule returns *inconclusive*. Every failure comes from $n = 5$ units, apart from one $n = 3$ curve (`tz` on a 65-point odd subgroup). The causes:
  - extremely clustered counts: 97 to 100 % zeros for the small arms;
  - zero sample means, which make the registered geometric mean of per-curve ratios 0, $infinity$ or NaN;
  - one $q = 8$ curve whose 6-point $FF_q$ base is trace-homogeneous by chance;
  - two tiny odd subgroups, of 65 and 257 points.

*Post hoc pooled analysis* (not registered; units pooled as the mean of $kappa$ over curves, with a bootstrap over targets within curves):

#table(columns: 4, align: (left, center, center, center),
  [group, class], [$H = K_"hz"\/K_"std"$], [$L = K_"tw"\/K_"hz"$], [$D = K_"tw"\/K_"std"$],
  [odd $n$, all 18, $2E$], [2.08 [1.62, 2.72]], [0.90 [0.71, 1.11]], [1.86 [1.62, 2.16]],
  [odd $n$, `std` mixed (17), odd order], [2.11 [1.55, 2.80]], [1.00 [0.80, 1.26]], [2.10 [1.70, 2.65]],
  [$n = 4$, 6, $2E$], [1.00 [0.80, 1.26]], [0.99 [0.84, 1.17]], [0.99 [0.84, 1.19]],
)

On the $n = 4$ control, the trace-mixed line has $K_"mix"\/K_"std" = 0.51$ [0.40, 0.66], the mirror image of the odd-$n$ effect. For `tz`, $eta_"tz"\/eta_"std"$ on $2E$ is 0.91 [0.70, 1.13] of its composition prediction, which is about $2^(2-n)$. Restricting $FF_q$ to its trace-zero half therefore loses per column.

== Reading

- The extra 2 for odd $n$ is a real per-trial gain, at equal base size, of any trace-homogeneous base over the trace-mixed $FF_q$.
- An ordinary index calculus gets it by using a trace-homogeneous line `hz` of the standard size, with the same system shape.
- So it belongs to the baseline, not to the twisted line. Crediting $2^(n+1)$ to the twisted line is accounting.
- For the Oakley lane ($n = 5$, large $q$), on cryptographic targets:
  - against plain $FF_q$: $2^(n+1) = 64 times$ yield per trial, $32 times$ in trials;
  - against `hz`: $2^n = 32 times$ yield, $16 times$ in trials.
- This reading is post hoc. The registered rule was inconclusive, and the solver cost of `hz` was not measured.

== Not claimed

No ECDLP cost, leaderboard, benchmark or rho row. Execution was local exact counting; the AGENTS.md §9a isolab-eks confirmatory run is open. The KB was unavailable (no `research-kb` binary, MCP not connected), so the lead atlas and `rg` were used instead.
