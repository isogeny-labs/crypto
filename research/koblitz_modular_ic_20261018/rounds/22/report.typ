#set page(paper: "a4", margin: (x: 18mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false, leading: 0.6em)
#set heading(numbering: none)
#let summary = json("runs/round22-native-summary.json")
#let validation = json("validation-summary.json")
#let repo = "https://github.com/isogeny-labs/crypto/tree/codex/koblitz-r22-integral-recovery/research/koblitz_modular_ic_20261018/rounds/22"
= Koblitz cover transfers: Round22
Exact integral recovery and the proper-geometry frontier

Started2026-10-26;validation2026-10-27 UTC

Scientific input: #raw("69f67409f5fe6b525c23b000edea047dd38fcf83") \
Publication parent: #raw("3c87193e7c754e9abefa401b7cd0974d0708b3b1") \
Conductor T-409 · public synthetic fixtures · no clock or IC efficiency result.

== The exact recovery calculation succeeds
Replacing the four regular target cycles by their actual integral augmentation
factors gives least recovery scalar *1*. The complete integer system has eight
rows and 32 columns. All eight Smith factors are 1; the integral kernel has rank
24. Stored transformations and two-sided integer inverses certify completeness.
An independent integer representation and a separate coefficient-pairing
construction agree. No coefficient-box cutoff is imposed.

With $w^2+w+2=0$ and the actual augmentation rotation $R$, one small certificate is

$ M_1=(2+w)+7R+(6+2w)R^3, quad M_3=7I, $
$ B_1=(1+3w)+(2+5w)R+3R^2+(1+2w)R^3, $
$ B_3=(-2+4w)-3R^2+(-2+3w)R^3, $
$ B_0=B_2=0, quad B_1 M_1+B_3 M_3=I. $

Both blocks commute with $R$. Native CM products and independent integer products
agree; all 22 order-11 fixtures pass across both ordinary Koblitz signs, including
the identity. A separate universal integer polynomial identity and actual matrix
bindings are kernel checked. The full 24-parameter Smith solution family remains
available; the small certificate is just one representative.

The inherited augmentation metric is $G=J^T J=I+"ones"(4)$, with Smith factors
1,1,1,5. The original weighted Gram is retained. R21's sharp denominator 5 theorem
for regular target cycles is still valid. The retraction on norm-zero subobjects
does not construct a map to the level-77 modular factor or the final polarized
quotient.

== Acceptance criteria still open
#table(columns: (1.05fr, 2.9fr), inset: 5pt, stroke: .4pt + gray,
  [*Track*], [*Observed status / exact gap*],
  [Integral recovery], [Complete in the stated equivariant CM augmentation category.],
  [Proper geometry], [Complete base divisors and generic tower; normalized lifted branches, complete prescribed RR and subgroup map remain open.],
  [Node187], [Coupled valuative exclusion under written bridges; no explicit polynomial separator or h-power membership.],
  [Direct relations], [Requires an arbitrary modular-divisor reverse evaluator; ordinary 460/367 fibers and images remain open.])

#pagebreak()
= What the exact evidence connects
#image("round22.svg", width: 100%)

The bars compare different target objects in the same algebraic recovery question.
They are exact scalar values, not timing samples, speedups or an IC improvement.
The dashed links in the lower diagram identify outstanding obligations.

Raw certificate: #raw("runs/g32-normzero-integral-recovery.json") \
Independent verifier: #raw("runs/g32-independent-integer-replay.json") \
Small certificate: #raw("runs/g33-small-two-map-recovery.json")

The native negative controls return scalar 2 for the system multiplied by 2,
no positive scalar for the zero system, and reject a corrupted recovery block.
The inherited metric check rejects replacing $G$ by the identity.

#pagebreak()
= Proper level-77 geometry and coupled divisors
== Actual geometry now recorded
The Tate11 base is birational to the smooth genus-one cubic
$v^2+v=u^3+u$. Complete base divisors of $b,c,delta_0,1+c,b+c+c^2,x(4P)$
and the generic $nu$ denominator are checked by independent native arithmetic,
local series, infinity orders and exact norm factors. The two residual closed
supports have degree five. The supersingular support of $1+c$ is essential to
the exceptional $nu$ geometry; the generic lambda projection loses information
there while the original incidence open is smooth.

The original SS5 fiber now has both ideal-inclusion certificates, dimension 16,
invertible registered units and Jacobian determinant $b^4 != 0$. Its exact q32
Frobenius matrix gives four residue fields of degree 4 over F32, hence four
closed points of degree 20 over F2. This retains original $nu$; the complete
proper normalized fiber accounting remains a separate geometric bridge.

The generic QR presentation has degree 16. Both lift matrices, the irreducible
degree-32 plane projection, birational coordinate recovery and the smooth
registered open are checked separately. All 245 rational circuit gates pass
exact cross-multiplication checks. The exact trace variable
$z=lambda^2+(1+c)lambda+b$ gives a degree-8 subfield and quadratic lambda lift.

The degree-96 and degree-72 place calculations exhaust the 4 GiB cap. Affine
normalization and a local integral-basis attempt reach the 1200 second cap.
The smaller degree-16 trace plane has total degree 56; its place calculation
also exhausts 4 GiB. Its initial metadata-print error is retained and corrected
metadata is checked separately. No failed run supplies proper normalized places.

Every lifted coordinate and registered denominator divisor, including exceptional
branches, is still required before a complete prescribed Riemann–Roch search.
A candidate must satisfy exact function-field identities, descent compatibility
and nonzero action on the order-11 subgroup. A shared residual Hecke system does
not establish that action. Independent modular equations are only a model check
after certifying their relationship to this incidence component.

== Why the old node187 models cannot be full arcs under the written bridges
All four actual programs and five coefficient conjugates are expanded: 80 ordinary
functions $F_j=P_j(x)+y S_j(x)$, denominator 1, degrees 9 and 8, monic $S_j$.
Each has its only pole at infinity, of order 19 and leading coefficient 1.
The actual gates contain $F_0 F_1$ at node187 and $F_0 F_1 F_2$ at node247.

For an arc with $"ord"(h)=N>0$, the old scalar residues force
$v_0,v_1>-N$ and $v_0+v_1=-N$, hence both are negative. Proper reduction and
the complete coefficient descent force the third source point to infinity too,
so $v_2<0$. But regularity of the actual node247 coordinate requires
$"ord"(h F_0 F_1 F_2)=v_2>=0$, a contradiction. The joint leading coefficient
condition is retained in every conjugate; no separate nineteenth-root argument
for a single factor replaces it.

Lean proves this implication with the proper/pole hypotheses explicit. It is
not a formalized scheme/DVR bridge or an extracted polynomial certificate.
The requested explicit separator with $h^k f in I$, full quotient saturation
and proper polarized embedding remain open.

#pagebreak()
= Validation, failures and reproducibility
The exact observed validation counts below are loaded from the native sealed
ledger, rather than estimated from build targets.

#table(columns: (2.2fr, 2.2fr), inset: 6pt, stroke: .4pt + gray,
 [*Check*], [*Observed*],
 [New named Lean declarations], [#validation.new_named_Lean_theorems],
 [All audited project theorems], [#validation.audited_project_theorems; zero failing, zero trusted native],
 [Integrated library tests], [#validation.root_library_tests.passed passed; #validation.root_library_tests.ignored ignored],
 [Integrated native tests], [#validation.native_tests.passed passed; #validation.native_tests.ignored ignored],
 [Exact recovery fixtures], [22 across both ordinary signs],
 [Coupled native controls], [129024 integer assignments; 80 actual conjugate functions])

The project pins Lean 4.34.1 and mathlib d13f23b. The complete build and transitive
audit accept only propext, Classical.choice and Quot.sound. No sorry, user axiom
or trusted native evaluation is admitted. The proof map names every declaration
and identifies all native and written geometric bridges.

Preserved failures include initial generated Lean errors, Singular API/syntax
errors despite process exit zero, all bounded normalization failures, an
incompatible F8-coordinate Klein invocation, interrupted integrated builds,
the inherited SAT closing-brace defect and absent sparse include fixtures.
The same brace already exists on current main; the exact tracked fixtures were
restored without weakening tests. Matching F512-coordinate Klein and F32 n5
controls replay separately. Their transfer domains remain distinct from the
unconstructed F2 modular-factor map.

Migration receipts and existing jobs were reconciled before retrying local
work. Diem's managed owner and original deadline were preserved. No duplicate
generator or EKS job was launched; the separate waiting-freeze lane was untouched.

== Evidence and primary references
The source-linked Markdown report, RECOVERY, CM, SATURATION, PROOF_MAP,
VALIDATION, ledger, deterministic raw archive and SHA-256 manifest live
#link(repo)[in the Round22 directory]. The directory artifact manifest points
to immutable objects in the user-specified setec-astronomy-artifacts bucket.
The original scientific input and publication parent are recorded on page 1.

Primary sources: #link("https://math.mit.edu/~drew/X1_optcurves.html")[Sutherland's modular models];
#link("https://github.com/Singular/Singular")[Singular's native normalization libraries];
#link("https://stacks.math.columbia.edu/tag/01J8")[Stacks valuation specialization].
These inform the methods; none is claimed to prove the source-specific recovery
or to identify this incidence model with the independent modular model.

No relation-rank promotion, complete modular transfer or IC efficiency verdict
is supplied by this report. Missing geometric/evaluator deliverables remain
part of the original user-authorized task.
