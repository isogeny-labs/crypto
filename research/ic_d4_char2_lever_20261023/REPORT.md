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
of *counted* work reads the same at w = 0, 0.1 and 1 GAE: 0.8543 at m = 12 and 0.8154 at m = 14. That
is not a §5a robustness result: the eigen step's XOR accumulation and table lookups are real native work
that neither session emits, so the band prices them at 0 by omission. The ratio is a counted-unit
(F_q multiplication) result and the IC S values are lower bounds; whether the improvement survives
pricing the uncounted work is **unverified** until those counters are emitted (open implementation gap). Intervals are degenerate because counts are deterministic;
per-target ratios at m = 12 are 0.85430, 0.85430, 0.85432. m = 14 is one target: `within` says nothing
about other targets. The lever is expected to remove that work roughly in proportion, but that is not measured.

Against the prediction PROTOCOL.md froze: m = 12 (0.8543) falls inside its predicted interval
[0.85, 0.97]. m = 14 (0.8154) falls just *outside* its predicted [0.82, 0.95], on the favourable side.
The prediction is recorded as missed, not adjusted. Added 2026-10-28 after the independent validation
(`validation/VALIDATOR_REPORT.md`) pointed out the omission.

Per-trial components at m = 12 (300 random targets, `examples/d4_char2_profile`, F_q mults): root
finding 55,505 -> 22,977; total 224,196 -> 191,435; charpoly 87,728, echelon 54,809, normal forms
21,913 unchanged (asserted equal in the unit test). In-process wall per trial 4.595 ms -> 4.439 ms
(ratio 0.966; shared VM, one run, descriptive only).

### External implementation comparison (AGENTS.md §15): outstanding

The computation the lever speeds up is root finding of the eigen-step polynomial over F(2^m)
(m = 48, 56), inside the D₄ decomposition oracle. The reference tools for that are FLINT
(`fq_nmod_poly` / `fq_zech_poly` roots), NTL (`GF2EX` `FindRoots`), Magma and PARI/GP. None was run
on these inputs: no exporter of the per-trial polynomials exists yet and none of the tools is installed
on this host. So this study measures progress against our own previous kernel only (§10) and says
nothing about where the kernel stands against them; no "fastest" claim is made. Still missing: an
exporter of the per-trial polynomials, and an equal-output, same-host, single-thread timing against
FLINT and NTL (Magma if a licence is available).

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

After validation (`VALIDATION.json`, validated with caveats): leaderboard component section
(`docs/ic/LEADERBOARD.md`, outside tables A–C) and benchmarks entry
(`benchmarks/20261028-d4-char2-frob-roots/`). Graph:
`figures/s_lever.svg` (data `results/table-m*.tsv`, layout `figures/plot.py`); PDF: `REPORT.pdf`.
