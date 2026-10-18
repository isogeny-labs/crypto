#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 15pt, weight: "bold")[GLS-enabled index calculus at $k = 4$: \ the GLS endomorphism on the $V_4$ line (E25)]
  #v(4pt)
  `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.15 (registered before code) and §9.16 · 2026-10-18
]

= Result

On GLS twists over $FF_(p^4)$ of curves over $FF_(p^2)$ whose 2-torsion the twist splits, the GLS
endomorphism $psi$ ($psi^2 = -1$) keeps E24's $V_4$ base line and acts on it as $z |-> plus.minus i slash z$,
so the base folds by $chevron.l V_4, psi chevron.r$ with $8$ points a column instead of $4$. Measured end to
end on one curve, one base and one target stream (10 curves, $p = 101$–$397$), *the $psi$ fold cuts
the $k = 4$ index calculus by $1.35$–$2.67 times$ (median $1.86 times$), and against the matched
$chevron.l plus.minus psi chevron.r$ walk the $psi$ arm gains $1.26$–$1.97 times$ over the control at every
$p >= 197$.* The modelled handover on these curves moves from $2^(93.2)$ (control) to $2^(88.0)$.
Every logarithm on both arms and every one of the 960 walks verified. *Class: engineering, a
constant* — the fitted exponent of $S slash "rho"$ is $-0.29$ against $-0.30$.

*Distance remaining.* Every row is still $1.1 dot 10^4$–$2.7 dot 10^5 times$ slower than the matched
rho ($1.13 dot 10^4 times$ at $p = 397$, the best). *What it does not do.* Every curve of this
construction has cofactor $h = 16$ (E24's had $4$), which makes rho $2 times$ cheaper at a given $p$
and absorbs most of the fold at matched $n$: arm B's own handover $2^(88.0)$ against E24's own
$2^(90.1)$, from different curve families. *Scope:* $p equiv 1 med (mod 4)$, $p <= 397$,
$n <= 2^(30.5)$.

#figure(image("figures/arms_s.svg", width: 90%), caption: [$S$ end to end per curve, control (A, $V_4$ fold) and $psi$ arm (B, $chevron.l V_4, psi chevron.r$ fold), same curve, base and target stream; run `experiments/30_gls_v4` (binary `46202dc97`).])

= What ran

Registered in §9.15 before any code. Curves: `generate_gls_klein_instance` — $e_2 in FF_(p^4) without FF_(p^2)$,
$e_3 = e_2^(p^2)$, $e_1 = -e_2 - e_3$, roots $mu e_i$ with $mu$ a non-square; the GLS order
$(p^2 - 1)^2 + t^2$, $psi^2 = -1$, $psi(G) = [lambda] G$, $psi$ fixing $T_1$ and swapping $T_2, T_3$ all checked
on points. Arms: E24's pipeline unchanged (the $V_4$-invariant $S_5$ solve at weighted degree 11,
one verified decomposition per target, Wiedemann until $[d]G = Q$ verifies, every phase charged in
$FF_p$ multiplications), with the base folded by $V_4$ (A) or $chevron.l V_4, psi chevron.r$ (B, coefficients
$plus.minus lambda^e$ each checked on points). Walks: 16 each of $chevron.l plus.minus psi chevron.r$, negation and
unfolded per curve. Host: one shared 4-vCPU container; operation counts only.

*Stage A (gate), 56 curves, $p = 29$–$397$:* every curve exists; $8.00$ points a column on every
non-empty base; $nu^2 in.not FF_p$ on all; distinct column logarithms equal the column count wherever
computed ($p <= 149$): not confined. Density $0.49$ pooled, $0$–$0.75$ per curve (orbits of 8 lift
together). $h = 16$ on every curve.

#table(
  columns: 9,
  align: right,
  [*p*], [*seed*], [*$log_2 n$*], [*cols A/B*], [*$S_A slash S_B$*], [*neg/$plus.minus psi$ steps*], [*$S_A slash$ rho neg*], [*$S_B slash$ rho $plus.minus psi$*], [*impr.*],
  [101], [1], [22.6], [10/5], [1.94], [1.03], [$1.53 dot 10^5$], [$6.77 dot 10^4$], [2.26],
  [101], [2], [22.6], [14/7], [2.11], [1.49], [$6.02 dot 10^4$], [$2.93 dot 10^4$], [2.05],
  [149], [1], [24.9], [10/5], [2.67], [1.09], [$8.31 dot 10^5$], [$2.65 dot 10^5$], [3.13],
  [149], [2], [24.9], [14/7], [1.77], [0.96], [$1.58 dot 10^5$], [$7.01 dot 10^4$], [2.25],
  [197], [1], [26.5], [22/11], [1.35], [1.52], [$7.71 dot 10^4$], [$6.11 dot 10^4$], [1.26],
  [197], [2], [26.5], [30/15], [1.69], [1.62], [$3.97 dot 10^4$], [$2.62 dot 10^4$], [1.51],
  [269], [1], [28.3], [30/15], [1.74], [1.61], [$7.45 dot 10^4$], [$4.84 dot 10^4$], [1.54],
  [269], [2], [28.3], [40/20], [2.02], [2.08], [$2.24 dot 10^4$], [$1.56 dot 10^4$], [1.44],
  [397], [1], [30.5], [60/30], [1.72], [1.36], [$1.99 dot 10^4$], [$1.13 dot 10^4$], [1.76],
  [397], [2], [30.5], [48/24], [2.32], [1.67], [$3.38 dot 10^4$], [$1.71 dot 10^4$], [1.97],
)

#figure(image("figures/distance_to_rho.svg", width: 78%), caption: [$S slash "rho"$ against $log_2 n$: E25 arm A against the negation walk, arm B against its matched $chevron.l plus.minus psi chevron.r$ walk, and E24's 12 curves ($h = 4$) against the negation walk; 16 walks per curve and fold.])

= Predictions and verdicts

#table(
  columns: 3,
  align: left,
  [*id*], [*registered*], [*measured*],
  [G2.32], [curves exist, $psi$ verified, base $psi$-stable, $>= 7.8$ pts/col, density $[0.3, 0.7]$, true coefficients, log recovered], [holds; density $0.49$ pooled, 14 of 56 curves outside per curve (the band did not say which)],
  [G2.33], [cols A/B in $[1.9, 2.0]$], [$2.00$ on every curve: holds],
  [G2.34], [$y_B slash y_A$ in $[0.8, 1.25]$], [$0.69$–$1.42$, pooled $0.91$: falsified per curve (5–60 relations a curve)],
  [G2.35], [$S_A slash S_B$ in $[1.6, 2.4]$], [$1.35$–$2.67$, median $1.86$: falsified per curve (8 of 10 in band)],
  [G2.36], [steps mean $[1.2, 1.6]$; improvement $[1.1, 2.0]$], [steps $1.44$ holds; improvement in band at $p >= 197$, above it at $p <= 149$, where walk set-up dominates],
  [G2.37], [$n^* in [2^(84), 2^(92)]$], [$2^(88.0)$ ($2^(85.7)$ from $p = 397$): holds],
)

#figure(image("figures/walks.svg", width: 95%), caption: [The matched walks: steps (left) and $FF_p$ multiplications with set-up and class representatives (right), negation against $chevron.l plus.minus psi chevron.r$, 16 walks each per curve. Below $n approx 2^(25)$ set-up dominates and the $psi$ walk costs more.])

= Addendum: a GLS construction with $h = 4$ (E26)

Registered in §9.17 before code; results §9.18. On the GLS twist, $E(FF_(q^2))$ is a $ZZ[i]$-module
killed by $(q - 1) - t psi$, so its 2-part is $(ZZ slash 2)^2$ exactly when $t equiv 2 med (mod 4)$. The
$V_4$ line needs the halves of $T_1$ and $T_2$ to have rational abscissae; when $h = 4$ that forces
$E_0[4] subset E_0(FF_(q^2))$, i.e. $pi^2 equiv 1 med (mod 4)$ for the $q$-Frobenius of $E_0$, which is
impossible when $pi$ swaps $T_2, T_3$ and $det pi = q equiv 1 med (mod 4)$. *Census of 560 twists,
$p = 29$–$397$:* the 310 curves with $t equiv 2 med (mod 4)$ all have $v_2(\#E) = 2$ and all fail the
halves condition; the 250 with $4 | t$ all satisfy it and all have $v_2(\#E) >= 4$. *No GLS twist
carries the $V_4$ line with $h = 4$; E25's $h = 16$ is the minimum* (accounting; records
`experiments/31_gls_cofactor/`).

#figure(image("figures/census.svg", width: 85%), caption: [E26: per $p$, the 40 GLS twists split into the two classes; the $V_4$ line's condition holds exactly on the $4 | t$ class.])

= Records

Raw (E26): `experiments/31_gls_cofactor/census.json`. Raw (E25): `experiments/30_gls_v4/p{101,149,197,269,397}s{1,2}.json` (both arms in each), `stage_a.json`,
`run.log`, `tables.md`. Code: `src/cryptanalysis/quartic_folds.rs` (`generate_gls_klein_instance`,
`klein_base_with`, `RhoFold::Gls`, `run_k4_gls_pipeline`, `run_gls_stage_a`), `examples/quartic_pipeline_gls.rs`,
`examples/gls_stage_a.rs`. Figures: `figures/plot.py` (recorded values only).
