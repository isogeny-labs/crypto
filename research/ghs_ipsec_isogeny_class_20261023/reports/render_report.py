#!/usr/bin/env python3
"""Document rendering only: plot frozen counts and typeset the archive README.

No field arithmetic, search, certificate verification or new research result.
Requires matplotlib and reportlab; produces SVG, PNG and PDF beside this file.
"""
from pathlib import Path
import json
import re
from xml.sax.saxutils import escape

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle

ROOT = Path(__file__).resolve().parent


def plain(s):
    return re.sub(r'!?(\[[^\]]*\])\(([^)]+)\)', lambda m: m[1][1:-1]+' ('+m[2]+')', s).replace('`', '').replace('**', '')


def main():
    data = json.loads((ROOT/'screen-populations.json').read_text())
    labels, values = list(data['populations']), list(data['populations'].values())
    fig, ax = plt.subplots(figsize=(8.5, 3.1))
    bars = ax.barh(labels, values, color=['#7894b3', '#315e86', '#243f58'])
    ax.invert_yaxis()
    ax.set_xlim(0, 33000)
    ax.set_xlabel('Recorded absolute-Frobenius orbits (nested populations)')
    ax.set_title('IPsec group 3 search checkpoint - '+data['date'])
    for bar, value in zip(bars, values):
        ax.text(value+250, bar.get_y()+bar.get_height()/2, f'{value:,}', va='center')
    ax.spines[['top', 'right']].set_visible(False)
    fig.text(0.04, 0.01, '0 candidates in implemented screens. Not a full-class or security conclusion.', fontsize=9)
    fig.tight_layout(rect=(0, .06, 1, 1))
    fig.savefig(ROOT/'screen-populations.svg')
    svg = ROOT/'screen-populations.svg'
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')
    fig.savefig(ROOT/'screen-populations.png', dpi=150)
    plt.close(fig)

    styles = getSampleStyleSheet()
    styles['BodyText'].fontSize = 8.5
    styles['BodyText'].leading = 11
    story = []
    lines = (ROOT.parent/'README.md').read_text().splitlines()
    paragraph, table, code = [], [], False

    def flush():
        if paragraph:
            story.append(Paragraph(escape(plain(' '.join(paragraph))), styles['BodyText']))
            story.append(Spacer(1, 5))
            paragraph.clear()

    def flush_table():
        if table:
            cells = [[Paragraph(escape(plain(c.strip())), styles['BodyText']) for c in row.strip('|').split('|')] for row in table]
            widths = [1.6*inch, 1.35*inch, 3.85*inch]
            t = Table(cells, colWidths=widths[:len(cells[0])], repeatRows=1, hAlign='LEFT')
            t.setStyle(TableStyle([('BACKGROUND', (0,0), (-1,0), colors.HexColor('#e8eef3')), ('VALIGN',(0,0),(-1,-1),'TOP'), ('GRID',(0,0),(-1,-1),.3,colors.lightgrey)]))
            story.extend([t, Spacer(1,8)])
            table.clear()

    for line in lines+['']:
        if line.startswith('|'):
            flush()
            if not re.match(r'^\|[ :|-]+\|$', line):
                table.append(line)
            continue
        flush_table()
        if line.startswith('```'):
            flush()
            code = not code
        elif code:
            story.append(Paragraph(escape(line), styles['Code']))
        elif line.startswith('#'):
            flush()
            story.append(Paragraph(escape(line.lstrip('# ')), styles['Title'] if line.startswith('# ') else styles['Heading2']))
        elif line.startswith('!['):
            flush()
            story.append(Image(str(ROOT/'screen-populations.png'), width=6.8*inch, height=2.48*inch))
        elif not line.strip():
            flush()
        else:
            paragraph.append(line)
    pdf = SimpleDocTemplate(str(ROOT/'REPORT.pdf'), pagesize=(8.5*inch,11*inch), rightMargin=.6*inch, leftMargin=.6*inch, topMargin=.5*inch, bottomMargin=.5*inch)
    pdf.build(story)


if __name__ == '__main__':
    main()
