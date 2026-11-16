"""Lay out proved statements and recorded evidence; no research computation."""
from pathlib import Path
import json, fitz
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing, Rect, Line, String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

r = Path(__file__).resolve().parent
o = r / 'reports'
o.mkdir(exist_ok=True)
pdfmetrics.registerFont(TTFont('DV', '/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'))
pdfmetrics.registerFont(TTFont('DVB', '/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'))
pdfmetrics.registerFontFamily('DV', normal='DV', bold='DVB')
body = ParagraphStyle('body', fontName='DV', fontSize=9, leading=12, spaceAfter=7)
title = ParagraphStyle('title', fontName='DVB', fontSize=17, leading=20, spaceAfter=12)
def p(s): return Paragraph(s, body)
blue = colors.HexColor('#234b70')
fig = Drawing(500, 120)
boxes = [(0, 68, 'Split boundary polynomials', 'Root multisets + multiplicity'),
         (262, 68, 'Interchange nested products', 'Keep declared degrees and sign'),
         (0, 0, 'Every integral domain', 'Injective fraction-field descent'),
         (262, 0, 'Every coefficient field', 'Injective algebraic-closure descent')]
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="120" viewBox="0 0 500 120">']
for x, y, a, b in boxes:
    fig.add(Rect(x, y, 238, 48, rx=5, fillColor=colors.HexColor('#edf5fa'), strokeColor=blue))
    svg.append(f'<rect x="{x}" y="{120-y-48}" width="238" height="48" rx="5" fill="#edf5fa" stroke="#234b70"/>')
    for k, txt in enumerate([a, b]):
        fig.add(String(x+119, y+29-k*15, txt, fontName='DV', fontSize=9, textAnchor='middle', fillColor=blue))
        svg.append(f'<text x="{x+119}" y="{120-y-29+k*15}" text-anchor="middle" font-family="DejaVu Sans,sans-serif" font-size="9" fill="#234b70">{txt.replace("&", "&amp;")}</text>')
for x1,y1,x2,y2 in [(238,92,261,92),(381,68,381,49),(262,24,239,24)]:
    fig.add(Line(x1,y1,x2,y2,strokeColor=blue))
    svg.append(f'<path d="M{x1},{120-y1} L{x2},{120-y2}" stroke="#234b70"/>')
svg.append('</svg>')
(o/'proof-flow.svg').write_text('\n'.join(svg)+'\n')
gate = json.loads((r/'evidence/gate.json').read_text())
flow = [Paragraph('Resultant interchange survives degree drops', title),
    p('<b>Proved component.</b> Over every commutative integral domain, iterated fixed-size Sylvester resultants commute under explicit bidegree bounds. Both intermediate degree bounds are derived. Signed standing: unverified. [pcc:unverified]'),
    p('Write a=deg P(y), c=deg R(x), and deg<sub>x</sub> Q≤dx, deg<sub>y</sub> Q≤dy. Sizes after the semicolon are declared sizes, retained after specialization. [pcc:unverified]'),
    p('<b>Res<sub>x</sub>(Res<sub>y</sub>(P,Q; a,dy),R; a·dx,c)<br/>= Res<sub>y</sub>(P,Res<sub>x</sub>(Q,R; dx,c); a,dy·c).</b> [pcc:unverified]'),
    p('No nonzero, monicity, splitting, separability, characteristic or intermediate degree-preservation assumption is added. Natural outer sizes also work if both intermediate degree bounds are sharp; that condition is explicit in a separate theorem. [pcc:unverified]'),
    fig, Spacer(1,10),
    p('<b>Direct negative control over ℚ.</b> P=y²−y, Q=xy+1, R=2x+1. Both Q bidegrees are1. The inner polynomials are x+1 and y−2; the first has degree1 below formal bound2. [pcc:unverified]')]
tab=Table([['Outer sizes','Left','Right'],['Natural degrees','−1','2'],['Retained bounds','2','2']],colWidths=[315,85,85])
tab.setStyle(TableStyle([('FONTNAME',(0,0),(-1,-1),'DV'),('FONTSIZE',(0,0),(-1,-1),9),('BACKGROUND',(0,0),(-1,0),colors.HexColor('#edf5fa')),('GRID',(0,0),(-1,-1),0.3,blue),('BOTTOMPADDING',(0,0),(-1,-1),7),('TOPPADDING',(0,0),(-1,-1),7)]))
flow += [tab,Spacer(1,10),
    p('<b>Scope.</b> This refutes an unrestricted helper that resets degrees. It does not refute Semaev’s qualified theorem or establish a failure of its canonical family. Actual arbitrary-split normalization, higher symmetry and higher absolute irreducibility remain separate obligations. [pcc:unverified]'),
    p(f'<b>Evidence.</b> Whole Lean gate: {gate["declarations"]} declarations, 0 failures, 0 trusted-native. Lean4.35.0 / Mathlib56a00585…. Standard foundations only; no holes, custom axioms, native evaluation, raised limits or waivers. See sources.json, proof-map.json and evidence/ for exact source/attempt/gate pins. [pcc:unverified]'),
    p('<b>Sources.</b> Semaev2004 Theorem1, p2 arbitrary-split formula; Mathlib Resultant/Basic and Polynomial/Bivariate/BigOperators; repository DeclaredResultantRoots, ResultantDegreeStep and RecursiveSemaevDegree/Symmetry. KB unavailable; bounded fallback, no novelty claim. No native experiment, paid compute or agent upload. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'), pagesize=(595.276,841.89), leftMargin=47,rightMargin=47,topMargin=38,bottomMargin=38).build(flow)
doc=fitz.open(o/'report.pdf')
layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
    bad=[]
    for block in page.get_text('dict')['blocks']:
        for line in block.get('lines',[]):
            for span in line['spans']:
                x0,y0,x1,y1=span['bbox']
                if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height: bad.append(span['text'])
    layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad})
    page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/resultant-interchange-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n')
print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_within_pages':all(x['text_within_page']for x in layout['pages'])}))
