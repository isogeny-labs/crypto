# Fourteen-target audit: remaining formal bridges and execution prerequisites

- **Status:** open
- **Kind:** proof gaps, implementation gaps and access/tool limits; no impossibility claim
- **Affected work:** [fourteen-target audit](../research/theorem_audit_20261108/README.md)
- **Next action owner:** audit continuation; environment owner for credential/host provisioning
- **Last updated:** 2026-11-10

## Problem

The request for all fourteen source audits, experiments and faithful Lean proofs
is incomplete: local checkpoints are now authorized and bounded native
components have run, while independent-host execution/main integration and
several actual curve/Jacobian/implementation bridges remain unresolved.
[pcc:unverified]

## Evidence and uncertainty

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

1. The documented bounded Semaev census has run. Continue remaining
   resultant proofs beyond m4/degree proofs and the all-p<500torsion/binary/ternary census
   obligations under their exact original scope, with a new frozen protocol
   before affected native work. Never convert finite agreement into a
   universal theorem. [pcc:unverified]
2. For item7, provide the concrete Jacobian/factor-base/subgroup family and a
   faithful Lean group/divisor bridge; the abstract confinement theorem is
   already available and is insufficient by itself. [pcc:unverified]
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
generic polynomial symmetry/irreducibility/arbitrary-split normalization
remain open. See [the original-source study](../research/semaev_source_audit_20261110/README.md).
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
| 1 Semaev | Partial: canonical all-arity zero-sum equivalence/degrees and literal binary S4 display disproof | Full polynomial symmetry, absolute irreducibility and arbitrary-split normalization |
| 2 BK | Partial: excluded-case disproof and actual qualified F5/r3 criterion at every positive extension degree | Generic torsion rank/determinant/Weil bridges and complete p<500 census |
| 3 Supersingular | Partial: direct exact-degree-two counterexamples; complete general-model checks over F2/F3/F4/F9 | Higher extensions, general Hasse/classification and geometric/torsion-field bridges |
| 4 Pairings/MOV/FR | Partial: actual composite Tate-domain counterexamples and pairing-domain controls | Canonical Weil/Tate constructions and all stated MOV/FR domains; original Frey–Rück unread |
| 5 Diem factor base | Partial: excluded subfield family, qualified arithmetic estimate and unrestricted small-field counterexample | Scheme/admissibility geometry and qualified general factor-base bound |
| 6 Gaudry smoothness | Partial: exact small-model counts and finite reduced-pair checks | Whole geometric Jacobian identification and fixed-genus asymptotic error proof |
| 7 Subgroup smoothness | Partial: complete finite Cantor-group counterexamples and growing family with all-pair arithmetic/group consistency | Full ambient group/Picard embedding and qualified typical/large-prime asymptotics |
| 8 Bézout/multiplicity | Partial: actual quotient length/support witness and 16 native checks | General n!dⁿ bound and decomposition-system hypotheses |
| 9 Gröbner heuristics | Partial: Lean counterexample to universal regularity reading | Generic/Joux–Vitse statements and requested exact system measurements |
| 10 Rho walks | Partial: exact product-PMF variance theorem, duplicate-step counterexample and finite populations | Collision-time penalty and qualified asymptotic mixing proofs |
| 11 Standards | Partial: actual 160-bit anomalous curve/cardinality and concrete Smart finite-program certificate | Generic p-adic/complexity/native refinement and matched SafeCurves benchmark |
| 12 msolve/F4 | Partial: exact F4Run ideal-containment theorem, saturation-reading counterexample and source/run audit | Ordinary C refinement, crash/hang diagnosis and complete trace binding |
| 13 Independence | Blocked for the requested other-machine replay | Prior dispatch HTTP404 ran no job; one runtime available; a different authorized host or registered workflow is required |
| 14 Negative control | Verified current control | Historical runs were not preregistered with it; no retroactive promotion |

The accessible runtime has no callable second execution host. A fresh tool
capability search did not expose a separate browser/remote execution host;
the only environment service describes this task's existing managed runtime.
The user has been asked for the missing host/workflow identifier, not for
reapproval of the already authorized replay. No paid cloud resource has been
launched. [pcc:unverified]

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
