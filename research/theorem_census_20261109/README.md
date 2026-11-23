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

## Further load-bearing primary-paper audits

The [actual doubling-surjectivity audit](../elliptic_doubling_surjectivity_audit_20261116/README.md) proves actual point doubling surjective for every nonsingular general Weierstrass model over every algebraically closed field, in all characteristics. Actual inverse-coordinate polynomial N-C(a)D has degree4; its root and an actual curve fibre yield an actual point whose doubled abscissa is a. The official nonsingular no-common-zero result forces its denominator nonzero. Actual ordinate factorization gives the desired doubled point or its negative; actual group negation supplies a half. No assumed isogeny surjectivity or chosen abstract endomorphism substitutes for actual addition. [pcc:unverified]

Lean derives surjectivity for every actual multiplication-by-2^n point map. Actual closedfield curves in characteristics2,3 and0 pass every dyadic control. On actual rational F5 E:y²=x³+x, kernel proofs show every actual rational point doubles tozero, while the actual point(0,0) is nonzero and has no rational half. The explicit unguarded universal reading is false. Galbraith9.6.3 is on geometric E(kbar), confirmed in fresh complete printed172 text/visual beforeprotocol; the qualified source survives. Point surjectivity is distinct from functionfield-map surjectivity. Full Scheme/general[n]/larger pairing comparison remains open. [pcc:unverified]

Local **PCC-20261116-3b1033 PASS at 2026-11-16T19:51:07Z**, signedunverified. Wholegate20/0failures/0trustednative;6roots from01,newwarnings0. 8states preserve failures01,02,03,04,05,06/passes07,08. All7906preservation entries57initialplans match; producer7963records/2137unique committed paths.28packages/no copied imports/hydration/update. Complete199-character main/SHA7dfe887cc3949392b6f23eb77a13fa15fff652fa038e8e7806a32654f2175c35,5transparent predicates/fullsupportheaders; separatebinding1complete0prefix0unresolved/emptysuffix aftergate. 282formal22artifactpins/222pinned204actualimports/18extra frozen. Lossless249files/120objects/194116XZ/4parts/roundtrip/actualrestore. Report1page/46580B/full4126chars/eachindividualvisual/layout inspected. Ordinaryfoundations/defaultlimits/no holes/customaxioms/nativeproofevaluation/newflags/waivers/frozenedits. [pcc:unverified]

SixKB and source/publicationConductor127; canonical/official/primary fallback,noabsence/novelty. Complete currentprimary printed172/PDF8 text and1300pxvisual beforeprotocol/code; earlier ranges/errata separatelycredited,8.2.7/fullchapter unreadhere. Samehost checks not independentparser/compiler/host/signedstanding. All14active/zero paidcompute/agents/nativeexperiments. [pcc:unverified]


The [actual level-two Weil audit](../weil_two_nondegeneracy_audit_20261116/README.md) proves left/right nondegeneracy of the EXISTING actual geometric divisor pairing at2 for every nonsingular general Weierstrass model over every algebraically closed field with2nonzero. Actual cubic discriminant16Delta and splitting give3distinct roots, each yielding actual two-torsion. A different root supplies a partner. Actual vertical full divisors and normalized infinity residue1 prove the auxiliary scalar equation c*(x-u)=u-x, hence c=-1. Actual proper moving representatives and arbitrary representative agreement identify this with the already defined quotient. No abstract pairing or assumed rank/perfectness replaces it. [pcc:unverified]

The actual ordinary binary y²+xy=x³+1 over algebraic closureF2 has nonzero discriminant and actual nonzero point(0,1) killedby2. All actual level2 root values collapse to1, so it is an actual radical element. Lean explicitly negates the universal nondegeneracy reading without a characteristic guard. Galbraith26.2.2 already has coprimality; this confirms its qualified theorem at2. Actual characteristic0/3 curves y²=x³+x give nontrivial positive values; zero/self controls pass. General levels, multiplication-isogeny-definition comparison, descent/canonical Tate/MOV/Frey–Ruck remain open. [pcc:unverified]

Local **PCC-20261116-07ba6b PASS at 2026-11-16T19:18:34Z**, signedunverified. Wholegate26/0failures/0trustednative;6roots from01,5concrete+1officialbridge,final newwarnings0. 9states retain failures01,02,03,04,05,06,07,08/passes09. All7538preservation entries/56initialplans match; producer7594records/2119unique committed paths.34packages/no copies/hydration/update. Complete195-character main/SHAdc23f02dd695a75cb02e4cab65eac2e73e2fc6b2fc5aca19c1b580d96256a953,5transparent predicates/fullsupportheaders/context; separatebinding1complete0prefix0unresolved/emptysuffix aftergate. 359formal9artifact368pins/281pinned277actual customimports/4extra frozen. Lossless266files/126objects/202508XZ/4parts/roundtrip/actualrestore. Report1page/46630B/full4181textchars/full individualvisual/layout inspected. Ordinaryfoundations/defaultlimits/no holes/customaxioms/nativeproofevaluation/newflags/waivers/frozenedits. [pcc:unverified]

Six KB and source/publicationConductor127; bounded canonical/official/primary fallback,noabsence/novelty. Galbraith26PDF1–5/printed573–577: freshpre-code lines1–230/247 covered allcritical26.2/26.3.3; final17Millerdefinition/recursion lines rereadafter09, earlierfullboundedtext/visuals credited separately. Initial broad timing wording explicitly corrected withsnapshotretained. Full external Silverman/Washington proofsunread. Samehost checks not independentparser/compiler/host or signedstanding. Zero paidcompute/agents/nativeexperiments. All14 stay active. [pcc:unverified]


The [actual doubling-separability audit](../elliptic_doubling_separability_audit_20261116/README.md)
proves that the ALREADY CONSTRUCTED actual doubling image extension is
separable for every nonsingular general Weierstrass model over every fieldK
with(2:K)nonzero, including characteristics0/3. A finite-dimensional field
extension is proved separable when its characteristic does not divide its
actual degree: each minpoly degree divides the full degree, and the official
irreducible-expand alternative forces characteristic divisibility for an
inseparable minpoly. This includes characteristic0. Actual finite degree4
then applies without a separability premise; any characteristic dividing4
would contradict2nonzero inK. Actual identity image degree1/separability
is a healthy control. [pcc:unverified]

On actual nonsingular binaryE:y²+y=x³ overF2, official pointaddition and
actual curve relations give the constructed pullback x->x^4,y->y^4+1.
Actual coefficient/x/y generators identify the EXISTING fieldmap with actual
Frobenius composed Frobenius composed actual canonical negation/conjugation.
Negation is an actual involution, so doubling(iota(f))=f^4. The actual image
extension is therefore purely inseparable; if also separable its inclusion,
hence doubling, would be surjective, contradicting actual degree4 versus
actual top-field degree1. Lean explicitly negates the unconditional universal
reading that all nonsingular doubling image extensions are separable.
Actual F3 and rational y²=x³+x pullbacks are separable; binary actualdegree4/
geometric-kernel-cardinality1 controls are retained. Galbraith9.7.3's qualified
claim is confirmed at this scope. Allgeneralchar2 models/all[n]/fullScheme
comparison/general nondegeneracy and correctedgeneral factorization remain
unfinished formal obligations. CoefficientPoint.map is not doubling globally;
its predecessor negative control stays preserved. [pcc:unverified]

Local **PCC-20261116-378474 PASS at 2026-11-16T18:36:58Z**, signedunverified. Wholegate22 audited
declarations includinggenerated/0failures/0trustednative;6roots(5concrete+
1officialbridge),newwarnings0.5states retain failures01–03/passes04/05.
All7237preservation entries/55initialplans match around builds/gate/binding/
replay; producer7292records/2091unique committed paths match.
28packages/no copied imports/hydration/update. Full229-character main/SHA
46ef1ebeb0b158a4824b639bf9fefbfad41b64d9133ca854497b33de5c145a0d, five transparent pinned predicates/full supporting
headers/context;1complete/0prefix/0unresolved/empty suffix after completedgate.
282formal19artifact301pins/222pinned204actually imported custommodules
(198credited+6new),18extra frozen modules preserved as pins.
Lossless194files106objects191120XZ/3parts/roundtrip/actualrestore match.
Firstvalidonepage47059B/full4520textchars/fullindividual1500pxvisual/inbounds.
Standard foundations/defaultlimits/no holes/custom axioms/nativeproofevaluation/
newflags/waivers/frozenedits. Samehost checks do not establish independent
compiler/parser/host or signed assurance. [pcc:unverified]

Six KB/source and publication Conductor127; canonical/official/primary fallback,
no absence claim. Galbraith author ch9.pdf SHA
f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495,
complete printed179/PDF15 text/Cor9.7.3 before protocol/setup/code; older
bounded ranges/175–176 visuals credited separately, no fullchapter/full
external Silverman/newvisual read claimed. Actual binary identity derived
from official formulas, not supplied from an unavailable source. No novelty,
native experiment, agents or paidspend. All14 remain active. [pcc:unverified]


The [actual elliptic doubling-degree audit](../elliptic_doubling_degree_audit_20261116/README.md)
constructs the doubling pullback on the actual full function field for every
field K and every nonsingular GENERAL Weierstrass model, including binary
and ternary characteristic. The actual quartic numerator and cubic-or-lower
denominator are proved coprime using actual fiber roots over AlgebraicClosureK
and the official no-common-vanishing duplication lemma. Reduced rational
normal forms prove index4/transcendence. The actual doubled generic point
is nonzero and supplies coordinates satisfying W; the actual coordinate norm
forces substitution injective, then the actual fraction-field lift fixes
coefficients and respects localization/x/y. Compatible actual quadratic
towers prove the actual image extension finite of degree4. No injectivity
or degree4 conclusion is assumed. [pcc:unverified]

Actual nonsingular F2/F3 controls have degree4, and actual identity has degree1.
For actual binary y²+y=x³, point doubling over EVERY extension field has
kernel exactly the identity, so its actual geometric kernel has Nat.card1.
Lean negates the explicit unguarded kernel-cardinality-equals-field-degree
reading; the qualified separable theorem is preserved. The short char2 model
y²=x³+x is singular at(1,0), respecting the acknowledged source exclusion.
A second explicit negative control proves coefficient Point.map is NOT point
doubling on all function-field points: it fixes a nonzero coefficient point,
whose double differs from it. The checked connection is actual GENERIC-POINT
naturality. No full Scheme/isogeny-category comparison, general[n] degree n²,
arbitrary multiplication separability, corrected general factorization or
general Weil nondegeneracy closure is inferred. [pcc:unverified]

Local **PCC-20261116-6361ce PASS at 2026-11-16T18:04:58Z**, signedunverified. Wholegate48 audited
declarations including generated equations/0failures/0trustednative;
6roots(5concrete+1officialbridge)/newwarnings0.8states retain failures01–07
and pass08. All6945preservation entries/54initialplans match around
builds/gate/binding/replay; producer6999records/2063unique committed paths match.
27packages/no copied import modules/hydration/update. Full216-character
main/SHA1e058b5a5b963f8dca5778b1b114779e7bc09a171f7c7fb0f5aa08088ae235e9, five explicit predicates with full supporting
headers/context pinned;1complete/0prefix/0unresolved/empty suffix after
completedgate.273formal19artifact292pins/216pinned198actually imported
custom modules(192credited+6new),18extra frozen modules preserved as pins.
Lossless250files125objects191940XZ/3parts, roundtrip/actualrestore match.
Firstvalid onepage47147B/full4708textchars/fullindividual1500pxvisual/inbounds.
Standard foundations/default limits only, no holes/custom axioms/native proof
evaluation/newflags/waivers/frozen edits. Samehost checks do not establish
independent compiler/parser/host or signed assurance. [pcc:unverified]

Six KB calls/two source and one publication Conductor127; bounded canonical/
official/primary fallback, no absence claim. Galbraith ch9.pdf SHA
f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495;
pre-setup relevant bounded fragments/official generalized duplication lemmas,
complete printed173–175/178–181 ranges read AFTER08. Initial too-broad
“complete” wording explicitly corrected before seal, initial protocol retained.
Earlier175/176 visuals credited; no new/fullchapter/full external Silverman
read claimed. Author char2/gcd/odd-m proof-gap corrections credited; no
novelty claimed. No native experiments or paid spend. All14 stay active.
[pcc:unverified]


The [actual elliptic Frobenius degree audit](../elliptic_frobenius_degree_audit_20261116/README.md)
proves over every finite field K that the actual q-power map, q=#K, has an
actual image field over which the full elliptic function field is finite
of degree q. Official reduced rational-function degree proves the actual
K(X^q) image index q. The canonical K(X) embedding has actual degree2;
compatible mapped intermediate fields and two actual towers cancel the
positive factor2 to obtain the elliptic degree. No degreeq conclusion is
assumed as a premise. Galbraith Corollary9.6.15 is confirmed at this actual
function-field scope, including characteristics2/3; no novelty claimed.
[pcc:unverified]

The actual geometric point map is bijective and its geometric point-set
kernel has Nat.card1. Lean explicitly negates the universal unguarded
reading equating field degree with this cardinality. The qualified
separable theorem is preserved: this actual image extension is purely
inseparable and nonseparable. Actual nonsingular F2/F3 controls have degrees2/3,
and the actual identity field-image extension has degree1. All are in the
complete main. Geometric point sets are distinguished from group-scheme
kernels; no full Scheme/isogeny-category comparison, corrected general
factorization or general Weil nondegeneracy is inferred. [pcc:unverified]

Local **PCC-20261116-d7ba18 PASS at 2026-11-16T17:35:17Z**, signedunverified. Wholegate19 declarations
including generated/0failures/0trustednative,6roots(5concrete+1officialbridge),
newwarnings0.5states retain failures01–03/passes04/05. All6660preservation
entries/53initialplans match around builds/gate/binding/replay;
producer6713records/2033unique committed paths match.
26packages/no copied import modules/hydration/update. Full201-character
main/SHA52f31a64335a62261ce11693e1f77758132f340070790511323aa014809d17be, five transparent predicates with full supporting
headers/context pinned;1complete/0prefix/0unresolved/empty suffix after
completedgate.264formal21artifact285pins/210pinned192actually imported
custom modules(186credited+6new),18extra frozen modules preserved as pins.
Lossless192files112objects184740XZ/3parts, roundtrip/actualrestore match.
Final onepage46455B/full3984textchars/fullindividual1500pxvisual/inbounds;
first font failure PDF/layout/script preserved losslessly. Standard
foundations/default limits only, no holes/custom axioms/native proof
evaluation/newflags/waivers/frozen edits. Samehost checks do not establish
independent compiler/parser/host or signed assurance. [pcc:unverified]

Six KB calls/source and publication Conductor127; bounded canonical/official/
primary fallback, no absence claim. Galbraith printed175 two-tower proof,
Example9.6.14/Cor9.6.15 and official relative-field degree APIs re-read;
primary ch9.pdf SHA
f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495,
credited bounded175/176 visuals, no fullchapter/external Silverman proof
read claimed. Author gcd(a,b)=1 erratum respected by actual reduced
normal forms. No native experiment or paid spend. All14 remain active.
[pcc:unverified]


The [actual Frobenius factorization-guard audit](../isogeny_factorization_guard_audit_20261116/README.md)
formalizes an acknowledged Galbraith9.6.18 correction. The downloaded
version2 chapter says kernel(phi)subsetkernel(psi) andPSIseparable imply
psi=lambda composedphi. Author errata explicitly requiresPHIseparable,
crediting Pierre Chretien. This is a known correction, not a novel source
error or a refutation of the corrected general theorem. [pcc:unverified]

For every actual nonsingular Weierstrass model over every finite fieldK,
q=#K>=2, Lean constructs the actual coordinate-ring and full function-field
q-Frobenius maps and proves localization/coefficient/genericpoint x/y
naturality. The actual infinity parameter has order1, so f^q=pi would
force q*ord(f)=1. Thus actual field Frobenius is injective but notsurjective,
and no arbitrary ring-map lambda* can satisfy phi*composedlambda*=identity.
The extension over its ACTUAL image field is purely inseparable and
notseparable. This verifies that the author-correctedPHI guard excludes
the example. The image field is distinct from the quadratic projection
baseK(x). [pcc:unverified]

The actual q-Frobenius on actual elliptic points over AlgebraicClosureK
is bijective with trivial kernel. Identitypsi has trivial kernel and
actual separable identity field extension, satisfying the wrong reading's
premises. Lean negates that explicit actual-function-field/rational-
coordinate factorization statement. Geometric point-set kernels, as in
source9.6.1, are distinguished from group-scheme kernels; scheme-kernel
inclusion would fail here. A full Mathlib Scheme/isogeny-category comparison
is not built. Healthy identity-left/Frobenius-square factorizations and
actual nonsingular F2/F3 examples pass. Corrected general factorization
and general Weil nondegeneracy remain open. [pcc:unverified]

Local **PCC-20261116-bc1b4e PASS at 2026-11-16T17:08:35Z**, signedunverified. Wholegate23 declarations,
0failures/0trustednative;6roots(5concrete+1officialbridge),newwarnings0.
Ten states retain failures01–08/passes09/10 and a separate source-interface
query1. All6385 predecessor/dependency entries52initialplans match around
builds/gate/binding/replay. Producer6437records/2005unique committed paths
match.25packages/no copied importmodules/hydration/update. Full172-character
main/SHAaa3cc46970752b0e498ea28e956816cea287f5920a3904fa2dced71ed034d996,
four transparent pinned bundles/full supporting headers/context;
1complete/0prefix/0unresolved/empty suffix after completedgate.
255formal20artifact275pins/204pinned186actually imported custommodules
(180credited+6new),18extra frozen modules retained as pins. Lossless
312files135objects188300XZ/3parts/roundtrip/actualrestore. Firstvalid
onepage47097B/all5055textchars/fullindividual1500pxvisual/inbounds.
Standard foundations/default limits, no holes/custom axioms/native proof
evaluation/newflags/waivers/frozenedits. Samehost checking does not establish
independent compiler/parser/host or signed assurance. [pcc:unverified]

Primary ch9.pdf387389B/SHA
f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495;
complete bounded text printed165–167,172–176/individual visuals175/176.
Actual author errata HTML SHAbdba99cb397a95b1bd6b37d2f672d23b487038dd7a3737de76c6ccee8edf825c;
its printed150 reference differs from downloaded PDF176, no pagination
equivalence claimed. Full Silverman/Washington external proofs unread.
9KB/two source Conductor/publication Conductor127; bounded primary/
canonical/official fallback, no absence/novelty claim. No native experiment
or paid spend. All14 remain active, including the recorded geometric,
counting, walk and implementation obligations. [pcc:unverified]


The [actual degree-word audit](../msolve_degree_word_audit_20261116/README.md)
proves the missing UInt16 range guard and a counterexample to unrestricted
stored-LCM degrees. Actual Lean word storage equals the natural degree
modulo65536, and equals the natural degree iff it is below65536. Input
monomials(65535,0),(65534,1) have in-range degree65535; their LCM has
true degree65536 and stored degree0. Over every field, the actual family
x^(n+2)+1,x^(n+1)y excludes y from every original degree(n+2) F4Run,
while an actual degree(n+3) run produces y. This proves the degree needed
to produce y; no full generic Groebner-basis/solving-degree theorem is
asserted. [pcc:unverified]

The frozen native protocol and analyzer were committed before data.
One registered healthy control passes: exit0/exact basis y,x²+1/degree3.
The single boundary call records the predicted wrap/selecteddegree0/
true-degree65536 rows and writes y,x^65535+1, but returns timeout124.
Its normal-completion criterion FAILS; native execution remains
notvalidated. No retry/postdata analyzer repair. Lean decodes the actual
written set/captured prefix and proves its mathematical F4Run obstruction,
without asserting complete execution, formal parser/C/compiler refinement
or independent-host assurance. Unified export check --verify-files passes;
flow2attempted/1completed/1notcompleted. [pcc:unverified]

The actual historical f4-model-sources.json header is detected as a prefix
of the full613-character theorem, stopping inside(K:=...). That frozen
record stays unchanged. A new transparent bundle rebinds all seven
inherited clauses from the original theorem. New complete131-character
main/SHA285221ce90632adbf8398f66d7c011b1c8b9ad6cdc5be42b95e1616cd6011e64:
1complete/0prefix/0unresolved/empty suffix after completed gate; separate
legacy negative0complete/1prefix/0unresolved. [pcc:unverified]

Local **PCC-20261116-3e0526 PASS at 2026-11-16T16:19:26Z**, signed unverified. Wholegate68 declarations
including generated helpers/0failures/0trustednative. Six modules (five
concrete, one official bridge), new warnings0. Seven states retain four
failures01/02/03/05 and successes04/06/07;04 was generic mathematical
phase only. All6306 predecessor/dependency entries and51initial plans
match around builds/gate/binding/replay, plus46native instrument/data
pins. Producer6357records/1937unique committed paths match. Eleven
packages/no Lean copies/hydration/update.20formal59artifact79pins/
11actually imported custommodules(5credited+6new). Lossless282files/
141objects/196748XZbytes/4parts/roundtrip/actualrestore. Firstvalid
onepage47024B/fulltext/fullindividualvisual/inbounds. Standard foundations/
default limits, no holes/custom axioms/native proof evaluation/raised
limits/waivers/frozen edits/mathPython/spend. [pcc:unverified]

Primary pinned msolve5e72b9d code and official Lean/Mathlib interfaces are
recorded with bounded ranges. Faugere1999 DOI10.1016/s0022-4049(99)00005-5
checked as metadata only; original fullpaper503/403 and C-standard503
unread. Nine KB/two source Conductor calls127; publication scope
Conductor127. Available unmodified canonical protocol tool005a7d7 pinned;
fetched newer reviewed paths opaqueVLT1/unread. No novelty/absence claim.
All14 remain active, including whole-C/row-trace refinement, completed
normal boundary execution, physical-host independence and the existing
geometric/counting/walk obligations. [pcc:unverified]


The [actual Green-envelope upper-bound audit](../markov_green_upper_audit_20261116/README.md)
formalizes the valid upper-bound conclusion supported by the KMPT2010
p8 argument. For actual doubly stochastic transition kernels, the
pointwise envelope P^j(x,y)<=c+d^j with0<=d<1 implies BOTH actual
inclusive Green-square maxima are at most
c*(T+1)*(T+2)+2/(1-d)². A finite weighted-geometric identity is proved
by induction, with exact constant contribution and zero-term/one-term/
zero-ratio controls. No desired Green bound is assumed. [pcc:unverified]

For varying actual finite nonempty state spaces and kernels, a fixed d
and actual IsLittleO error e=o(1/t²), the pointwise envelope is required
only at powers0<=j<=t. Lean constructs the nonnegative explicit error
R_t=|e_t|*(t+1)*(t+2), proves R_t tends to0, and bounds both actual
Green maxima by2/(1-d)²+R_t. For every epsilon>0 they are eventually
at most2/(1-d)²+epsilon on the same eventual set. This is a one-sided
conclusion. The credited actual uniform prime-state family with d=1/2
satisfies the bound8+R_t and has Green limits1, retaining the literal
printed-equality counter. Qualified source Lemma3.3 and all earlier
counterexamples/controls stay frozen. General Markov collision second-
moment/integer-block repair remains open. [pcc:unverified]

Local **PCC-20261116-02e783 PASS at 2026-11-16T10:18:06Z**, signed unverified. Wholegate17 declarations
including generated helpers,0failures/0trustednative. Four concrete new
modules, final new warnings0. Eight states retain failures01–07/pass08,
including the |* lexical error and rejected partial-definition target.
All6226 entries/49initial plans match around every state/gate/replay:
6190 published preservation entries plus36StaticHash dependencies.
Producer6275records/1903unique committed paths allmatch.
Fifteen packages/no copies/hydration/update. Full154-character header
SHA`28f396a4cd16557abd05803143f203b1bd0c6aee358a5177ce995f27bda734b0`,
four transparent pinned bundles/full supporting headers; separate complete
binding after completedgate1complete/0prefix/0unresolved/emptysuffix,
normalized/parsedheaders read separately.51formal19artifact70pins;
30pinned/23actually imported custommodules(19credited+4new), seven extra
frozen modules retained as pins. Lossless229files116objects168468XZ/
3actualparts/roundtrip/actualrestorematch. Firstvalidonepage46956B/all4376
textchars/fullindividual1x and1.4xvisuals/titlebboxchecked/inbounds.
Samehost proof/header check is not independent compiler/parser/host
assurance. Standardfoundations/defaultlimits, no holes/customaxioms/
native proof evaluation/newflags/waivers/frozenedits/mathPython/spend.
[pcc:unverified]

Source: KMPT2010/DOI10.1214/09-AAP625, actual arXiv0712.0220v2 electronic
reprint27Sep2010/300644B/SHA
`67bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07`.
Previous bounded text1–8/start9 and individual visuals6–8 credited;
current complete bounded page8 calculation/paragraph text re-read.
No full29-page reading or journal/reprint-identical claim. Six actual
KB calls and source-review Conductor127; canonical/official/primary
fallback, no novelty/absence claim. All14 remain active, including
general collision block repair, qualified adding-walk limiting constants,
practical hashes/scalar recovery and recorded geometry/counting/solver
obligations. [pcc:unverified]


The [actual general Green-bound audit](../markov_green_bound_audit_20261116/README.md)
proves the finite inequality in KMPT2010 Lemma3.3 for actual transition
kernels with uniform stationary distribution. From an official row-
stochastic matrix, Lean constructs actual PMFs with those real entries,
derives stochasticity of every power, and defines the inclusive Green
sum and actual finite maxima. Row normalization proves the sharper
bound sum(2j+1)*max(P^j), hence the source2*sum(j+1)*max(P^j).
Column normalization gives the column half through transposition.
Uniform invariance of normalized1/card(S) is proved equivalent to that
column condition. No desired Green moment is assumed. [pcc:unverified]

An actual positive reset chain on ZMod11 has q(0)=1/2 and all ten other
weights1/20. Its normalized PMF is irreducible/period1 and q is stationary,
but the uniform law is not. AtT1 column energy at0 equals19/4 while the
source right-hand side is4, so the actual column maximum violates the
bound if uniform stationarity is deleted. Qualified Lemma3.3 survives.
[pcc:unverified]

A separate actual uniform prime-state family refutes the literal printed
asymptotic equality following Lemma3.3 on reprintp8. For every nonzero N,
actual Green maxima equal1+(T²+2T)/N. Choose actual primes N_t>=3^(t+1),
take T=t, and error1/N_t. Lean proves error=o(1/t²), and the actual
pointwise envelope P_t^j(x,y)<=error+(1/2)^j for ALL powers j including0.
Both actual Green maxima tend to1, so the literal displayed equality
(2+o(1))/(1-1/2)², which would tend to8, is false. Even an eventual
representation with an actual vanishing remainder is negated. Every
member still satisfies the finite source bound; zero-time and singleton
controls pass. This counter concerns the printed equality. The source
finite upper bound, collision asymptotic order and qualified larger-
partition adding constant remain separately scoped. [pcc:unverified]

Local **PCC-20261116-7bb318 PASS at 2026-11-16T09:55:43Z**, signed unverified. Wholegate78 declarations
including generated equations,0failures/0trustednative. Seven modules,
six concrete and one official bridge; final new-source warnings0. Nine
states retain failures01–08 and pass09. All6163 entries/48initial plans
match around every state/gate/replay:6127 published preservation entries
plus36 StaticHash dependency entries. Producer6211records across
1874 unique committed paths all match.
Fourteen packages/no copies/hydration/update. Full172-character main
header SHA`79c651c05bc20361d063307cb87fcbfdacebef115c99371791ee0e6eda340318`
with four transparent pinned bundles/full supporting headers; separate
complete binding after actual gate1complete/0prefix/0unresolved/empty
suffix, normalized/parsed headers read separately.44formal/19artifact
pins, full inline statement/sourceplan17614B;26pinned custom modules,
19actually imported(12credited+7new),7additional frozen modules retained
as pins. Lossless295files126objects172612XZ/three actual parts, roundtrip
and actualrestore equal. Firstvalid one-page47052Breport/all4409textchars
read/complete individual visual/inbounds. Same-host checks are not
independent compiler/parser/host assurance. Standard foundations/default
limits only; no holes/custom axioms/native proof evaluation/new flags/
waivers/frozen edits/mathPython/spend. [pcc:unverified]

Source: Kim–Montenegro–Peres–Tetali2010/DOI10.1214/09-AAP625; actual
arXiv0712.0220v2 electronic reprint27Sep2010,300644B/SHA
`67bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07`;
its pagination/typography differ. Bounded text1–8/start9 and individual
visuals6–8 credited; current full lemma proof text and complete page8
visual reviewed, not all29pages. Twelve actual KB receipts and three
source-review Conductor receipts127; initial protocol’s count10 corrected
to actual12 in source review. Canonical/official/primary fallback; no
novelty/absence claim. All14 remain active, including general Markov
collision integer-block repair, qualified adding-walk limiting constants,
practical hashes, scalar-recovery runtime and recorded geometry/solver/
counting obligations. [pcc:unverified]


The [actual Markov birthday finite-bound audit](../markov_birthday_rounding_audit_20261116/README.md)
now refutes the literal finite all-c formula of KMPT2010 Theorem3.2.
The actual uniform-reset chain on47 states has a normalized transition
PMF, doubly stochastic matrix, uniform stationary law, positive one-step
reachability and period1. Its actual time1 Green sum includes the identity,
and both actual maximum row/column square sums equal50/47. With T=1,
m=M=1 and c=1/44, the displayed time equals exactly1. The actual
least-repeat collision probability is1/47, strictly below1-exp(-1/44).
Taking the ceiling of the displayed time also gives1. Lean directly
negates the all-c statement on these actual objects. [pcc:unverified]

The first-collision law is connected to the actual chronological Markov
word/path PMF by credited deferred decisions only until the first repeat.
In one step a repeat means the actual uniform next state equals the start,
so its probability is1/p. The source p8 proof uses integer k-block
amplification and then substitutes k=2cM²/m², which is1/22 for this
witness. The finite formula needs an integer-time qualification. This
counterexample does not refute the source asymptotic square-root order
or the qualified larger-partition adding-walk constant. A full general
Markov second-moment/block-ceiling repair is still open. [pcc:unverified]

For the actual uniform-reset chain, Lean proves a healthy exact law.
Selfavoiding n-label words are genuinely bijective to embeddings into
the complement of the initial state, so their number is
(p-1).descFactorial(n). Actual word mass is(1/p)^n, giving that exact
no-repeat probability, including zero once n>=p. Its recurrence proves
survival<=exp(-n(n+1)/(2p)). Every INTEGER n with n(n+1)>=2pc therefore
gives collision probability>=1-exp(-c); ceil(sqrt(2pc)) is a valid
choice for all c>=0. The same47-state/c1/44 instance passes a two-step
control, with zero-time/confidence and singleton controls. Nonzero p is
required; no empty uniform law is assumed. [pcc:unverified]

Local **PCC-20261116-0afecf PASS at 2026-11-16T09:11:41Z**, signed status unverified. Wholegate60
declarations including generated equation helpers, zero failures/trusted
native proof evaluations. Nine modules(eight concrete, one official
bridge), final new warnings0. Nine states retain failures01–08 and pass09,
including the actual default-recursion-limit failure. All6110 entries
from47 frozen plans match around every state/gate/replay:6074 published
preservation entries and36 static-hash dependency pins. Producer check
matches all6157 source/artifact/manifest entries across1843 unique
committed paths. Thirteen packages/no copies/hydration/update.
Full149-character header SHA
`b09028b7712726177e63155f7f154897daced7cef1b86748882864e4ebd7027a`
names four transparent proposition bundles in pinned source; full
supporting headers include section-variable context. Separate complete
binding after completed gate:1complete/0prefix/0unresolved/empty suffix;
normalized and parsed headers read separately.34 formal/19 artifact
pins/sourceplan15832B/full inline statement;19 custom modules all
imported(10 credited+9 new). Lossless315 files/144 objects/171640-byte
XZ/three actual parts/round-trip and actual restore equal. First-valid
one-page47307-byte report/all4946 text chars read/fullpage visual/inbounds.
Same-host prover/header checking is not independent compiler/parser/host
assurance. Standard foundations/default limits only; no holes/custom
axioms/native proof evaluation/new flags/waivers/frozen edits/mathPython/
spend. [pcc:unverified]

Source: Kim–Montenegro–Peres–Tetali, Annals of Applied Probability20(2),
495–521(2010), DOI10.1214/09-AAP625. Actual arXiv0712.0220v2 electronic
reprint,300644B/SHA
`67bd4313585101da1138a8e08bf3a3045b6347fe289d4bd076594b9d57f91b07`;
its pagination/typography differ. Complete bounded text1–8/start9 and
complete individual visuals6–8 read, not full29 pages. Publisher access
failed; Crossref/OpenAlex located the actual reprint. Twelve KB calls
and Conductor unavailable127; canonical prior/official/primary fallback,
known methods credited and no novelty/absence claim. Original Teske
remains unread. All fourteen targets remain active, including the
qualified adding-walk limiting constants/practicalhash/scalar-recovery
runtime and the other recorded geometry/solver/counting obligations.
[pcc:unverified]


The [actual geometric pairing covariance proof](../weil_galois_covariance_audit_20261116/README.md)
now proves field-extension covariance of the source26.1 geometric divisor
quotient for every natural level, including zero, over algebraically closed
fields and general Weierstrass models with nonzero discriminant. The point,
function, local-ring and value maps are actual maps. Actual normalized
orders and residues preserve proper unit domains, actual recursive Miller
functions commute with basechange, and all four auxiliary admissibility
conditions survive. Actual principal divisors, disjoint supports and point
sums identify the chosen auxiliary representatives with the selected
geometric pairing. At zero the actual quotient equals1; root membership
alone would be vacuous. [pcc:unverified]

An actual field automorphism acts on the coefficient-twisted curve and on
the field-unit value. Same-curve covariance requires an explicit equality
of the twisted and original models. On the actual nonsingular binary curve
y²+y=x³+1 over the algebraic closure of F2, Frobenius fixes the model and
squares the actual level3 pairing value. A credited actual torsion pair
has value u unequal to1, so u²≠u; Lean disproves point-only invariance
with the codomain action omitted. The same witness supplies a coefficient
moved by Frobenius, and Lean constructs a nonsingular y²+y=x³+c whose
coefficient model changes. These refute deleted actions/model hypotheses.
Galbraith26.2.3(4) includes the value action; its qualified covariance is
proved, not refuted. [pcc:unverified]

Local **PCC-20261116-edc634 PASS at 2026-11-16T08:13:03Z**, signed status unverified. Wholegate27,
seven modules(six concrete, one official bridge), zero failures/trusted
native proof evaluations and zero new-source warnings. Eleven source
states retain failures01–07,09,10 and passes08,11. All368 predecessor
pins match before/after every state/gate and around replay. Thirty-four
packages, no copies, hydration or update. Full162-character main header
SHA`ff44e0ff24ef06b93d2666bc49f788f7335f482ead464508c5c0352ee8412648`
names four transparent proposition bundles defined in the pinned source;
separate binding after completed gate gives1complete/0prefix/0unresolved/
empty suffix. Full normalized and parsed headers read separately.
Supporting full written headers retain nested named arguments and inline
proof terms as supplemental source metadata; the frozen main exporter is
unchanged. The proof map pins282 custom modules,278 imported(271 credited
and7 new).285 direct formal/17 artifact pins match. Full predecessor
manifest is pinned and its368 entries expanded, including older
configurations/artifacts; metadata dedup removes no checked dependency.
Sourceplan61185B/full inline main. Lossless233 files/88 objects/60876-byte
XZ/one part/round-trip and actual restore equal. First-valid one-page
47303-byte report/all4900 text chars read/fullpage visual/inbounds.
Same-host prover/header checking is not independent compiler/parser/host
assurance. No holes/custom axioms/native proof evaluation/raised limits/
new flags/waivers/frozen edits/mathPython/spend. [pcc:unverified]

Sources: Galbraith chapter26,222784B/SHA
`c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e`;
fresh complete bounded PDF2–4 text and separately recovered PDF3 visual,
with predecessor PDF1–5 text/all individual visuals credited.26.2.2 has
the coprime guard;26.1 fixes quotient orientation. External Silverman/
Washington/Hess proofs not read in full. Six KB wrapper calls and initial/
expanded Conductor returned127; canonical actual/official/primary fallback,
known methods credited, no novelty or absence claim. General
multiplication-isogeny comparison, all-level nondegeneracy, descent,
canonical Tate/general MOV/Frey–Rück/nonclosed residues/backend runtime
and the other fourteen-item obligations remain open. [pcc:unverified]


The [actual geometric Weil divisor laws](../weil_geometric_laws_audit_20261116/README.md)
now construct the source26.1 quotient on the actual integer n-torsion subgroup
and prove roots, inverse under swapping, bilinearity in both arguments,
full alternation including even levels, zero/negative laws and level
compatibility e_nm(P,Q)=e_n(mP,Q) for the actual correctly typed torsion
points. Every algebraically closed field/general Weierstrass model with
nonzero discriminant and every integer level/characteristic are covered.
Proper representatives/functions exist and arbitrary choices agree;
no abstract pairing law is assumed. [pcc:unverified]

Bilinearity uses actual moving to a common opposite divisor, actual divisor
addition and function multiplication. Alternation at common zeros/poles
uses genuine balanced local units; the local correction sign reduces to
the actual diagonal tame symbol, with global product1 from reciprocity and
actual degree zero. Level compatibility uses actual g^m and mD, with actual
principal equations and proper domains derived. The new quotient equals
the credited actual divisorWeilThree at3. Full geometric level3
nondegeneracy transfers with the explicit nonzero3 guard, and an actual
characteristic-two nontrivial positive checks this agreement.
[pcc:unverified]

In characteristic p every field-unit p-th root is1 and the actual level-p
quotient collapses. On the actual F5 curve y²=x³+3x+2, P=(1,1) is
nonsingular/nonzero/killed by5; its actual geometric image has exactorder5.
Lean disproves existence of any root-valued pairing on this actual curve
satisfying the literal unqualified exact-order nondegeneracy clause of
Miller1986 Definition13: it would require a fifth-root value unequal to1.
At N=1 the identity has exactorder1 and every first-root value is1, giving
a separate exact-order-wording counterexample. These target the literal
clauses without characteristic/N>1 exclusions. The modern qualified
coprime-characteristic theorem remains unrefuted. [pcc:unverified]

Local **PCC-20261116-7cb8bc PASS at 2026-11-16T07:20:06Z**, signed status unverified.
Wholegate43/10 modules(9 concrete, one official bridge)/zero failures or
trusted native evaluations;350 formal/18 artifact pins match. All355
predecessor pins match before/after11 states and gate; failed01–07 and
passes08–11 retained. Final new-source warnings0;33 packages/no copies,
hydration or update. Full323-character header SHA-256
`834caf5bae6dc8058e8bc0668dbeac30b34e63c75eeaeba56c1ef207754774ba`
names transparent proposition bundles in the pinned certificate source;
expanded individual law statements are in the proofmap. Complete separate
binding after completed gate, zero prefix/unresolved/empty suffix; full
normalized and parsed headers read separately. Lossless273 files/101
objects/66200-byte XZ/two actual parts64000+2200/round-trip and actual
restore equal. First-valid one-page47282-byte report/all4700 chars read/
fullpage visual/inbounds/no drafts. Sourceplan64692 bytes/full inline
statement and every full path/size/hash pin; redundant module and assurance
maps reference the fully pinned proofmap.275 custom modules pinned,
271 imported(261 credited+10 new). Same-host prover/procedural binding
are not independent compiler/parser/physical-host assurance. No holes/
custom axioms/native proof evaluation/raised limits/new flags/waivers/
frozen edits/mathPython/spend. [pcc:unverified]

Sources: fresh Miller, Short Programs for functions on Curves, May6,1986,
99358B/SHA39c76c7643278b87b3d8c24b9a07d0b4cbfb561cd13735548990848e0f0bd166;
complete boundedPDF4–5 text/both individual visuals and earlier algorithm
text read. This is not the2004 journal article. External references not
all read. The displayed B on p5 differs from the later evaluation formula;
no algorithm refutation is inferred. Immediate predecessor Galbraith26
PDF1–5/full individual visuals includes the explicit coprime guard26.2.2;
external isogeny-comparison/nondegeneracy proofs remain unread. Six KB
calls and initial/expanded Conductor unavailable127; bounded actual/
official/primary fallback. Known methods and existing proofs credited;
no novelty claim. [pcc:unverified]

General multiplication-isogeny comparison, full general nondegeneracy,
descent, nonclosed residue fields, canonical Tate, general MOV/Frey–Rück
and backend runtime remain open. All fourteen targets remain active.
[pcc:unverified]


The [actual Weil divisor-choice proof](../weil_divisor_choice_audit_20261116/README.md)
now proves that Galbraith26.1's quotient g(D)/f(E), on actual degree-zero
point divisors with disjoint supports and actual functions of divisors nD,nE,
is independent of both functions and both divisor representatives. It
covers every algebraically closed field/general Weierstrass model with
nonzero discriminant, every characteristic, and every integer n (hence
all natural levels). Only disjointness inside each pair is required;
old and new pairs need not be cross-disjoint. Actual root-of-unity
membership and inverse under swapping follow. At n=0 the quotient itself
is proved1, beyond vacuous zero-power membership. [pcc:unverified]

Actual local-unit evaluation gives addition/integer scaling/function powers.
Actual geometric Picard principality and point avoidance move any degree-zero
divisor outside any finite set, with an actual function witness. Changing
D to D+div(h) corrects f to f*h^n. Actual geometric reciprocity gives
 g(div(h))=h(div(g))=h(E)^n, so both quotient factors cancel. A common
intermediate divisor avoiding both right supports gives two-coordinate
independence. Actual degree-zero/torsion point-sum conditions derive
n-principal functions and proper representatives for every actual torsion
pair. No pairing-law, reciprocity or abstract population premise is assumed.
[pcc:unverified]

On the actual nonsingular F2 curve y²+y=x³, the rational points are exactly
O, P=(0,0), and -P, with both nonzero points proved distinct. No rational
S makes (P+S)-(S) disjoint from {O,P}. This disproves deletion of the
geometric-field condition from that shifted moving construction and agrees
with the source footnote; it does not refute all possible rational divisor
representations or the qualified source theorem. [pcc:unverified]

Local **PCC-20261116-c356c4 PASS at 2026-11-16T06:26:06Z**, signed status unverified.
Wholegate34/9 modules(8 concrete, one official bridge)/zero failures or
trusted native evaluations;337 formal/18 artifact pins match. All342
predecessor pins match before/after15 states and both gates;12 failed
source states include default200000-heartbeat failures05/06/07. Initial
FAIL_no_sources gate from sparse staging is retained; explicit sparse
staging and actual gate02 fix it. Final new-source warnings0;32 packages,
no copies/hydration/update. Full1289-character header SHA-256
`2c2c650e7341720a2c8e4e3a290e87ee62224df0b4af1ea50f1b412d8249d270`;
complete binding after completed gate, zero prefix/unresolved/empty suffix;
full normalized and parsed headers read separately. Lossless324 files/
111 objects/65704-byte XZ/two actual parts64000+1704/round-trip and actual
restore equal. First-valid one-page47158-byte report/all4471 chars read/
fullpage visual/inbounds/no drafts. Sourceplan65362 bytes/full inline
statement;265 custom modules pinned,253 imported (244 credited+9 new).
Same host/prover and procedural binding are not independent compiler,
parser or physical-host assurance. No holes/custom axioms/native proof
execution/raised limits/new flags/waivers/frozen edits/mathPython/spend.
[pcc:unverified]

Sources: fresh Galbraith chapter26 PDF1–5/printed573–577 text and all five
individual visuals, Exercise26.1.1/formula26.1/reciprocity sketch/complete
displayed26.3.2 proof/footnotes2–3. A truncated five-image batch was not
counted; every page was then viewed separately. External nondegeneracy and
multiplication-isogeny comparison references were not read in full. Six
KB calls and initial/expanded Conductor unavailable127; bounded actual/
official/primary fallback. No novelty or qualified-source refutation.
[pcc:unverified]

Arbitrary geometric divisor/function choice is now proved at these guards.
Multiplication-isogeny comparison, full canonical nondegeneracy, descent,
nonclosed residue fields, canonical Tate, general MOV/Frey–Rück and backend
runtime remain open. All fourteen requested targets remain active.
[pcc:unverified]


The [full geometric elliptic reciprocity proof](../elliptic_global_reciprocity_audit_20261116/README.md)
now proves the actual global tame product is one for every pair of nonzero
functions over every algebraically closed field and general Weierstrass
model with nonzero discriminant. Every characteristic is allowed. Actual
finite support precedes the global product; common zeros and poles are
handled by proper balanced local units. The corrected inverse convention
also has product one. For disjoint principal divisors, the actual proper
evaluation identity f(div g)=g(div f) follows with both support guards.
[pcc:unverified]

The actual point-to-base map has singleton infinity and the proved distinct
full finite fibers. Actual quadratic fiber roots and the discriminant guard
derive nonsingular lifts over every finite coordinate. Finite-support
restriction and regrouping, normalized finite/infinity norm projections and
canonical P1 symbols derive the mixed global identity and reciprocity.
The actual localized basis (1,y) gives coefficients for every function.
For nonbase f it derives g=a*f+b; zero coefficients are handled explicitly.
For nonzero a,b, actual units U=(-a/b)*f and V=g/b satisfy U+V=1. Actual
Steinberg, derived base-factor invariance and geometric degree zero give
full reciprocity. No global-product premise, abstract unbridged quadratic
model, prescribed residue or arbitrary square cancellation. [pcc:unverified]

On the actual singular cusp y²=x³, the divisor of x on nonsingular points
is minus two infinity: the singular finite zero is absent from that domain.
The actual constant product is c^(-2) over every field. Over the actual
algebraic closure of F5, coefficient2 gives embedded4 unequal to1, disproving
deletion of the nonzero-discriminant guard even over an algebraically closed
field. The credited nonsingular F5 rational-only product4 remains a counter
to omitting geometric points; its actual geometric extension gives1. On
nonsingular y²=x³+1 over the algebraic closure of F5, actual y and y-1 are
both proved outside K(x), with classical/inverse product1. Actual binary and
ternary geometric models give product1 for every function pair in both
conventions. Known methods and inherited proofs are credited; qualified
source theorems remain unrefuted. [pcc:unverified]

Local **PCC-20261116-b8dc3c PASS at 2026-11-16T05:26:12Z**, signed status unverified.
Whole-project gate34/9 modules (8 concrete, one official bridge), zero
failures or trusted native evaluations. All325 formal/17 artifact pins
match;256 custom modules are pinned,238 actually imported (229 credited
predecessors+9 new). All330 predecessor pins match before/after9 states and
gate; failed01/02/05/07 and passes03/04/06/08/09 retained. Final new-source
warnings0/39 inherited advisories per build/one auditor deprecation;31
packages/no copies, hydration or update. Full2745-character header SHA-256
`968cd6f69d14288e3b91b80fbd383a83bdf532265286bfe82b71429a005ed4a9`;
separate binding check after completed gate: one complete, zero prefix or
unresolved, empty suffix; full normalized and parsed headers read. Lossless
216 files/82 objects/55664-byte XZ/one part/round-trip and actual restoration
equal. First-valid one-page47222-byte report/all4759 characters read/fullpage
visual/inbounds/no drafts. Sourceplan64965 bytes/full inline statement.
Same-host/prover and procedural binding provide no independent compiler,
parser or physical-host assurance. No holes/custom axioms/native proof
evaluation/raised limits/new flags/waivers/frozen edits/mathPython/spend.
[pcc:unverified]

Sources checked: six fresh KB calls and initial/expanded Conductor were
unavailable(127), with bounded actual/official/primary fallback. Fresh
Gille–Szamuely PDF218–220/printed204–206 text and three individual visuals:
complete bounded7.4.1–4 proof bodies and Remark7.4.5, including reference-
based7.4.3 and the7.4.4 diagram proof. Earlier norm/completion/integral-closure
and external dependencies were not all read. Complete bounded7.2.8–10
quadratic-generation proof is credited from the P1 predecessor. The
beginning of7.4.6 is not its full proof. No novelty or absence claim.
[pcc:unverified]

Geometric global reciprocity and proper disjoint principal-divisor evaluation
are now proved at their stated guards. General non-algebraically-closed
closed points and residue norms, scheme identification, arbitrary divisor
representatives, multiplication-isogeny comparison, general descent,
canonical Tate and general MOV/Frey–Rück remain open. All fourteen targets
remain active. [pcc:unverified]


The [rational-function reciprocity proof](../p1_tame_reciprocity_audit_20261116/README.md)
now derives the product formula for actual nonzero rational functions with
split numerators and denominators over any field. Any finite set containing
their roots gives finite tame product times infinity symbol equal to one.
Actual finite support is proved before finprod. Over every algebraically
closed field, all rational functions satisfy the splitting guard, so the
full rational-place product and the corrected inverse product are one.
The global product is derived from actual factorization and local values;
no reciprocity conclusion is supplied as a premise. [pcc:unverified]

Canonical proper residues at every rational finite base place are constructed
using Y²+Y=(X-a)³ and its proved nonsingular local point (a,0), with derivative
in Y equal to one and a distinct negative point. Global nonsingularity of
that auxiliary model is not asserted. The proper residue fixes K, is
surjective, has maximal-ideal kernel and agrees with every realizing actual
model and point above a. Polynomial residues equal evaluation. The actual
normalized infinity symbol is also independent of its realizing model.
Derived constant/linear orders and symbol values prove all generator-pair
products, including repeated roots. Official polynomial root factorization,
actual numerator/denominator identities and two character kernels extend
the result to actual split rational units. Known methods and inherited
proofs are credited. [pcc:unverified]

Actual F5 controls distinguish a finite product 2 from the corrected product
1 after including infinity; removing local signs gives product 4 instead of
1. Actual binary and ternary products also equal one, with infinity values
1 and 2. These refute incorrect deletions without refuting a qualified source
theorem. [pcc:unverified]

Local **PCC-20261116-22c62d PASS at 2026-11-16T04:54:24Z**, signed status unverified.
Whole-project gate88/11 modules (10 concrete, one official bridge), zero
failures or trusted native evaluations. All313 formal and17 artifact pins
match;247 custom modules are pinned and229 actually imported. All316
predecessor pins match before/after19 states and gate; eight failed states
and both default-limit failure types are retained. Final new-source warnings
zero;30 packages, no copies, hydration or lake update. Full2626-character
header SHA-256 `1d1d356599cb566e6abb9c15552e91e5ed1adad3bc6ca96c5f1fdbbe78e60f15`;
separate binding check after completed gate: one complete, zero prefix or
unresolved statements, empty suffix; full normalized and parsed headers read.
Lossless401 files/125 objects/64768-byte XZ/one physical part/round-trip and
actual restoration equal. First-valid one-page47110-byte report: all4672
characters read, full-page visual inspected, in bounds, no drafts. The62636-
byte source plan keeps the full inline statement. Same-host/prover and
procedural binding provide no independent compiler/parser/host assurance.
No holes, custom axioms, native proof evaluation, raised limits, waivers,
frozen edits, mathematical Python or paid compute. [pcc:unverified]

Sources checked: six KB calls and initial/expanded Conductor unavailable
(127), with bounded official and frozen-source fallback. Gille–Szamuely
PDF204–206/208–209 (printed190–192/194–195), text and five individual visuals,
contain complete bounded proofs7.2.1–4 and7.2.8–10. The displayed7.2.2 checks
bilinearity explicitly for n=2. Earlier Milnor relations and every norm/base-
change dependency were not all read;7.2.5's field-norm identification proof
was not displayed. Initial PDF extraction preceded the fresh KB fallback;
that order is recorded. No novelty or absence claim. [pcc:unverified]

General closed-point residue norms, elliptic global reciprocity, arbitrary
divisor representatives, isogeny comparison, general descent, canonical
Tate and general MOV/Frey–Rück remain open. All fourteen targets remain
active. Rational-function reciprocity does not close these bridges.
[pcc:unverified]


The [unified finite-place projection proof](../elliptic_finite_place_projection_audit_20261116/README.md)
proves the norm/tame-symbol projection formula at every actual finite
nonsingular point of a general Weierstrass model over any field. The base
place is the official normalized (X-a)-adic valuation on K(X). The fiber is
the actual full set {P,-P}, with duplicates removed. Its cardinality and
the valuation pullback exponent are derived: two points with exponent one,
or one point with exponent two. The normalized order of the norm equals
the sum of point orders over that distinct fiber. [pcc:unverified]

Proper base residues fix K, are surjective, have the maximal ideal as kernel,
and agree across models and choices of point above a. The proof derives the
projection formula for every nonzero function and rational base unit in both
the classical and corrected inverse conventions. It supplies no assumed
projection, residue compatibility, or arbitrary square cancellation.
Actual F5 controls give full split-fiber value 1 versus one-point value 2,
and distinct fixed-fiber value 4 versus duplicated-pair value 1. Actual
binary split and ramified models give identical base maps for all arguments,
despite different fiber sizes; a ternary fixed-point case also passes.
These controls refute omitted hypotheses and incorrect implementations;
they do not refute a qualified source theorem. [pcc:unverified]

Local **PCC-20261116-3a0b23 PASS at 2026-11-16T04:06:31Z**, signed status unverified.
The whole-project gate checked 44 declarations in eight modules, with zero
failures or trusted native evaluations. All 299 formal and 17 artifact pins
match; 218 custom modules are actually imported, including eight new modules.
Seven source states are retained, including the failed fourth state. The
full 6055-character main header has SHA-256
`a5055bbf5e43e7fb1aee7caf9718cf6059ea60c4e97098d0525015538c8ab110`.
Its separate procedural binding check reports one complete statement, zero
prefixes and zero unresolved statements after the completed proof gate.
The lossless archive restores all 175 files; the one-page report was read
and visually inspected. The 64631-byte source plan preserves the full inline
header. The earlier unsupported reference-only metadata candidate and its
compatibility repair remain outside the frozen proof pins. [pcc:unverified]

Sources checked: six KB calls and both Conductor attempts were unavailable
(exit 127); bounded local and primary-source review supplied the fallback.
Gille–Szamuely PDF pages 218–220 (printed 204–206) were read and individually
viewed: complete bounded proof bodies of 7.4.1–4 and Remark 7.4.5. Earlier
norm, base-change, P1-complex and external dependencies were not all read;
7.4.6 was only partially displayed. Known methods and inherited proofs are
credited. No novelty claim is made. [pcc:unverified]

General closed points, residue extensions, inert fibers, scheme lengths,
P1 and global reciprocity, arbitrary divisor representatives, the isogeny
comparison, general descent, canonical Tate pairing, general MOV/Frey–Rück,
and completion of all fourteen items remain open. Verification used the
same host and prover; independent compiler, parser and physical-host
assurance remain open. [pcc:unverified]


The [actual ramified finite norm/tame projection proof](../elliptic_ramified_norm_projection_audit_20261116/README.md)
now proves exact normalized projection at every actual finite nonsingular
P=(a,b) fixed by negation, over every coefficient field/general Weierstrass
model/characteristic. The actual official height-one ideal(X-a) gives an
independent normalized RatFunc valuation/parameterexp−1/surjectivity. Actual
rootMultiplicity decomposition and proper nonvanishing cofactor residue
derive factor2 on every polynomial and fraction, not merely one parameter.
The actual full point fiber is a singleton/card1; normalizedbase order(Nf)
equals actualpointorder. Actual base ring/residue fixesK/surjection/maximal-
idealkernel/properunit compatibility and base residue/unit/symbol model-
independence are derived. [pcc:unverified]

Actual fixedpoint derivativeYzero and actual nonsingular finiteDVR derive
Y-b simpleparameter/order1. Actual conjugation error proportional to
(X-a)/(Y-b) has derived positiveorder, giving parameter ratioresidue−1,
including wildbinary signcollapse. Actual pointcongruence/localunit residue
covariance/leadingunit decomposition derive everyfunction ratiocharacter
=(-1)^pointorder. Actual proper balancedword comparison gives exact
baseTame(Nf,c)=tameP(f,embeddedc), every actual nonzero function/rationalbase
unit, and corrected inverse convention. No sign/character/projection
premise or arbitrarysquare cancellation. [pcc:unverified]

Actual normparameter has normalizedbase order1, normalizedbase and elliptic
symbols−1, rawrestricted1. On credited nonsingular F5 y²=x³+1 at(4,0), actual
Y and its official norm give normalized4≠raw1, disproving silent replacement
of the normalized place by its raw restriction. New actual nonsingular
binary y²+xy=x³+1 at(0,1) and ternary y²=x³+x at(0,0) have actual fixedpoints/
simpleparameters/singleton/fullfactor2/allfunction classical+inverse projection
positives and normalizednormparameter values1/2. No presumed model/order/norm/
symbol value is supplied as a hypothesis. Known methods credited/no novelty
or qualified-source refutation. [pcc:unverified]

Exact local **PCC-20261116-3686b2 PASS at 2026-11-16T03:16:18Z**, signedunverified. Wholetracked/current-
source gate59/0fail0trusted-native/10modules(9concrete1officialbridge);
288formal17artifact305pins/228pinnedcustom210actuallyimported(200creditedfrozen+
10new).292oldpins beforeafter12states/gate; ninefails01–08/10 and passes09/11/12
retained. Exact rootsfrom01/28packages/0copies0hydration/no lakeupdate.
Final12newwarnings0/39frozen inherited per build/one auditor deprecation.
Full5942header SHAc5c4e792674058653323e5e27b88d3742043087ad4c71c7bbdf93933e2d30a13;
native1complete0prefix0unresolved/emptysuffix after actualcompletedgate/full
normalized+parsedheader read. Lossless269files100objects57748B XZ/onepart/
roundtrip/actualrestoreequal/fivegatefiles. Firstvalidonepage47282B PDF/all5023
charsread/individualfullpagevisual/inbounds/no drafts. Sourceplan64548B,
redundant module-name list replaced by pinned complete import-map; no paths,
pins or imports dropped. SixcorrectKBcalls/initial+expandedConductor127/bounded
fallback/noabsence. FreshprimaryPDF214–215printed200–201 completeLemma7.3.10
proof/allfourcases/ramifiedcase4sign; displayed7.3.9 incomplete at rangeend,
full earliernorm/basechange/externalSerreNeukirchproofsNOTallread. Samehost/prover/
proceduralbinding/noindependentcompiler/parser/physicalhost assurance. Inline
producer-metadata SyntaxError stopped before any effects, corrected after
actual branch/HEAD/issuerfile checks; observed error retained outsidepins.
No holes/customaxioms/nativeproofeval/raisedlimits/newflags/waivers/frozenedit/
cryptoPython/spend/uploads. [pcc:unverified]

General closed points/residue extensions/scheme ramification, P1/global
reciprocity, arbitrary divisor representatives/isogeny comparison/general
geometric descent/canonical Tate/general MOV-FR and full fourteen-item closure
remain open. Normalized ramified rational finite fixed points are closed at
the stated actual nonsingularity/fixedpoint guard. [pcc:unverified]


The [actual split finite norm/tame projection proof](../elliptic_split_norm_projection_audit_20261116/README.md)
now derives normalized projection over every coefficient field/general
Weierstrass model/characteristic at an actual finite nonsingular P with
P≠-P. Actual RatFunc valuation restriction, actual valuation-ring embedding,
constantfixing/surjective/maximalidealkernel residue, and proper localunit
compatibility are proved. Actual P/-P base valuations agree; the official
actual point theorem gives the complete xRep fiber {P,-P}/card2. Actual
nonzero base X-x(P) embeds to the vertical function; its full divisor derives
order1/valueexp(-1)/surjectivity normalization. No assumed population or
normalization premise. [pcc:unverified]

Actual base norm order is orderP(f)+orderMinusP(f). The proper classicalbase
symbol uses an actual balanced local unit whose membership is proved;
embedding comparison and actual norm/negation covariance give exact
baseTame(Nf,c)=tameP(f,embeddedc)*tameMinusP(f,embeddedc), every actual
function/baseunit, and corrected inverse convention. Raw restricted orbit
identity remains separate at fixed points; no arbitrary square cancellation
or silent ramified normalization. [pcc:unverified]

Credited actual nonsingular F5 y²=x³+1 tangent at(0,1)/base x: actual field
cancellation (y-1)/x³=1/(y+1) gives proper positivebalancedresidue3;
at the negative point the original tangent is a proper local unit of
residue3. Actual signed factors2/3 give base normsymbol1≠P-only2, disproving
omission of the conjugate factor on actual functions. Actual fixedpoint
(4,0)/base X-4 has restricted order2≠1/valueexp(-2)≠exp(-1), disproving
deletion of the split guard. Actual nonsingular binary/ternary split points
satisfy fullfiber/normalization/allfunction projection positives. No numeric
norm/symbol value is assumed. Known methods credited/no novelty or qualified-
source refutation. [pcc:unverified]

Exact local **PCC-20261116-02c748 PASS at 2026-11-16T02:36:18Z**, signedunverified. Whole tracked gate51/
0fail0trusted-native/7modules(6concrete1officialbridge);275formal17artifact292pins/
218pinnedcustom200actuallyimported(193credited frozen+7new).282oldpins before-
after12states/gate; sevenfails02–04,06–08,11/fivepasses01,05,09,10,12 retained,
including actual defaultmaxRecDepth02 fixed by directed Finsupp evaluation,
without raised limits. Exact rootsfrom01/27packages/0copies0hydration/no lakeupdate.
Final12newwarnings0/39frozen inherited per build/one auditor deprecation.
Full3891header SHA7290edd42bdeee7698c14afe96f50140b667fda04943d6471b7cdd7b71ed4b68;
native1complete0prefix0unresolved/emptysuffix after actualcompletedgate/full
normalized+parsedheader read. Lossless251files91objects52160B XZ/onepart/
roundtrip/actualrestoreequal/fivegatefiles. Firstvalidonepage47274B PDF/all4999
charsread/individualfullpagevisual/inbounds/no drafts; sourceplan63296B.
SixcorrectKBcalls/initial+expandedConductor127/bounded fallback/noabsence.
FreshprimaryPDF218–219printed204–205 completebounded7.4.1–3proofs/earlierrefs;
fullnorm/basechange/externalSerreNeukirchproofsNOTallread, fresh7.4.4statementonly.
Samehost/prover/proceduralbinding/noindependentcompiler/parser/physicalhost
assurance. Actual external4377merge61d616/tree76e15 equalslocal134f, oldbranch
deleted/newreadyfollowup route. Mainledger1187/local632/sharedbyte-lineprefix274/
notfullprefix retained, no rechain/rebase/force/retarget/mainmerge. No holes/
customaxioms/nativeproofeval/raisedlimits/newflags/waivers/frozenedit/cryptoPython/
spend/uploads. [pcc:unverified]

Ramified finite normalized projection, general closed points/residue extensions/
scheme ramification, P1/global reciprocity, arbitrary divisor representatives/
isogeny comparison/general geometric descent/canonical Tate/general MOV-FR and
full fourteen-item closure remain open. The normalized split finite case is
closed at the stated guard. [pcc:unverified]


The [actual infinity norm/tame projection proof](../elliptic_infinity_norm_projection_audit_20261116/README.md)
now proves the exact infinity place for every coefficient field/general
Weierstrass model/characteristic, including wild binary. Actual official
RatFunc infinity valuation ring embeds with factor2 into the actual elliptic
regular ring. Actual residue fixesK, is surjective, and has the actual maximal
ideal as kernel. These derive canonical independence of the elliptic model
for base residue/unit evaluation and the classical base tame symbol. Proper
balanced local-unit membership is derived; no total meromorphic evaluation.
[pcc:unverified]

Actual negated-y and small-monomial estimates derive parameter x/y conjugation
ratio residue−1. Actual residue covariance on local units and actual leading
unit decomposition give everyfunction residue(iota(f)/f)=(-1)^ord∞f. Actual
base/elliptic balanced-word comparison plus that derived sign proves exact
baseTame(Nf,c)=ellipticTame∞(f,embeddedc) and the corrected inverse convention,
for every actual fieldunitf/rationalbaseunitc. No sign/residue/localprojection
premise or arbitrary square cancellation. Actual normparameter baseorder1,
both actual classicalsymbols−1/signlessbase1 give the credited nonsingular
F5 missing-base-sign counter on actualπ andc=Nπ. Actual nonsingular binary
and ternary values are1 and2. Known methods credited/no novelty or qualified-
source refutation. General finite closed points/residue extensions/scheme
ramification/arbitrary base places/local norm projection/P1/global reciprocity,
arbitrary divisor representatives/isogeny comparison/general geometric descent/
canonical Tate/general MOV-FR/full14 remain open; the infinity case is closed.
[pcc:unverified]

Exact local **PCC-20261116-545da0 PASS at 2026-11-16T01:44:01Z**, signedunverified. Whole tracked gate42/
0failures/0trusted-native/6concrete modules;265formal17artifact282pins/
211pinnedcustom193actuallyimported(187credited frozen+6new).273oldpins before-
after11states/gate;7fails01–05,07–08/passes06,09,10,11 retained. Exact roots
from01/26packages/0copies0hydration/no lakeupdate. Final11newwarnings0/39frozen
inherited per build/one auditor deprecation. Full2782header SHA
f4893039755a8ddc8b468b26a2457933be87eea961a96443882de54488a12af1,
native1complete0prefix0unresolved/empty suffix after actual completedgate;
fullnormalized+parsedheader read. Lossless223files83objects50524B XZ/roundtrip/
actualrestoreequal/fivegatefiles. Firstvalidonepage47589B PDF/all5050charsread/
individualfullpagevisual/inbounds/no drafts; sourceplan60079B. SixcorrectKBcalls/
initial+expandedConductor127/fallback/noabsence. FreshprimaryPDF214–215printed
200–201 completeLemma7.3.10 proof/case4 read; earlier full Milnor norm and
externalSerreNeukirchproofsNOTread. Samehost/prover/proceduralbinding, no
independent compiler/parser/physicalhost assurance. Owner0135 external4369merge
bc976/tree1c494 equalslocalfcce; oldbranchdeleted/newfollowup route, formal
sources/criteria unchanged. Metadata character/UTF8-byte assertion corrected
from unchanged chunks, no ledgercontentchange. No holes/customaxioms/native
proofeval/raisedlimits/newflags/waivers/frozenedit/cryptoPython/spend/uploads.
[pcc:unverified]


The [actual elliptic norm transport proof](../elliptic_norm_transport_audit_20261116/README.md)
now derives actual official norm-unit/point-order/principal-divisor transport:
ord_P(Nf)=ord_P(f)+ord_-P(f), D(Nf)=D(f)+iota*D(f), with actual finite support.
Actual RatFunc infinity valuation of Nf equals actual elliptic infinity
valuation of f for every function, including0. Base infinity order of Nf
equals elliptic order of f; the embedded norm upstairs has twice that order.
Every actual rational-base unit has infinity pullback factor2. Actual local
unit residues of Nf multiply the actual residues at P and -P, under both
actual unit guards. Actual signed tame-symbol negation covariance, local
norm orbit formula and derived rational-base invariance hold. Actual finite
support/negation permutation gives global covariance and the norm/square
identity; no reciprocity conclusion is inferred from a square being1.

Actual credited F5 tangent has orders3/0 at distinct P/-P, giving normorder3
not6. Everymodel infinity parameter has base normorder1/upstairs2; actual
nonsingular binary/ternary models instantiate the distinction without a tame-
ramification premise. Credited actual F5 rational-only product4≠1 has square1
and actual normproduct1, disproving invalid square cancellation on an actual
curve/function pair. Matched geometric positive stays separately credited.
No qualified-source refutation or novelty claim. General closed points/residue
extensions/scheme ramification/arbitrary base places/local norm projection/P1/
global reciprocity/arbitrary divisor representatives/isogeny comparison/general
geometric descent/canonical Tate/general MOV-FR/full14 remain open.

Exact local **PCC-20261116-06a53c PASS at 2026-11-16T01:09:49Z**, signedunverified. Whole tracked gate02
29/0failures/0trusted-native/7modules(6concrete/1officialbridge);256formal17artifact
273pins/205pinnedcustom187actuallyimported(180credited frozen+7new).263oldpins
beforeafter7states/bothgates;3failed02–04 and passes01,05,06,07 retained.
Exact roots from01/25packages/0copies0hydration/no lakeupdate. Final07newwarnings0/
39frozen inherited per build/one auditor deprecation. Native01actualexit0 but
prefix correctly rejected; exact same main guard named via separately proved
helper. Actualgate02 then native02full3663header SHA
1cae4115737fd03b2835cab7a3d71d1f889483a3d9a48a286e678ebba5fafaf3,
1complete0prefix0unresolved/empty suffix/fullnormalized+parsedheaderread.
Lossless195files83objects54672B XZ/roundtrip/actualrestoreequal/bothnativeinputs/
oldgate01 retained. Duplicate archivepath preparation failure and dependent
missing-metadata/report failures retained as actual structured observations,
not fabricated rawstderr/PDF. Overlap removed before first archive; no evidence
omitted or issued archive repacked. Firstvalidonepage47294B PDF/all4844chars
read/individualfullpagevisual/inbounds; sourceplan58804B. SixcorrectKBcalls/
initial+expandedConductor127/boundedfallback/noabsence. FreshAppendixA.6PDF327/
printed313 fulltextvisual statements/reference-only proofs read; underlying
Serre/Neukirch proofs NOTread. Prior completebounded7.3.9/7.3.10 credited,
earlierfullnormconstructionnotallread. Samehost/prover/proceduralbinding,
no independent compiler/parser/physicalhost assurance. No holes/customaxioms/
nativeproofeval/raisedlimits/newflags/waivers/frozenedit/cryptoPython/spend/
uploads. [pcc:unverified]


The [actual quadratic elliptic field-norm proof](../elliptic_field_norm_audit_20261116/README.md)
constructs the actual K(x)=RatFunc K embedding/scalar towers, derives actual
full-field localization by clearing every denominator with its conjugate and
coordinate norm, and localizes the actual1,y basis to derive degree2.
Official norm-localization and actual fractions give the finite-base norm
N(f)=f·iota(f) for every actual function. Literal minimal quadratic,
irreducibility and rational-linear representation are derived. Actual normality
always holds; separability and Galoisness hold exactly when not(2=0 and
a1=0 and a3=0). Actual discriminant excludes that boundary on every nonsingular
general model, including characteristics2/3. Actual finite-base conjugation
fixes every rational-base function, preserves norm and is nonidentity on every
nonsingular model. No rank/norm/separability/Galois premise is supplied.

Skeptical controls use actual functions and models. Actual x is transcendental
over K; the coefficient extension is not finite, so the official determinant
norm over that wrong base defaults1, while the finite-base norm of x has a
pole and differs. A credited actual nonsingular F5 example instantiates it.
Actual nonsingular F2/F3 positives have degree2 and are Galois. Actual singular
F2 cusp y²=x³ has degree2 and is normal, but is neither separable nor Galois;
its actual conjugation is identity and its actual norm is squaring. This
refutes deleted hypotheses, with no qualified-source refutation or novelty.

Exact local **PCC-20261116-826480 PASS at 2026-11-16T00:45:06Z**; signed standing unverified.
Whole tracked gate57/0failures/0trusted-native;11modules(10concrete/1official
bridge),246formal17artifact263pins/198pinnedcustom180actuallyimported
(169credited frozen+11new).249oldpins beforeafter12states/gate;10failedstates
01–10 and successes11/12 retained. Exact roots from01/24packages/0copies/
0hydration/no lakeupdate. Finalnewwarnings0/39frozen inherited per build/
oneauditor deprecation. Full1984header SHA
cfa04ee22e730426f22ddc08fcfb61d56a28200e218393752dd8582efe56d33f,
native1complete0prefix0unresolved/empty suffix after completedgate. Lossless
275files99objects53048B XZ/roundtrip/actualrestoreequal. Firstvalidonepage
47138B PDF/all4957charsread/individualfullpage inspected/inbounds. Stale
preparation stdout14states corrected against actual12without repackaging;
sourceplan55573B retains full paths and module map. Same prover/host/procedural
binding, no independent compiler/parser/physical-host assurance. Seven correct
KB calls/initial and expandedConductor127/bounded primary-official-prior
fallback/noabsence or novelty. Complete bounded local normcompatibility and
Galbraith normalform proofs read; earlier full Kato dependencies notallread.
Known methods and transparent local Field.toCommRing pattern credited.
General local norm/tame projection/ramification/P1/global reciprocity,
arbitrary divisor representatives/multiplication-isogeny comparison/general
geometric descent/canonical Tate/general MOV-FR/full14 remain open. No holes/
customaxioms/nativeproofeval/raisedlimits/newglobalflags/waivers/frozenedit/
cryptoPython/paidcompute/uploads. [pcc:unverified]


The [actual elliptic tame-symbol proof](../elliptic_tame_symbol_audit_20261115/README.md)
now constructs the classical signed local symbol using the actual general
Weierstrass function field/local orders/regular-ring unit character. Actual
balanced local orders cancel; both multiplicative laws/skew/diagonal/local
Steinberg hold in every characteristic. Actual translation constructs order1
parameters at every actual point of every nonsingular model over every field;
formula/independence for every actual order1 parameter is proved. Finite
multiplicative support is derived before finprod. Actual disjoint principal
supports give exact proper f(div g)/g(div f), with both actual guards; product1
iff equality of the two evaluations is proved as an equivalence, not asserted
for arbitrary functions. Exact coefficient-field constant/diagonal degree
formulas and geometric degreezero product1 are derived.

On credited actual F5 y²=x³+1, parameter/constant2 distinguish classical and
corrected Milnor inverse conventions; actual simple poles f+g=1 have signed1
but unsigned−1≠1. Actual x−1 has no rational finite zero/divisor−2O, giving
actual rational-point-only product4≠1 with constant2. Same actual scalar-
extended curve/functions have geometric product1. Actual nonsingular F2
simple poles have signed/unsigned1. Author-hosted Gille–Szamuely first-edition
errata p6 already corrects p187line4 inverse error; credit authors, no novelty
or qualified-source refutation. Full general norm projection/ramification/P1
reduction/global reciprocity/arbitrary divisor representatives/multiplication-
isogeny comparison/general geometric descent/canonical Tate/general MOV-FR
and remaining fourteen-item obligations remain open.

Exact local **PCC-20261115-2d7887 PASS at 2026-11-16T00:02:27Z**, signedunverified. Wholegate71/0failures/
0trusted-native/10newmodules (9concrete/1officialimportbridge);232formal17artifact
249pins/187pinnedcustom/169actuallyimported (159credited frozen+10new).
All238Auxiliarypins beforeafter14states and gate;9failed/5successes retained,
including actual default recursion failure08 symmetric simp, fixed by directed
congruence without raised limits. Exact current roots from01/0copies0hydration/
23packages/no lakeupdate. Final14newwarnings0/39frozen inherited per build/one
auditor deprecation. Full2194header SHA3880274ed056ca85c3f6fc2fac3f727c73fac24b4fc5e1f4e9b9103702625efb,
native1complete0prefix0unresolved/empty suffix after actual completed gate.
Lossless306files107objects56608B XZ/roundtrip and actualrestoreequal/fivegate
files. Firstvalidonepage47201B PDF/all4644textcharsread/individualfullpagevisual/
inbounds/no reportdraft. Fresh bounded complete primary local proof/stated
norm-reduction proof/author erratum and official/prior APIs read; earlier norm
map dependencies not allread.5correctKBcalls/Conductor127/boundedfallback/no
absence claim; expandedpublicationConductor127beforemaps/outsidepins.
Same prover/host/procedural binding, no independent compiler/parser/physical-
host assurance. Standardfoundations/defaultlimits/no holes/customaxioms/native
proofeval/newflags/waivers/frozenedits/cryptoPython/spend/uploads. [pcc:unverified]


The [actual auxiliary Weil proof](../weil_auxiliary_ratio_audit_20261115/README.md)
now constructs the exact Galbraith26.1 divisor-formula pairing at three and
proves it equals the inverse of the credited explicit tangent pairing for
every geometric three-torsion pair. Actual zero, equal, opposite and distinct-x
branches are all proved. The actual source representatives have derived
degreezero/disjointness and both required principal-divisor equations; their
class correction is the actual divisor of1/stepFunction. Both proper evaluation
domains are derived. Every auxiliary choice and every valid full geometric
function choice agrees, with no normalization premise.

The underlying actual all-n chosen Miller auxiliary quotient is a nonzero
coefficient scalar, from actual translation/negation divisor cancellation and
the credited geometric constant-kernel theorem. Actual local-unit evaluations
agree and actual admissible points exist, including simultaneous translated
domains, from quadratic fibers/finite actual divisor support. No reciprocity,
constancy, auxiliary independence, alternation or point-avoidance premise.
The actual infinity quotient of a negated y-monic line and another line has
residue−1; actual square/cube products give diagonal1, same-divisor cancellation
gives opposite1 and zero-point functions give1. The actual curve equation
classifies equal-x ordinates. All statements use every algebraically closed
coefficient field/every nonsingular general model/every characteristic.

The constructed divisor-formula pairing is alternating and has cube1. Both
bilinear/zero-radical laws have explicit nonzero3 guard. The credited actual
BinaryClosureF2 curve y²+y=x³+1 has Char2/nonzero3/nonsingular/nontrivialcube
and source inverse different from tangent/sourcevalue not1: omission of
inversion is false. The concrete actual AlgebraicClosureF3 ordinary curve
y²+xy+y=x³ has nonzero discriminant/nonzero radical when coprimality is
deleted. Actual restriction to a nonzero cyclic torsion line is degenerate.
Known methods/curves credited; no new population, qualified-source refutation
or novelty. Multiplication-isogeny definition26.2.2 comparison, arbitrary
divisor representatives beyond the translated family, general reciprocity,
general geometric divisor/Picard/Galois descent/canonical Tate/general MOV-FR
and full14 remain open. The all-n result is auxiliary independence, not a
complete all-n pairing construction/nondegeneracy theorem.

Exact local **PCC-20261115-004f3b PASS at 2026-11-15T23:22:08Z**, signed standing unverified.
Final wholetracked/transitive gate101/17newconcrete/0failures/0trusted-native;
160frozenmodules0copies0hydration/199oldformal/all233oldpins match before-after
38states/bothgates/22packages/no lakeupdate.219formal19artifact238pins/177custom
imports; full2925-character headerSHA
ac22919aa0acb5570a8c11ac17484c04af34ab74b4c4a5f36b45716c783f18dd,
native1complete0prefix0unresolved/empty suffix after completed finalgate02.
Lossless935files226objects80192B logicalXZ/two bounded parts64000/16192B/
roundtrip and actual restore equal/five finalgate files. Firstvalidonepage47591B
PDF/all5324characters read/individual fullpagevisual/inbounds/no draft.
24failed states14successes/default200000heartbeat failures retained; exact
current roots every state from01. Firstgate101PASS had5unused header-name
warnings; preflight aborted initializer but shell without errexit continued
staging/gate. Actualeffects recovered before formaledit. Final38 prefixes only
those names, samehypotheses/proof; newsourcewarnings0/39frozen inherited per
build/one auditor deprecation. New preparation wrappers use errexit.
Fresh Galbraithch26PDF1–4/16 printed573–576 fulltext/every individualvisual,
reciprocity sketch/definition/exact26.1/inversion footnote; referenced complete
external reciprocity/isogeny equivalence proofs notread/no wholechapter/book.
Five correct KB calls/Conductor127 before respective routes/code, expanded
publicationConductor127beforemaps/outsidepins; bounded fallback/noabsence.
Copyright local/excluded; no regenerablePNG archive. Same host/prover/procedural
header check, no independent parser/compiler/physical-host assurance. Standard
foundations/defaultlimits/no holes/customaxioms/nativeproofeval/newglobalflags/
waivers/frozenedits/cryptoPython/newnativecryptoexperiment/spend/upload.
[pcc:unverified]


The [actual every-field translation proof](../general_translation_audit_20261115/README.md)
removes the algebraically closed coefficient restriction from the actual
function-field action and all-point local translation interfaces. For every
field, every nonsingular general Weierstrass model and every characteristic
including two and three, actual translated generic x is transcendental,
coordinate substitution injective, and the actual fraction-field map is a
coefficient-fixing RingEquiv with composition, identity, explicit negative-point
inverse and faithfulness. Actual local values/orders/regular-ring isomorphisms
and residues commute at every point, including infinity. No transcendence,
injection, inverse, local invariance or ramification premise is supplied.

The actual infinity valuation is trivial on coefficient functions. Exact
translated x-difference values exp(−1)/exp(−2), with the second derivative
branch using actual nonsingularity, let the official
Valuation.transcendental_of_ne_one prove transcendence over the original field.
Credited norm-product injection and fraction generation derive the action.
Actual coordinate formulas prove the arbitrary coefficient-field extension
square. Actual point-evaluation/localization/full-field/residue diagrams,
chosen-uniformizer preservation and actual unit/DVR factorization prove
normalized finite-point basechange. Credited infinity basechange completes
every-point image normalization. Actual AC comparison in the proof gives
every-field local translation, with no AC assumption on the conclusion.
Actual coefficient-extension regular-ring injection and residue diagram are
also derived. These extension statements concern images of actual K-rational
points, not all closed places or geometric divisor descent.

Actual K-rational point divisors satisfy D(φ_Qf)=pushforward(−Q)D(f), with
support-disjointness iff and guarded evaluation transport. Actual nonsingular
F2 y²+y=x³ and credited F3 y²=x³+x+1 have constructed nonzero points and
nonidentity function-field translations. The credited source formula
x↦1/x,y↦−y/x² now holds over every valid coefficient field; actual F5 affine
coordinate-ring preservation fails. The credited F3 numerator x²+1 has actual
rational-only divisor degree −4, preserved under every translation. Thus
rational-point transport does not imply geometric degree zero. Known methods
and controls credited; no new population, qualified-source refutation or novelty.

Exact local **PCC-20261115-833af1 PASS at 2026-11-15T22:06:27Z**, signed standing unverified.
Whole tracked/transitive gate92/twelve concrete modules/zero failures or
trusted-native exemptions.105 frozen imports/zero copies/117 old formal files/
all134 predecessor pins match before-after11states and gate.132formal17artifact
149pins/117customimports; full3968-character header SHA
1b948fe2fb1c6a1af59ef94db55340d8d15f3080d15f149ee4d80771b2161168,
native1complete/0prefix/0unresolved/empty suffix after completed gate.
Lossless259files/91objects/40432B XZ/roundtrip equal/five gate files.
First valid onepage47541B PDF/all5013characters read/individual full-page
visual/inbounds. Four failed01/03/06/07/seven successful states retained;
exact source/root coverage asserted every state from01; final/source warnings0,
one frozen auditor deprecation. Fresh Galbraithch7PDF6–9 printed128–131 and
ch9PDF3–4 printed167–168 fulltext/each page visual. Lemma7.3.6 is an exercise,
its proof not read/no whole chapter or Scheme-proof claim. Official valuation,
actual norm/Point.map/local DVR proofs read/reused; relative algebraic closure
reviewed alternative not selected. Copyright files local/excluded. Five correct
KB calls/Conductor127 before code; expanded publication Conductor127 before
maps/outside pins, bounded fallback/no absence claim. Same host/prover/procedural
written-header check, no independent parser/compiler/physical-host assurance.
No holes/custom axioms/native proof evaluation/raised limits/new global flags/
waivers/frozen edits/cryptoPython/native crypto experiment/spend/upload.

Separate Scheme/line-bundle comparison, general geometric divisor/Picard/Galois
descent, reciprocity, isogeny/Lang/Jacobian and canonical Weil–Tate/MOV–FR remain
open. The fourteen-item request remains active. [pcc:unverified]


The [actual local translation proof](../local_translation_audit_20261115/README.md)
closes the all-local compatibility gap left by the actual function-field
translation construction. For every nonsingular general Weierstrass model
over an algebraically closed field, every characteristic including two and
three, Lean proves actual normalized valuation/order identity
v_P(φ_Qf)=v_(P+Q)(f), including all exceptional points. Actual regular rings
are isomorphic with explicit negative-point field inverse; actual residue
evaluation commutes. Local preservation, valuation/residue invariance and
ramification index one are derived, not supplied premises.

Actual shifted curve cancellation gives strict infinity-small coordinate
differences and exact actual point-uniformizer value exp(−1). The actual
coordinate/residue diagram proves localization denominators units; the actual
local map commutes with the full field and residues. Actual units and official
DVR factorization give all-function infinity equality. The actual previously
proved field composition law then gives every point. The actual principal
divisor of φ_Qf equals point pushforward through negative Q. Actual local-unit
character and divisor evaluation transport with actual support-disjointness
equivalence derived. Product-helper defaults outside the local-unit domain are
not claimed as meromorphic residues.

The actual ACF5/CharP5/nonsingular scalar extension of the credited F5 curve
y²=x³+1 has actual nonzero 3-torsion P distinct from negative P and a nonzero
actual tangent with divisor3P−3O. Its orders at P/−P are3/0; the pullback through
P has infinity order3, while positive-P pushforward has coefficient0. Lean
disproves that wrong sign and proves the negative sign. Known finite control
and source methods credited; no new population, qualified-source refutation
or novelty claim. Separate projective Scheme.localRing/line-bundle comparison,
general base-field descent, reciprocity, isogeny/Lang/Jacobian and canonical
Weil–Tate/MOV–FR remain open; the full14 request remains active.

Exact local **PCC-20261115-67cf17 PASS at 2026-11-15T21:07:29Z**, signed standing unverified.
Whole tracked/transitive gate78/nine new concrete modules/zero failures or
trusted-native exemptions.96 frozen modules/zero copies/105 old formal files/
all124 old pins checked before/after21states and gate; prior82/536 gates
credited separately.117 formal17 artifact134 pins/full2102-character
SHA77d0bb9d6c1b5941dec62869d0023c6bb2917322004d870fc762d26ffcc50dc0;
native one complete/zero prefixes/zero unresolved/empty suffix after completed
gate. Lossless418files/120objects/41704B XZ/roundtrip equal/five gate files.
First valid report onepage47202B/all4515characters read/individual full-page
visual/inbounds; no draft. Fourteen failed states/seven successes retained.
Early latest-only Lake roots could reuse old oleans:17 fixed roots and rebuilt
all seven current modules; exact root assertion from18/final21 all nine/whole
source gate passed. Missing imports caused retained default200000-heartbeat
failures; explicit imports/concrete regular-ring interfaces fixed them.
Final/source-build warnings0/one frozen auditor deprecation. Fresh corrected
Galbraithch7PDF10–11 printed132–133 fulltext/individual visuals; relevant
uniformizer/valuation lemmas and beginning7.5.1 only, no whole7.5.2 claim.
Ch9PDF3–4 text reread/prior visuals credited/Q/O exclusions retained. Official
DVR/adic and prior actual local/residue/translation proofs read and reused;
copyright local/excluded. Four correct KB calls/Conductor127 before code and
expanded publication Conductor127 before maps/outside pins; bounded fallback/
no absence claim. Same host/prover/procedural header check, no independent
parser/compiler/physical-host assurance. No holes/custom axioms/native proof
evaluation/raised limits/new global flags/waivers/frozen edits/cryptoPython/
native crypto experiment/spend/upload. [pcc:unverified]


The [actual elliptic translation proof](../elliptic_translation_audit_20261115/README.md)
constructs coefficient-fixing automorphisms of the actual full function field
for every nonsingular general Weierstrass model over an algebraically closed
field, in every characteristic including two and three. Actual translated
generic-point transcendence and coordinate substitution injectivity are derived
from the actual quadratic fiber, credited minimal-polynomial argument and
official norm product. Actual fraction-field lifting, official Point.map,
and coefficient/x/y/basis/fraction generation derive composition, identity,
explicit negative-point inverse, RingEquiv and faithfulness. No supplied
transcendence, injectivity, inverse or action premise.

Actual point pushforward permutes integer divisors, preserves degree and
changes point sum by degree(D)•Q. The actual elliptic principality criterion
proves principal preservation and the actual induced Picard action is identity.
An actual function with shifted principal divisor exists, but is not yet
identified with the constructed field pullback. Pushforward Q would match
field pullback negative Q. All-local valuation/residue compatibility remains
open, alongside general base-field descent, Scheme/line-bundle APIs,
reciprocity, isogeny/Lang/Jacobian and canonical Weil–Tate/MOV–FR.

On the source Example9.2.4 curve y²=x³+x and Q=(0,0), the actual constructed
field automorphism sends x to1/x and y to−y/x². Actual reciprocal x has infinity
order2 and is outside the actual affine coordinate-ring image, whose nonzero
functions have nonpositive infinity order. A concrete actual algebraic closure
of F5 with actual CharP5/nonzero discriminant proves translation need not
preserve that ring. Actual F5 raw affine evaluation at Q fails against Q+Q=O;
ordinary P=(2,0) has nonzero denominator and actual translate(3,0), agreeing
with the formula. The source explicitly excludes Q/O: qualified source survives,
known example and methods credited, no new population or novelty claim.

Exact local **PCC-20261115-2e952c PASS at 2026-11-15T20:30:05Z**, signed standing unverified.
Whole tracked/transitive gate82/12 new concrete modules/zero failures or trusted
native;84 frozen modules/zero copies/90 old formal files/all107 old pins checked
before/after24 states and gate. Fourteen failed states/ten successes retained;
final24/new source warnings0/one frozen auditor deprecation.105 formal19 artifact
124 pins; full1910-character SHA2569e8f18530d274a7a60789d4a0a33f511b376793a856212a165811f5f72bb8;
native one complete/zero prefixes/zero unresolved/empty suffix after completed
gate. Lossless480 files/136 objects/40900B XZ, roundtrip equal/five gate files.
Final onepage47299B PDF/all4744 characters read/full-page visual inbounds;
seven first-report files retained in47132B XZ after correcting strictly-negative
wording for constants. Fresh Galbraith correctedch9 PDF3–4/36, printed167–168,
Definition9.2.2/Example9.2.4 fulltext and individual visual; freshch7 PDF8–9,
printed130–131 fulltext context only. No whole-chapter read; copyright local.
Four correct KB calls/Conductor127 before code; expanded publication Conductor127
before maps/outside pins; bounded fallback/no absence claim. Same host/prover/
procedural written-header check, no independent parser/compiler/physical-host
assurance. No holes/custom axioms/native proof evaluation/raised limits/new global
flags/waivers/frozen edits/cryptoPython/native crypto experiment/spend/upload.
Full fourteen-item request remains unfinished. [pcc:unverified]


The [actual elliptic divisor-class bridge](../elliptic_picard_bridge_audit_20261115/README.md)
now constructs **the actual elliptic Abel–Jacobi group equivalence**
P ↦ class((P)−(O)) for every nonsingular general Weierstrass model over an
algebraically closed field, in every characteristic including two and three.
The quotient is actual finite integer point divisors of degree zero modulo
divisors of actual full-function-field units. Both directions are proved,
with no supplied Picard, genus, Riemann–Roch, normality, no-simple-pole or
point-sum-zero premise. Weak Nullstellensatz identifies every affine maximal
ideal with an actual nonsingular point. The actual denominator ideal and
credited actual local DVR integer lift derive finite-regular iff actual
coordinate-ring membership. Actual infinity order is negative norm degree;
the official norm-degree-not-one theorem excludes a lone simple infinity pole,
giving injectivity. Actual chord/vertical functions prove addition; actual
finite integer-divisor reduction proves surjectivity and constructs an AddEquiv.
An actual geometric divisor is principal iff its degree and actual point sum
are both zero. Actual (P)−(Q) is principal iff P=Q, and every actual principal
point sum is zero. The elliptic geometric point-divisor quotient bridge is
closed; line-bundle/projective Scheme Picard comparison, quotient descent/
basechange, translations, Weil reciprocity, isogeny/Lang/Jacobian and canonical
Weil–Tate/MOV–FR remain open.

The credited actual nonsingular F3 curve y²=x³+x+1 and unit X²+1 have
rational-point-only divisor degree−4. Lean proves that the actual principal
map cannot even be restricted to degree-zero rational-point divisors with
unchanged values. Over the actual algebraic closure of F3, the credited mapped
nonsingular curve has the actual bijection and a nonzero quotient class.
This credits the old control, not a new population or qualified-source refutation.
Established source divisor reduction, official affine norm method and prior
actual DVR/divisor/chord proofs are credited; no novelty claim.

Exact local **PCC-20261115-d00adb PASS at 2026-11-15T19:25:02Z**, signed standing unverified. Whole tracked/
transitive gate35 theorems/eight new concrete modules/zero failures or trusted
native;76 frozen modules/zero copies/79 old formal files/all96 old pins checked
before/after13 states and gate. Six failed states retained, final13 clean;
prior536 gate credited separately. All90 formal17 artifact107 pins match.
Full1056-character SHA1d0707209326c4217584e719bcfaa2f2284a11b5d12e390a72c59c093d7ef9d4;
native one complete/zero prefixes/zero unresolved/empty suffix after completed
gate. Lossless226 files/83 objects/32044 bytes, roundtrip equal, five gate files.
First report valid: one page47363 bytes/all4807 characters read/full-page visual
inbounds. Final source warnings0, one frozen auditor deprecation retained.
Fresh Galbraith correctedch7 PDF16–21 fulltext/individual visual16,20,21,
printed138–143, Definition7.8.1/Theorems7.9.7–9. Freshch8 PDF17–18/19 fulltext
and individual visual, printed161–162, Theorems8.6.1/8.6.4/Cor8.6.5. No whole-
chapter read; copyright files local and excluded from archive. The source's
genus/differential argument is distinct from this explicit norm proof.
Official Point.toClass injection/norm proof read and credited; its affine
ideal class group is not silently relabelled as the geometric quotient.
Four correct KB calls and Conductor127 before code; publication Conductor127
before maps/outside pins; bounded fallback/no absence claim. Same host/prover/
procedural header check, no independent parser/compiler/physical host assurance.
No holes/custom axioms/native proof evaluation/raised limits/new flags/waivers/
frozen edits/cryptoPython/native crypto experiment/spend/upload/full fourteen-
item completion claim. [pcc:unverified]


The [actual seven-torsion rank proof](../seven_torsion_rank_audit_20261115/README.md)
derives **49 actual geometric points killed by seven**, 48 of exact order seven,
and an actual two-element ZMod7 basis for every nonsingular general Weierstrass
model over an algebraically closed field with seven nonzero, including
characteristics two, three and five. The literal pinned Mathlib preΨ7 point
criterion is proved in every characteristic. Actual tripling and fourfold
x-representatives come from the official actual addition/subtraction map and
duplication; all division domains are proved. Structured derivative identities
over every commutative ring derive separability; the official degree theorem
gives 24 distinct roots, two actual ordinates each, and an explicit point/fiber
equivalence gives the whole kernel49. Actual annihilation and cardinality
then supply the module rank and basis. No kernel count, rank, separability,
multiplication formula or point/polynomial bridge is supplied as a premise.
The general-model **rank gap at seven is closed**. Determinants for primes≥5,
ranks for general primes≥11, canonical Weil and the full all-k p<500 census
remain open.

Actual characteristic2/3/5 controls have kernel49/rank2. Over the actual
algebraic closure of F7, y²=x³−x has discriminant1 and literal Ψ7(0)=−1.
Its leading coefficient vanishes, so degree≤23. The distinct-root bound and
actual ordinate fibers give whole kernel **≤47, hence not49**, and rank≠2.
This strict bound disproves deleting coprimality from rank/count; it is not an
exact kernel1 count, BK point-count-divisibility refutation, group-scheme
length or supersingularity-definition claim. Prior actual3/5 methods and
official degree/duplication/sym2x results are credited; no novelty or
qualified-source refutation claim.

Exact local **PCC-20261115-919b86 PASS at 2026-11-15T18:58:37Z**, signed standing unverified. Whole tracked
and transitive gate59 theorems/11 new modules(10 concrete/1 public adapter),
0 failures/0 trusted native;22 frozen modules/0 copies/28 old formal configs,
all45 old pins checked before/after8 states and gate. One failed07 and seven
successes retained, final08 clean; prior50 gate credited separately. All42
formal17 artifact59 pins match. Full1611-character header SHA
1846d4084e69a4dc55b87caa81922228875456387d79d874638524e22fd54cc9;
native1 complete/0 prefixes/0 unresolved/empty suffix after completed gate.
Lossless172 files/68 objects/23872 bytes, exact roundtrip and five gate files.
First report valid: one page48129 bytes, all5016 characters read and full-page
visual inbounds. New-source warnings0;7 inherited advisories+1 auditor retained.
Galbraithch9 PDF16–17/36, printed180–181, §9.8/Def9.8.5/Lemma9.8.6/Thm9.8.7
fulltext and individual visual in preceding five-rank study credited exactly;
its short-model/char!=2,3 theorem is not silently generalized. Pinned official
AddSubMap definitions/theorem read; opening TODO stale relative to later proof.
Four correct KB calls and Conductor127 before code; publication Conductor127
before maps/outside pins; bounded fallback/no absence claim. Copyright files
local. Same host/prover/procedural header check; no independent compiler/parser
or physical host assurance. No holes/custom axioms/native proof evaluation/
raised limits/new global flags/waivers/frozen edits/cryptoPython/native crypto
experiment/spend/upload/full fourteen-item completion claim. [pcc:unverified]


The [actual five-torsion rank proof](../five_torsion_rank_audit_20261115/README.md)
now derives25 actual geometric points killedby5,24 points of exactorder5 and an
actual Fin2 basis over ZMod5 for every nonsingular general Weierstrass model
over every algebraically closed field with5 nonzero, including characteristics
2 and3. The literal pinned-library W.preΨ5 point criterion is proved in every
characteristic: actual5P=0 iff Ψ5(x)=0. Actual duplication/equal-x classification
and coprime2/5,3/5 annihilation prove both directions. With D=Ψ2Sq,T=Ψ3,
H=preΨ4,C=6X²+b2X+b4, universal kernel-checked identities H=TC−D²,
U=D²H−T³,Uprime=5D(CH−T²),T²H=D²(CH−T²)−CU derive separability; actual
nonsingularity excludes D/T zeros. The official degree theorem gives12 distinct
roots, actual ordinate fibers have two points each, and an explicit actual
point/fiber equivalence gives12×2+identity=25. Actual annihilation/cardinality
then gives the module rank and basis. No kernel count, rank, separability or
polynomial/group equivalence is supplied as a premise. Thus the general-model
**rank gap at prime5 is closed**; determinant at5, general rank/determinant for
primes≥7, canonical Weil and the full all-k p<500 census remain open.

Credited actual binary/ternary curves now have five-kernel25/rank2. In every
characteristic5 field, the actual curve y²=x³+1 has discriminant3, literal
fifth polynomial evaluating to−1 everywhere and whole five-kernel1. This
refutes deleting coprimality from the rank/count claim. It is not a BK
point-count-divisibility counterexample, a nonreduced group-scheme length or
supersingularity-definition claim. The earlier ordinary characteristic-five
kernel5 control is credited separately. Actual three-torsion/root/ordinate/
fiber methods and official degrees/duplication are credited; established
methods, no novelty or qualified-source refutation claim.

Exact local **PCC-20261115-2cfd10 PASS at 2026-11-15T18:18:29Z**, signed standing unverified. Whole tracked
and transitive gate50 theorems/10 new concrete modules/0 failures/0 trusted native;
12 frozen custom modules,0 copies,15 old formal configurations and31 old pins
checked before/after10 states and gate. Six failures/four successes retained,
including09 internal exception#3 and recovery errors, never accepted. Final10
uses an explicit Unit equivalence and passes. Prior77 gate credited separately.
All28 formal17 artifact45 pins match. Full1281-character header
SHA7cf18bb12120869fac57c17df562ecbdb06329a691fd1ae4446fe7ce97cc10df;
native1 complete/0 prefixes/0 unresolved/empty suffix after completed gate.
Lossless archive197 files/80 objects/23824 bytes, exact roundtrip and five gate
files. First report valid: one page48116 bytes, all5052 characters read and
full-page visual inbounds. Final new-source warnings0;7 inherited advisories
and1 frozen auditor deprecation retained. Fresh Galbraithch9 PDF16–17/36,
printed180–181 §9.8/Def9.8.5/Lemma9.8.6/Thm9.8.7 fulltext and individual visual;
its stated polynomial theorem uses short models/char!=2,3, so this n5 general
extension is proved directly rather than attributed to that passage. Earlier
BK printed142–143 review credited, not broadened. Four correct KB calls and
Conductor127 before processing/code, publication Conductor127 before maps/
outside pins; bounded fallback/no absence claim. Copyright primary files local.
No holes/custom axioms/native proof evaluation/raised limits/new global flags/
waivers/frozen edits/cryptoPython/native crypto experiment/spend/upload/new
physical host/compiler or full fourteen-item completion claim. [pcc:unverified]


The [actual three-torsion tangent pairing](../three_tangent_pairing_audit_20261115/README.md)
is now proved bilinear in both arguments, alternating and nondegenerate on the
actual geometric kernel of every nonsingular general Weierstrass curve in
characteristic other than3, including2. Its explicit value is1 at infinity or
equal x-coordinates; otherwise it is −ℓP(Q)/ℓQ(P), with tangent coefficient1 on y.
Actual outside-domain tangents are proved nonzero. Actual normalization at P
identifies this ratio with the credited cubic character. Both zero radicals,
actual field-map covariance and the coordinate determinant identity are proved.
They force **det(actual q-Frobenius)=q mod3 in every actual basis**, without a
rational fixed point, point-count divisibility or normal-form-family premise.
Thus the general all-model determinant gap **at prime3** is closed. Every
compatible chosen finite field of sizeq^k containing full geometric3-torsion
must satisfy3|q^k−1, without a rational point-count premise. Earlier actual
rank3/basis, normalized character, point-group transport and Frobenius/descent
are credited. This does not identify the tangent formula with a canonical
Weil-function/divisor definition; that comparison and primes≥5 remain open.

Actual controls prove failure after deleting the characteristic guard on the
nonsingular ordinary char3 curve y²+xy+y=x³: a nonzero actual point killedby3
pairs to1 with every actual killedby3 point. Its earlier coordinates/kernel3
are credited. Restriction to the actual cyclic submodule spanned by any
nonzero three-torsion point also gives a nonzero radical. The credited actual
char2 curve y²+y=x³+1 has an actual nontrivial cube-root pairing value. These
are explicit domain controls, not refutations of qualified source theorems.

Exact local **PCC-20261115-8c0b5f PASS at 2026-11-15T17:46:05Z**; signed standing unverified. Whole tracked
and transitive gate43 theorems,10 new concrete modules,0 failures/0 trusted native;
33 frozen custom modules,0 copies,54 old formal configurations and65 old pins
checked before/after13 states and gate. Nine failed states and four successes
are retained. Prior50 gate is credited separately. All67 formal and17 artifact
pins match. Complete2174-character header SHA0315aa3c3afc383f33716cf70c3ccd8fc73b92aaec58f2b5a1d71baaf12990e5;
native1 complete/0 prefixes/0 unresolved/empty suffix after completed gate.
Lossless archive259 files/94 objects/29664 bytes, roundtrip equal; five gate files.
First report valid: one page47626 bytes, all4994 characters read and full-page
visual inspection inbounds. Final new-source warnings0;39 inherited advisories
and1 frozen auditor deprecation retained. Fresh selected ElkiesIII.8 passages
and Galbraithch26 PDF3/16, printed575, were read; the latter also individually
viewed. Formula26.1 has disjoint-divisor conditions, not silently imported as
our pointwise identity. Four correct KB calls and initial Conductor127 before
processing/code; publication Conductor127 before maps/outside frozen pins.
Bounded fallback/no absence or novelty claim; copyright primary files local.
No holes/custom axioms/native proof evaluation/raised limits/new global flags/
waivers/frozen edits/cryptographic Python/native crypto experiment/spend/upload/
new physical host/compiler or full fourteen-item completion claim.
[pcc:unverified]


The [general actual torsion Frobenius bridge](../bk_general_frobenius_audit_20261115/README.md)
constructs the actual r-killed point subgroup and canonical ZModr module for every
naturalr, genuine coefficient pointmaps, and actual q-Frobenius linear maps.
Frobenius is injective over any ambient field extension and an actual linear
automorphism over algebraic extensions of a finite base. Its actual k-power
fixedkernel is linearly equivalent to actual torsion over each compatible
chosen finite field of sizeq^k; finiteness/cardinality follow from actual finite
point encoding. A prime dividing actual base point count supplies a nonzero
actual rational fixedpoint. r1trivial/r0allpoints guards are proved. No geometric
rank2/determinantq/basis/globaltorsion-finiteness premise is smuggled into these
claims. The original BK1998 Remarks1 F19 curve y²=x³+x+6 is checked: actual
count18, generator(0,5) exactorder18/cyclic wholegroup, rational3kernel3,
geometric3kernel9/fixedkernel3, actual Frobenius nonidentity and exactorder3.
The actual division polynomial factors3(X−12)(X³+12X²+13X+9); cubic monic,
degree3 and all19nonroots give irreducibility. Its actual AdjoinRoot field has
dimension3/card6859 and an algebraic embedding/tower into the closure; actual
finitefield power law and point geometry/sign rigidity prove full geometric
3torsion over compatible19^k fields iff3|k. The constructed cubic field has all
nine killedby3 points (including infinity), and positive extensiondegree is at
least3. Source-only determinant19mod3 is derived. This verifies the source's
known exception, not a new refutation/novelty/independent population. Original
qualified BK theorem retained; arbitrary-prime geometric rank2/general detq,
canonical Weil necessity/full all-k p<500 census and remaining14 stayopen.
Fresh primary fulltext and individual visual review: BK1998PDF2–3/5, printed
142–143, DOI10.1007/s001459900040, SHAac837b8edc47b346dbb341f787c41de0402c95371a159f9786b747bb2c987499.
Earlier allr point descent, actual3basis/matrix and Cauchy credited; official
module/Frobenius/fixedfield/AdjoinRoot methods credited. Five fresh correct KB
calls and initial Conductor127 before processing/code; publication Conductor127
before maps/outside pins; bounded fallback/no absenceclaim. Exact local
**PCC-20261115-bec94e PASS at 2026-11-15T17:19:12Z**, signed standing unverified. Fresh59theorems/
10newmodules(9concrete1adapter)/0fail0trusted/fulltransitive foundations;
24frozen localmodules0copies/39oldformalconfigs/55oldpins checked before/after
15states/gate,10failed5success retained. 52formal17artifact69pins. Full2588
header SHA1496e8cfdebbf83c0db95c47a6fd234bcd8e9f29294f9a4ae96b80b88e4be677;
native1complete0prefix0unresolved/empty suffix after completed gate. 300files/
99objects/32344byte losslessXZ/roundtrip equal/fivegatefiles. Onepage47798PDF/
all5064charsread/fullvisual/inbounds, first render valid/no reportdraft. New
sourcewarnings0/38inherited exactbaseline+1auditor advisory retained. No holes/
customaxioms/nativeproofeval/raisedlimits/newglobalflags/waivers/frozenedits/
cryptoPython/nativecryptoexperiment/spend/upload/newphysicalhost/compiler/
canonicalpairing or full14 claim. [pcc:unverified]


The [both-nonzero two-choice refinement](../rho_nonzero_support_audit_20261115/README.md)
derives an actual uniform ordered-distinct support PMF with both increments
nonzero and the actual fair static-vertex-label first-repeat mean from0. Actual
discarded zero-containing support mass2/p and actual0≤tablemean≤p, plus a proved
finite conditional-mean/cardinality bound, give E_all≤E_nonzero+2. The credited
old lowerbound therefore yields E_nonzero≥m²/2−2 atp≥4m²(4m+1),m>0, and≥m²/4
form≥3. For every naturalC some primep>2 has E_nonzero²>Cp; no nonnegative real
uniformsqrtconstant exists. Actual coordinate/nonzeroembedding bijection and
p>2population witness are derived; kernelcheckedp2empty/p3exacttwo-support
controls prevent treating an empty finite sum as a normalizedPMF. This closes
the zero-step qualification for the r2 calibration, not the qualified nS≥8/20
larger-partition conjectures. Established conditioning and preceding actual
maximal/path/counting/deferred-decisions/Bertrand proofs are credited; this is a
domain refinement without novelty or an independentnewcounterfamily claim.
Exact local **PCC-20261115-f8a8f9** replay PASS at 2026-11-15T16:23:38Z; signed standing unverified.
Fresh20theorems/4newconcretemodules/0fail0trusted/fulltransitivefoundations;
17frozenlocalmodules0copies/prior70gate credited.39formal19artifact58pins;
all51predecessorpins checked before/after3states/gate,1failed2success retained.
Full827-character SHA29cb895f96ba2ac7281a8b977ab8fe18e9dadb2d4502b638f4003e4a2bfb51b5/
native1complete0prefix0unresolved/empty suffix after actualgatecompletion.
78files38objects18216byte losslessXZ/roundtrip equal/fivegatefiles; validfirst
reportdraft7files47780byteXZ after prosewrap improvement. Finalonepage47827PDF/
all4665charsread/visual/inbounds. Zero new-source warnings/4inheritedmodule+
1frozen auditor advisory retained. Primary qualifications credited exactly from
preceding GalbraithPDF4,5,12,13/38 and BL11,12/28 fulltext+visual, no broader
reading/originalTeske claim. FourfreshcorrectKBcalls/initialConductor127 before
processing/code, publicationConductor127 beforemaps/outsidefrozenpins. Bounded
localofficial fallback/noabsenceclaim. No Floyd/cyclealone/scalarrecovery/
practicalhashruntime conclusion. Largerpartitionconstants/practicalhash/full14
remainopen. No holes/customaxioms/nativeproofeval/raisedlimits/flags/waivers/
frozenedits/cryptoPython/solverrun/spend/upload/newhost/compiler/full14claim.
[pcc:unverified]


The [two-choice random-support proof](../rho_twochoice_collision_audit_20261115/README.md)
defines the actual first-repeat mean E over uniform ordered distinct increments
in Z/p and independent fair static binary vertex labels, starting at0. Individual
zero increments are allowed; equal increments are excluded. Lean proves
E≥m²/2 whenever p≥4m²(4m+1), and for every naturalC some primep has E²>Cp;
no nonnegative real constant bounds E≤K√p for allprimes. Actual uniform-support
PMF/bijection, iid-word maximal imbalance, chronological paths, exact shortrelation
mass1/p and finite union bound are derived; credited deferred-decisions survival
and postcollision-word counter are retained. This averages the support table;
the prior fixed20 Sidon counterfamily did not. Established methods/prior results
are credited, no novelty claim. This is r2 negativecalibration of an unqualified
extension, not refutation of source-qualified nS≥8/20 conjectures, and not Floyd,
cyclelengthalone, scalar-recovery or practicalhash runtime.
Exact local **PCC-20261115-db512a** replay PASS at 2026-11-15T16:05:42Z; signed standing unverified.
Fresh70theorems/7newmodules(6concrete1adapter)/0fail0trusted/fulltransitive
foundations;10frozenlocalmodules0copies/prior37gate credited.32formal19artifact51pins;
all36predecessor pins checked before/after13states/twogates. Sevenfailedstates/six
successes retained. First native exit0 recorded435characters of actual1134 before
inline letI; it was not accepted as complete. Final explicit primeFact header
1090characters/SHAce864beb518e06e523e89e965cc96ef0d00e05836cae76e1e4d229659cb0d339/
native1complete0prefix0unresolved/empty suffix, after completed finalgate.
245files91objects29268byte losslessXZ/fivefinalgatefiles/roundtrip equal;
valid first report draft7files48140byteXZ, regenerable raster local after initial
oversize archive; premature unissued seal preserved/corrected before producer.
Finalonepage48131PDF/all4973textcharacters read/visual/inbounds; zero new-source
warnings/4inheritedmodule+1frozen auditor advisory preserved. Fresh Galbraith
ch14PDF4,5,12,13/38 and Bernstein–LangePDF11,12/28 fulltext+visual; original Teske
paper unread thisstage. Four correct KB calls/initialConductor127 before processing/
code; publicationConductor127 outside frozenpins. Bounded fallback/noabsenceclaim.
Larger-partition collision penalty/limitingconstants/practicalhash and remaining14
obligations stayopen. No holes/customaxioms/nativeproofeval/raisedlimits/flags/
waivers/frozenedits/cryptoPython/solverrun/spend/upload/newhost/compiler/full14claim.
[pcc:unverified]


The [general residue-weighted length proof](../residue_weighted_length_audit_20261115/README.md)
derives scalar dimension as the actual finite prime sum of canonical local
self-lengths times actual residue-field K-degrees for every finite-dimensional
commutative algebra over a fieldK. It also proves the same formula with actual
Spec structure-sheaf stalk self-lengths and actual stalk residueK-degrees.
Actual coefficient algebra/tower and canonical-prime/stalk-residue K-linear
comparison are derived; no desired decomposition/length/residue compatibility
conclusion is supplied. Artinian/finiteprime/finite-localization data are derived
from finite scalar dimension. Established local-ring extension/composition-series
and Artinian decomposition methods are credited; no novel formula is claimed.
Actual F2×GaloisField(2,2) has scalar dimension3/selflength2/two prime closed
points, complete actual projection-kernel classification, local lengths1,1 and
actual residue algebra isomorphisms/degrees1,2. Weighted3 versus unweighted2
refutes dropping residue weights; the product is not local. These are actual
prime/closed points overF2, not a new base-rational/geometric point population.
Actual splitF2×F2 totals2; known actualJV weighted8/derived canonical residue
 degree1/actualzerochart0 are calibrations, not new populations.
Exact local **PCC-20261115-f28255** replay PASS at 2026-11-15T15:24:00Z; signed standing unverified.
Fresh42theorems/7newmodules(6concrete1adapter)/0failures/0trusted-native/full
transitive standardfoundations;28frozen local modules/0copies/prior21gate credited.
59formal+18artifact=77pins; all67predecessor pins checked before/after nine
states/gate. Six failures/three successes retained, including06default200000
heartbeat timeout; smaller local-equivalence proof succeeded without raisedlimits.
Setup01exact metadata path failure before project mutation retained/setup02correct.
Full2033-character binding/native1complete/0prefix/0unresolved/empty suffix.
185files/73objects/26216byte losslessXZ/five gate files.
Valid first report draft retained8files48740byteXZ after wording clarification;
finalonepage47955bytePDF/all5386textcharacters read/visual/inbounds.
Zero new-source warnings;22inherited module advisories and1frozen auditor warning
preserved. Fresh fulltext+visual DiemPDF18–19/printed91–92 section4.2/Prop4.4–Lemma4.7;
cumulative6–8,18–19/not31. Current JV7/21 and actual F4/JV predecessor credits retained.
Four correct KB calls/initial Conductor127 before processing/code; publication
Conductor127 before maps is outside frozen pins. Bounded official/local/primary
fallback/no absence or novelty claim. General Chow-cycle/intersection comparison,
positive-dimensional isolated components/projective bounds, Rojas C refinement
and the remaining fourteen-item obligations stay open. Qualified Diem n!dⁿ with
multiplicities unrefuted; ordinary cone not its product-projective ambient.
No new solver/curve/order/scalar/decomposition/host/compiler/canonical-promotion/
full14 claim, cryptoPython/raisedlimits/flags/waivers/frozen edits/spend/upload. [pcc:unverified]


The [actual JV scheme-stalk proof](../jv_scheme_stalk_audit_20261115/README.md)
identifies the actual origin structure-sheaf stalk of Spec(A), for the literal
first homogeneous F101 quotient, with its canonical prime localization. The
actual stalk has its own self-module length8 and canonical residue field F101.
The official categorical comparison is retained, and the actual coefficient map
is proved equal to the actual global-section germ map. Actual scheme
nonreducedness follows from the credited nilpotent witness and official affine
reducedness equivalence. Unique point/isolation/cardinality1 are credited
scheme corollaries rather than a new population. Stalk length8 differs from1.
Actual first quotient/localization schemes and principal basic open are empty;
actual second source-root-induced quotient/open/cone points and second basic-open
membership prove the nonempty controls. No comparison, length, locality, residue,
reducedness or germ correctness conclusion is supplied as a premise. Official
comparison/scalar-restriction methods and predecessor73-theorem gate are credited.
Exact local **PCC-20261115-2f14e0** replay PASS at 2026-11-15T14:41:02Z; signed standing unverified.
Fresh21theorems/4newmodules(3concrete+1import-only)/0failures/0trusted-native
exemptions/full transitive standard foundations;24frozen local modules/0copies.
49formal+18artifact=67pins; all59predecessor pins unchanged before/after each
numbered state. Four states/three failures01–03/final04success retained.
Full709-character binding/native1complete/0prefix/0unresolved/empty suffix.
92files/42objects/18148byte losslessXZ/five gate files.
Reportdraft01spill129characters to page2 retained8files49128byteXZ; finalonepage
47974bytePDF/all5464textcharacters read/visual/inbounds. Zero new-source warnings;
12inherited module advisories and1frozen auditor deprecation retained.
Current-session primary JV7/21 and Diem6–8/printed79–81/31 reads credited; Diem
text reread and official scheme/stalk/reducedness APIs checked. Four correct KB
calls/initial Conductor127 before processing/code; publication-scope Conductor127
before living-map edits recorded outside frozen pins. Bounded local/official
fallback/no absence/novelty claim. The actual scheme-stalk comparison gap for
this concrete quotient is closed. General cycles/multihomogeneous intersections,
positive-dimensional isolated components/projective bounds/RojasC and the other
fourteen-item obligations remain open. Explicit localized-Spec/basic-open scheme
isomorphism is not constructed. Qualified Diem bound unrefuted; ordinary cone
not its product-projective instance. No second root uniqueness/whole-curve order/
scalar/decomposition/newsolver/host/compiler/canonical-promotion/full14 claim,
cryptoPython/limits/flags/waivers/frozen edits/spend/upload. [pcc:unverified]


The [actual JV local-multiplicity proof](../jv_local_multiplicity_audit_20261115/README.md)
derives self-module length8 and locality for the literal first homogeneous F101
quotient, unique isolated origin prime, canonical origin localization with its
own self-module length8, and actual origin/localized residue fields F101.
No locality, length, isolation, residue or evaluation-correctness conclusion is
supplied as a premise. Actual basis tails form a strict eight-step ideal flag;
scalar restriction supplies the upper bound. Actual h³≠0/h⁴=0 proves nonreducedness.
The first actual h=1 quotient and principal-open localization are zero rings,
self-length0 and not nontrivial local. The second literal source triple has the
actual F101 root(69,75,1); actual descended quotient/open evaluation maps prove
both charts nontrivial, without uniqueness, whole-curve/order/scalar/decomposition
claims. The cone pointcard1 over every characteristic101 field is explicitly a
corollary of the credited all-field zero characterization. The actual ring length
is8 despite one distinct point. The ordinary cone is not Diem's product-projective
multihomogeneous setup; qualified n!dⁿ with multiplicities remains unrefuted.
Exact local **PCC-20261115-2e7ce0** replay PASS at 2026-11-15T14:21:50Z; signed standing unverified.
Fresh73theorems/6new modules/0failures/0trusted-native exemptions/full transitive
standard foundations;18frozen dependency modules/0copies/prior gates26 and28
credited separately.42formal+17artifact=59pins. All61predecessor pins unchanged
before/after eleven numbered states; six failures/five successes retained.
Full935-character binding/native1complete/0prefix/0unresolved/empty omitted suffix.
206files/86objects/32736byte losslessXZ/five gate files;
one-page48087byte PDF fully read, visually inspected and inbounds.
Zero new-source warnings;12inherited module advisories and1frozen auditor
deprecation preserved. Fresh fulltext+visual JV PDF7/21 and DiemPDF6–8/printed79–81
of31; no whole-paper claim. Eight exact historical native files retained; raw
GB sizes6/1/2 are not a direct multiplicity8 certificate. No solver rerun.
Five correct KB calls/initial Conductor call127 before processing/code; second
publication-scope Conductor127 before living-map edits is outside the frozen
packet. Bounded primary/official/local fallback; no absence or novelty claim.
Actual Scheme localRing/cycle comparison, general multihomogeneous intersection,
positive-dimensional isolated components/projective cycles, Rojas C refinement
and the remaining fourteen-item obligations stay open. No new host/compiler
independence, canonical promotion, cryptoPython, limits/flags/waivers/frozen edits,
paid compute or agent uploads. [pcc:unverified]


The [actual Miller level-compatibility proof](../miller_level_compatibility_audit_20261115/README.md)
proves the chosen-function and final-evaluation core of Galbraith Exercise26.3.8.
For every field with actual DecidableEq data and nonzero discriminant, actual
nP=O implies f_(a*n)=f_n^a, including characteristics2/3 and zero/one extensions.
The actual final local-unit subgroup derives the higher domain from the lower
one; actual divisor evaluation gives f_N(D)=f_n(D)^(N/n) for n|N. For positive
n,N and n|N|M, final exponents M/n and M/N agree. No function-correctness or
evaluation-commutation premise is supplied. Canonical Tate construction,
representative independence and reciprocity remain separate. No claim is made
that the source omitted its implicit first-argument torsion domain.
Both actual controls use E/F5:y²=x³+1, n2/N4/M4 and actual field cardinality5.
Positive P=(4,0), exact order2, D=(0,1)−(2,2): proper values2/4 and final4=4≠1.
Negative P=(0,1), exact order3/2P≠O, D=(2,3)−(4,0): proper values1/3 and final1≠3.
Both degreezero divisors and all four full geometric final domains over actual
AC(F5) are proved. Actual f4≠f2² in the negative control, with actual orders0
versus−2 at(0,4). Thus deleting torsion fails while the qualified source core
survives. Values come from actual coordinate fractions/local residues, not an oracle.
Exact local **PCC-20261115-29cf72** PASS at2026-11-15T13:08:48Z; signed standing unverified. Fresh gate
55theorems/6newmodules/0failures/0trustednative/fulltransitive standard
foundations;87frozen dependency modules/0copies/previous679audit credited
separately. All90 dependency source/config files and9 new formal/config files
plus18artifacts=117pins. All107 predecessor pins checked before/after each
numbered build. Full3589-character binding/native1complete/0prefix/0unresolved.
Tenstates/sevenfailures/threesuccesses;185files/71objects/
29952byte losslessXZ/fivegatefiles/PDF47533fulltextread/visual/inbounds.
Final10project clean; inherited frozen-auditor String.mk deprecation retained.
Ch26 fresh fulltext+visualPDF7/printed579, cumulative1–7/notall16; Lemma26.3.11
proof continuationPDF8 and original Miller paper unread. Five correct KB calls/
two Conductor calls127 beforeprocessing/edits; bounded local/official/primary
fallback and existing local dependency pattern credited. No novelty/canonical
promotion/newhost/backend/runtime/frozenedits/cryptoPython/raisedlimits/global
flags/waivers/spend/upload. General scheme/divisor basechange/descent,
translation/reciprocity/isogeny/Lang/Cartier/Jacobian/canonical Weil–Tate/MOV–FR
and the remaining fourteen-item obligations stay open. [pcc:unverified]


The [actual binary Miller audit](../binary_miller_audit_20261115/README.md)
proves the carried point/function state is (nP,chosen f_n) for every field with
actual DecidableEq operation data and actual nonzero discriminant, including
characteristics2/3 and the source zero/vertical branches. The implementation
carries the point; it does not recompute scalar multiples inside each iteration.
Exactly Nat.log2(n) bit rounds and at most that many conditional additions are
proved as structural counts. Actual numerator/denominator principal-support
unit guards generate a carried mathematical numerical execution relation.
Every recursively admissible input has a run for every n; every run erases to
the actual binary state and its numerical recurrence equals actual divisor
evaluation. No correctness/commutation premise is supplied. A concrete numerical
backend/compiler/runtime refinement remains separate.
On actual E/F5:y²=x³+1, P=(0,1) has exact order3. For D=(0,4)−(4,0), degree0
and actual full geometric final support disjointness hold over actual AC(F5).
The normalized chosen f3=y−1 has actual proper value2. Intermediate f2=(y−1)/x
has actual order−1 at Q=(0,4), also after geometric embedding. Strict numerical
trace rejects and the totalized raw trace returns0≠2; the generic trace guard
fails. The correct final sum-zero vertical multiplication by x is retained.
Safe D=(2,2)−(4,0) satisfies each actual factor guard and the generic trace
domain; strict/proper final values agree at4. The literal unconditional numerical
Algorithm29 domain needs intermediate-factor guards. Qualified function-field
recursion/guarded numerical algorithm survive. Original Miller paper unread;
no novelty/original-paper/canonical-pairing/runtime refutation is claimed.
Exact local **PCC-20261115-16a863** PASS at2026-11-15T12:31:28Z; signedunverified. Whole679/0fail/
0trusted/standardfoundations;87modules76exactcredited+11new/107pins.
Full2123-character binding/native1complete/0prefix/0unresolved.
Sixteenstates/sixfailures/tensuccesses;210files/95objects/
29656byte losslessXZ/fivegatefiles/PDF47749fulltextread/visual/inbounds.
All failures/advisories retained; final16project clean, inherited frozen-auditor
String.mk deprecation. Ch26 fresh fulltext+visualPDF6/printed578 and partialPDF5,
credited cumulative1–6/notall16. Six correct KB calls/two Conductor calls127
beforeprocessing/edits; bounded fallback. Prior actual geometry/evaluation and
official log2/order APIs credited. No holes/customaxioms/nativeproofeval/raised
limits/globalflags/waivers/frozenedits/cryptoPython/newhost/spend/upload.
General scheme/divisor basechange/descent, translation/reciprocity/isogeny/Lang/
Cartier/Jacobian/canonical Weil–Tate/MOV–FR and full14 remain open. [pcc:unverified]


The [actual local/divisor evaluation audit](../finite_point_evaluation_audit_20261115/README.md)
constructs actual finitepoint-local residue evaluation/kernelmax/residueFieldK/unitcriterion,
and applies the official DVR integer-lift theorem to the actual proved point DVR/full
function field to identify the actual regular domain with the official finite adic
valuation subring. Actual finite/infinity maps give residue evaluation at every
actual point, coefficient identity and nonzero value iff valuation1, with actual
nonzero-function order0 iff valuation1. Actual coordinate fractions have numerical
value pointEval(a)/pointEval(b) under the nonzero-denominator guard; actual unit
values agree. The actual principal-support disjointness/local-unit-domain bridge
is proved. Actual unit-valued divisor products satisfy function multiplicativity,
common-support divisor additivity, the factor c^degree(D) and degreezero scalar
invariance. Prior actual geometric scalar classification gives choice invariance
for equal geometric principal divisors under actual IsAlgClosedK/WΔneq0/degree0/
disjoint guards: the elliptic geometric forward Exercise26.1.1 case, not its converse,
allcurves or reciprocity. Actual nonzero X−x precludes total coefficientfixing
FF→K point evaluation; its inverse has no local representative and adicvalue>1,
while its actual local value is0. Singleton±1divisors cancel, but totalizedzero
negative powers break the product/add law. On actual prior E/F3:y²=x³+x+1,
P=(0,1)NS/Δneq0, the vertical has order1atP/−2atO; singleton and actualprincipal
supports are unequal yet overlap. The intended disjoint source qualification
survives; merelyunequal or deleted-domain readings fail. No arbitrary evaluation
outside the tested singleton is inferred. Full scheme/localRing/Divisor APIs,
finitepoint divisor basechange/tensor/Galois descent, translation/reciprocity/
isogeny/Lang/Cartier/Jacobian/canonicalpairing/MOVFR/binaryruntime remainopen.
Exact local **PCC-20261115-1a1570** PASS at2026-11-15T11:01:06Z; signedunverified. Whole536/0fail/0trusted/
standardfoundations;76modules67exactcredited+9new/96formal-artifactpins;
full3887-character native1complete/0prefix/0unresolved. Fourteenstates/fivefailures/
ninesuccesses;174files/81objects/25172byte losslessXZ/
fivegate/PDF47985fulltextread/visual/inbounds.13four namedlogicalguard advisories;
14allguards retained anonymously/projectlogs clean, inherited frozenauditor
String.mk deprecation retained. Failedcompilerplaceholders are diagnostics,
notacceptedproofs. Ch26newfulltextvisualPDF1/printed573/cumulative1–6/notall16;
ch7partial1–72rereadwithinpriorcoverage/notall21. OfficialDVR/unit/Finsupp APIs
andprioractualgeometry/scalarcriterion credited. FivecorrectKBcalls127beforequery
processing/twoConductor127fallback; datedprecodeprotocolrefinements/samebudget.
No novelty/canonicalpromotion/newphysicalhost/frozenedit/cryptoPython/spend/upload.
[pcc:unverified]


The [actual Miller basechange audit](../miller_basechange_audit_20261115/README.md)
constructs an injective actual function-field embedding under field extensions,
proves actual norm/infinity valuation and leading-residue-one compatibility,
and actual point/step/every-n chosen-function compatibility. Proper
GeometricMiller includes actual discriminant nonzero and actual function/point
images in actual AlgebraicClosureK with the full geometric divisor. Actual
constructed functions satisfy it; any proper function equals the chosen base
function. Exact chosen i+j addition descends to every base field, including
characteristics2/3 and zero/vertical branches. The credited prior normalized
nonconstant F3 ratio times every chosen function satisfies our old rational-only
predicate, differs from the chosen function, fails the proper predicate and has
the wrong actual embedded geometric divisor for every actual P,n. n0/n1 equal
the ratio; its actual embedded geometric divisor is nonzero. Our weak definition
is false; qualified source Definition26.3.4/Exercise26.3.5 survive. Full finite-point/
local-divisor/scheme/tensor basechange, Galois descent, evaluation/translation/
reciprocity/isogeny/Lang/Cartier/Jacobian/canonical pairing/MOV-FR/binary runtime
remain separate. Exact local **PCC-20261115-b11451** PASS at2026-11-15T10:06:31Z; signedunverified.
Whole471/0fail/0trusted/standardfoundations;67modules58exactcredited+9new/
70formal-config+17artifactpins;full2145-character complete native binding.
Initial gate471accepted the full valid theorem but exporter recorded161of2253
characters; native parser exit0 reported0complete/1prefix. Transparent positional
aliases retained every hypothesis/conclusion; final native1complete/0prefix/
0unresolved, both source/export/gate/native states retained. Frozen verifier/
keyring unchanged; existing prefix study credited. Tenstates/fourfailures/
sixsuccesses;153files/73objects/23492byte losslessXZ/
fivefinalgate/PDF47596fulltextread/visual/inbounds. Final10projectlogs clean;
04two unused-simp advisories and inherited auditor String.mk deprecation retained.
Ch26 exact partial reading and ch7 additional partial text scope recorded;
not whole chapters. Official coordinate/point/fraction/norm maps and prior
geometric uniqueness credited. Four correct KB calls127 before queryprocessing,
Conductor127 fallback. No novelty/canonicalpromotion/newphysicalhost/frozenedits/
cryptoPython/spend/upload. [pcc:unverified]


The [actual geometric principal-divisor audit](../geometric_principal_divisor_audit_20261115/README.md)
proves degreezero for every actual nonzero elliptic rational function under
actual IsAlgClosed K and nonzero-discriminant guards, including characteristics2/3.
Actual polynomial factorization/vertical divisors and actual full-field norm/
negation give integer degreezero without a supplied degree premise or division
by2 inK. Actual infinity residue/local-ring maps plus no-poles/degreezero give
no-poles iff nonzero constant, zero actual divisor iff nonzero constant and
equal actual divisors iff a nonzero scalar multiple, without assuming global
regularity classification. Leading-residue-one normalization gives actual
uniqueness, any valid Miller function equals the chosen function and exact
chosen all-n i+j addition holds in every zero/vertical/nonvertical branch.
These conclusions retain geometric scope. On actual E/F3:y²=x³+x+1,
the nonzero polynomial X²+1 has rational-only divisor−4(O), degree−4.
The credited prior normalized nonconstant zero-rational-divisor ratio versus1
reconstructs a counter to rational-only normalized uniqueness. An actual mapped
elliptic model over actual AlgebraicClosureF3 has Δnonzero/verticaldegree0,
realizing the geometric hypotheses. Elliptic geometric cases of source
Theorem7.7.11/Cor7.7.13/7.7.14 survive, not the all-curve theorem.
General function-field/divisor basechange-descent, separate Scheme.localRing/
Divisor APIcomparison, translations/reciprocity/isogeny/Lang/Cartier/Jacobian/
canonical pairings/MOV-FR/binary runtime and full14 remain open.
Exact local **PCC-20261115-9eb5c1** PASS at2026-11-15T08:50:29Z; signedunverified. Whole428/0fail/0trusted/
standardfoundations;58modules47exactcredited+11new/61formal-config+15artifactpins;
full1883-character binding87af23f53b8644b39d35a8d39e7b83e65d5a5c4e15b785494cadc5bab49e10dd/native1complete/0prefix/0unresolved.
Tenstates/fivefailures/fivesuccesses;156files/68objects/
20220byte losslessXZ/fivegate/onepagePDF47263fulltextread/visual/inbounds.
09seven named-binder advisories retained;10logicalguards anonymous/proofnamed
below, no hypothesis deleted/waiver/flag.10projectlogs clean, inheritedauditor
String.mk deprecation retained. Full elliptic7.7.11proof/Cor7.7.13/14/PDF15–16/
printed137–138 text reread, cumulative partial chapter coverage credited,
notall21pages. Established source norm/regular-function methods and official
roots/Finsupp/local-residue APIs credited. KB8calls127(initial4wronginterface,
corrected4documentedCLI; no arguments/query processed), Conductor127 fallback.
No novelty/canonicalpromotion/independentphysicalhost/frozenedits/cryptoPython/
spend/upload. [pcc:unverified]


The [actual negation pullback audit](../negation_pullback_audit_20261115/README.md)
constructs the actual coordinate involution, full FunctionField automorphism and
actual point-localization isomorphism. Actual evaluation identifies allowed
denominators; actual maximal ideals and uniformizer generators transport.
The earlier actual nonsingularity-derived DVR/full fraction field and actual
unit-times-integer-power theorem derive the official normalized finite-adic
formula v_P(iota*f)=v_(-P)(f) for every actual function, without a valuation-
invariance premise. Actual norm preservation handles infinity; every actual
point order and finite-support principal divisor consequently transports by
actual point negation. The actual algebra norm equals the product with actual
conjugation, including the full-field fraction identity. On the actual
nonsingular F5 curve y²=x³+1, the actual nonzero tangent at actual3torsion
P=(0,1) has order3atP and0at−P. This disproves deleting function pullback;
the correct pulled-back function has order0atP. Qualified Galbraith
Lemma7.7.12 survives. Involutive means squareidentity, not exactorder2 for
every general model/characteristic. Actual nonsingularKpoints include all
geometric points for an elliptic model over algebraically closedK.
Geometric degreezero/constantkernel/uniqueness, algebraicclosurebasechange,
separate Scheme.localRing/Divisor APIcomparison, translation/reciprocity/
isogeny/Lang/Cartier/Jacobian/canonical pairings/MOV-FR/full14 remain open.
Exact local **PCC-20261115-421c18** PASS at2026-11-15T08:05:52Z; signedunverified. Whole381/0fail/0trusted/
standardfoundations;47modules39exactcredited+8new/50formal-config+15artifactpins;
full1280-character bindingaeb39697095990a68ba9433163f8b7fa516850bef89f64e69f2b1a4ecac6ed1e/native1complete/0prefix/0unresolved.
Eleven retainedstates/sixfailures/fivesuccesses;97files/42objects/
15592byte losslessXZ/fivegate/onepagePDF47090fullread/visual/inbounds.
Final11/projectlogs clean; inheritedauditordeprecation retained. Raw01source/log/
exit retained; initial01config explicitlyreconstructed from02/single-targetedit,
not falsely presented as raw. Setupreceipt summaryrecordedafterobservedexit0.
Full sourceLemma7.7.12 proof/PDF16/printed138 freshlytext-read; exact priorvisual
and cumulativepartialchaptercoverage credited, notall21pages. Official
AdjoinRoot/localization/DVR/norm APIs and establishedQuadraticAlgebra
conjugation credited; KB/Conductor127 explicitfallback. No novelty/canonical
promotion/independentphysicalhost/frozenedits/cryptoPython/spend/upload.
[pcc:unverified]


The [actual general-function divisor audit](../function_divisor_finiteness_audit_20261115/README.md)
now derives finite nonzero actual local orders for every nonzero actual
Weierstrass rational function, not only constructed Miller functions. Actual
evaluation zero implies actual nonzero norm-polynomial root; finite roots and
monic quadratic fibers give finite affine zero sets. Actual nonvanishing local
evaluation gives an actual local unit/official adic value1. Actual fractions
therefore have support in numerator zeros/denominator zeros/infinity. General
actual Finsupp principal point divisors/product-quotient laws/function-unit
additive divisor homomorphism/principal subgroup range and all-n Miller
compatibility are proved with no supplied finiteness premise.
The actual nonsingular F3 curve y²=x³+x+1 has actual function
f=(X²+1)/(X²+X+2), nonzero and nonconstant by actual polynomial embedding and
coefficient comparison. All actual F3-rational point orders, including infinity,
are0; the actual rational-point divisor is0 and the function is infinity-normalized.
This refutes dropping geometric-point scope from zero-divisor-implies-constant;
the source uses bar-k geometric points and its qualified statement is not refuted.
Over arbitraryK these are nonsingularKpoint divisors; an elliptic model over
algebraically closedK includes all geometric points. Geometric degreezero/
constantkernel/uniqueness, algebraicclosurebasechange, separate Mathlib
Scheme.localRing/Divisor APIcomparison, translation/reciprocity/isogeny/Lang/
Cartier/Jacobian/canonical pairing/MOV-FR and complete14 remainopen.
Exact local **PCC-20261115-c81d29** PASS at2026-11-15T07:24:51Z, signedunverified. Whole334/0fail/0trusted/
standardfoundations;39modules33exactcredited+6new/42formal-config+14artifactpins;
full1029-character bindingf4c0c971036efced202ba6da453857cdaaaa70f6e116619f5839ce5d10ad1b1b/native1complete/0prefix/0unresolved.
Five snapshots/twofailures/threesuccesses, 220files/63objects/
36076-byte losslessXZ/fivegate/onepagePDF46987fullread/visual/inbounds.
Final05/projectlogs clean, inheritedauditordeprecation recorded. Galbraithch7
Theorem7.7.1planeproof andfreshPDF13/16text/visual, prior exactnorm/valuation
proofs credited. Full elliptic7.7.11/Lemma7.7.12/Cor7.7.13/7.7.14 newlytext-read
ascontext afterprecedingpacketissue, notformallyproved; notall21pages.
No novelty/canonicalpromotion/independentphysicalhost/frozenedits/cryptoPython/
spend/upload. [pcc:unverified]


The [actual all-n Miller-function audit](../miller_recursion_audit_20261115/README.md)
constructs actual nonzero rational functions for every n and actual nonsingular
point, with actual finite-support point divisor n(P)−([n]P)−(n−1)(O).
The actual step is1 at a zero input, vertical for negative points, and actual
chord/vertical otherwise. Its exact actual local-valuation divisor is proved
in every branch; induction gives all-n source divisors and the n-torsion
specialization n(P)−n(O). Source leading-residue-one normalization uses actual
X/Y and the actual infinity valuation, with proved product/quotient closure.
Any valid normalized i,j functions yield a valid normalized i+j function;
constructed f1=1. Chosen-function uniqueness and binary-program/runtime
refinement are not inferred. Actual F5 nonzero T=(4,0), T+T=O, has correct
vertical step infinity order−2; unsafe ordinary chord has pole−3 and the
actual prescribed chord-divisor assertion is false. Source Algorithm29
explicitly requires the exceptional simplification; qualified source survives.
Over arbitraryK these are nonsingularKpoint divisors; elliptic model over
algebraically closedK includes all geometric points. SeparateMathlib
Scheme.localRing/Divisor APIcomparison, general all-function divisor finiteness,
translation/reciprocity/isogeny/Lang/Cartier/Jacobian/canonical pairing/MOV-FR
and complete14 remainopen. Exact local **PCC-20261115-529612** PASS at2026-11-15T06:52:24Z, signedunverified.
Whole285/0fail/0trusted/standardfoundations;33modules27exactcredited+6new,
36formal-config+14artifactpins; full1244-character binding3be8f68d63b18ecd7bf18c28c045b53530c90948403c38146513bc3bcdb1d3bd/native1complete/
0prefix/0unresolved. Sevenstates/twofailures/fivesuccesses, 261files/
64objects/32952-byte losslessXZ/fivegate/onepagePDF46939
fullread/visual/inbounds; final07/projectlogs clean, inheritedauditordeprecation
recorded. Ch26PDF2–6/printed574–578 text/visual scoped, notall16pages; Def26.3.4/
Ex26.3.5/Algorithm29/Ex26.3.6 and exactofficial/prior proofs credited. No novelty/
canonicalpromotion/independentphysicalhost/frozenedits/cryptoPython/spend/upload.
[pcc:unverified]


The [actual finite-point local-ring and divisor audit](../miller_finite_point_audit_20261115/README.md)
now derives actual nonsingular-point local DVRs, full actual FunctionField fraction
rings and official normalized adic valuations. Actual evaluation kernel is the
coordinate point ideal; the exact shifted equation and a nonzero partial derive
the local uniformizer/DVR rather than assuming regularity. Actual point ideals
localize to the maximal ideal at the same point and to the unit ideal elsewhere.
The proved actual chord/negative-pair ideal equalities yield every finite zero
order, including repeated intersections. Combining the actual infinity orders
proves finite support and actual Finsupp principal point divisors:
line=(P)+(Q)+(−P−Q)−3(O), vertical=(P)+(−P)−2(O), and actual three-torsion
tangent=3(P)−3(O). Actual F5 controls show X−4 is not a local uniformizer at
(4,0), while the ordinary tangent at(2,2) has order2 there and another simple
zero at(0,1), with actual divisor unequal to3(P)−3(O). Countercontrols delete
hypotheses; the qualified source statements survive. Over arbitraryK these
Finsupps are on nonsingularKpoints; for an elliptic model over algebraically
closedK they include all geometric points. Separate Mathlib projective
Scheme.localRing/Divisor interoperability, general all-function divisor
finiteness/general-n Miller recursion, translation/reciprocity/isogeny/Lang/
Cartier/Jacobian/canonical pairing/MOV-FR and all14 completion remain open.
Exact local **PCC-20261115-bb5771** PASS at 2026-11-15T06:18:06Z, signed unverified. Whole224/0fail/0trusted/
standardfoundations;27modules=19byte-identicalcreditedcores+8new; full1652-character
binding140ca58f39c177a432e21f1aee49ae26a9e3075c50cced761301751b1db04b09/native1complete/0prefix/0unresolved;
30formal-config+14artifactpins. Twelve snapshots/seven failures/five successes,
344files/69objects/31408-byte losslessXZ/fivegate/one-page47123-bytePDF fully
text-read/visual/inbounds. Final12/projectlogs clean; auditoroneinheriteddeprecation.
Primary Galbraith chapter7 finite-point Example7.3.4/Lemma7.7.10 proof freshly
text-read, prior scopedvisuals/Def7.7.2 retained, not all21pages/fullTheorem7.7.11.
Chapter26 and officialMathlib/prior proof credits retained. No novelty/canonical
promotion/newphysical-hostvalidation/frozenedits/cryptoPython/spend/upload.
[pcc:unverified]


The [actual infinity chart localization audit](../miller_infinity_chart_audit_20261115/README.md)
now identifies the actual origin-localized chart-function image algebra in
X/Y and1/Y with the norm-degree DVR by an explicit RingEquiv preserving actual
function-field inclusion. Unique/surjective origin evaluation/ker(t,z), actual
Noetherian local DVR/maximal ideal(t), full actual FunctionField fraction-ring
identifications and the explicit inverse are derived, not assumed. The official
normalized maximal-ideal adic valuation equals norm degree everywhere, giving
source-sign line/vertical pole orders−3/−2, correct normalization and the
reciprocal-X substitution countercontrol. The actual local residue field is
explicitly K, with exact evaluation formula. This closes the specified actual
chart-function/localization/adic-order comparison left by the preceding packet;
a separate Mathlib projective Scheme.localRing comparison is not asserted.
**Full finite-point principal divisor, translation/reciprocity/isogeny/Lang/
Cartier/Jacobian/canonical pairing/MOV-FR and all14 completion remain open.**
Exact local **PCC-20261115-6a306d** PASS at2026-11-15T05:37:49Z, signed unverified. Whole129/0fail/0trusted/
standardfoundations;16modules=7byte-identicalcreditednormcores+9new; full1033-char
binding0bae769dea940a972891b819c4a49162b9aa2189ca7156fc5fa387f267ec81f5/native1complete/0prefix/0unresolved;
19formal-config+14artifactpins. Fifteen snapshots/eight failures/APIprobe/two
protocols,290files/69objects/25340-byte losslessXZ/fivegate/one-page46953-bytePDF
fullread/visual/inbounds; final15/projectlogs clean, auditoroneinheriteddeprecation.
Galbraithchapter7 PDF1–2,7–10,14–15 fulltext/PDF2,8,9,15 visual, not all21pages;
infinity subsection of Example7.3.4 verified, not fullTheorem7.7.11. Existing
chapter26/reusedofficialMathlib arguments credited. No frozenedits/novelty/
canonicalpromotion/independenthostvalidation/spend/upload. [pcc:unverified]


The [actual Miller norm valuation/DVR bridge](../miller_norm_valuation_audit_20261115/README.md)
constructs a valuation on the actual curve function field from the actual rank-two
algebra norm degree, deriving multiplicativity, ultrametricity and zero support.
Actual X/Y is a uniformizer of its actual DVR and generates its maximal ideal;
values exp2 and exp3 for X and Y, exp3/exp2 for lines/verticals, and exp(-1)
for the parameter are derived. Actual normalized lines/verticals reduce to1
under the actual residue map. Reciprocal X is not a uniformizer and its
substitution makes normalized-line difference value1. No valuation, DVR,
coordinate-weight or normalization premise is supplied. **Canonical projective
infinity local-ring identification, full canonical principal divisor and
pairing/MOV-FR/all14 completion remain open.** This advances the constructed
valuation/normalization portion, not that geometric identification.
Exact local **PCC-20261115-c0e6db** PASS at 2026-11-15T04:43:54Z, signed unverified. Whole gate57/0fail/
0trusted/standardfoundations; full1108-character binding9e902bb2a63699a532e54ad1d475b5c8e1fb224ab597a92cc49f98ee0ca43dc9;
native1complete/0prefix/0unresolved;7newmodules/10formal-config+14artifactpins.
Fourteen states/nine failures/success03,05,08,12,14,151files/54objects/15848-byte
losslessXZ/fivegate/one-page46894-bytePDF fullread/visual/inbounds. Final14/project
logs clean; auditoroneinheritedString.mkdeprecation retained. Four exit-file
writes skipped by set-e were recovered from actual retained PTY completions,
eachactualexit1, without rerunsmanufacturingrecords. Official norm-degree and
DVR/uniformizer proofs credited; samehost/prover/Mathlib, not independentphysical
validation/newprimarypapercoverage/novelty/canonicalpromotion. [pcc:unverified]


The [actual Miller affine ideal bridge](../miller_principal_ideal_audit_20261115/README.md)
proves the generic coordinate-ring chord principal ideal and actual three-torsion
tangent cube, nonzero regular/function-field line, and actual ring-hom evaluation
agreeing with the numerical short-model Miller tangent expression. Actual F5
positive and negative controls prove the three-torsion guard is necessary.
This closes only the affine ideal/function/evaluation portion beyond the earlier
polynomial certificates; projective infinity valuation, canonical divisor/pairing,
reciprocity/isogeny/Lang/Cartier/Jacobian/MOV-FR and all14 completion remain open.
Exact local **PCC-20261115-357b60** PASS at 2026-11-15T04:16:36Z, signed standing unverified. Whole gate
25/0fail/0trusted; full1470-character binding d2b0fd734afc3313003d0373976189f696193cbf25807d2b3b20610645ef40ce;
native1complete/0prefix/0unresolved;7formal-config+14artifactpins. Eight snapshots
retain five failed states, successes04/07/08 and the attempt03 path error.
100files/44objects/63732-byte losslessXZ/fivegatefiles; corrected one-page46797-byte
PDF fully read/visually inspected/inbounds, first draft retained. Project08/build
logs are warning-free; frozen audit tool emits one inherited String.mk deprecation.
Four new modules use no prior custom proof modules, same prover/Mathlib/host,
not independent physical-host validation. No canonical promotion/novelty claim.
[pcc:unverified]

Diagnostic correction: the [outside-pin receipt](audit-tool-diagnostics-20261115.json)
records one inherited String.mk deprecation warning in each of seven previous
frozen audit logs. Any broad earlier claim that all gate logs were warning-free
is superseded. Existing project-source S3/Galois warnings remain separately
recorded; theorem verdicts/counts/axioms and all frozen files are unchanged.
[pcc:unverified]


The [known composite Tate control reconstruction](../composite_tate_domain_audit_20261115/README.md)
rechecks the F5/n6 result already covered by PCC-20261110-5e63e9/gate50;
**no new source coverage or original fourteen-item gap closure** is claimed.
Actual kernel/rightquotient/unitpowerquotient cardinalities6/6/2, both explicit
nonzero3-torsion radicals and the literal nontrivial-root counterconsequence
are proved, with generic prime-root arithmetic. Four new modules import no
priorcustomproofmodules, but share prover/Mathlib/host; this is not independent
implementation/physicalhost validation or the original degree negativecontrol.
The duplicate was detected at05 after too-narrowinitialpriorsearch, userinformed,
protocolamended; all states retained. Exactlocal **PCC-20261115-e2a305** PASS at2026-11-15T03:43:24Z, signed
unverified; whole53/0/0/full738-character bindingc0b96f82…/native1complete/0prefix/
0unresolved;7formal-config+14artifactpins;109files/46objects/16304-byte losslessXZ/
fivegatefiles/one-page47274-bytePDF fullread/visual/inbounds. Eight snapshots/six
failed states/APIprobe/threeprotocolsamendments retained, final08/gate full logs
no warnings/errors. Qualified generalTate/canonicaldivisor/MOVFR and all original
remaining gaps unchanged. No novelty/canonicalpromotion/newnativecrypto/spend/
upload. [pcc:unverified]


The [actual characteristic-five BK control](../bk_char_five_geometric_audit_20261115/README.md)
disproves the unrestricted geometric-point inclusion paraphrase. Actual
E/F5:y²=x³+3x+2 is nonsingular, has actualcount5 and four explicit nonzero
points of exactorder5. For every extensionK/F5, all actual5-torsion points
lie in the actual Point.map image of E(F5), with a whole-kernel bijection
and actualcardinality5. Thus the entire geometric point kernel over the actual
algebraic closure is rational over F5, while5 doesnotdivide5^k−1 for anyk>0.
Prime5 dividesactualbasecount and doesnotdivideq−1. The proof derives x1/2,y±1
using two actual repeated-S3 relations, a kernelchecked eliminant factor and
coprime3/5 control, then actual point-map descent/cardinality. No torsion rank,
point count or canonical fifth-division-polynomial identification is assumed.
**The original BK r²-count criterion survives**: actualkernel5≠25 and5 doesnotdivide
actualfiniteextensioncardinality−1, so both sides arefalse in everyactualfinite
extension. No extra characteristic hypothesis is attributed to the source.
This concerns the geometric point kernel, not the nonreducedp-torsion group scheme.
Prior characteristic3 analog credited; no novelty claim.
Exactlocal **PCC-20261115-e8c175** PASS at2026-11-15T03:04:24Z, signedstandingunverified. Whole61/0/0,
complete644-character bindinga625348a8083ec690a7a7b8d5f09b3330f7df44e6f814806b1bd4b61e14b4ee3,
independentnative1complete/0prefix/0unresolved. Sevenmodules/twoadaptedpriorcores
plusfivenew; tenformal-config+15artifactpins. Fifteensnapshots/13failedstates/
successes09,15/failedAPIprobe/pre-editmoduleamendment retained;189files/65objects/
27544-byte losslessXZ/fivegatefiles plusfourinitialreportdraftfiles in48016-byte
separateXZ. Finalone-page47593-bytePDF fullyread/visual/inbounds. Fullfinal15 logs:
exactlyseveninheritedS3argumentadvisories, zero new-modulewarnings/errors.
Original mathematical argument bodies unchanged; frozen originals untouched.
Exact originalBK printed142–143/PDF2–3 freshlyfulltextread/visual, no fresh
full-five-page read claimed. Generalprime≥5/canonicalWeil/rank/determinant source
proof/full-populationallkcensus/other14 gaps remain. No canonicalpromotion/
full14closure/cryptoPython/nativeproofeval/limits/flags/waivers/spend/upload.
[pcc:unverified]


The [general finite-field cyclotomic proof](../finite_cyclotomic_degree_audit_20261115/README.md)
advances items2/3/4. For every actual finite field K and natural r nonzero in K,
the field adjoining all rth roots inside AlgebraicClosureK has actual finrank
orderOf(#K:ZModr). Primitive root existence/simple algebraicity/finite dimension
and carrier/actual Frobenius order are derived. For every actual finite
intermediate extension L, all rth roots belong to L iff r divides Nat.cardL−1,
equivalently (#K)^actualfinrank(K,L)−1; actual splitting descent and finite-module
cardinality prove the criterion. Every characteristicp finite field has actual
pth-root field=base/degree1 versus modularorder0 and p∤#K−1, refuting deletion
of the characteristic guard on the same actual fields. Every field actualr1/r2
rootfields have degree1. Correctly qualified cyclotomic statement survives.
Exact local **PCC-20261115-b5d0f3** PASS at 2026-11-15T02:29:20Z, signed standing unverified. Whole23/0/0;
complete861-character bindingf288e1fae592ab4d95cb11362fa1784f7a077a5ba6ee54060279e215ce8c5742;
independent native1complete/0prefix/0unresolved. Five modules/eight formal-config
plus14artifactpins;88files/44objects/16332-byte losslessXZ/five gate files;
fully inspected one-page46709-byte PDF. Eight snapshots/three failed01/03/04
retained; final08 and gate full logs have no warnings/errors. Credited prior
arguments and official APIs, exact primary read scope retained, originals unchanged.
The actual full elliptic E[r] field/canonical Weil-Tate geometric bridge,
general nonprime supersingular≤6 and other all14 obligations remain separate.
No novelty/canonical promotion/full14 closure/cryptoPython/new native crypto/
paid compute/agent upload. [pcc:unverified]


The [prime-field source theorem](../prime_trace_weak_bound_audit_20261115/README.md)
removes the explicit Hasse/count/trace-zero premise from the earlier conditional
prime-field argument. For every odd finite field q>3, actual cubic Euler-power
nonconstancy follows from Xq-X divisibility/formal differentiation/degree
contradiction. Actual completed-square and nonsingular-point/fiber equivalences
prove1<#E(K)<2q+1. For every primep>=5/all nonsingular general Weierstrass models,
FST2010’s exact trace/gcd definition of supersingularity forces actual trace0
and pointcountp+1. Every integer r>2 dividing that count, composite permitted,
has exact modularorder2 and actual cyclotomic extensiondegree2: the field is
literally IntermediateField.adjoin Fp of all rth roots in AlgebraicClosureFp.
Primitive-root/simple algebraicity/finiteness and actual Frobenius order2 are
derived, not assumed from a modular-order/field-degree identification.
Actual nonsingular F3count1/7/sourceSS/trace±3 controls disprove removing small-field
guards. Actual F5sourceSS count6/pointorder2/root-adjoin degree1 refutes literal
unrestricted degree2; actual F7sourceSS count8/pointorder8/root-adjoin degree2
is a positive composite control. Known examples and earlier arguments credited.
Exact local **PCC-20261115-b69cc7** replay PASS at 2026-11-15T01:57:20Z, signed standing unverified;
whole56/0/0, full1233-character binding
`d190809525ceaf4787000e559cc18b79c39b86f3c811bddb94579290d7c2c638`,
independent native1complete/0prefix/0unresolved;10formal/config+14artifactpins,
140files/56objects/25832-byte losslessXZ/fivegate files/inspected one-page47699-byte
PDF. Thirteen snapshots/eight failed states retained; final13 full logs have
no warnings/errors. Source FST2010 printed230/PDF7 and237/PDF14 fully text-read/
visually inspected, not all57pages. Initial derivative-protocol sign mistake
corrected before code in separate log; original preserved. Alternate geometric
SS/p-rank/Hasse-invariant comparisons, general Hasse theorem, general nonprime-field
at-most6 classification and full geometric torsion fields remain separate.
The repaired prime-field source clause is proved; literalr2 assertion is false.
No novelty/canonical corpus promotion/full14 closure/frozen edits/cryptoPython/
new native crypto/spend/upload. [pcc:unverified]


The [actual continuous-H1 comparison](../continuous_h1_comparison_audit_20261115/README.md)
discharges the separate homogeneous-complex comparison gap listed in earlier
packets below. Actual Mathlib continuousCohomology1 is linearly equivalent to
all continuous crossed cocycles modulo all principal cocycles. Actual d0/d1,
both homogeneous/crossed inverse maps, full principal-image equality and actual
categorical kernel/cokernel/index isomorphisms are proved. Representatives use
actual categorical homologyπ, with the exact representative formula proved.
For every finite K and finite Abelian M with continuous Galois action into
discrete automorphisms, the actual coefficient TopRep derives joint continuity;
actual H1 identifies with the whole standard class quotient and Frobenius
coinvariants. Under exponent dividing #K−1, both perfectness MulEquivs and
both radicals hold on actual H1, with exact source chi(F(1,Frob)) formula.
Every finite characteristic2 field/M=Z2/trivial action has actual H1 nonzero
and all characters trivial; deleting annihilation makes right-nondegeneracy
impossible on these same canonical domains. Qualified source survives.
Exact local **PCC-20261115-de23d0** replay PASS at 2026-11-15T01:19:32Z; signed standing unverified.
Whole100/0/0 includes11byte-identical prior cores/6new modules;
20formal/config+14artifactpins, full1405-character binding
`65928939b66df9e96c426f625abbea3493ad127ec27c3bae6ed4cc38c7d8848d`,
independent native1complete/0prefix/0unresolved;189files/61objects/24844-byte
losslessXZ/fivegatefiles/inspected one-page47307-byte PDF. Twelve snapshots,
five failed states and setup01 failure retained. Final12 has exactly two
inherited unused-section advisories, no new-module advisories. Prior pairing
advisory-free prose is corrected outside its frozen pinset.
Isogeny-kernel/Lang/Cartier group scheme/Jacobian/divisor/full Tate/MOV/FR
bridges remain open; original FR94 unread. No novelty/canonical promotion/
full14 closure, frozen edits/cryptoPython/new native crypto/spend/upload.
[pcc:unverified]


The [actual Galois-character pairing audit](../galois_character_pairing_audit_20261115/README.md)
proves the finite-module pairing displayed in Bruin2011 Lemma2.1 for every
finite K and finite Abelian M with continuous action and exponent dividing
#K−1. Its first factor consists of all actual Gamma-equivariant characters
M→(AlgebraicClosure K) units; unique descent to K units follows from actual
Frobenius fixed fields, and invariance under all Gamma follows from actual
density. The second factor is all continuous crossed cocycles modulo all
principal cocycles. Both perfectness MulEquivs, exact source formula on every
representative, both bilinear laws, both trivial radicals and equal cardinality
are proved. Seven credited prior core files remain byte-identical.
For every finite characteristic-2 K, actual M=Z/2 with trivial action has
all characters trivial and an actual nonzero continuous-cocycle class;
no right-nondegenerate bilinear map on the same domains exists after deleting
the annihilation hypothesis. The qualified source lemma survives.
Exact local **PCC-20261115-4711ad** replay PASS at 2026-11-15T00:52:09Z; signed standing unverified.
Whole gate73/0/0 includes reused proofs, complete1184-character binding
`117deabc6ffa81828e499acbcc42d2c0a96947d2d3e5605128590db25ba4aee7`,
independent native1complete/0prefix/0unresolved;15formal/config+14artifactpins,
106files/41objects/18280-byte losslessXZ/fivegate files and inspected one-page
46919-byte PDF. Six snapshots/four failed states retained; final06 passes with two unused-section advisories (the prior advisory-free report wording is corrected in the packet).
Separate continuous homogeneous-complex H1 comparison, isogeny-kernel/Lang/
Cartier group scheme/Jacobian/divisor/full Tate/MOV/FR bridges remain open;
original FR94 remains unread. No novelty, canonical promotion or full14 closure.
No frozen edits/cryptographic Python/new native cryptographic experiment/spend/upload.
[pcc:unverified]


The [finite continuous-cocycle audit](../procyclic_galois_cocycle_audit_20261115/README.md)
proves the finite-coefficient functionalformula behindBruin2011Eq2.1.
For everyfinitefieldK, actualKrullGal/qFrobenius/fixedfielddescent/infinite
Galoiscorrespondence deriveFrobeniusdensity. Actualfinitefieldextensions/
intermediatefieldrestriction/cyclicGaloisgroups deriveuniquefinite-target
continuoushomwithanyprescribedFrobeniusvalue. For everyfiniteAbelianM/
continuousactualaction, actualfinite semidirectproducts/graphhom yield
allcontinuouscrossedcocycles/evaluationMulEquivtoM. Actualallprincipal
continuouscocycles/wholeimage andactualstandardall-cocycleclassquotient
haveaMulEquivtoFrobeniuscoinvariants with exactrepresentativeformula and
principalclassrelation. Noassumedprocyclic/H1value/duality/NDconclusion
orrestrictedrepresentatives. Everyfinitebasefield/integertrivialaction
hasallcontinuouscocycleszero/nonzeroactualcoinvariantclass1/sameactual
classevaluationnonsurjective, generalizingcreditedearlierF2control.
Qualifiedfinitecaseproved/notrefuted. Comparison to separateMathlib
continuoushomogeneouscomplexH1/Lang/Cartier/Jacobian/divisor/fullTate/
MOV/FRbridgesremainopen; originalFR94primary stillunread afterbounded
publisher403/sixarchive503routes. No novelty/canonicalpromotion/full14
closure; previousfullBruinprimary/selectedfreshreading credited.
Exactlocal **PCC-20261115-ed6495** PASS at 2026-11-15T00:25:12Z, signedstandingunverified;
whole36/0/0/full953-character binding
`4f6948a2e69332e5f2a6f72ef7a13f751e3653bda1dbfa30f90482305c59c613`,
independentnative1complete/0prefix/0unresolved;6custom/9formal-config+
14artifactpins;106files/44objects/17160-byte losslessXZ/fivegate and
visuallyinspectedone-page47416byte report. Eleven snapshots/sevenfailed
states incl08Leanpanic/fullbacktrace retained; final11advisoryfree.
No limits/options/waivers/nativecrypto/cryptoPython/spend/upload.
[pcc:unverified]


The [Diem cyclic-module audit](../diem_cyclic_module_audit_20261114/README.md)
recovers the archivedMarch24,2003 original odd-GHS draft,33pages.
All33OCRpages inclappendix/references read; PDF8/10/13/18/23 visuallychecked.
Brokenfont extraction/OCRformula limitations and journal32page-version gap
retained. Section7 generaloddn/Lemma22 invokes prime-onlyLemma17.
ActualF2[X]/(X6+X4+X+1) hasdimension6/card64, actualrootunitorder21,
actualcyclicgroupcard21/faithfulrepresentation/orbitspan and actual
F2[C21]modulecyclicity. Actualevaluation-at1kernel yieldspropernonzero
invariantspace and actualgroupalgebrasubmodule/directcriterionnegation;
ord_21(2)=6/odd21/coprime2/composite. Thisrefutes the composite-degree
algebraicinference, not a constructedGHSfunctionfield/geometricLemma22
or norm-conormkernelTheorem1. Universaloddnrepair addsactualannihilator
f dividesPhi_n/degreeord_n2, derivesirreducibility/no properinvariants;
actualn21failsguard, actualprime5/7quotientcontrols pass/dim4and3.
No constantdescent/regularfunctionfields/Kummer/intermediatefield/source
circulantcoordinate/genus/classgroup/Jacobian/runtime bridge, novelty,
canonicalpromotion or sourceprimeapplicationrefutation. Copyrightprimary
PDF/OCR/images localonly; separateCoverAttacks2003 remains1–10read.
Exactlocal **PCC-20261114-cc4013** PASS at 2026-11-14T23:50:33Z, signedstandingunverified;
whole64/0/0/full1037-character binding
`54ccb5e70c8f6740f7224c4a6c3053312df871bfe63c4dca93c1869795b7029d`,
independentnative1complete/0prefix/0unresolved;4custom/7formal-config+
14artifactpins;81files/40objects/16588-byte losslessXZ/fivegate and
visuallyinspectedone-page47603byte report. Nine snapshots/sixfailed
buildstates/scratchfailedexploration/earlyupdate-hookoverlap retained.
Final09advisoryfree, no limits/options/waivers/nativecrypto/spend/upload.
Historical44snapshot unchanged; original14 remain separately tracked.
[pcc:unverified]


The [Momose cubic homography audit](../momose_cubic_homography_audit_20261114/README.md)
recovers actual primary eprint2005/277 and checks complete sourceLemma1
and its printed explicit matrix. Full21page extracted text reviewed;
exactLemma1/PDF6 andLemma3/PDF11 visuallyinspected; glyphlimitations
outsidecheckedtarget retained, notallformulas/tables verified.
For actual K,L/algebraembedding/cardK=q²/cardL=q⁶/q≥2,
derive degree3 and εoutsideK from ε^(q³)=ε/ε^q≠ε.
Actual cubicgeneration/powerbasis/quadraticindependence yield an actual
GL2(K) matrix with nonzero denominator sendingε to every targetoutsideK,
unique in actual MathlibPGL and up to actualunit scalars.
The printed matrix[a(ar+c)−b²,a²e−bc;a,−b] is verified whenε³=rε+e.
ActualF9/F3/card9/degree2/a²2/nonbasea has a nonidentityactualPGL
matrix[0,2;1,0] fixinga, directly refuting uniqueness ifcubicdegree is
deleted. QualifiedsourceLemma1 is proved/notrefuted; wholecover/
Jacobian/isogeny/kernel/density/Lemma3–5/classification/runtime claims
remain open. No zero-witness shortcut for nonzero normalization,
novelty/canonicalpromotion; historical44snapshot unchanged.
Exact local **PCC-20261114-8dd57c** PASS at 2026-11-14T23:18:14Z, signedstandingunverified;
whole40/0/0/full1206-character binding
`180630ee33d7ebd4be7786362c32efdafdb2f0cc7802f6e698432768f72fdc2a`,
independent native1complete/0prefix/0unresolved,7custom/10formal-config+
14artifactpins,89files/45objects/61248-byte losslessXZ/fivegate and
visuallyinspected one-page48032byte report. Seven snapshots/fourfailed
buildstates and earlier two-page report draft retained. No nativecrypto/
spend/upload. DiemoddGHS original33pages retrieved but onlyselectedPDF3
OCRread duefontissues; separateCoverAttacks2003 onlypages1–10read.
Those partialsources notpromoted tofullaudits. [pcc:unverified]


The [HPS first-fall primary-paper audit](../hps_first_fall_audit_20261114/README.md)
reads all24PDFpages of the institutional version of Hodges–Petit–Schlather2014,
DOI10.1016/j.ffa.2014.07.001, and formally refutes the printed general-field
Theorem4.9 rank bound and upstream4.7/AppA.2 strict positivity.
Actual B=Fp[X]/(X^p), for every primep≥5, has actual λ=x² of exact
degree2/minimal linear-generatorrank1/firstfallp, strictly above(p+3)/2.
ActualF5 gives5>4. Quotient power basis/independent spanning homogeneous
pieces/graded multiplication/positive-part qnilpotence and actual
annihilator quotient/minimum/rank are derived. Exact source qnomial
coefficients and actual bi-infinite Γ satisfy Γ₅(1,2,4,2)=0; no cutoff
approximation. The sharp one-variable monomial repair gives firstfallq
andrank1 for q≥3,2≤d<q; q3 attains the printedbound3, while q2 x²/degree2
piecezero is an invalid-domain control. Original corpusnode
`THM-GB-HPS-FIRST-FALL-WEIL` is binaryq2: its specialization and m²+1
bound are not refuted or fully proved. FullWeil/filteredgraded/allVtensor/
solvingdegree/runtime bridges remain open. No novelty/canonical-status
promotion; historical44snapshot unchanged. Exact local **PCC-20261114-0c9bcf** PASS at
2026-11-14T22:48:00Z, signedstanding unverified; whole45/0/0/full1506-character
binding`e1eb8e2ab8f67dad4a4467cb03e2437b6ff46b888c2c36fc6c36c6373abfea9e`,
independent native1complete/0prefix/0unresolved,7custom/10formal-config+
14artifactpins,136files/61objects/64160-byte losslessXZ/fivegate and
visually inspected one-page48268byte report. Eleven build snapshots/nine
failed states/defaulttimeout plus corrected report-count draft retained.
No nativecrypto/spend/upload. [pcc:unverified]


The [Vélu Frobenius qualification audit](../velu_frobenius_audit_20261114/README.md)
binds original44-backlog node`ALG-RP-VELU-COST` to the actual primary
Bernstein–DeFeo–Leroux–Smith2020/DOI10.2140/obs.2020.4.39.
Full22page arXivv1 prose/math/appendix read; publishedselected
Theorem4.11/Algorithm2/§4.14 printed47–49/PDF10–12 read,11–12visual,
notfullpublisher20pages/originalVelu1971read. The irrational-generator
paragraph says FrobP=[λ]P; arbitraryextensionpoints neednot satisfythis
without cyclicFrobstability. Actual nonsingular E/F5:y²=x³+1,
K=GaloisField5,2/card25/extensiondegree2 supplies a²2,
actualfixedA(0,1),antifixedB(1,a) and P=A+B exactorder3,
nonrational overF5, actualFrobP outsideitsowncyclicsubgroup/nointegerλ.
Genericactualsubgrouppreservation iff existenceofintegerscalar onP is
proved. The sameactualcurve has a stableirrationalprime3generatorB
withFrobB=−B, so nonrationality alone doesnot invalidate the extension.
The publishedparagraph may intend rationalcyclickernel fromisogenycontext;
that qualifiedcontext/mainTheorem4.11rational-generator operationcount
is notrefuted. Arbitraryrational-kernel inputencoding/cost/fullisogeny
algorithm/canonicalGaloisdescent and ambiguousminimalxdegree/eigenvalue
order are notverified. Historical44snapshot/canonicalstatuses unchanged,
no noveltyclaim. Exact local **PCC-20261114-3cb039** PASS at 2026-11-14T22:14:11Z, signedstanding
unverified; whole38/0/0, full1385-character binding
`762ed418a4d3a6fe07f37b73f9066c427aa5cf722f3dcfd0c30666e955335fa7`,
independent native1complete/0prefix/0unresolved,12custom/24formal-config+
14artifactpins,89files/43objects/13396-byte losslessXZ/fivegate/inspected
onepagePDF47684bytes. Nine snapshots/sixfailedstates retained, including
defaultprettyprintertimeout handled by actualcurveAddEquiv/defaultlimits.
No nativecrypto/spend/upload. [pcc:unverified]


The [original Thériault2003 audit](../theriault_intersections_audit_20261114/README.md)
adds full18page primary reading for original44-backlog node
`ALG-WD-THERIAULT-2003`/source`S-WD-THERIAULT-2003`.
Printed85–86/PDF11–12 Theorem1 and its proof were visually inspected.
Its literal lower bound 2s²/(3n)≤E for3≤s<n/2 is directly refuted
in the actual stated uniform paired-point model. Actual pointsFinM×Bool
have n=2M/freeinvolution/orbits, and genuine uniform pointwordPMF has
proved independentuniformwithreplacement productmasses. ActualQevent
probabilities are nonnegative/normalized; the printed E=sumk*kQ is
proved equal to the actualPMF expectation, with derived
E=s−M+M(1−1/M)^s and source recurrence. AllM≥4/s3 retain allguards,
yet E=3/M−1/M²<3/M=2s²/(3n). Actual n8/512triples gives11/16<3/4.
The source correctly derives the closed expectation, then its last chain
drops the finite s−1 correction too aggressively. Lean also proves
4s²/(9n)≤E≤s²/n throughout the original window, retaining quadratic
order. Theorem3singlelargeprime q-exponent is not refuted; wholeJacobian/
Picard/smoothness/randomwalk/runtime bridges remain separate.
Fullprimaryreading is distinct from fullpaperLeanverification; historical
44unreadsnapshot/canonicalstatuses retained. No noveltyclaim.
Exact local **PCC-20261114-9391e1** PASS at 2026-11-14T21:40:20Z, signedstanding unverified;
whole37/0/0, full1265-character binding
`382c82b534b06bdbfdc28545d65acb0a2c1ea9e303eb73ca3f02ec130f2e9175`,
independent native1complete/0prefix/0unresolved,6custom/15formal-config+
14artifactpins,116files/52objects/16000-byte losslessXZ/fivegate/inspected
onepagePDF47329bytes. Nine snapshots/fivefailedstates preserved.
No nativecrypto/spend/upload. [pcc:unverified]

## Original fourteen requirements

The [actual Hasse coefficient recurrence](../hasse_norm_recurrence_audit_20261114/README.md)
now verifies MIT2019PS4Problem1(g). Every oddprime p, fieldcharp,
actual polynomial fdegree≤3 and allr≥0 satisfy actual
H_(p^(r+1))=H_(p^r)*Hp^(p^r) and
H_(p^r)=Hp^(sum_i<r p^i). Forr>0, H_(p^r)zero iff Hpzero.
Actual sparse product convolution/proved degree bound/halfpower
decomposition/official actual iterateFrobenius derive this, without a
coefficient-recurrence conclusion premise. For finiteKcard=p^r and
actual nonsingular moniccubic Mathlib E, the unchanged actualPoint
count bridge derives actual count-defined integertrace cast=explicit
Hp Frobeniusproduct and actual pdivisibility iff Hpzero forr>0.
ActualF9nonsingular curve has Hp=a≠H9=a^4=1=actualtracecast;
the distinct exponents are connected by the correct qualified source law.
Literal char2X^3 recurrence, F3X^3r0vanishing and F3X+X^5
arbitrarydegree recurrence are directly negated. These are polynomial
helpercontrols, not fullnonsingularcurve supersingularity counterexamples;
degree3sufficiency not claimed sharp. Canonical Algebra.norm and
Frobenius-endomorphism/categoricaltrace, general Hasse/geometricSS/
p-rank/fulltorsionfield/integercountrecovery beyond13 remain open.
Earlier frozen packet history retained; newproof additive, no novelty or
qualified-source refutation. Exact local **PCC-20261114-262c52** PASS at 2026-11-14T21:14:35Z,
signedstanding unverified; whole18/0/0, full1909-character binding
`5f1d5ed1fd33903abc4af64d7cb205336f82ae131aba8ccf43ba159f9c8daa39`,
independent native1complete/0prefix/0unresolved,57custom/105formal-config+
14artifactpins,84files/35objects/15084-byte losslessXZ/fivegate/inspected
onepagePDF47431bytes. Nine snapshots retain four proof failures,
one premature-setup harnessfailure and one unavailable processcompletion;
only03/07/09pass, no invented05verdict. No nativecrypto/spend/upload.
[pcc:unverified]


The [actual Hasse-coefficient bridge](../hasse_trace_coefficient_audit_20261114/README.md)
proves the classical congruence for every finite odd-characteristic field
and actual nonsingular monic cubic Mathlib elliptic model. With q=cardK,
N=Nat.cardE.Point, actual count-defined integer t=q+1−N and actual
coefficient Hq=[X^(q−1)]f^((q−1)/2), derive cast(N)=1−Hq,
cast(t)=Hq, and Hq=0 iff the characteristic divides t. Actual rootfibers,
finitefield power sums, a proved strict polynomial degree bound and the
unchanged actual Point/fiber equivalence supply the proof; no count/trace
conclusion premise. Every oddprime specialization is included.
Actual nonsingular F13 curves x³−x and x³+4 have counts8/21,
integer traces6/−7 and equalH13=6, directly negating unrestricted trace
recovery. SourceMIT already asks this control and correctly retains p>13.
Actual F9 nonsingular y²=x³+a x²+1 with a²=−1 has Hp=a but
Hq=actualtracecast1, directly distinguishing p and q exponents.
ActualF2rootfiber helper and actualF3X^4 nonstrictdegree controls retain
their exact helper scopes, not claimed fullcurvecounterexamples.
Canonical Frobenius endomorphism/categorical trace, general Hasse bound,
geometric supersingularity/p-rank/fulltorsionfield, generic Hp-to-Hq norm
and qualified integer count recovery beyond13 remain open. No novelty
or qualified-source refutation. Exact local **PCC-20261114-20a800** PASS at 2026-11-14T20:56:17Z,
signedstanding unverified; whole35/0/0, full1978-character binding
`f95163d2e900734bb416540950d0eaa880dcbfac10f07433ee777214cf07a88a`,
independent native1complete/0prefix/0unresolved,54custom/99formal-config+
14artifactpins,83files/36objects/16468-byte losslessXZ/fivegate/inspected
onepagePDF46981bytes. All9buildsnapshots/5failedstates retained.
No nativecrypto/spend/upload. [pcc:unverified]


The [actual starting-point law bridge](../start_invariant_rho_audit_20261114/README.md)
closes the explicit zero-start versus uniform-start sampler comparison.
Actual hash-domain translation is an equivalence preserving product weights
and every weighted hash-observable expectation. For every nonzero Z/q,
finite labels and actual additive steps, actual iterate conjugacy transports
the actual Nat.find leastrepeat. Every real testfunction of firstcollision
has equal expectation at every start. For any normalized nonnegative label
law, the actual mean under the unchanged genuine joint static-hash/two-
independent-uniform-ordered-start PMF equals the actual zero-start mean;
actual pointmass/finite sum exchange/derived startinvariance/cardinality
cancellation prove this, without a distribution-equality premise.
The frozen source-generic Sidon table has p≤951T and T≤p at every hash/start;
its actual uniform-start mean E retains unbounded E²/p over primes,
direct real uniform-bound negation and kernelchecked prime10000019/E²>4p.
The actual fresh event probability7581/(8000p) and this mean now explicitly
use the same genuine jointPMF. Fixedtable versus random-table ensemble
qualifier retained; no novelty/source/asymptotic-constant/practicalhash
refutation. Exact local **PCC-20261114-52a042** PASS at 2026-11-14T19:24:01Z, signedstanding unverified;
whole22/0/0, full1697-character binding`67e5dab5404c635114f843a3257bb0556c706a3a6359d7ecffd69373015c65b7`,
independent native1complete/0prefix/0unresolved;19custom/43formal-config+
14artifactpins,48files/30objects/58988-byte lossless XZ/fivegate/inspected
onepagePDF47037bytes. Fourbuildsnapshots/first01twoerrors and both earlier
2pagePDF/renderer/layout drafts retained. No nativecrypto/spend/upload.
[pcc:unverified]


The [source-qualified fixed-table rho counterfamily](../sidon_rho_counterfamily_audit_20261114/README.md)
proves that exact degree-two local conditions do not force a uniform
square-root firstcollision bound. Actual20 shifted Sidon increments in
[477,951] satisfy unchanged DegreeTwoGeneric for allp>1902. Under the actual
original20-label static product-hash law, starting at0, every hash has
p≤951T and T≤p, hence p≤951E and E≤p. Actual variance19/20 and genuine fresh
two-step event probability7581/(8000p) hold under the same hash law.
Nevertheless E²/p is unbounded over primes, and Lean directly negates a
real uniform bound retaining the literal degree guard. Prime10000019 and
E²>4p are kernel checked. The bounded951-label embedding is pointwise;
its distribution is never assumed uniform on Fin951. This strengthens the
known bounded fixed-support control and credits source dependency warnings;
no novelty or qualified random-table-ensemble refutation. Limiting random-table
penalty/constants/practicalhash remain open. Exact local **PCC-20261114-9633b4** PASS at
2026-11-14T18:33:34Z; signed standing unverified. Whole27/0/0, complete1043-character
binding`f4550e4315788edd2a98a3625997981779e48497f43bb1391580f556d5203953`,
independent native1full/0prefix/0unresolved;17custom/38formal-config+
14artifact pins,50files/27objects/9628-byte lossless XZ/fivegate/inspected
onepagePDF47086bytes. Five snapshots/harness01 and cast02failures retained.
No new crypto experiment/spend/upload. [pcc:unverified]


The [general actual degree-two collision proof](../general_two_step_collision_audit_20261114/README.md)
closes the actual finite-group/support coefficient gap. For every finite
additive commutative group, finite labels/nonnegative normalized law and
literal source-qualified increment table, Lean derives actual four-query
injectivity, fresh-pair↔admissible-label/hash-agreement equivalence with
both inverses, fixedterminal expected count c=1−I2−I2²+I4 and global
expected count |G|c. The genuine joint static-hash/uniform ordered-start
PMF has real pointmass productlabelweights/|G|²; the actual fresh-event
outermeasure is ofReal(c/|G|). Query independence/count/coefficient/probability
are derived, not premises. Independent uniform starts are with replacement;
fresh event requires x,Fx,y,Fy distinct and F²x=F²y. Known Bernstein–Lange2013
local calculation credited; no novelty/qualified-source refutation.
Limiting firstcollision penalty/constants, higher-degree asymptotic correction
and practical fixed/keyedhash refinement remain open.
Exact local **PCC-20261114-7a948c** PASS at 2026-11-14T18:05:58Z, signed standing unverified;
whole35/0/0, complete777-character binding`5802fbd3152989fdd4805235c86098686c69810ad95be92dadfca6606495d47c`,
independent native1full/0prefix/0unresolved;11custom/29formal-config+
14artifact pins,85provenance files/34objects/12032-byte lossless XZ/
fivegate/inspected onepagePDF47185bytes. Seven builds/two failures retained.
Original generic cylinder/label/finite controls/sourceguards/claims unchanged.
No new crypto experiment/spend/upload. [pcc:unverified]


The [degree-two local collision proof](../two_step_collision_audit_20261114/README.md)
proves the generic normalized weighted admissible-label sum1−I2−I2²+I4
and checks the actual source correction on the complete primeZ7 population:
128staticlabelmaps×49orderedstarts=6272samples, genuine uniform PMF with
seven independent vertex-label/two independent starting-point factors.
Steps1,3 satisfy the literal source degree0/1/2genericity modulo
commutativity. Kernel one-step count448/rate1/14 and fresh two-step
count336/rate3/56 are proved; uniform labelcoefficient3/8 divided by7
matches the actual two-step rate. Steps1,2 are still distinct/nonzero but
fail degreegenericity, with actual freshcount112/rate1/56; Lean directly
disproves deleting the source degree condition. Known Bernstein–Lange2013
correction is credited; no novelty/qualified-source/asymptotic refutation.
General actual-group/support coefficient, limiting firstcollision penalty
and practicalhash refinement remain open.
Exact local **PCC-20261114-480d25** PASS at 2026-11-14T17:30:06Z, signed standing unverified;
whole26/0/0, complete1347-character binding`4c1acf0fd9722f21066a7ce0cb3de28321e7e626a7fbb2eaa2efafd2c441c000`,
independent native1full/0prefix/0unresolved;4custom/7formal-config+
14artifact pins,96provenance files/44objects/12128-byte lossless XZ/
fivefinalgate/inspected onepagePDF47371bytes. Seven builds/four failures,
both native inputs/protocols and both gates retained. Initial scoped-type
parser failure repaired with identical explicit ENNReal type, rebuilt/regated;
frozen parser/sharedverifier unchanged. No new native crypto experiment/
spend/upload. [pcc:unverified]


The [exact static-hash collision proof](../static_hash_collision_audit_20261114/README.md)
closes the first-collision model bridge: any finite state update and any
nonnegative normalized label law, actual independent static vertex-label
PMF survival equals actual iid chronological-word selfavoidance at every
time. Actual queried-state injectivity is derived from the word path;
the generic actual cylinder theorem supplies product probabilities.
The unchanged actual leastrepeat Nat.find on every nonzero Z/p is linked
to prefix injectivity, giving exact tail probabilities and finite tail-sum
expectation, with arbitrary update/start. Actual primeZ3/steps1,2distinct
nonzero/positiveuniformlabels gives word0,1,1 staticmass0versusiid1/8
and directly disproves deleting selfavoidance from chronological word mass.
It does not identify unrestricted postcollision histories or a practical
fixed/keyedhash with iid labels. Limiting random-increment-table collision
penalty/constants remain open. Known Bernstein–Lange2013higher-degree
anticollision corrections are credited, not claimed as a newly disproved
qualified source theorem or proved asymptotic correction.
Exact local **PCC-20261114-25d8e2** PASS at 2026-11-14T17:05:51Z, signed standing unverified;
whole37/0/0, complete1421-character binding`be9c6e9f61b3fd7984f70666f38565d4b55df432cbeecdc75723d02c47859a1f`,
independent native1full/0prefix/0unresolved;10custom/22formal-config+
14artifact pins,97provenance files/41objects/14148-byte lossless XZ/
fivegate/inspected onepagePDF46588bytes. Six failed states retained.
No new crypto experiment/spend/upload/novelty. [pcc:unverified]


The [actual local-algebra length proof](../local_algebra_length_audit_20261114/README.md)
now derives self-module length m for actual K[t]/(t^m) and
K[x,y]/(x^m,y−x), for every field K and natural m. The proof constructs
actual multiplication-closed tail ideals and a strict chain from a nilpotent
power basis; scalar restriction gives the opposite bound. Positive m gives
actual local rings with support1 over every extension field, arbitrarily
large local length at support1. The m0 zero rings have length0 and are not
local. Actual F4/F2 is a local field with ring length1 versus scalar length2,
with a direct inequality proving why a residue-degree qualification matters.
No ring-length/locality/cycle/intersection result is assumed. This closes
self-module length/locality for the actual truncated/bivariate family,
not general scheme-cycle/localization/intersection geometry, positive-dimensional
component extraction, Diem's qualified n!d^n bound or JV ringlength8.
Exact local **PCC-20261114-81195a** PASS at 2026-11-14T16:43:35Z, signed standing unverified;
whole28/0/0, complete771-character binding`58235242b9dd4b2995d959c49454952926fbd7657a0cd011701e36af26bd4249`,
independent native1full/0prefix/0unresolved;7custom/13formal-config+
14artifact pins,64provenance files/30objects/10964-byte lossless XZ/
fivegate/inspected onepagePDF46331bytes. Three failed states retained.
No qualified-source refutation, novelty, new crypto experiment/spend/upload.
[pcc:unverified]


The [canonical JV graded-piece proof](../jv_graded_hilbert_audit_20261114/README.md)
closes the prior finite per-degree identification gap. For the unchanged
literal homogeneous F101 source ideal I, Lean proves actual ideal
homogeneity and that the kernel of the actual quotient map restricted
to Mathlib P_d is exactly I∩P_d. The canonical piece P_d/(I∩P_d) is
linearly equivalent to the actual image, which is proved equal to the
matching-degree span of the inherited actual quotient basis. Matrix-degree
propagation and monomial/homogeneous induction derive that equality;
no grading/Hilbert/GB/regularity/native result is a premise. Exact canonical
piece dimensions are1,3,3,1,0 thereafter, zero iff d≥4. Total scalar
dimension/length8 remain distinct, with a direct HF0≠totaldim control.
This does not prove internal F4 degrees, generic JV/FGHR claims or a
general graded-module Hilbert polynomial theorem. Exact local **PCC-20261114-aec576**
replay PASS at 2026-11-14T16:06:12Z, signed standing unverified; whole26/0/0,
complete712-character binding`288fa232f61b729eee2a64e17f2741efcf7c144c019684d0bc676e5fcd2db8b6`,
independent native1full/0prefix/0unresolved;11custom/20formal-config+
14artifact pins,62provenance files/32objects/11452-byte lossless XZ/
fivegate/inspected onepagePDF47096bytes. Four failed states retained.
Original sources/claims untouched; generic/larger/internalF4/wholecurve
and remaining original14 gaps stay open. [pcc:unverified]


The [cubic-permutation family proof](../cubic_permutation_count_audit_20261114/README.md)
proves for every finite field qmod3=2/char≠2,3/bnonzero that the actual curve
y²=x³+b is nonsingular, its actual Point type is equivalent to Option K,
its cardinality isq+1 and its point-count trace is0. Cube inverse and
actual equivalence/count are derived, not assumed. Every divisor r>2 of
its actual cardinality has modular embedding order2, composites included;
Lagrange derives the same result for each actual point order>2. The
primefield specialization covers every prime p≥5/pmod3=2 with no upper
p bound and derives both characteristic guards. Exact F5b0/F2b1/F7b1
actual nonsingular-point counts5/2/12 and direct omission negations,
plus actual F5b1r2order1, retain the exceptional cases. Galbraith
Exercise9.10.4/Example9.11.4 and prior controls are credited; no novelty
or qualified-source refutation. This is the count/trace/modular-order
family, not a general geometric supersingularity/p-rank/endomorphism/Hasse
or full-torsionfield equivalence. Exact local **PCC-20261114-2d6d52** replay PASS at
2026-11-14T15:39:33Z, signed standing unverified; whole26/0/0,
complete1230-character binding`bb894ca30f297e8548c0a85fdbce66f660ae55191557c09b50f5f846b9c3dcaa`,
independent native full-header1complete/0prefix/0unresolved;3custom modules/
6formal-config+14artifact pins,69provenance files/32objects/10268-byte
lossless XZ archive/fivegate/inspected onepagePDF46954bytes. All failed
buildstates retained; original14/general source bridges remain open.
[pcc:unverified]


The [original-request scope map](REQUEST_SCOPE.md) separates completed requested bounded checks from stronger follow-ups and unfinished mathematical bridges. Higher-arity absolute irreducibility is an optional item1 extension; generic p-adic/limiting theory and stronger author/runtime independence are separate from the completed concrete Smart/rho/prime-S3-host components. The full14 request remains partial. [pcc:unverified]

The [complete actual BK wrapper](../bk_full_binding_audit_20261114/README.md)
repairs the concrete old receipt by addition. **PCC-20261114-8c5c1e** exact local replay
PASS at 2026-11-14T15:14:07Z, signed standing unverified. The actual F₅/r3/all-k theorem
is unchanged; the new written934-character type explicitly quantifies L and
all field assumptions, with actual point-map helpers. Frozen extractor and
independent native official Lean parser agree on complete binding
`255334b4de88ddc92803431efa042b40aa02db63affd414666c7a20c01cb02ed`:
1complete/0prefix/0unresolved. Whole new project gate1/0/0;
14custom modules/26formal-config+14artifact pins,44provenance files/25objects/
8824-byte lossless XZ archive/five final gate files/inspected one-page
PDF47183bytes. Both failed elaboration states are retained. Old issued
PCC-20261110-99983d/source/ledger and shared verifier stay unchanged;
no false mathematical theorem inferred. The local concrete binding repair
is closed; canonical semantic parser/telescope/dependency contract and other
original14 proof/execution gaps remain open. [pcc:unverified]


The [written-signature integrity audit](../statement_binding_audit_20261114/README.md)
checks62 named bindings in65 immutable added source plans:61 complete written
headers/1 issued prefix/0 unresolved, using native official Lean parsing,
source/recorded hashes and qualified source-namespace selection. A public-tree
replay reproduces the result. Three native/catalog-only plans remain outside
the theorem count. The affected issued **PCC-20261110-99983d** stores176characters
inside a legal named argument; full written BKActual type913characters is
recorded independently. Original actual-object source/pins/whole37 proof gate
remain; no false BK theorem inferred. A versioned explicit-context full-binding
wrapper is now published as the additive repair above; old claim/ledger/source/shared-verifier stay unchanged.
Surface-header completeness does not bind ambient telescopes or every
definition/dependency. Native counts are procedural evidence; the kernel
certificate proves genuine semantic controls and transparent delimiter-model
noninjectivity only, not a Python/native/SHA refinement. Model exact local
**PCC-20261114-286b6f** replay PASS at 2026-11-14T14:37:43Z, signed standing unverified; gate8/0/0,
complete596-character binding`b3e3af513d61f8acbcfbca85f672699b2b8132900fdbe18a09c0208ea39c1988`,
3custom/33formal-config+16artifact pins,419 provenance files/140objects/
58788-byte lossless XZ archive/5gate/inspected one-page PDF46251bytes.
All failed controls/first incomplete pass/public-tree replay preserved.
Canonical semantic binding and other original14 gaps remain open. [pcc:unverified]


The [qualified expected random-support mixing proof](../random_support_average_audit_20261114/README.md)
now closes Hildebrand1994Theorem1 for every fixed k=d+1>=2/strictly positive
normalized label PMF. For every epsilon>0 it derives gamma>0 and N before
prime modulus/support: actual expected halfL1<epsilon at exact
author time floor(gamma*p^(2/d)), for all primes p>=N/all starts. Actual
uniform all-ordered-distinct-support point masses and first-point/relative
injective-sample expectation decomposition are proved, never assuming
independence of distinct coordinates. Every standard character is covered
bijectively; actual Gaussian moment average, finite mean-square TV bound,
weighted Cauchy, denominator limit and exact floor/rpow estimates derive the
source exponent, without supplying mixing/average/asymptotic conclusions as
premises. The actual zero-weight pure-label walk is deterministic; averaged
distance>=1/2 for all primes/times/starts, directly negating any gamma/N giving
<1/4 on sufficiently large primes. This deletes positivity, not a qualified
refutation. Prior literal k2 counting/Gaussian equality errors retain known
correction credit and valid repairs. This supersedes every earlier open
expected-support exponent entry in this living map; static rho/hash-table
collision penalty and limiting constants remain open. Exact local **PCC-20261114-4f738d**
replay PASS at 2026-11-14T13:26:37Z; whole gate34/0/0,
complete1586-character binding`83b7a165067e380a276ddcf08733e5cdecb404684ee336251d3560d70d5af070`,
30 custom modules/60 formal-config plus11 artifact pins,
126 provenance files/44 objects/20528-byte lossless archive
(including first whole gate/dry prefix), five final gate files/inspected
one-page PDF46364bytes. Dry248-character parser prefix caught before issue;
equivalent explicit type rebuilt/gated and full statement bound. Signed
standing unverified. [pcc:unverified]


The [Gaussian source-step and actual support estimates](../support_gaussian_audit_20261114/README.md)
prove a literal intermediate equality false at allowed k3: the actual convergent
2D Gaussian sum is strictly smaller than the printed geometric expression.
The printed real-power exponent and positive1.1epsilon/(2p) coefficient are
retained in a direct affine equality negation; the valid <=repair is proved.
This does not refute qualified mixing. Actual Gaussian summability/integral
comparison give the zero-excluded integer and finite-subset bound sqrt(pi/a),
with nonsummability and finite-singleton controls at a0. Actual uniform
injective-sample product expectation has the descending-factorial denominator
and nonnegative product-of-coordinate-sums bound without independence of the
coordinates. Canonical centered actual characters, global Jordan bound and
actual PMF variance derive all-time actual walk moment exponential decay;
positive constant is derived solely from fixed label weights before modulus,
support, frequency, time or start. Original1994proof199/preprint17 selected
review recorded; no novelty claim. Full expected support-average exponent
remains open. Exact local **PCC-20261114-ee1e7f** replay PASS at 2026-11-14T13:04:59Z; whole gate46/0/0,
complete1806-character binding
`ea36a9f58e9ab88db16a3ee051fda3460c55e793deb152b798b0543c30c18f61`,
20 custom modules/47 formal-config plus11 artifact pins,78 provenance files/
32 objects/16755-byte lossless archive/final five gate files/inspected one-page
PDF46294bytes. Signed standing remains unverified. [pcc:unverified]


The [finite Fourier upper-bound proof](../finite_fourier_upper_bound_audit_20261114/README.md)
now closes the finite-abelian Parseval/TV bridge. Actual character orthogonality
derives finite complex Plancherel; actual fullL1/halfL1 probabilities, every-event
bound and an attaining event identify the source max-event convention exactly.
Actual uniform coefficients and probability normalization remove the trivial
character. Thus halfL1² <=one quarter of nontrivial Fourier energy, and applying
the actual iid character-moment law yields the actual walk/every-event spectral
bound for all finite abelian groups/times/starts. Actual ZMod2 pointmass/uniform
has fullL1=1/halfL1=1/2/energy1: direct mixed-normalization negation, correct
quarter equality and quarter-constant necessity. Original1981 fullL1 and
Diaconis author1987/Hildebrand halfL1 conventions are credited, no qualified
source refuted. Selected original1981 proof/author1987 chapter3B read, not full
paper/book; exact scopes and versions retained. This supersedes the earlier
open finite-abelian Parseval/TV entries; full minimum cutoff and expected-support
spectral averaging remain open. Exact local **PCC-20261114-e1a4a9** replay PASS at 2026-11-14T12:27:36Z;
whole gate38/0failures/0trusted, full2195-character binding
`8e1b8c3254622300cf9d726f5eff360ee88e895237fc9e27aedcf53dc96b175b`,
13 custom modules/31 formal-config plus11 artifact pins;99 provenance files/
40 objects/19469-byte lossless archive (including first whole gate), final
five gate files/inspected one-page PDF46802bytes. Dry parser prefix357chars
was detected before issue; equivalent explicit abs syntax rebuilt/gated and
complete final statement bound. Signed standing unverified. [pcc:unverified]


The [actual walk-character proof](../walk_character_audit_20261114/README.md)
derives actual finite complex PMF expectation/bind and the independent additive
walk Fourier law: character expectation equals `λ^t*ψ(start)`, with actual
squared moment `normSq(λ)^t`. Normalized unit-phase variance and nonnegative
pair defects give the quantitative character-pair/cosine alignment estimate.
The actual standard ZMod character’s cosine formula and prime nonzero-index
injectivity yield strict contraction for an injective support with two
positively weighted labels. Actual zero-frequency coefficient/moments1,
prime-field pure-label weights1/0/distinct supports0/1/coefficient1/direct
strict-contraction negation, and positive-fair-weight ZMod4/supports0,2/
nonzero index2/coefficient1 controls are bound. No spectral formula or abstract
mixing law is supplied as a premise. Full source minimum-over-pairs cutoff,
Parseval/TV and expected random-support spectral averaging remain open.
Exact local **PCC-20261114-c4aad9** replay PASS at 2026-11-14T12:00:51Z; whole gate37/0failures/0trusted,
complete2294-character binding
`5aa92485a1a41a58f879a300b849daef3d3f6c899cecd962c688d629e0196b8c`,
7 custom modules/19 formal-config plus11 artifact pins;79 provenance files
in32 objects/13961-byte lossless archive/five gate files/inspected corrected
one-page PDF47556bytes. Signed standing unverified. [pcc:unverified]


The [corrected Hildebrand box-asymptotic proof](../hildebrand_box_asymptotic_audit_20261114/README.md)
now closes the corrected author-preprint section4.2/Lemma2 counting bound.
For each fixed d=k−1≥2 and ε>0, Lean derives a threshold uniform in integer
cell shifts, first support point and nonzero Fourier index. Every sufficiently
large prime satisfies the actual full-conditional uniform box probability
bound `1.1*(ε/2)/(2^d*p)`, with literal real-power endpoints bound. Actual
source cardinality and uniform-law equality, normalized descending-factorial
limit1, source width growth/exact power and correction-factor limit1 are
all derived. No asymptotic or probability premise is supplied. The known
k2 actual/no-eventual-bound counter remains in the main certificate, along
with the exact fixed-width identity and direct failure of width growth.
This supersedes the finite packet’s earlier open asymptotic gap; the stronger
expected random-support mixing exponent still needs its spectral argument.
Exact local **PCC-20261114-cbcb79** replay PASS at 2026-11-14T11:39:39Z; whole gate21/0failures/0trusted,
complete1346-character binding
`ac4b0f9ca0aed6dcff2ab1d2c9f94db4b35439e81fd4401d449e0a85e544ce54`,
14 custom modules/32 formal-config plus11 artifact pins;97 provenance files
in37 objects/13990-byte lossless archive/five gate files/inspected one-page
PDF47466bytes. Signed standing remains unverified. [pcc:unverified]


The [exact finite injective-support counting proof](../injective_box_audit_20261114/README.md)
derives the actual uniform sample cardinality and point masses, exact box
probability, and coordinate-product upper bound with denominator
`(p−1).descFactorial d`. An explicit equivalence covers all conditional
injective prime-field supports with fixed first point; the mapped PMF equals
the actual uniform source law. Nonzero Fourier index gives canonical centered
coordinate counts at most `p*(U−L)+1`, retaining the lattice correction.
For every prime p≥3, Lean also proves that deleting the nonzero-index
hypothesis makes the one-coordinate zero-width probability bound false:
the actual probability is1, exceeding `1/(p−1)`. The qualified source lemma
is retained. Corrected asymptotic1.1 and the stronger expected random-support
mixing exponent remain open. Exact local **PCC-20261114-8f6cf7** replay PASS at 2026-11-14T11:23:45Z;
whole gate49/0failures/0trusted, complete1615-character declaration binding
`74bd1e4a6ad421bdc085caeacc841339cffa62495e283ed494f6c18499ec6f35`,
10 custom modules/25 formal-config plus11 artifact pins. All131 build and
processing files retained losslessly in46 objects/23591-byte archive;
five gate files and visually inspected one-page PDF46968bytes. Signed
standing remains unverified; no native experiment or paid compute. [pcc:unverified]


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
| 2 BK | Partial overall: complete p<500 k1census; actual F5/r3 qualified and F7/r3 excluded all-k cases; general r2geometry and characteristic2controls; general actual geometric r3rank2/basis/Frobenius/descent; normalformactualcubiccharacter/detq. **Qualified prime3 criterion now proved for every finitefield/arbitrary nonsingular Weierstrassmodel/everychosencompatibleextension**, with actual inclusion AND originalnine-point count, actualnormalization/group/scalar/tower/Frobenius bridges (50). Exactlocal PCC-20261114-3f6943 PASS, signedunverified. F7q1/r² and ordinarychar3controls retained. See ../bk_three_normalization_audit_20261114. Otherprimes≥5/canonicalWeil/allkp500remainopen. [pcc:unverified]  Originalprime3nine-point criterion nowprovedwithoutaddedcoprime; generalchar3actualcard1or3/modulefinrank0or1 withcreditedconcretecontrols (22), exactlocalPCC-20261114-72d986PASS/signedunverified. Inclusionformstillqualified. See ../three_torsion_char3_audit_20261114. [pcc:unverified]  Directoriginalr2/r3finite-fieldcountcriteria nowprovedwithonlyAlgebraFK/cardq^k, no suppliedclosure/tower/coprime (7), exactlocalPCC-20261114-e0dd93PASS/signedunverified. See ../bk_small_prime_source_audit_20261114. [pcc:unverified]  Written-header audit detects one issued old BKActual binding prefix176vs913chars; original actual37-proof gate retained, full-context934-character wrapper now locally verified with independent full-header match. [pcc:unverified] |
| 3 Supersingular | Partial: actual r=2 exact-degree-two counterexamples; complete F2/F3/F4/F9 and p<500 basefield checks. Generic char2/3 actual geometric j/torsion/module criteria; all-model F2/F3 kernel source/geometry/Hasse/count/order4/6(44) and F5/F7 Hasse/source-trace0/count6/8/composite-order/named cyclic6/8(29). **Every finite field of characteristic2 or3, every actual nonsingular general model: exact source-trace iff j0 iff actual geometric p-torsion card1/rank0, with finite-extension source invariance.** Char2 also rational-geometric equality/rank; imperfect-field control rejects deleting finiteness(26). Char3 uses actual complete-square, finite cube roots, paired twist three-point construction and actual counts(55); rational replacement and trace-zero strengthening are directly refuted. Generic odd-field actual twist count/trace/source invariance and literal Galbraith9.5.3 special-j F7 counter(42) retained. General p>=5 trace-geometric/Endrank, Hasse beyond checked cohorts, embedding/full-torsion-field and further explicit extension censuses remain open. ExactlocalPCC-20261114-3a8304PASS/signedunverified; see ../finite_char_three_trace_audit_20261114. [pcc:unverified]  Cubic family now proves actualcountq+1/trace0 and divisor+actual-point modularorder2forallfiniteqmod3=2/char≠2,3/bnonzero, with derived p≥5prime specialization and exact omission/r2controls; no general geometricSS/Hasse/fulltorsionfield closure. [pcc:unverified]  Actual odd-field Hasse-coefficient bridge now proves actual Point count/actual count-defined trace congruence and coefficientzero iff characteristic divides trace; explicit F13 nonuniqueness/F9 exponent/helper controls. General geometricSS/Hasse/Frobenius-endomorphism/fulltorsionfield remains open. [pcc:unverified]  Actual source Hasse primepower recurrence/Frobeniusproduct/positiveextension Hp-Hqvanishing and actual Hp-trace-divisibility now proved, with char2/r0/degree5/F9controls. Canonicalnorm/Frobendomorphism/generalgeometry/Hasse gaps retained. [pcc:unverified] |
| 4 Pairings | Partial: literal composite-order/nontrivial-root Tate source counterexamples on actual F3/F5 curves(50); actual F25 Weil/Tate Miller tables/proper quotient domains and rational restriction disproofs(126); actual k1 F7 true quotients/all-valid-auxiliary nondegeneracy and torsion-restriction controls(116); original Verheul printed formula actual F29/F841 counter and actual group-automorphism repair(49); generic odd-field twist counts/source invariance and special-j literal counter(42). **Characteristic-order actual domains(37): anychar-p unit roots, finite power target1, actual E/nE=kernel cardinalities, unbounded ordinarychar2 and pairedordinarychar3 domain obstructions; imperfectF2(X) target control; direct weakened-claim negations and proved prime-root=>coprime source guard.** Qualified source and group-scheme pairings retained. Canonical divisor/valuation Weil/Tate, generic MOV/FR and general End/Tate-module bridges remainopen; original FR94 remainsunread. ExactlocalPCC-20261114-f7faf2PASS/signedunverified; see ../char_p_pairing_domain_audit_20261114. [pcc:unverified]  Finite character/coinvariant algebraic core of fullyread Bruin2011: both evaluation MulEquivs/derived finitefield roots, alloddfield actual2torsion, F3positive4/4/units2 and unboundedchar2negative1/2 with directannihilation-deletionnegation(34); continuousH1/Lang/Cartier/divisor bridges remainopen. [pcc:unverified]  Literal broadBruinEq2.1 evaluation assertion outsidefinite/torsionscope refuted onactualKrullGalF2/exactsquareFrobenius/trivialdiscreteZ: allcontinuouscocycleszero, nonzerointegercoinvariantclass unreachable, no evaluation-induced classquotientbijection(16). FiniteLemma2.1/application retained; canonicalcontinuousH1notreconstructed. [pcc:unverified] |
| 5 Diem size | Partial: stable branch locus fails actual Condition2.7. Prop4.11also explicitly requires n≥2and log2q≥7n, now read/visually checked on printedp95. Conditional arithmetic absorption/countbound now proved; actual nonsingularF4curve/pointchart control has emptyfactorbase and directly refutes unrestrictedhalfbound. q2/n2control violates sizehypothesis; actualscheme degree/ramification/admissibility and geometricestimate remain open. No counterexample to qualifiedbound. |
| 6 Gaudry | Original2000 §§2–3 read; qualified fixed-genus limit retained. Complete Lean models x5-minus-x have8/7overF3and16/16overF5; Stollx5+1/F3has10/8. Native all-monic-quintic F3/F5census completes4cells with full Rust/C++matching and byte-identical repeats:162/2500squarefree models per repeat. Formal Picard identification and scaling error remain open; the asymptotic statement is not refuted. |
| 7 Subgroup smoothness | Lean disproves universal finite equality in complete actual Cantor groups: F3x5-minus-x/order2fraction1vs7/8; new complete Stollx5+1/F3checks all100actual additions, defines unique normalized arithmetic before group transfer, and proves actual(X,1)order5/nonchar3/proper subgroup fraction1vs4/5. Excluding identity from both domains gives1vs7/9and a direct universal negation. Whole Mumford gate808passes. Native all-model counts42/1510odd-prime/noncharacteristic unequal strata remain separately attested. New unbounded-genus F3family f=X^(24t+5)+1,g=12t+2proves actual unique normalized generator runs, exact ncycles, smooth fraction1vsambient<1and direct universal cycle-equality negation; whole separate gate91passes. This adds actual arithmetic scaling, while all-pair ambient group law, Picard identification and qualified typical/large-prime asymptotics remain open. See ../mumford_family_audit_20261110. A further dependent project proves all-pair normalized outputs before group transport, exact cyclic group law on the family, actual generatorordern/all membersgenerated, and equality of the earlier actualStep with generatoraddition;64/0/0gate. The full ambient group/Picard bridge still remains open. See ../mumford_family_group_audit_20261110. A further Dirichlet-based Lean extension proves unbounded prime24t+5orders with all actual arithmetic/group/fraction conclusions and direct prime-family uniformity negation;5/0/0gate. Genus grows; full ambient/Picard and typical/dominant-prime asymptotics remain open. See ../mumford_prime_family_audit_20261110. New fixed-genus2/unbounded-fieldF_(3^(2t+1))actual Cantor image-group/cyclic order2counterfamily and even-degree splitting control (60). Full ambient Picard/group remains open; no uniform limiting-gap or qualified large-prime asymptotic refutation. See ../mumford_fixed_genus_audit_20261113. [pcc:unverified] |
| 8 Bézout | Partial: actual length2/support1/native16; JV quotient dimension/F-length8/support1 (51). Diem F2two-torus source-helper disproof/qualified chart repair and overhead (31). Generic actual solution/quotient-map equivalence, finite support≤dimension=scalar-module length, actual unbounded (x^m,y−x) family and m0/m1/target-domain controls (44). Main n!dⁿ/decomposition node unrefuted; scheme/local multiplicity/positive-dimensional/Rojas bridges remain open. See ../coordinate_length_audit_20261114 and ../diem_chart_audit_20261114. [pcc:unverified]  Actual self-module length=m and positive-m locality now proved for truncated/bivariate family, all-extension support1/unbounded local-length gap/m0nonlocal controls; actual F4/F2 ringlength1≠scalarlength2. General cycle/intersection remains open. [pcc:unverified] |
| 9 Gröbner heuristics | The older short-Weierstrass [f,0] counterexample is retained with its exclusions. New actual F25 twisted-Edwards input has two nonzero quadratic equations and a finite/Artinian affine quotient of dimension2, yet its actual highest parts fail regularity. Actual source-polynomial symmetrization, basis-coordinate descent, quotient equivalence and actual factor-base point sum pass a separate55-declaration gate. This refutes the literal universal n=2 reading; generic/n>=3 variants, conditional complexity and JV remain open. A new frozen18-call native cohort now checks eight literal systems twice, including this Edwards input/topparts/homogenization and both original JV F101³ toy inputs affine/homogeneous; independent native GB/remainder/S-pair/source/A/A checks pass. JVempty homogenized returnedGBdegree4 but selectedpairdegree5 adds0new/2zero; this is not a full internal-degree refutation. A separate39-declaration Lean project now proves the literal JVempty affine coordinate ideal is unit, the actual homogeneous ideal is proper and contains every monomial of total degree>=4. No genericity premise or native GB oracle. This is an uppercutoff only. A separate59-declaration Lean project now constructs the actual F101³field, checks the nonsingular curve/nonzero target, identifies all three printed coordinate equations through genuine coefficient maps and proves actual factor-base sums or signed sums cannot equal that target. The source example is confirmed at this scope. A separate51-declaration Lean project now proves the actual quotient has an eight-element basis and scalar-field dimension/length8, geometric support1, and sharp monomial cutoff4 via h³ nonmembership in the actual ideal. No native GB/Hilbert premise. Whole-curve order/scalar relations, internalF4refinement remain open. Generic/n>=3 and larger m,n sweeps remain open. See ../edwards_descent_audit_20261113, ../groebner_system_audit_20261113, ../jv_macaulay_certificate_20261113, ../jv_source_field_audit_20261113 and ../jv_hilbert_audit_20261113. [pcc:unverified]  Canonical actual P_d/(I∩P_d) Hilbert function now proved1/3/3/1/0, zeroiffd≥4, actual idealhomogeneity/kernel/image/basis-span bridge and HF0≠totaldim control; generic/internalF4 remain open. [pcc:unverified] |
| 10 Walks | Actual product-PMF in-degree mean/variance and finite collision populations, plus unbounded-prime fixed-support collision obstruction. Qualified HildebrandTheorem2 uniform quadratic mixing and Theorem1 expected random-support exponent now proved for all fixed k>=2/strict positive normalized weights/prime moduli, actual iid walk laws. Theorem1 retains full uniform ordered-distinct-support PMF, exact floor(gamma*p^(2/(k-1))), constants before modulus/support and all starts; finite Fourier/Gaussian/without-replacement/denominator bridges derived. Zero-weight all-support expected distance>=1/2 for all primes/times and direct no-gamma/N negation. Known literal k2 counting and Gaussian equality errors have valid repairs; no qualified theorem refuted. See ../random_support_average_audit_20261114/README.md and ../rho_collision_audit_20261110. Static rho/random-table collision penalty and limiting constants remain open. [pcc:unverified]  Exact actual static-hash versus iid firstcollision survival/tail/finiteexpectation bridge now proved for arbitrary finite update/label law; actualZ3distinctnonzero-step postcollision word0,1,1 staticmass0≠iid1/8 direct control. Limiting randomtable penalty/practicalhash remain open. [pcc:unverified]  Generic degree2weightedlabelcoefficient1−I2−I2²+I4 and actualZ7complete6272sample PMFrates1/14(one)/3/56(two) now kernelproved; actualdistinctnonzero-step guard-deletion counterrate1/56. Known BLcorrection credited; asymptotic penalty remains open. [pcc:unverified]  General actual finite-group degree2fresh-event coefficient now proved: actual pair-label equivalence/derived queryinjectivity, expectedcount and genuine jointPMF measure(1−I2−I2²+I4)/|G|. Limiting firstcollision penalty/practicalhash remain open. [pcc:unverified]  Fixed20-step source-DegreeTwoGeneric counterfamily now retains actual local rate7581/(8000p)/variance19/20 yet has unbounded E²/p over primes; local conditions alone do not imply uniform square-root collision time. Qualified random-table ensemble retained. [pcc:unverified]  Starting-point/sampler bridge now proved: allreal firstcollision-statistic expectations invariant under start, genuine same jointPMF uniform-start mean equals zero-start mean; source-generic counterfamily/allstarts bound/unboundedprime ratio retained. Random-table/practicalhash law remains open. [pcc:unverified] |
| 11 Standards | Actual corrected 160-bit curve now has a Lean-certified prime-order-p point and actual known-answer relation; whole59gate and full local packet90623bpass. The old-preprint j-invariant disproof reproduces the author's already published erratum, not a new correction or a refutation of the corrected source. A separately frozen three-call native Smart run now recovers the known answer in all4fixed lifts; native packet162b83 remains not_covered. New Lean finite-program certificate proves all1,680transitions, lifting/inverses/coefficient recovery and actual returned dG=Q under the prescribed policy, with separate whole gates28/51and full local packetd8466dPASS. Signed standing remains unverified. The entire corrected160-bit curve now also has actual Nat.card E.Point=p proved in Lean: closed nonresidue certificate, actual missingfiber cardinal bound and Lagrange from the certified orderp point, without Hasse. Actual F5/#10/order5control directly refutes orderp-alone inference. Whole gate17and full local packet541e6fPASS, signedunverified. The requested finite repository rho measurement now completes 768/768 calls, 576/576 measured replays and all same-seed controls; a separate Lean gate passes 11 declarations for exact report arithmetic. Recorded means exceed the standard-negation reference; emitted-counter pricing reverses the plain/negation ordering across weights 0 and 1. This is a finite accounting observation, not an asymptotic refutation. Generic p-adic transfer/complexity, native refinement and limiting laws remain open. See ../rho_constant_audit_20261112. See ../anomalous_cardinality_audit_20261110. See ../anomalous_curve_audit_20261110/smart-v1. Screened NIST recommendation is not refuted. See ../anomalous_curve_audit_20261110. |
| 12 msolve/F4 | Exact original F4Run modules now imported unchanged. Generic ideal containment and direct disproof of an unrestricted saturation-output reading are proved, with actual F101 expected-row certificates; whole gate573passes. Selection/preprocessing/simplify source mapping is documented. Native F101 saturation rejects low characteristic; v1 registration defect retained. Separately frozen F65537 trial crashes in ordinary mode and times out in saturation; both not_validated, complete export structurally valid. No ordinary C/libm refinement, crash/hang diagnosis, full parameter-map or row-trace certificate yet. See F4_MODEL.md. A separate frozen eight-cell diagnostic finds original exactLA1ordinary passes, LA2ordinary139, bothsat5stimeouts, allfour combined sanitizer cells stop earlier at hash.c409signed1shift31. Cause remains unproved; no Crefinement or independently signed execution. See ../msolve_crash_diagnostic_20261110. Address-only follow-up locates LA2 READfault atla_ff_32.c543. Common-logged baseline rewrites old2 to matrix3, then reads basis3/live3 and aborts; one matrix-lookup backport returns exact ordinary known basis, both saturation cells stilltimeout. New14-declarationLean project proves the observed operational copy invariant and direct wrong-table negation, not Ctrace authenticity/pointer/wholeF4Run refinement. Currentupstream and pinned16bit alreadyusematrixlookup; no noveltyclaim. See ../msolve_reindex_lean_audit_20261110 and native coefficient-remap-v3. |
| 13 Independence | Actual separate-host C++andRust replay completed on public run43501438026/job132147937956/attempt1; both942models/4188162triples/244236zeros,0directional errors. Canonical export2planned/2completed/0excluded passes all hashes, unchanged frozen protocol. Existing Lean foundation11passes. Native PCC-20261113-9b23b3not_covered/signedunverified; compiler/runtime and independence from same author open. Original manualHTTP404 retained. See ../s3_host_replay_20261113. [pcc:unverified]  Native independent Lean surface-header audit resolves62bindings:61complete/1issued BKprefix/0unresolved; genuine controls/model proof separate from procedural counts. Actual BK wrapper is now complete; ambient/dependency/shared-verifier semantic contract remains open. [pcc:unverified] |
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
