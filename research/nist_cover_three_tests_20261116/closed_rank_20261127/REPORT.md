# P-192 closed-rank search, 27 November 2026

The completed fibre, line/conic, norm, residual graph and cross-image tests
give **zero additional rank** on the unchanged original 225 elliptic columns.
The positive-rank objective remains open. These are finite component results
for the recorded P-192 d7/kernel3 construction. [pcc:unverified]

![Recorded full/auxiliary ranks above (log scale) and closed rank below. Norm225 uses the225-column diagnostic. Mixed-image bounds are derived. Starred cases restore102private ranks to computed core counts;see text. One finite-domain evaluation per case; no statistical error bars.](ranks.pdf)

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
| Mixed-sign x-fibre pencils | 538new pencils;771492occurrences | Zero supported conics;computed core134/128/6 |
| Expanded balanced-image bound | 481images;464166pairs at each target0,2kappa | Only formal zero patterns;no2kappaquartet;core134/128/6 |
| Stratified mixed-image pencils | 192pencils;274756candidate occurrences | 192known products;zero additional conics;core134/128/6 |
| Full retained-image union | 685images;940506pairs at each0,kappa,2kappa | Formal zero,237known kappa rows,no2kappaquartet;direct236/230/6 |
| Complete old-pair strata | 168choices for seeds0,32,64,96;164new pencils | 28distinct known products;zero additional conics;core134/128/6 |
| Product-free four-line pencils | 512new pencils;734208candidate occurrences | Zero supported conics;core134/128/6 |
| Three-point/tangent pencils | 384new pencils;550656candidate occurrences | Zero supported conics;384higher-contact parameters checked;core134/128/6 |
| Two-point/bitangent pencils | 128new pencils;183552candidate occurrences | Zero supported conics;256higher-contact branches checked;core134/128/6 |
| Actual higher-contact profiles3+1and4 | 512new pencils;734208candidate occurrences | Zero supported conics;768exact branch replays;core134/128/6 |
| Degree-six poles 2E_ref | 15,390,826 shifted triples at target 2rho | Only formal zero row; direct 108/102/6 |
| Degree-six poles H+2R | 15,390,826 shifted triples at target kappa | Exactly seven known rows; direct 108/102/6 |
| Conics tangent at exceptional R | 15,390,826 shifted triples at target 2kappa | No six-image word; direct 108/102/6 |
| Three whole residual traces | 15,390,826 shifted triples at target 3kappa | No word; combined signed-case bound leaves 108/102/6 |
| Same-image residual m=5,6 | Shared 101,926 pairs; 902 multiple queries | No matches in either degree; direct 108/102/6 |
| Actual ramification | Degrees 1,1,2; four simple geometric supports | Map indices 2; specified-axis polar rank 3; Ram-H gives 237/231/6 |
| Individual ramification poles | H+2B1, H+2B2, H+Q; 15,390,826 triples each | Zero words in all three; joint 237/231/6; partial poles reduce to canonical H |

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

## Mixed-sign pencils and broader balanced-image bound

Native planning groups the new points by x-coordinate,including both signed
images. There are91fibres with at least four rational points and625general-
position quadruples,with no degeneracies. It binds/skips all87completed
same-sign cases and schedules538new mixed-sign pencils in five batches of
128,128,128,128and26. Plan digest:

```
f1d6a36e7957bb528865e2cce17e7233c47076300a21284fdebc8e7f1764e1d7
```

The original102added tangent rows still have private witnesses in their204
columns;every new row has zero coefficients there. Exact removal leaves a
134-row/481-column working core,with134full/128auxiliary/6closed ranks.
Original236/230ranks are rechecked against that core. A mutant new row
touching an old private column rejects. Restoring the private block adds102
to both ranks;restored236/230counts are **derived**,not a new larger dense
computation. The256row/1024column helper caps stay unchanged. [pcc:unverified]

All five batches finish with771492candidate occurrences and no supported
conic. Final native combination checks gap-free,nonduplicated complete
coverage and provides a fresh replay path for any accepted support,row and
origin-pencil witness. Nonempty replay and collector controls pass;changed
plans/support/rows and omitted/duplicated batch ranges reject. The combined
batch-receipt digest is

```
d2ab06b8fd5f26ad0c160214e33a64b222cd1fefd2e0fe1f8850b8b7abfc5178
```

Thus all625pencils from general-position quadruples in the new x-fibres are
now covered. Core ranks remain134/128/6. See MIXED_SIGN_CONIC_PROTOCOL.md
and evidence/mixed-sign-conics-v1/. [pcc:unverified]

The balanced-image follow-up uses all481old/new signed image labels. A conic
has total image2kappa. If four image slots sum to0,the complementary four
must sum to2kappa,even when those slots come from different fibres. The new
complete target0and2kappaenumerations each cover464166pairs with matching
OpenSSL/GMP streams and grouped/sorted joins. Target0has one coefficient-
zero row only;target2kappa has no quartet. All135saved kappanorm rows are
verified/reused without reenumeration. Appending the new zero row leaves
core rank134/128/6. Nonempty fixtures at0and8Gand coefficient,row/pair and
private-column corruption controls pass. [pcc:unverified]

This excludes the stated balanced conic pattern over the fixed rational
image domain,including repeated slots. Combined with the imported kappa
inventory,it also extends the earlier single-aggregate residual-trace bound
to481images at closed rank6,using the elimination in TRACE_PROTOCOL.md.
Other image patterns and nonrational individual elliptic images remain
outside this bound. See EXPANDED_BALANCED_PROTOCOL.md and
evidence/expanded-balanced-v1/. [pcc:unverified]

## Mixed-image split-line pencils

The192stratified mixed-image pencils use128old/new and64new/new line pairs
on the saved1434-point pool. Each anchor tuple is general and has a nonzero
four-image sum,so this is a different selection from the completed x-fibre
and balanced-image tests. All192pencils recover their nominated line product.
The274756candidate occurrences yield192supported known products and zero
additional conics. Each product has an exact normalized polynomial identity
and a row equal to the sum of its two parent line rows. The reference
dictionary contains9180distinct products of135known lines. [pcc:unverified]

Native final combine replays geometry,coverage,parent certificates and
labelled row-space representations. Only an independent added row would
enter the capped basis. The actual core remains134full,128auxiliary,6closed;
adding the102private ranks gives derived236/230counts. A separate numerical
old-column unit vector reaches closed rank7and checks the independence
collector. It is synthetic and enters no actual elliptic matrix. See
MIXED_IMAGE_CONIC_PROTOCOL.md and evidence/mixed-image-conics-v1/.
[pcc:unverified]

## Complete old-pair strata

The192pencil plan used one old pair per seed. For seeds0,32,64,96,the
extension enumerates every unordered pair on each of the seven old lines,
combined with the seed's two new residual points. All168choices are general,
mixed-image and nonbalanced. Four exact tuples were completed in the prior
plan;the other164are scanned once. Native validation recomputes the prior
plan digest and verifies the complete168-choice partition. [pcc:unverified]

The164new pencils cover234684candidate occurrences and recover28distinct
known line products,no additional conic and no independent row. Several
pencils contain the same product,so28is a count of distinct accepted conics,
not a count of pencils. Complete geometry and exact parent-row dependence
certificates replay in final combine. Core ranks remain134/128/6;restored
236/230are derived. This completes only these four seed strata. See
MIXED_IMAGE_EXTENSION_PROTOCOL.md and evidence/mixed-image-extension-v1/.
[pcc:unverified]

## Product-free four-line pencils

To test a selection without a guaranteed known product,for s=0..31choose
one residual point from each of seeds s,s+32,s+64,s+96. The sixteen binary
choices for each s give512raw tuples. Every tuple is general,mixed-image
and nonbalanced. Native incidence checks show no known line contains two
anchors. Two known lines therefore cannot cover all four anchors;none of
the9180known products lies in an actual pencil. Known-product anchors fail
this predicate in a separate rejection fixture. [pcc:unverified]

Both completed192and164plans bind356distinct prior tuples;none repeats.
All512admitted pencils cover734208candidate occurrences in four batches.
No supported conic is found. Nonempty separate collector/replay fixtures
still detect their known conic,and numerical/dependence/private-block
controls pass. Final combine checks exact coverage and witness replay.
Core ranks remain134/128/6;restored236/230are derived. This is a complete
null for the fixed512choices,not all product-free conic pencils. See
FOUR_LINE_CONIC_PROTOCOL.md and evidence/four-line-conics-v1/.
[pcc:unverified]

## Three-point conics with forced tangency

The next experiment changes the base scheme to2+1+1:three distinct points,
with a tangent condition at one. For s0..15,it uses one residual point on
each of seeds s,s+32,s+64,all eight residual choices and each of the three
choices of tangent anchor. All384raw choices are admissible. The anchors
are noncollinear,and neither other anchor lies on the chosen C-tangent.
Four independent constraints on six conic coefficients give a checked
two-dimensional basis. No other pool point is a base point,distinguishing
these pencils from the earlier four-distinct-point base schemes.
[pcc:unverified]

The first derivative at the forced tangent point vanishes for the entire
pencil. The collector therefore uses its second local coefficient to select
the unique higher-contact parameter. It also enumerates every other pool
point and tangency at each ordinary anchor. All384forced parameters are
checked and have contact at least three. Exact local-monomial coefficients
come from the existing precision9smooth lift;accepted higher multiplicities
are compared with the original homogeneous evaluator. [pcc:unverified]

A separate same-collector fixture uses two known seed lines sharing old
anchor0. Their product passes through one residual point on each and has
order two at0;the collector recovers its nonzero image row. The fixture's
selected higher-contact parameter has order three. Using the identically
zero first-derivative selector,omitting that parameter,changing the basis,
tangent origin,support or row and corrupting batch coverage all reject.
The fixture never enters the actual data. [pcc:unverified]

Three native batches and final combine cover550656candidate occurrences
and find no supported conic or independent added row. Actual pencils
exclude known line products by anchor incidence and dictionary checks.
Core ranks remain134/128/6;restored236/230counts are derived. Numerical
closed7is a separate synthetic control. This completes the fixed384choices,
not all tangent conics. See TANGENT_CONIC_PROTOCOL.md and
evidence/tangent-conics-v1/. [pcc:unverified]

## Two-point conics with tangency at both anchors

The next base scheme is2+2. For s0..31,the experiment takes one residual
point on each of seeds s,s+64,using all four choices:128raw pairs. Every
pair passes mutual C-tangent exclusion and four independent point/value
and derivative constraints,giving a verified two-vector nullspace. No other
pool point is a base point. All pairs are mixed and have nonzero weighted
base image sum. [pcc:unverified]

At two anchors,one point per known line does not by itself exclude a
product. The planner verifies every incident known line has contact one,
then checks every unordered pair of known lines,including repetitions,
against both required contact orders. None of the9180known products meets
2+2at an actual pair. The same test rejects a squared-known-line control
from the actual domain. [pcc:unverified]

The collector checks second local coefficients at both forced anchors,
in addition to every other pool point. Both first derivatives vanish
identically. All256higher-contact branches select order at least three;
branches count separately even if coefficient keys coincide. The separate
same-collector fixture uses nonreference line1squared and two of its support
points. It recovers a nonzero actual row;the two higher-contact parameters
have orders3and3. Zero selectors,omitted parameters,bad basis/contact
profile/origin/support/row and coverage reject. [pcc:unverified]

One native128pencil batch and final combine cover183552candidate occurrences
and find no supported conic or independent row. Core ranks remain134/128/6;
restored236/230are derived. The numerical closed7fixture never enters actual
data. This completes only the fixed128choices. See
BITANGENT_CONIC_PROTOCOL.md and evidence/bitangent-conics-v1/.
[pcc:unverified]

## Higher-contact fixtures and actual3+1/4pencils

The generic engine represents a base scheme by distinct weighted anchors
whose contacts sum to four. It imposes exact local coefficients from zero
through one less than each prescribed contact,then checks rank four and
a two-vector nullspace. Extra-point buckets and the first unfixed
coefficient at each anchor cover fully supported degree-eight sections.
Higher contacts use the existing precision9smooth lift and original
homogeneous evaluator. [pcc:unverified]

First,a control-only preflight selects seed10and ordinary old anchor3.
Its split C-tangent T has two missing residual points/images. A temporary
1436-point/483-column workspace validates both:T times the seed line
supplies the3+1fixture,and T squared supplies the4fixture. The collector
recovers each nonzero row and checks tangent-plus-seed/twice-tangent row
decompositions. Higher-contact parameter orders are [4,2] and [5]. Control
row rank two is a fixture result. It is excluded from the actual rank search.
[pcc:unverified]

![Native control preflight: two points and two image labels are control-only. Exact original point/base/reference/matrix contents and the full subject digest match after restoration;zero fixture rows enter actual data.](control-isolation.pdf)

The original1434points,481columns and matrix restore exactly;an intentional
sign mutation is rejected. Control mapping also restores local precision
after exceptional-map evaluation. No actual pencil runs in this preflight.
See HIGH_CONTACT_CONTROL_PROTOCOL.md and evidence/high-contact-controls-v1/.
[pcc:unverified]

The subsequent actual experiment fixes256raw3+1choices from paired seeds
s,s+64 for s0..31,all residual choices and either triple point. Another256raw
choices impose contact four at each of the two residual points of all128
seeds. All512pass nonflex/geometry,rank-four basis,known-product contact
and weighted-image checks. Only original pool points and labels are used.
Four native batches cover734208candidate occurrences and find no supported
conic or independent row. Core134/128/6is computed;restored236/230counts
are derived. [pcc:unverified]

Every one of768parameter witnesses retains its normalized coefficients,
origin,branch and observed order. Final native combine rederives the unique
first-unfixed parameter,compares the original homogeneous order,and checks
complete gap-free/nonduplicate batch and branch coverage. The histogram
contains256branches each at1->2,3->4and4->5. Coefficient/order/profile/basis,
omitted-parameter,support/row and restoration controls all reject their
mutants. No exceptional contact order occurs in these finite branches.
[pcc:unverified]

![Native contact-order histogram for512actual pencils and768branches. Labels show base profile,prescribed contact and observed higher contact. Each bar is a deterministic finite count;fixtures are excluded.](contact-orders.pdf)

This exercises all five degree-four base multiplicity types on declared
finite families;their other anchor choices remain untested. See
HIGH_CONTACT_SCAN_PROTOCOL.md and evidence/high-contact-scan-v1/.
[pcc:unverified]

## Full retained-image union with active auxiliaries

The combined685-image domain contains all429original images plus256new
images. Earlier429and481tests overlap on225old labels;their separate nulls
did not establish the cross-domain condition. Here all460auxiliary columns
stay active. Original108rows plus128mapped seed rows form an independent
236-row basis. Both completed kappa inventories bind to exact coordinates
and basis representations:109old rows and135new rows give237distinct rows
in their union. [pcc:unverified]

For each target0,kappa,2kappa,the new enumeration covers940506unordered
signed pairs,including O,repetitions and both signs. Every pair sum and
complement agrees in GMP and named OpenSSL arithmetic. Independent grouped
and sorted joins agree on every coefficient row and minimal witness. The
results are one formal zero row at0,exactly237imported rows at kappa,and no
quartets at2kappa. Every retained row passes both elliptic checks and an
exact reconstruction in the236-row basis. No new nonzero row or independent
constraint is found. [pcc:unverified]

Direct matrix elimination gives full rank236,auxiliary rank230,closed rank6.
This calculation uses all685columns;no private102-rank block is removed or
restored. It extends the necessary canonical-line/single-aggregate trace
bound and exclusion of conics with a four-slot zero-sum subpattern to this
combined domain. Other conic patterns remain outside the bound. Nonempty
G,2Gcontrols cover all three targets;coefficient,missing-row/pair and basis
certificate mutations reject. The separate numerical closed7fixture never
enters actual data. See FULL_IMAGE_BOUND_PROTOCOL.md and
evidence/full-image-bound-v1/. [pcc:unverified]

## Degree-six functions with a different pole divisor

The native reference-shifted test changes the pole divisor to
$D=2E_{\mathrm{ref}}$, where $E_{\mathrm{ref}}$ consists of original
quartic points 223, 224 and 225. Fresh actual map evaluation identifies
these images as the negatives of columns 222, 223 and 224. If
$\rho=f_*(E_{\mathrm{ref}})$, a function in $L(D)$ has effective zeros of
degree six, after padding cancelled poles, whose elliptic image sum is
$2\rho$. The native target differs from $O$, $\kappa$ and $2\kappa$.
Thus the previous zero-target six-symbol result does not answer this
question. [pcc:unverified]

The exact target is:

```
x = 2605172243064974128828395173461348789790288923828041863253
y = 4139875000803477430951892779468652918082041115551322701550
```

Each unordered triple is shifted by $-\rho$. Two such shifted sums are
opposites exactly when the original six images sum to $2\rho$. The
complete 451-symbol alphabet consists of $O$ and both signs of every
original column. Native GMP and named OpenSSL arithmetic agree on all
15,390,826 triples and their ordered hash:

```
097e969b718c8f5ee9b16fbe73b3b62ced6c7ef77b37c3f218e0dbb4aac2760c
```

An in-place grouped join and a separately indexed orientation-stream
merge agree on all coefficient rows and witnesses. There are three
opposite-orientation pairs and one identity self-pair. All recover the
single zero row with formal word 446,446,448,448,450,450, representing
the doubled reference itself. No nonzero coefficient row remains.
The actual full/auxiliary/closed ranks are 108/102/6. All rows have only
original-column support, so this is the same closed rank-six span as
in the expanded core. [pcc:unverified]

The 32-byte table payload is 492,506,432 bytes, and the second join uses
61,563,304 bytes of index payload. These are exact native data sizes,
not peak-memory measurements. The table is not committed. A separate
public G/2G fixture recovers a nonzero row of rank one; both joins equal
an independent brute unordered six-word enumeration, including their
minimal witnesses. Changed coefficients, omitted rows/triples and
altered orientation reject. Fixture rows enter no actual matrix.
[pcc:unverified]

This necessary bound excludes added projected rank from $L(2E_{\mathrm{ref}})$
when each individual zero image lies in the original signed rational
image domain. Repetitions and closed source points mapping to those
rational images are included by multiplicity. A zero projected row can
still correspond to different curve-point supports; it contributes no
new equation to this elliptic-column matrix. The test does not exhaust
other pole divisors, longer words or nonrational individual images.
It does not construct Riemann–Roch functions. See
[the frozen protocol](SHIFTED_SIX_PROTOCOL.md) and the separate
[interrupted](reports/shifted-six-v1/REPORT.pdf) and
[complete](reports/shifted-six-v2/REPORT.pdf) reports. [pcc:unverified]

## Canonical reference with an exceptional-point pole

For $D=H+2R$, the reference section $H$ is the actual line $x=56$
with support indices 42,43,44,45. Original point 0 is the exceptional
point $R$, and fresh local-chart evaluation verifies $f(R)=O$.
The degree-six reference word therefore has image sum $\kappa$.
Since the curve has genus three, Riemann–Roch gives $l(D)=4$.
The previous target-$\kappa$ bound used four symbols. [pcc:unverified]

The native six-symbol test verifies the complete factor product for $H$,
its four images, both copies of $R$, and a group half of $\kappa$ used
only for the shifted join. Both native engines agree on all 15,390,826
triples. Both joins recover exactly the seven known coefficient rows,
including zero, from 15,715 opposite-orientation pairs. There are no
identity pairs and no independent added row. Direct ranks remain
108/102/6. The exact ordered digest is:

```
7eb97904c8cd8659c0c6165662a89a3267f8cd3489464a45ffd196dbf56fa234
```

All seven original section rows must be recovered, and an omitted-row
mutation rejects. The nonzero fixture and independent brute enumeration
also pass. Thus this larger pole family supplies no added projected rank
when all individual images lie in the unchanged signed original domain.
Other images remain outside the bound. See
[the protocol](CANONICAL_SIX_PROTOCOL.md) and
[the run report](reports/canonical-six-v1/REPORT.pdf). [pcc:unverified]

## Complete necessary bound for conics tangent at R

A conic $N$ with intersection order at least two at exceptional point $R$
has six remaining intersections, counting multiplicities. With the fixed
reference line equation $L_H$, its rational function satisfies
$\operatorname{div}(N/L_H^2)=Z+2R-2H$. Therefore
$f_*(Z)=2\kappa$. Equivalently, the Riemann–Roch divisor
$D=2H-2R$ has degree six and $l(D)=4$. It has a negative coefficient at
$R$, so it is not an effective pole divisor. [pcc:unverified]

Native checks freshly verify that $H$ is disjoint from smooth $R$, whose
exceptional image is $O$. Conic value and tangent constraints at $R$ have
rank two on the six conic monomials. This verifies the four-dimensional
conic numerator family without constructing every numerator. The full
six-symbol necessary image test uses shift $-\kappa$ and target $2\kappa$.
The earlier target-$2\kappa$ test had four symbols. [pcc:unverified]

All 15,390,826 triples agree between native GMP and named OpenSSL
arithmetic. Both joins have zero opposite-orientation pairs and zero
identity pairs: **there is no six-image word at all** at this target in
the fixed signed original domain. The actual matrix remains 108/102/6.
There is no required formal zero candidate for this divisor, and none is
inserted. The exact ordered digest is:

```
c8763b80a5afe8880a93fea5fe747b5174d4be6da992e951a147ad7a44f51e85
```

Consequently no conic tangent at $R$ has all six residual individual
images in that domain, including repeated slots and closed source points
with rational listed images. This bounds the whole declared conic family,
rather than only the earlier finite pencil samples. It does not cover
other image domains or conics without the prescribed contact at $R$.
Nonzero fixture/brute enumeration and actual-reference coefficient
corruption checks pass. No fixture enters the actual matrix. See
[the protocol](EXCEPTIONAL_CONIC_SIX_PROTOCOL.md) and
[the run report](reports/exceptional-conic-six-v1/REPORT.pdf). [pcc:unverified]

![Three completed degree-six necessary image bounds. Each uses the same original 451-symbol alphabet and 15,390,826 triples; counts are exact finite evaluations. No repeated-sample uncertainty applies. Fixture rows are excluded. Divisor dimensions come from Riemann–Roch; actual conic local constraint rank is separately checked.](degree-six.pdf)

## Three whole residual traces

For three degree-two residual traces $T_i=\kappa-A_i-B_i$, closure
$T_1+T_2+T_3=O$ requires six signed old images to sum to $3\kappa$.
The new native test exhausts that necessary condition on the same
451-symbol alphabet. The target differs from $O$, $\kappa$, $2\kappa$
and the completed $2\rho$ target. Both arithmetic streams agree on all
15,390,826 triples, and both exact joins find zero pairs and zero
coefficient rows. No formal zero word is required or inserted.
Actual full/auxiliary/closed ranks stay 108/102/6. [pcc:unverified]

The exact ordered stream digest is:

```
31439860aa79a92bb3c50dfd66804354735d6901f2ac559be77bfb261c280560
```

Three actual split sections have support indices 42..45,79..82,91..94.
For each, native checks reconstruct the complete quartic factor product,
evaluate all four actual map images and compare the residual trace with
$\kappa-A_i-B_i$ using both group implementations. Changed actual traces
reject. These witnesses verify the trace identity; they do not assert
three-trace closure for those actual sections. A separate public fixture
uses $\kappa=2G$ and nonzero traces $G,G,-2G$. It detects coefficient row
$(-4,2)$ on $G,2G$, of rank one. Changed anchors break closure. The
ordinary nonzero six-word fixture and independent small brute enumeration
also pass. No fixture enters the actual matrix. [pcc:unverified]

The result combines with the earlier complete six-symbol target-$\kappa$
bound. For signs $s_i=\pm1$, closure requires the six signed anchor
images to sum to $(s_1+s_2+s_3)\kappa$, hence to one of
$\pm\kappa,\pm3\kappa$. Negation is already included in the alphabet.
The $3\kappa$ case is empty; the $\kappa$ case has exactly seven known
rows in the original rank-six span. Therefore **every signed combination
of three whole traces adds no projected rank within this anchor domain**.
This does not exclude every mixed-sign cycle; such cycles can reproduce
known rows. Selected individual residual points, larger combinations and
other anchor domains remain outside the bound. [pcc:unverified]

![Signed-case reduction for three whole traces. The reduction is algebraic; counts copy the new 3kappa result and the earlier complete kappa result. Exact finite domains, no statistical error bars.](three-trace.pdf)

See [the frozen protocol](THREE_TRACE_PROTOCOL.md),
[the immutable run report](reports/three-trace-six-v1/REPORT.pdf) and
`evidence/three-trace-six-v1/` for exact target coordinates, actual trace
witnesses, controls, hashes, argv, exits and resource costs.

## Same-image residual degrees five and six

For $m=5,6$, a function with poles bounded by $H+(m-2)R$ and two
old image slots $A,B$ leaves effective residual degree $m$. If all its
residual images equal a rational old point $P$, its trace is $mP$ and
$\kappa-A-B=mP$ is necessary. In particular a closed source point of
residue degree $m$ mapping to rational $P$ has that trace. Existence of
such a closed point or principal divisor is not assumed. [pcc:unverified]

The native test reconstructs 101,926 signed unordered pair records once.
Every sum/trace agrees between GMP and named OpenSSL, and the ordered
digest matches the saved original norm receipt. It then checks 902 query
records: all 451 signed labels, including $O$, multiplied by five and
six. Both native query streams agree, and there are 901 distinct query
points, with $O$ shared between the families. Lookup and sorted-merge
joins compare every raw match and normalized coefficient witness.
They find **zero matches in each degree** and zero candidate rows.
Actual full/auxiliary/closed ranks remain 108/102/6. [pcc:unverified]

The exact query stream digest is:

```
fba19f52794e1b667f7a8acef287ddae0ac1ac2be616df70fd7ff7ec3bafa963
```

Fresh map/factor checks bind $H$ and exceptional $R$. All seven retained
rows from the earlier target-$\kappa$ result pass both group checks and
exact representation in the frozen six-row span. No completed six-symbol
enumeration is repeated. Public $G,2G$ controls with $\kappa=12G$ detect
primitive row $(2,-1)$ in both new families: raw match counts are two and
three. A second control exercises $P=O$ twice in each family. Both
collectors equal independent small-domain brute evaluation; coefficient,
query, missing-pair/query and missing-candidate corruptions reject.
Each fixture has rank one and enters no actual matrix. [pcc:unverified]

For $m=1,2,3,4$, the image word $A+B+mP$ has length at most six and
can be padded by $O$. The prior complete target-$\kappa$ bound already
puts every such row in the original rank-six span. Combined with the
new degree-five/six nulls, this covers **every possible residue degree
of one closed source point mapping to a rational old image under this
degree-six map**, for the stated reference divisors and two old slots.
For $m=1$, interpret $H-R$ as an RR divisor with a required zero at $R$.
This is degree coverage of this same-image pattern, not a classification
of all closed divisors or all pole classes. Multiple residual image
labels, higher multiplicities with larger total degree and other anchor
or image domains remain outside the bound. [pcc:unverified]

![Closed-residual degree coverage. Counts for m=5,6 copy the new native result. Reduction of m=1..4 to the earlier kappa six-symbol domain is algebraic. Exact finite domains, no statistical uncertainty.](closed-residual.pdf)

See [the frozen protocol](CLOSED_RESIDUAL_PROTOCOL.md),
[the immutable attempt report](reports/closed-residual-v1/REPORT.pdf) and
`evidence/closed-residual-v1/` for witnesses, hashes, exits and costs.

## Actual ramification support and the retained exceptional points

The retained pullback differential is
$f^*\omega_E=L(x,y)\,dx/F_y$. On a smooth plane quartic, adjunction
identifies a linear canonical section with its line-intersection divisor.
The exact differential identity therefore identifies
$\operatorname{Ram}(f)=C\cap\{L=0\}$. The new native run rechecks the small
identity and global elliptic equation, without repeating the completed
composition experiment. [pcc:unverified]

![Actual canonical ramification supports, local map checks, image traces and auxiliary elimination. Values copied from ramification-v1; targeted axis classification is separate from the rank calculation.](ramification.pdf)

The complete monic restriction factors with distinct degrees **1,1,2**,
each with multiplicity one. Its degree is four and its infinity gcd is
constant, so there are no omitted points at infinity. Each of the four
geometric points has local order one for $L$ and actual map index two.
The quadratic component and its image both have Frobenius degree two;
a fresh conjugate local lift agrees in endpoints, valuations, leading
coefficients and indices. Traces include every source conjugate. The
three rational traces sum to $\kappa$ in GMP and the named OpenSSL group.
The full factors, extension coordinates and local coefficients are in
`evidence/ramification-v1/calls/full/stdout.json`.

At every ramification support the displayed $Y$ rational function has
numerator and denominator orders **1/1**. These removable values are
resolved by the local jets; direct substitution into that fraction would
miss them. The two rational points over $O$, retained pool indices **0**
and **226**, instead have map index **one**, $X,Y$ valuations $(-2,-3)$,
and $L$ order zero. This explicitly checks map ramification rather than
inferring it from the source coordinate chart.

Let the three signed-canonical trace columns be 685,686,687. None occurs
among the retained 685 images, even up to sign. The actual principal
relation from $L/L_H$ is

$$e_{41}-e_{42}-e_{43}+e_{44}+e_{685}-e_{686}+e_{687}=0.$$

All 460 old auxiliary columns remain active. Adding the three new ones
and this one row changes the actual ranks from **236/230/6** to
**237/231/6**: the row is independent, and its auxiliary part also adds
one rank. It supplies no new relation supported entirely on the original
225 columns. The separate old-unit numerical fixture reaches closed
seven and fails both group-relation checks; it never enters actual data.

For an involution with axis $L$, a center $V$ must satisfy
$\nabla F(Q)\cdot V=0$ for every projective point $Q$ of $L$.
Restricting the three homogeneous polar cubics gives a **4 by 3 matrix
of rank three**, excluding a center over every extension field. The
known Fermat reflection and its noncoordinate conjugate pass; an odd
perturbation has polar rank two but fails exact invariance, showing why
the necessary linear condition alone is insufficient.

A derived consequence concerns a specific proposed factorization of
this map. It cannot be a degree-two map to an elliptic curve followed
by a degree-three isogeny, even over the algebraic closure. Such an
isogeny is unramified in this characteristic; the double-cover involution
would fix these four ramification points. The canonical plane embedding
represents an involution as a projective reflection, and four distinct
collinear fixed points force its axis to be $L$. The rank-three polar
certificate excludes precisely that possibility. This does not classify
all automorphisms, all elliptic quotients, or degree-three-first
factorizations. This implication is a mathematical interpretation of
the recorded certificate, not another measured search. [pcc:unverified]

The principal row and local supports are verified components. Further
closure needs other actual relations connecting the three new traces to
the frozen base; this single row cannot eliminate them. A positive goal
result still needs additional closed rank with separate elliptic and
elimination replay and actual cover/KM validity.

## Individual ramification components as pole divisors

The whole ramification divisor is linearly equivalent to $H$, so using
it wholesale repeats the canonical pole class. The successor instead
uses $D_1=H+2B_1$, $D_2=H+2B_2$ and $D_3=H+Q$, where $B_1,B_2$ are
the two rational ramification points and $Q$ is the degree-two closed
component. Each divisor has degree six and Riemann--Roch dimension four.
Writing $P_i=f(B_i)$ and $T=\operatorname{Tr}(f(Q))$, the necessary
six-image targets are

$$\kappa+2P_1,\qquad\kappa+2P_2,\qquad\kappa+T.$$

The degree-two source component contributes its full trace **once**.
The native plan checks all three targets, full-width references and
public half-targets in GMP and named OpenSSL before allocating a triple
table. The targets are distinct up to sign from one another and from
all completed full-six-word targets $0,\kappa,2\kappa,3\kappa$ and
$2\rho_{\mathrm{ref}}$. Because principal divisors push forward to zero,
these target differences also distinguish the pole classes from those
previous classes with different targets. This is a necessary signature,
not a classification of the entire Picard group. [pcc:unverified]

These three targets were chosen to let the new trace columns cancel.
Let $h$ denote the old coefficient word for $H$. If old-image words
$W_1,W_2,W_3$ realized the three targets, their relations would be
$W_1-h-2P_1$, $W_2-h-2P_2$, and $W_3-h-T$. Adding the first two,
twice the third, and twice the actual relation
$P_1+P_2+T-h$ leaves the pure-old word

$$W_1+W_2+2W_3-6h.$$

Its independence would still require the rank test and actual RR/KM
realization; the construction does not assume a rank gain.

Each of the three workers completes all **15,390,826 unordered triples**
on the 451-symbol original signed alphabet. Every shifted triple and
stream hash agrees between GMP and named OpenSSL; the grouped and indexed
joins agree on every row and witness. All three families return **zero
candidate words**. The separate native combine validates all receipts
and leaves actual and candidate ranks **237/231/6**, with zero joint
additional closed rank. [pcc:unverified]

![Three actual ramification pole classes, their new image targets, complete signed-triple bounds and joint auxiliary elimination. All values copied from ramification-poles-v1.](ramification-poles.pdf)

All 688 columns, including the three ramification trace columns, remain
in the reference and candidate rows. The signed contributions beyond
$H$ are $+2e_{685}$, $-2e_{686}$ and $+e_{687}$ respectively. The
237-row actual matrix and all 463 auxiliary columns remain active.
Candidate words are checked on this entire width; removing their
nonzero trace contributions would not give valid old-base relations.

A nonempty auxiliary-reference fixture uses old symbols $G,2G$, a
separate auxiliary $3G$, and target $6G$ with reference $2(3G)$.
Both joins equal independent brute six-word enumeration. Including the
baseline row $(-2,1,0)$ gives full rank two, auxiliary one and closed one.
An isolated numerical old-unit mutant gives closed two and fails both
group checks. Changed reference coefficient, trace sign, repeated
quadratic-trace weight, finite branch image, orientation, omitted
candidate/triple and missing-family controls reject. No fixture enters
actual data. [pcc:unverified]

The literal comparison uses the **complete padded zero divisor**
$D+\operatorname{div}(u)$ with all six images in the original signed
rational domain. Here a Riemann--Roch argument also handles cancellation
at the new poles. Because $H$ is canonical in genus three,

$$l(H)=3=l(H+B_i),\qquad L(H+B_i)=L(H).$$

Thus a function in $L(H+2B_i)$ either uses the full double pole at $B_i$
or belongs to $L(H)$; an intermediate simple pole supplies no new
function. A function over $\mathbb F_p$ has equal pole orders at the two
conjugates of $Q$, so for $L(H+Q)$ it likewise uses both simple poles or
neither. The new ramification supports are disjoint from the split
reference $H$.

Consequently, when the three full-pole bounds are null, any function in
these three spaces whose **actual zero images** all belong to the old
domain must lie in $L(H)$. Padding cancelled poles at $H$ uses only its
known old images, so the prior complete canonical bound already puts
its projected row in the original rank-six span. This is derived
whole-space coverage for these three divisors, not another measured
search. Other pole classes and zero images outside the fixed domain
remain outside the result. Necessary candidates would still require
actual RR/KM realization and separate replay for positive goal
completion. [pcc:unverified]

## Resource and failure record

All mathematical computation is native C++. Each build is limited to300s,
each worker to600s,with one allowed CPU and1GiB virtual address space.
Each of45immutable attempt reports retains complete per-call and wrapper
costs,including historical failures. The new control preflight takes17.989
build,10.875worker and28.931wrapper seconds. The actual higher-contact
pipeline takes21.747build,46.623plan,
62.156/62.566/59.306/60.530batch and51.188combine seconds;wrapper364.196seconds. Every value is one recorded execution,not a speed comparison or repeated
timing estimate. The corrected reference-shifted build takes 15.338 seconds;
its full worker takes 170.862 seconds and wrapper 186.243 seconds.
The H+2R build, full worker and wrapper take 14.526, 178.044 and
192.615 seconds respectively. The exceptional tangent-conic build, full
worker and wrapper take 14.459, 175.874 and 190.378 seconds respectively.
The three-trace build, full worker and wrapper take 14.425, 175.104 and
189.579 seconds respectively. The same-image closed-residual build, full
worker and wrapper take 15.062, 8.122 and 23.229 seconds respectively.
Earlier costs remain in their unchanged attempt reports.

Four build failures are preserved: audit-old-v1 lacked iostream; norms-v1
had an SSL type-name ambiguity and strict indentation warning; trace-v1
used multiplication in a boolean condition under strict warnings;divided-v1
had an unqualified SSL helper type. Its build stopped after2.191seconds;
the corrected divided-v2worker takes7.985seconds. A separate
same-image conic wrapper failed during input copy, before any build or worker.
The all-expanded-conics-v1wrapper repeated that input-copy argument defect;
its fresh corrected v2attempt passes. Both failed wrappers remain intact.
Fresh attempts
fixed those integration defects. The first shifted-six native run was
intentionally stopped with SIGTERM after 44.156 seconds when source review
found an off-by-one final formal-word label. Its actual target was correct,
but no complete enumeration or final digest is claimed. The strict build
had passed in 13.789 seconds. The corrected fresh v2 derives labels from
actual verified images and checks the formal word before enumeration.
No worker timed out. The interrupted attempt retains its partial records
and available source/input/binary hashes; it has no final result manifest.
Each attempt's stdout, stderr, exits, source/input/binary manifests and costs
remain under evidence/. The norm source commit is an ancestor; the frozen
source snapshots and hashes identify the exact executed source.

The ramification-support strict build takes 16.145 seconds, its native worker
30.713 seconds and complete wrapper 46.908 seconds. These are resource
receipts from one capped execution, not performance comparisons.

The individual-ramification pole run has strict build 17.317 seconds,
plan 15.918 seconds, family workers 185.503/185.083/184.422 seconds and
combine 17.949 seconds. The serial wrapper totals 606.283 seconds across
all six calls; the 600-second limit applies to each native worker.
These are single-execution resource receipts, not performance comparisons.

## Next discriminating work

The residual graph covers every rationally split pair/tangent section on
the652-point old pool. Aggregate/divided labels do not couple to its individual
images. Actual cover-anchor expansion now reaches1434curve points and256new
auxiliary images,but the complete necessary canonical-line image condition
still retains closed rank6. The conic follow-up now exhausts all87pencils
through four new points with the same signed image,again with no gain.
A mixed-sign follow-up now covers the remaining538pencils,completing all625
quadruples in new x-fibres without a supported conic. The481-image bound also
excludes conics containing a four-slot zero-sum subpattern. The192mixed-image pencils have now completed and recovered only certified
known products. Their null does not cover all conic pencils on the1434-point
pool. The full685-image union now directly checks interactions
between all460auxiliary labels and retains closed rank6. The old-pair extension completes all168choices for four seed strata,
including164new scans and four reused tuples. The512product-free four-line
pencils also complete with no supported conic. Other seed combinations and
conic pencil types remain outside those finite domains.

The three-point/one-tangent successor has now completed384pencils and
550656candidate occurrences,including the forced higher-contact branch,
with no supported conic. The2+2bitangent successor has also completed128pencils and256higher-
contact branches,with no supported conic. Both higher-contact types have now completed512additional pencils with
no supported conic. This is finite type/anchor coverage,not exhaustion of
all conics on the pool.

All three degree-six successors now complete: $2E_{\mathrm{ref}}$ has
only the formal zero row, $H+2R$ has exactly seven known rows, and
$2H-2R$ has no six-image word. They leave the frozen closed span at six.

The three-whole-trace successor now completes with no $3\kappa$ word.
Together with the prior $\kappa$ six-symbol bound it closes every signed
three-whole-trace combination at the current projected span.

The degree-five/six same-image successor now completes with no match;
together with the prior target-$\kappa$ result it covers residue degrees
one through six for the stated one-point residual pattern.

The structural ramification follow-up now completes. The canonical line
has simple support of degrees 1,1,2, with actual map index two at every
geometric point. Its three fresh rational traces yield one actual row,
which leaves closed rank six after full auxiliary elimination. The
specified-axis reflection is excluded by a rank-three certificate.
The three individual-ramification pole spaces $H+2B_1$, $H+2B_2$ and
$H+Q$ now yield no new old-base relation. The full-pole bounds are null;
Riemann--Roch reduces every incomplete new pole to the already bounded
canonical space. No extra simple-pole enumeration is needed.

The remaining degree-two divisor supported on the ramification set is
$B_1+B_2$: the complete list is $2B_1,B_1+B_2,2B_2,Q$.
Its pole class $H+B_1+B_2$ has target
$\kappa+P_1+P_2=2\kappa-T$. Preflight that target, freeze its protocol,
and test it without repeating the three completed searches. Reuse their
receipts in a joint elimination check. This fourth target is not tested
by the current run. Positive old-base gain still needs actual RR/KM and
separate elliptic/elimination replay. [pcc:unverified]

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
  [complete same-image conic family](FULL_FIBRE_CONIC_PROTOCOL.md),
  [mixed-sign pencils](MIXED_SIGN_CONIC_PROTOCOL.md) and
  [expanded balanced-image bound](EXPANDED_BALANCED_PROTOCOL.md),
  [mixed-image pencils](MIXED_IMAGE_CONIC_PROTOCOL.md),
  [old-pair extension](MIXED_IMAGE_EXTENSION_PROTOCOL.md),
  [four-line pencils](FOUR_LINE_CONIC_PROTOCOL.md),
  [tangent pencils](TANGENT_CONIC_PROTOCOL.md),
  [bitangent pencils](BITANGENT_CONIC_PROTOCOL.md),
  [higher-contact controls](HIGH_CONTACT_CONTROL_PROTOCOL.md),
  [actual higher-contact scan](HIGH_CONTACT_SCAN_PROTOCOL.md) and
  [full retained-image union](FULL_IMAGE_BOUND_PROTOCOL.md) and
  [reference-shifted six symbols](SHIFTED_SIX_PROTOCOL.md),
  [canonical-plus-exceptional six symbols](CANONICAL_SIX_PROTOCOL.md) and
  [exceptional tangent-conic images](EXCEPTIONAL_CONIC_SIX_PROTOCOL.md) and
  [three whole residual traces](THREE_TRACE_PROTOCOL.md) and
  [same-image closed residuals](CLOSED_RESIDUAL_PROTOCOL.md) and
  [actual ramification supports](RAMIFICATION_PROTOCOL.md) and
  [individual ramification poles](RAMIFICATION_POLE_PROTOCOL.md).
- Run wrappers: run-fibres.sh, run-sections.sh, run-audit.sh, run-norms.sh,
  run-trace.sh, run-conics.sh, run-fibre-conics.sh, run-six.sh,run-partial.sh,
  run-audit-partial.sh, run-graph.sh, run-mixed.sh, run-divided.sh and
  run-mixed-divided.sh,run-anchors.sh,run-anchors-expanded.sh,
  run-expanded-conics.sh,run-all-expanded-conics.sh,run-mixed-sign-conics.sh
  and run-expanded-balanced.sh,run-mixed-image-conics.sh,run-full-image-bound.sh,run-mixed-image-extension.sh
  and run-four-line-conics.sh,run-tangent-conics.sh,run-bitangent-conics.sh,run-high-contact-controls.sh and
  run-high-contact-scan.sh, run-shifted-six.sh, run-canonical-six.sh and
  run-exceptional-conic-six.sh, run-three-trace-six.sh, run-closed-residual.sh, run-ramification.sh and run-ramification-poles.sh.
- Immutable attempt reports and diagrams are in reports/.
- Noncanonical reference source: ../km_pushforward_20261126/REPORT.md,
  its frozen pushforward/verify sources,and ../correspondence_audit_20261124/REPORT.md
  specify the degree-three reference;scoped construction-source reads found
  no established target-automorphism result.
- Retained differential source: ../composition_verification_20261126/REPORT.md,
  its frozen check.cpp:110--119, and the existing L record in backmap.json.
- Prior finite-domain results: ../recorded_pool_20261123/REPORT.md and
  ../closure_20261123/REPORT.md; the signed-pair helper is copied with provenance.
- Diem, *On the discrete logarithm problem in class groups of curves*,
  JTNB24 (2012),639--667, DOI10.5802/jtnb.815, canonical line and
  large-prime construction: [primary paper](https://www.numdam.org/item/10.5802/jtnb.815.pdf).
- Gaudry, Theriault and Thome, double large-prime work, graph recombination:
  [primary paper](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf).
- KB tools unavailable in this runtime; these primary sources and repository
  negative results were reviewed before the experiments. No novelty claim.

Canonical graph scope: parent map flow is unchanged because no map changed.
The index-calculus scoreboard and progress timeline receive no speed or
PCC promotion: this is producer evidence of finite closure nulls. The study-local rank graph records completed actual finite domains,including nulls.
The new control-isolation diagram and contact histogram copy native receipts;
the control-only preflight is excluded from the rank-case graph.
The degree-six flow diagram and its editable data copy the three native
terminal results; earlier parent-map and scoreboard figures remain unchanged.

The signed three-trace diagram combines the new native target3kappa receipt
with the earlier native targetkappa receipt; its sign reduction is derived
algebraically. Other parent-map and scoreboard graphs remain unchanged.

The closed-residual diagram copies new native degree-five/six counts and
identifies the reused low-degree bound; other completed figures remain unchanged.

The ramification diagram and its editable values copy the new native support,
local-index and trace/rank records. Other parent-map and scoreboard graphs
remain unchanged.

The individual-ramification pole diagram copies all three new native target
family results and their joint ranks; its partial-pole reduction is the
written Riemann--Roch argument. Other parent-map and scoreboard figures
remain unchanged.
