# Fourteen-target audit: remaining formal bridges and execution prerequisites

- **Status:** open
- **Kind:** proof gaps, implementation gaps and access/tool limits; no impossibility claim
- **Affected work:** [fourteen-target audit](../research/theorem_audit_20261108/README.md)
- **Next action owner:** audit continuation; environment owner for credential/host provisioning
- **Last updated:** 2026-11-14

## Problem

The request for all fourteen source audits, experiments and faithful Lean proofs
is incomplete: local checkpoints and bounded native components have run,
including the requested separate-host prime S3 replay. Main integration,
further independent replays and several actual curve/Jacobian/implementation
bridges remain unresolved.
[pcc:unverified]

## Evidence and uncertainty

The first two infrastructure observations below describe the original local session. Approved local checkpoints and the completed hosted prime-S3 replay supersede those original launch/second-host dependencies; compiler/runtime/author independence and signed standing remain separate. Current formal coverage is in the all-fourteen table and later additive receipts. [pcc:unverified]

- Observed: a complete resume document was supplied to `scripts/checkpoint.sh
  gate`; upload failed because neither the configured AWS profile nor an
  upload token exists. Local builds and fixed development tests continued.
  The same checkpoint dependency appears in
  [the existing launch-gates record](arity-completion-launch-gates.md).
  No paid cloud resources were requested or launched. [pcc:unverified]
- Observed: the selected runtime contains one execution environment. The
  Rust production controls and additional C++ fixture execute on it; this
  does not supply the requested other-machine validation. [pcc:unverified]
- Local mathematical components: `PROOF_MAP.md` describes the actual Lean
  statements. The S3 existence equivalence is general in its stated domain;
  the Frobenius, branch, pairing and subgroup lemmas expose their missing
  bridges. An arithmetic identity is not labeled a full Jacobian, pairing or
  algorithm-refinement theorem. [pcc:unverified]
- Missing formal bridges: E[r]'s Frobenius representation/characteristic
  polynomial; supersingularity and Hasse trace-zero result; actual Weil/Tate
  constructions and domains; Diem ramification and intersection geometry;
  reduced-divisor/Jacobian definitions and smoothness counting; msolve row
  arithmetic/provenance/degree refinement; probabilistic mixing/collision
  theorems on the appropriate walk ensemble. These are not infrastructure
  errors or assumed conclusions. [pcc:unverified]

## What would unblock it

- The checkpoint-upload prerequisite was removed by explicit user approval
  of local checkpoints for this audit; do not ask again. New local runs
  still need their own frozen protocols/evidence, and this exception does
  not authorize paid compute or settle a theorem. [pcc:unverified]
- Supply an independently controlled execution host, or a reviewed CI run
  of the appropriate independently implemented validator and original inputs;
  retain its real provenance and signed report. [pcc:unverified]
- Complete each missing formal bridge against the exact source hypotheses,
  build its actual module, audit axioms and map its conclusion to the corpus
  statement before promoting a result. [pcc:unverified]

## Next discriminating steps

1. The bounded Semaev census, all-arity zero-sum/degree/symmetry/source-split
   proofs and p<500 base-field k1 census have run. Continue higher-arity
   absolute irreducibility, generic odd-prime torsion/Frobenius/determinant/
   descent, general supersingular geometry and remaining all-k census
   obligations. New native work needs its own frozen protocol; finite
   agreement does not prove a universal statement. [pcc:unverified]
2. For item7, extend the checked concrete Cantor groups, growing-genus prime
   family and fixed-genus unbounded-field family to the full ambient
   Jacobian/Picard group/divisor identification. The actual image-group
   counterfamilies do not alone supply that geometric bridge or a qualified
   typical-subgroup asymptotic theorem. [pcc:unverified]
3. For item12, bind recorded C matrix rows to monomial/prior-polynomial
   certificates and total degrees; separate ordinary GB, trace replay and
   saturation. A source comment naming F4 is not a refinement proof.
   [pcc:unverified]

## Resolution and history

Unresolved. The proof-and-control increment does not close the original
fourteen-item request, and opening or merging its PR would not do so.
[pcc:unverified]

### 2026-11-09 follow-up

The user explicitly approved local checkpoints for this audit. That removes
the upload prerequisite recorded above; it must not be asked for again.
The native Semaev census subsequently completed its documented bounded
domain with no discrepancy counters. The missing frozen JSON preregistration
is preserved as a deviation, not repaired retroactively. [pcc:unverified]

The [follow-up study](../research/theorem_census_20261109/PRIMARY_AUDIT.md)
now has an expanded74-declaration local Lean gate, actual BK excluded-case
geometric3-torsion, an F5 r=2 corpus counterexample, and a direct FSTp239
counterexample using actual F7(mu2) field degree1 and nonrational2-torsion.
The finite-field source trace definition is connected to actual point counts;
the corrected odd-prime theorem retains Hasse as an explicit premise. No
general p-rank equivalence or Hasse theorem is thereby supplied.
[pcc:unverified]

Primary BK/FST/Gaudry/Hildebrand sources were recovered/read at the recorded
scopes. All44unread-label tasks and239source records remain catalogued; the
full fourteen-item objective and independent second-host census stay open.
Main integration has an additional immutable-ledger fork, which was not
resolved by selecting a side or rewriting prior rows. [pcc:unverified]

### 2026-11-09 general-model extension

The S3component now has a universal actual-group proof for the general
Weierstrass model in every characteristic, not only short/ordinary binary
forms. Both model specializations and geometric false-zero controls over
F2/F3are checked. Actual geometric p-torsion families in characteristics2/3
are proved without assuming a trace/Hasse bridge. Expanded project gate167
passes,0failing/0trusted-native exceptions. General resultant recursion,
irreducibility/degree and the other original obligations remain open.
See [new proof scope](../research/theorem_census_20261109/GENERAL_S3.md).
[pcc:unverified]

The current independent Rust local validator agrees on4188162triples;
its distinct-host manual dispatch returned404and never executed. A new
attempt needs normal workflow registration/permissions and an explicit
protocol amendment. At the general-S3checkpoint, main's ledger has483rows versus local479,
common prefix457; no chain has been selected/rebuilt. Those access/integration
dependencies do not prevent the independent local proof work above.
[pcc:unverified]

### 2026-11-09 first resultant step

The actual fixed degree2/2Mathlib Sylvester-resultant equivalence for S4is
now proved in every characteristic on nonsingular general Weierstrass
models. A faithful geometric F5control disproves the finite-common-root
shortcut under simultaneous degree loss while preserving the valid
four-point zero-sum theorem. Whole census gate197passes,0failing and
0trusted-native exceptions. See
[scope and declarations](../research/theorem_census_20261109/GENERAL_S4.md).
General recursion beyond m4, irreducibility and individual-variable degree
proofs remain open. This increment adds no native population or host.
[pcc:unverified]

The16:26Zread-only ledger comparison is483main rows versus481local,
common prefix457; both histories remain intact. Parent3739is still
open/unmerged. Local mathematics continues independently of that merge
dependency. [pcc:unverified]

### 2026-11-09 composite-order arithmetic

The corrected modular embedding-order theorem now allows every integer
r>2dividing the actual curve cardinality, retaining Hasse explicitly.
An actual smooth F7curve has P=(3,3)of order8and modular embedding order2.
Full census gate205passes,0failing/0trusted-native exceptions; unsigned
packet10789dpasses its full local check. These composite orders are positive
controls, not counterexamples under the trace-zero divisibility condition.
General Hasse/geometric/field-degree bridges and original censuses remain
open. See [scope](../research/theorem_census_20261109/COMPOSITE_TORSION.md).
[pcc:unverified]

### 2026-11-09 S4individual degrees

The m4degree component now has a canonical polynomial-ring proof of
degree4in each variable, all characteristics, with the other inputs formal.
The fourth coefficient after specializing them is S3², and exact degree4
holds iff S3≠0. The actual smooth F5specialization S4(0,1,4,T)=1has degree0;
this refutes arbitrary preservation of degree, not the generic theorem.
Whole census gate259passes,0failing/0trusted-native exceptions. See
[source and exact scope](../research/theorem_census_20261109/S4_DEGREE.md).
General all-mrecursion/degree, m5degree and irreducibility remain open.
No solving-degree/regularity or attack-complexity result follows.
[pcc:unverified]

### 2026-11-09 general resultant degree step

The actual fixed-size resultant now has a universal arbitraryDleading
coefficient/degree proof; nonzero incoming fover a domain gives degree2D.
The canonical S5application gives last-input degree8with all other inputs
formal. A Lean counterexample shows that resetting Dafter specialization
changes the operator. Whole census gate291passes,0failing/0trusted-native
exceptions. See
[source and scope](../research/theorem_census_20261109/RESULTANT_DEGREE_STEP.md).
This narrows the earlier m5degree gap to the remaining input positions;
general all-mfamily/zero-sum recursion, every-input degrees and irreducibility
remain open. No asymptotic or complexity conclusion follows. [pcc:unverified]

### 2026-11-09 every-arity last-input degree

The canonical family is now defined using actual independent polynomial
coefficient rings and fixed-size Sylvester-resultant successors. Lean
proves last-input degree2^(m−2)for every aritym≥3, the successor coefficient
square and an exact equality with canonical S4at arity4. Whole census gate
308passes,0failing/0trusted-native exceptions. See
[source and scope](../research/theorem_census_20261109/RECURSIVE_SEMAEV_DEGREE.md).
This closes the earlier all-arity family-definition/last-input-degree gap.
The other input degrees beyondS4, symmetry, all-mgeometric zero-sum
equivalence and irreducibility remain open, along with the other original
requirements. [pcc:unverified]

### 2026-11-09 complete finite Mumford-pair count

The separate Jacobian-smoothness study now proves a complete bijection with
all actual reduced polynomial pairs for F3y²=x⁵−x:8valid/7split, fraction7/8
and finite deviation3/8from1/2. The curve polynomial is proved monic,
degree5and separable/squarefree; the project gate passes80declarations,
0failing/0trusted-native exceptions. See
[source and scope](../research/jacobian_smoothness_audit_20261109/README.md).
Formal geometric Picard identification, ambient Cantor group law/subgroup
proportions, multi-field native census and a scaling family remain open.
The finite exact-half strengthening fails; Gaudry's qualified limit and
MMT's binary large-prime heuristic are not refuted. [pcc:unverified]

### 2026-11-09 actual Cantor subgroup arithmetic

The new170-declaration gate checks actual normalized gcd, exact composition
and all64reductions, unique output/group equivalence on the complete
F3reduced-pair model. The proper cyclic subgroup generated by(X,0)has
order2and smooth fraction1versusambient7/8; Lean directly negates universal
finite cyclic-proportion equality. Nonzero smooth numerators give1/2versus
3/4with whole-group denominators. See
[source and scope](../research/jacobian_smoothness_audit_20261109/CANTOR_SUBGROUP.md).
This closes the finite-model Cantor-law/subgroup comparison gap above.
Geometric Picard identification, native multi-field census, scaling family
and qualified typical/large-prime asymptotics remain open. [pcc:unverified]

### 2026-11-09 pre-data F5control

The complete F5reduced-pair model for y²=x⁵−x has16pairs/all16split
and everyv0, with a faithful Lean proof. Current separate gate240passes;
packete81718full local check passes, signed standing unverified. This
supplies the exact all-smooth native negative detector control. Built
Rust/C++sources and a frozen two-fixture calibration protocol are ready,
but no curve call has run at this writing. Geometry/native census/scaling
and qualified-regime requirements remain open. See
[control scope](../research/jacobian_smoothness_audit_20261109/FIVE_CONTROL.md).
[pcc:unverified]

### 2026-11-10 actual-source and all-extension follow-up

Three new separate dependent projects preserve all issued parent source
and protocol bytes. Composite-order Tate domains directly disprove the
literal nontrivial-root hypothesis in Galbraith26.3.3: actual F3/n4 and
F5/n6 point groups and unit quotients, gate50/localPCC5e63e9PASS, signed
unverified. Correct primitive-root/prime-order results remain open. See
[the exact source boundary](../research/pairing_composite_order_audit_20261110/README.md).
[pcc:unverified]

The entire original six-page Semaev2004 author manuscript is now read,
with its exact PDF hash retained. Its literal missing-plus binaryS4 display
is refuted by actual F4 zero-sum points; the additive repair universally
equals the actual resultant and all16F2tuples agree. Gate28/localPCC95607fPASS,
signedunverified. Historical source-read receipts remain unchanged;
All-arity polynomial symmetry and arbitrary-split normalization are now proved in the later packets below; higher-arity absolute irreducibility remains open. See [the original-source study](../research/semaev_source_audit_20261110/README.md).
[pcc:unverified]

For actual E/F5:y²=x³+1/r3, the geometric-torsion-inclusion criterion is
now proved for every extension degree, with actual point images and explicit
GaloisField5k construction/cardinality/embedding for everypositivek. The
actual nonrational order3 witness proves every odd degree fails. Finalgate37/
localPCC99983dPASS, signedunverified. Generic torsionrank/determinant/Weil
necessity, actual geometriccard9 and requested allp<500census remain open.
See [the actual BK case](../research/bk_actual_case_audit_20261110/README.md).
[pcc:unverified]

Item7's next proof target now uses the known torsion family overF3 with
f=X^(24t+5)+1,g=12t+2: actual reduced split pairs(X^d,±1), an explicit
nonsplit reducedpair(X²+X+2,X), and normalized Cantor addition traces.
The variable-genus reduced-pair/Picard/Jacobian correspondence is still
missing. A checked polynomial orbit will not be promoted to a geometric
subgroup counterexample without that bridge. The entire14item request
remains open; another-machine execution still requires actual authorized
host/workflow access. [pcc:unverified]

### 2026-11-10 growing-family arithmetic result

The new [growing-family study](../research/mumford_family_audit_20261110/README.md)
checks all polynomial obligations for addition by (X,1) on f=X^(24t+5)+1
over F3, genus12t+2. Its n distinct smooth reduced pairs form an actual
normalized arithmetic cycle of exact periodn; the finite universe of all
valid reduced pairs has smooth fraction<1by the uniform nonsplit pair
(X²+X+2,X). Lean directly negates equality of these cycle/ambient fractions.
The whole91-declaration gate passes with no failing or trusted-native
exceptions. All39attempt/source files are archived losslessly. [pcc:unverified]

This supplies an unbounded-genus arithmetic family. It does not supply
an all-pair ambient Cantor group law or geometric Picard identification.
Those are the next formal obligations before calling this an unconditional
geometric Jacobian subgroup family. The periods24t+5are not all prime,
and typical/large-prime asymptotics remain separate. Known torsion-family
prior art (Jędrzejak2016) is acknowledged; no novelty claim. [pcc:unverified]

### 2026-11-10 all-pair arithmetic on the growing family

The [new dependent project](../research/mumford_family_group_audit_20261110/README.md)
proves all pairwise normalized Cantor outputs and uniqueness for explicit
selected Bézout coefficients before introducing a group law. The signed
residue bijection respects every actual output. Transported group addition
is then proved equal to independently defined normalized arithmetic.
The actual generator(X,1)has exactorder24t+5, every family member is
generated, and the earlier actualStep equals generatoraddition.
The whole64-declaration gate passes,0failing/0trusted-native;22attempt/source
files retained losslessly. [pcc:unverified]

This closes the all-pair group-law obligation within the family. It leaves
the complete ambient reduced-pair group law and its geometric Picard
identification open; no unproved ambient embedding is asserted.
The smooth fraction1versusstrictlysmallerambient result keeps that exact
polynomial-model scope. [pcc:unverified]

### 2026-11-10 actual quadratic fields F4/F9

The [quadratic-field study](../research/supersingular_quadratic_fields_audit_20261110/README.md)
now covers every general coefficient tuple over actual fields of sizes4/9
(1024/59049tuples). Generic monic-quadratic fibre bounds handleF4.
A proved characteristic3coordinate/count bijection reducesallF9models
to729normalized models, with allnine81-model kernel proofs accepted.
The source trace criterion yields actual pointcount sets1/3/5/7/9and
4/7/10/13/16. Every prime divisor has modular degree≤6,coprimality,
and an actual prime-order point. Explicit actual prime-order7/13curves
bothhave modular degree3. Wholegate42/0/0,57attempt/source/resource
files retained losslessly, including resource-aborted143attempt. [pcc:unverified]

The geometric trace/p-rank/endomorphism equivalence, full torsion/roots-of-unity
field interpretation and higher extensionsn≥3remain open. No general
Waterhouse/Hasse premise is used to prove these finite-field classifications.
A renewed bounded primary lookup for original Frey–Rück1994stillfails:
AMSprimaryPDF403, JSTORprimaryPDFinaccessible throughwebtool;
searchesfoundcitations, not a readable original. Its unread status remains.
[pcc:unverified]

### Completion review of the approved fourteen-item request

The current [fourteen-item evidence map](../research/theorem_census_20261109/README.md)
is the requirement-to-proof index. Work has reached every item; the complete
request remains partial. An accepted narrow counterexample does not settle
the source theorem with its hypotheses restored. [pcc:unverified]

| Item | Status at this checkpoint | Concrete remainder |
|:--|:--|:--|
| 1 Semaev | Partial: canonical all-arity zero-sum equivalence/degrees, literal binary S4 display disproof, and actual S3 symmetry/absolute irreducibility after every field scalar extension with2≠0, and actual source-ordered k2/S4-block polynomial normalization at every arity≥5; full all-arity input symmetry with actual coordinate/scalar semantics (90); actual ordinary characteristic2S3absolute irreducibility/allscalar maps/group semantics/model control (43); literal all-source-split recursion with actual natural degrees/source-last variable order and all domain specializations (82) | Higher-arity absolute irreducibility |
| 2 BK | Partial: actual qualified F5/r3 and excluded F7/r3 criteria at all positive extension degrees; complete p<500 basefield k1census; qualified matrix criterion/actual rational prime-torsion input; generic actual r2geometry/binary basis and characteristic2card1/2/coprimality-deletion control (67) | Generic odd-prime geometric rank, actual Frobenius determinant/representation/descent and all-k extension census over the p<500population |
| 3 Supersingular | Partial: actual exact-degree-two counterexamples and complete F2/F3/F4/F9model/p<500basefield checks; generic characteristic2geometric2torsioncard1/2and actual Δ1models (67) | General Hasse/supersingular geometric classification, further extension-field censuses and full torsion-field bridges |
| 4 Pairings/MOV/FR | Partial: actual composite/F25controls; actual k1cyclic9group, true quotient/target and all-valid-auxiliary proper Miller-domain ND with rationalrestriction disproofs (116); original Verheul printed-formula disproof and actual point-group/cyclic repair with order3control (49) | Canonical divisor/valuation identification and generic MOV/FR theorems; original Frey–Rück unread; general Verheul End/Tate-module theorem5 |
| 5 Diem factor base | Partial: excluded subfield family, qualified arithmetic estimate and unrestricted small-field counterexample | Scheme/admissibility geometry and qualified general factor-base bound |
| 6 Gaudry smoothness | Partial: exact small-model counts and finite reduced-pair checks | Whole geometric Jacobian identification and fixed-genus asymptotic error proof |
| 7 Subgroup smoothness | Partial: complete finite Cantor-group counterexamples and growing family with all-pair arithmetic/group consistency; fixed-genus2/unbounded-field actual Cantor image-group/order2cyclic counterfamily, all ambient pairs and even-extension control (60) | Full ambient group/Picard embedding and qualified typical/large-prime asymptotics |
| 8 Bézout/multiplicity | Partial: actual length/support/native16/JV8; literal F2torus-helper disproof/point-chart repair (31); generic actual solution/quotient-map equivalence and finite scalar-length bound, unbounded actual bivariate nilpotent family with m0/m1/domain controls (44) | General n!dⁿ/decomposition node unrefuted; scheme-cycle/local multiplicity, isolated points in positive-dimensional components, projective geometry and Rojas/C runtime/refinement |
| 9 Gröbner heuristics | Partial: actual Edwards universal n=2 counterexample; 18 native source-system calls; literal JV ideal/source-field/group proofs; actual eight-element quotient basis and sharp monomial cutoff4 | Generic/n>=3 and larger m,n sweeps, full internal F4 refinement and canonical graded Hilbert-function identification |
| 10 Rho walks | Partial: actual first-collision linear lower bounds and unbounded-prime fixed-support disproof, exact product-PMF variance theorem and finite populations | Qualified random-table collision penalty and asymptotic mixing proofs |
| 11 Standards | Actual 160-bit anomalous curve/cardinality and concrete Smart finite-program certificate; finite repository rho run 768/768, measured replays 576/576, exact A/A controls and Lean report-arithmetic gate 11/0/0 | Generic p-adic/complexity/native refinement and qualified asymptotic collision law |
| 12 msolve/F4 | Partial: exact F4Run ideal-containment theorem, saturation-reading counterexample and source/run audit | Ordinary C refinement and complete trace binding; saturation hang beyond the tested coefficient lookup repair |
| 13 Independence | Requested actual separate-host primeS3 replay completed in public run43501438026, both languages match all6fields/942models/4188162triples; canonical export2/2/0excluded passes | Compiler/runtime/physical-host proof and independence from the same author remain open. Native attestation PCC-20261113-9b23b3not_covered/unsigned; original manualHTTP404 unchanged. |
| 14 Negative control | Current control passes locally | Historical runs were not preregistered with it; no retroactive promotion |

The earlier direct-host capability search and manualHTTP404 were historical
infrastructure gaps. A new frozen main-push workflow was admitted through
normal PR/merge gates and actually ran on a standard hosted runner; see
the separate-host packet below. No paid cloud resource was launched.
[pcc:unverified]

The source-reading/access gaps and missing mathematical formalizations above
remain explicit. No claim that all 44 cited sources were read or all fourteen
original statements were proved/disproved is made. Local Lean PASS and signed
independent standing remain separate; the new packets are unsigned.
[pcc:unverified]

### 2026-11-10 unbounded prime-order family extension

The [prime-family project](../research/mumford_prime_family_audit_20261110/README.md)
uses the actual pinned Mathlib Dirichlet theorem, with proved coprimality5/24,
to extract arbitrarily large t with24t+5prime. It combines actual family
cardinality/order, all-pair normalized arithmetic, generation and fraction1
versusambient<1. Unbounded prime-order disparity witnesses and a direct
universal prime-family equality negation are proved. Fullgate5/0/0;
allfiveattempt/source and fivegatefiles retained losslessly. [pcc:unverified]

This extends the earlier arithmetic results to unbounded prime orders.
The earlier source receipts did not claim this extension and remain unchanged.
Genus grows with order; full ambient/Picard identification and typical,
dominant-prime or fixed-genus asymptotic statements remain open. The host
question for item13remains pending; authorized local proof work continued
while that input was unavailable. [pcc:unverified]

### Registered-runner availability recheck

A fresh read-only GitHub preflight confirms the repository is public and the
prepared manual S3workflow endpoint still returns404. The frozen protocol
already permits a free standard public GitHub runner. The remaining issue
is a callable registered workflow/access route. [Actual receipt](../research/independent_s3_validator_20261109/evidence/registration-probe-20261110.json).
No new dispatch/native attempt or paid resource was submitted. [pcc:unverified]

[AGENTS.md lines861–870](../AGENTS.md) requires research workflows on main,
schedule or manualdispatch. The separate frozen-workflow replay job restores
30unrelatedstudies and readsAWSartifacts; it is not an S3executionadapter.
No PRresearchtrigger, signer reroute, frozen protocol or original rejection
was changed. The userhost/workflowidentifierquestionremains pending.
[pcc:unverified]

### 2026-11-10 actual first-collision counterfamily

The [collision project](../research/rho_collision_audit_20261110/README.md)
defines the least actual repeated state and proves p≤rT and T≤p for
every labeling with increments1..r. The actual independent uniform node
PMF satisfies p≤rE[T] and E[T]≤p. With r20, distinct/nonzero support
and actual indegree variance19/20 hold; no realC bounds E[T]²≤Cp
uniformly over all primesp>20. Whole gate35/0/0. [pcc:unverified]

Galbraith14.2.1 chooses the increment table uniformly, while this family
fixes1..20. Hildebrand's main average also samples support. This known
fixed-support obstruction disproves a universal variance-only reading;
qualified average/random-table laws remain open. No new experiment,
paid compute or novelty claim. Exact statement pins and failed attempts
are retained; PCC replay and signed standing recorded in the study.
[pcc:unverified]

### 2026-11-10 bounded solver diagnostic

Eight new frozen cells cross exactLA1/2, original/combined-sanitizer builds
and ordinary/saturation. OriginalLA1ordinary exact output passes; LA2
ordinary139andbothsat5stimeouts remain. Allfourcombinedsanitizer cells
stop at earlier hash.c409signed1shift31. This does not establish cause
of the original failures. No source edit, Crefinement or generic-ideal
regularity conclusion. All8outcomes retained; export passesfilechecks;
PCC-20261110-743f20not_covered/unsigned. A separate predata address-only
amendment is frozen to inspect beyond that first-error check. [pcc:unverified]

### 2026-11-10 concrete sparse lookup and Lean invariant

Address-only cells locate the17bitLA2READfault atla_ff_32.c543.
A new predata common-logged2x2experiment records old[0,2,0] to
new[2,3,4],livebasis3; baselineordinaryreads basis3andaborts134,
one matrix-lookup backport passes its exact ordinary known answer.
Both saturation cells stilltimeout5s. Currentupstream and pinned16bit
alreadyusematrixlookup; no noveltyclaim. All14prior relevantF65537
lineagecallsand4newcells retained, exportpasses; native4867bbnot_covered.
[pcc:unverified]

A separate [Lean project](../research/msolve_reindex_lean_audit_20261110/README.md)
defines the three operational copy updates, proves correct matrix lookup
for every coefficient content, direct wrong-table universal negation,
and an in-range wrong-value counterexample. Fullgate14/0/0.
This is snapshot table semantics, with no assumed Cexecution/trace,
physicalallocation bound, parser or wholeF4Run refinement conclusion.
Earlier signedshift and saturation diagnosis remain open. [pcc:unverified]

### 2026-11-10 actual Miller expressions and quotient domains

The [Miller project](../research/pairing_miller_audit_20261110/README.md)
proves actualF25field, E:y²=x³+1, actual3torsioncard9/additiveindexhom,
both81-entryMillerexpressiontables and allusednonzeroevaluationvalues.
Actualallpoint6R=0 provesunique3torsionrepresentativesofE/3E,card9.
Actualunitfinalexponent8hascubesaskernelandissurjectiveontocuberoots;
bothquotient/rootstargetscard3. Raw/reducedMillerclassmapsarebilinear
andnondegenerateonactualquotientdomains. Wholegate126/0/0. [pcc:unverified]

ActualFrobenius(a,b)=(a,-b)identifiesbaseF5rationaltorsionwithPline,
Qnonrational. TheWeilMillerrestrictionisconstant1anddirectlyfailsND.
BaseF5unit/cubetargettrivialprecludesanyleftNDmapfromactualnonzero
3torsion. Tangenttripleintersectionpolynomialidentitiesareprovedby
actualcoefficientchecksanduniversalringalgebra. Canonicaldivisor/
valuationidentification,genericMOV/FRandcomplexityremainopen; no
sourceNDtheoremisassumed. OriginalFR94correctDOIsuffix-6confirmed
byCrossrefbutprimaryPDFstill403/unread. No nativeexperimentorpaidcompute.
[pcc:unverified]


### 2026-11-12 actual geometric torsion bridge

The [dependent geometric study](../research/pairing_geometric_torsion_audit_20261112/README.md)
proves that every actual3-torsion point of E:y²=x³+1 in any field receiving
an F5-algebra embedding of F25 descends from an actual F25 torsion point.
The actual restricted point homomorphism is an additive equivalence.
Over every algebraically closed characteristic5 extension, the embedding
is constructed by IsAlgClosed.lift, and actual geometric3-torsion has
cardinality9 and an actual (Z/3)² basis. Frobenius acts by(a,-b), and the
actual F5-rational image is exactlyb0. No general rank/basis hypothesis.
The exact final gate/packet receipts are in that study. [pcc:unverified]

This closes geometric coverage for this concrete curve/r3; genericallcurve
BK rank/determinant/Weil and the p<500 census remain open. Canonical
principal-divisor/valuation identification of Miller expressions and generic
MOV/Frey–Rück remain open. All other original requirements retain the
matrix's remaining obligations. No paid compute or new native experiment
belongs to this formal extension. [pcc:unverified]

### 2026-11-12 finite repository rho measurement

The requested finite benchmark component of item11 is now recorded in
[the rho audit](../research/rho_constant_audit_20261112/README.md): 768/768
verified calls, 576 measured replays, 192 measured and 64 warmup identical
same-seed pairs, with a separate detected counter-corruption control.
The protocols were committed before their data, all counts and fixed setup
remain explicit, and the completed main export passes file/contract checks.
All scientific clocks are invalid; no timing conclusion is made.
[pcc:unverified]

The separate Lean report project passes 11 declarations, zero failures and
zero trusted-native exceptions. It proves exact arithmetic on the recorded
totals, rational mean intervals and emitted-counter ordering reversal. The
finite means above sqrt(pi/4) do not contradict its qualified asymptotic
average. Native execution and signed independent validation remain separate
assurance scopes. This closes this measurement component, not every open
method in the original fourteen-item request. [pcc:unverified]

### 2026-11-12 general actual geometric3-torsion count

[The general short-curve project](../research/three_torsion_rank_audit_20261112/README.md)
proves exactly nine actual points killedby3 for every nonsingular short
Weierstrass coefficient pair over algebraically closed fields of
characteristic other than2and3. Explicit coordinate equivalence precedes
cardinality; whole gate20/0/0, no rank/basis premise. This closes the
generalr3 geometric count component. Genericallr determinant/BK and
p<500 census remain open. [pcc:unverified]

### 2026-11-13 nonzero, finite Edwards descent

The [new Edwards project](../research/edwards_descent_audit_20261113/README.md)
passes its full55-declaration Lean gate with zero failures and zero trusted
exceptions. It checks a nonsingular twisted-Edwards input over F25, actual
factor-base points and their Mathlib group sum, the exact source F3
symmetrization and actual coefficient descent. Both equations are nonzero
quadratics; an actual algebra equivalence to F5[T]/T² gives dimension2,
finiteness and an Artinian quotient. Their actual highest homogeneous
parts directly fail Mathlib regularity. This strengthens the earlier
[f,0] witness for the literal universal n=2 reading. It supplies no generic
or n>=3 refutation and does not contradict conditional Theorem4.1.
[pcc:unverified]

Joux–Vitse's author manuscript is now read at pp4–5/7/9–10, including its
actual F101³ toy systems and exact Assumption2 about homogenized Gröbner
computation. That condition is not the Edwards regularity statement.
Source-known toy cases provide the next concrete native execution targets;
the specified FGHR/generic/JV finite runs remain open until actually frozen,
executed and analyzed. No native outcome is inferred from the Lean witness.
[pcc:unverified]

### 2026-11-13 original-source Gröbner executions

The [new native study](../research/groebner_system_audit_20261113/README.md)
completes all18frozen calls: two calibrations and eight systems twice,
including the actual Edwards affine/highest-parts/homogeneous input and
both original Joux–Vitse F101³ worked examples affine/homogeneous.
All16solver outputs match independent native GB/source/remainder/S-pair
checks and all8GB repeats are byte-identical. Canonical export/full file
checks pass. This closes those finite source-known execution components,
not the larger generic or m,n populations. [pcc:unverified]

The empty JV homogenized example has a returned GB of degree4 and finite
staircase length8. Its raw F4selected-pair degree5round adds0new
polynomials/twozero reductions. The source's internal maximum and intended
Assumption2 computation are not fully traced or formally refined, so this
is not promoted to a disproof. Source-field descent and native execution
remain unsigned/native; the separate55-declaration Edwards kernel proof is
preserved at its actual scope. [pcc:unverified]

### 2026-11-13 literal JV algebraic cutoff

The [new39-declaration Lean project](../research/jv_macaulay_certificate_20261113/README.md)
proves the first printed JVtoy affine coordinate ideal is unit, and its
actual homogeneous ideal is proper and contains every monomial of total
degree at least4. Explicit polynomial identities precede actual ideal
membership and quotient relations; no generic regularity or native GB
oracle is assumed. Wholegate39/0failures/0trusted. This provides an
algebraic uppercutoff and an independent mathematical control for the
native degree5zero-reduction diagnostic. [pcc:unverified]

The cutoff's minimality, exact Hilbert function/length8, actual F101³curve
and coefficient-descent identification, and full internal F4execution
refinement remain separate. No generic/JV computation-degree refutation
is claimed. Primary source scopes and failed attempts remain explicit.
[pcc:unverified]

### 2026-11-13 actual JV source-field and group bridge

The separate [59-declaration source-field project](../research/jv_source_field_audit_20261113/README.md)
closes the first empty example's actual field/curve/descent/group gap.
Lean constructs F101[t]/(t³+t+1), proves irreducibility and field cardinality
1,030,301, checks the nonsingular source curve and nonzero printed target,
and extracts exactly the three printed coefficient equations through actual
basis-coordinate maps. The earlier affine unit-ideal certificate and checked
S3 group equivalence imply that every sum or signed sum of two actual
factor-base points avoids this target. Whole gate59/0failures/0trusted;
standard foundations only. This confirms the stated example's behavior.
[pcc:unverified]

The earlier cutoff project remains scoped to its original polynomial-ideal
statement; this new versioned packet supplies the additional semantic bridge.
Source whole-curve order1,029,583/scalar relations, the second split example,
minimal cutoff/exact length8, generic JV and internal F4 refinement remain
open. No native execution, independent-host or corpus status is promoted.
[pcc:unverified]

### 2026-11-13 exact JV coordinate algebra and sharp cutoff

The [new exact-algebra project](../research/jv_hilbert_audit_20261113/README.md)
has a final51-declaration whole gate with zero failures/trusted-native
exceptions. An actual coefficient functional annihilates every polynomial
multiple of each literal source generator but takes h³ to1. Its complete
multiplication annihilator is proved to be an ideal containing the source
ideal. Thus h³ is outside that ideal and the earlier monomial cutoff4 is
sharp: the all-high-monomials property at thresholdd holds iff4≤d.
[pcc:unverified]

All64dual identities prove independence of the proposed actual quotient
classes. All24coordinate multiplication relations, polynomial induction
and quotient surjectivity prove spanning. Lean constructs the actual
basis(1,x,y,h,xh,yh,h²,h³), yielding finrank8 and Module.length(F101,A)=8.
Over every characteristic101 field, the source equations hold exactly
at the origin. This separates geometric support1 from scalar-field
algebra length8, providing a further item8 control. [pcc:unverified]

The earlier39/59packets and176native artifact pins remain unchanged.
This closes the literal monomial-minimality and scalar-field dimension
gap; it does not prove internal F4/solving degrees, a canonical graded
Hilbert-function identification or generic JV/FGHR statements. Source
whole-curve order/scalar relations and all14 broader obligations remain.
[pcc:unverified]

### 2026-11-13 actual S3 absolute irreducibility and symmetry

The [new64-declaration S3 project](../research/semaev_irreducibility_audit_20261113/README.md)
proves actual short-form S3 irreducibility for every field with2≠0,
arbitrarya,b, and after every field homomorphism/scalar extension. This
includes algebraic closure. Its exact evaluation agrees with the earlier
SemaevS3.S3 definition. Actual permutation symmetry follows from the
checked elementary-symmetric expression. Whole gate64/0failures/0trusted.
[pcc:unverified]

The proof includes the actual nested rational-function discriminant,
odd-degree nonsquare/root exclusion, actual coefficient embedding,
proved primitive condition, Gauss bridge and explicit algebra equivalences
to MvPolynomial(Fin3). No irreducibility or isomorphism assumption stands
in for these steps. The direct F2/a=b0nonirreducibility control factors
the literal short polynomial as a nonunit square; this tests the
restriction rather than refuting the qualified source or distinct
ordinary binary model. [pcc:unverified]

Originalp3uses the algebraic closure of the entire function field K(X3),
as confirmed by visual inspection of the PDF and enlarged paragraph.
Text extraction loses the bar; no field-of-definition counterexample
is asserted. The later full-symmetry and general-split packets below close those obligations. Higher-arity absolute irreducibility remains open.
No native or corpus status is promoted. [pcc:unverified]

### 2026-11-13 degree-drop control and corrected resultant interchange

The [new resultant-interchange project](../research/resultant_interchange_audit_20261113/README.md)
proves the fixed-size iterated Sylvester-resultant identity over every
commutative integral domain and derives both intermediate degree bounds
from explicit bidegree bounds. Root multisets retain multiplicity;
injective coefficient/swap maps descend through algebraic closure and
fraction field. Whole gate46/0failures/0trusted, with standard foundations
only; exact local PCC-20261113-97e420 replay passes, signed standing
unverified. [pcc:unverified]

The actual rational control P=y²−y,Q=xy+1,R=2x+1 has inner polynomials
x+1,y−2 and Q bidegrees1. Lean directly proves strict intermediate
degree loss and natural-size sides−1 versus2; retained formal sizes give
2both. The natural-size compatibility corollary explicitly requires
sharp intermediate degree equalities. This refutes an unrestricted helper,
not the qualified Semaev source or the actual canonical family. The later full-symmetry and general-split packets below bind the actual canonical recursive polynomials, variable maps and all splits. Higher-arity absolute irreducibility remains open. [pcc:unverified]

### 2026-11-13 actual source-order two-step normalization

The [new reassociation project](../research/semaev_reassociation_audit_20261113/README.md)
proves equality of the actual two existing resultantStep polynomials and
the balanced source-ordered expression with existing lastPolynomial(S4).
The actual recursivePolynomial W(n+2) is identified with this expression
at every stage, so the k=2/S4-block split is normalized for every arity≥5.
All declared sizes are retained; zero P, repeated coordinates and every
characteristic are included. The source-order theorem uses actual curve
b-invariants; no nonsingularity is needed for the polynomial identity.
Whole gate60/0/0 and exact local PCC-20261113-cc3069 replay pass; signed standing remains unverified. [pcc:unverified]

Independent coordinates and a generic leading coefficient are specialized
by actual ring homomorphisms, and the generic triple evaluation is proved
the identity on the formal polynomial ring. Actual-curve S4 middle
transposition and cyclic rotation close the source argument-order gap;
earlier within-pair/block swaps alone did not supply this bridge. The
direct rational b=(0,0,0,1) negative control disproves an unrestricted
independent-b extension, and Lean proves no rational Weierstrass curve
has these coefficients. No qualified source theorem is refuted.
The subsequent full-symmetry and general-split packets close those obligations. Higher-arity absolute irreducibility remains open. [pcc:unverified]


### Full actual canonical input-permutation symmetry

The [new symmetry project](../research/semaev_symmetry_audit_20261113/README.md)
proves the literal canonical polynomial is fixed by every permutation of all
inputs at every arity≥3, for every field and actual general-Weierstrass curve.
Its actual multivariate image is IsSymmetric through explicit ring equivalences;
input coordinates, coefficient-field fixation and outer coordinate0=X are
proved. No characteristic/nonsingularity restriction is needed for this
polynomial identity. Whole gate90/0failures/0trusted and exact local PCC-20261113-cda3d2
replay pass; signed standing remains unverified. The subsequent general-split packet closes all source splits. Higher absolute irreducibility remains open. The inherited rational independent-b
negative control refutes only that unrestricted extension. [pcc:unverified]


### All-extension actual BK excluded-boundary certificate

The [new F7project](../research/bk_unipotent_audit_20261113/README.md) proves
actual geometric3torsioncard9 and full point images over every chosen field
of size7^k iff3dividesk, for every positivek and every compatible embedding.
Cyclotomic divisibility holds for allk; embeddingdegree1 andr²divides#E9.
The explicit infinite3n+1family refutes dropping the source exclusion;
original Theorem1 and its Remarks already distinguish this boundary.
Whole gate32/0failures/0trusted and exact local PCC-20261113-da72ba replay pass;
signed standing remains unverified. Generic all-prime rank/determinant/Weil
and the complete all-extension census remain open. [pcc:unverified]


### Actual k1 pairing-domain certificate

The new [actual k1 pairing certificate](../research/pairing_k1_audit_20261113/README.md)
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

### Actual separate-host continuation

Item13 now has an actual [separate-host two-language S3 replay](../research/s3_host_replay_20261113/README.md).
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

### Actual ordinary binary S3absolute irreducibility

The [ordinary binary S3project](../research/binary_semaev_irreducibility_audit_20261113/README.md)
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
The subsequent general-split packet proves all source splits. Higher-arity absolute irreducibility remains open.
[pcc:unverified]

### Qualified BK linear step and actual rational input

The [qualified BK linear packet](../research/bk_linear_audit_20261113/README.md)
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

### Every literal source-ordered Semaev resultant split

The [general source-split packet](../research/semaev_general_split_audit_20261113/README.md)
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

### Fixed genus, unbounded fields: actual arithmetic counterfamily

The [fixed-genus arithmetic packet](../research/mumford_fixed_genus_audit_20261113/README.md)
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

### Actual geometric 2-torsion rank and characteristic boundary

The [actual geometric2torsion packet](../research/two_torsion_rank_audit_20261113/README.md)
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

### Diem two-torus helper: actual F2 obstruction and rational-point repair

The [Diem chart packet](../research/diem_chart_audit_20261114/README.md)
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

### Generic actual coordinate-length accounting and unbounded family

The [coordinate-length packet](../research/coordinate_length_audit_20261114/README.md)
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

### Original Verheul distortion formula and actual repair

The [original Verheul source audit](../research/verheul_distortion_audit_20261114/README.md)
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
at recorded scope; additive [coverage](../research/verheul_distortion_audit_20261114/coverage.json)
does not promote the frozen unread labels or imply a full-paper read.
No novelty or native experiment claimed. [pcc:unverified]


### Original Thériault finite collision lower bound

The [original Thériault collision audit](../research/theriault_collision_audit_20261114/README.md)
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


### Actual generic iid collision recurrence bridge

The [generic actual iid bridge](../research/theriault_iid_audit_20261114/README.md)
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
