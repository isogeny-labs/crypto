# Rank-5 special-curve representation result

Date: 2026-11-01

Verdict: **reject at representation-size scope**

Protocol: [`RANK5_REPRESENTATION_PROTOCOL.md`](RANK5_REPRESENTATION_PROTOCOL.md)

## Result

The frozen 24-cell panel completed under Rust 1.78 after all 26 native tests
passed.  Every cell was constructed twice from the same typed canonical IR;
all IR hashes and exact circuit statistics matched.  No timing was collected.

Neither `j=0` nor `j=1728` crossed the precommitted `-0.05` slope-difference
gate for Boolean variables, ordinary-CNF clauses, or total CNF-XOR
constraints in both modulus families.  The largest absolute fitted-slope
difference was only `0.0006031943606744083`.

| special class | modulus family | variable slope difference | CNF difference | CNF-XOR difference |
| --- | --- | ---: | ---: | ---: |
| `j=0` | Solinas | 0.0005998812 | 0.0006031944 | 0.0006024580 |
| `j=0` | generic | 0.0001471667 | 0.0001488330 | 0.0001513503 |
| `j=1728` | Solinas | -0.0000580554 | -0.0000571546 | -0.0000577937 |
| `j=1728` | generic | -0.0000035127 | -0.0000026722 | -0.0000032396 |

Positive values are worse than the matched generic curve.  The small negative
`j=1728` values are roughly three orders of magnitude below the advance gate.

## Exact primary counts

The committed JSON contains every registered IR, gate, assertion, byte, hash,
CNF, and CNF-XOR statistic.  The three decision metrics are reproduced here.

| bits | modulus family | curve | Boolean variables | ordinary-CNF clauses | CNF-XOR constraints |
| ---: | --- | --- | ---: | ---: | ---: |
| 20 | Solinas | `j=0` | 424085 | 1439669 | 934271 |
| 20 | Solinas | `j=1728` | 443840 | 1507085 | 977234 |
| 20 | Solinas | generic | 452137 | 1535245 | 995587 |
| 20 | generic | `j=0` | 447805 | 1521241 | 984607 |
| 20 | generic | `j=1728` | 465360 | 1581177 | 1022730 |
| 20 | generic | generic | 474313 | 1611593 | 1042475 |
| 24 | Solinas | `j=0` | 620933 | 2108461 | 1367671 |
| 24 | Solinas | `j=1728` | 648292 | 2201857 | 1427110 |
| 24 | Solinas | generic | 660553 | 2243475 | 1454223 |
| 24 | generic | `j=0` | 644069 | 2188569 | 1415679 |
| 24 | generic | `j=1728` | 668788 | 2272989 | 1469310 |
| 24 | generic | generic | 681689 | 2316823 | 1497751 |
| 26 | Solinas | `j=0` | 742733 | 2522329 | 1635811 |
| 26 | Solinas | `j=1728` | 773830 | 2628507 | 1703328 |
| 26 | Solinas | generic | 788621 | 2678716 | 1736029 |
| 26 | generic | `j=0` | 749057 | 2545873 | 1645639 |
| 26 | generic | `j=1728` | 777814 | 2644095 | 1708008 |
| 26 | generic | generic | 792779 | 2694952 | 1740979 |
| 28 | Solinas | `j=0` | 844989 | 2869749 | 1860951 |
| 28 | Solinas | `j=1728` | 881728 | 2995189 | 1940722 |
| 28 | Solinas | generic | 898441 | 3051921 | 1977675 |
| 28 | generic | `j=0` | 870433 | 2958297 | 1912851 |
| 28 | generic | `j=1728` | 903812 | 3072313 | 1985230 |
| 28 | generic | generic | 921229 | 3131495 | 2023619 |

Across the four sizes, the special/generic ratios are stable rather than
shrinking: `j=0` ranges from `0.9377` to `0.9453`, while `j=1728` ranges from
`0.9810` to `0.9817`, over all three metrics and both modulus families.  This
is a constant representation shift, not the registered scaling effect.

## Evidence identity and boundary

- Machine result: [`rank5-representation-panel.json`](rank5-representation-panel.json)
- Result SHA-256: `faee7d4bf6dece47e8b3d0694966010f08cd7c9364f29e3dfdef34a42b10a925`
- Result bytes: `31177`
- Cells: `24/24`; repeated-construction matches: `24/24`
- Native test suite: `26 passed, 0 failed`

This rejects only the hypothesis that the selected special curves or Solinas
syntax improve deterministic representation-size growth in this exact affine
shared-coefficient encoding.  It does not disprove specialized propagation,
arithmetic-instruction, automorphism, subgroup, relation-yield, or alternate-
chart effects.  Those broader routes remain blocked on a common production
solver/backend and matched subgroup fixtures rather than negatively decided.
