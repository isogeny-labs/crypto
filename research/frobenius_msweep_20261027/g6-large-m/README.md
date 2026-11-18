# Cell G6: Frobenius and 2-torsion structure at m = 9..20 (prepared, not run)

**Status: prepared, not run.** No experiment, sweep, benchmark or data-producing binary ran. Only `cargo build` and
`cargo test` (32 tests on tiny fixtures, all passing) were executed. The S3 resume-checkpoint gate blocks runs in this container
and was not bypassed.

Question (revision 2): does any Frobenius or 2-torsion structure change the relation yield of an ordered-slot factor base at toy n,
against the same instrument without the structure, at matched slot-size profile? The cost half and the attribution to m were dropped after
review: NI and UP have identical operation counts by construction, and at fixed toy n the slot size moves with m (slot-size control cells added).

Instrument: an exact-enumeration census of the ordered-slot (rotated-partition) design on toy curves (n = 23, 19; Koblitz a = 0, 1). For a
slot system it computes the number of ordered decompositions H[R] of every group element at once (meet in the middle, all pairs), then
compares a Frobenius-interleaved normal-basis design (NI) with a generic design whose slots have exactly the same sizes (UP), paired
draw by draw, with positive control (duplicate slot), A/A (UP2), invariants and a frozen equivalence rule. It also audits the
2-torsion translation exactly and tabulates the chained encoding's sizes as exact functions of m.

What the scout and the design work found before any data (all in `PROTOCOL.md`):

* **The NI yield in the class-(0,1) cells is derived, not measured.** There the NI slots are {T, +-sigma^i P}, a chain of consecutive Frobenius
  images, and tau^2 -+ tau + 2 = 0 forces structural collisions: 9711 distinct sums of 177147 tuples (n = 23, m = 12), 2335 of 19683 (m = 14), 543 of
  2187 (n = 19, m = 12), checked against the group law and the real NI builder. NI/UP yield efficiency is x1/4..x1/18 and dispersion x6..x31 as an
  algebraic fact, a toy-slot-size artefact. These instances are a `CHAIN_DERIVED` stratum, cross-checked at run time, never tested; the earlier
  prior "EQUIVALENT within 10 percent" is withdrawn.

* The "l about n/m = 1..3 at toy n" premise holds only for the ordered-slot design; the symmetric design needs l about 4.2..4.6.
* On-curve status is Frobenius invariant, so full interleaved slots are Frobenius rotations of each other and their sizes jump by
  whole bits together. By hand arithmetic the production partition cannot reach the target size window at toy n for m >= 16 (n = 23)
  and m >= 14 (n = 19): those 9 cells are expected `UNMATCHABLE` (size census only; a toy-n artefact, not a statement about the production
  partition). Only 13 of the 22 original cells are enumerable; informative statistics exist at m <= 14.
* iota (x -> sqrt(b)/x) is translation by T (proved, tested). For prime n only a dimension-1 subspace is iota-stable, so the 2^(m-1)
  stabiliser needs non-subspace slots; the exact identity H_iota[R] = 2^(k-1)(H_plain[R] + H_plain[R+T]) says an iota-stable base is the plain base
  queried at R and R + T, at 2^k times the enumeration. Relabelling unless an iota-aware enumerator exists.
* The scout's "16384 patterns at m = 18" is 2^16 = 65536 (16384 is m = 16).

Cells: 26 (22 original + 4 slot-size control at n = 13, 17). Cannot reach: the effect of m as such, any solver cost, n = 131, orbit-union bases at m >= 9 (oversized at toy n), Frobenius-stable slots, the x = 0-excluded variant,
the G-stratum, m = 13/15/17/19, the n = 83 production gate. This cell cannot show an improvement over the index-calculus baseline and
reports no distance to rho.

| file | content |
|---|---|
| [`PROTOCOL.md`](PROTOCOL.md) | design (revision 2, not yet frozen), claim classes, arms, controls, decision rule, pricing, resolution, runtime estimate |
| [`RUN_CARD.md`](RUN_CARD.md) | route, preconditions, commands block by block |
| [`run_cells.sh`](run_cells.sh) | single entry point (stamp check, tests, build, pre-flight, iota audit, cells, analyze, archive) |
| `src/`, `tests/` | std-only crate `g6-large-m`, binary `g6census` (`cells`, `sizes`, `cell`, `iota-audit`, `analyze`); `src/digits.rs` is the exact Z[tau] chain model |
| [`plot_g6.py`](plot_g6.py) | plots recorded values of a run only |
| [`DESIGN.pdf`](DESIGN.pdf) (source `DESIGN.typ`, figure from `plot_design.py`) | two-page design note; its figure is hand arithmetic (PROTOCOL 4.3), not data |

Next: obtain the checkpoint token and an Iso Lab route (RUN_CARD sections 0 and 1), freeze the protocol, run, then write the report.
