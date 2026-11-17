#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#align(center)[
  #text(size: 14pt, weight: "bold")[The growth law of the last fall degree in the support width: what is proved, what is fitted]
  #v(2pt)
  2026-11-12 · `research/prime_ic_leads_20261110/growth_law/` · derivation; data from `devcheck/` rows (development checks, `[pcc:unverified]`); extends `prime_fall_degree_20261027/DERIVED.md` D1–D8
]

= Answer
Four results are proved; the growth itself is fitted, not derived. *Theorem 1:* for a fixed system, $d_F$ is non-decreasing in the tower height $t$ (reduction modulo $z^(2^t) - 1$ is a filtered ring map sending variables to variables or 1, and closures map into closures). *Theorem 2:* a degree-$c$ closure uses the top relation $y_(t-1)^2 = 1$ only through the top $c - 1$ variables (a carry leaves the tower only if bits $j..t-1$ are all set). *Theorem 3:* $d_F$ and $"ffd"$ have generic values determined by the support; the normal form of $S_3$ fills the width-2 box exactly, so "null equals tower" means $S_3$ sits in that box's generic stratum. *Theorem 4:* $"ffd" = 3, 4, 5, 5$ for widths $W = 1, 2, 3, 4$ (a common variable in the top form forces a fall one degree above it), equal to every measured row, and the reason $W = 3$ and $W = 4$ coincide.

= The fitted law (Conjecture G, $m = 2$)
$ d_F (W, t) = "ffd"(W) + max(0, floor((t - s_W) / 4)), quad s_1 = 0, s_2 = 2, s_3 = s_4 = 5 $
It matches every completed cell ($W = 1$: $t = 2..11$; $W = 2$: $t = 2..12$, which is C-step; $W = 3, 4$: $t = 2..10$, censored cells as lower bounds). Period four tower levels at every width, i.e. slope $1\/8$ per unknown bit. *Falsifying predictions (not yet run):* $W = 1$ reads 6 at $t = 12$; $W = 3$ reads 6 at $t = 10..12$ and 7 at $t = 13$; $W = 2$ reads 7 at $t = 14$.

#figure(image("fig_growth.svg", width: 100%), caption: [Recorded last fall degrees of the width ladder ($m = 2$, Kummer at $P_1$, two random targets per cell; triangles: censored lower bounds) and the fitted law (dashed). Stars: the falsifying predictions.])

= Statements and proofs (summary; full text in README)
*Theorem 1.* $phi: R_(t+1) -> R_t$, $z^e |-> z^(e mod 2^t)$, is a ring homomorphism with $"pop"(e mod 2^t) <= "pop"(e)$; it maps $y_j |-> y_j$ ($j < t$), $y_t |-> 1$, and $f^((t+1)) |-> f^((t))$. The set of $u in V_c (f^((t+1)))$ with $phi(u) in V_c (f^((t)))$ satisfies the closure property, so it is all of $V_c (f^((t+1)))$; hence $1 in V_c (f^((t+1))) => 1 in V_c (f^((t)))$.

*Theorem 2.* $e + 2^j >= 2^t$ iff bits $j..t-1$ of $e$ are set, so $"pop"(e) >= t - j$; the closure multiplies only elements of degree $< c$, so $j >= t - c + 1$. Over the twisted algebra $F_p (zeta)[z] \/ (z^K - zeta)$ the closure is identical outside this window, and a curve point with $z^K = zeta$ is a true solution; a growing lower bound needs a functional that corrects such points inside the top window. Open.

*Theorem 3.* The closure is a finite tree of rank decisions on matrices polynomial in the coefficients; the branches are constructible and partition affine space, so one is dense open. $S_3 (g z_1, g z_2, x_R)$ has the nine coefficients $g^4$, $-2 x_R g^3$ (×2), $x_R^2 g^2$ (×2), $-2(x_R^2 + a) g^2$, $-(2 a x_R + 4 b) g$ (×2), $a^2 - 4 b x_R$: generically all non-zero, so its support is the full $\{0,1,2\}^2$ box. No probability bound is claimed.

*Theorem 4.* If every monomial of the top form contains one variable $y$, then $y f$ falls (because $y^2 = 0$ in $"gr" R$) and is not in $V_D = ⟨f⟩$ because $f$ is a unit, so $"ffd" = D + 1$. $W = 1$: top $y_(10) y_(20)$, $D = 2$. $W = 3, 4$: the only popcount-2 exponent in $[0, W]$ is 3, top $y_(10) y_(11) y_(20) y_(21)$, $D = 4$. $W = 2$: D3's bilinear argument.

= Open, and formalization
Open: a lower bound growing with $t$ (Theorem 2 says where it must live), the period 4, the offsets $s_W$, isogeny towers. Lean: not done; the container has no Lean toolchain, and the Mathlib cache exceeds the disk allowance. Theorem 1 and the Lemma of Theorem 4 are short Lean targets for a session with the `crypto-lean` gate. Knowledge base unavailable in this session.
