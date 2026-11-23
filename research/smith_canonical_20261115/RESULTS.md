# Results: the canonical map psi: X -> C of Smith's trigonal construction

Study `smith_canonical_20261115`. Module `src/cryptanalysis/smith_canonical.rs`.
Protocol `PROTOCOL.md`, sha256 `519b465adfe5bf218189bc190a9b028646477ed4ce9bf55d401747c010e0b8ae`,
frozen 2026-11-15T02:31:04Z; first sweep cell started 02:31:11Z. Raw output:
`raw/cell_{A..E}.log`; aggregate: `summary.json`.

All statements below are `[pcc:unverified]` assertions of this session (no
pcc claim was filed, no independent validator has replayed the run). Toy
primes; no cost claim, no comparison with rho, no leaderboard row.

## What was built

`psi(t; b) = T(t) (b0 : b1 : b2)` with the symmetric matrix
`T_ik = [x^2]( x^(i+k) D adj(F mod G) mod G )` in `F_p[t]`. The three
coordinates are the traces along Smith's correspondence `R` of the
differentials `x^i dx / y` of `H` (derivation in the module doc). No
singularity analysis of the plane model and no Riemann-Roch computation is
used. `C` is the unique quartic through the images of the points of `X`
over `U`. `Psi` to `Pic^0(C)` uses `plane_quartic_pic` unchanged; the class
of a conjugate pair is built from the `F_p`-conics through the pair and the
base point.

This is not the route suggested in the brief (adjoints of
`Phi(t, rho) = 0`). The plane model is still supported as an input
(`psi_t_rho`), by lifting `(t, rho)` to `X`.

## Positive control: Smith's Example 1 (F_37), unit tests

| check | result |
|---|---|
| fitted quartic (mons_of_degree(4) order) | `[1,26,36,29,22,36,21,26,24,13,15,36,12,15,36]`, 52 distinct fit points |
| independent differentials, no conic, no cubic, one quartic | yes |
| smooth | yes |
| `#C(F_37^k)`, k = 1, 2, 3 | 42, 1342, 50070 |
| `#H(F_37^k)`, k = 1, 2, 3 | 42, 1342, 50070 |
| same counts from the printed zeta numerator | 42, 1342, 50070; `#J = 55666` |
| printed quartic, same three counts | 42, 1342, 50070 |
| projectively equivalent to the printed quartic over F_37 | yes: exactly one projectivity found by exhaustive search over frames of rational points, `[[32,3,15],[27,32,34],[9,1,9]]`; it maps all 42 rational points onto the printed curve |
| `psi` of the image points on `C` | 60/60 |
| `psi` from `(t, rho)` | 60/60; 58 at smooth points of the plane model, 2 at a node (two lifts, the right one among them) |
| `psi` at points with `t` in `F_37^2 \ F_37` | 11 plane points found by search, 11 mapped onto `C` |
| line sections through conjugate pairs give one class | 28 lines, 15 through conjugate pairs, all equal |
| principal divisors map to 0 | 8/8; perturbed: 8/8 nonzero |
| two representations of a class agree | 10/10 |
| `Psi(D1 + D2) = Psi(D1) + Psi(D2)` | 10/10 |
| `#J_H Psi(D) = 0` | 10/10 |
| `ord D / ord Psi(D)` | 1 (6 classes), 2 (4 classes) |
| rational 2-torsion killed iff in `S` | 1/1 (the one nonzero rational 2-torsion class is in `S`) |
| DLP transfer, `k = 22359` | `ord D = ord Psi(D) = 27833`; `Psi([k]D) = [k]Psi(D)`; BSGS on `C` returns 22359 |
| perturbed `T` (negative control) | fit fails |
| `count_quartic_points` vs brute-force enumerator | equal (q = 7: k = 1, 2, 3; q = 11: k = 1, 2) |

## Sweep over random Smith-positive curves (frozen protocol)

70 curves, none dropped. The canonical map was built on all 70; no curve
gave a conic (no hyperelliptic `X`), and no `FAIL` row.

| cell | p | deg F | curves | fit points (min-max) | dim 3, one quartic | smooth | `#C = #H` over | psi on C | (t, rho) input (at nodes) |
|---|---|---|---|---|---|---|---|---|---|
| A | 37 | 7 | 20 | 40-72 | 20/20 | 20/20 | k = 1,2,3: 20/20 | 1228/1228 | 1228/1228 (70) |
| B | 101 | 7 | 20 | 124-188 | 20/20 | 20/20 | k = 1,2,3: 20/20 | 3468/3468 | 3468/3468 (40) |
| C | 211 | 7 | 10 | 324-384 | 10/10 | 10/10 | k = 1,2: 10/10 | 4092/4092 | 4092/4092 (22) |
| D | 37 | 8 | 10 | 36-60 | 10/10 | 10/10 | k = 1,2,3: 10/10 | 608/608 | 608/608 (22) |
| E | 101 | 8 | 10 | 140-168 | 10/10 | 10/10 | k = 1,2,3: 10/10 | 1780/1780 | 1780/1780 (16) |

| cell | relations -> 0 | perturbed -> nonzero | curves with no relation run | repr | additivity | `#J_H`-annihilated | order ratio 1 / 2 / 4 / 8 | 2-torsion agree (in S / not in S) | DLP verified | k recovered exactly |
|---|---|---|---|---|---|---|---|---|---|---|
| A | 169/169 | 169/169 | 1 | 200/200 | 200/200 | 200/200 | 116 / 84 / 0 / 0 | 104/104 (42 / 62) | 20/20 | 16/20 |
| B | 200/200 | 200/200 | 0 | 200/200 | 200/200 | 200/200 | 72 / 128 / 0 / 0 | 108/108 (50 / 58) | 20/20 | 20/20 |
| C | 100/100 | 100/100 | 0 | 100/100 | 100/100 | 100/100 | 47 / 53 / 0 / 0 | 34/34 (20 / 14) | 10/10 | 10/10 |
| D | 29/29 | 29/29 | 7 | not run | not run | not run | not run | not run | not run | not run |
| E | 99/99 | 99/99 | 0 | not run | not run | not run | not run | not run | not run | not run |

Reading the columns:

* "DLP verified": `Psi([k]D) = [k]Psi(D)` with `[k]D` computed on `H` and
  moved to `C` independently of `D`, and the BSGS logarithm on `C` equal to
  `k` modulo `ord Psi(D)`. "k recovered exactly": the logarithm equals `k`
  itself. The four curves of cell A where it does not are those where
  `ord Psi(D) = ord D / 2`: the transfer loses the 2-part that lies in the
  kernel, as it must. Orders of `Psi(D)` ranged 832 to 32202 (A), 43176 to
  576044 (B), 138376 to 5032808 (C).
* "2-torsion": every nonzero rational 2-torsion class `(u, 0)` of `J_H`.
  112 classes in `S` all map to 0; 134 classes outside `S` all map to a
  nonzero class. Membership in `S` is decided from `N, D` by polynomial
  arithmetic, without `psi`.
* The order ratio is 1 or 2 on all 500 classes, never 4 or 8, as it must
  be for a kernel of exponent 2.

## Checks that did not run

* Cells D, E (deg F = 8): checks 7-9 are not implemented (no `J_H`
  arithmetic for the even-degree model). Stated in the protocol.
* Relations: on 1 curve of cell A (i = 19) and 7 curves of cell D no pair
  of split functions was found within the search budget (few usable points
  at p = 37; the 0/0 rows). Those curves have no relation evidence. For the
  cell A curve the class checks 7-9 ran and passed; the 7 cell D curves
  have no evidence for C2 at all; only C1 is supported there.
* Where fewer than 10 relation pairs are listed for a curve, the two
  functions drawn were identical and the pair was skipped.
* Cell C: `#C(F_p^3)` not counted (protocol).

## Deviations from the protocol

* None in the sweep: same binary for all five cells
  (sha256 `184217c0348c37bcc3fb11e2205c2f2dce8fecab125ab5df8e07606274bf3715`,
  source sha256 `7ec3a7193780ffa90242cd831e006f017ab3b4c2fd878012f58b932fb5f430cb`),
  seeds and counts as frozen, cells run in the order A, D, B, E, C.
* After the sweep the source changed in three places, none in the
  instrument: an import moved into the test module, the unit test for `t`
  in `F_p^2` now tries twelve values of `t` instead of one (the single value
  used first happened to have no `F_p^2` point over it), and the module doc
  gained two sections. The unit tests were rerun on the final source: 9
  passed, 1 ignored (the sweep).
* The identity `Psi([P-P']) + Psi([P'-P'']) = Psi([P-P''])` of the brief is
  formal under the definition; it is a unit test (50/50) and is not counted
  as evidence (stated in the protocol before the freeze).
* The brief's "kernel elements `[(W') - (W'')]`" were tested through every
  rational 2-torsion class rather than through `smith_prop24`'s pair list.

## Proven by code, and assumed

Checked by exact computation on every curve of the table:

* the three coordinate functions are linearly independent on `X`, the image
  lies on no conic or cubic and on exactly one quartic, which is smooth;
* that quartic has the point counts of `H` over the listed fields (so,
  with k = 1, 2, 3, the same zeta function);
* for deg F = 7: `Psi` is well defined on the tested classes (independent
  representations agree), additive, annihilated by `#J_H(F_p)`, kills the
  rational part of `S` and no other rational 2-torsion, and carries a
  planted logarithm.

Follows from those by a standard argument, not separately computed: `psi`
is birational onto `C` (a non-constant map from a genus-3 curve to a smooth
plane quartic not through a conic has degree 1), hence `C` is the canonical
model of `X`.

Assumed, not checked:

* Smith's theorem that `R` induces an isogeny `J_H -> J_X` with kernel `S`
  (used to derive `T`; only its consequences above are tested);
* that the fitted quartic equals the canonical image at the points of `X`
  not evaluated: the fibres over `t = infinity` and over the `t` outside
  `U`. The zeta match covers them in aggregate only;
* the kernel beyond rational 2-torsion: classes of `S` that are rational
  as a set but whose elements are not rational are not tested one by one;
* correctness of `plane_quartic_pic` (its own study) and of
  `smith_trigonal::image_points`;
* nothing is claimed for primes beyond 211.

## Not produced

No figure, PDF, benchmark record or leaderboard row: this is an exact
correctness study with no timing or cost measurement. No pcc claim filed.
