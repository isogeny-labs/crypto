# P-192 closed-rank search, 27 November 2026

The completed old-base fibre saturation and fixed-column signed-norm test
give **zero additional rank** on the unchanged original 225 elliptic columns.
The positive-rank objective remains open. These are finite component results
for the recorded P-192 d7/kernel3 construction. [pcc:unverified]

![Recorded ranks; one exhaustive finite-domain evaluation per mode. No statistical error bars apply.](ranks.pdf)

## Frozen question and inputs

Keep the original 225 signed-canonical elliptic columns, six baseline rows
and their rank-six span unchanged. The existing tangent matrix has 108 rows
and 429 columns, including 204 auxiliary columns. Success requires positive
additional rank on the original base after eliminating every auxiliary
column, with actual geometric/KM and elliptic verification. Merely adding
columns or increasing full rank does not meet that criterion.

All inputs bind to the public NIST P-192 named curve prime192v1:

```
p = 6277101735386680763835789423207666416083908700390324961279
r = 6277101735386680763835789423176059013767194773182842284081
```

The exact
quartic, map and old matrix are frozen in the input and source manifests.
The degree-six map and exceptional charts are established in parent PR5258;
this study does not change those maps.

## Completed native evaluations

| Evaluation | Domain | Recorded outcome |
|---|---|---|
| Fibre pilot | O and signed old column0 | 2 and 5 rational preimages; old witnesses retained |
| Full fibre lift | 225 signed columns and O | 652 rational C points, including all original226 |
| Original line scan | Original226 points | Seven unique complete sections; old rank6 |
| Lifted line scan | 652 points;212878 attempts | Same seven sections; old rank6 |
| Original and lifted audits | Original tangent matrix plus sections | Full108, auxiliary102, closed6, gain0 |
| Old signed-norm domain | 451 symbols;101926 pairs | Seven coefficient rows including zero; rank6 |
| Augmented signed-norm domain | 859 symbols;369370 pairs | 109 rows including zero; full108, auxiliary102, closed6 |

Fibre solving uses projective Macaulay quotients, stable-image extraction,
finite-field root extraction and actual map evaluation, including Laurent
evaluation at denominator-zero points. It does not accept base-locus points
merely because cleared equations vanish. The line scan checks complete
quartic restrictions, tangencies, multiplicities and actual elliptic images.
The independent audit uses named OpenSSL group arithmetic and GMP matrix
arithmetic. [pcc:unverified]

Every canonical line section in the fixed signed429-column image domain
has four images, counting multiplicity and O, whose sum is the unchanged
reference sum kappa. The norm test exhausts the necessary four-symbol
condition through complementary unordered pairs. Every pair sum and complement
is compared between OpenSSL and separate GMP affine arithmetic. Grouped and
sorted joins agree on coefficient rows and lexicographically minimal witnesses.
All original rows are recovered. Controls detect omitted required rows,
missing pair data and a changed known-answer coefficient. The separate G,2G
fixture detects (-4,2) at target4G and never enters the actual matrix.
[pcc:unverified]

The augmented union still has closed rank6. Consequently no subset realized
by canonical line sections in that same image domain can add old-base rank.
This rejects the proposed auxiliary-fibre saturation H-SA at that scope,
without performing its expensive geometric lift. This includes closed curve
points mapping individually to rational listed images with multiplicities;
it does not include traces of nonrational elliptic images, new image columns,
or higher-degree function relations. [pcc:unverified]

## Resource and failure record

All mathematical computation is native C++. Each build is limited to300s,
each worker to600s, with one allowed CPU and1GiB virtual address space.
Wrapper wall seconds, recorded once per evaluation, are:
pilot18.070; full lift98.915; original scan28.197; lifted scan131.778;
failed old audit build1.642; corrected old audit5.887; lifted audit6.236;
failed norm build1.583; corrected norm test17.734. These are feasibility
cost receipts, not speed comparisons or repeated timing estimates.

Two build failures are preserved: audit-old-v1 lacked iostream; norms-v1
had an SSL type-name ambiguity and strict indentation warning. Fresh attempts
fixed those integration defects. No mathematical worker failed or timed out.
Each attempt's stdout, stderr, exits, source/input/binary manifests and costs
remain under evidence/. The norm source commit is an ancestor; the frozen
source snapshots and hashes identify the exact executed source.

## Next discriminating work

Do not repeat the exhausted fixed429-column canonical-line search. The next
candidate needs a wider relation domain: nonrational image trace columns,
new shared auxiliary images, or higher-degree functions. Prioritize a small
bounded trace-column experiment: first determine whether degree-two residual
traces from secants through old-base points can contribute anything beyond
the already tested pair-sum collisions. Their trace is fixed by the two
prescribed images and the reference sum, so repeated fibres alone are not a
reason to expect gain. Only proceed to geometric collection if this necessary
rank condition admits a new row; otherwise move to higher-degree functions
with a separately frozen protocol. Every candidate must survive elimination
against the same original base. This is proposed follow-up, not a demonstrated
mechanism. [pcc:unverified]

## Sources and reproduction

- Frozen protocols: PROTOCOL.md, SECTION_PROTOCOL.md, NORM_PROTOCOL.md.
- Run wrappers: run-fibres.sh, run-sections.sh, run-audit.sh, run-norms.sh.
- Nine immutable attempt reports and diagrams are in reports/.
- Prior finite-domain results: ../recorded_pool_20261123/REPORT.md and
  ../closure_20261123/REPORT.md; the signed-pair helper is copied with provenance.
- Diem, *On the discrete logarithm problem in class groups of curves*,
  JTNB24 (2012),639--667, DOI10.5802/jtnb.815, canonical line and
  large-prime construction: <https://www.numdam.org/item/10.5802/jtnb.815.pdf>.
- Gaudry, Theriault and Thome, double large-prime work, graph recombination:
  <https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf>.
- KB tools unavailable in this runtime; these primary sources and repository
  negative results were reviewed before the experiments. No novelty claim.

Canonical graph scope: parent map flow is unchanged because no map changed.
The index-calculus scoreboard and progress timeline receive no speed or
PCC promotion: this is producer evidence of finite closure nulls. The new
study-local rank graph records both finite domains, including the nulls.
