# P-192 closed-rank search, 27 November 2026

The completed old-base fibre saturation and fixed-column signed-norm test
give **zero additional rank** on the unchanged original 225 elliptic columns.
The positive-rank objective remains open. These are finite component results
for the recorded P-192 d7/kernel3 construction. [pcc:unverified]

![Recorded ranks. Norm225 uses the225-column diagnostic; other panels retain the429-column tangent matrix. One finite-domain evaluation per mode; no statistical error bars.](ranks.pdf)

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

## Residual traces and conic sections

The signed residual-trace follow-up tests all quartet sums at0,kappa and
2kappa, covering both equal and opposite canonical trace matches. In both
the225and429column domains, target0has only the formal zero row and target
2kappa has no quartet. Targetkappa reproduces the previous stream hashes and
7/109rows. The union remains closed rank6. Thus the single-aggregate
degree-two secant residual-trace family also has no additional old-base rank
within these domains. Positive known-answer fixtures exercise both sign
branches. See TRACE_PROTOCOL.md and evidence/trace-v2/. [pcc:unverified]

The conic follow-up searches64fixed pencils through four points in general
position on the652-point pool. Exact parameter buckets and base-point
tangency parameters cover fully supported degree-eight sections in those
pencils. Smooth local series determine multiplicities when the first tangent
coefficient vanishes; total intersection order must be8. The search examines
41632candidate occurrences and retains21conics: exactly the products of two
distinct original section lines. These rows span the original rank6.
The repeated reference conic has four points of multiplicity2and row0;
changed-coefficient and missing-support controls pass. Closed rank stays6.
This is exhaustion of the declared pencils, not of all conics.
See CONIC_PROTOCOL.md and evidence/conics-v1/. [pcc:unverified]

The cover-fibre extension adds every conic pencil through the25signed image
groups with four rational preimages each. All25quadruples are in general
position; none is excluded. Across89total pencils and57932candidate
occurrences, the only supported conics are still the21known line products.
Closed rank remains6. This extension uses the actual cover fibres rather
than arbitrary extra anchors. See FIBRE_CONIC_PROTOCOL.md and
evidence/fibre-conics-v2/. [pcc:unverified]

## Complete signed length-six norm domain

The full original225-column test enumerates15390826unordered triples over
the451-symbol signed alphabet. Every triple sum agrees between OpenSSL and
GMP; the ordered-stream hash is:

```
80c72c8ca7f89093bc7262f94231d86a404d8968671efb930bfef6b93208dd1d
```

Grouping by signed canonical sum gives7644676groups, largest size452.
Representative differences span every within-group six-word relation;
identity-sum triples are handled separately. Every retained coefficient row
cancels formally: zero nonzero rows, closed rank6. The first64-column pilot
and public G,2G fixture controls pass, including a nonzero known-answer row
and comparison against all matching fixture pairs. [pcc:unverified]

This exhausts signed coefficient relations of L1norm at most6on the fixed
old base, not longer relations or other point sets. No geometric-positive
claim follows from this null norm test. Table payload492506432bytes; no
giant table is committed, only source,input bindings and ordered-stream hash.
See SIX_PROTOCOL.md and evidence/six-v1/. [pcc:unverified]

## Resource and failure record

All mathematical computation is native C++. Each build is limited to300s,
each worker to600s, with one allowed CPU and1GiB virtual address space.
Wrapper wall seconds, recorded once per evaluation, are:
pilot18.070; full lift98.915; original scan28.197; lifted scan131.778;
failed old audit build1.642; corrected old audit5.887; lifted audit6.236;
failed norm build1.583; corrected norm test17.734; failed trace build6.502;
corrected trace test40.438; conic search27.453; signed-six171.878;
same-image conic extension42.602.
The same-image conic wrapper first failed before build in0.012seconds.
These are feasibility
cost receipts, not speed comparisons or repeated timing estimates.

Three build failures are preserved: audit-old-v1 lacked iostream; norms-v1
had an SSL type-name ambiguity and strict indentation warning; trace-v1
used multiplication in a boolean condition under strict warnings. A separate
same-image conic wrapper failed during input copy, before any build or worker.
Fresh attempts
fixed those integration defects. No mathematical worker failed or timed out.
Each attempt's stdout, stderr, exits, source/input/binary manifests and costs
remain under evidence/. The norm source commit is an ancestor; the frozen
source snapshots and hashes identify the exact executed source.

## Next discriminating work

Do not repeat the exhausted fixed-column line or single-trace families.
A successor needs a concrete mechanism for additional supported divisors or
for coupling several distinct residual image columns. The tested families
do not exhaust those possibilities. Repeating the tested single-trace,
same-image conic or length-six domains cannot produce a different result
without changing the declared inputs or family. Any positive candidate still
needs actual cover/KM and separate elliptic/elimination replay.
[pcc:unverified]

## Sources and reproduction

- Frozen protocols: PROTOCOL.md, SECTION_PROTOCOL.md, NORM_PROTOCOL.md,
  TRACE_PROTOCOL.md, CONIC_PROTOCOL.md, FIBRE_CONIC_PROTOCOL.md, SIX_PROTOCOL.md.
- Run wrappers: run-fibres.sh, run-sections.sh, run-audit.sh, run-norms.sh,
  run-trace.sh, run-conics.sh, run-fibre-conics.sh, run-six.sh.
- Immutable attempt reports and diagrams are in reports/.
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
