# Degree seven excluded on the twelve recorded trace +22 carriers

## Exact finite finding

The twelve monic odd-degree genus-five carrier models in the earlier finite
trace-`+22` census split into two six-member orbits under exact
`PGL₂(F_3)` coordinate substitutions:

| representative | carrier masks |
|---:|---|
| 32280 | 32280, 36380, 106121, 113379, 168265, 175771 |
| 35349 | 35349, 51009, 67186, 112490, 127268, 175537 |

The [orbit receipt](CARRIER_ORBITS.json) enumerates all 24 projective
matrices and retains a matrix from each representative to each of its six
models. [audit_second_carrier.py](audit_second_carrier.py) independently
recomputes all twelve displayed substitutions using pure-Python polynomial
arithmetic over `F_3`; its [audit receipt](SECOND_CARRIER_AUDIT.json) passes.
For a matrix `[a,b,c,d]`, the checked identity is
`(cx+d)^12 f_m((ax+b)/(cx+d))=f_n(x)`.

The [fixed-carrier theorem](DEGREE_SEVEN_FIXED_CARRIER_THEOREM.md) excludes
degree-seven maps to source masks 68 and 84 on carrier 32280 across every
normalized denominator class. The prior quadratic-square census excludes
that class on carrier 35349. The five newly frozen second-carrier arms
complete its other classes:

| second-carrier arm | mask 68 exact hits | mask 84 exact hits | CPU s, 68 / 84 |
|---|---:|---:|---:|
| sparse | 0 | 0 | 38.261 / 38.589 |
| repeated linear square | 0 | 0 | 1.863 / 1.852 |
| cubic square | 0 | 0 | 5.583 / 5.607 |
| two branch poles plus linear square | 0 | 0 | 28.080 / 28.095 |
| three branch poles | 0 | 0 | 21.379 / 20.889 |

Every new receipt binds to its committed source SHA256 and passes the
second-carrier structural audit. The cubic systems have zero censored cells;
the sparse/linear counters, 7,906,140 two-pole logical candidates, and
643,043,610 three-pole logical candidates match the frozen contracts per
source. All point-filter survivors were checked by exact polynomial square
testing. The original twelve-carrier quadratic-square receipts cover the
missing arm with zero hits. The first and second carrier audits together
cover 24 representative carrier/source/arm cells; the base-field coordinate
changes and the two five-element source Frobenius orbits transport the
conclusion to **120 exact carrier/source equation pairs**.

**Computer-assisted conclusion:** none of those 120 pairs admits a
degree-seven `F_(3^5)` morphism from its recorded genus-five carrier to its
recorded elliptic source. The normalization, denominator-parity partition,
and CRT completeness are written in the fixed-carrier theorem. The
coordinate substitutions have a separate pure-Python replay; the full
cover-elimination algorithms still have one Sage implementation and
structural audits, not independent algebra-system reimplementations.
[pcc:unverified]

## What the bound does and does not say

The twelve carriers are precisely the **recorded monic odd-degree models in
the finite census**, not every genus-five curve with the same Weil
polynomial. Howe's published theorem still guarantees a cover of degree at
most 26 for each recorded carrier and trace-`+22` source curve; the earlier
geometric floor is four. The resulting possible degree set is
`{4,5,6,8,…,26}` for these pairs. The exclusion of seven does not identify
which other degree occurs or provide a computable subgroup map.

The trace-`+22` source class has a 37-subgroup at `K` and an 80-bit prime
factor on the `s=11` base extension in the existing order receipt. Those
are construction and cost-screen facts. There is no explicit transfer into
the small-field Jacobian or full same-target GHS/rho measurement for this
class. The present result is a finite obstruction, **not a new vulnerable
family**.

Prior work checked: [Howe's elliptic-cover bound](https://arxiv.org/pdf/2110.04221),
the repository's primary-source audit of
[Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf), and the
earlier finite carrier/candidate receipts. The shared KB revision used was
`9141e386254340a9bdba3036f2bb706a` (14 November 2026), older than these
runs; search queries for degree-eight covers and exact trace-`+22` maps found
no matching construction, which does not establish global priority.
