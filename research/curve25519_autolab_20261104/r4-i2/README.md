# R4-I2 Stage 0: the 2-torsion quotient on the k = 20 Curve25519-family miniature

2026-11-04 · pre-registration: `../ideas/STRUCTURE-R4.md`, record `R4-I2-linear-2torsion-quotient-family`
· generator [`gen.py`](gen.py) · runner [`run.py`](run.py) · table [`summarize.py`](summarize.py) →
[`results.tsv`](results.tsv) · systems [`systems/`](systems/) (hashes in [`SYSTEMS.sha256`](SYSTEMS.sha256))
· logs [`logs/`](logs/) · Singular elimination [`ji.sing`](ji.sing), [`logs/ji_elim.out`](logs/ji_elim.out)

**Standing:** every number below comes from a run in this directory and has no claim id. Each is tagged
[pcc:unverified]. Nothing here changes the ledger, the leaderboard or any status. No ecbench session was run
(Stage 0 specifies none).

## Result in one paragraph

Stage 0 does **not** pass as pre-registered [pcc:unverified]. Arm A (the #3408 quotient on E) lowers the
F4 solving degree by one at m = 3 and m = 4 in every cell that finishes (d' = 3 and 9), and the quotient
dimension divides by exactly 2^(m-1), but (a) at the pre-registered k = 20 base (d' = 27) every m = 3 and
m = 4 cell stops at the 2 GiB or 1,200 s cap, so the degree comparison there is undecided, and (b) the m = 2
line (widest matrix at most half) fails at d' = 9 and d' = 27. Arm B (the 2-isogenous curve) is falsified on
its own distinctive line: the c-coordinate tree of the Jacobi-intersection model has the same shape as the
Edwards tree of a third curve, the quotient dimension divides by 2^(m-1) and not by 4^(m-1), and D equals
that of arm A in every finished cell. Stage 1 is not recommended as registered (section 5).

## 1. What was built

Curve and base (all checked in `gen.py` before any write, `self_check`):
- p = 1048573, Montgomery A = 926, #E = 8r with r = 131203. A + 2 = 928 is a nonsquare and A - 2 = 924 is a
  square [pcc:unverified], so the complete twisted Edwards model is a u^2 + v^2 = 1 + d u^2 v^2 with
  (a, d) = (A - 2, A + 2) = (924, 928) (the swap by v -> 1/v the record anticipates). Its group order was
  checked by scalar multiplication on random points: 8r times a point is the identity, and r times 8 times a
  point is the identity.
- Base: v in g * mu_N with N = 4 d', d' odd, d' | p - 1 = 4 * 27 * 7 * 19 * 73. Cells use d' = 27 (N = 108,
  the registered base), and d' = 9 (N = 36) and d' = 3 (N = 12) as supplementary cells that finish. mu_4 lies
  in F_p, so the 2-power tower is replaced by a short 2-power tail and a cubic part.
- Towers: a cherry (w1, w2) is carried as s = w1 + w2, q = w1 w2 with the remainder chain T^N = 1 modulo
  T^2 - sT + q. A cube is a square followed by a product, so every chain equation has degree at most 3. A
  single leaf w uses u -> u^f chains (degree at most 3). E1Q uses W = w^2 in mu_(N/2).
- Trees, as in round 7 (`../../ic_tree_gluing_20261103/round7_edwards/`): m = 2 is one root node, m = 3 is
  a cherry plus a leaf, m = 4 is two cherries. Every system passed a planted-point check on every equation
  before it was written (`write_ms`); targets are random planted sums with random point signs (target seeds
  1 and 2).

Arms:

| arm | model | coordinate | relation |
|:--|:--|:--|:--|
| T | E, Montgomery | x in mu_N | S3M, the x-only relation (control) |
| E1 | E, twisted Edwards | v in g mu_N | S3E |
| E1Q | E | invariants W = w^2, Y = X^2, signed products p | SI(A,B,C) - 2(a-d) pi = 0, pi^2 = ABC |
| JI | E' = E/<T0>, twisted Jacobi intersection a s^2 + c^2 = 1, b s^2 + d^2 = 1, (a, b) = (A + 2, A - 2) | c in g mu_N | S3J, from `ji.sing` |
| JIQ | E' | quotient of JI by c -> -c, same construction as E1Q | SI_J - 295051 pi = 0, pi^2 = ABC |

- E' check [pcc:unverified]: the JI curve with (a, b) = (928, 924) has 1049624 = 8r points (counted over all
  s in F_p, numpy), equal to the point count of y^2 = x(x - (A+2))(x - (A-2)), the 2-isogenous curve. The JI
  addition law is complete because a b = A^2 - 4 is a nonsquare. Associativity, 8r * P = O, and the three
  rational 2-torsion points (0, -1, -1), (0, -1, 1), (0, 1, -1) were checked in `self_check`. The 2-torsion
  translations act on (s, c, d) as the sign changes (s, -c, -d), (-s, -c, d), (-s, c, -d), as the record says.
- Msolve version: 0.10.1 from the `passagemath-msolve` wheel, not 0.6.5 as pre-registered. Flags are the
  registered ones (`-g 2 -t 1 -v 2`, 2 GiB address space, 1,200 s). msolve 0.10.1 does not print the quotient
  dimension with `-g 2`, so `run.py` counts the standard monomials of the reduced Groebner basis (checked: 12
  for E1 and 3 for E1Q at m = 3, d' = 3 and 9, the values round 7 recorded).
- Not run: the TQ negative control (x-line quotient), and any F4 run on the incumbent's own system.

## 2. Derived finding about arm B: the c-tree is an Edwards tree of a third curve

Singular eliminates s_i, d_i from the JI addition law (`ji.sing`) and returns, besides the denominator locus
c1^2 c2^2 - c1^2 - c2^2 - 376760, the relation [pcc:unverified]

S3J = c1^2 c2^2 c3^2 - (c1^2 c2^2 + c1^2 c3^2 + c2^2 c3^2) - 295051 c1 c2 c3 - 376760 (c1^2 + c2^2 + c3^2) + 376760.

- It has degree 2 in each variable and total degree 6, the shape of S3E. With rho = -376760 mod p it is the
  Edwards relation of a curve with a/d = rho, because 2 (rho - 1) = 295051 (mod p) [pcc:unverified].
- That curve has Montgomery A'' = 235023 and j = 307137, against j = 481878 for E, so it is E'/<T3>, a third
  curve in the isogeny class, not E and not E'.
- Reason: c has degree 4 on the JI curve. Its fibre is {P, -P, P + T3, -P + T3} with T3 = (0, 1, -1), so T3
  acts trivially on c and the (Z/2)^2 acts on c through a single sign. The three translations act by sign
  changes on the triple (s, c, d), but any one coordinate sees one sign group. A relation that sees the whole
  (Z/2)^2 needs both c and d per summand (they lie on the conic b c^2 - a d^2 = b - a, so d^2 is affine in
  c^2). That is a different system (two variables per summand), and it was not built here.

## 3. Stage 0 quantities against the pre-registration

Raw rows: [`results.tsv`](results.tsv) (D is the largest selected degree, ">=" when msolve stopped at the cap
and the value is a lower bound; widest is the widest matrix; sumcells is the sum of rows times columns over
rounds; qdim is the quotient dimension; exit -11 is the 2 GiB hash-table failure, 124 is the 1,200 s limit). Both
targets gave identical D, qdim and near-identical widths in every cell, so the table lists one value per cell.

**D, widest, quotient dimension per arm** [pcc:unverified]. Cells are (D, widest matrix, qdim).

| d' | m | T | E1 | E1Q | JI | JIQ |
|--:|--:|:--|:--|:--|:--|:--|
| 3 | 2 | 4, 108, 2 | 4, 108, 2 | 4, 35, 1 | 4, 108, 2 | 4, 35, 1 |
| 3 | 3 | 6, 3867, 12 | 6, 3915, 12 | 5, 907, 3 | 6, 3915, 12 | 5, 907, 3 |
| 3 | 4 | 6, 25657, 48 | 6, 16759, 48 | 5, 4353, 6 | 6, 16756, 48 | 5, 4353, 6 |
| 9 | 2 | 4, 500, 2 | 4, 495, 2 | 4, 308, 1 | 4, 495, 2 | 4, 308, 1 |
| 9 | 3 | 6, 31332, 12 | 6, 31655, 12 | 5, 6042, 3 | 6, 31655, 12 | 5, 6042, 3 |
| 9 | 4 | >=7, >=41952, NA | >=7, >=156265, NA | >=6, >=104611, NA | >=7, >=156265, NA | >=6, >=104611, NA |
| 27 | 2 | 4, 1751, 2 | 4, 1647, 2 | 4, 1124, 1 | 4, 1647, 2 | 4, 1124, 1 |
| 27 | 3 | >=6, >=61700, NA | >=6, >=60628, NA | >=6, >=72178, NA (1,200 s) | >=6, >=60628, NA | >=6, >=72178, NA (1,200 s) |
| 27 | 4 | >=6, >=68257, NA | >=7, >=52954, NA | >=6, >=28381, NA | >=7, >=52954, NA | >=6, >=28381, NA |

**Against each pre-registered line** [pcc:unverified]:

| line | prediction | measured | falsification line | status |
|:--|:--|:--|:--|:--|
| armA_m3 D | D_E1Q = D_E1 - 1 | d' = 3: 5 vs 6; d' = 9: 5 vs 6 (both targets). d' = 27: both >= 6 at the cap, undecided | arm A dead if D_E1Q >= D_E1 in every comparable cell | holds where decided; not decided at the registered base |
| armA_m4 D | D_E1Q <= D_E1 - 1 | d' = 3: 5 vs 6. d' = 9: >= 6 vs >= 7. d' = 27: >= 6 vs >= 7. The cap rows are lower bounds | same | holds at d' = 3 only |
| armB_m3 D | D_JIQ <= D_E1Q | 5 vs 5 (d' = 3, 9), equal in every finished cell; d' = 27 undecided | arm B dead if D_JIQ >= D_E1Q everywhere | equality everywhere, so the line "D_JIQ >= D_E1Q everywhere" holds: arm B has no D advantage over arm A |
| m2 widest | widest(E1Q) <= 0.5 widest(E1) | d' = 3: 35 / 108 = 0.32; d' = 9: 308 / 495 = 0.62; d' = 27: 1124 / 1647 = 0.68 | none stated | fails at d' = 9 and 27 |
| qdim arm A | ratio 2^(m-1) exactly | 2 -> 1 (m = 2, all d'); 12 -> 3 (m = 3, d' = 3, 9); 48 -> 6 (m = 4, d' = 3) | none stated | holds in every finished cell pair |
| qdim arm B | ratio 4^(m-1) against unquotiented JI | 2 -> 1, 12 -> 3, 48 -> 6, that is 2^(m-1) | arm B dead if the action is not by signs in any F_p model | fails: section 2 gives the reason |
| controls | T, E1 unquotiented | D_T = D_E1 where both finish (6 at m = 3, d' = 3, 9; 6 at m = 4, d' = 3), as round 7 found; E1 at d' = 9, m = 3 took 224 s against 707 s for T, at d' = 3, m = 4: 7.6 s against 502 s [pcc:unverified] | none | consistent with round 7; one host, a constant factor, no pipeline priced |

Other readings of the table, all [pcc:unverified]:
- At m = 2, the arity of the algebraic incumbent, D is 4 in both E1 and E1Q at every d'. There is no degree
  drop, only a smaller matrix: sum of rows times columns falls to 0.127 (d' = 3), 0.195 (d' = 9) and 0.323
  (d' = 27) of E1's.
- At d' = 27, m = 3, E1 and T stop at the 2 GiB cap after 226 s and 457 s, and E1Q is still in a degree 6 round
  at 1,200 s. The round-7 pattern (D drops by 1 or 2) cannot be confirmed or refuted on the registered base.
  The same failure appears at m = 4, d' = 9 and d' = 27. An informal retry of `d9_m4_s1_E1` at 6 GiB failed at the
  same round with the same allocation message (log not kept), so the limit looks like msolve's, not the address
  space.
- Cap rows are lower bounds and are not compared as values.

## 4. Does Stage 0 pass?

No, as pre-registered [pcc:unverified]:
- Arm A: partial support. It survives its falsification line (D falls by 1 in every resolved m >= 3 cell
  pair: d' = 3 at m = 3 and 4, d' = 9 at m = 3, both targets), the quotient dimension divides by 2^(m-1) exactly, and the m = 2 matrix is smaller by
  a constant. It fails the m = 2 line at the k = 20 base size and it is undecided at that base for m >= 3. The
  pre-registered probability of 0.5 for the D line stands as "supported on small bases, untested on the
  registered one".
- Arm B: fails. Single-coordinate JI is arm A on a third curve, so D and qdim match arm A and the 4^(m-1) claim
  is false for it.
- Not covered by this run: the two-variable (c, d) JI system that could see the whole (Z/2)^2.

## 5. What Stage 1 would need, and where it would land

Stage 1 is not recommended as registered. If the Coordinator wants it anyway, it needs:

1. **A base closed under the sign action.** The quotient needs a factor base closed under v -> -v of the
   Edwards model (translation by T0). The incumbent base `prime-isogeny:t=6,seed=1093` is a set of abscissae
   from an auxiliary-curve Velu chain (`src/cryptanalysis/ic_framework/prime_tower.rs`) with no such closure by
   construction; closure was not tested in Rust. `prime-kummer` needs 2^t | p - 1, and v2(p - 1) = 2 here. So
   a new factor-base plug-in is needed (points with v in g * mu_(4 d'), d' = 27), plus a new oracle model.
2. **An engine that takes cubic towers.** `pkm_semaev.rs` hands F4 a tower ring with the square rule
   y_(j)^2 = y_(j+1) (`f4_fp_tower`, `SquareRule`). A d' = 27 base needs a cube rule, or msolve called as in
   Stage 0.
3. **The model option itself** on `pkm-semaev`: E1Q's invariant variables and signed products at m = 2.
4. **A same-base, no-quotient control arm** (E1 on the same base). Changing the base moves the trial count and
   so S at w = 0 for reasons unrelated to the quotient, and the registered "S(w = 0) within 5% of 26.448"
   cannot hold across a base change. Equal-base class counts are needed (the record's own risk).
5. **Spec:** `ic.pipeline`, factor base the new plug-in at N = 108, oracle `pkm-semaev:m=2,mode=full` with the
   quotient model, panel {same-base control, quotient arm, `rho.negation`, `inc-aa`}, 6 targets by 2 rounds,
   and the incumbent for reference, as T5 did.

**Expected S at k = 20, against matched rho** [pcc:unverified, derived from the ledger and the table above]:
- w = 0: 26.448 for the incumbent against rho-neg 1.447, a gap of 18.3 (17.6 against 1.5). The quotient does not
  change the trial count, so the w = 0 gap stays as it is. The matched-rho rule is met in the trivial way:
  rho runs no polynomial systems, so no saving here is one rho could also use, and the saving is also priced
  only where native work is priced.
- w = 0.1 and 1: the incumbent's S is 1.073e6 and 1.073e7, almost all native work. At m = 2 the Stage 0
  evidence is a constant factor and no degree drop. Scaling the native part by the d' = 27 sum-of-cells ratio
  0.323 (an illustration only: the incumbent runs its own tower system, not these) gives about 3.5e5 at
  w = 0.1 (gap 2.2e5 to rho-neg 1.547, from 6.9e5) and about 3.5e6 at w = 1 (gap 1.4e6 to 2.448, from 4.4e6).
  The pre-registered range "2x to 300x" at w = 0.1 rests on round 7's degree drops at m = 3 and 4, which do
  not occur at the incumbent's m = 2.
- Distance remaining to matched rho therefore stays at the order of 18x at w = 0 and at least five orders
  of magnitude at w = 0.1, with at best a factor near 3 to be gained from this lever at m = 2.

## 6. Deviations and what was not done

- msolve 0.10.1, not 0.6.5; quotient dimension computed from the reduced basis, not printed by msolve.
- Three bases were run (d' = 3, 9, 27). Only d' = 27 is the registered base. Cells d' = 3 and 9 were added to
  reach finished cells; 90 cells in all (3 bases, m = 2, 3, 4, 5 arms, 2 targets).
- 3 jobs in parallel on a 4-core shared host: wall times show ordering only.
- `.resume/RESUME.md` and the S3 checkpoint were not written. The task limited edits to this directory, and no
  ecbench or EKS experiment run was launched; the msolve cells are local development checks.
- The Conductor check and the ecdb lookup were not run (no tool in this session). The curve data (A = 926,
  r = 131203) come from the ledger family record and were re-checked by group-order tests in `self_check`.
- No graph or PDF (research-visuals); Stage 0 is a diagnostic and no row is proposed for the leaderboard or
  `benchmarks/`.
