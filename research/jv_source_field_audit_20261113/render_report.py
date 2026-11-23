"""Render the proof map and explanatory diagram; no research computation."""
from pathlib import Path
import json
import fitz
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, KeepTogether
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing, Rect, Line, String, Polygon
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

root=Path(__file__).resolve().parent
out=root/'reports'
out.mkdir(exist_ok=True)
styles=getSampleStyleSheet()
pdfmetrics.registerFont(TTFont('DejaVuSans','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DejaVuSans-Bold','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DejaVuSans',normal='DejaVuSans',bold='DejaVuSans-Bold')
styles.add(ParagraphStyle(name='BodySmall',fontName='DejaVuSans',fontSize=8.2,leading=10.8,spaceAfter=5))
styles.add(ParagraphStyle(name='SmallTitle',fontName='DejaVuSans-Bold',fontSize=14,leading=18,spaceAfter=8))
style=styles['BodySmall']
def p(s): return Paragraph(s,style)

svg='''<svg xmlns="http://www.w3.org/2000/svg" width="500" height="136" viewBox="0 0 500 136">
<defs><marker id="arrow" markerWidth="7" markerHeight="7" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6" fill="#244b70"/></marker></defs>
<g fill="#edf5fa" stroke="#244b70" stroke-width="1.2">
<rect x="2" y="4" width="151" height="45" rx="5"/><rect x="174" y="4" width="151" height="45" rx="5"/><rect x="346" y="4" width="151" height="45" rx="5"/>
<rect x="346" y="83" width="151" height="45" rx="5"/><rect x="174" y="83" width="151" height="45" rx="5"/><rect x="2" y="83" width="151" height="45" rx="5"/>
</g><g stroke="#244b70" fill="none" marker-end="url(#arrow)">
<path d="M154,26 L172,26"/><path d="M326,26 L344,26"/><path d="M421,50 L421,81"/><path d="M345,105 L327,105"/><path d="M173,105 L155,105"/>
</g><g font-family="DejaVu Sans,sans-serif" font-size="10" fill="#16354f" text-anchor="middle">
<text x="77" y="23">Actual extension field</text><text x="77" y="38">degree 3, cardinality 101³</text>
<text x="249" y="23">Nonsingular source curve</text><text x="249" y="38">actual nonzero target R</text>
<text x="421" y="23">Literal source S3</text><text x="421" y="38">exact symmetrization</text>
<text x="421" y="101">Actual coefficient maps</text><text x="421" y="117">three printed equations</text>
<text x="249" y="101">Earlier unit-ideal proof</text><text x="249" y="117">no simultaneous zeros</text>
<text x="77" y="101">Actual point group</text><text x="77" y="117">P + Q and P − Q avoid R</text>
</g></svg>'''
(out/'proof-flow.svg').write_text(svg)
flow=Drawing(500,136)
blue=colors.HexColor('#244b70')
for x,y,lines in [(2,87,['Actual extension field','degree 3, cardinality 101³']),
                  (174,87,['Nonsingular source curve','actual nonzero target R']),
                  (346,87,['Literal source S3','exact symmetrization']),
                  (346,8,['Actual coefficient maps','three printed equations']),
                  (174,8,['Earlier unit-ideal proof','no simultaneous zeros']),
                  (2,8,['Actual point group','P + Q and P − Q avoid R'])]:
 flow.add(Rect(x,y,151,45,rx=5,ry=5,fillColor=colors.HexColor('#edf5fa'),strokeColor=blue))
 for n,label in enumerate(lines):
  flow.add(String(x+75.5,y+26-n*15,label,textAnchor='middle',fontName='DejaVuSans',fontSize=9,fillColor=blue))
for x1,y1,x2,y2 in [(154,110,172,110),(326,110,344,110),(421,86,421,55),(345,31,327,31),(173,31,155,31)]:
 flow.add(Line(x1,y1,x2,y2,strokeColor=blue))
 if y1==y2:
  dx=1 if x2>x1 else -1
  flow.add(Polygon([x2,y2,x2-5*dx,y2+3,x2-5*dx,y2-3],fillColor=blue,strokeColor=blue))
 else:
  flow.add(Polygon([x2,y2,x2-3,y2+5,x2+3,y2+5],fillColor=blue,strokeColor=blue))
story=[Paragraph('The actual field behind the JV empty example',styles['SmallTitle']),
p('Lean source-field bridge | Item9 | 13 November 2026 | [pcc:unverified]'),
p('The first printed Joux–Vitse F101³ example is nondecomposable into two factor-base points. This project proves that statement for the actual Mathlib elliptic-curve point group, by connecting the source coefficients to the previously proved affine unit ideal.'),
flow,Spacer(1,8),
p('<b>Exact input.</b> K = F101[t]/(t³+t+1), with alpha = t. In the basis (1,alpha,alpha²), a=(44,52,60), b=(58,87,74), xR=(85,84,37), yR=(15,3,86). The cubic is irreducible and |K|=1,030,301. The short discriminant expression 4a³+27b² has coordinates (23,95,23), hence is nonzero. R is an actual nonzero point, and xR lies outside the embedded base field.'),
p('<b>Exact descent.</b> The source S3 at (x,y,xR) equals the symmetrized polynomial at (x+y,xy). Genuine coefficient maps in the quotient-field basis give exactly the printed three equations. Their affine ideal is the whole polynomial ring by the earlier characteristic101 ideal certificate, so there is no base-field solution.'),
p('<b>Group implication.</b> InFactorBase consists of all actual affine points with base-field x-coordinate, including either sign. The checked S3 equivalence covers distinct, repeated, tangent and vertical cases. For every pair P,Q from that set, both P+Q and P−Q differ from R.'),
p('<b>Proof map.</b> All names below have namespace JVSourceFieldAudit.'),]
rows=[['Source file','Principal declarations'],
['SourceField.lean','modulus_irreducible; field_cardinality; coordinate_representation'],
['ActualCurve.lean','curve_nonsingular; target_nonzero; target_x_not_base'],
['SourceDescent.lean','source_polynomial_identity; actual_descent; source_S3_has_no_base_solution'],
['ActualDecomposition.lean','actual_target_not_decomposable; literal_jv_source_field_certificate']]
t=Table([[p(x)for x in row]for row in rows],colWidths=[120,380])
t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#edf5fa')),('VALIGN',(0,0),(-1,-1),'TOP'),('LINEBELOW',(0,0),(-1,0),0.5,colors.grey),('LEFTPADDING',(0,0),(-1,-1),4),('RIGHTPADDING',(0,0),(-1,-1),4)]))
story += [t,Spacer(1,7),
p('<b>Assurance and limits.</b> Lean4.35.0, Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679. Standard foundations only: propext, Classical.choice, Quot.sound. No holes, custom axioms, native_decide, waivers or raised elaboration limits. Local gate/replay is distinct from independently signed standing. No new native experiment or paid compute.'),
p('This confirms the source example; it is not a generic regularity or internal F4 theorem. Whole-curve order1,029,583, source scalar relations, the split example, minimal cutoff and exact Hilbert length8 remain outside this certificate. All14 broader acceptance requirements remain tracked separately.'),
p('<b>Primary source.</b> <link href="https://www-fourier.ujf-grenoble.fr/~viva/research/articles/DLFq5.pdf" color="blue">Joux–Vitse author manuscript, page7</link>, PDF SHA256 prefix6f3268a0ef8c; full hash and read scopes in evidence/source-review.json. Previous ideal proof: research/jv_macaulay_certificate_20261113. S3 group equivalence: research/theorem_audit_20261108/lean/S3Completion.lean. KB unavailable (MCP absent/wrapper127); bounded primary/repository/Mathlib fallback. No novelty claim. [pcc:unverified]')]
doc=SimpleDocTemplate(str(out/'report.pdf'),pagesize=(595,842),leftMargin=47,rightMargin=48,topMargin=31,bottomMargin=31)
doc.build(story)
pdf=fitz.open(out/'report.pdf')
pages=[]
for i,page in enumerate(pdf):
 bad=[list(b[:4])for b in page.get_text('blocks')if b[0]<0 or b[1]<0 or b[2]>page.rect.width or b[3]>page.rect.height]
 assert not bad,(i,bad)
 page.get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(f'/workspace/scratch/jv-source-field-report-{i+1}.png')
 pages.append({'page':i+1,'text_blocks_in_bounds':True})
(out/'layout.json').write_text(json.dumps({'pages':pages,'visual_inspection':'pending'},indent=2)+'\n')
print(json.dumps({'pages':len(pdf),'pdf_bytes':(out/'report.pdf').stat().st_size}))
