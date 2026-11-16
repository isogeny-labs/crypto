# Renders RESULT.md (headings, paragraphs, bullets, tables, one image) to RESULT.pdf. Layout only.
import re
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import mm
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, ListFlowable, ListItem
from reportlab.lib import colors
ss = getSampleStyleSheet()
body = ParagraphStyle('b', parent=ss['BodyText'], fontSize=8.6, leading=11.2)
cell = ParagraphStyle('c', parent=body, fontSize=7.4, leading=9.2)
def md(t):
    t = t.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;')
    t = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', t)
    t = re.sub(r'`(.+?)`', r'<font face="Courier" size="7.6">\1</font>', t)
    t = re.sub(r'\[([^\]]+)\]\([^)]+\)', r'\1', t)
    return t.replace('\\|', '|')
lines = open('RESULT.md').read().split('\n'); story = []; i = 0
while i < len(lines):
    l = lines[i]
    if l.startswith('# '): story += [Paragraph(md(l[2:]), ss['Title'])]
    elif l.startswith('## '): story += [Spacer(1, 5), Paragraph(md(l[3:]), ss['Heading2'])]
    elif l.startswith('!['): story += [Image('cross_isa.png', width=170*mm, height=75*mm)]
    elif l.startswith('|'):
        rows = []
        while i < len(lines) and lines[i].startswith('|'):
            if not re.match(r'^\|[\s:\-|]+\|$', lines[i]):
                rows.append([Paragraph(md(c.strip()), cell) for c in re.split(r'(?<!\\)\|', lines[i])[1:-1]])
            i += 1
        n = len(rows[0]); w = 180*mm / n
        t = Table(rows, colWidths=[w]*n, repeatRows=1)
        t.setStyle(TableStyle([('GRID', (0,0), (-1,-1), 0.3, colors.grey), ('BACKGROUND', (0,0), (-1,0), colors.HexColor('#e8edf3')), ('VALIGN', (0,0), (-1,-1), 'TOP')]))
        story += [t, Spacer(1, 4)]; continue
    elif re.match(r'^(\d+\.|-) ', l):
        items = []
        while i < len(lines) and re.match(r'^(\d+\.|-) ', lines[i]):
            items.append(ListItem(Paragraph(md(re.sub(r'^(\d+\.|-) ', '', lines[i])), body))); i += 1
        story += [ListFlowable(items, bulletType='bullet', start='•', leftIndent=10)]; continue
    elif l.strip(): story += [Paragraph(md(l), body), Spacer(1, 3)]
    i += 1
SimpleDocTemplate('RESULT.pdf', pagesize=A4, leftMargin=15*mm, rightMargin=15*mm, topMargin=14*mm, bottomMargin=14*mm, title='Item 1 cross-ISA').build(story)
