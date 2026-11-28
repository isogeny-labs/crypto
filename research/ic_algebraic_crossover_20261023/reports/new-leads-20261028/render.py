#!/usr/bin/env python3
"""Render the recorded proposal text only; no cryptographic computation."""
import argparse
import json
from pathlib import Path
from xml.sax.saxutils import escape

from reportlab.graphics import renderPDF, renderSVG
from reportlab.graphics.shapes import Drawing, Line, Rect, String
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.platypus import Paragraph, SimpleDocTemplate, Spacer, PageBreak


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output-dir', type=Path, required=True)
    args = parser.parse_args()
    source = Path(__file__).resolve().parent
    data = json.loads((source / 'shortlist.json').read_text())
    out = args.output_dir
    out.mkdir(parents=True, exist_ok=True)
    diagram = Drawing(510, 168)
    palette = ['#245d82', '#326e54', '#805421']
    labels = [('OBSERVED GAP', 8), ('PROPOSED BRIDGE', 181), ('ACCEPTANCE GATE', 354)]
    for label, x in labels:
        diagram.add(String(x, 158, label, fontName='Helvetica-Bold', fontSize=8, fillColor=colors.HexColor('#273545')))
    import textwrap
    for idx, lead in enumerate(data['leads']):
        y = 110 - 46 * idx
        for col, key in enumerate(['diagram_input', 'diagram_hypothesis', 'diagram_gate']):
            x = 173 * col
            diagram.add(Rect(x, y, 161, 39, fillColor=colors.HexColor('#f1f5f8'), strokeColor=colors.HexColor(palette[idx]), strokeWidth=0.7))
            for line, text in enumerate(textwrap.wrap(lead[key], width=31)):
                diagram.add(String(x + 7, y + 26 - line * 10, text, fontName='Helvetica', fontSize=8))
            if col < 2:
                diagram.add(Line(x + 163, y + 19, x + 170, y + 19, strokeColor=colors.HexColor('#48596a')))
    diagram.add(String(0, 3, 'ALL THREE BRIDGES ARE PROPOSED - no experiment run or measured gain', fontName='Helvetica-Oblique', fontSize=7, fillColor=colors.HexColor('#576575')))
    renderSVG.drawToFile(diagram, str(out / 'proposal-map.svg'))
    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle(name='BodySmall', fontName='Helvetica', fontSize=9, leading=11.6, spaceAfter=6, alignment=TA_LEFT))
    styles.add(ParagraphStyle(name='LeadTitle', fontName='Helvetica-Bold', fontSize=11.3, leading=13.5, spaceBefore=9, spaceAfter=5))
    styles.add(ParagraphStyle(name='NoteSmall', fontName='Helvetica', fontSize=7.8, leading=9.8, spaceAfter=5))
    story = []
    def para(text, style='BodySmall'):
        story.append(Paragraph(text, styles[style]))
    para(escape(data['title']), 'Title')
    para('28 October 2026 | Proposal note | No new scientific measurements', 'NoteSmall')
    para(escape(data['intro']))
    story.extend([Spacer(1, 5), diagram, Spacer(1, 8)])
    for lead in data['leads']:
        para(f"{lead['rank']}. {escape(lead['name'])}", 'LeadTitle')
        para('<b>Evidence.</b> ' + escape(lead['observation']))
        para('<b>New experiment.</b> ' + escape(lead['delta']))
    story.append(PageBreak())
    para('Discriminating tests and prior-work boundaries', 'Heading1')
    for lead in data['leads']:
        para(f"{lead['rank']}. {escape(lead['name'])}", 'LeadTitle')
        para(escape(lead['test']))
        para('<b>Stop condition.</b> ' + escape(lead['stop']))
        para('<b>Atlas scope.</b> ' + escape(lead['atlas']), 'NoteSmall')
    para('Evidence and limits', 'Heading2')
    para(escape(data['limits']), 'NoteSmall')
    para(escape(data['novelty_scope']), 'NoteSmall')
    for i, ref in enumerate(data['sources'], 1):
        para(f"[{i}] <link href=\"{escape(ref['url'])}\" color=\"#245d82\">{escape(ref['label'])}</link>", 'NoteSmall')
    para(escape(data['source_gaps']), 'NoteSmall')
    para(escape(data['canonical_graphs']), 'NoteSmall')
    def footer(canvas, doc):
        canvas.saveState()
        canvas.setFont('Helvetica', 7.5)
        canvas.setFillColor(colors.HexColor('#576575'))
        canvas.drawString(39, 23, 'ECDLP follow-on nominations | All three leads are proposed and untested')
        canvas.drawRightString(A4[0] - 39, 23, str(doc.page))
        canvas.restoreState()
    SimpleDocTemplate(str(out / 'report.pdf'), title=data['title'], subject=data['status'], pagesize=A4, rightMargin=39, leftMargin=39, topMargin=33, bottomMargin=37).build(story, onFirstPage=footer, onLaterPages=footer)
    md = ['# ' + data['title'], '', data['status'], '', data['intro'], '', '![Proposed routes](proposal-map.svg)', '']
    for lead in data['leads']:
        md.extend([f"## {lead['rank']}. {lead['name']}", '', '**Evidence.** ' + lead['observation'], '', '**New experiment.** ' + lead['delta'], '', '**Discriminating test.** ' + lead['test'], '', '**Stop.** ' + lead['stop'], '', '**Prior work.** ' + lead['atlas'], ''])
    md.extend(['## Scope and sources', '', data['limits'], '', data['novelty_scope'], ''])
    md.extend([f"- [{ref['label']}]({ref['url']})" for ref in data['sources']])
    md.extend(['', data['source_gaps'], '', data['canonical_graphs'], ''])
    (out / 'REPORT.md').write_text('\n'.join(md))
    for file in ['shortlist.json', 'render.py']:
        if (source / file).resolve() != (out / file).resolve():
            (out / file).write_bytes((source / file).read_bytes())
    print(json.dumps({'pdf': str(out / 'report.pdf'), 'diagram': str(out / 'proposal-map.svg'), 'status': data['status']}))


if __name__ == '__main__':
    main()
