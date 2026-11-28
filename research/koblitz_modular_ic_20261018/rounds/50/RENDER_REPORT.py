"""Render recorded results; no native mathematics or new measurements."""
from pathlib import Path
import html
import json
import re

h = Path(__file__).resolve().parent
r = json.loads((h / "results.json").read_text())
assert not r["status_counts"].get("running"), "finish or reconcile native attempts first"
c = r["construction_counts"]
assert c["global_coordinate_defect_F2_dimension"] == 992
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="290" viewBox="0 0 900 290">',
       '<rect width="900" height="290" fill="white"/>',
       '<g font-family="DejaVu Sans,sans-serif" fill="#10263b">',
       '<text x="24" y="36" font-size="23">Complete coordinate defect over F₂</text>',
       '<text x="24" y="64" font-size="15">Actual plane → integral-v refinement; defect50-01</text>']
for y, label, value, color in [(100, "Finite chart", 898, "#176ca4"),
                              (170, "Infinity (z=0)", 94, "#d07924")]:
    width = value / 898 * 600
    svg += [f'<text x="24" y="{y+23}" font-size="17">{label}</text>',
            f'<rect x="215" y="{y}" width="{width:.3f}" height="34" fill="{color}"/>',
            f'<text x="{225+width:.3f}" y="{y+23}" font-size="17">{value}</text>']
svg += ['<text x="24" y="245" font-size="16">Total: 992 dimensions. Arithmetic-genus difference: 1953 − 961 = 992.</text>',
        '<text x="24" y="273" font-size="14">Deterministic exact lengths; units: dimension over F₂. No statistical uncertainty.</text>',
        '</g></svg>']
(h / "round50.svg").write_text("\n".join(svg) + "\n")

def txt(s):
    return 'text(' + json.dumps(s) + ')'

rows = []
for row in r["runs"]:
    receipt = (f'exit {row["native_exit"]}' if row["kind"] == "Singular" else
               f'compile {row.get("compile_exit")}, audit {row.get("audit_exit")}')
    rows += [txt(row["attempt"]), txt(row["status"]), txt(receipt)]
source = '''#set page(paper: "a4", margin: 16mm)
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
F₂[t,u]/(A,B). Its Bézout splitting isolates the cyclic block of dimension128,
with (u+1)⁷ nonzero and (u+1)⁸ zero. Full finite and infinity lengths are898
and94. The infinity truncations at exponents1,2,4,8,16,32 give dimensions
16,32,64,86,94,94, including the multiplicities. Transition weights independently
give arithmetic genera1953 and961. These are native identities and written
geometric deductions; the linked notes state their hypotheses.

== Formal algebra
Twelve unique mathematical declarations compile and pass the axiom audit:
six Rational50 norm, coordinate, field-map and overlap lemmas; one Separation50
evaluation-separation lemma; and three Local50 scaling, constructive CRT and
nilpotency lemmas;two Marked23_50 binary-prefix and prime-order lemmas.
Zero failing or trusted declarations. Actual CAS arrays,
scheme gluing and complete pole geometry are outside these Lean propositions.
Lean4.34.1/mathlib d13f23b, one thread,4096MiB,1200s per compile/audit.

#pagebreak()
== Complete RR algebra and larger marked base
All68 descended sections and196 complete-RR products have actual32-entry
field vectors,complete denominator identities and full reconstruction.
Coefficient inversion transports them to z=1/u. The PID module has rank32,
common denominator degree37 and index length376 over F₂. Both original-column
lifts and the entire source power-basis embedding pass.

All528 unordered products of the32 basis functions are checked in the actual
source quotient. The complete32-by528 table identity and corruption control
pass. A fresh process reloads the actual serialized parents,products,table
and source embeddings and rechecks all complete identities. This gives an
explicit finite free source algebra. The written RR and Nakayama argument
identifies it with the affine normalization under the retained genus61,
proper source,pole support and complete-space hypotheses. Those hypotheses
are stated in the notes and not formalized by the conditional Lean lemmas.

The known sign quotient now has monic degree8 equations on both proper
charts,full coupled-source identities,generic ranks16 and32,and five overlap
products. Its arithmetic genus is449. It supplies source geometry for
reconstructing the anti-invariant double cover.

The larger marked23 base passes over the actual prime degree7 characteristic2
field. The Tate-coordinate inverses,nonzero discriminant,2P/3P controls and
every binary prefix1,2,5,11,23 pass. The final point is infinity and P is
nonidentity,hence has exact order23. All actual arrays are retained. Next
apply only universal QR7 equations S3-S5 to this base;degree112 remains a
prospective extension value.

The next proper-source gluing uses the RR algebra and its diamond transport.
The overlap begins with h0=z+V,h1=z+V+z²;full module lifts must certify
regularity of every transported generator,including exceptional branches.
A complete132-section finite-u RR intersection from saved1476-dimensional
spaces is a separate route. See NEXT_SOURCE_NORMALIZATION.md.

#pagebreak()
== Actual attempts and receipts
'''
source += f'{len(r["runs"])} attempts: {r["status_counts"].get("accepted",0)} accepted, {r["status_counts"].get("rejected",0)} rejected.\n'
source += '#set text(size: 8pt)\n#table(columns: (2fr,1fr,2fr), inset: 4pt,\n'
source += ',\n'.join([txt("Attempt"), txt("Status"), txt("Receipt"), *rows]) + ')\n#set text(size: 9pt)\n'
source += '''
== Preserved failures and next construction
The unsupported coefficient-field extgcd path and direct polynomial division
path fail explicitly; supported Euclid and leading-term division repair them
in fresh attempts. Two narrow-import Lean compiles fail before the accepted
separation module. The weighted plane normalizer reaches normalP and caps at
1200s without module output. The refined finite normalizer exports candidate
arrays but fails a coefficient-display-string equality: Singular displays
the prime field as ZZ/2. That producer remains rejected. A separately frozen
finite verifier hits1200s in the22-variable standard-basis construction.
The repaired infinity producer exports a14-variable presentation with delta376;
its independent presentation verifier remains prepared,unexecuted.

The first marked23 native attempt fails a reserved identifier;the repaired
run passes. Its first Lean compile misses distributivity;the repaired build
and audit pass. The PID writer's comment header swallows a declaration;
the first table consumer rejects. The repaired consumer strips only that
comment prefix,rechecks both full saved module products,and the accepted
table subsequently passes a fresh reload. Every failure remains indexed.

All native CAS invocations use4GiB virtual memory,1200s,seed50 and one thread,
serially. Failed late PASS markers never count. R27–R49 stay sealed.
CPU time was not instrumented;actual native UTC wall timestamps are retained.
No new research EKS job, Diem/freezer restart, Conductor claim or agent merge.
This report describes component checks; independent scientific replay and
semantic review have not been performed. See LOCAL-GEOMETRY.md,
DEFECT-GEOMETRY.md, RR-NORMALIZATION-LEAD.md and the exact attempt logs.
'''
source = re.sub(r"\b(All|all|and|actual|rank|degree|length|genus|the|of|index|delta|with|by|at|is|prefix|exponent|complete)(?=\d)", r"\1 ", source)
source = re.sub(r",(?=\S)", ", ", source)
source = re.sub(r"(?<=\d)(?=GiB|MiB|s\b)", " ", source)
(h / "report.typ").write_text(source)
report = f'''# R50: explicit RR algebra,proper charts and larger marked base

The complete RR space now gives an explicit rank32 algebra over F2[z],with
index length376 and all528 multiplication products. A fresh process reloads
and checks the complete table and source embedding. Its normalization
interpretation uses the retained proper/genus/pole/RR hypotheses,stated in
[RR-ALGEBRA.md](RR-ALGEBRA.md). The larger marked23 base also passes the
actual degree7 prime,coordinate inverses,discriminant and exact order23 check.

The checked plane-to-integral-v coordinate defect has total F2 dimension992:
898 on the finite chart and94 at infinity. Its boundary contains the exact
128-dimensional cyclic block, with saturation exponent8. The actual matrix,
Bézout coefficients and all parameter-power normal forms are retained.

There are {len(r["runs"])} attempts: {r["status_counts"].get("accepted",0)} accepted,
{r["status_counts"].get("rejected",0)} rejected. Twelve unique conditional algebra
theorems pass Lean compile and audit, with zero failing/trusted declarations.
Every actual receipt, failed compile, cap, error and late marker is indexed
in [results.json](results.json). The report source, recorded-value chart and
PDF are report.typ, round50.svg and report.pdf.

See [local algebra](LOCAL-GEOMETRY.md), [complete coordinate defect](DEFECT-GEOMETRY.md),
[proper geometry](GEOMETRY.md), [candidate verification](NORMAL-CANDIDATE-RECOVERY.md),
and [RR module construction](RR-NORMALIZATION-LEAD.md). Sources and limits are
recorded in the prospective protocols. These are component observations and
written implications, with exact claim boundaries in the notes.

Next constructions are the explicit localized diamond gluing and the larger
QR7 extension in [NEXT_SOURCE_NORMALIZATION.md](NEXT_SOURCE_NORMALIZATION.md).
The complete-RR132-section finite-u intersection is retained as another route.
The existing factor-map and ordinary-divisor tasks retain their separate
obligations.
'''
(h / "REPORT.md").write_text(report)
