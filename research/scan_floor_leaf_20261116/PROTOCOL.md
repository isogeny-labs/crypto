# Protocol: scan-floor leaf leads B ("dual leaf") and C ("y-free t")

Frozen 2026-11-16, before any measured run. Local pilot on a shared 4-vCPU
container: wall times are indicative, no benchmark row follows from it.
Source of the leads: `research/sat_leads_round4_20261116/PROPOSALS.md`, R4-B and R4-C.

## Baseline and the deviation that is already known

The repository's own arm `lin21-ht` (`examples/ic_algebraic_floor/linear21.rs`)
could not be built: the one permitted `cargo build --release --example
ic_algebraic_floor` stopped with `error[E0308]` at
`src/cryptanalysis/koblitz_index_calculus.rs:1646` (library does not compile at
HEAD 5f0dbd38f1). Per the task rule the baseline is therefore a **standalone
port** (`scan_leaf.rs`, `rustc -O`, own F_2^n arithmetic on `u64` words with
`pclmulqdq`, as the repository's `Gf2` uses). Baseline and candidates share that
arithmetic, the same binary and the same targets. Known differences of the port
from the repository arm, fixed now:

- curve `y^2 + xy = x^3 + x^2 + 1` (a = 1, b = 1) over `F_2[z]/(f)`, `f` the
  irreducible `z^n + low` with the numerically smallest `low`; factor base = every
  rational point with `x` in `V = span(1..z^(l-1))`, both signs, as in the repository;
- targets are `[2]P` for seeded random rational `P` (not prime-subgroup multiples of
  a generator: no group order is computed in the port);
- every arm scans all leaves and returns **all** decompositions (the repository arm
  stops at the first witness). On a non-decomposable target the work is the same.
- leaf logic (`pair_half_trace`, `split_root_and_verify`, `verify`, `solve_f2`, blocks
  of 256 leaves, batched addition then batched inversion of `x(Q)`) is ported line by line.

## Cells

n = 23, 29, 35, 41, 47 with l = (n-2)/3 = 7, 9, 11, 13, 15. Seed 20261116.
Per cell: the first 128 seeded random targets the baseline finds non-decomposable
("no" class), every decomposable target met while drawing them ("natural yes"), and
16 planted targets `P_i + P_j + P_k` with seeded random base indices.

## Arms (all in one binary)

- `base`: port of lin21-ht. Timed twice per target (`A1`, `A2`) for the A/A noise floor.
- `B1` (primary for B, the proposal as written): per leaf, rows `T(z^j / t)`,
  `j < 2l-1`, `T(a) = (Tr(G(m_i) a))_i` over a precomputed basis `m_i` of `V^perp`
  (byte tables), eliminated without combination tracking; the leaf is rejected
  unless `tau = (Tr(m_i sqrt(b)/t))_i` lies in their span (equivalent to
  `Tr(m sqrt(b)/t) = 0` for every left-kernel `m`). Survivors run the full baseline leaf.
- `B2` (secondary): the left-kernel test on the baseline's own half-trace system:
  same columns, elimination without combination tracking for the verdict, the
  tracked solve only on survivors.
- `C`: for each base pair `±P1` with abscissa `x`, `e = x + xR`, `P = x xR`,
  `D = e (P + sqrt b)`, one batched inversion of `D`, `a = e + sqrt(b)(1/x + 1/xR)`
  (table of `sqrt(b)/x` over the base, no multiplication), `w = H(a) + a`,
  `k = P/e^2`, `k' = P/(P^2+b)`; leaves `(t, 1/t) = (k w, k'(w+1))` and
  `(k(w+1), k' w)`. No y, no group addition; `Q` is recovered by a group addition
  only for a leaf whose split lands in `V`. Degenerate abscissae
  (`x = 0`, `x = xR`, `D = 0`, a one-point abscissa) use the baseline path.
- stage arms for the breakdown: `L1` (t stage only, timed twice: `L1a`, `L1b`),
  `L2` (+ system build), `L3` (+ elimination), `base` (= all), and `C1` (C's t stage only).

First check for C ("already present?"): done by reading `linear21.rs` before this
protocol. It is **not** present: `decide` lifts nothing but uses stored `y`, calls
`FastCurve::add_many` (full affine addition, `y(Q)` computed) and then a second batched
inversion for `1/x(Q)`. So C is run.

## Correctness gates (any failure voids the arm's ratios)

1. Every arm returns the same set of decompositions (sorted index triples) as
   `base` on every target of every cell.
2. n = 23, 29, 35: that set equals exhaustive pair enumeration (hash of all
   `P_j + P_k`, looked up at every `R - P_i`, with the baseline's skip rules).
3. B1 and B2 per-leaf verdict = baseline "system consistent" flag on every leaf
   of every target. C: the multiset of `t` (and of `1/t`) equals the baseline's
   multiset of `x(R - P_i)` on every target.
4. Every planted target is found decomposable with its planted triple in the set.

## Metrics

Primary: counted operations per target, mean over the "no" class, from a counting
build of the same code (`const` generic switch, results asserted equal to the
timed build). One scheme for all arms, counting what the code executes:
`mul` (field multiplications), `sqr`, `inv` (field inversions; a batched inversion
is 3(k-1) `mul` + 1 `inv`), `lookup` (table loads: byte tables of linear maps,
per-abscissa tables, index map), `wop` (word XOR/AND/OR/shift/clz/compare outside
field operations; Gauss: 2 per pivot probe, +1 per reduction, +1 more when the
combination is tracked, +1 per pivot insert). Group additions are counted by
their field operations. Categories are reported separately and as one total under
three prices fixed here:

| price | mul | sqr | inv | lookup | wop |
|---|---|---|---|---|---|
| **hw (primary)**: the binary's kernel, one CLMUL + shift-xor fold | 8 | 8 | 8(n+8) | 1 | 1 |
| clmul-min | 3 | 3 | 3(n+8) | 1 | 1 |
| bit-serial software multiplier | 2n | n | 2n(n+8) | 1 | 1 |

Decisions use **hw**; the other two are sensitivity rows and decide nothing.

Secondary: wall ns per target, single thread, timed build. Per target every arm
is run in an order rotated by target index, 3 repetitions, per-target time = the
minimum of the 3. Cell ratio = sum over the "no" class of baseline `A1` time /
sum of the arm's time; 95% interval by paired bootstrap over targets (2,000
resamples, seeded, in Rust). A/A band = the same statistic for `A1/A2` (and
`L1a/L1b` for the t stage). "Outside the band" = the arm's 95% lower bound is
above the A/A 95% upper bound.

## Stage breakdown and the Amdahl stop

Before any B or C arm is run, a `breakdown` run of `base, L1, L2, L3` reports the
baseline per stage: t stage, half-trace elimination (system build + solve),
root split, membership/verify, in hw-priced counted operations and in wall
differences (`L1`, `L3-L1`, `base-L3`). If the stage a proposal acts on
(t stage for C; half-trace elimination for B) is under 10% of the hw-priced leaf
at n = 47, the proposal is recorded as "cannot matter" and its arms are not run.

## Decision rules

B (applied to B1; B2 reported with the same rule, labelled secondary):
- **holds**: hw-priced counted operations per target of `base` / arm >= 1.5 at
  n = 35, 41, 47, all gates pass, and the wall ratio is outside the A/A band at those n;
- **fails**: that counted ratio < 1.1 at n = 47;
- otherwise **inconclusive**.

C: the same with threshold 1.3 on the **t stage's** hw-priced counted operations
(`base` stage T / `C` stage T; stage T = everything up to having `t` and `1/t` per
leaf) and the t-stage wall ratio `L1a/C1` against the `L1a/L1b` band; fails if the
t-stage counted ratio < 1.1 at n = 47. The whole-leaf counted and wall ratios
(`A1/C`) are stated next to it and decide nothing.

No tuning after a measured row is seen. A change to code after the first measured
run is listed under deviations with the reason, and all cells are rerun.
Result lines are tagged `[pcc:unverified]`; ratios are stated with the baseline
named ("standalone port of lin21-ht"), never as claims about the repository arm.
