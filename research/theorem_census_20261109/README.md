The new [160-bit point certificate](../anomalous_curve_audit_20261110/README.md)
proves the actual prime, bit bounds, nonsingularity, exact prime order of an
actual Mathlib point, and its fixed known-answer scalar relation. Its source
control reproduces the author's published erratum for an omitted coefficient
digit; the corrected source is not refuted. The complete gate passes 59 public
theorems with zero failing or trusted-native exceptions, and the full local
packet check passes. Subsequent linked studies certify the finite Smart execution and whole-curve
cardinality. The finite repository rho measurement is now recorded in
[the count audit](../rho_constant_audit_20261112/README.md); generic transfer,
complexity, native refinement and limiting-law obligations remain separate. [pcc:unverified]

The new [original F4 model audit](F4_MODEL.md) proves ideal containment for
every actual original F4Run and directly refutes its unrestricted extension
to saturation outputs. Exact imported original modules and literal F101
expected rows are checked; the whole gate passes 573 theorems with zero
failures or trusted-native exceptions. Native failures remain explicit:
F101 saturation is unsupported; F65537 ordinary crashes and saturation times
out. V1's sample-count registration error remains frozen and disclosed;
v2's complete export passes structural checks with no successful solver row.
Ordinary C refinement and the crash/hang diagnosis remain open.
Signed standing is recorded separately. [pcc:unverified]

The new [Diem domain proof](DIEM_DOMAIN.md) checks the arithmetic last
step under n≥2, 2^(7n)≤q and the explicit geometric lower estimate. An
actual F4=F2[alpha] curve y²+y=x³+alpha has discriminant1, actual point
count1 and an empty factor base for the checked Möbius chart alpha+1/x.
Lean directly refutes the unrestricted chart/count bound. This q2/n2
control lies outside the source size hypothesis; actual scheme degree,
ramification/admissibility and genus/Hasse–Weil bridges remain open.
The qualified source bound is not refuted. Whole census gate535passes,
zero failures/zero trusted-native exceptions. Full local packetdda065passes;
signed standing remains unverified. [pcc:unverified]

The new [finite-extension Frobenius bridge](FROBENIUS_FINITE_EXTENSION.md)
proves that actual points over a chosen finite extension K/F are exactly
those fixed by [K:F] iterations of actual base-field point Frobenius.
It checks the actual cardinality/degree relation and the equivalent
|K|=|F|^k formulation, including the entire actual r-torsion set. All
embeddings and scalar towers are explicit. Qualified BK still needs
actual geometric torsion rank, determinant and Weil necessity. Whole census
gate490passes, zero failures/zero trusted-native exceptions. Full local
packeta496b9passes; signed standing remains unverified. [pcc:unverified]

The new [actual Frobenius descent proof](FROBENIUS_DESCENT.md) checks,
over every finite base field and every extension, that actual curve points
are Frobenius-fixed iff they lie in the base-change image of rational
points. It proves actual torsion stability and the rational-torsion iff
identity-on-torsion criterion. The nonsingular F7/r3excluded-case witness
now gives actual nonfixed3-torsion despite9|#E. Qualified BK still needs
geometric torsion rank/determinant and Weil necessity; the later finite-extension
component now closes the chosen-extension iterate/point-image bridge. Whole census gate479passes, zero failures/zero
trusted-native exceptions. Full local packetf1abe2passes; signed standing
remains unverified. [pcc:unverified]

The new [all-arity geometric proof](SEMAEV_GEOMETRY.md) closes the actual
canonical polynomial-to-point bridge: over every algebraically closed field,
for a general Weierstrass curve with nonzero discriminant and every arity
m≥3, canonical evaluation is zero iff actual curve points with the prescribed
affine x-coordinates sum to zero. Actual polynomial evaluation, fixed-size
resultant leading-zero cases, point-list gluing, S3/S4 bases and strong
induction are all checked. A direct Lean disproof on the nonsingular F5
curve refutes the rational-only strengthening. Full polynomial permutation
invariance and irreducibility remain separate obligations. Whole census gate471passes,
zero failures/zero trusted-native exceptions. Full local packetaf2329passes; signed
standing remains unverified. [pcc:unverified]

The new [fixed-size root criterion](DECLARED_ROOTS.md) proves the exact
common-finite-root OR both-leading-zero alternative over algebraically
closed fields with positive declared sizes and degree bounds. Lean refutes
the affine-only version using padded constants, and removal of closure
using X²+1overQ. Latest whole census gate431passes, zero failures/zero
trusted-native exceptions. This is the polynomial bridge used by the later all-arity geometric proof. [pcc:unverified]

The new [all-input degree proof](ALL_INPUT_DEGREES.md) checks sharp
individual degree2^(m−2)for every formal input, every aritym≥3and every
field in the actual canonical resultant family. Explicit variable
permutations identify each input; a Lean finite-position case split covers
all positions. The all-input-stage whole census gate passes423declarations with
zero failures and zero trusted-native exceptions. The later geometric proof closes higher recursion; full polynomial symmetry
and irreducibility remain open; no degree preservation after field
evaluation is claimed. [pcc:unverified]

The new [surviving-input degree proof](RECURSIVE_FOCUS.md) gives an exact
old-variable degree2d and leading-coefficient formula for the actual
fixed-size resultant. The actual canonical third-newest input now has
degree2^(m−2)at every aritym≥5, extending all-arity coverage to the three
newest inputs. The later all-input proof closes the remaining older-input degree component.
The focus-stage whole census gate357passes, zero failures/zero trusted-native.
[pcc:unverified]

The new [trace-class consequence](SUPERSINGULAR_TRACE.md) proves modular
order at most6, coprimality and an actual prime-order point for actual
finite-field curves, conditional on an explicit trace-square classification.
Its composite-divisor control is arithmetic only. The trace-stage whole census
gate passes331declarations, zero failures/zero trusted-native exceptions.
Original Waterhouse4.1pp536–537has now been read and visually inspected;
classification and the embedding-field bridge remain formal gaps. [pcc:unverified]

The [newest-pair symmetry proof](RECURSIVE_SYMMETRY.md) now shows that
every actual recursive resultant extension is invariant under exchanging
its two newest independent variables. Both input degrees are 2^(m−2)
for every recursive arity m≥4. Complete census gate: 320 declarations,
zero failures or trusted-native exceptions. Full permutation invariance remains open; the later geometric proof closes higher
recursion and the all-input component closes formal degree coverage. [pcc:unverified]

# Skeptical Lean refutations and exact Semaev census

Lean4.35.0 now proves the full S3equivalence, first S4resultant step,
S4individual symbolic degree4in every input, a general fixed-size resultant
degree step and last-input degree2^(m−2)at every aritym≥3for the explicitly
defined canonical recursive family, for general Weierstrass models
in every characteristic, with all repeated-coordinate cases and actual
Mathlib point sums. It also checks model-domain counterexamples over F2/F3,
geometric p-torsion families, actual torsion-boundary witnesses, a direct r=2
source correction in FST2010, the rational-point strengthening of Semaev's
converse, and nonregularity of a genuine F25 symmetrized descent. The proofs
use actual elliptic point groups and intermediate fields. The qualified BK
theorem, Semaev's algebraic-closure theorem, and generic FGHR regularity
are not refuted; their exact domains and remaining bridges stay explicit.
[pcc:unverified]

The torsion-only proof snapshot audited74declarations with0failing and
0trusted-native exceptions. The current recursive-family follow-up
audits308declarations; the original41-declaration and intermediate96/116/134/167/197/205/259/291-declaration receipts are
retained as earlier snapshots. Counts include generated equation lemmas,
not308new mathematical results. Both proof attempts and final receipts are
retained. The torsion snapshot's unsigned PCC records are `PCC-20261109-44730b` and
`PCC-20261109-5bd3a4`. Earlier packets omitted required declaration hashes;
they remain as superseded records, and their failed metadata check is
preserved. The Lean kernel audit itself passed. [pcc:unverified]

CI subsequently found that superseded packets still participate in
`verify-all`. Their missing hashes were repaired in the active unsigned
packets with append-only correction events; exact original bytes are retained
under evidence/metadata-originals/. The mathematical statements were not
changed. See evidence/metadata-binding-amendment.json. [pcc:unverified]

The follow-up adds actual BK excluded-boundary torsion and a direct FST2010
r=2 source correction. For Algorithm3.3(1)'s E/F7:y²=x³+x, Lean establishes
#E=8, the source supersingular trace condition, a rational point of order2,
and nonrational geometric2-torsion; nevertheless the **actual field**
F7(μ2)/F7 has degree1. This contradicts the literal degree2 sentence on
p239 under Definition2.1. The primary page was visually checked.
[pcc:unverified]

See [PRIMARY_AUDIT.md](PRIMARY_AUDIT.md) for the exact statements and proof
bridges, [the torsion report](reports/proof-20261109-torsion/report.pdf), and
[pcc-torsion-records.json](pcc-torsion-records.json) for unsigned claims.
The entire44-unread-label backlog and all239source records, including170
primary-publication records, are tracked in [UNREAD_BACKLOG.md](UNREAD_BACKLOG.md)
and [paper-audit-backlog.json](paper-audit-backlog.json). Those are metadata
counts; untouched papers are not labelled audited. A later additive full-primary read
receipt for S-SEMAEV-2004 is in
[the original-source study](../semaev_source_audit_20261110/coverage.json);
the historical snapshot is unchanged. The new original Verheul statement/formula read and actual Lean correction have additive [coverage](../verheul_distortion_audit_20261114/coverage.json); general Theorem5 remains open and no full-paper-read promotion is made. [pcc:unverified]

## Exact counterexamples

The subsequent [Mumford counting study](../jacobian_smoothness_audit_20261109/README.md)
proves completeness and counts in the actual reduced polynomial-pair model
for F3y²=x⁵−x:8valid,7with split u, fraction7/8and finite difference3/8from1/2.
Its initial80-declaration gate and full local packet07eb0dcheck pass, unsigned.
The subsequent170-declaration gate now checks actual Cantor additions on
all64pairs and a proper order-two cyclic subgroup with smooth fraction1
versus7/8ambient, directly disproving universal finite equality in this
arithmetic model. See [Cantor scope](../jacobian_smoothness_audit_20261109/CANTOR_SUBGROUP.md).
The subsequent F5control proves16actual pairs/all16split before native
calibration; current separate project gate240passes0failing/0trusted-native.
Its full local packete81718check passes, unsigned. The frozen calibration
protocol is not yet executed at this writing. Geometric Picard identification, scaling families and the native
multi-field census remain open. This finite deviation does not refute
Gaudry's qualified fixed-genus limit. [pcc:unverified]

The canonical family is now explicitly defined over recursively extended
independent polynomial coefficient rings. Lean proves its last-input
degree2^(m−2)for every aritym≥3, successor leading-coefficient square,
actual base S3evaluation and an exact bridge to canonical S4. See
[RECURSIVE_SEMAEV_DEGREE.md](RECURSIVE_SEMAEV_DEGREE.md),
[source](lean/RecursiveSemaevDegree.lean) and
[report](reports/proof-20261109-recursive-semaev-degree/report.pdf).
Packet3bc113and fixedD8scalar supplementIDC1h0872017c01480c3dremain unsigned.
The later all-input component proves every formal input degree. Full
permutation symmetry, geometric zero-sum equivalence beyondS4and
irreducibility remain open. [pcc:unverified]

The actual fixed-size resultant step now has a general coefficient/degree
proof for every declared size D. Its leading coefficient is f(Z)², and its
last-variable degree is2Dover a domain when f≠0and natDegree(f)≤D.
Resetting Dafter specializing fto1changes the resultant: declared sizes2and0
give degrees4and0. The actual canonical S5has last-input degree8with other
inputs formal. See [RESULTANT_DEGREE_STEP.md](RESULTANT_DEGREE_STEP.md),
[source](lean/ResultantDegreeStep.lean) and
[report](reports/proof-20261109-resultant-degree-step/report.pdf).
Packet621d56and fixedD4scalar supplementIDC1h7dd754d70e343323remain unsigned;
the later recursive-family component extends the last-input degree result
to every arity, while full permutation symmetry and irreducibility remain open.
[pcc:unverified]

The canonical S4polynomial has symbolic degree4in every input position,
with the remaining inputs independent formal variables. For fixed other
coordinates, the fourth coefficient is S3²and the degree is4exactly when
S3≠0. On actual smooth F5:y²=x³−x, S4(0,1,4,T)=1ofdegree0. Thus arbitrary
specialization does not preserve the generic degree. These are actual
polynomial-ring Sylvester determinants, not finite-field interpolation.
See [S4_DEGREE.md](S4_DEGREE.md), [source](lean/S4Degree.lean), and
[report](reports/proof-20261109-s4-degree/report.pdf). Packetb93977remains
unsigned; the remaining input degrees beyondS4and irreducibility remain open.
[pcc:unverified]

Composite subgroup orders do not break the conditional prime-field
arithmetic: the corrected modular-order theorem now applies to every
integer r>2dividing the actual point count, retaining Hasse explicitly.
On actual E/F7:y²=x³+x, P=(3,3)has order8and modular embedding order2.
The r2counterexample and missing Hasse/geometric/field-degree bridges
remain separate. See [COMPOSITE_TORSION.md](COMPOSITE_TORSION.md),
[Lean source](lean/CompositeTorsion.lean), and
[report](reports/proof-20261109-composite/report.pdf). Packet10789dis
unsigned. [pcc:unverified]

The S4follow-up proves the actual fixed degree2/2Sylvester-resultant
zero-sum equivalence for four points, including partial sums at infinity.
Its smooth F5curve y²=x³−x gives S3(0,0,t)=1andS3(1,1,t)=4for every
geometric t, yet the fixed-size resultant at(0,0,1,1)is0and an actual
zero-sum quartet exists. This refutes a finite-common-root shortcut,
not the qualified recursion. See [GENERAL_S4.md](GENERAL_S4.md),
[Lean proof](lean/GeneralS4.lean) and
[proof report](reports/proof-20261109-general-s4/report.pdf).
Unsigned packets21757f/ce6722and supplementIDC1hbf1f44625ee306dcbind this
scope; general m/irreducibility/degree proofs remain open. [pcc:unverified]

The general S3follow-up proves the algebraic-closure equivalence on the
actual general Weierstrass model without a characteristic restriction.
The ordinary binary expression gives a false geometric zero on
F2:y²+y=x³at(1,0,0); the short expression gives a false geometric zero on
F3:y²=x³+x²+1at(0,0,0). Both are smooth actual curves; qualified normal-form
formulas are not refuted. Direct geometric p-torsion families are checked
without assuming the trace/Hasse bridge. See
[GENERAL_S3.md](GENERAL_S3.md), [main Lean proof](lean/GeneralS3.lean),
[actual controls](lean/GeneralS3Controls.lean), and
[two-page proof report](reports/proof-20261109-general-s3/report.pdf).
Unsigned packets4d5cd7/b32b34and polynomial rewrite
IDC1h392e1740fcb7cf05bind the stated scopes. [pcc:unverified]

On the nonsingular F2model y²+xy=x³+1, Lean proves #E(F2)=4,
`#E[2](F2bar)=2`, and all geometric2-torsion rational, yet2does not divide1.
The original BK source requires r²torsion points, so a set-inclusion
translation also needs a characteristic/rank qualification, which the actual
corpus node already retains as gcd(r,q)=1. This is a negative control of
that exclusion. Every actual
μ2-valued map on this nontrivial geometric torsion group is pointwise
degenerate, because the actual μ2value type has1element. This refutes
unrestricted pointwise nondegeneracy, not group-scheme Weil duality or
qualified pairing constructions. Packet92d64dremains unsigned. See
[CHARACTERISTIC_TORSION.md](CHARACTERISTIC_TORSION.md),
[actual Lean objects](lean/CharacteristicTorsionBoundary.lean) and
[proof report](reports/proof-20261109-characteristic/report.pdf).
[pcc:unverified]

The binary follow-up proves the full ordinary characteristic-two S3
equivalence over an algebraically closed field, using actual Mathlib elliptic
points and all repeated-coordinate/two-torsion cases. It also checks a domain
negative control overF2: a true zero-sum triple has binaryS3=0 while the
short-Weierstrass expression is1. This refutes substituting a formula on the
wrong model. Claim PCC-20261109-517cb7 remains unsigned. See
[BINARY_S3.md](BINARY_S3.md), [Lean proof](lean/BinaryS3.lean),
[actual counterexample](lean/BinaryMisapplication.lean), and
[proof report](reports/proof-20261109-binary/report.pdf). [pcc:unverified]

The solver-control follow-up proves the actual algebra equivalence
K[x,y]/(xy,x+y−c) ≃ K[t]/(t²−ct), length2, and a closed F5support1/length2
witness. All16preregistered msolve0.6.5 calls pass their exact known-answer
checks, including the prior degree3control. The expanded project gate now
audits96declarations with0failing/0trusted-native exceptions. Claim
PCC-20261109-ee57d0 is unsigned. See
[solver multiplicity study](../solver_multiplicity_audit_20261109/README.md)
for the frozen protocol, raw outputs and report. [pcc:unverified]

The retained msolve preflight also announces k^344 but prints an integer
with a different F5residue for k=2. Lean proves the unequal residues and
out-of-int32-range arithmetic; pinned source casts floating pow before
field reduction. This source/log audit is separate from a formal C/libm
execution proof. Claim PCC-20261109-c7c353 is unsigned. See
[source observations](../solver_multiplicity_audit_20261109/SOURCE_OBSERVATIONS.md).
[pcc:unverified]

1. On `y²=x³+1` over F5, `S3(3,1,4)=0`, but there is no F5-point above x=3.
   `rational_only_s3_converse_false` uses Mathlib's actual elliptic point
   group. The curve is nonsingular. The field qualification matters.
2. Let `K=F5[t]/(t²−2)` and `E:y²=x³+x+1`. Lean establishes irreducibility,
   `[K:F5]=2`, `#K=25`, nonsingularity, and the actual base-changed target
   `(2,1)≠O`, with `3(2,1)=O`. For `e1=x+y`, `e2=xy`, the symmetrized S3
   equation is

   `f=(e1²−4e2)4−4(e1(e2+1)+2)+(e2−1)²−4e1`.

   Descent by actual monic-remainder coordinates gives `[f,0]`. Lean proves
   nonregularity both of this sequence and of its top homogeneous parts.
   `universal_descent_regularity_counterexample` bundles the concrete input
   validity and failure. [pcc:unverified]

FGHR2014 Hypothesis3 says that systems from symmetric-coordinate descent
are regular, without an explicit generic-input qualification in that sentence.
Our witness refutes the **universal square-system reading**. Definition5
requires nonzero input equations; §3 also restricts the complexity analysis
to zero-dimensional square systems. The witness violates those conditions,
so it does not contradict the conditional complexity theorem. Discarding the
zero equation changes the square presentation, leaving one equation in two
variables. No claim about generic inputs, random sampling, Joux–Vitse
Assumption2, or the generic ideal-degree claim follows. [pcc:unverified]

This witness uses the short-Weierstrass representation. It does not establish
an Edwards/Jacobi-model counterexample to a representation-restricted
interpretation of Hypothesis3. The universal reading and those narrower
domains must be kept distinct. [pcc:unverified]

A separate [13 November Edwards study](../edwards_descent_audit_20261113/README.md)
now supplies an actual F25 twisted-Edwards counterexample with both
nonzero quadratic descent equations and an Artinian affine quotient of
F5-dimension two. Their highest homogeneous parts coincide and fail
Mathlib regularity. Its actual factor-base points add to the target in a
Mathlib elliptic point group. This removes the zero-equation,
positive-dimension and representation limitations of the older witness
above for the literal universal n = 2 reading. Generic inputs, n ≥ 3,
conditional Theorem4.1 and Joux–Vitse Assumption2 remain separate.
Whole separate gate55/0failures/0trusted. [pcc:unverified]

The formal bridge includes `symmetrization_over_extension`, the actual
coefficient descent (not an invented list), and Mathlib's
`RingTheory.Sequence.IsRegular`. No custom regularity axiom is used.
The identity record is `IDC1h819303b7fd181c8c`; its formal pointer names
`symmetrization_universal`, and the unchanged repository checker accepts its
32 deterministic evaluation points plus11 extra points. These evaluations
are auxiliary: the Lean proof establishes the polynomial identity.
[pcc:unverified]

## Finite census

The native C++ program checks both implications against actual elliptic point
sums in quadratic extensions. It checks full symbolic degrees, including
when degree exceeds field size; it does not estimate degrees by evaluation.
Every tested model and base-field coordinate tuple is included in the stated
domains. All discrepancy and degree-failure counters are zero.
[pcc:unverified]

| Base fields | Models | m | Exact scope |
|---|---|---|---|
| F5,F7,F11,F13,F17,F19 | Every nonsingular short model | 3 | Every ordered x tuple, y lifts in Fp² |
| F5,F7 | Every nonsingular short model | 4,5 | Full sparse S4/S5 resultants; every ordered x tuple |
| F2,F4 | Every ordinary model `y²+xy=x³+a2*x²+a6`, `a6≠0` | 3,4,5 | Every coefficient choice and tuple; y lifts in F4/F16 |

Raw output: [evidence/semaev-native.jsonl](evidence/semaev-native.jsonl).
Padded universal Sylvester formulas are checked against scalar determinants,
including all eight-coefficient choices over F2. Fixed F5 controls compare
S4/S5 term counts426/43267 and zero counts209/1282. These controls and the
census use this one machine; they are not independent-host validation.
Finite agreement does not prove the general recursion, degree theorem or
binary equivalence. [pcc:unverified]

The run had a before-run Markdown bound, but lacked a committed frozen JSON
preregistration. This reporting omission is retained as a protocol deviation.
`export.json` is an observational export: structural/file validation is
distinct from frozen-protocol acceptance. No before-data freeze or signed
scientific acceptance is asserted. No performance claim is made; clocks are
missing, and no paid cloud resources were used. [pcc:unverified]

## Reproduce

From this directory, with the pinned Lean toolchain and dependency cache:

```sh
cd lean
lake build
lake env lean Refutations.lean
```

From the repository root:

```sh
tools/lean/proof-gate.sh research/theorem_census_20261109/lean
g++ -O2 -Wall -Wextra -Werror -std=c++17 research/theorem_census_20261109/native/semaev.cpp -o /tmp/semaev-audit
/tmp/semaev-audit --controls
timeout 600 /tmp/semaev-audit
```

The exact native sources, binary digest, compiler identity, host observation,
failed Lean builds and both axiom-audit receipts are retained under
[evidence](evidence). The dependency on
`../theorem_audit_20261108/lean` pins the earlier checked actual-group S3 work.
The identity issuer is reproduced with `cargo run --release --locked` from
`identity/`; it imports the unchanged repository checker and SHA256 source.

## Original fourteen requirements

The [acknowledged Hildebrand cell-bound counter](../hildebrand_cell_audit_20261114/README.md)
refutes literaloriginal1994Lemma4 at k2/epsilon1/m2=4/a1=0/j1 for everyprime
p>=3. The literalReal.rpowcell endpoints/decimalbound reduce inLean to
1/p,5/(4p),11/(40p). Actualnonzerochoicecardp-1/fullconditionalinjectivesupport
equivalence/uniformPMFpushforward/sourceargument areproved. Actualcentered
representativecongruence/halfopenrange/uniqueness proves the celliffa2=-1;
its actualuniformprobability1/(p-1)>11/(40p). Lean directlynegates any
eventualprimebound usingEuclid, no cutoff. Independentvalidpositiveweight
k2mixingcontrol retained. Laterauthorpreprintmath/0401189v1section4.2
explicitlyhandlesk2separately/restrictscountinglemmatok>=3/acknowledgesolder
failureexercise; reference14p36 identifiesoriginal1994paper. Thisis aknown
boundaryformalization/no novelty, not a refutationofTheorems1/2orcorrected
k>=3countingbound. Original1994full13pageprimaryread/inspectioncredited;
laterpreprintselected13–16/referencep36read/visual15–16only. Authorcorrection
attachmentunreadafterrecordedfailures. Strongergeneralrandomsupportexponent
andother14gapsremainopen. ExactlocalPCC-20261114-4dc478PASSat2026-11-14T10:38:01Z, whole30/0failures/
0trusted, full1213characterbindinge378e39300818a69d943d83ce16522687b10d1131ccbf76cfa654c5a0fd12a22,
12custommodules/24formal-config+11artifactpins;54build/processing/provenance
files/25objects/9000Blosslessarchive/fivegatefiles/inspectedonepagePDF47123B.
Signedstandingunverified. No corpuspromotion/full14closure/nativeexperiment/
paidcompute/upload. [pcc:unverified]


The [general-weight cycle-coupling packet](../weighted_cycle_coupling_audit_20261114/README.md)
proves the qualified uniform-support quadratic Hildebrand1994Theorem2 for
any fixed finite k>=2/strictlypositive normalized weightvector: gamma is
positive and chosen before prime modulus/support; everyinjectivesupport,
start/event and integer t>=gamma*p² has strict eventdistance<epsilon.
Actualnormalizedweightvector→PMF/iidbind/process expectation, actualproduct
labelPMF=independentbinds/equal-mass specialpair transposition/both alltime
marginalexpectations/coalescence/normalizedmodulardifference/potentialdrift
2w_aw_b/uniformstationarity areproved. Strongerquantitativebound
p²/(8w_aw_bt)needs only two positiveweights at distinctincrements; gamma
1/(4w_aw_bepsilon) works. Prior fairantitheticpacket retains sharperconstant
p²/(4t) versus genericproduct p²/(2t) at halfweights; scopegainonly.
ActualZMod4/injectiveincrements0/2/bothhalfweights confined to actual
card2event{0,2}: distance1/2foralltime/directnoquadraticconstant after
prime-deletion. Sourcep192alreadywarnscompositesubgroups; no noveltyclaim.
FullHildebrand13pages191–203/proofs/discussion/referencesread/all13rendered
pages inspected; LPWcouplingselectedscopecredited/notfullbookread. This
preservesqualifiedTheorem2; strongerexpectedrandomsupportTheorem1exponent,
staticrho/collisionlaws and other14gapsremainopen. ExactlocalPCC-20261114-c938e8
PASSat2026-11-14T10:13:48Z, whole59/0failures/0trusted, full1948characterbinding
6392c4699f5b7f881febf34a2c29b4bc3ec709fca57e48741aae00f803125fe9,
ninecustommodules/18formal-config+11artifactpins; all102build/processing/
provenancefiles/35objects/20486Blosslessarchive/fivegatefiles/inspected
onepagePDF47254B. Signedstandingunverified. Historical44-unreadlabelsnapshot
notpromoted; possibleLemma4k2boundarycandidate notprovedinthispacket.
No nativeexperiment/paidcompute/upload/full14closure. [pcc:unverified]


The [independent cycle-coupling packet](../cycle_coupling_audit_20261114/README.md)
proves every-event distance to uniform<=n²/(4t) for the actual independent
half-stay/half-add-one PMF on everyZModn, n>=2/t>0, with no cutoff. Actual
fairBool map/bind expectation, concrete coalescent pair map, both marginal
expectations, exact potential drift and actual uniform stationarity are
proved. Everyprimefield/two distinct equalweight increments inherits the
bound by exact time-dependent affine coordinates/permutation, not a fixed
kernel conjugacy. Actual deterministic two-cycle walk has eventdistance1/2
for everytime/directeventual-mixing-negation; timezero extension is directly
refuted. Selected Hildebrand1994 Theorems1–2/proof2 and credited LPW
coupling/potential sources read; no fullsource/noveltyclaim. This is a
quantitative k2/equalweight component of qualified Hildebrand, not a
refutation. General unequalweights/k>=3/randomsupport asymptotics,
staticrho mixing/random-table collision penalty/constants remainopen.
Exactlocal PCC-20261114-13f615 PASSat2026-11-14T09:48:10Z, whole51/0failures/0trusted,
full1270characterbindingdbd609ef808defc71765389c4746628d85d20739fc39bec056a270e1855631f0,
fivecustommodules/11formal-config+11artifactpins; all92build/processing/
provenancefiles/33objects/18181Blosslessarchive/fivegatefiles/inspected
onepagePDF46941B. Signedstandingunverified. No corpuspromotion/full14closure/
nativeexperiment/paidcompute/upload. [pcc:unverified]


The [continuous integer-cocycle packet](../continuous_cocycle_audit_20261114/README.md)
refutes the necessary evaluation-surjectivity in literal Bruin2011p325Eq2.1
(authorpreprintp3Eq1) for arbitrary discrete Gk-modules. The actual Krull
absoluteGalF2 is compact; actual Frobenius is the square AlgEquiv with
its x² formula proved. The actual discrete integer action is trivial,
with identity/composition/jointcontinuity verified. Every continuousmap
satisfying its standard one-cocycle identity vanishes; bundling them as
continuoushoms loses no cocycles. The actual sigma-1 coinvariant quotient
has nonidentity class represented by integer1. Evaluation misses it at
every actual Galois element and specifically actual Frobenius. Lean directly
rules out any evaluation-derived bijection on any surjective cocycle-class
quotient. No homemade H1 object or supplied cohomology value is substituted.
Integersnontorsion/noncompactdiscreteZidentityhom controls pass. Primary
publishedmodule/actionparagraph and broadEq2.1 rechecked, priorfullnote
read/7pageinspection credited; selectedauthorpreprintpage3 rendered/fully
inspected, samewording. No novelty/globalerrataabsenceclaim. The following
finiteLemma2.1/application/qualifiedTateFR theorem remainsintact; corrected
finite/torsionH1identification, Lang/Cartier/canonicaldivisor bridges and
other14gaps remainopen. Exactlocal PCC-20261114-2fd770 PASSat2026-11-14T09:15:14Z, whole16/
0failures/0trusted, full803character binding,117formal/config+11artifactpins;
all73build/processing/provenancefiles/29objects/39289Blosslessarchive,
fivegatefiles/inspectedone-pagePDF47103B. Signedstandingunverified.
OriginalFR94/Serre citedtheorem notmarkedread/refuted. No corpuspromotion/
full14closure/nativeexperiment/paidcompute/upload. [pcc:unverified]


The [finite Tate-character packet](../finite_tate_duality_audit_20261114/README.md)
proves the algebraic finite character/coinvariant core used in Bruin2011Lemma2.1.
For every finite field K/q, finite abelian G killed by q−1 and automorphismσ,
actual invariant K*-characters pair perfectly with actual G/range(σ−1):
both evaluation MulEquivs, their formulas, equal cardinalities and both radicals
are proved. Field roots are derived from actual cyclic units and source
annihilation; existing Mathlib finiteabelian biduality is credited. No root,
basis, rank, supplied duality or ND premise. Every finiteoddfield actual
rational2torsion satisfies annihilation for any groupautomorphism. Actual
F3y²=x³−x has nonzeroDelta/P(0,0)order2/actual2tor4; identity characters4,
coinvariants4, units2 and perfectevaluation. Every finitechar2 credited actual
ordinary y²+xy=x³+1 has identitycoinvariants2/characters1/explicit nonidentity
radicalclass and sourceannihilationfalse. Lean directly negates the universal
rightND claim after deleting only annihilation. Qualified result retained.
Bruin2011 DOI10.5802/jtnb.764 allsixarticlepages323–328/proofs/referencesread,
allsevenPDFpages includingcover fullyrendered/inspected (649044B,
SHA528b63797f936e094861c98a65da5eb1615dc46747960ba3444516197fbf49a7).
OriginalFR94 remainsunread after additional403/503author/archive routes;
Bruin is a separate primary, not a replacement original reading. No actual
continuousH1/Lang/isogenydivisibility/Cartier/polarisation/canonicaldivisor
comparison bridge assumedorproved; no fullBruinLemma2.1/Tate/FR closure.
Exactlocal PCC-20261114-cdb4f1 PASSat2026-11-14T08:52:45Z, whole34/0failures/0trusted,
full1592-character binding,111formal/config+11artifactpins, all79build/
processing/provenancefiles/32objects/31093B losslessarchive/fivegatefiles/
inspectedone-pagePDF47031B. Signedstandingunverified. No novelty/corpuspromotion/
full14closure/nativeexperiment/paidcompute/upload. [pcc:unverified]


The [characteristic-order pairing-domain packet](../char_p_pairing_domain_audit_20261114/README.md)
proves every characteristic-p field has only identity p^k-th unit roots;
over every finite such field the power homomorphism is surjective and its
actual quotient has cardinality1. Actual finite E/nE and actual E[n] have
equal cardinality. Every finite ordinary characteristic-two general model
has actual left/right/target sizes2/2/1 and excludes unit-target candidates
over any characteristic-two field and candidates on proper Tate domains.
The credited actual y²+xy=x³+1 model has a rational order2 point over every
characteristic-two field. Every finite characteristic-three ordinary cubic
and nonsquare partner have a constructed nonzero3-point on one partner;
that partner has actual domain sizes3/3/1 and the same impossibility. The
credited F3 twist point(1,1) has exact order3. Prior actual torsion/group/
twist construction proofs are credited. Over F2(X), the actual X-unit has
nonidentity square class, target cardnot1 and squaremap nonsurjective;
unitroots of1 remain trivial. Lean directly negates a universal trivial
target after deleting finiteness. It also directly negates a prime-order/
actual-order-divisibility unit-pairing existence claim after deleting
coprimality, and separately an unqualified Tate claim omitting both
coprimality and root hypotheses. A proved prime-nonidentity-root=>coprime
lemma preserves the source guard: no counter to a retained-prime-root Tate
statement is claimed. Galbraithauthorch26Def26.2.2/Thm26.2.3/Ex26.3.1/
Thm26.3.3 selectedread, PDF2–5/printed574–577 fullyinspected. Qualified
sources/group-scheme perfect pairings notrefuted; canonical divisor/
valuation Weil/Tate/MOV/FR and generalEnd/Tatemodule remainopen.
Exactlocal PCC-20261114-f7faf2 PASSat2026-11-14T08:22:13Z, whole37/0failures/0trusted,
complete1986-character binding,117formal/config+11artifactpins; all113
build/provenancefiles/34objects/46310B lossless archive, fivegatefiles,
inspectedone-pagePDF46932B. Signedstandingunverified. KB/Conductor
unavailable, explicit bounded fallback/source-fidelity correction recorded.
No fullbookread/novelty/corpuspromotion/nativeexperiment/paidcompute/upload.
[pcc:unverified]


The [every-finite-characteristic-three packet](../finite_char_three_trace_audit_20261114/README.md)
proves the exact source gcd(actual trace,q) predicate iff three divides actual
trace iff actual point count is one modulo three iff b2=0 iff j=0 iff actual
geometric three-torsion has cardinality one iff actual ZMod3 rank is zero.
This holds for every finite characteristic-three field and every nonsingular
general Weierstrass model, with no field-size or extension-degree bound.
Actual complete-square group/Delta/card/b2 transport reduces to cubic models.
Finite cube-root surjectivity supplies an actual division-polynomial abscissa;
complementary twist fibers construct a nonzero three-point on one ordinary
partner. Actual Cauchy/Lagrange, twist-source invariance, and the actual
count sum prove both branches without rational-point/count/basis/rank/Hasse/
trace-classification/End-rank premises. The source predicate is invariant
under every finite field extension. Prior actual geometric j/module/group/
count proofs are credited. Together with the preceding char2 packet, this
closes the source-to-geometric p-torsion bridge in both characteristics2/3.
The ordinary F3 pair has actual counts5/3 and rational3torsion1/3; twist
P(1,1) has exact order3, and both source predicates are false. Lean rejects
replacing geometric by rational torsion. The source-SS F3 pair has counts7/1
and traces-3/+3, directly negating universal trace-zero strengthening.
Qualified source p>=5 is retained. Exactlocal PCC-20261114-3a8304 PASS at2026-11-14T08:01:36Z,
55declarations/0failures/0trusted, complete2121-character binding,
102formal/config+11artifactpins, all51attemptfiles/20objects
retained losslessly in27072Barchive,
fivegatefiles and inspectedone-pagePDF48037B.
Signedstandingunverified. FSTselectedpp230/237, Galbraithselected9.5.4
context and Milne selected geometricp-rank/Endrank distinction reviewed,
with prior primary-page inspections credited; no fullpaper/bookread or
canonicalcorpuspromotion. Generalp>=5trace-geometric, Endrank, general
Hasse/embedding/fulltorsionfield and other14gaps remain open. Further
explicit native extension censuses are not automatically discharged.
No newnativeexperiment/paidcompute/agentupload. [pcc:unverified]


The [actual quadratic-twist packet](../quadratic_twist_count_audit_20261114/README.md)
proves every finite odd field/nonsingular cubic y²=x³+a*x²+b*x+c/nonsquare
d scaled to(d*a,d²*b,d³*c) has actual point-count sum2(q+1), opposite
actual traces and equal source gcd(trace,q) predicates. Actual equation/
sigma-root fiber equivalences, complementary square-root count2, x-reindexing
and Delta scalingd6 supply the proof without nonisomorphism/Frobenius-trace/
point-count premise. Galbraith short-count Exercise9.5.4 and the explicit
cubic-with-x² extension are proved. Literal authorversion2Exercise9.5.3
nonisomorphism-if-nonsquare fails: F7y²=x³+x/j1728/nonsquare3 gives
y²=x³+2x with actual admissible C(u4), coordinate(4x,y)/inverse(2x,y),
actual AddEquiv/bothcounts8, and universal homogeneous/inverse identities
over every commutative ring of characteristic7. Lean directly negates the
universal no-admissible-change assertion. This special-j counterexample
retains separately qualified Lemma9.5.7's exceptions; no novelty or
previously-unreported-error claim. F3actual#5/d1 count10not8 directly
refutes deleting nonsquare; F5singularcusp#5/nonsquared2 count10not12
directly refutes deleting Delta. Prior actual models/counts credited.
Wholegate42/0failures/0trusted, exactlocal PCC-20261114-501186 replayPASS at2026-11-14T07:21:51Z,
full1423character binding,97formal/config+11artifactpinsmatch; all64build/
primary-processing/provenancefiles/25objects/36300Barchive/fivegatefiles,
inspectedone-pagePDF47600B. Signedstandingunverified. Authorchapter9
387389B/36pages/SHAf9a3272a… selectedTheorem9.3.4/Definition9.5.1/
Example9.5.2/Exercises9.5.3–9.5.4read, PDF7/printed171fullyinspected;
authorlanding/erratareadincludingadjacent9.5.8correction, no fullbookread/
corpuspromotion. MilneIV7char≠2,3restrictionretained. Generalchar3trace-
geometricbridge/Hasse/Endrank/embedding/fulltorsionfield/other14gapsopen.
No nativeexperiment/paidcompute/agentupload. [pcc:unverified]


The [all-finite-characteristic-two packet](../finite_char_two_trace_audit_20261114/README.md)
proves the exact source gcd(trace,q)!=1 criterion iff a1=0 iff j=0 iff
odd actual point count iff rational two-torsion cardinality1 iff geometric
two-torsion cardinality1, for every finite characteristic-two field and
every nonsingular general Weierstrass model. No field-size/degree bound.
Actual rational torsion count1/2 and induced ZMod2 rank0/1 depend on j;
rational/geometric torsion counts agree over every algebraically closed
scalar extension. The source predicate is invariant under every finite
field extension. The finite-field bridge uses official finite reduced-ring
squares to construct the actual ordinary two-point, Cauchy/Lagrange,
positiveq=2^n and coprime-power/parity arithmetic. No Hasse, trace-square,
Endrank, rank/basis or rational-torsion-existence premise. Prior actual
geometric j/module/Cauchy proofs credited. An actual F2(X) curve
 y²+xy=x³+X has DeltaX≠0/a1=1/j≠0/rational2tor1/geometric2tor2;
X is not a square by integer-degree parity, and Lean directly negates
ordinary rational descent after deleting finiteness. The credited F3
curve with actual#E5/rational3tor1/sourcepredicatefalse directly negates
changing characteristic two to three in the rational-source criterion.
These are hypothesis controls, not qualified-source refutations.
Wholegate26/0failures/0trusted, exactlocal PCC-20261114-8d2918 replayPASS at2026-11-14T06:48:42Z,
full1718character binding,91recursiveformal/config+11artifactpinsmatch;
all65build/provenancefiles/26objects/33868Barchive/fivegatefiles and
inspectedone-pagePDF47585B. One unsuppressed legacy/modern-module
compatibility advisory remains; failed migration/reversion retained,
no frozen migration/flags/waivers. Signedstandingunverified. Selected
FSTpp230/237 and MilneRemark3.16/contextpp98–99/p75notes reread,
priorPDF104/80inspectioncredited; no fullread/corpuspromotion/novelty.
General characteristic-three/other-prime trace-geometric and Endrank
identifications, Hasse/embedding bounds and remaining original14 gaps
stay open. No nativeexperiment/paidcompute/agentupload. [pcc:unverified]


The [actual prime-field trace packet](../prime_field_trace_audit_20261114/README.md)
proves actual Hasse and source gcd(trace,q)!=1 iff actual trace zero iff
actual point count6/8 for every nonsingular general Weierstrass model over
F5/F7. Explicit unit-u1 short normalization, unchanged discriminant and
credited actual additive point equivalence transport actual cardinality,
trace and source predicate. Closed kernel tables over all25/49short pairs
cover every general model; no general Hasse/classification premise.
Every actual source-SS count divisor is coprime toq/orderdiv2/exact2ifr>2,
including composites without a prime premise. Named F5 point(2,2) on
y²=x³+1 has exact order6, F7 point(3,3) on y²=x³+x has exact order8,
and both modular orders are2, while modulo2 the order is1. F5model is
credited to priorS3/r2controls. SingularF5cuspDelta0 has actual nonsingular
point-group cardinality5/raw equation count6, source predicate false/counted
predicate true; Lean directly negates the universal count bridge after
deleting nonsingularity. This is a hypothesis-control, not qualified-source
refutation. Whole gate29/0failures/0trusted, exact local PCC-20261114-e6d254 replayPASS
at2026-11-14T06:28:46Z, full1636character binding,90recursiveformal/config+12artifactpins
match; all56build/provenancefiles/21objects/32391Barchive/fivegatefiles,
inspectedone-pagePDF47387B. Initialsetup143incident is separately transcribed,
not claimed byte-exact replay. Signedstandingunverified. FSTselectedp230/237
andMilneII1.5/1.6/II2Theorem2.1printedp50/PDF55read/fullyinspected;
officialMathlibNormalForms/actualshiftEquiv credited. Sourcep>=5condition
retained, no full-paper/book reading, corpus promotion or novelty claim.
Generalprime/extensionfieldHasse/trace-geometric-Endrankidentifications,
field-degree/full-torsion-field bridges and remaining original14 obligations
stay open. No nativeexperiment/paidcompute/agentupload. [pcc:unverified]


The [all-model F2/F3 trace and geometry packet](../small_prime_trace_geometry_audit_20261114/README.md)
proves a complete kernel census of actual general Weierstrass coefficient
models:32/243 raw,16/162 nonsingular,8/54 source-supersingular. A generic
finite-field point equivalence proves actual point cardinality equals
literal equation-pair count plus identity, hence actual trace equals counted
trace. For every nonsingular F2/F3 model, source gcd(trace,q)!=1 is equivalent
to actual geometric killed-p subgroup cardinality one. Actual Hasse bounds
and source trace-square/count cases are proved, with no classification
premise. Every actual source-supersingular count divisor, including composite,
has gcd(r,q)=1 and modular order dividing4/6, with bounds4/6 respectively.
This does not assert that every divisor occurs as a cyclic point order.
Explicit F2 point(0,0) on y²+y=x³+x has order5/modular order4; explicit F3
point(0,1) on y²=x³+2x+1 has order7/modular order6. The source p>=5 prime-field
clause is retained; these controls do not refute it. Prior native178-model
baseline/conditional modular arithmetic/geometric j criterion credited.
Whole gate44/0failures/0trusted, exact local PCC-20261114-6fec58 replay PASS at2026-11-14T06:12:46Z,
87recursive formal/config+11artifact pins match, all83attempt files/
32unique objects/36791B archive/five gate files retained,
inspected one-page PDF47307B. Signed standing unverified. FST2010 selected
pp230/237/Definition2.1/Remark2.2/Section3 and Milne IV9/Theorem9.4 proof
reviewed, rendered pages inspected; no full-paper/book read or corpus
promotion. General prime/extension-field Hasse, trace/Endrank/geometric
identifications beyond F2/F3 and field-degree/full-torsion-field bridges
remain open. No qualified-source refutation, novelty, native experiment,
paid compute or agent upload. [pcc:unverified]


The [geometric j packet](../small_characteristic_j_audit_20261114/README.md)
proves actual p-torsion/j criteria for every nonsingular general
Weierstrass model in characteristics two and three. Over an algebraically
closed field, actual killed-p count is1ifj0elsep and induced ZModp
module rank is0ifj0else1; j0iff every actual killed-p point isidentity.
Every actual algebraically closed scalar extension has the count determined
by the base Mathlibj. Official coefficient-level j identities/oldchar2
geometry and generalchar3countclassification credited; actualchar3coefficient
classification and actualpoint/scalar/module bridges supplied.
ActualF3y²=x³+2x²+1 hasDelta1/j1/five rationalpoints/rationalkilled3card1/
geometriccard3. Lean directly refutes the universal version after deleting
algebraic closure; this is a hypothesis-control, not qualified-source error.
Sourceprime-fieldclausealreadyhasp≥5, preserved. Wholegate30/0failures/0trusted;
exactlocal PCC-20261114-04e0a2 replayPASS at2026-11-14T05:33:50Z,79recursiveformal/config+11artifact
pinsmatch;62attemptfiles/24uniqueobjects/29867Barchive/fivegatefiles,
inspectedone-pagePDF46924B. Signedstandingunverified. Milneselected
Remark3.16/context/pp98–99andp75notes read/PDFpages104/80inspected;
no fullbookread/Endrank↔p-rank/FSTtrace↔geometry/Hasse/embeddingtheorem
or otherprime/all14closure. No qualified-source refutation/novelty/corpus
promotion/nativeexperiment/spend/upload. [pcc:unverified]


The [direct small-prime source packet](../bk_small_prime_source_audit_20261114/README.md)
proves original BK count criteria at primes two and three for arbitrary
nonsingular models over every finite base field and every actual finite
extension field of cardinalityq^k. Only F,K and their actual algebra
structure are supplied: no auxiliary closure, embedding or scalar tower,
nor extra coprimality/rank/determinant/pairing/normal-form premise. Retain
rdividingactual#E andrnotdividingq−1; exactlyr²points killedr including
identity iff rdividesq^k−1. Atthree, construct the canonical closure of
chosenK internally and reuse the preceding original-source proof.
Attwo, the sourceq−1condition forceschar2; existingactualnonzero2point
uniqueness now gives killed2card1or2/finiteness/never4overeverychar2field,
andcyclotomicdivisibilityisfalse. Sourcepointcountpremiseiskeptbutunneeded
attwo. CreditedactualDelta1models/count2and1/ordinaryevenpointcardrealize
thebranches. Wholegate7/0failures/0trusted; exactlocal PCC-20261114-e0dd93 replayPASS
at2026-11-14T05:13:24Z,73recursiveformal/config+11artifactpinsmatch;31attemptfiles/
16uniqueobjects/10310Barchive/fivegatefiles and inspectedone-pagePDF46816B.
Signedstandingunverified. Otherprimes≥5/canonicalWeil/fullallkp500/
SSdefinition-trace/original14stayopen. No full-source-read/novelty/
qualified-source refutation/corpuspromotion/nativeexperiment/spend/upload.
[pcc:unverified]


The [characteristic-three/source-form packet](../three_torsion_char3_audit_20261114/README.md)
proves every nonsingular general Weierstrass model over every
characteristic-three field has exactly one or three actual points killed
by three, actual subgroup finiteness and induced ZMod3 module rank zero
or one. Nontrivial torsion yields actual tangent normalization; the literal
normal-form division polynomial/curve equation give an explicit actual
subgroup/OptionBool bijection. The trivial branch is an actual singleton.
It proves the original prime-three nine-point BK criterion without adding
coprimality: with3dividing actual#E and3notdividingq−1, exactly nine
killed-three points in each compatible chosen extension of cardinalityq^k
iff3dividesq^k−1. Outsidechar3, finiteCauchy/field3nonzero derivescoprime;
inchar3 both sides are false. Actual card1/card3 controls are credited.
The inclusion paraphrase remains coprime-qualified. Wholegate22/0failures/
0trusted; exactlocal PCC-20261114-72d986 replay PASS at 2026-11-14T05:02:48Z,60recursiveformal/config
and11artifact pins match;55attemptfiles/21uniqueobjects/16561Barchive,
fivegatefiles and inspectedone-pagePDF47261B retained. Initialbuild01
manifest-snapshot gap is explicit; subsequentchangedbuild snapshots complete.
Signedstandingunverified. Otherprimes≥5/canonicalWeil/fullallkp500,
supersingularitydefinition/traceclassification and original14 gaps stayopen.
No fullpaper/bookread/novelty/source-refutation/corpuspromotion/nativeexperiment.
[pcc:unverified]


The [general prime-three BK packet](../bk_three_normalization_audit_20261114/README.md)
proves the qualified criterion for every nonsingular general Weierstrass
model over every finite field, including characteristic two. With
gcd(3,q)=1, 3 dividing the actual rational group cardinality and
3 not dividing q−1, actual full geometric killed-three containment in
every chosen compatible extension of cardinality q^k is equivalent to
3 dividing q^k−1. It also proves the original exactly-nine-point form,
including the identity. Actual Cauchy supplies the rational point; the
curve equation, actual tangent slope and literal division polynomial
prove the required normalization. An explicit actual point-group
AddEquiv, scalar/tower/Frobenius naturality and all-n torsion/image
transport connect the preceding normal-form character theorem.
No normalization/determinant/rank/pairing premise is supplied.
Credited F7 q1/r²divisibility and ordinary characteristic-three rank-drop
controls remain explicit; the qualified source is not refuted.
Whole gate50/0failures/0trusted; exact local PCC-20261114-3f6943 replay PASS
at 2026-11-14T04:47:08Z,54recursiveformal/config+11artifact pins match,
149attemptfiles/42uniqueobjects/45090Barchive/fivegatefiles and inspected
one-page PDF47108B retained. Signed standing unverified. This closes
the earlier arbitrary-model normalization/qualified prime-three criterion
gap. Other primes≥5, canonical Weil identification, the full all-k p<500
census and other original fourteen-item obligations remain open.
No full-source-read/novelty/corpus promotion/native experiment/spend/upload.
[pcc:unverified]


The [actual cubic-character packet](../three_torsion_character_audit_20261114/README.md)
proves actual Frobeniusdetq at prime3 for every nonsingular model
 y²+a1xy+a3y=x³ over everyfinitefieldchar≠3, includingchar2. Literaldivision/
curve/nonsingularity identities supply an actualprimitivecube-root character
with exactorigin-line kernel, actualorigintranslations/negation and actual
Frobeniuscovariance. A constructedactualbasis with originfirst identifies
its value with the second-coordinate exponent, forcingdetq in everybasis.
The qualifiedBK criterion follows for everychosenfiniteextension/compatible
embedding, with qmod3≠1 and no rank/determinant/pairingpremise. Genuineordinary
char3modely²+xy+y=x³ hasΔ1/actualtorcard3/modulefinrank1/allalreadyrational;
3neverdivides3^k−1fork>0. Directdeleted-coprimality inclusion-paraphrase
negation and singularcontrols are proved. OriginalBKnine-pointcondition
remainsunrefuted. Wholegate77/0failures/0trusted; exact local PCC-20261114-efe7ee
replayPASS at2026-11-14T04:16:34Z,46recursiveformal/config+16artifactpins match,
209attemptfiles/62uniqueobjects/64558Barchive/fivegatefiles and inspected
one-page PDF47220B. ScalarIDC1h3cd3ab88de4dfcba has universalLeanidentity/
official32+32-point check. Signedstandingunverified. Arbitrarycurve actual
coordinate/group/Frobenius normalization and genericdetq outsidefamily,
canonicalWeil identification/otherprimes/fullallkp500 remainopen. Selected
primarysections/priorF5F7F25/generalrank/descent credited; no fullpaper/book
read, novelty, qualified-source refutation, corpus promotion or newnative
experiment. [pcc:unverified]


The [actual Frobenius bridge](../bk_frobenius_bridge_audit_20261114/README.md)
constructs the actual general geometric3torsion linear map/basis matrix,
proves its coordinate action, powers and basis-independent nonzero determinant,
and integrates existing chosen finite-field descent. Actual point-count
3divisibility supplies the nonzero fixed vector; a basis is constructed from
actual geometric rank, not assumed. The generic qualified criterion remains
explicitly conditional on the actual determinantq and qmod3≠1. Concrete
F2:y²+y=x³+1 has3rationalpoints/9geometrictorsionpoints, actualFrobenius²I
and nonidentity; kernel-checked finite matrix classification proves actualdet2.
It therefore satisfies the all-chosen-extension criterion without a determinant
premise. The earlier F7nonfixed witness gives a constructed actual matrix
nonidentity/q1boundary. Whole gate31/0failures/0trusted; exact local PCC-20261114-e203ff
replay PASS at2026-11-14T03:36:21Z,39recursiveformal/config and16artifact pins match,
135attemptfiles losslessly stored as41uniqueobjects/34320Barchive/fivegatefiles,
inspected one-page PDF47356B. Scalar IDC1h6d1cab18310b048b has universal Lean
identity/official32+32-point check. Signed standing unverified. OriginalBK
qualifications retained; generic actualdetq/canonicalpairings, otherprimes≥5
and fullallkp500remainopen. Prior F5/F7/F25cases/generaldescent/shortcount
credited, no fullpaper/bookread/novelty/source-refutation/native experiment.
[pcc:unverified]


The [general three-torsion extension](../three_torsion_rank_audit_20261114/README.md)
proves actual geometric rank2 at prime3 for every nonsingular Weierstrass
model in every algebraically closed characteristic except3, includingchar2.
The earlier20-declaration packet already counted9points on short curves
with2/3nonzero; its short count is credited as the baseline, and the new
short derivation is a cross-check. The extension supplies the actual
all-characteristic Ψ3point criterion, monic ordinate fibers/separability,
actual subgroupcard9/exact-order3count8, induced ZMod3modulefinrank2 and
actual Fin2basis, plus every algclosed scalar extension. Concrete F2closure
modely²+y=x³+1 hascard9/rank2/basis; genuine F3closure modely²=x³−x hascard1,
with nonzero discriminants and both CharPclasses bound. Whole gate77/0failures/
0trusted; exact local PCC-20261114-502574 replay PASS at2026-11-14T02:50:52Z,15formal/config and16artifact
pins match,219attemptfiles/fivegatefiles and inspected one-page PDF47486B.
Scalar IDC1h64000045fc926a9e has generic Lean proof/official32+32-point check.
Signed standing unverified. This closes the general geometric prime3basis
component, not other primes or the actual Frobenius/determinant coupling
with existing descent components/full qualifiedBK theorem/allkp500census.
SourceTheorem1/proof/Remarks re-read/pp142–143 inspected; qualifications
retained, no qualified-source refutation/fullpaper/book/novelty claim.
[Post-issue prior-art reconciliation](../three_torsion_rank_audit_20261114/prior-art-reconciliation.json)
records the late discovery of the earlier short proof; frozen records are
unchanged and no earlier review is invented. No native experiment. [pcc:unverified]


The [subgroup extraction audit](../subgroup_extraction_audit_20261114/README.md)
formalizes another original Thériault condition: coefficients modulo the
full ambient order cannot be tested only for being nonzero before
extracting a prime-subgroup logarithm. Generic recovery is proved for
every additive group/exact prime-order point when β is nonzero in the
subgroup field. Actual ZMod(2r),P2/orderr/card2r gives unbounded prime-order
cofactor examples with βr ambient-nonzero/nonunit yet no log information.
Actual nonsingular E/F29:y²=x³+1 has30points and P(4,6)order5; β5nonzero
mod30 gives an uninformative relation and the raw-inverse recovery fails.
Useful3P+2P0 recovers1mod5 althoughβ2is nonunit mod30; ambientunitβ1control
passes. A universal ambient extraction implication is directly negated.
Whole gate28/0failures/0trusted, exact local PCC-20261114-bf5572 replay PASS at2026-11-14T02:00:34Z;
seven formal/config and sixteen artifact pins match,63attemptfiles/
fivegatefiles and inspected one-page PDF47950B retained. Scalar supplement
IDC1hc2ffb42b1e7ce04d has a symbolic Lean proof/official32+32-point check.
Signed standing unverified. Originalp78/§§2.1/2.4/2.5/5.10 inspected;
this is an algebraic inference/actual elliptic point-group audit, not a
full genus≥3 Jacobian/Picard or realized algorithm-trace counterexample.
Full runtime and failure-probability claims remain unrefuted/open; no
coefficient distribution is proved. No full-paper-read/corpus promotion/
novelty/native experiment. [pcc:unverified]


The [generic actual iid bridge](../theriault_iid_audit_20261114/README.md)
now derives the source collision closedform and recurrence from actual
uniform finite samples, with no recurrence hypothesis. Missing-color
function subtype count(m−1)^s and finite support-indicator sum interchange
give exact expectation s−m+m(1−1/m)^s. Product-function projection transfers
the average to free paired poolsα×Bool; involution/orbit semantics are bound.
Direct corrected4s²/(9n)≤E≤s²/n holds for3≤s≤n/2. Actual pools
Fin(t+4)×Bool give an unbounded even-pool counterfamily to the original
finite lower bound at s3 within the strict source domain. Paired8mean11/16
and unpaired8mean23/64 are checked controls. Whole gate32/0failures/0trusted,
exact local PCC-20261114-0a4692 replay PASS at2026-11-14T01:46:59Z, seven formal/config and eleven
artifact pins match;59attemptfiles/fivegatefiles and inspected one-page
PDF47284B retained. Signed standing unverified. Earlier frozen conditional
repair and source-review packet remain unchanged; this closes their generic
paired iid recurrence gap. Actual algorithm walks and Jacobian/subgroup
distribution, linear-algebra rank/invertibility and full runtime stay open.
No full-paper-read/corpus promotion/novelty/native experiment. [pcc:unverified]


The [original Thériault collision audit](../theriault_collision_audit_20261114/README.md)
refutes the finite lower bound in source Theorem1,p85 at n8,s3 within its
actual sampling model: all512 uniform ordered draws from four free
negation pairs give total352 and expectation11/16, below printed3/4.
The domain3≤s<n/2 holds; source Definition7/Proposition2 count repeats
after the first, and triple-repeat/distinct-orbit/upper controls pass.
The source closedform at s3 equals6/n−4/n², below claimed6/n for alln>6.
Explicitly conditional on the displayed recurrence, repaired bounds
4s²/(9n)≤e(s)≤s²/n hold for3≤s≤n/2. The original packet left the generic paired iid recurrence
bridge open; the later generic iid packet below closes it. The full
algorithm/complexity Theorem3 is not refuted. Whole gate18/0failures/0trusted, exact local PCC-20261114-332aee replay
PASS at2026-11-14T01:34:25Z, six formal/config and sixteen artifact pins match;
59attemptfiles/fivegatefiles and inspected one-page PDF47646B retained.
Scalar supplement IDC1h939108711fe75407 has an actual symbolic Lean
proof and official checker32+32points. Signed standing unverified.
Publisher primary/relevant sections and exact pp85–86 read/inspected;
additive coverage preserves frozen unread labels, with no full-paper-read,
novelty or native experiment claim. [pcc:unverified]


The [original Verheul source audit](../verheul_distortion_audit_20261114/README.md)
checks the published p288 formula and original Theorem5 hypotheses. The
printed y-scaling map for y²=x³+a sends an actual p29/l5 point off the
curve; both primes exceed3. The repaired x-scaling is proved as an actual
point-group automorphism over every field/nonzero-discriminant model,
with group law, bijection and period3. Actual F29/F841, nonbase primitive
cube root, point count30/source trace0 and base/extensionP=(4,6)order5
are bound. CorrectedQ has order5 and is outside the actual cyclic P;
every nonzero subgroup member is distorted. Actual order3point(0,1) is
fixed as a boundary control. Whole gate49/0 failures/0 trusted and exact
local PCC-20261114-0c4538 replay pass at 2026-11-14T01:10:05Z; eight formal/config and eleven
artifact pins match, 68 attempt files/five gate files and inspected
one-page PDF47805B retained. Signed standing unverified. This refutes
the explicit printed formula, not general existence Theorem5. Its
endomorphism-ring/Tate-module and canonical pairing/scheme bridges remain
open. Original publisher/author sources and exact corpus node were read
at recorded scope; additive [coverage](../verheul_distortion_audit_20261114/coverage.json)
does not promote the frozen unread labels or imply a full-paper read.
No novelty or native experiment claimed. [pcc:unverified]


The [coordinate-length packet](../coordinate_length_audit_20261114/README.md)
constructs actual ideal solutions/coordinate-quotient algebra-map equivalence
over every field extension, for every variable type. A finite quotient gives
finite support bounded by dimension, which equals scalar-module length,
with source nilpotents permitted. The actual two-variable ideal (x^m,y−x)
has quotient isomorphic to K[t]/t^m, dimension/K-length m and exact
nilpotent powers. For m>0 its support is one over every field extension;
Lean proves arbitrary dimension gap and directly negates universal
support=dimension. The m0 empty/zero-dimension and m1 reduced/isomorphic-K
controls pass. Actual F3 dual numbers have dimension2 and at least three
endomorphisms from coefficient scalings, directly disproving the bound
after target-domain deletion. Whole gate44/0 failures/0 trusted and exact
local PCC-20261114-c3ba4f replay pass at 2026-11-14T00:34:55Z; eight formal/config and ten artifact
pins match, 62 attempt files/five gate files and inspected one-page PDF47161B
retained. Signed standing unverified. Scalar-module length is not formally
identified with scheme cycles/local intersection multiplicities. General
projective isolated-point/positive-dimensional geometry, equal-multidegree
n!dⁿ and Rojas/C refinement remain open; qualified source/node unrefuted.
Existing16 msolve calls unchanged; no new native experiment or novelty.
[pcc:unverified]


The [Diem chart packet](../diem_chart_audit_20261114/README.md)
disproves a literal helper in Proposition2.5(b)'s proof, printedp81:
two copies of the multiplicative group cannot cover the actual projective
line over F2 (three rational points versus one per torus), even allowing
arbitrary maps. Every positive product has the same obstruction. The
proposition explicitly accepts every primepower; later Condition2.7 is
not its hypothesis. Lean proves the exact finite-field rational-point
qualification: two maps cover iff cardK≥3, with explicit injective formulas.
Three explicit injective charts cover every field and product; any F2
n-torus cover needs at least3^n charts. Actual index cardinality and
3^n≤5n!/3^nB≤5B² for B=n!d^n,d>0 give the repaired chart-overhead
allowance. Whole gate31/0 failures/0 trusted and exact local PCC-20261114-66624a
replay pass at 2026-11-14T00:17:51Z; all7 formal/config and10 artifact pins match,
53 attempt files/five gate files and inspected one-page PDF47209B retained.
Signed standing unverified. This refutes the source helper only; the
main Bezout/decomposition theorem and exact corpus node remain unrefuted.
Formal scheme charts, multihomogeneous substitution, isolated-point
multiplicity geometry and the Rojas algorithm/runtime remain open. No
novelty or new native experiment is claimed. [pcc:unverified]


The [actual geometric2torsion packet](../two_torsion_rank_audit_20261113/README.md)
proves the generic r2geometry: actual killed-by2points are finite,
equivalent to identity plus distinct cubic roots, card≤4; every
algebraically closed field/scalar extension with2≠0 gives card4 and a
concrete binary basis (actual0/A/B/A+B generation and F2coefficient
independence). In characteristic2 the actual geometric cardinality is
1ifa1=0, otherwise2, derived from actual point negation/equations.
Concrete ordinary y²+xy=x³+1 has Δ1, actualF2pointcount4, geometric
2torsioncard2 over actualAlgebraicClosureF2 and all those points already
in the actual basepoint image. Prime2 and its square divide #E, while
2∤2^k−1 for everypositivek. A direct concrete point-inclusion/cyclotomic
equivalence negation tests deletion of coprimality. The full corpus node
explicitly retains gcd(r,q)=1 andr∤q−1; neither node nor qualified BK
paper is refuted. Whole gate67/0failures/0trusted and exact local PCC-20261113-3aa5ac
replay pass at 2026-11-13T23:40:16Z;11formal/config and10artifact pins match,
57attemptfiles/fivegatefiles and inspected one-page PDF47542B retained.
Signed standing unverified. Generic odd-prime geometry/Frobenius
determinant/representation/descent and full supersingular Hasse/geometric
classification remain open. No native experiment, novelty or group-scheme
formalization claim. [pcc:unverified]


The [fixed-genus arithmetic packet](../mumford_fixed_genus_audit_20261113/README.md)
now gives counterexamples over every actual fieldF_(3^(2t+1)), with
unbounded field size and the same monic/separable quinticX5−X, genus
bound2. Its injective coefficient image of the complete F3Cantor group
has8elements; every image sum is equivalent to its unique actual
normalized Cantor gcd/quotient/reduction output. The actual proper
cyclic subgroup generated by(X,0) has order2/card2 and smooth fraction1.
The full finite universe of all reduced pairs has fraction<1, with
actual nonsplit witness(X2+1,0). Lean proves unequal fractions beyond
every field-size bound and directly negates universal equality on this
family. Even fieldsF_(3^(2t+2)) make the same witness smooth, testing the
odd-degree restriction and directly refuting an unqualified extension
assertion. Whole gate60/0failures/0trusted and exact local PCC-20261113-eee828
replay pass at 2026-11-13T22:57:14Z;15formal/config and10artifact pins match,
33attemptfiles/fivegatefiles and inspected one-page PDF47390B retained.
Signed standing unverified. The group law is on the coefficient image;
full ambient group/Picard identification remains open. Order stays2;
no uniform limiting gap or typical/large-prime asymptotic refutation.
Gaudry's qualified setting is preserved. [pcc:unverified]


The [general source-split packet](../semaev_general_split_audit_20261113/README.md)
now proves the literal canonical resultant recursion for every permitted
split. The elimination variable is last and both factor degrees are
proved exactly:2^(m+1)/2^(n+1), with total arityN=(m+2)+(n+2).
Every original N≥4,1≤k≤N−3 is covered byk=n+1. Universal evaluation in
the actual MVpolynomial ring binds the literal equality; injective
independent-variable coefficient maps prove the natural dimensions.
All domain specializations under arbitrary coefficient maps retain
prescribed dimensions, allowing repetitions and degree drops. Exact
natural-degree-reset and independent-b/noncurve negative controls remain
explicit, without refuting the qualified source recursion. Whole gate82/
0failures/0trusted and exact local PCC-20261113-31f9cd replay pass at 2026-11-13T22:34:15Z.
All60formal/config and10artifact pins match;93attemptfiles/fivegatefiles
and inspected one-page PDF46864B retained. Signed standing unverified.
General splits are now proved; higher-arity absolute irreducibility
remains open. No new native experiment, novelty or source-refutation
claim. [pcc:unverified]


The [qualified BK linear packet](../bk_linear_audit_20261113/README.md)
proves the source’s actual rational-point input over every finite field:
a prime divisor of the point count gives a nonzero rational point of
exact prime order. Separately, every 2×2 matrix over a coefficient field
with a nonzero fixed vector and determinant q≠1 satisfies A^k=I iff
q^k=1 for every k, with trace and characteristic polynomial derived.
Every-prime Jordan matrices retain determinant1/nonzero fixed vectors
but have identity powers only at multiples of r; a direct universal
negation removes only q≠1. Whole gate22/0failures/0trusted and exact
local PCC-20261113-c139f1 replay pass; 10formal/config and10artifact pins match,
30attemptfiles/fivegatefiles and inspected one-page report retained.
Signed standing unverified. Generic actual E[r] rank2, Frobenius
determinant, actual-to-representation transport, torsion-field descent
and characteristic-order rank remain open. The matrix step is proved;
abstract Jordan is not labeled computed curve Frobenius. Qualified
original BK remains unrefuted. [pcc:unverified]


The [ordinary binary S3project](../binary_semaev_irreducibility_audit_20261113/README.md)
proves the literal (xy+xz+yz)²+xyz+b is symmetric and irreducible after
every field scalar extension over every characteristic2field, for allb.
Actual evaluation and nonsingular ordinary Mathlib group zero-sum semantics
are bound in the same closed certificate. An actual Artin–Schreier change,
proved degree1pole obstruction, primitive Gauss bridge, injective algebraic
closure coefficient map and explicit MVequivalence supply the proof.
The short F2expression is a nonunit square/directly nonirreducible and
provably unequal to the ordinary polynomial. No qualified ordinary source
is refuted. Whole gate43/0failures/0trusted and exact local PCC-20261113-1e2412replay
pass;24formal/config and10artifact pins match,33attemptfiles/fivegatefiles
and inspected one-page report retained. Signed standing unverified.
General splits are now proved in the general source-split packet above; higher-arity absolute irreducibility remains open.
[pcc:unverified]


The new [actual k1 pairing certificate](../pairing_k1_audit_20261113/README.md)
proves E/F7:y²=x³+3x+2 is cyclic9, with rational3torsion A and actual
B=E(F7)/3E(F7) both order3 but zero natural A→B map. Prescribed actual
Miller maps on A×B are bilinear/nondegenerate in both slots, including
raw field-unit/cubes classes and the finalexponent2 cube-root target.
Every actual point representative/every valid auxiliary is covered.
Both rational-torsion restrictions are trivial, with direct Lean negations
of nondegeneracy; every alternating biadditive map on actual A is trivial.
Whole gate116/0failures/0trusted and exact local PCC-20261113-39a4c6 replay
pass;23formal/config and12artifact pins match. Canonical divisor/valuation
identification and generic MOV/Frey–Rück remain open; original FR94remains
unread. Signed standing unverified. [pcc:unverified]

Item13 now has an actual [separate-host two-language S3 replay](../s3_host_replay_20261113/README.md).
PR4072merged at43875b707caedfb7b8175de0961e219a1152bbb3 after required checks;
first main-push run43501438026/job132147937956/attempt1 completes both
fresh C++andRust calls. Each checks942models/4188162triples/244236zeros,
with0directional discrepancies and0build/run/compare exits. Complete
public host/compiler/binary/output records are retained. The canonical
export validates with all file hashes:2planned/2attempted/2completed,
0excluded; the prospective protocol3bd9bdec… and original records remain
unchanged. Native PCC-20261113-9b23b3 is not_covered, signed standing
unverified; actual separate-host evidence is not a compiler/runtime proof
or independence from the same author. Existing11-declaration Lean actual
quadratic-field/point-oracle foundation remains linked separately.
Zero paid compute/uploads; no scientific clocks. [pcc:unverified]


The separate [all-extension BK boundary certificate](../bk_unipotent_audit_20261113/README.md)
proves actual E/F7:y²=x³+3x+2 has nine geometric 3-torsion points, with
full torsion in a field of size 7^k exactly when 3 divides k, for every
positive k and every compatible embedding. Yet 3 divides 7^k−1 always,
embedding degree is 1 and 3² divides actual #E(F7)=9. An infinite family
of actual field-image counterexamples includes every degree 3n+1. Whole
gate 32/0failures/0trusted and exact local PCC-20261113-da72ba replay pass; signed
standing remains unverified. This confirms the original paper’s excluded
boundary mechanism, refuting only deletion of its r not dividing q−1
hypothesis. Generic all-prime rank/determinant/Weil and the complete
all-extension census remain open. [pcc:unverified]


The separate [full canonical symmetry project](../semaev_symmetry_audit_20261113/README.md)
now proves every input permutation fixes the actual existing recursivePolynomial
at every arity≥3. Explicit ring equivalences identify its literal multivariate
image as IsSymmetric. The same action sends every actual input coordinate to
the permuted coordinate, fixes all coefficient-field scalars, and identifies
outer coordinate0withX. Actual S3/S4 bases, source-ordered S4-block normalization
and resultant naturality give all adjacent swaps; Mathlib finite adjacent
generation gives every permutation. Whole gate90/0failures/0trusted and exact
local PCC-20261113-cda3d2 replay pass. Signed standing remains unverified.
The subsequent general source-split packet above proves all splits. Higher absolute irreducibility remains open; earlier issued reports retain their historical scope and remain immutable.
[pcc:unverified]


The separate [actual source-order reassociation project](../semaev_reassociation_audit_20261113/README.md)
now proves literal polynomial equality between two actual canonical S3
steps and the source-ordered S4-block split, at every arity≥5 (k=2).
It retains declared sizes through degree drops, repeated coordinates and
zero P. Actual-curve S4 cross-pair symmetry supplies the argument-order
bridge. A rational independent-b counterexample directly refutes its
unrestricted extension, with a Lean proof that those coefficients cannot
come from a rational Weierstrass curve. Whole gate60/0/0 and exact local PCC-20261113-cc3069 replay pass, signed standing unverified;
the subsequent general source-split and full symmetry packets above close those obligations; higher absolute irreducibility remains open. No qualified source refutation or original-all14completion claim.
[pcc:unverified]

The additional [resultant-interchange component](../resultant_interchange_audit_20261113/README.md)
proves the fixed-size iterated Sylvester identity and both intermediate
degree bounds over every commutative integral domain. Its direct rational
counterexample gives natural-size sides−1/2 versus fixed-size sides2/2,
with both bidegrees1 and a proved strict degree drop. The natural-size
corollary exposes sharp intermediate degree equalities. Whole gate46/0/0;
exact local PCC-20261113-97e420 replay passes, signed standing unverified.
This remains an item1 helper, with no qualified Semaev refutation. The subsequent general source-split packet above binds the actual canonical family and closes arbitrary-split normalization. The original fourteen requirements retain their scope below.
[pcc:unverified]

This adds formal evidence to items1,2,3and9 and primary-source review to
items6and10. It does **not** complete the fourteen-item
request. The original hypotheses and all gaps are retained in the
[acceptance map](../theorem_audit_20261108/README.md#exact-acceptance-map).
The table here records the current delta; earlier components remain valid
only at their already-stated scope. [pcc:unverified]

| Item | Current status and concrete remaining obligation |
|---|---|
| 1 Semaev | Partial: full general-Weierstrass S3equivalence in every characteristic, repeated-coordinate cases and actual point sums proved. Short/ordinary binary forms are checked specializations. Rational-only converse and wrong-model extensions refuted, including actual F2/F3geometric controls. Actual fixed-degree S4resultant equivalence and degree-drop control proved; bounded S3/S4/S5native census completed. Canonical individual degree4in each m4input and sharp specialization criterion proved. Universal fixed-size resultant degree step, actual recursive family and last-input degree2^(m−2)for every aritym≥3now proved; reset-of-size counterexample retained. New actual bivariate-exchange proof gives newest-pair symmetry and sharp degree for both newest inputs at every recursive aritym≥4; symmetry-stage wholegate320passes. New sharp surviving-variable degree/leading coefficient theorem gives actual canonical third-newest degree2^(m−2)at allm≥5; latest wholegate357passes. All formal input degrees are now proved through actual variable permutations with explicit Fin-position coverage; all-input gate423passes. General fixed-size root criterion and leading-zero/closure controls now proved; latest census gate431passes. The new actual evaluation and point-list strong induction now prove all-m geometric zero-sum equivalence over algebraically closed fields/nonzero discriminant, with a direct nonsingular F5rational-only disproof. A separate64-declaration Lean project now proves actual S3 permutation symmetry and irreducibility for every field with2≠0/arbitrarya,b, after every field scalar extension including algebraic closure, with exact evaluation as the existing S3. The proof uses actual rational functions, the discriminant odd-degree nonsquare argument, a proved primitive Gauss bridge and explicit MV algebra equivalences. A direct F2/a=b0nonirreducibility control is a nonunit square; it does not refute the qualified source or distinct ordinary characteristic2 model. The separate actual reassociation project now proves source-ordered k2/S4-block literal polynomial normalization at every arity≥5, with actual-curve S4 cross-pair symmetry and a direct independent-b restriction control. Full actual canonical input-permutation symmetry is now proved in the separate90-declaration project, with original polynomial, all variable images and coefficient-field fixation identified. All general source splits are now proved (82); higher absolute irreducibility remains open. See ../semaev_irreducibility_audit_20261113 and ../semaev_reassociation_audit_20261113. Original six-page2004 author manuscript is now fully read, with pages2/5 visually inspected. A separate Lean certificate refutes the literal missing-plus binaryS4 product display using actual F4 rational points: actual sum0/resultant0 vs literal1; universal additive repair and all16F2tuple agreement are proved. Whole separate gate28/local packet95607fPASS, signedunverified. Historical unread/access receipts remain frozen. See ../semaev_source_audit_20261110. [pcc:unverified] Actual ordinary characteristic2S3absolute irreducibility after every field scalar map is now proved for allb with actual ordinary group zero-sum semantics and direct short-versus-ordinary model control (43). See ../binary_semaev_irreducibility_audit_20261113. [pcc:unverified] |
| 2 BK | Partial overall: complete p<500 k1census; actual F5/r3 qualified and F7/r3 excluded all-k cases; general r2geometry and characteristic2controls; general actual geometric r3rank2/basis/Frobenius/descent; normalformactualcubiccharacter/detq. **Qualified prime3 criterion now proved for every finitefield/arbitrary nonsingular Weierstrassmodel/everychosencompatibleextension**, with actual inclusion AND originalnine-point count, actualnormalization/group/scalar/tower/Frobenius bridges (50). Exactlocal PCC-20261114-3f6943 PASS, signedunverified. F7q1/r² and ordinarychar3controls retained. See ../bk_three_normalization_audit_20261114. Otherprimes≥5/canonicalWeil/allkp500remainopen. [pcc:unverified]  Originalprime3nine-point criterion nowprovedwithoutaddedcoprime; generalchar3actualcard1or3/modulefinrank0or1 withcreditedconcretecontrols (22), exactlocalPCC-20261114-72d986PASS/signedunverified. Inclusionformstillqualified. See ../three_torsion_char3_audit_20261114. [pcc:unverified]  Directoriginalr2/r3finite-fieldcountcriteria nowprovedwithonlyAlgebraFK/cardq^k, no suppliedclosure/tower/coprime (7), exactlocalPCC-20261114-e0dd93PASS/signedunverified. See ../bk_small_prime_source_audit_20261114. [pcc:unverified] |
| 3 Supersingular | Partial: actual r=2 exact-degree-two counterexamples; complete F2/F3/F4/F9 and p<500 basefield checks. Generic char2/3 actual geometric j/torsion/module criteria; all-model F2/F3 kernel source/geometry/Hasse/count/order4/6(44) and F5/F7 Hasse/source-trace0/count6/8/composite-order/named cyclic6/8(29). **Every finite field of characteristic2 or3, every actual nonsingular general model: exact source-trace iff j0 iff actual geometric p-torsion card1/rank0, with finite-extension source invariance.** Char2 also rational-geometric equality/rank; imperfect-field control rejects deleting finiteness(26). Char3 uses actual complete-square, finite cube roots, paired twist three-point construction and actual counts(55); rational replacement and trace-zero strengthening are directly refuted. Generic odd-field actual twist count/trace/source invariance and literal Galbraith9.5.3 special-j F7 counter(42) retained. General p>=5 trace-geometric/Endrank, Hasse beyond checked cohorts, embedding/full-torsion-field and further explicit extension censuses remain open. ExactlocalPCC-20261114-3a8304PASS/signedunverified; see ../finite_char_three_trace_audit_20261114. [pcc:unverified] |
| 4 Pairings | Partial: literal composite-order/nontrivial-root Tate source counterexamples on actual F3/F5 curves(50); actual F25 Weil/Tate Miller tables/proper quotient domains and rational restriction disproofs(126); actual k1 F7 true quotients/all-valid-auxiliary nondegeneracy and torsion-restriction controls(116); original Verheul printed formula actual F29/F841 counter and actual group-automorphism repair(49); generic odd-field twist counts/source invariance and special-j literal counter(42). **Characteristic-order actual domains(37): anychar-p unit roots, finite power target1, actual E/nE=kernel cardinalities, unbounded ordinarychar2 and pairedordinarychar3 domain obstructions; imperfectF2(X) target control; direct weakened-claim negations and proved prime-root=>coprime source guard.** Qualified source and group-scheme pairings retained. Canonical divisor/valuation Weil/Tate, generic MOV/FR and general End/Tate-module bridges remainopen; original FR94 remainsunread. ExactlocalPCC-20261114-f7faf2PASS/signedunverified; see ../char_p_pairing_domain_audit_20261114. [pcc:unverified]  Finite character/coinvariant algebraic core of fullyread Bruin2011: both evaluation MulEquivs/derived finitefield roots, alloddfield actual2torsion, F3positive4/4/units2 and unboundedchar2negative1/2 with directannihilation-deletionnegation(34); continuousH1/Lang/Cartier/divisor bridges remainopen. [pcc:unverified]  Literal broadBruinEq2.1 evaluation assertion outsidefinite/torsionscope refuted onactualKrullGalF2/exactsquareFrobenius/trivialdiscreteZ: allcontinuouscocycleszero, nonzerointegercoinvariantclass unreachable, no evaluation-induced classquotientbijection(16). FiniteLemma2.1/application retained; canonicalcontinuousH1notreconstructed. [pcc:unverified] |
| 5 Diem size | Partial: stable branch locus fails actual Condition2.7. Prop4.11also explicitly requires n≥2and log2q≥7n, now read/visually checked on printedp95. Conditional arithmetic absorption/countbound now proved; actual nonsingularF4curve/pointchart control has emptyfactorbase and directly refutes unrestrictedhalfbound. q2/n2control violates sizehypothesis; actualscheme degree/ramification/admissibility and geometricestimate remain open. No counterexample to qualifiedbound. |
| 6 Gaudry | Original2000 §§2–3 read; qualified fixed-genus limit retained. Complete Lean models x5-minus-x have8/7overF3and16/16overF5; Stollx5+1/F3has10/8. Native all-monic-quintic F3/F5census completes4cells with full Rust/C++matching and byte-identical repeats:162/2500squarefree models per repeat. Formal Picard identification and scaling error remain open; the asymptotic statement is not refuted. |
| 7 Subgroup smoothness | Lean disproves universal finite equality in complete actual Cantor groups: F3x5-minus-x/order2fraction1vs7/8; new complete Stollx5+1/F3checks all100actual additions, defines unique normalized arithmetic before group transfer, and proves actual(X,1)order5/nonchar3/proper subgroup fraction1vs4/5. Excluding identity from both domains gives1vs7/9and a direct universal negation. Whole Mumford gate808passes. Native all-model counts42/1510odd-prime/noncharacteristic unequal strata remain separately attested. New unbounded-genus F3family f=X^(24t+5)+1,g=12t+2proves actual unique normalized generator runs, exact ncycles, smooth fraction1vsambient<1and direct universal cycle-equality negation; whole separate gate91passes. This adds actual arithmetic scaling, while all-pair ambient group law, Picard identification and qualified typical/large-prime asymptotics remain open. See ../mumford_family_audit_20261110. A further dependent project proves all-pair normalized outputs before group transport, exact cyclic group law on the family, actual generatorordern/all membersgenerated, and equality of the earlier actualStep with generatoraddition;64/0/0gate. The full ambient group/Picard bridge still remains open. See ../mumford_family_group_audit_20261110. A further Dirichlet-based Lean extension proves unbounded prime24t+5orders with all actual arithmetic/group/fraction conclusions and direct prime-family uniformity negation;5/0/0gate. Genus grows; full ambient/Picard and typical/dominant-prime asymptotics remain open. See ../mumford_prime_family_audit_20261110. New fixed-genus2/unbounded-fieldF_(3^(2t+1))actual Cantor image-group/cyclic order2counterfamily and even-degree splitting control (60). Full ambient Picard/group remains open; no uniform limiting-gap or qualified large-prime asymptotic refutation. See ../mumford_fixed_genus_audit_20261113. [pcc:unverified] |
| 8 Bézout | Partial: actual length2/support1/native16; JV quotient dimension/F-length8/support1 (51). Diem F2two-torus source-helper disproof/qualified chart repair and overhead (31). Generic actual solution/quotient-map equivalence, finite support≤dimension=scalar-module length, actual unbounded (x^m,y−x) family and m0/m1/target-domain controls (44). Main n!dⁿ/decomposition node unrefuted; scheme/local multiplicity/positive-dimensional/Rojas bridges remain open. See ../coordinate_length_audit_20261114 and ../diem_chart_audit_20261114. [pcc:unverified] |
| 9 Gröbner heuristics | The older short-Weierstrass [f,0] counterexample is retained with its exclusions. New actual F25 twisted-Edwards input has two nonzero quadratic equations and a finite/Artinian affine quotient of dimension2, yet its actual highest parts fail regularity. Actual source-polynomial symmetrization, basis-coordinate descent, quotient equivalence and actual factor-base point sum pass a separate55-declaration gate. This refutes the literal universal n=2 reading; generic/n>=3 variants, conditional complexity and JV remain open. A new frozen18-call native cohort now checks eight literal systems twice, including this Edwards input/topparts/homogenization and both original JV F101³ toy inputs affine/homogeneous; independent native GB/remainder/S-pair/source/A/A checks pass. JVempty homogenized returnedGBdegree4 but selectedpairdegree5 adds0new/2zero; this is not a full internal-degree refutation. A separate39-declaration Lean project now proves the literal JVempty affine coordinate ideal is unit, the actual homogeneous ideal is proper and contains every monomial of total degree>=4. No genericity premise or native GB oracle. This is an uppercutoff only. A separate59-declaration Lean project now constructs the actual F101³field, checks the nonsingular curve/nonzero target, identifies all three printed coordinate equations through genuine coefficient maps and proves actual factor-base sums or signed sums cannot equal that target. The source example is confirmed at this scope. A separate51-declaration Lean project now proves the actual quotient has an eight-element basis and scalar-field dimension/length8, geometric support1, and sharp monomial cutoff4 via h³ nonmembership in the actual ideal. No native GB/Hilbert premise. Whole-curve order/scalar relations, canonical graded Hilbert-function identification and internalF4refinement remain open. Generic/n>=3 and larger m,n sweeps remain open. See ../edwards_descent_audit_20261113, ../groebner_system_audit_20261113, ../jv_macaulay_certificate_20261113, ../jv_source_field_audit_20261113 and ../jv_hilbert_audit_20261113. [pcc:unverified] |
| 10 Walks | Actual PMF fixed-support nonmixing, uniform graph-variance correction and exact3930-support/82034-map populations remain checked. New Lean theorem proves actual in-degree mean1/variance1−Σpᵢ²for every finite abelian group, distinct finite increments and normalized nonnegative label law, using a genuine product PMF and actual edge counts at every target. Direct negative control shows nonzero-only increments insufficient: duplicate+1steps have variance0versusunmerged-label1/2. Whole walk gate70passes. New actual first-collision project proves p≤rT≤rp for every labeling with increments1..r, transfers this to a genuine independent uniform product PMF, and at r20 proves actual indegree variance19/20 yet no realC bounds E[T]²≤Cp uniformly over primep>20. Whole gate35/0/0. This fixed-support obstruction does not refute the random-table collision heuristic or qualified Hildebrand theorem; averaged random-support laws and limiting constants remain open. See ../rho_collision_audit_20261110. See ../walk_audit_20261109/WEIGHTED_GENERAL.md.  Actual independent PMF cycle coupling51: everyZModn n>=2/t>0 everyevent distance<=n²/(4t), explicitcoalescence/potentialdrift/bothmarginalexpectations/uniformstationarity; allprime two distinct equalweights by time-dependentaffinepermutation. Deterministicalltime1/2/directnever-mixes and t0controls. Earlier k2/equalweight packet retains its sharperconstant; the general-weight uniformquadratic extension follows below. Stronger randomsupport/staticrho/collisionconstant gaps remain. [pcc:unverified]  Generalweight packet59 proves qualifieduniformquadraticHildebrandTheorem2 foranyfixedfinitek>=2/allstrictpositiveweights/allprime/injectivesupports, actualPMF/productindependence/margins/drift/stationarity and explicitp²/(8w_aw_bt)/gamma independentofsupport/modulus. Actualpositive-halfweight distinctincrements0/2 onZMod4 givealltimegap1/2/directnoquadraticconstant afterprime-deletion. Full13pageprimaryread/inspection; sourceitselfwarnscomposites. StrongerrandomsupportTheorem1exponent/staticrho/collisionlaws remainopen. [pcc:unverified]  Acknowledgedliteral1994Lemma4k2counter30: epsilon1/m2=4/a1=0/j1, actualfullconditionaluniformsupports/canonicalcenter/literalformulas, singletonprobability1/(p-1)>11/(40p)forallprimep>=3/directnoeventualprimebound. Laterauthorpreprintk>=3repair/knownexercisecredited, independentvalidk2mixingretained; Theorems1/2notrefuted. [pcc:unverified] |
| 11 Standards | Actual corrected 160-bit curve now has a Lean-certified prime-order-p point and actual known-answer relation; whole59gate and full local packet90623bpass. The old-preprint j-invariant disproof reproduces the author's already published erratum, not a new correction or a refutation of the corrected source. A separately frozen three-call native Smart run now recovers the known answer in all4fixed lifts; native packet162b83 remains not_covered. New Lean finite-program certificate proves all1,680transitions, lifting/inverses/coefficient recovery and actual returned dG=Q under the prescribed policy, with separate whole gates28/51and full local packetd8466dPASS. Signed standing remains unverified. The entire corrected160-bit curve now also has actual Nat.card E.Point=p proved in Lean: closed nonresidue certificate, actual missingfiber cardinal bound and Lagrange from the certified orderp point, without Hasse. Actual F5/#10/order5control directly refutes orderp-alone inference. Whole gate17and full local packet541e6fPASS, signedunverified. The requested finite repository rho measurement now completes 768/768 calls, 576/576 measured replays and all same-seed controls; a separate Lean gate passes 11 declarations for exact report arithmetic. Recorded means exceed the standard-negation reference; emitted-counter pricing reverses the plain/negation ordering across weights 0 and 1. This is a finite accounting observation, not an asymptotic refutation. Generic p-adic transfer/complexity, native refinement and limiting laws remain open. See ../rho_constant_audit_20261112. See ../anomalous_cardinality_audit_20261110. See ../anomalous_curve_audit_20261110/smart-v1. Screened NIST recommendation is not refuted. See ../anomalous_curve_audit_20261110. |
| 12 msolve/F4 | Exact original F4Run modules now imported unchanged. Generic ideal containment and direct disproof of an unrestricted saturation-output reading are proved, with actual F101 expected-row certificates; whole gate573passes. Selection/preprocessing/simplify source mapping is documented. Native F101 saturation rejects low characteristic; v1 registration defect retained. Separately frozen F65537 trial crashes in ordinary mode and times out in saturation; both not_validated, complete export structurally valid. No ordinary C/libm refinement, crash/hang diagnosis, full parameter-map or row-trace certificate yet. See F4_MODEL.md. A separate frozen eight-cell diagnostic finds original exactLA1ordinary passes, LA2ordinary139, bothsat5stimeouts, allfour combined sanitizer cells stop earlier at hash.c409signed1shift31. Cause remains unproved; no Crefinement or independently signed execution. See ../msolve_crash_diagnostic_20261110. Address-only follow-up locates LA2 READfault atla_ff_32.c543. Common-logged baseline rewrites old2 to matrix3, then reads basis3/live3 and aborts; one matrix-lookup backport returns exact ordinary known basis, both saturation cells stilltimeout. New14-declarationLean project proves the observed operational copy invariant and direct wrong-table negation, not Ctrace authenticity/pointer/wholeF4Run refinement. Currentupstream and pinned16bit alreadyusematrixlookup; no noveltyclaim. See ../msolve_reindex_lean_audit_20261110 and native coefficient-remap-v3. |
| 13 Independence | Actual separate-host C++andRust replay completed on public run43501438026/job132147937956/attempt1; both942models/4188162triples/244236zeros,0directional errors. Canonical export2planned/2completed/0excluded passes all hashes, unchanged frozen protocol. Existing Lean foundation11passes. Native PCC-20261113-9b23b3not_covered/signedunverified; compiler/runtime and independence from same author open. Original manualHTTP404 retained. See ../s3_host_replay_20261113. [pcc:unverified] |
| 14 Negative control | Prior Lean lower/upper degree certificates, production Rust control, and actual msolve degree3known-answer run pass. No retroactive claim that older censuses used this control. |

The actual higher-genus Jacobian, pairing, general geometric supersingularity and C-refinement
bridges are substantial missing formalizations, not assumed lemmas. A proof
about an abstract group or a chosen polynomial list does not discharge them.
No unsupported claim node is promoted. PCC records remain producer assertions
until a separate signed verdict. [pcc:unverified]

Sources: [sources.json](sources.json), including exact primary BK/FST/Gaudry/Hildebrand/FGHR hashes/read scopes;
the KB service remains unavailable. Reports and plots:
[reports/run-20261109-native/](reports/run-20261109-native/).
Validation receipts and inherited repository-wide check limitations are
listed in [CHECKS.md](CHECKS.md).

Integration remains blocked by divergent immutable claim ledgers on the
parent branch and main. Neither history was rewritten or selected; exact
prefix counts/revision are in CHECKS.md. The proof/census artifacts remain
reviewable on the separate stacked branch. [pcc:unverified]

Latest finite-model additions: [Leanprime-five cycle](../jacobian_smoothness_audit_20261109/STOLL_CYCLE.md), [complete native census](../jacobian_smoothness_audit_20261109/CENSUS.md). Exact348gate, full local theorem packet check and unsigned measurement statuses are linked there; the original fourteen-item audit remains partial. [pcc:unverified]

## General geometric3-torsion follow-up, 12 November

The separate [general short-curve proof](../three_torsion_rank_audit_20261112/README.md)
now proves Nat.card E[3]=9 over every algebraically closed field with
2≠0,3≠0 and4a³+27b²≠0, using actual Mathlib points and explicit inverse
coordinate maps. No rank/basis/Hasse/pairing premise. Whole gate20/0/0.
This extends the concrete F5/r3 count; the all-prime rank/determinant,
generic BK criterion and p<500 census remain separate. [pcc:unverified]
