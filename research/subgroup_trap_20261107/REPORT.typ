#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
= Subgroup traps in lift-dense factor bases

2026-11-07. A follow-up to the C1 toy ladder (PR \#2704) and the lift-dense study (PR \#2686). The protocol was frozen before any computation at `7ac2099b8`, and Amendment 1 at `fe8b2b62e`. The run is `reports/run-20261107T2/`. Every statement is an unverified assertion of the producing session [pcc:unverified].

== Question

In the C1 ladder (field $FF_(Q^4)$, $Q = 2^k$, $n = 4$), the positive control A0, $y^2 + x y = x^3 + 1$, returned zero decompositions on every sampled target at $k = 7$ and 9, where the counting identity predicts about 15.5 per target. At $k = 5$ its counts were extremely clustered. The registered checks H-Y1 and H-Y3 failed on A0.

== Mechanism

*T1, subfield trap.* Let $K = FF_2(u FF_Q, a, b)$. A lifting point with $x in K$ has $y in K_2 inter FF_(2^N)$, where $K_2$ is the quadratic extension of $K$. So the factor base lies in $E(K^*)$ with $K^* = K_2 inter FF_(2^N)$. Every sum of factor-base points stays in that subgroup. *A trap occurs if and only if $[K : FF_2] < N$ and $2[K : FF_2] != N$* (Amendment 1).

On A0's standard line, $K = FF_Q$. A point with $x in FF_Q$ satisfies $sigma_Q P = plus.minus P$, so the factor base lies in $H = E(FF_Q) + ker(sigma_Q + 1)$. The two pieces meet only in $E(FF_Q)[2]$, which has order 2, so $|H| = |E(FF_(Q^2))| slash 2$.

*T2, 2E concentration.* $P in 2E$ if and only if $"Tr"(x_P) = "Tr"(a)$. Every C1 factor base has $"Tr"(x) = 0$. A sum of four points therefore lies in $2E$, or in the matching coset when $"Tr"(a) = 1$.

== Results (registered)

#figure(image("fig_subgroup_trap.png", width: 100%), caption: [(a) A0 standard base: the exact mean count on $H$ against the identity prediction over $E$. (b) Every frozen row: on targets with $"Tr"(x_R) = "Tr"(a)$, the mean count divided by the prediction. Targets off $2E$ all have count 0.])

- *Identity.* A second full run (`run-20261107T1`) agrees on every frozen field. Regenerating the targets with the run's seeds reproduces every frozen count: 72 rows, 0 mismatches.
- *P1 PASS.* At $k = 5, 7, 9$, every factor-base point of A0 `std` and `tw_a` ($[K:FF_2] = k$, trapped) lies in $H$.
  - $k = 5$: 16 of the 20,000 targets lie in $E(FF_(Q^2))$, 9 of them in $H$. No target outside $H$ has a nonzero count.
  - $k = 7$ and $k = 9$: none of the 8,000 and 1,500 targets lies in $E(FF_(Q^2))$, and every count is 0.
  - A target lands in $H$ with probability $|H| slash |E(FF_(Q^4))| approx 1 slash (2 Q^2)$: 0.25 expected hits at $k = 7$, 0.003 at $k = 9$.
- *P2 PASS.* $sum_(R in E(FF_(Q^2))) N(R) = m^4$ exactly:
  - $k = 5$: 14,776,336 over 968 points;
  - $k = 7$: 4,162,314,256 over 16,472 points.

  The support is exactly $H$: 484 and 8,236 points, half of $E(FF_(Q^2))$ in each case, and every point of $H$ is reached. The mean on $H$ is $m^4 slash |H|$: 30,530 and 505,381.
- *P3 PASS.* Across all 72 rows (every curve and arm at $k = 5$ to 11), the 309,870 (row, target) pairs with $"Tr"(x_R) != "Tr"(a)$ have zero decompositions. On the other half, the mean count divided by the prediction has a median of 2.007 over the non-A0 rows (range 1.60 to 2.43).
- *P4 PASS.* No line that the merged studies call applicable is trapped:
  - Oakley groups 3 and 4: $[K:FF_2] = N$; for odd $N$ the trap is impossible.
  - c2pnb176v1 to 368w1: every recorded line has $[K:FF_2] = N$. Since $deg b = 16$ on every curve, no $u in FF_(2^16)$ can trap.
  - C1 A1, A2 and B1 to B3: 54 rows, none trapped.

== Post hoc: the borderline case is not benign

Amendment 1 called the case $2[K:FF_2] = N$ benign: the factor base lies in $E(K) + ker(sigma_K + 1)$, which equals $2E$. A0 `tw_b` ($u in FF_4 without FF_2$, $[K:FF_2] = 2k$) contradicts that in the sense that matters.

Write the factor base as $A union B$, with $A subset E(K)$ and $B subset ker(sigma_K + 1)$. Each piece is a group of size about $Q^2$, but a single line point covers only about $Q$ of it. A 4-sum therefore reaches a generic point of $2E$ only when exactly two points come from each piece. The typical target, and every cryptographic target of odd order, sees only the fraction $binom(4,2) p^2 (1-p)^2$ of the mass, where $p = |B| slash m$. The rest sits on sets of relative size $O(1 slash Q)$.

Fraction of the 2E-half targets with zero count. A0 `tw_b` (borderline): 74 %, 72 % and 74 % at $k = 5, 7, 9$. A1 `tw_a` (full degree): 31 %, 27 % and 25 %. The predicted generic share for `tw_b`, $6 p^2 (1-p)^2$, is 0.29, 0.37 and 0.37.

This is algebra written after the data, with descriptive statistics. It is not a registered test. No applicable line in \#2686 or \#2704 is borderline (P4: all have $[K:FF_2] = N$). The construction rule becomes: *use a line only when $[K:FF_2] = N$.*

== What it means

- A0 was never a valid positive control. Its H-Y1 and H-Y3 failures in \#2704 are explained exactly, and the verdicts stay as registered.
- The yield and cost-model rows of \#2686 and \#2704 stand (P4).
- The factor 2 on $2E$ is accounting. Cryptographic targets of odd order lie in $2E$.
  - For even $n$ the standard base is in $2E$ too, so the twisted/standard ratio is unchanged.
  - For odd $n$ (Oakley, $n = 5$), the ratio against the plain $FF_q$ base becomes $2^(n+1)$. This is derived, not measured. The extra 2 is the known cofactor mechanism, available to any trace-zero base (`LEAD-ICB-cofactor-torsion-quotients`), so it is not an improvement.
- ECC2K-130: $N = 131$ is prime, so $K in {FF_2, FF_(2^131)}$ and no subfield trap is possible.
- This complements `lift_dense_torsion_pairing_20261106`, which gives the rule to skip $u in b^(1 slash 4) FF_q$ and wrote the subfield explanation as unmeasured algebra for $n = 2$. A0's line, $b = 1$, belongs to both classes.

== Not claimed

No ECDLP cost, and no leaderboard, benchmark or rho row. Execution was local: these are exact algebraic checks of frozen data (AGENTS.md §9a, as for the parent studies). The KB was unavailable (no `research-kb` binary, MCP not connected). The lead atlas and `rg` were used instead.
