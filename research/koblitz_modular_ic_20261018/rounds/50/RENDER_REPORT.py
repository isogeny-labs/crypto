"""Render recorded results; no native mathematics or new measurements."""
from pathlib import Path
import html
import json

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
    return '#text(' + json.dumps(s) + ')'

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
Ten unique mathematical declarations compile and pass the axiom audit:
six Rational50 norm, coordinate, field-map and overlap lemmas; one Separation50
evaluation-separation lemma; and three Local50 scaling, constructive CRT and
nilpotency lemmas. Zero failing or trusted declarations. Actual CAS arrays,
scheme gluing and complete pole geometry are outside these Lean propositions.
Lean4.34.1/mathlib d13f23b, one thread,4096MiB,1200s per compile/audit.

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
verifier tests its hash-bound arrays independently, using characteristic2 and
zero coefficient parameters and the original full acceptance conditions.
Every later attempt and its exact outcome is indexed in results.json.

The RR normalization lead uses the already complete196-section L(2D0) basis.
A written RR and Nakayama argument makes this a possible full affine
normalization-module construction over F₂[1/u], with the retained proper/pole
bridge made explicit. The frozen rational-vector producer evaluates those
actual saved functions before any PID-module calculation. The marked23
model and larger-n plan remain saved separately in SCALEUP.md.

All native CAS invocations use4GiB virtual memory,1200s,seed50 and one thread,
serially. Failed late PASS markers never count. R27–R49 stay sealed. No new
research EKS job, Diem/freezer restart, Conductor claim or agent merge.
This report describes component checks; independent scientific replay and
semantic review have not been performed. See LOCAL-GEOMETRY.md,
DEFECT-GEOMETRY.md, RR-NORMALIZATION-LEAD.md and the exact attempt logs.
'''
(h / "report.typ").write_text(source)
report = f'''# R50: explicit charts and exact coordinate saturation

The checked plane-to-integral-v coordinate defect has total F2 dimension992:
898 on the finite chart and94 at infinity. Its boundary contains the exact
128-dimensional cyclic block, with saturation exponent8. The actual matrix,
Bézout coefficients and all parameter-power normal forms are retained.

There are {len(r["runs"])} attempts: {r["status_counts"].get("accepted",0)} accepted,
{r["status_counts"].get("rejected",0)} rejected. Ten unique conditional algebra
theorems pass Lean compile and audit, with zero failing/trusted declarations.
Every actual receipt, failed compile, cap, error and late marker is indexed
in [results.json](results.json). The report source, recorded-value chart and
PDF are report.typ, round50.svg and report.pdf.

See [local algebra](LOCAL-GEOMETRY.md), [complete coordinate defect](DEFECT-GEOMETRY.md),
[proper geometry](GEOMETRY.md), [candidate verification](NORMAL-CANDIDATE-RECOVERY.md),
and [RR module construction](RR-NORMALIZATION-LEAD.md). Sources and limits are
recorded in the prospective protocols. These are component observations and
written implications, with exact claim boundaries in the notes.
'''
(h / "REPORT.md").write_text(report)
