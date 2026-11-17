This research branch adds arbitrary-public-target hyperelliptic index-calculus controls on the **initial mixed-term genus-2, genus-3 and genus-4 benchmark families**, with exact correctness and complete operation-count records. The requested 10× mathematical speedup is **not established**.

The generic path now includes a weighted two-large-prime walk, a Riemann–Roch pencil collector, and an exact leading-coefficient formula that extracts the remaining quadratic without constructing the full norm polynomial. Source/target coefficients, signs, repeated roots, and weighted cycles that leave one large-prime variable are retained. Every returned scalar is checked against the original public source and target. The public solver API receives no expected scalar, point-log table, or hidden cofactor.

A conservative Hasse/order check restricts this control to a unique cyclic prime-order component. Generic whole-field root tables and finite-difference catalogue construction are offered **equally to all six solver arms**, built fresh inside every call. The table variant is bounded to p≤65537; it is not an arbitrary-size field algorithm.

Validation and evidence:

- All 11 initial curve/subgroup identities are retained, with 22 fixed public source/target fixtures. The general pencil completes all 22; genus four needs 145–268 anchors under the prospectively frozen 512-anchor limit.
- The pencil correctness gate checks 14,089 queried norm/root products, 6,742 native principal relations, repeated roots and anchor overlaps, plus dense weighted-elimination prefixes. The cheap-tail gate exactly reproduces every queried quotient and all final reports.
- Root/catalogue/oracle qualification matches legacy semantics on all initial catalogues, all field residues, 19,687 exhaustive small-field monic polynomials and 5,632 additional seeded polynomials.
- The latest shared-preparation gate completes 264 fresh calls across Fast/Dense/Hybrid full-base, 2LP walk, full-norm pencil and tail pencil. Prior outcome streams and subgroup-field vectors replay exactly; repeat vectors and lookup counts match. All setup and internal final verification are included under the recorded valid-public-instance contract.

Against the lowest relevant counts among shared-prepared walk/full-base controls, tail-pencil base-field REDC ratios are 1.40–2.58× at genus two, 1.81–4.34× at genus three and 1.09–2.39× at genus four. Subgroup-field ratios are reported separately. One genus-four subgroup regression remains. These are field-count results, **not wall-time gains, a mixed-field cost estimate, or a 10× result**. Root lookups, field additions, integer work and memory remain explicit; independent review is pending (`[pcc:unverified]`). The methods reproduce classical 2LP/pencil/Vieta machinery rather than claim mathematical novelty.

Review entry points:

- [Initial-instance applicability and scope](research/hec_residual_span_20261116/native-scope-audit/RESULTS.md)
- [Arbitrary-target weighted 2LP API and correctness](research/hec_residual_span_20261116/native-public-2lp/RESULTS.md)
- [General pencil identities and coverage](research/hec_residual_span_20261116/native-pencil-2lp/RESULTS.md)
- [Quadratic-tail derivation](research/hec_residual_span_20261116/native-pencil-2lp/QUADRATIC-TAIL.md) and [exact implementation gate](research/hec_residual_span_20261116/native-pencil-tail/RESULTS.md)
- [Prepared oracle qualification](research/hec_residual_span_20261116/native-prepared-oracle/RESULTS.md)
- [Latest fair all-six-arm count results](research/hec_residual_span_20261116/native-prepared-solvers/RESULTS.md)
- [Working journal and retained failures](research/hec_residual_span_20261116/SESSION.md)

The branch also retains the earlier **special binomial-family** automorphism and CM-quotient investigations. Their “original targets” mean the targets within those frozen special-family panels, not the initial mixed-term benchmarks. No checked transport makes those results apply to the initial families. Keeping exact polynomial and subgroup identity is mandatory; matching genus/field/constant alone is insufficient.

Protocols, source/binary/input hashes, raw outputs and failure records are in Git while S3 is unavailable. No S3 upload or external independent admission is claimed. Keep the PR open for review; no merge or auto-merge is authorized. Remaining research must demonstrate a genuinely stronger complete-cost mechanism and qualified runtime evidence across the intended scope before the full goal can be marked achieved.

Cubic-tail follow-up: [correctness](research/hec_residual_span_20261116/native-pencil-3lp/RESULTS.md) qualifies all22 public fixtures, but [complete field costs](research/hec_residual_span_20261116/native-pencil-3lp-cost/RESULTS.md) regress against quadratic tails on every genus2/3 fixture. Genus4 has one isolated improving fixture. Retain this negative result; it does not establish a general gain or10x result.

Shared quadratic residue images: [qualification and full field counts](research/hec_residual_span_20261116/native-pencil-images/RESULTS.md) preserve all22publicfixture outcomes/subgroupvectors and reduce completebaseREDC by1.080–1.304x(g2),1.024–1.053x(g3),1.005–1.013x(g4). This is a classical circuit optimization, not elapsed-time/10x/newmathematical evidence.

Deferred target membership: [proof, exact cost identity and negative controls](research/hec_residual_span_20261116/native-deferred-membership/RESULTS.md) retains source-order validation and native final equality, omits only preflight ellT. All44successful calls replay;16outside-subgroup targets rejected at final check,6controls unavailable. Basefield counts improve1.056–1.278x depending fixture/genus; subgroup unchanged. Failure-path latency changes explicitly; no10x/runtime/newattack claim.

Generic incremental target recovery: [prefix qualification and mixed complete costs](research/hec_residual_span_20261116/native-stream-target/RESULTS.md) verifies every completed-row target pivot against nativeFast. Earlier stopping lowers some collection costs, but subgroup counts regress on all6genus3fixtures. Retain both buffered and streaming controls; no uniform replacement or10xclaim.

Generic weighted affine forest: [qualification and complete field counts](research/hec_residual_span_20261116/native-affine-forest/RESULTS.md) reduces subgroupREDC onall22fixtures against bothstreaming/buffered controls. Stream/forestratiosg21.486–3.203,g31.301–1.612,g41.412–1.620; baseFULL16unchangedfromstream. Classicalsparsepayloaddata structure, notfullylazyDAG, walltimeor10xclaim.

Grounded-point feedback: [qualification and negative/mixed complete costs](research/hec_residual_span_20261116/native-grounded-harvest/RESULTS.md) verifies genericaffinepromotion withoutnewunknowns; all-groundedscan policy regresses all12genus2 subgrouprecords and mosthighergenus subgrouprecords. Genus4base improves1.047–1.422x. Retainstaticforestcontrol andallregressions; no generalgain/10xclaim.

Publicsource/targetconstraints: [chargedqualification/control](research/hec_residual_span_20261116/native-public-constraints/RESULTS.md) inserts24of44attemptedinputdecompositions, retains20nonsplitcases; all22solvesand16negativerejections pass. Completefieldcosteffects small/mixed (largestsubgroupgain1.205x), notuniformreplacement/10xresult. Keepthiscontrolandstaticforest.

First-rho1quotientdescriptors: [all22rank2classification/componentcost](research/hec_residual_span_20261116/native-quotient-descriptor/RESULTS.md) verifiesparticular/nullidentitiesagainsteverypriorrawrow andexplicitgroundedminorwitnesses. Fullprincipalrelationexistencefollowsunderdereivedsaturationargument; low-degree/coldcostnotestablished. Descriptorconstructioncostsareexplicit, notacompletesolveor10xresult.

Fixedlowdegreegroundedfamily: [structuralall22finishingcoverage](research/hec_residual_span_20261116/native-grounded-span/RESULTS.md) executes2816prescribedtrials, finds414nonzero-signaturerelations andfinishesall22savedrho1stateswithin22attempts. Allnorm/split/nativechecks pass; cheapbranchcontrolalsofinishes5fixtures. Prefix/descriptorsprovidedfrompriorrecords, so nocoldcost/10xclaim.

Integratedlazyquotientfinisher: [completecostqualification/negative result](research/hec_residual_span_20261116/native-lazy-finisher/RESULTS.md) includesallfreshbootstrap/setup/search/verification,132countedcalls/66positiveaudits/32negativeaudits. All22finish,butsubgroupcountsregressagainsteverypublic+branch-normalizedcontrol; onechangedbootstrapneeds96trials(fixed128bound). Retainconstructiveproof/negativecostevidence; no10x/runtime/newmathclaim.

Originalmixedterm2-kernelcensus: [all11sources/completeisotropicenumeration](research/hec_residual_span_20261116/native-two-kernel/RESULTS.md) finds1rationaltractablekerneloneachg3p61/211/401, g2p101/1009oneeach, g4p31eleven;other5sourceszero. Kerneladmissiononly—noquotient/trigonalrationality/nonhyperellipticity/transport/speedclaim. Smithrouteispriorart; fullgenus2+objectiveopen.
