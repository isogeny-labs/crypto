#!/usr/bin/env python3
"""Document layout only: reads recorded Rust values; performs no crypto or solving."""
from pathlib import Path
from html import escape
import json
import re
from reportlab.graphics.shapes import Drawing, Rect, Line, String
from reportlab.graphics import renderSVG
from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Preformatted, PageBreak

HERE = Path(__file__).resolve().parent
DATA = HERE.parents[1] / 'followons_20261029' / 'validation'


def diagrams():
    transfer = json.loads((DATA / 'transfer.json').read_text())['controls']
    sharing = json.loads((DATA / 'sharing.json').read_text())['controls']
    quotient = json.loads((DATA / 'quotient.json').read_text())['original_e0']
    facts = Drawing(510, 190)
    palette = ['#245d82', '#326e54', '#805421']
    lanes = [
        ['TRANSFER', f"Symmetric D2 rank {transfer[-1]['symmetric_degree2_closure_rank']}", f"{transfer[-1]['selected_consequences']} certified degree<=4 rows", 'Exact pullback + chain witness'],
        ['SHARING', 'All four branches retained', f"Input block rank {sharing[-1]['shared_input_block_rank']}", 'Same three refutations + one positive'],
        ['QUOTIENT', f"Kernel points {quotient['kernel_distinct']}", 'Phi = [23](-1-Frob)^3', f"{quotient['velu_sample_points_checked']} Velu comparisons; sign recorded"],
    ]
    for i, (title, left, middle, right) in enumerate(lanes):
        y = 137 - i * 52
        facts.add(String(0, y + 35, title, fontName='Helvetica-Bold', fontSize=9, fillColor=colors.HexColor(palette[i])))
        for j, text in enumerate([left, middle, right]):
            x = j * 173
            facts.add(Rect(x, y, 163, 29, strokeColor=colors.HexColor(palette[i]), fillColor=colors.HexColor('#f3f6f8'), strokeWidth=.7))
            import textwrap
            for k, part in enumerate(textwrap.wrap(text, 30)):
                facts.add(String(x + 6, y + 18 - k * 10, part, fontName='Helvetica', fontSize=7.7))
            if j < 2:
                facts.add(Line(x + 164, y + 14, x + 171, y + 14, strokeColor=colors.HexColor('#536572')))
    facts.add(String(0, 4, 'Fixed software controls. Complete costs/speedups unknown. No scaling or crossover claim.', fontSize=7.5, fontName='Helvetica-Oblique'))
    renderSVG.drawToFile(facts, str(HERE / 'component-evidence.svg'))

    ranks = Drawing(510, 250)
    labels = ['V=[1,2], x=0', 'V=[1,2], x=1', 'V=[1,4], x=2']
    panels = [
        ('Transfer dimensions', 0, 30,
         [('Symmetric D2', '#245d82', [c['symmetric_degree2_closure_rank'] for c in transfer]),
          ('Degree<=4 image', '#75a7c3', [c['selected_consequences'] for c in transfer])]),
        ('Branch-space dimensions', 270, 16,
         [('Common input block', '#326e54', [c['shared_input_block_rank'] for c in sharing]),
          ('After independent solves', '#9bc3a8', [c['shared_intersection_rank'] for c in sharing])]),
    ]
    for title, origin, maximum, series in panels:
        ranks.add(String(origin, 237, title, fontName='Helvetica-Bold', fontSize=10))
        ranks.add(String(origin, 224, 'Rank (independent Boolean rows)', fontName='Helvetica', fontSize=8))
        x0, y0, height = origin + 23, 54, 130
        ranks.add(Line(x0, y0, x0 + 212, y0, strokeColor=colors.grey))
        ranks.add(Line(x0, y0, x0, y0 + height + 5, strokeColor=colors.grey))
        for j, (name, color, values) in enumerate(series):
            ranks.add(Rect(origin, 207 - 12*j, 7, 7, fillColor=colors.HexColor(color), strokeWidth=0))
            ranks.add(String(origin + 11, 207 - 12*j, name, fontSize=7.6))
            for k, value in enumerate(values):
                x = x0 + 18 + 69*k + j*19
                h = height * value / maximum
                ranks.add(Rect(x, y0, 16, h, fillColor=colors.HexColor(color), strokeWidth=0))
                ranks.add(String(x + 3, y0 + h + 4, str(value), fontSize=8))
        for k, text in enumerate(labels):
            ranks.add(String(x0 + 7 + 69*k, 36, text, fontSize=6.9))
    ranks.add(String(0, 15, 'Native conformance records: transfer.json / sharing.json; three fixed controls; exact ranks, no uncertainty estimate.', fontSize=7.1))
    ranks.add(String(0, 3, 'Different spaces/stages are labelled. These dimensions are neither costs nor speedups; post-solve sharing is diagnostic.', fontSize=7.1))
    renderSVG.drawToFile(ranks, str(HERE / 'row-ranks.svg'))
    return {'component-evidence.svg': facts, 'row-ranks.svg': ranks}


def inline(text):
    text = escape(text)
    text = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', text)
    text = re.sub(r'`([^`]+)`', r'<font name="Courier">\1</font>', text)
    text = re.sub(r'\[([^]]+)\]\(([^)]+)\)', r'\1', text)
    def url(m):
        target = m.group(0).rstrip('.,')
        return '<link href="' + target + '" color="#245d82">' + target + '</link>'
    return re.sub(r'https?://[^\s<>]+', url, text)


def main():
    figures = diagrams()
    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle(name='ReportBody', fontName='Helvetica', fontSize=9.1, leading=12, spaceAfter=6))
    styles.add(ParagraphStyle(name='ReportTable', fontName='Helvetica', fontSize=7.5, leading=9))
    styles['Heading1'].fontSize = 13
    styles['Heading1'].leading = 16
    styles['Title'].fontSize = 18
    styles['Title'].leading = 22
    story = []
    lines = (HERE / 'REPORT.md').read_text().splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        if not line.strip():
            i += 1
            continue
        if line.startswith('# '):
            story.append(Paragraph(inline(line[2:]), styles['Title']))
            i += 1
        elif line.startswith('## '):
            if line[3:] == 'Sources, novelty and graphs':
                story.append(PageBreak())
            story.append(Paragraph(inline(line[3:]), styles['Heading1']))
            i += 1
        elif line.startswith('!['):
            name = re.search(r'\(([^)]+)\)', line).group(1)
            story.extend([Spacer(1, 4), figures[name], Spacer(1, 7)])
            i += 1
        elif line.startswith('|'):
            rows = []
            while i < len(lines) and lines[i].startswith('|'):
                cells = [x.strip() for x in lines[i].strip('|').split('|')]
                if not all(re.fullmatch(r'[:\- ]+', x) for x in cells):
                    rows.append([Paragraph(inline(x), styles['ReportTable']) for x in cells])
                i += 1
            width = 510 / len(rows[0])
            table = Table(rows, colWidths=[width] * len(rows[0]), repeatRows=1, hAlign='LEFT')
            table.setStyle(TableStyle([('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#e7eef3')),
                ('VALIGN', (0, 0), (-1, -1), 'TOP'), ('LINEBELOW', (0, 0), (-1, 0), .6, colors.grey),
                ('TOPPADDING', (0, 0), (-1, -1), 5), ('BOTTOMPADDING', (0, 0), (-1, -1), 5)]))
            story.extend([table, Spacer(1, 7)])
        elif line.startswith('    '):
            block = []
            while i < len(lines) and (lines[i].startswith('    ') or not lines[i].strip()):
                block.append(lines[i][4:] if lines[i].startswith('    ') else '')
                i += 1
            story.append(Preformatted('\n'.join(block).rstrip(), ParagraphStyle(name='Code', fontName='Courier', fontSize=8, leading=10, spaceAfter=7)))
        elif line.startswith('- '):
            block = [line[2:]]
            i += 1
            while i < len(lines) and lines[i].strip() and not lines[i].startswith(('- ', '#', '|', '![', '    ')):
                block.append(lines[i])
                i += 1
            story.append(Paragraph(inline(' '.join(block)), styles['ReportBody'], bulletText='\u2022'))
        else:
            block = []
            while i < len(lines) and lines[i].strip() and not lines[i].startswith(('- ', '#', '|', '![', '    ')):
                block.append(lines[i])
                i += 1
            story.append(Paragraph(inline(' '.join(block)), styles['ReportBody']))
    def footer(canvas, doc):
        canvas.saveState()
        canvas.setFont('Helvetica', 7.5)
        canvas.drawString(42, 24, '29 October 2026 | Component conformance; full cost and crossover unestablished')
        canvas.drawRightString(A4[0]-42, 24, str(doc.page))
        canvas.restoreState()
    SimpleDocTemplate(str(HERE/'report.pdf'), pagesize=A4, leftMargin=42, rightMargin=42,
        topMargin=36, bottomMargin=40, title='Checked components for three ECDLP follow-ons',
        author='ECDLP campaign', invariant=1).build(story, onFirstPage=footer, onLaterPages=footer)
    print(json.dumps({'pdf':'report.pdf','visuals':list(figures),'input_records':'followons_20261029/validation','role':'layout of recorded native values only'}))


if __name__ == '__main__':
    main()
