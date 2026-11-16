# Frozen remainder of the degree-seven denominator partition

16 November 2026. This continuation completes the remaining denominator shapes for the same seven new trace-minus14 source/carrier representative cells under `PROTOCOL.md`. The E11/f9735 cell has an earlier complete negative degree-seven screen. The two quadratic-square shapes and four sparse branch-pole shapes have separate frozen protocols and complete receipts.

| Source | SHA256 | Shapes |
|---|---|---|
| `linear_square.sage` | `33b74175dd81e7aa285a53d48e3773c18acdd2ad83473c07c283b34fe6030add` | `H²`, `H⁴`, `H⁶`, `QiH²`, `QiH⁴` with linear H |
| `cubic_square.sage` | `2baeaaf16a6e38ba665fea287e2084d32f87a12ecada7f977254d82835ba942e` | `H²` with monic cubic H |
| `two_poles_linear.sage` | `1e97198f2d8cdf908eabf30390f2f3c09614514acc99d87840c475b6045b3cbe` | `QiQj H²` with linear H |
| `three_poles.sage` | `f87aec9025c16ac94ab97ca00ac1e0721c6bfab82657a10debdec27fd12efb10` | `QiQjQk` |

The partition is by parity of finite poles in `B`, with `deg B` even and at most six. The carrier has one rational branch factor L and five quadratic branch factors Qi. The rational branch point cannot be a pole of odd multiplicity in an even-degree B. The scripts reconstruct the numerator from the rational two-torsion residue at L and all three residues at each unpoled Qi. Every surviving polynomial pair is checked against `f C²=B(A³+A²B+bB³)`, degree seven and coprimality. Point evaluations and bitsets are necessary-condition filters with exact full-identity checks after filtering; points are chosen outside f=0 for each carrier. This uses the same algorithms as the prior fixed-pair degree-seven screen, adapted to the two carrier representatives and four source representatives. The cubic system must report no censored cases. Each script is one algebra-system implementation and needs independent replay before a broad theorem claim.

For each name in the table and each of `(11,17371),(202,9735),(202,17371),(38,9735),(38,17371),(80,9735),(80,17371)`, run `sage <name>.sage <bmask> <fmask> > <name>_<bmask>_<fmask>.json 2> <name>_<bmask>_<fmask>.stderr`. Retain all raw receipts and source hashes. Any hit is a candidate map requiring independent identity, subgroup transfer, and full attack-cost verification.
