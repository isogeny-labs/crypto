The new [160-bit point certificate](../anomalous_curve_audit_20261110/README.md)
proves the actual prime, bit bounds, nonsingularity, exact prime order of an
actual Mathlib point, and its fixed known-answer scalar relation. Its source
control reproduces the author's published erratum for an omitted coefficient
digit; the corrected source is not refuted. The complete gate passes 59 public
theorems with zero failing or trusted-native exceptions, and the full local
packet check passes. Smart execution, its formal bridge and the matched
SafeCurves benchmark remain separate obligations. [pcc:unverified]

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
counts; untouched papers are not labelled audited. [pcc:unverified]

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

This adds formal evidence to items1,2,3and9 and primary-source review to
items6and10. It does **not** complete the fourteen-item
request. The original hypotheses and all gaps are retained in the
[acceptance map](../theorem_audit_20261108/README.md#exact-acceptance-map).
The table here records the current delta; earlier components remain valid
only at their already-stated scope. [pcc:unverified]

| Item | Current status and concrete remaining obligation |
|---|---|
| 1 Semaev | Partial: full general-Weierstrass S3equivalence in every characteristic, repeated-coordinate cases and actual point sums proved. Short/ordinary binary forms are checked specializations. Rational-only converse and wrong-model extensions refuted, including actual F2/F3geometric controls. Actual fixed-degree S4resultant equivalence and degree-drop control proved; bounded S3/S4/S5native census completed. Canonical individual degree4in each m4input and sharp specialization criterion proved. Universal fixed-size resultant degree step, actual recursive family and last-input degree2^(m−2)for every aritym≥3now proved; reset-of-size counterexample retained. New actual bivariate-exchange proof gives newest-pair symmetry and sharp degree for both newest inputs at every recursive aritym≥4; symmetry-stage wholegate320passes. New sharp surviving-variable degree/leading coefficient theorem gives actual canonical third-newest degree2^(m−2)at allm≥5; latest wholegate357passes. All formal input degrees are now proved through actual variable permutations with explicit Fin-position coverage; all-input gate423passes. General fixed-size root criterion and leading-zero/closure controls now proved; latest census gate431passes. The new actual evaluation and point-list strong induction now prove all-m geometric zero-sum equivalence over algebraically closed fields/nonzero discriminant, with a direct nonsingular F5rational-only disproof. Full polynomial permutation invariance beyondm4 and irreducibility remain open; original2004PDFstill403/unread. |
| 2 BK | Partial: original r²point-count formulation read; actual excluded-case geometric3-torsion witness even with9dividing#E. New F2/r2actual geometric torsion has2points all rational, refuting unrestricted set inclusion despite2notdividingq−1. The actual node already excludes this by gcd(r,q)=1; original r²condition is not refuted. Actual finite-base point descent, torsion stability and rational-torsion iff Frobeniusidentity are now proved; actual excluded-case nonfixed3-torsion is checked. Actual chosen-finite-extension point/torsion descent via q-Frobenius iterates now proved in degree and cardinality forms. Qualified geometric rank/determinant and Weil necessity plus all-p<500 census remain open. |
| 3 Supersingular | Literal all-prime k=2 assertion refuted at r=2 using the source trace definition and actual groups. FSTp239directly checked with actual cyclotomic degree1 versus nonrational2-torsion. New characteristic2a1=0and characteristic3short families have trivial geometric p-torsion in actual groups. Corrected modular embedding proof now covers every integer r>2, with an actual order8positive control, and remains conditional on Hasse; New trace-class module proves prime-divisor modular order in{1,2,3,4,6}, coprimality and actual prime-order point, conditional on explicit trace-square classification; composite q289/r81control has order9but is arithmetic only. Latest census gate331passes. Original Waterhouse4.1pp536–537read/visually checked. General trace/p-rank/classification and embedding-field bridges plus binary/ternary census remain open. |
| 4 Pairings | Partial: cyclic restriction obstruction, actual cyclotomic/full-torsion distinction, and actual geometric F2torsion/μ2subtypes proving every point-valued map degenerate at characteristic order. Group-scheme duality is not refuted. Actual qualified Weil/Tate constructions, nondegeneracy and MOV/FR calculations remain open. |
| 5 Diem size | Partial: stable branch locus fails actual Condition2.7. Prop4.11also explicitly requires n≥2and log2q≥7n, now read/visually checked on printedp95. Conditional arithmetic absorption/countbound now proved; actual nonsingularF4curve/pointchart control has emptyfactorbase and directly refutes unrestrictedhalfbound. q2/n2control violates sizehypothesis; actualscheme degree/ramification/admissibility and geometricestimate remain open. No counterexample to qualifiedbound. |
| 6 Gaudry | Original2000 §§2–3 read; qualified fixed-genus limit retained. Complete Lean models x5-minus-x have8/7overF3and16/16overF5; Stollx5+1/F3has10/8. Native all-monic-quintic F3/F5census completes4cells with full Rust/C++matching and byte-identical repeats:162/2500squarefree models per repeat. Formal Picard identification and scaling error remain open; the asymptotic statement is not refuted. |
| 7 Subgroup smoothness | Lean disproves universal finite equality in complete actual Cantor groups: F3x5-minus-x/order2fraction1vs7/8; new complete Stollx5+1/F3checks all100actual additions, defines unique normalized arithmetic before group transfer, and proves actual(X,1)order5/nonchar3/proper subgroup fraction1vs4/5. Excluding identity from both domains gives1vs7/9and a direct universal negation. Whole Mumford gate808passes. Native all-model counts42/1510odd-prime/noncharacteristic unequal strata remain separately attested. Picard geometry, scaling family and qualified typical/large-prime asymptotics remain open. |
| 8 Bézout | Partial: actual bivariate quotient equivalence and support1/length2 Lean witness; all16msolve multiplicity/control calls pass. General n!dⁿ bound and application-specific decomposition systems remain open. |
| 9 Gröbner heuristics | Universal square regularity reading has this Lean counterexample; generic variants and JV remain open. Native msolve controls passed, but those heuristic systems are not yet measured. |
| 10 Walks | Actual PMF fixed-support nonmixing, uniform graph-variance correction and exact3930-support/82034-map populations remain checked. New Lean theorem proves actual in-degree mean1/variance1−Σpᵢ²for every finite abelian group, distinct finite increments and normalized nonnegative label law, using a genuine product PMF and actual edge counts at every target. Direct negative control shows nonzero-only increments insufficient: duplicate+1steps have variance0versusunmerged-label1/2. Whole walk gate70passes. General collision populations, random-support averaging and limiting collision constants remain open; qualified Hildebrand is not refuted. See ../walk_audit_20261109/WEIGHTED_GENERAL.md. |
| 11 Standards | Actual corrected 160-bit curve now has a Lean-certified prime-order-p point and actual known-answer relation; whole59gate and full local packet90623bpass. The old-preprint j-invariant disproof reproduces the author's already published erratum, not a new correction or a refutation of the corrected source. Whole curve point count=p, concrete Smart computation/formal bridge and matched SafeCurves rho benchmark remain open. Screened NIST recommendation is not refuted. See ../anomalous_curve_audit_20261110. |
| 12 msolve/F4 | Exact original F4Run modules now imported unchanged. Generic ideal containment and direct disproof of an unrestricted saturation-output reading are proved, with actual F101 expected-row certificates; whole gate573passes. Selection/preprocessing/simplify source mapping is documented. Native F101 saturation rejects low characteristic; v1 registration defect retained. Separately frozen F65537 trial crashes in ordinary mode and times out in saturation; both not_validated, complete export structurally valid. No ordinary C/libm refinement, crash/hang diagnosis, full parameter-map or row-trace certificate yet. See F4_MODEL.md. |
| 13 Independence | Separate Rust S3 arithmetic now matches the complete six-prime C++ population:942models/4188162triples/244236zeros,0directional discrepancies. New actual quadratic-field Lean foundation gate11pass. The single separate-host manual dispatch returnedHTTP404, with response retained; no remote execution is claimed. See ../independent_s3_validator_20261109. |
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
