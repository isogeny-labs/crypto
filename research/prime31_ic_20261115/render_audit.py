#!/usr/bin/env python3
"""Render the saved audit narrative and status diagram; no research arithmetic."""
from html import escape
from pathlib import Path
from reportlab.graphics.shapes import Drawing, Rect, String, Line
from reportlab.graphics import renderSVG
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Preformatted

root = Path(__file__).resolve().parent
d = Drawing(480, 180)
boxes = [
    (0, 105, 145, 65, '#e8eef7', ['Pinned source + saved data', 'C++ / JSONL / summary', 'head d5af3fb']),
    (167, 105, 145, 65, '#e1f2ea', ['Native local audit', 'answers + aggregation', '[pcc:unverified]']),
    (334, 105, 145, 65, '#fff0db', ['Open execution obligations', 'relations / costs / replay', 'independent signed verdict']),
]
for x, y, w, h, color, lines in boxes:
    d.add(Rect(x, y, w, h, fillColor=colors.HexColor(color), strokeColor=colors.HexColor('#607080')))
    for offset, text in enumerate(lines):
        d.add(String(x+w/2, y+h-18-offset*16, text, fontSize=8, textAnchor='middle'))
for x in (145, 312):
    d.add(Line(x, 137, x+22, 137, strokeColor=colors.HexColor('#607080')))
    d.add(Line(x+18, 141, x+22, 137))
    d.add(Line(x+18, 133, x+22, 137))
d.add(String(240, 72, 'Saved-answer correctness does not establish execution provenance.',
             textAnchor='middle', fontSize=10))
d.add(String(240, 51, '9 saved rows / 3 distinct public targets; no new performance trial.',
             textAnchor='middle', fontSize=9))
d.add(String(240, 30, 'Source: RUNS.jsonl, RESULTS.json, audit_recorded.results.json; 2026-11-15.',
             textAnchor='middle', fontSize=8))
renderSVG.drawToFile(d, str(root/'audit_scope.svg'))
styles = getSampleStyleSheet()
styles['BodyText'].fontSize = 9
styles['BodyText'].leading = 13
styles['Code'].fontSize = 6
styles['Code'].leading = 9
story = [Paragraph('Recorded-data audit: scope and evidence', styles['Title']), d, Spacer(1,12)]
paragraph = []
code = []
in_code = False
def flush():
    if paragraph:
        story.append(Paragraph(escape(' '.join(paragraph)), styles['BodyText']))
        story.append(Spacer(1,7))
        paragraph.clear()
for line in (root/'AUDIT.md').read_text().splitlines():
    if line.startswith('```'):
        flush()
        if in_code:
            # Visual wrapping only; the exact commands remain in AUDIT.md.
            import textwrap
            wrapped = '\n'.join('\n'.join(textwrap.wrap(x, width=110, replace_whitespace=False)) for x in code)
            story.append(Preformatted(wrapped, styles['Code']))
            code.clear()
        in_code = not in_code
    elif in_code:
        code.append(line)
    elif line.startswith('#'):
        flush()
        story.append(Paragraph(escape(line.lstrip('# ')), styles['Heading2']))
    elif not line.strip():
        flush()
    else:
        paragraph.append(line)
flush()
SimpleDocTemplate(str(root/'AUDIT.pdf'), leftMargin=48, rightMargin=48,
                  topMargin=36, bottomMargin=36).build(story)
