# F4, F5 and an F6 analogue on prime-field curves: result

- **Protocol:** [`PROTOCOL.md`](PROTOCOL.md), committed before any measured
  run, and [`AMENDMENT_1.md`](AMENDMENT_1.md).
- **Analysis:** [`ANALYSIS.json`](ANALYSIS.json), written by the native
  analyser [`examples/pkm_tower_prime_analyze.rs`](../../examples/pkm_tower_prime_analyze.rs)
  from the records alone.
- **Run report:** [`reports/run-20261011T2244Z/`](reports/run-20261011T2244Z/REPORT.pdf).
- **Companion binary-curve round:**
  [`research/f4_f5_f6_cross_family_20261011/`](../f4_f5_f6_cross_family_20261011/RESULT.md).

## Answer in one paragraph

`ecbench` now has a prime-field algebraic decomposition oracle: Semaev's
`S₃` on a Petit–Kosters–Messeng Kummer tower, searched by a splitting tree
whose nodes a Gröbner engine decides. It was run on four prime-order curves
of 9–15 bits, with the tower growing with the curve (N = 10–16 unknowns).

| comparison | what was measured |
| --- | --- |
| signature F5 against tower F4 | visits the same tree in every cell; does 0.23–0.33 of F4's counted multiply-adds, the ratio rising with size |
| the same, in wall time | F5 goes from 1.8× faster (9-bit) to 2.1× slower (15-bit) |
| F6 analogue against F4 | saves nothing at any size |
| every arm against same-target rho | slower on every target |
| algebraic arms against `enum` | **lower** lower-bound `S`, but only because their multiply-adds are unpriced; priced at 1/63 GAE each they cost 670–900,000× `enum` |

On the F6 analogue: geometric closure prunes 27–47 % of the tree's nodes,
but almost all of the solver's work is in the shallow nodes it cannot reach.

On `enum`: it is the arm with no algebra at all, and once the
multiply-adds are priced it is far cheaper than every algebraic arm. In
relation-phase wall time the gap is 4,400–750,000×.

## What ran

| spec | session | curve | p | r | t (2^t abscissae; base points) | targets | runs verified | replay audit |
| --- | --- | --- | --: | --: | --- | --: | --: | --- |
| P5 | `ECBS1h744c8f794e9a` | `icv1-fp9-t25-33fa1ca2` | 257 | 233 | 5 (32; 32) | 8 | 40 / 40 | [ok](audits/P5-replay-all.json) |
| P6 | `ECBS1h91a14b12224a` | `icv1-fp11-t43-4d73e7f7` | 1,409 | 1,367 | 6 (64; 56) | 8 | 40 / 40 | [ok](audits/P6-replay-all.json) |
| P7 | `ECBS1h4e2538d423f5` | `icv1-fp13-tm41-756f569a` | 7,681 | 7,723 | 7 (128; 132) | 8 | 40 / 40 | [ok](audits/P7-replay-all.json) |
| P8 | `ECBS1ha6de54733ae8` | `icv1-fp15-t181-50ff72d2` | 18,433 | 18,253 | 8 (256; 254) | 4 | 20 / 20 | [ok](audits/P8-replay-all.json) |

- **Code and binary.** Every session records commit `faeeae26a`, a clean
  tree, and one `ecbench` binary (sha256 `64ed18283b4e8215…`).
- **Run window.** 2026-10-11 22:51 to 2026-10-12 00:37 UTC.
- **Host.** An x86-64 Linux cloud VM, isolation L1 or L2 as recorded per
  run. Counts and `S` are deterministic; wall time is a practicality note.
- **First attempt.** Its replay audit exposed a thread-count dependence in
  the tower F4 engine. It is kept voided in `attempt-1-voided/`
  ([Amendment 1](AMENDMENT_1.md)).

## Results

- **`W`:** counted `F_p` multiply-adds of the tower engines per one-target
  run.
- **`S`:** charged GAE over `√r`. It is a lower bound for the tower arms,
  whose multiply-adds are unpriced. Every arm's record also leaves native
  lookups and square roots unpriced; rho leaves its canonicalisations
  unpriced.
- **Statistics:** medians with ranges, over 8 targets (4 at P8).

### Cost per cell

| cell | rho-negation S | F4 S | sig-F5 S | F6 S | `enum` S | F4 W | sig-F5 W | F6 W | F4 nodes (F6 nodes) |
| --- | --: | --: | --: | --: | --: | --: | --: | --: | --: |
| P5 (9-bit) | 5.50 [4.13, 7.08] | 25.84 | 25.84 | 25.55 | 29.74 | 8.05 × 10⁷ | 1.82 × 10⁷ | 8.02 × 10⁷ | 61 (32.5) |
| P6 (11-bit) | 2.79 [2.30, 3.89] | 15.55 | 15.55 | 15.67 | 38.81 | 2.35 × 10⁹ | 6.51 × 10⁸ | 2.35 × 10⁹ | 145.5 (95) |
| P7 (13-bit) | 2.01 [1.37, 2.54] | 9.37 | 9.37 | 9.38 | 45.89 | 4.07 × 10¹⁰ | 1.25 × 10¹⁰ | 4.07 × 10¹⁰ | 339.5 (238) |
| P8 (15-bit) | 1.98 [1.24, 3.81] | 7.48 | 7.48 | 7.50 | 60.62 | 4.86 × 10¹¹ | 1.59 × 10¹¹ | 4.86 × 10¹¹ | 591.5 (432) |

**Relation-phase wall time** (medians):

| cell | F4 | sig-F5 | F6 | `enum` |
| --- | --: | --: | --: | --: |
| P5 | 0.36 s | 0.20 s | 0.35 s | 41 µs |
| P6 | 3.96 s | 3.62 s | 3.68 s | 139 µs |
| P7 | 34.9 s | 51.9 s | 35.1 s | 308 µs |
| P8 | 268 s | 552 s | 267 s | 716 µs |

### The protocol's questions, with its rules applied

**Q1. Signature F5 against F4.**

| cell | W(sig-F5)/W(F4), median [range] | same tree on every target | wall, sig/F4 | rule |
| --- | --- | --- | --: | --- |
| P5 | 0.227 [0.225, 0.231] | yes | 0.56 | **F5 cuts F4's work** |
| P6 | 0.277 [0.276, 0.280] | yes | 0.94 | **F5 cuts F4's work** |
| P7 | 0.307 [0.298, 0.310] | yes | 1.45 | **F5 cuts F4's work** |
| P8 | 0.326 [0.325, 0.327] | yes | 2.06 | **F5 cuts F4's work** |

- **"Same tree"** means equal nodes, refutations, leaves, trials and
  relations on every target.
- **The ratio rises with size:** 0.23 → 0.33 as N goes from 10 to 16.
  That is the direction the PKM tower study measured with the same engines
  on its own systems (§13: 0.28–0.37 at `m = 2`, and rising with `N`).
- **Wall time inverts the count by P7.** The signature engine spends its
  savings on bookkeeping that the multiply-add count does not see.

**Q2. F6 analogue against F4.** Statistic `log₂(W(F4)/W(F6))`.

| cell | median [range] | rule | F6 geometric additions | S(F6)/S(F4) |
| --- | --- | --- | --: | --: |
| P5 | 0.0059 [0.0054, 0.0073] | **no saving** | 47 | 1.011 |
| P6 | 0.0005 [−0.081, 0.0005] | **no saving** | 84 | 1.014 |
| P7 | 0.00007 [0.00006, 0.00008] | **no saving** | 154.5 | 0.999 |
| P8 | 0.00001 [0.00001, 0.00001] | **no saving** | 246.5 | 1.005 |

- **Closure prunes nodes but not work.** It removes 27–47 % of the nodes,
  but the nodes it closes are the deep ones: the root and first levels,
  where the full `S₃` system is solved, carry almost all of F4's work.
- **The one negative target at P6 (−0.081)** is target 2. F6's closure
  returned a different decomposition there, and its run made 43 trials
  (21 relations) against F4's 40 (19).
- **In the binary round F6-IC saved log₂ 0.29–0.37.** There the Boolean
  solver's work is spread through a deep tree; here it is concentrated at
  the top.

**Q3. Each arm against same-target rho.** Every arm is above rho on
every target of every cell: **slower than rho on every target**. Median
`S` ratios:

| cell | F4 (= sig-F5) / rho | F6 / rho | `enum` / rho |
| --- | --: | --: | --: |
| P5 | 4.64 | 4.71 | 5.42 |
| P6 | 5.39 | 5.43 | 12.00 |
| P7 | 4.77 | 4.75 | 21.71 |
| P8 | 3.91 | 3.92 | 29.00 |

The tower arms' ratio is a lower bound, because their multiply-adds are
unpriced.

**Q4. Algebra against geometry** (reported, no rule).

| cell | S(F4)/S(`enum`) | derived: priced S(F4)/S(`enum`) | derived: priced S(sig-F5)/S(`enum`) | relation wall F4/`enum` |
| --- | --: | --: | --: | --: |
| P5 | 0.854 | 2,949 | 670 | 8,041 |
| P6 | 0.414 | 26,741 | 7,392 | 25,845 |
| P7 | 0.224 | 156,145 | 47,819 | 98,722 |
| P8 | 0.119 | 899,485 | 293,124 | 360,005 |

- **Unpriced, the algebra looks cheaper.** F4's lower-bound `S` is below
  `enum`'s, and the gap widens with size, because `S` does not see the
  multiply-adds.
- **Priced, it is not.** At 1/63 GAE per multiply-add (the derived
  pricing, `RESEARCH_RESIDUAL_WALKS.md` §11) the order reverses by
  10³–10⁶, and wall time agrees in direction.
- **Why `enum` stays cheap.** Its `S` grows, but slowly: 29.7 → 60.6.

**Q5. Size (descriptive slopes of log₂ median `S` against log₂ r, four
cells).**

| arm | slope |
| --- | --: |
| rho-negation | −0.24 |
| F4 and sig-F5 (lower bound) | −0.29 |
| F6 (lower bound) | −0.28 |
| `enum` | +0.15 |

- **What the slopes mean.** In `S = cost/√r` a slope of 0 is rho's √r
  scaling. The two lower bounds fall with r only because the multiply-adds
  they leave out are growing fastest. F4's own multiply-adds per run grow
  about r^2.0 across the four cells, from 8.05 × 10⁷ to 4.86 × 10¹¹ while
  r grows about 78-fold.
- **No exponent is claimed** from four small sizes.

## Prime against binary, side by side

The binary figures are from the companion round. The units match (`S`,
lower bounds, same-target rho), but the curves, fields and solvers do not,
so read the two rows next to each other, not as one comparison.

| | Koblitz / generic binary (m = 13–19) | prime (9–15 bits) |
| --- | --- | --- |
| F5 against F4 | matrix F5 0.64–0.65 of matrix F4, same tree | signature F5 0.23–0.33 of tower F4, same tree |
| F6 against its F4 | F6-IC saves log₂ 0.29–0.37 (1.2–1.3×) | F6 analogue saves nothing (log₂ ≤ 0.006) |
| best algebraic arm / rho | inherited F4 2.3–7.2× | tower F4 3.9–5.4× |
| any arm below rho? | no | no |

## What this does not show

- **No speedup in `S`.** The tower arms' `S` is a lower bound. The priced
  figures are derived.
- **No exponent.** Four sizes up to 15 bits, and P8 has 4 targets.
- **Only `m = 2`, Kummer towers and one tree.** The oracle does two-point
  decompositions on Kummer towers only. Dickson and isogeny towers and
  `m ≥ 3` (the chain with partial-sum unknowns) are not built. A
  different split (both blocks, or a degree-driven F5 such as §14's
  variant) was not tried.
- **Nothing about cryptographic sizes.** These are toy curves chosen so
  that `2^t | p − 1`.
