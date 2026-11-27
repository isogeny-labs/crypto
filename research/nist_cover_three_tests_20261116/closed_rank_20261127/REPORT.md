# P-192 closed-rank search, 27 November 2026

The completed fibre, line/conic, norm, residual graph and cross-image tests
give **zero additional rank** on the unchanged original 225 elliptic columns.
The positive-rank objective remains open. These are finite component results
for the recorded P-192 d7/kernel3 construction. [pcc:unverified]

![Recorded full/auxiliary ranks above (log scale) and closed rank below. Norm225 uses the225-column diagnostic. Residual graph adds auxiliary columns;Mixed ranks are derived disjoint-block bounds,not a dense recomputation. One finite-domain evaluation per case; no statistical error bars.](ranks.pdf)

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
| Three-old-slot partial lines | 212843distinct pair/tangent lines | No partial section;seven complete sections |
| Separate GMP geometry replay | All pair-line keys,tangent contacts and retained supports | Exact match;closed6 |
| Two-residual graph | 106421new partial edges plus102seeds | Full106427,auxiliary106421,closed6,gain0 |
| Aggregate/individual cross-images | 101701signed aggregate labels;212842individual slots | Zero cross-aliases;derived208128/208122/6 |
| Divided aggregate aliases | m=1,2,3;101926pairs each | Zero unequal-expression aliases;actual108/102/6 |
| Divided/individual cross-images | 203402signed labels;212842individual slots | Zero cross-aliases;derived309829/309823/6 |
| First residual-fibre expansion | 16new image labels;39new curve points | Computed116/110/6;all15necessary image rows |
| Diversified residual-fibre expansion | 256new image labels;782new curve points | Computed236/230/6;all135necessary image rows |
| First new signed-fibre conic pencils | 16pencils;22944candidate occurrences | Zero supported conics;computed236/230/6 |
| Complete new signed-fibre quadruple family | 87pencils across31groups;124758total occurrences | Zero supported conics;computed236/230/6 |

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

## Partial lines and coupled residual images

The three-old-slot search groups212226unordered pair lines and652tangents
on the frozen652-point pool. Among212843distinct lines,212836have old
support degree2and seven have degree4;none has degree3. None of the652old
points is a flex. A hidden-support fixture recovers the fourth point of an
actual known section;changed-line and omitted-pair controls reject. Thus
this one-new-residual line family is empty on the saved pool.
[pcc:unverified]

A separate GMP prime-field implementation reproduces every ordered pair-line
key and the complete stream digest,all tangent contact orders,the support
histogram and seven retained supports. Histogram and off-curve corruptions
reject. This independently checks the finite geometry arithmetic under one
producer;the established elliptic map remains an input dependency. See
PARTIAL_PROTOCOL.md,evidence/partial-v1/ and evidence/partial-audit-v1/.
[pcc:unverified]

The next collector retains both individual residual images of every rationally
split quadratic on every old pair secant and tangent. It checks full polynomial
products,curve incidence,fresh exceptional-safe map values and actual elliptic
sums. All original204auxiliary columns and newly discovered image columns
remain auxiliary. This tests two coupled columns rather than one aggregate
trace. [pcc:unverified]

| Recorded graph quantity | Native result |
|---|---:|
| Pair/tangent attempts |212878|
| Rational split / irreducible sections |106463 /106415|
| Fully old occurrences / unique sections |42 /7|
| New partial / seed edges |106421 /102|
| Total edge rows / auxiliary vertices |106523 /212842|
| Singleton rows peeled |106319|
| Core rows / components / largest component |204 /102 /2rows|
| Full / auxiliary / closed rank |106427 /106421 /6|
| Additional old-base rank |0|

Every singleton witness is replayed after peeling. Each remaining component
undergoes exact coefficient-aware elimination with the unchanged256-row,
1024-column dense-helper caps. Its resulting old rows merge with the original
rank-six baseline. All102core components add no new old-base rank. Eight
separate synthetic controls match dense GMP ranks,including signed/unsigned
triangles,opposite aliases,loops and zero-auxiliary rows;two corrupted singleton
witnesses reject. Fixture rows never enter the actual matrix. See
GRAPH_PROTOCOL.md and evidence/graph-v1/. [pcc:unverified]

## Aggregate and divided-image comparisons

For every signed old pair A,B,form the aggregate T=kappa-A-B. The complete
101926-pair enumeration reproduces the frozen old norm digest with both
arithmetic implementations. After sign folding,101701distinct labels remain;
225duplicates have exactly the same old-point expression. No aggregate is O
or an original429-column label. [pcc:unverified]

The mixed test compares these labels against every individual rational
residual image from the completed graph:212842slots over106421partial
sections. It reconstructs the previously unretained image observable,with
cost charged to this test. There are zero cross-aliases. Cached X-evaluation
agrees with full map evaluation on all652old points,including two exceptional
cases,and the first64new slots. A separate actual-point fixture exercises
membership;changed coordinates/expressions reject,and small dense matrices
distinguish disjoint and coupled pins. [pcc:unverified]

Thus aggregate constraints form a disjoint auxiliary block with one private
pivot per distinct label. Adding that block to the saved graph gives the
**derived** full/auxiliary/closed bounds208128/208122/6. No dense matrix of
this size was constructed. The aggregate set is a necessary superset;the
zero-cross result also bounds any geometrically realized subset. See
MIXED_PROTOCOL.md and evidence/mixed-v1/. [pcc:unverified]

The divided test compares (kappa-A-B)/m for m=1,2,3. A degree-m closed
divisor whose images all equal a rational Q would have aggregate mQ;these
families test possible aliases without asserting that arbitrary labels are
realized on the cover. Inverse multiplication is unique in the prime-order
elliptic group;division by3does not supply rational3-torsion. Each family
checks226divisions by multiplication back with both GMP and OpenSSL,then
enumerates101926pairs with matching arithmetic. Across305103distinct
xlabels,there are no unequal-expression aliases and no candidate rows.
Each family has225formal duplicates. Actual elimination remains108full,
102auxiliary,6closed. The separate G,2Gfixture detects primitive(2,-1),rank1,
and exercises three identity labels;changed divisions,coefficients and missing
pairs reject. See DIVIDED_PROTOCOL.md and evidence/divided-v2/.
[pcc:unverified]

The last comparison checks the two divided families against every individual
graph image. All203852pairs reproduce the divided stream digests. After
450formal duplicates,203402signed labels remain. None equals O,an original
column or any of the212842individual residual slots. The complete individual
Xstream digest matches mixed-v1. All cached/full-map and matrix controls pass.
The **derived** disjoint-block ranks are309829/309823/6;each pin has auxiliary
coefficient6,a unit modulo r. Together,the three divided families are mutually
disjoint and have no image alias coupling to the saved graph. This is a bound
for this particular pin/graph model. See MIXED_DIVIDED_PROTOCOL.md and
evidence/mixed-divided-v1/. [pcc:unverified]

## Actual residual-fibre anchor expansion

The first expansion selects eight complete split sections after26old-pool
attempts. Their sixteen residual image labels lie outside every original429
column. Solving all sixteen signed target fibres with the complete existing
Macaulay/root/exceptional-map routines gives39new rational curve points,
bringing the pool to691. All sixteen known residual witnesses are recovered.
The116886signed-pair norm test over225old+16new image columns returns15rows
including zero. Both arithmetic streams and both joins agree;every actual
seed row appears. After embedding all rows in original429+16columns,the
computed ranks are116full,110auxiliary,6closed. See ANCHOR_PROTOCOL.md and
evidence/anchors-v1/. [pcc:unverified]

Those eight seed sections share source anchor0. The diversified expansion
keeps them and adds one first-qualifying section for each old anchor1..120.
In321selection attempts it obtains128sections and256distinct new image labels.
It revalidates/reuses the sixteen complete saved fibres and solves only240new
ones. The inventory has782new rational curve points,1434total;all256known
residual witnesses are recovered. Every new image remains auxiliary.
[pcc:unverified]

The complete464166-pair stream has digest

```
2429ab291e249e72dcc03e41b0c5f80fbf7504bf8f4549915827351c22c976bb
```

Both joins retain135necessary image rows including zero and recover all128
actual seed rows. Every retained row and all108original rows pass GMP and
OpenSSL group checks. The full243by685matrix respects the original256row/
1024column dense caps. Direct elimination gives236full,230auxiliary,6closed.
Known-witness omission,changed target,changed characteristic constant and
changed row controls reject;the public nonempty relation fixture passes.
See ANCHOR_EXPANDED_PROTOCOL.md and evidence/anchors-expanded-v1/.
[pcc:unverified]

These are actual fibre expansions with a complete necessary image-level
bound. Every fully supported canonical line in the declared expanded image
domain is a subset of the tested norm rows,and therefore cannot add old-base
rank. No expensive repeat of the geometric all-pair line scan is needed.
The conclusion does not cover other images,nonrational elliptic traces or
higher-degree function relations. [pcc:unverified]

## Conic pencils on expanded actual fibres

The higher-degree follow-up uses the saved1434-point pool without another
fibre solve. Revalidate every curve point,image/sign and128actual seed-line
restrictions and factor products. Original108rows plus128seed rows form a
236-row starting basis with ranks236/230/6. All135saved norm rows are bound
to the seed/baseline span. All original204and new256image columns remain
auxiliary. [pcc:unverified]

There are31new signed-image groups with at least four rational preimages.
Each admits a general-position quadruple. The first16chosen pencils give
22944candidate occurrences and no fully supported degree-eight conic.
The same parameter-bucket collector,run on a separate known-product pencil,
does recover a nonzero conic. Direct checks of21original line products span
rank6;the repeated reference has four multiplicities2and row0. Changed
coefficients,missing support/control conics,seeds and pencil accounting reject.
See EXPANDED_CONIC_PROTOCOL.md and evidence/expanded-conics-v1/.
[pcc:unverified]

The complete follow-up enumerates every four-subset in all31groups. There
are87general-position pencils and no degenerate quadruples. It binds/skips
the16completed empty pencils and scans only71new ones,with101814new candidate
occurrences. Together the two attempts cover124758occurrences. The new
ordered-bucket digest is

```
6fc19f4bf8020f35fee603a933a6bc6c902c8096f26d92003ac739e8704df2a6
```

No supported conic is found. All controls pass,and direct full/auxiliary
elimination remains236/230/6. This completes only the same-signed-image
quadruple/pencil family on the saved pool. Mixed signed-image pencils,
other anchors and other functions are not covered. See
FULL_FIBRE_CONIC_PROTOCOL.md and evidence/all-expanded-conics-v2/.
[pcc:unverified]

## Resource and failure record

All mathematical computation is native C++. Each build is limited to300s,
each worker to600s, with one allowed CPU and1GiB virtual address space.
Wrapper wall seconds, recorded once per evaluation, are:
pilot18.070; full lift98.915; original scan28.197; lifted scan131.778;
failed old audit build1.642; corrected old audit5.887; lifted audit6.236;
failed norm build1.583; corrected norm test17.734; failed trace build6.502;
corrected trace test40.438; conic search27.453; signed-six171.878;
same-image conic extension42.602;partial lines22.842;separate geometry6.555;
coupled residual graph277.788;mixed comparison92.381;failed divided build2.212;
corrected divided15.373;divided/individual comparison96.061.
The initial actual-anchor expansion takes29.979wrapper seconds;the larger
diversified expansion takes131.444wrapper seconds and reuses16fibre solves.
The first new conic pencils take36.278wrapper seconds;the complete same-image
follow-up takes47.927wrapper seconds and reuses16completed empty pencils.
Their workers take20.425and30.997seconds. A second pre-build wrapper failure
takes0.019seconds and is preserved.
The graph collector itself takes265.461seconds;mixed build13.501and scan78.801.
The divided/individual build takes14.143seconds and scan81.873seconds.
The same-image conic wrapper first failed before build in0.012seconds.
These are feasibility
cost receipts, not speed comparisons or repeated timing estimates.

Four build failures are preserved: audit-old-v1 lacked iostream; norms-v1
had an SSL type-name ambiguity and strict indentation warning; trace-v1
used multiplication in a boolean condition under strict warnings;divided-v1
had an unqualified SSL helper type. Its build stopped after2.191seconds;
the corrected divided-v2worker takes7.985seconds. A separate
same-image conic wrapper failed during input copy, before any build or worker.
The all-expanded-conics-v1wrapper repeated that input-copy argument defect;
its fresh corrected v2attempt passes. Both failed wrappers remain intact.
Fresh attempts
fixed those integration defects. No mathematical worker failed or timed out.
Each attempt's stdout, stderr, exits, source/input/binary manifests and costs
remain under evidence/. The norm source commit is an ancestor; the frozen
source snapshots and hashes identify the exact executed source.

## Next discriminating work

The residual graph covers every rationally split pair/tangent section on
the652-point old pool. Aggregate/divided labels do not couple to its individual
images. Actual cover-anchor expansion now reaches1434curve points and256new
auxiliary images,but the complete necessary canonical-line image condition
still retains closed rank6. The conic follow-up now exhausts all87pencils
through four new points with the same signed image,again with no gain.
A distinct successor takes four points in a common x-fibre whose images
include both Qand-Q. Native planning must count those mixed-sign quadruples,
skip all87completed same-sign cases,and keep any scan batches bounded.
The saved actual pool can be reused without another fibre solve. All images
outside the original225must remain auxiliary,and all execution/dense caps
remain fixed. This higher-degree successor is not executed here. Nonrational
elliptic traces remain another open direction. Repeating the completed
fixed relation families cannot produce a different rank.
Any positive candidate still needs actual cover/KM and separate
elliptic/elimination replay.
[pcc:unverified]

## Sources and reproduction

- Frozen protocols: [initial fibre lift](PROTOCOL.md),
  [sections](SECTION_PROTOCOL.md), [norms](NORM_PROTOCOL.md),
  [traces](TRACE_PROTOCOL.md), [conics](CONIC_PROTOCOL.md),
  [same-image conics](FIBRE_CONIC_PROTOCOL.md), [signed six](SIX_PROTOCOL.md),
  [partial lines](PARTIAL_PROTOCOL.md), [residual graph](GRAPH_PROTOCOL.md),
  [mixed images](MIXED_PROTOCOL.md), [divided images](DIVIDED_PROTOCOL.md),
  [divided/individual images](MIXED_DIVIDED_PROTOCOL.md),
  [actual anchors](ANCHOR_PROTOCOL.md),
  [diversified anchors](ANCHOR_EXPANDED_PROTOCOL.md),
  [expanded conics](EXPANDED_CONIC_PROTOCOL.md) and
  [complete same-image conic family](FULL_FIBRE_CONIC_PROTOCOL.md).
- Run wrappers: run-fibres.sh, run-sections.sh, run-audit.sh, run-norms.sh,
  run-trace.sh, run-conics.sh, run-fibre-conics.sh, run-six.sh,run-partial.sh,
  run-audit-partial.sh, run-graph.sh, run-mixed.sh, run-divided.sh and
  run-mixed-divided.sh,run-anchors.sh,run-anchors-expanded.sh,
  run-expanded-conics.sh and run-all-expanded-conics.sh.
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
study-local rank graph records the completed finite domains,including nulls.
