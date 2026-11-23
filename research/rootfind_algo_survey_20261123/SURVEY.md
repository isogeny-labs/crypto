# Root finding: is there an algorithmic (counted-cost) lever left? (2026-11-23)

Status: survey and derivation. No measurement in this note. Numbers quoted
from other studies carry those studies' standing; most are
`[pcc:unverified]`, L0. Statements marked **[derived]** are arithmetic in
this note, not measurements.

Knowledge-base receipt: the `research-kb` binary and the sibling
`crypto-autoresearcher` checkout (the lead atlas) are absent from this
container (`scripts/research-kb`: "no executable found"). The survey below
is a direct read of about 75 `research/` directories instead.

## What "root finding" means here

There are two separate programs.

- **(A) The decomposition oracle inside index calculus.**
  - The task: find base points with ±P_i ±P_j (±P_k) = R. These are the
    rational zeros of Semaev's S3/S4 on the base.
  - Oracles: `rootfind-subtract`, `rootfind-mitm` (pair table),
    `rootfind-pairs`, `rootfind-negfold`, `-window`, `-batch`, `-harvest`,
    `-torsion`, `mitm-prime-batch`, plus `ksum:m=k` and the triple table.
  - Curve families: binary Koblitz E0 over F_{2^n}, n = 13…37; prime-field
    abscissa bases, 9–40 bits.
- **(B) Exact all-roots solving of the original S3 system.** Lattice
  certificates on NIST charts and GF(2^83) norm cells (the
  `rootfinding_*` dirs). Its gate is genuine matched F4/F5/F6, but no
  genuine F4/F5/F6 arm has ever been run there.

## Incumbents (counted S = GAE/√r)

| family | incumbent | S | vs matched rho | source |
|---|---|---|---|---|
| prime, 20–32 bit | `ksum:m=5` | 6.3–7.4 | 5.4–7.3× rho-neg | `prime_ksum_vs_incumbent_20261029` |
| prime, exponent | `ksum4` α = 0.521 [0.506, 0.538] | — | rho-neg α = 0.435 | `prime_ksum_table_20261028` |
| Koblitz n = 13 | `rootfind-negfold` m=2 d=5 | 22.97 | about 1.55× rho-strong | `ecbench_rootfind_faster_20261014` |
| Koblitz n = 13…37 | PDP root finder | — | 2.9×, 15×, 61×, 3,053×, 25,116× | `pdp_root_finder_ecbench_20261013` |

No study claims IC below a matched rho in counted S. The 0.77× of
`ecbench_rootfind_speedup_20261014` is against a lower-bound rho-strong;
the signed-Frobenius rho there reaches S ≈ 1.9, about 6× below every IC
arm. The triple table's "0.81–0.93× rho" (`ic_triple_table_20260923`) is
against an unmatched rho; its own correction says the matched rho wins by
1.19–2.88×.

## Algorithmic levers already tried (verdicts as recorded)

| Lever | Verdict | Source |
|---|---|---|
| Pair table vs direct subtraction | supported; still above rho | `ecbench_rootfinding_assessment` |
| k-sum k = 3, 4, 5 | supported: α 0.620 → 0.521; floor BSGS | `prime_ksum_*` |
| Triple table | rate change; loses at small n | `ic_triple_table` |
| Sibling pairs (ksum, walk) | constant factor at w = 0; tie at w = 1 | `ic_ksum_sibling_20261118` |
| Window | −17 % S; dead end at d = 4 | `ecbench_rootfind_improve` |
| Stride-renew targets | −47 % at n = 13; 4.68× rho at n = 19 | `ecbench_rootfind_speedup` |
| m = 3 vs m = 2, prime | falsified, 1.43× worse | `ecbench_rootfinding_arity` |
| Low-memory 3-sum (Schroeppel–Shamir) | structural null | `ic_3sum_lowmem_20261117` |
| Multi-hit, block-tail, first-independent | trade-off / negative | `rootfind_multihit_relations`, `rootfind_block_tail_relations`, `rootfind_first_independent` |
| Single / double large primes | negative, 1.08–1.60× more S | `gaudry_k4_char2_crossover_20261015` |
| Fixed-offset probes `R ± T_i` (this session) | null: about one walk step per pair of probes | `ksum_fast_ic_20261118` round 3 |

## Why the remaining levers are constants [derived]

1. **Prime fields: generic decomposition.**
   - With A unknowns and a table of E entries, collecting about A relations
     costs A·r/(2E) walk steps plus E to build the table. Balanced, that is
     about √(2A·r), so S ≈ √(2A)·c.
   - At A = 19 that predicts about 6, and `ksum5` measures 5.4–7.3× rho-neg.
     The gap to rho is the number of unknowns, not the root finder.
   - A larger k only pushes the exponent from k/(2k−1) toward 1/2 (BSGS).
     Nothing in a prime-abscissa base is "smooth" in a way that would give
     a non-generic yield.
2. **Binary, m = 2, subspace V of dimension d.**
   - The decomposition probability is about 2^{2d−n}, and the pair table
     has 2^{2d} entries. Balancing the build against trials gives d ≈ n/3
     and cost 2^{2n/3}: the generic law again.
   - The measured ratio to rho grows from 2.9× to 25,116× over n = 13…37.
3. **The characteristic-2 algebra of S3 (checked here, not new
   structure).**
   - For y² + xy = x³ + a2 x² + a6, take e1 = x1 + x2 and e2 = x1 x2. The
     summation polynomial becomes (e2 + x3 e1)² + x3 e2 + a6. That is
     F_2-affine in (e1, e2), because squaring is F_2-linear.
   - With x1 = u, x2 = u + e1 and u, e1 ∈ V, decomposition is a bilinear
     F_2 system in 2d unknowns with n equations. Fixing u makes it linear
     in e1, but that is still 2^d linear solves per target: the cost of
     enumerating base points.
   - Bilinear-system Gröbner bounds (Faugère, Safey El Din and Spaenlehauer)
     do not make this polynomial when d ≈ n/2.
4. **Large-prime variations pay only when linear algebra dominates.** Here
   it is under 1 % of the cost (`gaudry_k4_char2_crossover`).

## Where real algorithmic content remains

- **Binary root finding as polynomial-system solving (program B, and m ≥ 3
  with Weil descent).** This is the only place where the "root finder" is
  an algorithm with non-generic structure: symmetrization (FGHR), Semaev's
  quadratic splitting, first-fall-degree behaviour. The lab has the code
  (`fghr_full.rs`, `gaudry_cubic_*`, F4 traces) but has never run a
  genuine matched F4/F5/F6 arm against the root finders.
- **Genus ≥ 2 (the pinned goal).** In hyperelliptic Jacobians,
  decomposition plus double large prime beats rho's exponent (2 − 2/g
  against g/2). That is a "root finding" setting where yield is
  non-generic.
- **Prime and Koblitz decomposition oracles.** Only constant-factor work
  is left, and the lab has mined it extensively.

## Addendum: the binary F4 direction (owner's choice, same day)

A second read covered the binary algebraic program
(`ic_m3_algebraic_vs_mitm_20261014`, `first_fall_closure_20261111`,
`binary_falldeg_20261118`, `hky_engine_speed*_20261117`,
`ic_algebraic_crossover_20261023`). It already holds the answer to that
direction's main question, under a frozen protocol:

- **Does any algebraic decomposition grow more slowly in n than the
  meet-in-the-middle floor?** On Koblitz m = 3 and m = 4, no.
  - These arms grow faster than the floor, with the 95 % slope-difference
    interval above 0: F4 on the S₃ chain, F6-IC, the unsplit-S₄ hybrids,
    and the wide chain solver at m = 4.
  - SAT and the S₃+S₄ split hybrid are undetermined, censored early. Their
    lower bounds sit 10⁶–10⁷ above the floor.
- **First fall degree.** The questions are closed
  (`first_fall_closure_20261111`): the Kosters–Yeo trace relation is
  Lean-checked, an exact count of degree-≤1 relations is proved, and the
  Kousidis–Wiemers bound `d_ff ≤ m² − m + 1` is reduced to a rank condition
  at m = 3. By owner decision (2026-11-11) no more measurement time goes to
  the first fall degree.
- **Frobenius-orbit against random bases.** The fall degree is under study
  in `binary_falldeg_20261118`; its data so far are pilot rows.

This agrees with the literature: Diem's subexponential results for binary
curves rest on a first-fall-degree assumption that Huang–Kosters–Yeo and
others give evidence against. A new structural lever here would have to
change the degree behaviour itself, not the solver.

The one recorded gap is a measurement gap: program B (exact all-roots S3
solving) has never been run against a genuine matched F4/F5/F6 arm, and
its 2×/8× gates are unscored.
