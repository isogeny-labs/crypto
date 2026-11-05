# Torsion levers for the Curve25519-family algebraic factor base (2026-11-03)

**Class:** exact-count census and engine measurements on the algebraic track.
These are component results, not a leaderboard row. Protocol:
[PROTOCOL.md](PROTOCOL.md), frozen in `012d945af9`; Addendum A was frozen in
`8d94c0e349` before its runs. Lead list: [LEADS.md](LEADS.md). Parent:
`research/curve25519_frame_search_20261103` (#3359).

## Question

The owner asked for the next goals, and for more leads on torsion points. The
parent study showed that, on the algebraic track, coverage depends on the
cofactor cosets of the base (E(F_p) ≅ Z/8 × Z/r). Three torsion levers
followed from that:

- T1, torsion-shifted targets;
- T2, a 2-descent-filtered base;
- T3, frame and coset selection at m = 3.

## Results

### Census (exact; five family-rule miniatures, k = 16–24; 2,000 frames per cell)

| | prediction | outcome |
|:--|:--|:--|
| Q1 | T1: joint (frame, coset) selection ≥ 1.10 × frame-only in ≥ 12/20 cells | **failed: 1/20** (gains 1.00–1.12). Frame selection already puts the coverage on coset 0. |
| Q2 | T1's log-free joint selector reaches ≥ 0.9 × the oracle | held (1.00 in every cell) |
| Q3a | T2: u(P) is a square ⟺ P ∈ 2E(F_p), so every base point is in an even coset | **held: 0 odd-coset points** over 20 cells × 2,000 frames |
| Q3b | T2 coverage per column² is 1.4–2.1 × plain (1.71 predicted) | **held at 1.89–2.02**. Cosets 0 and 4 cancel under all four signs and cosets 2 and 6 under two, which the 1.71 count left out. |
| Q4 | T3: m = 3 frame-selection gain > m = 2 gain | **held, 13/15** (m = 3: 2.8–5.4× at t ≤ 5) |

Not preregistered: the oracle-best T2 frame is 1.6–2.2× the best plain frame,
so frame selection stacks on T2 in coverage.

### Engine (k = 20, `curve25519_family f4cost`, 100 targets)

Binary sha256: `engine_pilot/binary.sha256`.

**Pilot (exploratory), lo = 0.** T2 raises the F4 cost per target 2.1–5.1×:
- solving degree 4 → 6 (t = 4, G = 3);
- solving degree 6 → 9 and terms 256 → 961 (t = 5, G = 0).

**Addendum A (frozen):** frame-selected `bits` against T2 at its
**oracle-best** frame, which is favourable to T2. G = 3.

| t | arm | L | cov | max degree | terms | mul-adds/target | F4 mul-adds / relation (derived) |
|--:|:--|--:|--:|--:|--:|--:|--:|
| 4 | bits@L | 797820 | 78 | 4 | 22 | 88,840 | **1.49·10⁸** |
| 4 | square-bits@L | 984071 | 128 | 6 | 32 | 186,900 | 1.92·10⁸ (1.28×) |
| 5 | bits@L | 233902 | 184 | 5 | 64 | 3,036,912 | **2.17·10⁹** |
| 5 | square-bits@L | 414271 | 350 | 7 | 124 | 9,888,000 | 3.71·10⁹ (1.71×) |

**E1 held: T2 is closed in its bit-encoded F4 form.** Its 2× coverage is paid
for with a higher solving degree. The same run adds a data point for the
parent's lever: at t = 4, the selected plain frame (1.49·10⁸) is **6.5×**
cheaper per relation than `bits lo=0` (9.71·10⁸, parent study).

## Reading

- Restricting the base's cosets **algebraically** (u = w², and likewise the
  T₀ quotient in `curve25519_t0_algebraic_20261102`) raises the polynomial
  degree, and on these sizes that costs more than the coverage gained.
- **Choosing** among frames leaves the system unchanged. It is free in F4
  cost, and the selection needs only coset labels ([r]P), not discrete logs.
- So the open torsion leads are selections on the other memberships, not
  algebraic restrictions:
  - the PKM isogeny tower's `seed` (the best algebraic arm, T4);
  - the `t0-bits` and Edwards `lo`;
  - m = 3, where selection pays more (Q4).

## Costs and scope

- The census is exact. The engine figures are counted F4 mul-adds; "per
  relation" is derived (mul-adds/target ÷ `coverage_exact`), and no relation
  was returned in these cells.
- Frame search is priced as in the parent study: 2000·2^t Legendre symbols
  plus [r]P per column.
- **Distance remaining** is unchanged by these results. The best algebraic arm
  against rho-neg is 18× (w = 0) and 9.6·10⁵× (w = 0.1)
  (`curve25519_algebraic_ecbench_20261102`). The generic baseline is 7.8× rho
  at k = 20. Rho remains ahead on every track.

## Reproduce

```bash
scripts/run.sh && python3 scripts/analyse.py          # census, ~1 min
cargo build --release --bin curve25519_family
B=target/release/curve25519_family
$B f4cost 20 square-bits@414271 5 3 100 2026111505 900 2 full 0 30000000   # one engine cell
```

## Deviations

- Conductor and the AWS CLI are absent: there was no Conductor task, no S3
  resume push and no EKS route. Everything ran locally as exact counts or
  counted mul-adds.
- The engine pilot was exploratory. Addendum A was frozen after it and before
  its own runs.
