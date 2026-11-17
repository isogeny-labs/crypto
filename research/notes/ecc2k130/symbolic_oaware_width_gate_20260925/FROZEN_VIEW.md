# Frozen source view: why CI replays this study against historical source

Added 2026-11-11. This file is not part of the freeze; `FROZEN.json`,
the evidence and every frozen file are unchanged.

## What drifted

`FROZEN.json` (SHA-256 `44f713e9…5d4c`, committed at `7e9f3e3131`) pins
24 files. On `main` at `80b026f5ab` the study files still match. Eight
source files do not, and `audit.py --preflight` fails on the first one it
meets ("frozen hash drift: src/binary_ecc/curve.rs"). Each of the eight
hashes to its pin at the freeze's `parent_commit`
`8b1e2d3734c1f30c14ce253b33094f894ab5005d` (checked with `git show`):

| File | Diff since `parent_commit` (`git diff --stat`) | Main changes after the freeze |
|:--|:--|:--|
| `src/binary_ecc/curve.rs` | +640 −1 | 2026-10-16 (#2028), 2026-10-27 (#2687, #2250) |
| `src/binary_ecc/f2m.rs` | +340 −33 | 2026-10-17 (#1798), 2026-10-26 (#2539), 2026-10-31 (#3078) |
| `src/cryptanalysis/pq_descent.rs` | +8 −4 | 2026-10-26 (#2335) |
| `src/cryptanalysis/pq_descent_symbolic.rs` | +373 −39 | 2026-10-11 (#1640), 2026-10-26 (#2335, #2375) |
| `src/cryptanalysis/pq_groebner_f2.rs` | +764 −60 | 2026-10-01 (#1167), 2026-10-26 (#2335, #2375) |
| `src/cryptanalysis/sat.rs` | +2709 −265 | 2026-10-27 (#2705, #2696), 2026-10-28 (#2756) |
| `src/cryptanalysis/semaev_decomp.rs` | +2737 −147 | 2026-10-11 (#1601), 2026-10-12 (#1752), 2026-10-24 (#2411) |
| `src/cryptanalysis/semaev_sat.rs` | +335 −16 | 2026-10-12 (#1701), 2026-10-26 (#2335, #2375) |

The frozen-pin index note named only `curve.rs`, because the preflight
stops at the first drift. All eight drift.

## What the study's checks do with these files

- `audit.py --preflight` (both workflow steps) hashes every file named in
  `FROZEN.json` and compares it with the pin. For the eight source files
  this checks only that they are the frozen source. Nothing is built or run
  from them.
- `ci_replay.py` reruns `width_decision`, which reads four of the eight
  (`pq_descent_symbolic.rs`, `pq_groebner_f2.rs`, `semaev_sat.rs`,
  `semaev_decomp.rs`) and asserts literal strings from them, such as
  `pub mask: u64` and `assert!(irr.degree <= 63, …)`. It then requires the
  committed producer receipt's `width` block to equal the recomputed one.
  This is a replay of the frozen width decision against the frozen code.
  The receipt names `source_commit` = `parent_commit`.
- `curve.rs`, `f2m.rs`, `pq_descent.rs` and `sat.rs` are only hashed.
  The n13 toy map in `audit.py` and `verify.py` is pure Python and reads
  none of them.

## Decision

A historical input view is legitimate here. Every check that touches these
files either verifies the identity of the frozen source or replays the
frozen decision against it. None of them makes a claim about current code.
`.github/scripts/symbolic-oaware-width-gate-source-view-v1.json` restores
the eight files from `parent_commit` in an isolated copy, after checking
each digest against this study's `FROZEN.json` (by JSON pointer into
`files`) and that manifest's SHA-256. The live checkout is not changed.

## What the replay does not say

The `NOT_ADMITTED` width result in [RESULT.md](RESULT.md) describes the
interfaces at `parent_commit`. It is not a statement about current `main`,
and the replay does not re-check it there. Current `main` has moved:
running `width_decision` on current `main` fails its first interface
assertion, because `pq_groebner_f2.rs` now stores monomials in a `Mask`
type that is `u128` under the `wide-vars` feature (#2335). Whether n131
fits the current interfaces was not re-assessed here. It needs a new
study, not this replay.
