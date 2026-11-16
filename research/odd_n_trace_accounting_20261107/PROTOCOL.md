# PROTOCOL: odd-n trace accounting of twisted-line relation yields

Frozen 2026-11-07, before any study data. The instrument (`src/`) was built
and its `selftest` mode run before freezing; the selftest uses its own seed
tags (`selftest-*`), draws no study curve and counts no study target. The
freeze commit and the sha256 of every source file are in `FREEZE.sha256`.

Parent: [`subgroup_trap_20261107`](../subgroup_trap_20261107/README.md)
(PR #3646), which derived, without measuring, that for odd n the
twisted/standard yield ratio on 2E targets is 2^{n+1}, not 2^n, and called
the extra 2 accounting.

## Question

For odd n, is the extra factor 2 on 2E targets
- (a) available to an ordinary (not lift-dense) index calculus of the same
  shape, so that crediting it to the twisted line is accounting, or
- (b) a per-trial advantage that only all-lift (twisted) lines have, or
- (c) absent (the derivation is wrong)?

## Setup and notation

E: y² + xy = x³ + a x² + b over F_{2^N}, N = d·n, q = 2^d. #E counts O.
For P ≠ O: P ∈ 2E iff Tr_N(x_P) = Tr_N(a) (checked against [#E/2]P = O on
every target, P8). A line is u·F_q^* and its factor base FB(u) is every
point with x ∈ u·F_q^*, both signs of y. Since
Tr_N(u t) = Tr_d(t·Tr_{N/d}(u)) for t ∈ F_q, a line is **trace-homogeneous**
(all its abscissae have Tr_N(x) = 0) iff Tr_{N/d}(u) = 0. The plain F_q
(u = 1) has Tr_{N/d}(1) = n mod 2: trace-homogeneous for even n, trace-mixed
for odd n.

N(R) is the number of ordered n-tuples of FB points with sum R. m = |FB|,
m0 = #FB ∩ 2E, m1 = m − m0, c = number of distinct abscissae (= m/2, the
number of linear-algebra columns).

**Exact coset identity (algebra, stated before data).** A sum of n FB points
lies in 2E iff it uses an even number of points off 2E. So, exactly,

  Σ_{R∈2E} N(R) = ((m0+m1)^n + (m0−m1)^n)/2,
  Σ_{R∉2E} N(R) = ((m0+m1)^n − (m0−m1)^n)/2,

and since |2E| = #E/2 the mean of N over 2E is ((m0+m1)^n + (m0−m1)^n)/#E.
This is the "composition-exact prediction" used below. For a single-coset
base it is 2 m^n/#E; for a balanced mixed base about m^n/#E. Over bases of
a fixed size m, the 2E mean is largest, 2 m^n/#E, exactly when m0·m1 = 0.
The empirical content of the study is (i) that the 2E criterion and counts
behave as derived, (ii) that sampled and odd-order (cryptographic) targets
see the coset mean, and (iii) which matched-size bases attain it.

Size-normalised density: κ = mean N · #E / m^n (κ = 1 uniform over E, 2 for
a single-coset base on its coset). Relations per trial per column:
η = mean N / c; trials to collect c relations ∝ 1/η.

## Cells

| n | d | N | targets |
|:-:|:-:|:-:|:--|
| 3 | 3, 4, 5 | 9, 12, 15 | census |
| 5 | 3 | 15 | census |
| 5 | 4, 5 | 20, 25 | sampled |
| 4 | 3 | 12 | census (even control) |
| 4 | 4 | 16 | sampled (even control) |

Census (N ≤ 15): every point of E is a target. Sampled (N > 15): T = 2000
per class. Field moduli: the fixed low-weight polynomials in `gf.rs`, checked
irreducible at start-up.

## Curves (per cell)

Master seed `0x0DD7ACE202611107`; every stream is splitmix64 seeded with the
master seed xor FNV-1a(tag). Four curves per cell:
- B1: a = 0; B2, B3: a ≠ 0 with Tr_N(a) = 0; NEG: Tr_N(a) = 1.
- b uniform with [F_2(b):F_2] = N, so F_2(u, a, b) = F_{2^N} for every line
  (no subfield trap, #3646 Amendment 1).
- Redraw (tag suffix `-attempt`) until three lines exist, found by uniform
  search over u ∉ F_q and u ∉ b^{1/4}·F_q (no T-stable line, 2026-11-06 rule):
  - `tw`: Tr_{N/d}(u) = 0 and Tr_{N/d}(√b/u) = 0 (all-lift when Tr(a) = 0);
  - `hz`: Tr_{N/d}(u) = 0 and Tr_{N/d}(√b/u) ≠ 0 (trace-homogeneous,
    about half the abscissae lift: an ordinary line of the standard size);
  - `mix`: Tr_{N/d}(u) ≠ 0 and Tr_{N/d}(√b/u) ≠ 0 (a generic trace-mixed
    line, A/A comparator for `std` at odd n).

## Arms

| arm | base | expected size |
|:--|:--|:--|
| `std` | x ∈ F_q^* | ≈ q |
| `tz` | `std` points with Tr_N(x) = Tr(a) (its 2E part) | ≈ q/2 (odd n) |
| `tw` | x ∈ u_tw·F_q^* | 2(q−1) (Tr(a) = 0) |
| `twh` | `tw` with x = u_tw·t, Tr_d(t) = 0: matched-size sub-base of `tw` | ≈ q |
| `hz` | x ∈ u_hz·F_q^* | ≈ q |
| `mix` | x ∈ u_mix·F_q^* | ≈ q |

`twh` is the "sub-base of tw matched in size to std" of the brief. `hz` is
the fair comparator: an ordinary (half-lifting) F_q-line of the standard
size with the same descended-system shape as `std` and `tw` (x = u·t, t ∈
F_q), differing only in Tr_{N/d}(u).

## Targets and classes

Three classes per curve: `2E` (Tr(x_R) = Tr(a)), `off` (the other coset),
`odd` (the odd-order subgroup, the cryptographic targets: R = [2^v]X with
#E = 2^v·h, h odd). Census: all points of each class. Sampled: uniform
affine points (x uniform, sign bit; non-lifting x redrawn) sorted into `2E`
and `off` until each has T; `odd` from a separate stream, R = [2^v]X with X
uniform. Every target is checked: trace criterion against [#E/2]R = O, and
[h]R = O for `odd`.

## Counting

N(R) by meet in the middle: c2 = ordered pair sums; n = 3: Σ_{P∈FB}
c2(R − P); n = 4: Σ_S c2(S)·c2(R − S); n = 5: Σ_S c3(S)·c2(R − S), with c3
the ordered triple sums (equal to the brief's sum over FB³). Brute force
(nested loops over all ordered n-tuples, no tables) is run for every arm
with m^n ≤ 3·10^7 and must agree on every target; this includes every arm
of the smallest cell (n = 3, d = 3).

## Statistics

Means per (curve, class, arm). Intervals: 99 % percentile intervals from a
paired bootstrap over targets (B = 2000; the same resampled indices for all
arms of a class; classes resampled independently; seed tag
`boot-n{n}-d{d}-{id}`). In census cells the bootstrap measures how
representative the targets of a class are; the coset identities are
additionally checked as exact integers.

A **unit** is one Tr(a) = 0 curve in one cell (odd n: 18 units; even n: 6).

**Pass rule (R)** for a statistic over a set of U units: the
composition-exact prediction lies inside the unit's 99 % interval in all but
at most max(1, ⌊U/10⌋) units, and the pooled geometric mean over units of
obs/pred lies in [0.9, 1.1].

## Predictions (all from the measured m0, m1, c, #E)

- **P1 (2E, tw/std).** mean_2E(tw)/mean_2E(std) =
  [(s+_tw)^n + (s−_tw)^n]/[(s+_std)^n + (s−_std)^n] (s± = m0 ± m1), which is
  ≈ 2(m_tw/m_std)^n for odd n and (m_tw/m_std)^n for even n. Rule R, odd and
  even groups separately. Discriminating part: D = κ_tw/κ_std on 2E; odd n:
  rule R and pooled D in [1.7, 2.3] with the interval's lower end > 1.25 in
  all but max(1, ⌊U/10⌋) units; even n: rule R and pooled D in [0.85, 1.18].
- **P2 (all targets).** (mean_2E + mean_off) of tw over that of std =
  (m_tw/m_std)^n (both cosets have #E/2 points). Rule R, all units.
- **P3 (odd n, coset split).** Every off-2E target has count 0 for `tz`,
  `tw`, `twh` and `hz` (exact; one nonzero fails). `std` mean_2E/mean_off =
  [(s+)^n + (s−)^n]/[(s+)^n − (s−)^n] ≈ 1, κ_2E(tz) = κ_2E(tw) = 2: rule R.
- **P4 (n = 4 control).** Every `std` point is in 2E and every off-2E `std`
  count is 0 (exact); κ_2E(std) = 2 and κ_2E(mix) = composition value: rule
  R; and the even group of P1 passes (tw/std = (m_tw/m_std)^4).
- **P5 (odd-order targets).** mean_odd(tw)/mean_odd(std) equals the P1
  prediction. Rule R, all units.
- **P6 (odd n, the comparator question).** On 2E and on odd-order targets:
  H = κ_hz/κ_std = composition value (≈ 2); L = κ_tw/κ_hz = 1;
  T = η_tz/η_std = composition value (≈ 2^{2−n}: 0.5 at n = 3, 0.125 at
  n = 5); and on 2E κ_mix/κ_std = composition value (≈ 1). Rule R for each.
- **P7 (Tr(a) = 1 controls).** `tw` has m = 0; `tz` has count 0 off 2E;
  every `hz` point is off 2E, and for odd n `hz` has count 0 on every 2E and
  odd-order target, for n = 4 count 0 off 2E. Exact.
- **P8 (instrument).** 0 mismatches between the trace criterion and
  [#E/2]R = O; 0 failures of [h]R = O on the odd class; 0 brute/MITM
  mismatches with the smallest cell fully checked; every census identity
  (sum over 2E, off 2E and E) exact; on Tr(a) = 0 curves `tw` has exactly
  2(q−1) points and `tw`, `twh`, `hz`, `tz` lie in 2E.
- Reported without a pass rule: η_tw/η_std, η_tw/η_hz, η_hz/η_std and
  mean_tw/mean_hz on odd-order targets; κ_2E(twh); per-arm means, κ and the
  fraction of zero-count targets.

## Interpretation rule (frozen)

Index calculus needs about c relations, and a trial on target R returns N(R)
of them, so the figure of merit is η = mean N / c on the targets actually
used (odd-order, i.e. inside 2E); at equal base size it is carried by κ.
Pooled over odd-n units and both the 2E and odd classes:

- **Outcome A (trace-homogeneity, adoptable by the baseline).** pooled H in
  [1.6, 2.5], H's interval lower end > 1 in all but max(1, ⌊U/10⌋) of those
  unit-classes, and pooled L in [0.85, 1.18]. Reading: the extra 2 is a
  property of trace-homogeneous bases. An ordinary index calculus gets it at
  the same base size and the same system shape by replacing F_q with a
  trace-homogeneous line (`hz`). Against the plain F_q base it is a real
  per-trial gain (η doubles on cryptographic targets), but it belongs to the
  baseline, not to the twisted line; crediting 2^{n+1} to the twisted line
  is accounting (wrong comparator). The twisted line's own gain is its
  lift density, (m_tw/m_hz)^n ≈ 2^n in yield per trial.
- **Outcome B (all-lift only).** pooled H ≤ 1.25 while pooled D (odd n, 2E)
  ≥ 1.6: the factor needs an all-lift line; it is a twisted-line advantage.
- **Outcome C (no factor).** pooled D (odd n, 2E) ≤ 1.25: the derivation
  is refuted.
- Otherwise: inconclusive, reported as measured.

Independent of the outcome, T decides whether *sub-selecting* F_q to its
trace-homogeneous half recovers the factor: predicted no (η falls by
≈ 2^{2−n}), so the trace-zero comparator must be a full line, not `tz`.

Not measured: the per-trial solver cost of `hz` versus `std` (both are
lines u·F_q, so the descended system has the same shape; C1 H-C2 found
tw/std per-solve cost 0.96 for the twisted line). No ECDLP cost, leaderboard,
benchmark or rho row is produced.

## Execution route

Exact counting at toy sizes, run locally on the session host. The AGENTS.md
§9a isolab-eks confirmatory run is open, as for the parent studies. No
cloud job is launched.

## Procedure

```sh
cargo build --release
./target/release/odd_n_trace selftest
R=reports/run-<stamp>
for c in "3 3" "3 4" "3 5" "5 3" "4 3" "4 4" "5 4" "5 5"; do
  ./target/release/odd_n_trace run $c $R
done
./target/release/odd_n_trace score $R
```

The run directory records the commit, host and sha256 of every output.
