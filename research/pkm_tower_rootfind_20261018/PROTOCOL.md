# Root finders against F4, F5 and the F6 analogue on prime-field curves: protocol

Status: **preregistered**. This file and the four specs were committed
before any measured run. The smoke run is disclosed below.

## Question

How do the repository's root-finding decomposers compare with the
Gröbner-based ones (tower F4, signature F5, and the F6 analogue) on the
same prime-field curves, the same factor base and the same public targets?

The decomposers come from two places:

- the root finders landed on `main` in PRs #1765, #1771 and #1889;
- the Gröbner engines are the `pkm-tower` oracle of
  `research/pkm_tower_ecbench_20261011/`.

The comparison is **against F4 and F5**. Same-target rho runs as context
only, and no decision reads it.

## Arms (all IC arms on one base)

**Base.** `prime-kummer:t=T,g=G`, the Kummer base on `main`. `G` is the
coset representative the 2026-10-11 round drew from `seed=1`:

| cell | T | G |
| --- | --: | --: |
| P5 | 5 | 37 |
| P6 | 6 | 684 |
| P7 | 7 | 7047 |
| P8 | 8 | 2551 |

`ecbench fb` gives the same `points_sha256` for `prime-kummer:t=T,g=G`
and that round's `prime-tower:t=T,seed=1` in all four cells. So the base
is the earlier round's, point for point.

**Gröbner and geometry arms** (the earlier round's oracle, now taking
`g`):

- **`f4`** (baseline): `pkm-tower:engine=f4`, tower F4 deciding a
  splitting tree.
- **`sig-f5`**: the same tree decided by signature F4 with the F5/GVW
  criteria.
- **`f6`**: F4 with nodes of at most 4 candidates closed by charged
  `R ∓ P₁` lookups.
- **`enum`**: closure at the root, with no algebra.

**Root finders** (on `main`, unmodified):

- **`rootfind-subtract`** (`m=2`): `prime_fast`'s direct subtraction.
  - Semaev's S₃ root in the last variable is `x(R ∓ P_i)`.
  - The `x_R − x_i` are inverted in one batch per target.
  - Each candidate is looked up in the base.
- **`rootfind-mitm`** (`m=2`): `R` is probed in a negation-folded pair
  table built once per run.
- **Context arms (`m=3`, a different relation shape):**
  - `rootfind-mitm:m=3`;
  - `rootfind-pairs:m=3` (no table).

**Reference.** `rho.negation`, as context.

**Workloads and run settings.**

- **Curves:** the four curves of the 2026-10-11 round:
  - `icv1-fp9-t25-33fa1ca2`
  - `icv1-fp11-t43-4d73e7f7`
  - `icv1-fp13-tm41-756f569a`
  - `icv1-fp15-t181-50ff72d2`
- **Targets:** that round's target seed, 202610112301, so the targets are
  the same ones. 8 targets per cell, 4 at P8.
- **Measurement:** seed 202610112302, one round, a 3,600 s cap, and
  isolation L1 requested.

## Units

- **`S`.** Charged group-addition equivalents over `√r`.
  - For the root finders `S` counts every group operation. Their records
    still say "lower bound", because lookups and their one batched
    inversion per target are uncharged.
  - For `f4`, `sig-f5` and `f6`, `S` also leaves the solver's `F_p`
    multiply-adds unpriced.
- **`M`, `F_p` multiplications per run.** This is derived from recorded
  counts with the library's own fixed costs, `PrimeCurve::FIELD_OPS_PER_ADD`
  and `FIELD_OPS_PER_DOUBLE`:
  - an affine addition is 2 multiplications, 1 squaring and 1 inversion;
  - a doubling is 5, 2 and 1;
  - so `M` = 3 × additions + 7 × doublings over every phase, plus the
    solver's multiply-adds (`solver.ops`) for the tower arms.

  Inversions are reported beside `M`, as additions + doublings (plus the
  root finder's native batched inversions), and are not converted.
- **Wall time.** Relation-phase wall time, a practicality note.

## Questions and decision rules

Per cell, over targets, with medians and ranges. "Every target" means
every target both arms verified.

1. **Root finder against F4 and against signature F5, in `S`.**
   - Statistic: `S(root)/S(F4)` and `S(root)/S(sig-F5)`, for
     `root ∈ {rootfind-subtract, rootfind-mitm}`.
   - Below 1 on every target: **"root finder proven cheaper than F4
     (F5)"**. The root finder's `S` charges all its additions, and the
     Gröbner arm's `S` is a lower bound on its own cost.
   - Otherwise: **"not decided by S"**.
2. **Root finder against F4 and against signature F5, in `F_p`
   multiplications.**
   - Statistic: `M(root)/M(F4)` and `M(root)/M(sig-F5)`.
   - Below 1 on every target, with the root finder's inversions also at
     most the Gröbner arm's: **"root finder cheaper in field
     arithmetic"**.
   - Above 1 on every target: **"F4 (F5) cheaper in field arithmetic"**.
   - Otherwise: **"mixed"**.
3. **Root finder against the F6 analogue and `enum`.** The same two
   statistics are reported, with no rule.
4. **Instrument check.** On every target, `f4`, `sig-f5` and `f6` must
   reproduce the 2026-10-11 round's counts: solver multiply-adds, nodes,
   trials, relations and `S`. The base is the same point for point, and
   the tree does not depend on the order in which `V` is listed.
   - `enum`'s order over `V` comes from `ζ`. It differs from the earlier
     round's, so its counts may differ, and it is not checked.
   - A mismatch voids the comparison until it is explained.
5. **Context.** `S/rho` for every arm. The `m = 3` root finders are
   reported beside the others, outside every rule, since their relations
   have three summands.

**Stop conditions.**

- A run that fails verification fails its cell.
- A cell with more than half its targets timed out gets no median.

## Smoke run (before this file; plumbing only)

`prime_search` 11 / 20, `prime-kummer:t=6,g=684`, one target (target seed
1). All seven arms verified.

| arm | S | relation-phase wall |
| --- | --: | --: |
| F4 | 16.42 (2.54 × 10⁹ multiply-adds) | 6.4 s |
| signature F5 | 16.42 (0.69 × 10⁹ multiply-adds) | 6.0 s |
| `rootfind-subtract` | 37.32 | 0.13 ms |
| `rootfind-mitm` | 35.78 | 0.12 ms |
| `rootfind-mitm:m=3` | 70.35 | 0.23 ms |
| `rootfind-pairs:m=3` | 91.36 | 0.48 ms |
| rho | 4.25 | — |

F4 and signature F5 matched the 2026-10-11 smoke run's counts on this
target.

## Commands

```sh
cargo build --release --bin ecbench
for s in P5 P6 P7 P8; do
  target/release/ecbench run --spec research/pkm_tower_rootfind_20261018/SPEC-$s.json \
    --out research/pkm_tower_rootfind_20261018/sessions/$s --wait
  target/release/ecbench verify --dir research/pkm_tower_rootfind_20261018/sessions/$s \
    --replay-all --out research/pkm_tower_rootfind_20261018/audits/$s-replay-all.json
done
```
