# Reduced S3: a checked generator-preprocessing lead

2026-10-25 · PR2381 · [protocol](../../REDUCED_S3_20261025_PROTOCOL.md) ·
[PDF](report.pdf) · [graph](degrees.svg) · [graph source](plot.py) ·
[native cases/audit/certificate](../../native/reduced-s3-20261025/) ·
[Lean](../../lean-reduced/ReducedS3.lean) ·
[validation and failures](../../validation/reduced-s3-20261025/README.md).

The new executed lead is **reduce the actual S3 polynomial modulo factor-base
membership before linearization**. C++ and independent Rust agree on all98
size2 input cases,188 main/twist base comparisons and1,384 target comparisons.
There are69 target comparisons with both a larger useful-rank increment and
lower finite closure degree, and zero with both a larger increment and lower
homogeneous cutoff. All cases/targets, including losses and transported controls,
are retained. This is exact finite algebra, not a timing or speedup result.

## Exact F43 example and degree changes

Use the existing registered models `icv1-fp6-t0-74779950` and
`icv1-fp6-t0-c6a8a47c`, order11 subgroups, main generator(31,18), twist
generator(24,26), and bases(4,5),(13,19) and(15,9),(17,6). The twist generator
has no new EC1 registration; the older generator's alias is not reused.

| First public relation target | Main original | Main reduced | Twist original | Twist reduced |
| --- | ---: | ---: | ---: | ---: |
| Useful independent rows |0|0|2|2|
| Ordered polynomial root pairs |0|0|3|3|
| Finite vanishing-space closure degree |6|4|4|2|
| First degree deriving a unit |5|3|none|none|
| Leading-form first-empty degree |3|2|3|2|

The new main remainder is41xy+39(x+y)+27, and the twist remainder is
17xy+3(x+y)+41=17(x-15)(y-15), all over F43. The twist's degree2 matrix
has rank3 in6 columns with3 independent root evaluations. Main degree2
has rank3 in6 columns but zero roots; a unit first occurs at3 and full
vanishing-space closure at4. The different properties are not conflated.

The exact input leading ideal after reduction is
`<xy,x^2,y^2>` in both fixture arms, with Hilbert vector `[1,2,0,...]`.
Thus preprocessing lowers this explicitly defined cutoff **on both curves**.
It does not provide a twist-only lower cutoff in these comparisons. Since
the underlying affine ideal is unchanged, this generator-sensitive quantity
must not be called the intrinsic regularity of the same affine ideal or its
saturated homogenization, nor a measured F4/F5 solving degree.
Affine unit/closure entries are first occurrences in the preregistered scan
starting at the maximum input-generator degree (2 here). In constant-remainder
cases that scan is not a claim that smaller degrees were tested. In the
displayed F43 fixture all inputs are quadratic, so degree0/1 has no input rows.

## Why the preprocessing is valid

For gx=x^2-sx+t and gy=y^2-sy+t, the universally checked division identity is

    S3=(y^2-2zy+z^2)gx+((s-2z)x+z^2-t)gy+h.

Here h=Axy+B(x+y)+C, with the exact coefficients in the protocol and Lean
source. Lean proves the identity over any commutative ring, equality of
the generated ideals and equivalence of the constrained equations. Native
arithmetic independently lifts every root to actual signed elliptic points;
relations, ranks and accepted coefficient sets are unchanged by preprocessing.

Fifteen new named Lean theorems add22 audited declarations. The unmodified
scoped gate now passes **159 named research theorems /207 audited declarations**
across13 modules, with zero failing or extra trusted axioms. Only propext,
Classical.choice and Quot.sound occur. The new project depends on the old
project by path so none of its frozen Lake/Lean inputs are rewritten.

| Claim | New Lean declaration |
| --- | --- |
| Universal division polynomial | `ReducedS3.division_identity` |
| Exact constrained roots and ideal equality | `membership_system_iff`, `generated_ideals_equal` |
| Reduced homogeneous component | `reduced_degree2_component`, `bilinear_head` |
| Actual leading ideal normalization and cutoff | `normalized_head`, `head_monomial_member`, `head_cutoff_exact` |
| Actual F43 coefficients, factorization and leading parts | `F43.main_remainder`, `twist_remainder`, `twist_remainder_factor`, `actual_head_components`, `both_head_ideals` |

Source-bound IDC1h77b3276833a18411 checks the division polynomial at32+16
native points; false-plus-one and false-source controls reject. This certificate
does not replace the Lean ideal/cutoff proofs or certify a running time.

## Reduction, attribution and next lead

The previously proved actual map phi(O)=O, phi(x,y)=(41x,11y) sends the main
generator to(24,26) and preserves arbitrary integer scalars. Thus the original
main DLP still reduces to this twist instance without a scalar-lookup map.
The old11 independent synthetic recovery checks remain their original receipts;
this round does not claim another scalar-recovery rerun. Both directions of
matched-base transport reproduce the new degree/rank profiles natively.

All10 nonzero F43 targets are plotted; the twist has losses as well as gains.
All100 pairings of its ten main and ten twist bases are retained, alongside
88 small-field prefixes. This is not stationary yield or a complete-work bound.
Preprocessing cost, destination solving, linear algebra, target descent,
transfer/recovery and external references remain to be charged before speed
claims. A gain reproducible on the main curve is attributed to base and
generator choice; it need not be intrinsically unique to twisting to be useful.

The next concrete lead is **adaptive reinsertion of newly found lower-degree
rows**, followed by a certified solver-degree comparison on these exact inputs.
It has a separate backlog card and is not reported as executed here. The
continuation policy keeps the next bounded experiment active after this result.
The shared Conductor CLI still exits127; no reservation/replacement is claimed.
CI/review/integration and broader mathematical gates remain open.

Earlier dated reports, all source-bound certificates, registry/benchmark rows
and both catalog provenance dependencies are unchanged. Canonical leaderboard
and catalog graphs stay unchanged because this round introduces no validated
performance row or catalog finding; this new degree graph records the result.
