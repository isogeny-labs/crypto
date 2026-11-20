#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")

#align(center)[
  #text(size: 16pt, weight: "bold")[Riemann–Roch decomposition: three cheap yes/no tests]

  2026-11-15 · `research/rr_directions_20261115/` · standing of every statement: `[pcc:unverified]`
]

*Scope.* Three directions the owner named for the Riemann–Roch (Nagao) step of elliptic-curve
index calculus: (1) torsion-quotient versions of the RR system, (2) SAT against F4 on the quadratic
RR system over $FF_(2^m)$, (3) a parameter count for a lattice/Coppersmith attack on the small-$x$
factor base through the RR parametrisation. Pick 1 is closed by a derived identity checked
exhaustively at three field sizes; pick 3 by a count checked by brute force at five primes; pick 2
is a stage diagnostic measured in this PR. No end-to-end cost, no rho comparison, nothing about
$n = 83$ or $131$; local L0 runs on a shared 4-vCPU container.

= Torsion quotient of the RR system: the quotient is the curve

On $E : y^2 + x y = x^3 + 1$ over $FF_(2^m)$, $m$ odd, with $T = (0,1)$ and $tau(x,y) = (x^2, y^2)$:

+ $x(P + T) = 1 \/ x(P)$ (divisor of $x compose tau_T$ is $2(O) - 2(T) = "div"(1\/x)$);
+ $t := x + 1\/x$ is the quotient coordinate of $E -> E\/angle.l T angle.r$;
+ $overline(tau) = -1 - tau$ has $tau overline(tau) = 2$ and $ker overline(tau) = angle.l T angle.r$, so
  $E \/ angle.l T angle.r tilde.equiv E$, and *$x(P + tau P) = t(P)$ exactly*.

Checked on every affine point at $m = 5, 9, 11$ (42, 506, 2114 points; `pick1_torsion_quotient/isogeny_check.txt`),
and reproduced by `examples/rr_directions_checks.rs isogeny`. Hence the torsion-quotient RR system with
factor base ${t in V}$ is the $x$-form RR system on the same curve with the target moved to
$overline(tau)(-R)$: same equation, same subspace distribution, same target distribution. Its solving
degree is the one `ic_rr_norm_ladder_20260930` already measured ($approx ell + 2$ at $m = 3$, a constant
lever); the variable count does not drop ($t_i in V$ still costs $ell$ bits per summand). For a
general binary curve the quotient is the 2-isogenous curve, whose solver cost the lab recorded as an
isogeny-class invariant; over prime fields with prime order there is no rational 2-torsion.

= Lattices for the small-$x$ base through the RR parametrisation: the count

$f in L((m+1) O)$ monic, $f(-R) = 0$ leaves $a in FF_p^(m-1)$; $F_a = N(f) \/ (X - x_R)$ has
coefficients $c_k (a)$ quadratic in $a$. A relation needs $F_a$ to split with roots in $[0, B]$,
$B approx p^(1\/m)$. The necessary box condition $|c_k| <= binom(m,k) B^k$ admits

$ p^(m-1) product_k binom(m,k) B^k \/ p = p^((m-1)\/2) product_k binom(m,k) $

pairs per target against $approx 1\/m!$ genuine decompositions: the box has volume
$B^(m(m+1)\/2)$ around a variety of size $B^m$. Eliminating $a$ gives one modular equation in $m$
small unknowns with $product X_k = p^((m+1)\/2) > p$, outside Coppersmith's reach even for a linear
equation; shrinking $B$ into reach kills the yield.

Brute force at $m = 3$, $y^2 = x^3 + 2x + 3$, all $(a,b) in FF_p^2$, 8 targets per prime
(`pick3_lattice_count/lattice_count.json`; Rust: `examples/rr_directions_checks.rs lattice`):

#table(
  columns: 7, align: (right, right, right, right, right, right, right),
  [*p*], [*B*], [*boxes*], [*box-passing / target*], [*heuristic*], [*splitting*], [*relations*],
  [503], [8], [25, 193, 513], [4820.1], [4825.0], [0.50], [0.50],
  [1019], [10], [31, 301, 1001], [9161.1], [9166.2], [0.00], [0.00],
  [2027], [13], [40, 508, 2198], [20305.8], [20320.0], [0.63], [0.63],
  [4091], [16], [49, 769, 4097], [37698.9], [37681.0], [0.25], [0.25],
  [8191], [20], [61, 1201, 8001], [71594.9], [71561.6], [0.25], [0.25],
)

#figure(image("figures/lattice_count.svg", width: 85%),
  caption: [Pick 3: box-passing $(a,b)$ per target (measured and heuristic) against the splitting and
  genuine-relation counts; $m = 3$, $B = "round"(p^(1\/3))$, 8 targets per prime, data `lattice_count.json`.])

*Verdict.* The idea fails the parameter count: the quantity a lattice can make small is not
selective enough by a polynomial factor in $p$.

= SAT against F4 on the quadratic RR system over $FF_(2^m)$

Preregistered in `PREREGISTRATION.md` before the panel. Instrument: `examples/rr_quadratic_panel.rs`
(Rust: draws, both systems from the library's symbolic field arithmetic, in-process Boolean F4
`pq_f4_f2`, in-process native-XOR CDCL `cryptanalysis::sat`, DIMACS/XOR export); CryptoMiniSat 5.18
run on the exported files as an external cross-check. Every solution and model is decoded and
verified by group arithmetic; the ground truth per draw is the group-arithmetic count of ordered
factor-base triples summing to $R$.

#include "RESULTS.typ"
