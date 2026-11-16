#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= K2 flatness test: GF(2) degree-bounded closure of binary decomposition systems (m = 3, 4)
_2026-11-15, research/k2_flatness_20261115. All numbers [pcc:unverified]. Frozen protocol: PROTOCOL.md (committed before the grid)._

*Question.* Does the degree-bounded GF(2) closure of the Weil-descended m = 3 / m = 4 point-decomposition
system solve it at a degree that stays flat as n grows (k ~ n/m), or does the degree grow?

*Premise correction.* The direct $S_4$ descent has Boolean degree 6 ($S_4$ is quartic in each abscissa),
so a degree-4 closure of the direct m = 3 system is undefined. Measured on the chain presentation
($S_3 (x_1, x_2, U) = 0$, $S_3 (U, x_3, x_R) = 0$; m = 4 with two auxiliary abscissae), every equation of degree at most 3,
N = mk + (m-2)n Boolean variables.

*Instrument.* Native standalone Rust (k2close.rs): closure $W_D$ under multiplication by variables, semi-echelon GF(2)
elimination, solutions by enumeration; solved at D iff rank $W_D$ equals the dimension of the degree-at-most-D part of the vanishing ideal.

*Controls.* Positive (m = 2 direct $S_3$): PASS. Negative (random systems of the chain's shape): PASS.

#table(columns: 10, align: right, stroke: 0.4pt,
  [*series*], [*m*], [*n*], [*k*], [*N*], [*targets (sat)*], [*D_sol*], [*d_reg*], [*cols D=4*], [*s at D=4*],
  [A], [3], [11], [4], [23], [4 (0)], [4 4 4 4], [7], [10903], [1.5],
  [A], [3], [12], [4], [24], [4 (2)], [4 4 4 4], [7], [12951], [2.2],
  [A], [3], [13], [4], [25], [4 (0)], [4 4 4 4], [7], [15276], [4.4],
  [A], [3], [14], [5], [29], [4 (0)], [4 4 4 4], [8], [27841], [17.4],
  [A], [3], [15], [5], [30], [4 (2)], [4 4 4 4], [8], [31931], [28.8],
  [A], [3], [16], [5], [31], [4 (0)], [4 4 4 4], [8], [36457], [44.5],
  [A], [3], [17], [6], [35], [4 (2)], [4 4 4 4], [8], [59536], [486.6],
  [A], [3], [18], [6], [36], [4 (0)], [4 4 4 4], [9], [66712], [351.8],
  [B], [3], [15], [3], [24], [4 (0)], [4 4 4 4], [6], [12951], [1.9],
  [B], [3], [15], [4], [27], [4 (0)], [4 4 4 4], [7], [20854], [10.0],
  [B], [3], [15], [6], [33], [4 (2)], [4 4 4 4], [8], [46938], [213.2],
  [C], [4], [8], [2], [24], [4 (1)], [4 4 4 4], [7], [12951], [3.0],
  [C], [4], [9], [2], [26], [4 (0)], [4 4 4 4], [7], [17902], [7.7],
  [C], [4], [10], [2], [28], [4 (0)], [4 4 4 4], [8], [24158], [16.3],
  [C], [4], [11], [3], [34], [4 (0)], [4 4 4 4], [9], [52956], [388.8],
  [C], [4], [12], [3], [36], [1 (1)], [4], [9], [66712], [942.2],
  [N], [3], [12], [4], [24], [1 (1)], [\>4], [-], [12951], [0.2],
  [N], [4], [8], [2], [24], [1 (1)], [\>4], [-], [12951], [0.1],
  [P], [2], [16], [8], [16], [4 (2)], [3 3 3 3], [5], [-], [-],
  [P], [2], [20], [10], [20], [4 (2)], [4 4 4 4], [5], [6196], [0.5]
)

D_sol: first degree at which the closure solves the system (">4" = not solved at the largest degree computed);
d_reg: Boolean semi-regular degree of the same shape.

#figure(image("figures/k2_dsol.svg", width: 85%))

*Verdict (registered rule).* Series A (m = 3): *flat*; series C (m = 4): *flat*; overall: *flat* on the measured range.

*What it does not show.* Nothing beyond the measured n (k at most 6 for m = 3, 3 for m = 4); not the lead's square cells
(k = 10, 11; k = 8); not a cost below rho: the per-relation yield 2^(mk-n)/m! is unchanged and the solve matrix
M(N, at most 4) grows as N^4. Closure degree is a proxy for solver cost (WEAKNESSES E-GB-064).
