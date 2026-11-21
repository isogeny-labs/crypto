"""Final report from reconciled native receipts; no new mathematics."""
from pathlib import Path
import json,html,hashlib
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());c=r['construction_counts']
assert r['status_counts']=={'rejected':4,'accepted':16}
assert c['audited_unique_theorems']==11 and c['structured_marked23_algebra_dimension']==112
panels=[
('Normal-chart geometry',[
 ('64 independent generator identities','Both 32-column maps; source products'),
 ('Actual nu is integral','All 32 module entries; full lift'),
 ('Actual nu diamond map','32 entries; fixed clearing 121'),
 ('11 audited mathematical statements','Two audited modules; zero trust')]),
('Portable source and larger algebra',[
 ('Universal QR polynomial certificate','Two products; explicit denominator'),
 ('Explicit quadratic F2 twist','All original equations descend'),
 ('Structured marked23 source','Six source equations; rank 112'),
 ('Next: primality and selected action','Primality and selected action')]),
('Packet and kernel relations',[
 ('QR packet is a line','16 packets; point algebra 48'),
 ('Generic Tate3 quotient','Separable; four kernel choices'),
 ('Polynomial3/7 collision certificate','Polynomial Bezout; no inverse A'),
 ('Complete finite QR7 action','1,008 matrices; no 16-cycle')])]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="470" viewBox="0 0 1200 470"><rect width="1200" height="470" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#17304a">','<text x="20" y="32" font-size="25">R51: exact source geometry and constructive relation leads</text>']
for col,(title,nodes) in enumerate(panels):
 x=20+col*395;svg += [f'<text x="{x}" y="{64}" font-size="18">{title}</text>']
 for row,(label,note) in enumerate(nodes):
  y=82+row*84;color='#edf0f3' if label.startswith('Next:') else '#d4eddf'
  svg += [f'<rect x="{x}" y="{y}" width="382" height="72" rx="7" fill="{color}"/>',f'<text x="{x+11}" y="{y+27}" font-size="20">{html.escape(label)}</text>',f'<text x="{x+11}" y="{y+51}" font-size="18">{html.escape(note)}</text>']
svg += ['<text x="20" y="452" font-size="14">Strict native receipts: results.json. Premises and proof scope: CM.md / PROOF_MAP.md.</text>','</g></svg>']
(h/'round51.svg').write_text('\n'.join(svg)+'\n')
def txt(s):return 'text('+json.dumps(s)+')'
rows=[]
for row in r['runs']:
 receipt=f'compile {row["compile_exit"]}; audit {row["audit_exit"]}' if row.get('kind')=='Lean' else f'exit {row["native_exit"]}'
 if row.get('error_lines'):receipt+='; '+str(len(row['error_lines']))+' error(s)'
 rows += [txt(row['attempt']),txt(row['status']),txt(receipt)]
source='''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R51: integral coordinates and the actual QR7 packet
The ordinary universal QR variables are the slope and intercept of the line
through P,2P,4P. The actual descended level 77 nu is integral in the retained
normal-chart algebra and has a checked diamond formula. A full polynomial
3/7 kernel certificate gives controlled Velu collision support. The marked23
source now has a complete structured algebra of dimension 112 over F2(x).
#image("round51.svg", width: 100%)

== Source coordinates and packet geometry
All 32 normalization generators have localized maps in both directions.
A fresh process verifies 64 direct numerator products,both complete 32-column
PID products and all 4 source-equation lifts. The actual nu has a full 32-entry
PID lift. Its diamond/sign products and localized 32-entry lift pass,with
fixed exponent 121=62+59 from the actual source coefficients.

For A=1+c,the QR packet line y=lambda*x+nu meets the Tate curve in
#raw("x³+(b+lambda²+A*lambda)x²+(A*nu+b*lambda)x+nu²+b*nu")
The degree 48 point algebra has a separable cubic,source-compatible doubling,
three-doubling identity and a full chord-recovery unit. A monic degree 24 point
equation has a full source membership certificate. All 5 serialized products
recheck. This supplies the actual packet coordinates for the next isogeny map.

== Universal descent and larger source
The archived universal QR matrices clear to polynomial identities with
D=(1+c)^5*Delta^3. Every original QR and triangular equation descends under
the quadratic gamma twist;G_twisted=G+(1+c)^16. The marked23 source retains
all 6 coupled equations and both complete matrices in auxiliary B,C. Actual
base units,raw-to-normal f23-multiple identities and nu normalization pass.
Its dimension 112 is an algebra statement;primality and selected 991 action
are subsequent tests. CM.md states the full geometric and source premises.

#pagebreak()
= Complete native record and formal scope
There are20 terminal attempts:16 accepted and4 rejected. Late markers after
question-mark errors are rejected even with native exit 0. The expanded
coefficient attempt hit its original 1200 s cap. No accepted producer was
repeated to repair publication. Both compiled Lean artifacts are retained.
#set text(size: 9pt)
#table(columns: (2.1fr,1fr,1.8fr),stroke:0.4pt,
[Attempt],[Status],[Receipt],
'''+',\n'.join(rows)+''')
#set text(size: 10pt)
== Eleven audited mathematical statements
Overlap51 has 5 theorems:three exact characteristic2 overlap identities,a
localized inverse,and inverse induced maps under injective compatible
embeddings. Specialize51 has 6:denominator-unit cancellation,scaled ideal
equality,the8P=P to7P=0 consequence,line intersection,quartic difference,
and the cross-kernel inverse from the Bezout/point identities.

Both modules compile and pass the axiom audit with zero failing and zero
trusted/native_decide declarations. Lean4.34.1/mathlibd13f23b remain pinned.
Native matrices,actual point/Bezout identities,complete RR and geometric
open-cover premises remain external inputs. Proper polarization and selected
modular-factor action are separate constructions. See PROOF_MAP.md.

== Decisive next normalization test
Construct multiplication by the now-integral nu on the full 32-generator
algebra. Starting from the original source power-basis module,close under
that matrix with the fixed Cayley-Hamilton bound 31. A full recovery of the
normal basis would give the specific four-coordinate affine model;otherwise
record the exact remaining PID cokernel/support. The old P67 RR embedding
is retained. Then transport the cubic packet through Velu and verify the
actual QR source equations,F2 descent and target action.

#pagebreak()
= Finite action and direct-relation construction
#image("cycles51.svg",width:100%)
The enumeration is complete,not sampled. It rules out a 16-cycle in the stated
QR action;it does not obstruct generic connectedness. The first good odd
split candidates are23 at level 77 and 11 at level 161. Their actual residual
eigenvalues still need checking.
#image("hecke-lead51.svg",width:100%)
The generic Tate3 map has full curve/order/unit products and serialized
replay. The collision matrices give exact norms Delta,Delta^5,Delta^16 and
U*psi3+V*psi7=Delta^16 with polynomial coefficients. No inverse A is used,so
this certificate includes good A=0 fibers. The explicit cross-kernel inverse
is Q*U(x)/Delta^16,using the quartic divided difference Q.

T3 is an inert-prime annihilator constraint. Construct its actual carrier
correspondence and compatible integral or odd-adic module before claiming
a projector scalar. A matrix only on F2 differentials does not certify that
integral/sign decomposition. Primary sources and the external formalization
lead are recorded in NEW-LEADS51.md;no external theorem is counted as locally
audited. Raw custody and source-byte checks are separate from scientific replay.
'''
(h/'report.typ').write_text(source)
