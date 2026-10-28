# `frob_roots`: -14.6 % (m = 12) and -18.5 % (m = 14) counted S in the char-2 D4 index calculus

Protocol: [`PROTOCOL.md`](PROTOCOL.md), committed (d2eec8df3) before any session.
Sessions: `results/lever-m12` and `results/lever-m14` (L0, `--cpus none --allow-busy`,
`RAYON_NUM_THREADS=1`). **Class: engineering.** Counted-unit result only; no wall-clock claim.

## What changed

`src/cryptanalysis/gaudry_quartic_bin.rs`: `SqMod` and `roots_with(.., frob)`. The `X^q - X mod p`
and Cantor-Zassenhaus squarings use `(sum a_i X^i)^2 = sum a_i^2 X^(2i)` with a per-modulus table of
`X^(d+j) mod p`, instead of a generic product plus reduction. Oracle parameter `frob_roots=1`
(`BinOptions.frob_roots`, default off). Identical polynomial, roots, rational solutions,
decompositions, witnesses and trial sequence.

The other two candidates were not available (PROTOCOL.md): the first-dependency stop is already what
`incremental-gauss` does here (linear algebra is 0.1 % of S); no exact cheap rejection test exists
because the solve cost does not depend on the outcome.

## Result (paired, same instances, targets and seeds; every run `verified`)

| curve | arm | S per target | trials (identical across arms) |
|:--|:--|:--|:--|
| m = 12 | ic-base | 44.03, 42.59, 41.47 | 186,121; 180,033; 175,246 |
| m = 12 | ic-lever | 37.62, 36.39, 35.43 | same |
| m = 12 | ic-base-aa | 44.03, 42.59, 41.47 | same |
| m = 14 | ic-base | 5.3701 | 749,350 |
| m = 14 | ic-lever | 4.3786 | 749,350 |
| m = 14 | ic-base-aa | 5.3701 | 749,350 |

`ic-base` reproduces the 2026-10-15 sessions exactly (44.0338 at m = 12 target 0, 5.3701 at m = 14).

| paired S ratio lever / base | value | 95 % interval (`ecbench compare`) | A/A (base-aa / base) |
|:--|:--|:--|:--|
| m = 12, 3 targets | 0.8543 | [0.8543, 0.8543] (cluster) | 1.0000 [1.000, 1.000] |
| m = 14, 1 target | 0.8154 | [0.815, 0.815] (within; one target) | 1.0000 |

IC arms emit no `*_uncharged` counter (`unpriced: []`, `lower_bound: false`), so the lever/base ratio
is the same at w = 0, 0.1 and 1 GAE per unpriced native operation: 0.8543 at all three weights at
m = 12 and 0.8154 at all three at m = 14. Intervals are degenerate because counts are deterministic;
per-target ratios at m = 12 are 0.85430, 0.85430, 0.85432. m = 14 is one target: `within` says nothing
about other targets. Caveat: non-multiplication native work in the eigen step (XOR accumulation, table
lookups) is not counted for either IC arm; the lever removes it roughly in proportion, but that is not
measured.

Per-trial components at m = 12 (300 random targets, `examples/d4_char2_profile`, F_q mults): root
finding 55,505 -> 22,977; total 224,196 -> 191,435; charpoly 87,728, echelon 54,809, normal forms
21,913 unchanged (asserted equal in the unit test). In-process wall per trial 4.595 ms -> 4.439 ms
(ratio 0.966; shared VM, one run, descriptive only).

### Is this relabelling or accounting?

No. Trials, relations, rank and every other component are identical; no work moved to another phase.
The removed multiplications are executed table multiplications of a generic product (about 64^2 per
squaring) replaced by 64 squarings, and the counter is unchanged. Limit: counted S falls 14.6 % at
m = 12 but oracle wall time only about 3 % in the one in-process comparison, because the removed
multiplications are cheap lookups next to the dense linear-algebra loops. This is a counted-unit
improvement; a wall-clock improvement is not established (needs L2).

### Distance to the matched rho (`rho-neg`, same targets, same session)

Rho carries `canonicalisations_uncharged` (about one per step, 4,945,607 at m = 12 target 0), priced at
w GAE each on rho. IC / rho ratios, base -> lever:

| curve | w = 0 | w = 0.1 | w = 1 |
|:--|:--|:--|:--|
| m = 12 t0 | 47.2 -> 40.4 | 42.9 -> 36.7 | 23.6 -> 20.2 |
| m = 12 t1 | 50.3 -> 43.0 | 45.7 -> 39.1 | 25.1 -> 21.5 |
| m = 12 t2 | 39.9 -> 34.1 | 36.3 -> 31.0 | 19.9 -> 17.0 |
| m = 14 t0 | 4.27 -> 3.48 | 3.88 -> 3.16 | 2.13 -> 1.74 |

Rho is ahead at every weight (order holds across the band). Against rho's theoretical cost the m = 14
ratio is 4.94, down from 6.06. Distance remaining, not a beat of rho.

## Controls

- A/A: `ic-base-aa` equals `ic-base` to the last digit on every target.
- Positive control: the profile's independent `roots` component falls 59 % (55,505 -> 22,977).
- Tests (`cargo test --release --lib gaudry_quartic_bin`, 7 pass): roots equal the generic path for
  degrees 2-64, m = 3-12; solutions and decompositions equal over 12 solves.

## Replay audit

`ecbench verify --replay-all` (binary sha256 13818f3e..., the binary that measured both sessions; auditor on the same host class, so this is a same-host replay certificate, not an independent-host one):

| session | records | verified | replays reproduced | problems | audit receipt SHA-256 |
|:--|--:|--:|--:|--:|:--|
| `results/lever-m12` | 12 | 12 | 12 | 0 | 781a3d8500e6093982f7a2a9da5057d255402ea96a4972f33186f5151967034b |
| `results/lever-m14` | 4 | 4 | 4 | 0 | 3d63173bdd224b9c85e8e02bce8d98fe00625b17f6896ebea151a62b5346689d |

Receipts: `results/lever-m12.audit.json`, `results/lever-m14.audit.json`.

## Next

1. Charpoly (42 %): 65 dense determinants per trial; structure-exploiting determinant or
   Krylov/Berlekamp-Massey with an exact fallback.
2. Echelon + normal forms (37 %): share elimination across trials through the t_R dependence.
3. Raise the 0.5 % per-trial success (second coset, larger Artin-Schreier class).
4. An L2 wall session to see whether the counted gain survives.

Not touched: leaderboard, `benchmarks/README.md` (no VALIDATION.json), no PR. Graph:
`figures/s_lever.svg` (data `results/table-m*.tsv`, layout `figures/plot.py`); PDF: `REPORT.pdf`.
