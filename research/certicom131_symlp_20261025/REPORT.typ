#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#show link: set text(fill: blue)

#align(center)[
  #text(size: 16pt, weight: "bold")[
    Symmetry breaking and double large primes for the\
    131-bit Certicom curves (ECC2-131, ECCp-131)
  ]
  #v(4pt)
  Study `research/certicom131_symlp_20261025` · 2026-10-25 · seed 20261025\
  Module `src/cryptanalysis/certicom131_symlp` · branch
  `claude/certicom131-symlp-20261020`
]

= Question and scope

Does adding the two standard index-calculus accelerators — symmetry
breaking in the point decomposition and a double-large-prime relation
graph — move the two non-Koblitz 131-bit Certicom challenges closer to
Pollard rho? ECC2-131 is a random binary curve over
$"GF"(2^131)$ (field polynomial $z^131 + z^13 + z^2 + z + 1$, cofactor 2);
ECCp-131 is over a 131-bit prime (cofactor 1). 131 is prime, so there is
no Weil descent to a subfield, and ECC2-131 has $a in.not "GF"(2)$, so no
Frobenius saving. This study measures the two accelerators on the
smallest decomposition arity, $m = 2$, and on 61–127-bit same-family
calibration curves, and prices them against rho measured with the same
field code on the same host. It makes no key-recovery claim: every
end-to-end run uses planted known logarithms and a synthetic target, and
the published challenge points are only checked to lie in the subgroup.

= Method

The factor base is the $F_2$-subspace $V_ell = {x : deg x < ell}$ (Diem /
Faugère–Perret–Petit–Renault), with a large-prime domain $V_(ell')$,
$ell < ell'$. A relation $R = plus.minus P_1 plus.minus P_2$ is a pair of
subgroup abscissae with $S_3(x_1, x_2, x_R) = 0$. In the elementary
symmetric variable $e_1 = x_1 + x_2$ the binary $S_3$ linearises to
$t^2 + t = e_1^2 + b\/x_R^2$; because $V_(ell')$ is closed under addition,
$x_1, x_2 in V_(ell')$ forces $e_1 in V_(ell')$, so the *symmetric oracle*
sweeps $e_1$ (each unordered pair once, one batched inversion) against a
*baseline* that sweeps $x_1$ and lifts each point with its own inversion.
For ECCp-131 the interval $[0, B)$ is not closed under addition, so $e_1$
ranges over $[0, 2B)$ and each step costs two square roots; the prime
oracle is included to show the saving is special to the binary subspace.

Partial relations (1 or 2 large primes) enter a union–find graph over
$ZZ\/r$ for the 130-bit subgroup order (Gaudry–Thomé–Thériault–Diem);
every emitted full relation carries its input ids and is replayed from
them. All code is in `src/cryptanalysis/certicom131_symlp`
(`gf2w`, `curves`, `sym2`, `sym2p`, `relgraph`, `planted`), tested by
`cargo test --lib certicom131_symlp` (18 tests). The driver is
`examples/certicom131_symlp_measure`; records are in `runs/`.

= Measured values

All rates are single-thread on this host, seed 20261025. The IC
decomposition step and the rho adding-walk step use the same field
implementation in the same process.

#figure(image("figures/fig1_step_rate.svg", width: 100%),
  caption: [Decomposition-oracle step rate against the rho reference
  (dashed). Binary: the symmetric $e_1$ oracle runs above the rho step and
  ~17–29× the per-point baseline, because the baseline pays one field
  inversion per point and the symmetric sweep batches them. Prime: the
  symmetric oracle is ~2× *slower* than its baseline (two square roots per
  step), and both are well under rho.])

#table(columns: 7, align: (left, left, right, right, right, right, right),
  [family], [curve], [n], [base M/s], [sym M/s], [sym/base], [IC/rho step],
  [binary], [ecc2-131], [131], [0.18], [5.24], [28.7×], [1.20],
  [binary], [cal], [127], [0.36], [7.59], [20.8×], [1.74],
  [binary], [cal], [61], [0.39], [4.52], [11.6×], [1.04],
  [prime], [eccp-131], [131], [0.17], [0.09], [0.6×], [0.02],
  [prime], [cal], [61], [0.37], [0.19], [0.5×], [0.03],
)
#align(center)[#text(size: 8pt)[Full table: `tables/measured.md`.]]

#figure(image("figures/fig2_yield_split.svg", width: 72%),
  caption: [Decomposition yield by large-prime count, full $V_(ell')$
  sweep over random targets on binary calibration curves. Direct relations
  (both abscissae in $V_ell$, zero large primes) are a fraction
  $tilde 5 times 10^(-7)$ of decompositions; the 2-large-prime relations
  are the bulk, so without the graph almost every decomposition is
  discarded.])

The planted end-to-end pipeline (`planted.rs`) runs on the *real*
131-bit groups: it closes graph cycles, re-checks every full relation as
a point equation in the group, solves the linear system modulo the
130-bit $r$, and recovers the planted logarithm (both curves, in the test
suite).

= Derived cost model

#align(center)[#emph[Every row here is computed, not measured.]]

At the balancing dimension $d = (n+1)\/3$ an $m = 2$ solve needs
$tilde 2^(d-1)$ relations at success probability $tilde 2^(2d-n-2)$, i.e.
$tilde 2^(2n\/3)$ decomposition steps — the method's floor. Rho needs
$tilde sqrt(pi r \/ 4)$ steps.

#table(columns: 5, align: (left, right, right, right, right),
  [curve], [n], [log₂ steps(IC m=2)], [log₂ steps(rho)], [log₂ distance],
  [ECC2-131], [131], [87.3], [64.8], [22.5],
  [ECCp-131], [131], [87.3], [65.3], [22.0],
)

The $m = 2$ exponent $2n\/3$ agrees with the merged algebraic ladder's
measured trial slope $0.647 in [0.639, 0.655]$ (PR #2493): enumeration and
the linear solve share the $2^(2n\/3)$ optimum. Symmetry breaking (a
measured ~29× per-step saving on binary) and the double-large-prime graph
(which rescues the 2-lp bulk of the yield) are constant-factor gains over
that baseline. Neither changes the $2n\/3$ vs $n\/2$ exponent gap, so the
distance to rho at $n = 131$ stays $tilde 2^(n\/6) approx 2^22$ steps,
*before* the linear-algebra floor, which the program gates
(`docs/ic/CERTICOM131_IC_PROGRAM.md` §2) place above rho for $m <= 3$ on
both curves.

= Findings and limits

- *Measured, binary:* the symmetric $e_1$ oracle's step costs about the
  same as a rho step on this host (IC/rho 1.0–1.7×) and ~17–29× the
  per-point baseline. This is a genuine, measured improvement to the
  index-calculus step cost.
- *Measured, prime:* no symmetry saving; the symmetric step is ~2× dearer,
  confirming the subspace-vs-interval asymmetry.
- *Measured yield:* direct relations are $tilde 5 times 10^(-7)$ of
  decompositions; the double-large-prime graph is what makes the bulk
  usable, and the full-size planted pipeline recovers a logarithm through
  it on both real groups.
- *Distance remaining (derived):* $tilde 2^22$ steps at $n = 131$ for
  $m = 2$, unchanged in exponent by either accelerator; the realistic
  distance including linear algebra is larger (program gates).
- *Limits:* calibration curves are not point-counted (yield uses random
  targets, not full solves); the cost model is a model, labelled
  throughout; no measurement bears on the published challenge points. The
  accelerators matter for $m >= 4$, which this study does not reach.
