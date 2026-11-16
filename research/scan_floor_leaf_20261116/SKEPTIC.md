# Skeptic review: scan-floor leaf pilot (leads B and C)

Independent review, 2026-11-16. Every number here is `[pcc:unverified]`, measured on the same
shared 4-vCPU container, single thread, one cell at a time. All computation is Rust
(`rustc -O`) and shell. Nothing in the repository was changed except this file. Scratch
directory: `/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-W2/`
(`patched.rs` = the producer's source plus my arms S and X, a 4-lane inversion switch and a dump;
`bf.rs` = my brute force).

## Summary

- **Reproduced: partly.** Counted operations reproduce exactly at n = 23 and n = 35 (the two cells
  I reran). Wall ratios reproduce at n = 35. n = 29, 41 and 47 were not rerun.
- **B fails**: confirmed for the cells rerun; the n = 47 decision value itself was not rerun.
- **C "inconclusive"** follows from the frozen rule, but the reading of C is weaker than stated:
  1. About a third of C's wall advantage is a **port artefact**: the port inverts with one serial
     Montgomery chain, the repository with four interleaved chains.
  2. Most of the rest is a **symmetry and shared-inversion effect, not a y-free effect**: an arm
     that keeps the stored `y` and uses the same ±P pairing gets nearly the same result.
  3. The "0.9 ms unexplained" gap is real, is shared by that y-using arm, and is not caused by the
     24-byte point records.

## 1. Port fidelity

How the original computes `t` (`examples/ic_algebraic_floor/linear21.rs`, `decide`, lines 445–479):
per block of 256 base points it calls `FastCurve::add_many(r, &negated[lo..hi])`, a full affine
batched addition that computes both `x(Q)` and `y(Q)` with one batched inversion of `x_R + x_i`.
It then runs a second `Gf2::batch_inv` on `x(Q)` to get `1/t`. No symmetry between `+P` and `-P`
is used, and nothing is batched across blocks or precomputed per target.

What matches the original, stage by stage:

| Stage | Original | Port | Match |
|---|---|---|---|
| Blocks | 256 leaves | 256 leaves | yes |
| Addition | affine, `x` and `y`, 2 mul + 1 sqr per leaf after the inversion | same | yes |
| Second inversion | batched over `x(Q)` | same | yes |
| System build (`pair_half_trace`) | `d + 2` mul, 2 sqr, one half-trace table map | same | yes |
| `solve_f2` | tracked elimination, 64-entry pivot array zeroed per leaf | same | yes |
| Root split, `verify` | half-trace root, `index_of`, affine `add` | same | yes |
| Kernel cap, skip rules (`q.infinity`, `q.x == 0`) | 16; skipped | same | yes |

What differs:

- **Batched inversion.** The repository's `Gf2::batch_inv` (`src/cryptanalysis/semaev_decomp.rs:657`)
  deals the slice to 4 independent accumulator chains kept in vector registers. The port uses one
  serial chain. The baseline's t stage runs two such inversions per leaf block, C runs one per
  pair block, so the serial chain penalises the baseline more. The producer lists this as
  deviation 6 without measuring it. I measured it (section 3): it inflates C's wall ratios.
- **Field reduction.** The port folds by shift-xor over the low terms; the repository folds with a
  second `pclmulqdq`. This applies to every arm alike.
- **Curve, field polynomial, targets, all-witness scan**: as the protocol states. On the "no"
  class the all-witness scan does the same work as the original.
- **Not a difference, but relevant to the claim "C is not already there"**: true for `linear21.rs`.
  The repository does have `FastCurve::add_many_lazy` (abscissa and slope only, no `y(Q)`) and an
  8-lane abscissa-only kernel. `lin21-ht` does not use them, so the port is faithful, but one of
  the three multiplications C saves per leaf (the `y(Q)` product) is existing repository
  machinery. C against that t stage was not measured by anyone.

Finding: the baseline's cost model matches the original in counted operations. It does not match
in wall time, because of the serial inversion chain.

## 2. The build error

Not reproduced by building (as instructed). By inspection it is a real type error on the current
tree, not a toolchain or feature artefact:

- `src/cryptanalysis/koblitz_index_calculus.rs:1634` declares
  `fn lift_x_packed(...) -> ([FastPoint; 2], usize)`.
- Lines 1644–1650, inside it, contain `return Vec::new();` (line 1646, the reported line) and
  `return vec![curve.lower(point), curve.lower(curve.neg(point))];`.
- The block has no `cfg` attribute and no feature gate, so every toolchain rejects it (`E0308`).
- `git diff 16aa66283b^1 16aa66283b` shows the block entering `lift_x_packed` in merge commit
  `16aa66283b` (2026-11-15, "Merge remote-tracking branch 'origin/main' into mt-work"): a
  cached-lift branch written for the older `Vec`-returning function was merged into the newer
  tuple-returning one.

The fallback to a standalone port was justified.

## 3. Rebuild and rerun

- `scan_leaf.rs` sha256 is `816d0831…c9196e`, as the README says. My binary's sha256 is
  `12af9a20…4671c`, not the README's `21056edc…04e102`; I did not investigate (output path or
  host differences are the likely cause).
- n = 23 (producer's command, 0.2 s): the four counted rows are identical to `tables.md`, digit
  for digit. All gates pass.
- n = 35 (5.5 s): the counted rows are identical to `tables.md`.

Wall at n = 35, unmodified source:

| Ratio | Producer | My rerun |
|---|---|---|
| A/A (A1/A2) | 1.012 [1.006, 1.018] | 1.008 [1.004, 1.013] |
| A1/B1 | 0.529 | 0.523 |
| A1/B2 | 1.093 | 1.071 |
| A1/C whole leaf | 1.200 [1.193, 1.208] | 1.189 [1.184, 1.195] |
| t stage L1a/C1 | 2.329 | 2.364 [2.332, 2.389] |

Wall at n = 35 with my extra arms (`patched.rs`; two runs for the serial build, ranges shown):

| Ratio | Serial inversion (as the port) | 4-lane inversion (as the repository) |
|---|---|---|
| A/A | 1.011–1.012 | 1.010–1.012 |
| t stage, base/C (L1a/C1) | 2.36–2.41 | 1.69 |
| t stage, base/S (L1a/S1) | 1.88–1.91 | 1.30 |
| whole leaf, base/C (A1/C) | 1.21 | 1.12–1.13 |
| whole leaf, base/S (A1/S) | 1.17–1.19 | 1.10–1.11 |
| whole leaf, S/C | 1.02–1.03 | 1.005–1.024 |
| whole leaf, base/X (A1/X) | 0.986 | 1.004 |
| baseline t stage, ns per target | 136,000 | 90,000 |

The 4-lane switch changes no counted operation. My 4-lane code is a plain scalar version, not
the repository's vector-register kernel, so the repository's own chain may be faster still.

## 4. Correctness and the symmetry question

Brute force (`bf.rs`, bit-serial arithmetic, no shared code, n = 23, all 150 targets):

- The base equals every rational point with `x` in `V`: 107 points, including the 2-torsion point
  `(0, 1)`.
- Truth is defined without restrictions: every index multiset `{i, j, k}` with
  `P_i + P_j + P_k = R`, repeated points, the 2-torsion point and pairs summing to infinity
  included, each triple re-verified by direct addition.
- The baseline's set and C's set equal that truth on all 150 targets: 0 false triples, 0 missing.
  Two reported triples have a repeated index.
- B1, B2 and my arms S and X were not dumped. Their equality with the baseline rests on the
  producer's gate code, which I read, and on the timed-build check (0 mismatches for all seven
  arms in my runs).

Convention, shared by all arms because all go through the same `leaf` and `verify`:

- The 2-torsion point is scanned as the first summand, but is rejected inside a pair
  (`x1 == 0 || x2 == 0`).
- A leaf with `Q` at infinity or `x(Q) = 0` is skipped.
- Repeated points are allowed.
- The only decompositions this can lose have `R` itself in the base. None occurred in the sample,
  so the exclusions were not exercised.

C's two roots: on 15,900 (target, base point) pairs, `x(R - P)` and `x(R + P)` both satisfy
`e^2 t^2 + P t + P^2 + b = 0`, are distinct, and sum to `P/e^2`. The producer's multiset gate
confirms C feeds the leaf the same `(t, 1/t)` multiset.

Does C scan half the base? No. Baseline and C both run all leaves (107 at n = 23, both signs); the
leaf counts are equal and nothing is double-counted. C gains by computing the two leaves of `±P`
together: one product `x·x_R`, one inversion of `e(P + sqrt b)` in place of two inversions, and
the Vieta relations for the second root and both reciprocals.

The baseline could use all of that with `y`. My arm S does: the same pairing and the same single
inversion, but `t1` from the stored `y` and the slope, `t2 = t1 + P/e^2`,
`1/t1 = t2·e^2/(P^2+b)`, `1/t2 = t1·e^2/(P^2+b)`, no half-trace. It costs 11 mul + 4 sqr per pair
against C's 10 mul + 3 sqr + one table map. S passes the set and multiset gates at n = 23 and 35.

| n = 35 | base / C | base / S |
|---|---|---|
| t stage, counted (hw) | 1.292 | 1.267 |
| whole leaf, counted (hw) | 1.045 | 1.042 |
| whole leaf, wall, serial inversion | 1.21 | 1.17–1.19 |
| whole leaf, wall, 4-lane inversion | 1.12–1.13 | 1.10–1.11 |

So C's result is mainly a **±P symmetry and shared-inversion effect that a y-using baseline can
equally have**. What is specific to being y-free is the difference S to C: 2% in counted t-stage
operations, 1.26–1.31 in isolated t-stage wall time, and 0.5–3% of the whole leaf.

## 5. Counter scheme, the wall gap, the decisions, tuning

**Counter scheme.** One `ct!` macro and one price vector serve every arm, and the counted and
timed builds are the same code under a const generic. I checked the baseline and C t-stage counts
against the source by hand (baseline 8 mul + 1 sqr per leaf; C 10 mul + 3 sqr + one table map per
pair): they are right. The scheme does not price dependent-chain latency or memory traffic. At the
hw price the Gauss word operations are 55% of the baseline leaf (508,199 of 918,835 at n = 35), so
whole-leaf ratios depend on the choice "1 word operation = 1/8 multiplication".

**Why whole-leaf wall is 1.15–1.20 when counted is 1.03–1.06.** Three parts at n = 35:

1. Serial inversion chain in the port: with 4 lanes the whole-leaf ratio falls from 1.21 to 1.12–1.13.
2. The t stage is cheaper in wall than in counted terms even with 4 lanes (1.69 against 1.29).
3. A residue not explained by the isolated t stage: A1 − C is about 122 µs per target while
   L1a − C1 is about 80 µs (4-lane: 70 µs against 37 µs). This is the producer's "0.9 ms at n = 47".

On the residue: the producer's suggestion (24-byte point records, leaf-loop layout) is
**refuted** by arm X, which keeps the baseline's t stage and uses C's array-fed leaf loop:
A1/X = 0.986 and 1.004. Arm S shows the same residue as C (about 40 µs), so it belongs to the
paired restructuring, not to y-freeness. I could not identify the cause (candidates: the smaller
per-block working set, or stage times measured in isolation not being additive). It remains
unexplained.

**Decisions under the frozen rules.**

- B1 fails (0.423 < 1.1 at n = 47) and B2 fails (1.062 < 1.1): these follow from the rule, given
  the n = 47 counts, which I did not rerun.
- C is inconclusive: the t-stage counted ratio 1.276 at n = 47 is between 1.1 and 1.3. That
  follows from the rule and does not depend on the port's inversion, since counts do not change.
- The Amdahl stop was applied as written.

**Protocol timing and tuning.**

- `PROTOCOL.md` was committed alone in `d2a43e435e` at 02:21:27 UTC and is byte-identical now.
- `scan_leaf.rs` has modification time 02:26; `breakdown.json` 02:27; `results.json` 02:29. All
  were untracked when I started and were first committed in `e0fdc0a272` at 02:31:51, during this
  review.
- So the source has exactly one version in Git. The statement that no source change followed the
  off-protocol debug run (whose ratio lines the producer saw) **cannot be verified**: the file was
  last written five minutes after the freeze and one minute before the first measured run.
- Nothing suggests tuning in the producer's favour: the thresholds and prices are in the frozen
  protocol, C lands just under its 1.3 threshold, and B fails.

## Problems

1. The baseline's wall time is inflated by the port's serial batched inversion. C's t-stage wall
   ratio of 2.3 becomes 1.69 with a repository-style 4-lane inversion, and whole-leaf 1.19–1.21
   becomes 1.12–1.13 (n = 35). The README's wall ratios for C should not be quoted without this.
2. "y-free t" misnames the effect. A y-using arm with the same ±P pairing and single inversion
   reaches 1.267 of C's 1.292 counted t-stage ratio and 1.10–1.19 of C's 1.12–1.21 whole-leaf wall
   ratio. The half-trace step itself is worth 0.5–3% of the leaf.
3. The baseline was not compared with the repository's existing abscissa-only batched addition
   (`add_many_lazy`), which already removes one of the three multiplications C saves.
4. The unexplained wall residue (about 6% of the leaf at n = 35) is not caused by the point-record
   layout, contrary to the README's suggestion; cause unknown.
5. "No source change after the debug run" is unverifiable from the history.
6. The exclusion rules (2-torsion inside a pair, `Q` at infinity) were never exercised by the 150
   targets at n = 23, so gate 2 does not test them.
7. My binary hash differs from the README's; counted output is identical.
8. Not checked by me: n = 29, 41, 47 reruns; B1/B2 sets against my brute force; the bootstrap code;
   the repository arm itself (it does not build). Treat the corresponding producer statements as
   unverified.
