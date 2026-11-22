#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[Grevlex solving degree of the normalized two-summand Kummer chain systems over $FF_p$]

  Study `research/chain_sd_bounds_20261115/` — 2026-11-15 — standing `[pcc:unverified]`; Lean theorems kernel-checked on `propext`, `Classical.choice`, `Quot.sound`.
]

= Results at a glance

#table(
  columns: (auto, 1fr, auto),
  align: (left, left, left),
  [*\#*], [*statement*], [*status*],
  [T1], [Quotient reduction $V_(F,c) = "NF"(V_(F,c)) xor J_(<= c)$, $"NF"(V_(F,c)) = U_c$ in $FF_p[(ZZ\/2^k)^2]$], [proved (paper)],
  [T2], [Unit ideal: pinned solving degree = refutation degree = HKY last fall degree = LFD($k$)], [proved; Lean],
  [T3], [Admissible unit family $y^2 = x^3 + n^2(n-1)x$, $t = n$, $n$ a non-square, $2^(k+1) | p-1$], [proved; Lean],
  [T4], [Depth monotonicity LFD($k'$) $<=$ LFD($k$) for $k' <= k$ (same $p, A, B, t$)], [proved; Lean (concrete algebra)],
  [T5], [$ZZ$-exponent model: $pi_k (W_c) subset.eq U_c (k)$; equality observed in 6 cells], [containment proved],
  [T6], [LFD($k$) $>= 4$ for all $k >= 2$ ($t != 0$, $A != -3t^2$)], [proved (paper)],
  [T7], [LFD($k$) $<= 2k+1$ for every unit instance], [proved (paper)],
  [T8], [LFD($k$) $= 4,4,4,4,5,5,5,5$ for $k = 2..9$ with verified upper and lower replay certificates; LFD $>= 6$ for $10 <= k <= 15$ on $p = 65537$ by one certified depth-10 closure and T4], [certified (see §3)],
  [T9], [Linear lower bound $c k - C <=$ LFD($k$), $c > 0$], [*open*; reduced to Missing Lemma Π (§6)],
  [T10], [Theorem A: if every cofactor $v$ off the top two residues with $"wt"(g v) <= c-1$ has per-coordinate popcount $<= k-3$, then $1 in.not V_c (k)$], [proved; Lean],
  [T12], [Slope 1/4 shared by Möbius and affine couplings], [computed],
)

Separately: finite-depth results (§3), constant bounds $4 <= "LFD"(k) <= 2k+1$ (T6, T7), linear growth (upper slope 2 proved; lower slope open; lane-P empirical slope $1\/4$).

= Pinned objects

Ring $FF_p[y_(1,0..k-1), y_(2,0..k-1)]$, grevlex, generators $y_(s,j+1) - y_(s,j)^2$, $y_(s,k-1)^2 - 1$, $S_3(y_(1,0), y_(2,0), t)$; no field equations (inert below degree $p$). Algorithm: Caminata–Gorla degree-by-degree Macaulay closure $V_(F,c)$; the solving degree is the degree at which it stops ($1 in V_(F,c)$ for the unit ideal). In the chain algebra $Q_k = FF_p[(ZZ\/2^k)^2]$ the generators of the tower disappear, the degree becomes the binary weight of the exponents, and multiplication by a variable is the shift by $2^j$ with carries; $U_c (k)$ is the least subspace containing $g = S_3(x_1, x_2, t)$ and closed under shifts of its weight-$<= c-1$ elements. LFD($k$) $= min{c : 1 in U_c (k)}$.

= Finite-depth results

#figure(image("lfd_vs_k.svg", width: 100%), caption: [LFD($k$) against depth. Red: exact values on instance F1 ($p = 12289$, $n = 11$) with verified replay certificates (independent C checker); blue crosses: lane-P values quoted from the repository, not re-verified here; dashed: the proved upper bound $2k+1$; dash-dot: the proved constant lower bound 4. Run 2026-11-15.])

#figure(image("dims_vs_k.svg", width: 100%), caption: [Recorded closure dimensions $dim V_D (k)$ below the LFD against the ambient weight-$<= D$ dimension $binom(2k, <= D)$: $dim V_3 = 2k+1$, $dim V_4 = 4k^2 + 172k - 571$ on $k = 6..12$ (seven exact values).])

#table(
  columns: 5,
  [*k*], [*instance*], [*lower certificate $1 in.not V_D$ (dim/cols)*], [*upper certificate $1 in V_D$*], [*LFD*],
  [2], [F1], [D=3: 5/15], [D=4: 16/16], [4],
  [3], [F1], [D=3: 7/42], [D=4: 57/57], [4],
  [4], [F1], [D=3: 9/93], [D=4: 163/163], [4],
  [5], [F1], [D=3: 11/176], [D=4: 386/386], [4],
  [6], [F1], [D=4: 605/794], [D=5: 1586/1586], [5],
  [7], [F1], [D=4: 829/1471], [D=5: 3473/3473], [5],
  [8], [F1], [D=4: 1061/2517], [D=5: 6885/6885], [5],
  [9], [F1], [D=4: 1301/4048], [D=5: 12616/12616], [5],
  [10], [F2 ($p$=65537, $n$=3)], [D=5: 18569/21700], [—], [$>= 6$],
  [11..15], [F2], [transferred from $k = 10$ by T4 (Lean)], [—], [$>= 6$],
)

*Depth-15 milestone.* On the admissible instance F2 beyond depth 14 the delivered pair is: lower certificate LFD(15) $>= 6$ (certified full closure $1 in.not V_5(10)$, verified by the independent checker with the full closure sweep, plus the Lean-proved reduction $Q_15 -> Q_10$), upper bound LFD(15) $<= 31$ (Theorem 7, a proof). The expected exact value 7 (lane-P law) needs the closure on 2,629,575 columns ($D = 7$) for an upper certificate and on 768,212 columns ($D = 6$) for a direct lower certificate; both exceed this host.

= Open problem and missing lemma

A linear lower bound needs a *locality* lemma: for $c <= alpha k - beta$, $1 in.not U_c (k)$; equivalently a co-closed family of functionals annihilating $g$ with one not annihilating $1$, or (through T5 and the observed equality $U_c = pi_k (W_c)$) that no element of the $ZZ$-exponent closure $W_c subset (S_3)$ is $equiv 1$ modulo $(x_1^N - 1, x_2^N - 1)$. The evaluation functionals at the Semaev-curve points with one coordinate in $mu_N$ are *not* the annihilator ($k = 6$, $D = 4$: 188 of 605 rows fail although $1 in.not V_4$); this counterexample is preserved in `reports/evaltest.md`.

= Follow-up 2026-11-22: the cofactor reduction

*Theorem A (Lean).* With $g$ a unit supported on ${0,1,2}^2$, $g_(2,2) != 0$: if every $v in Q_k$ supported on $[0, N-3]^2$ with $g v in Q_(<= c-1)$ has per-coordinate popcount $<= k-3$, then $1 in.not V_c (k)$. The subspace of weight-$<= c$ elements whose cofactor $g^(-1) u$ avoids the top two residues is closed under the degree-$c$ rule and misses 1. The boundary functionals of §4 test exactly this: they annihilate $u$ iff $S$ divides the lift $L(u)$ iff the cofactor avoids the top residues.

*Missing Lemma Π.* Let Π($w$) be the largest per-coordinate popcount of a cofactor $h$ with $S h$ of binary weight $<= w$ in $FF_p [x_1, x_2]$ (no $k$, no wrap-around). A bound Π($w$) $<= alpha w + beta$ gives LFD($k$) $>= (k - 3 - beta)\/alpha + 1$.

#figure(image("pi_vs_box.svg", width: 100%), caption: [Measured Π($w$) for the F2 instance against the cofactor box; dotted: counting prediction $m(w) = 6, 10, 14$. Π(3) = 6 is saturated; Π(4) $>= 10$ and Π(5) $>= 11$ still track the box edge.])

The counting heuristic matches Π(3) and Π(4) exactly and grows by about 4–5 per unit of weight for $w <= 7$, which is the observed slope 1/4. Asymptotically it grows like $w log_2 w$, so if it is sharp, Missing Lemma Π fails and Theorem A alone gives only $Omega(k \/ log k)$. The closure reaches fewer cofactors than all multiples (popcount 5 at $c = 4$ against Π(3) = 6), so LFD itself may still be linear.

#figure(image("lfd_universality.svg", width: 100%), caption: [LFD($k$) for three couplings of two 2-power chains: Semaev ⌊(k+14)/4⌋, Möbius ⌊k/4⌋+3, affine ⌊(k-1)/4⌋+3.])

*Depth 15.* The constant 1 enters the closure only at 84–99.7 % of full rank ($k = 4..9$); at $k = 15$, $D = 7$ that is a dense echelon of about 25 TB. A larger host does not close the gap; no paid compute was launched.

= Files

`REPORT.md` (full statements and proofs), `lean/ChainSd/{Closure,Popcount,GroupAlgebra,Reach,Unit,LowerThree,Cofactor}.lean` (Lean 4.34.1, Mathlib v4.34.1; 22 theorems audited, standard axioms only), `native/` (Rust engine), `native/checker/chaincheck.c` (independent checker), `certs/` (replay certificates and `README.md`), `reports/` (this document, graphs, `values.json`, `evaltest.md`).
