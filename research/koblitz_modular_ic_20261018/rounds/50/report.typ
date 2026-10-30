#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= R50: explicit proper charts and exact coordinate saturation
The source field has an explicit monic degree-32 model on two affine charts.
The integral-v refinement and its overlaps are checked against that field.
The complete coordinate defect has dimension 992 over F₂, including a cyclic
128-dimensional boundary block with exact parameter exponent 8.
#image("round50.svg", width: 100%)

== Checked construction
The quadratic norm recovers v and n rationally. Native Euclidean certificates
give the common denominator inverse; all four original forward source
equations satisfy full denominator-cleared polynomial products, with corrupted
witnesses rejected. The scaled t=u⁴λ equation is monic of degree 32.
Two monic charts and their complete overlap products construct a finite
locally free proper model. The intermediate ring retains integral v with a
monic degree-16 equation over v²+v=u³+u; both norm products and all twelve
relative overlap products pass.

The actual u=1 fiber separates two refined degree-2 points above one plane
point. A source-module presentation identifies the coordinate defect with
F₂[t, u]/(A, B). Its Bézout splitting isolates the cyclic block of dimension128,
with (u+1)⁷ nonzero and (u+1)⁸ zero. Full finite and infinity lengths are898
and 94. The infinity truncations at exponents1, 2, 4, 8, 16, 32 give dimensions
16, 32, 64, 86, 94, 94, including the multiplicities. Transition weights independently
give arithmetic genera1953 and 961. These are native identities and written
geometric deductions; the linked notes state their hypotheses.

== Formal algebra
Twelve unique mathematical declarations compile and pass the axiom audit:
six Rational50 norm, coordinate, field-map and overlap lemmas; one Separation50
evaluation-separation lemma; and three Local50 scaling, constructive CRT and
nilpotency lemmas;two Marked23_50 binary-prefix and prime-order lemmas.
Zero failing or trusted declarations. Actual CAS arrays,
scheme gluing and complete pole geometry are outside these Lean propositions.
Lean4.34.1/mathlib d13f23b, one thread, 4096 MiB, 1200 s per compile/audit.

#pagebreak()
== Complete RR algebra and larger marked base
All 68 descended sections and 196 complete-RR products have actual 32-entry
field vectors, complete denominator identities and full reconstruction.
Coefficient inversion transports them to z=1/u. The PID module has rank 32,
common denominator degree 37 and index length 376 over F₂. Both original-column
lifts and the entire source power-basis embedding pass.

All 528 unordered products of the 32 basis functions are checked in the actual
source quotient. The complete 32-by 528 table identity and corruption control
pass. A fresh process reloads the actual serialized parents, products, table
and source embeddings and rechecks all complete identities. This gives an
explicit finite free source algebra. The written RR and Nakayama argument
identifies it with the affine normalization under the retained genus 61,
proper source, pole support and complete-space hypotheses. Those hypotheses
are stated in the notes and not formalized by the conditional Lean lemmas.

The known sign quotient now has monic degree 8 equations on both proper
charts, full coupled-source identities, generic ranks16 and 32, and five overlap
products. Its arithmetic genus is 449. It supplies source geometry for
reconstructing the anti-invariant double cover.

The larger marked23 base passes over the actual prime degree 7 characteristic2
field. The Tate-coordinate inverses, nonzero discriminant, 2P/3P controls and
every binary prefix 1, 2, 5, 11, 23 pass. The final point is infinity and P is
nonidentity, hence has exact order23. All actual arrays are retained. Next
apply only universal QR7 equations S3-S5 to this base;degree 112 remains a
prospective extension value.

The next proper-source gluing uses the RR algebra and its diamond transport.
The overlap begins with h0=z+V, h1=z+V+z²;full module lifts must certify
regularity of every transported generator, including exceptional branches.
A complete 132-section finite-u RR intersection from saved1476-dimensional
spaces is a separate route. See NEXT_SOURCE_NORMALIZATION.md.

#pagebreak()
== Actual attempts and receipts
30 attempts: 20 accepted, 10 rejected.
#set text(size: 8pt)
#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
text("Attempt"),
text("Status"),
text("Receipt"),
text("boundary50-01"),
text("accepted"),
text("exit 0"),
text("candidate50-01"),
text("accepted"),
text("exit 0"),
text("chart50-01"),
text("accepted"),
text("exit 0"),
text("defect50-01"),
text("accepted"),
text("exit 0"),
text("denominator50-01"),
text("rejected"),
text("exit 0"),
text("denominator50-02"),
text("accepted"),
text("exit 0"),
text("fiber50-01"),
text("accepted"),
text("exit 0"),
text("forward50-01"),
text("rejected"),
text("exit 0"),
text("forward50-02"),
text("accepted"),
text("exit 0"),
text("lean50-01"),
text("accepted"),
text("compile 0, audit 0"),
text("local-lean50-01"),
text("accepted"),
text("compile 0, audit 0"),
text("local50-01"),
text("accepted"),
text("exit 0"),
text("marked-lean50-01"),
text("rejected"),
text("compile 1, audit None"),
text("marked-lean50-02"),
text("accepted"),
text("compile 0, audit 0"),
text("marked23-50-01"),
text("rejected"),
text("exit 0"),
text("marked23-50-02"),
text("accepted"),
text("exit 0"),
text("normal-finite50-01"),
text("rejected"),
text("exit 124"),
text("normal-relative-finite50-01"),
text("rejected"),
text("exit 0"),
text("normal-relative-infinity50-01"),
text("accepted"),
text("exit 0"),
text("quotient50-01"),
text("accepted"),
text("exit 0"),
text("relative50-01"),
text("accepted"),
text("exit 0"),
text("rr-field50-01"),
text("accepted"),
text("exit 0"),
text("rr-mult50-01"),
text("rejected"),
text("exit 0"),
text("rr-mult50-02"),
text("accepted"),
text("exit 0"),
text("rr-pid50-01"),
text("accepted"),
text("exit 0"),
text("rr-table-reload50-01"),
text("accepted"),
text("exit 0"),
text("separation50-01"),
text("rejected"),
text("compile 1, audit None"),
text("separation50-02"),
text("rejected"),
text("compile 1, audit None"),
text("separation50-03"),
text("accepted"),
text("compile 0, audit 0"),
text("verify-relative-finite50-01"),
text("rejected"),
text("exit 124"))
#set text(size: 9pt)

== Preserved failures and next construction
The unsupported coefficient-field extgcd path and direct polynomial division
path fail explicitly; supported Euclid and leading-term division repair them
in fresh attempts. Two narrow-import Lean compiles fail before the accepted
separation module. The weighted plane normalizer reaches normalP and caps at
1200 s without module output. The refined finite normalizer exports candidate
arrays but fails a coefficient-display-string equality: Singular displays
the prime field as ZZ/2. That producer remains rejected. A separately frozen
finite verifier hits1200 s in the 22-variable standard-basis construction.
The repaired infinity producer exports a14-variable presentation with delta 376;
its independent presentation verifier remains prepared, unexecuted.

The first marked23 native attempt fails a reserved identifier;the repaired
run passes. Its first Lean compile misses distributivity;the repaired build
and audit pass. The PID writer's comment header swallows a declaration;
the first table consumer rejects. The repaired consumer strips only that
comment prefix, rechecks both full saved module products, and the accepted
table subsequently passes a fresh reload. Every failure remains indexed.

All native CAS invocations use4 GiB virtual memory, 1200 s, seed50 and one thread,
serially. Failed late PASS markers never count. R27–R49 stay sealed.
CPU time was not instrumented;actual native UTC wall timestamps are retained.
No new research EKS job, Diem/freezer restart, Conductor claim or agent merge.
This report describes component checks; independent scientific replay and
semantic review have not been performed. See LOCAL-GEOMETRY.md,
DEFECT-GEOMETRY.md, RR-NORMALIZATION-LEAD.md and the exact attempt logs.
