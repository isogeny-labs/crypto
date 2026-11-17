"""Layout only: render the implementation-status report. No result computation."""
from pathlib import Path
from xml.sax.saxutils import escape
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle
from reportlab.graphics.shapes import Drawing, Rect, String, Line
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

root = Path(__file__).resolve().parent
fontdir=Path('/usr/share/fonts/truetype/dejavu')
pdfmetrics.registerFont(TTFont('Sans',str(fontdir/'DejaVuSans.ttf')))
pdfmetrics.registerFont(TTFont('Sans-Bold',str(fontdir/'DejaVuSans-Bold.ttf')))
pdfmetrics.registerFontFamily('Sans',normal='Sans',bold='Sans-Bold',italic='Sans',boldItalic='Sans-Bold')
boxes = [(12, 145, "Subfield kernel + u scan", "Implemented: native line builder", "#dceee9"),
         (292, 145, "Four-summand 2LP reference", "Implemented: exact bounded MITM", "#dceee9"),
         (12, 45, "Algebraic C1 + full pricing", "Missing: blocks C1 comparison", "#fae7d0"),
         (292, 45, "L2 execution + independent audit", "Blocked: no submission/checkpoint route", "#fae7d0")]
diagram = Drawing(550, 220)
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="550" height="220" viewBox="0 0 550 220">', '<rect width="550" height="220" fill="white"/>']
for x,y,title,subtitle,color in boxes:
    diagram.add(Rect(x,y,246,62,fillColor=colors.HexColor(color),strokeColor=colors.HexColor("#bac7ce"),radius=6))
    diagram.add(String(x+12,y+39,title,fontName="Sans-Bold",fontSize=10,fillColor=colors.HexColor("#18323e")))
    diagram.add(String(x+12,y+18,subtitle,fontName="Sans",fontSize=8,fillColor=colors.HexColor("#344c57")))
    svg.extend([f'<rect x="{x}" y="{220-y-62}" width="246" height="62" rx="6" fill="{color}" stroke="#bac7ce"/>',
                f'<text x="{x+12}" y="{220-y-39}" font-family="Arial" font-size="10" font-weight="bold">{escape(title)}</text>',
                f'<text x="{x+12}" y="{220-y-18}" font-family="Arial" font-size="8">{escape(subtitle)}</text>'])
for x1,y1,x2,y2 in [(258,176,292,176),(135,145,135,107),(415,145,415,107)]:
    diagram.add(Line(x1,y1,x2,y2,strokeColor=colors.HexColor("#738b96")))
    svg.append(f'<line x1="{x1}" y1="{220-y1}" x2="{x2}" y2="{220-y2}" stroke="#738b96"/>')
svg.append('</svg>')
(root/'pipeline.svg').write_text('\n'.join(svg)+'\n')
styles=getSampleStyleSheet()
styles['Title'].fontName='Sans-Bold'
styles['Title'].fontSize=18
styles['Heading2'].fontName='Sans-Bold'
styles['Heading2'].fontSize=12
styles['Heading2'].spaceBefore=8
styles['Heading2'].spaceAfter=5
styles.add(ParagraphStyle(name='BodyCustom',fontName='Sans',fontSize=9,leading=12,spaceAfter=7,textColor=colors.HexColor('#253d48')))
body=styles['BodyCustom']
story=[Paragraph('Trace-selected C1 / 2LP comparison',styles['Title']),
       Paragraph('Implementation status | 27 October 2026',styles['Heading2']),Spacer(1,14),
       Paragraph('<b>Partial implementation. No measurement run or end-to-end saving.</b> The native trace-line builder, explicit-base four-summand 2LP reference and independent relation replay are implemented. The algebraic C1 oracle and complete native pricing remain open.',body),diagram,
       Paragraph('The diagram shows implementation and access status, not measured performance. Source: README.md, PROTOCOL.md and problems/trace-c1-2lp-measurement.md.',body),
       Paragraph('What the reference can establish',styles['Heading2']),
       Paragraph('The standard arm uses the actual embedded subfield. The selected arm uses the first multiplier satisfying the two relative-trace conditions under a declared cap. Both use the same nested-base policy and exact four-summand collector, with every accepted raw relation retained. The replay tool checks the point sums and projected linear equations with general BinaryCurve arithmetic and independently verifies recovered scalars.',body),
       Paragraph('What it cannot establish',styles['Heading2']),
       Paragraph('MITM lookup cost is not an algebraic Gaudry-Diem trial cost. Existing GAE records leave native work unpriced. A higher relation yield therefore does not establish a lower complete solve cost. No experiment session, independent-host audit, priced C1 result or validation-gate receipt is claimed.',body),
       Paragraph('Execution and next acceptance gate',styles['Heading2']),
       Paragraph('Implement and cross-check the shifted x-line algebraic oracle, price every setup and online phase, freeze exact curve identities and executable/image hashes, then submit through an existing authorised Iso Lab route after the S3 checkpoint gate passes. The design includes standard, selected, A/A and matched one-target rho arms, four field sizes and independent holdouts. No local benchmark substitutes for this experiment.',body),
       Paragraph('Evidence and sources',styles['Heading2']),
       Paragraph('PR #2686: github.com/isogeny-labs/crypto/pull/2686 (source head 76fc6b286358e9c51515cb3e03427d5ccc290e10). Implementation: src/cryptanalysis/ecbench_large_prime/trace_line.rs and ecbench/methods.rs. Replay: examples/trace_line_replay.rs. Development checks: validation/README.md. This report is descriptive; the versioned study files govern exact parameters and status.',body)]
def footer(c,doc):
    c.setFont('Sans',8);c.setFillColor(colors.HexColor('#607984'));c.drawString(36,22,'Isogeny Labs | trace_c1_2lp_20261027 | implementation report');c.drawRightString(576,22,str(doc.page))
SimpleDocTemplate(str(root/'REPORT.pdf'),pagesize=(612,792),leftMargin=36,rightMargin=36,topMargin=35,bottomMargin=38).build(story,onFirstPage=footer,onLaterPages=footer)
