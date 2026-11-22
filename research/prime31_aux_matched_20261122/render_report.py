#!/usr/bin/env python3
"""Document layout only: render recorded Markdown and its existing SVG.

No arithmetic experiment, statistical calculation or research result is run.
Requires reportlab. The SVG uses only local rectangles, paths and text; this
small renderer ignores external resources and embeds the drawing as vectors.
"""
from pathlib import Path
import html
import re
import xml.etree.ElementTree as ET

from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import landscape, A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.graphics.shapes import Drawing, Rect, Path as DrawPath, Polygon, String
from reportlab.platypus import (SimpleDocTemplate, Paragraph, Spacer, Table,
                               TableStyle, Image, PageBreak, KeepTogether)

ROOT = Path(__file__).resolve().parent
FONT = Path('/usr/share/fonts/truetype/dejavu')
pdfmetrics.registerFont(TTFont('DejaVu', str(FONT / 'DejaVuSans.ttf')))
pdfmetrics.registerFont(TTFont('DejaVu-Bold', str(FONT / 'DejaVuSans-Bold.ttf')))
pdfmetrics.registerFontFamily('DejaVu', normal='DejaVu', bold='DejaVu-Bold',
                            italic='DejaVu', boldItalic='DejaVu-Bold')
styles = getSampleStyleSheet()
styles.add(ParagraphStyle(name='BodyReport', fontName='DejaVu', fontSize=10.5,
                         leading=15, spaceAfter=9, textColor=colors.HexColor('#24394d')))
styles.add(ParagraphStyle(name='TitleReport', fontName='DejaVu-Bold', fontSize=24,
                         leading=30, spaceAfter=14, textColor=colors.HexColor('#172b40')))
styles.add(ParagraphStyle(name='HeadingReport', fontName='DejaVu-Bold', fontSize=15,
                         leading=20, spaceBefore=12, spaceAfter=9,
                         keepWithNext=True,
                         textColor=colors.HexColor('#174963')))
styles.add(ParagraphStyle(name='CellReport', fontName='DejaVu', fontSize=9,
                         leading=12, spaceAfter=0))

def inline(text):
    text = html.escape(text)
    text = re.sub(r'`([^`]+)`', r'<font name="DejaVu">\1</font>', text)
    text = re.sub(r'\*\*([^*]+)\*\*', r'<b>\1</b>', text)
    return text

def footer(canvas, doc):
    canvas.setStrokeColor(colors.HexColor('#d8e1ea'))
    canvas.line(36, 31, doc.pagesize[0] - 36, 31)
    canvas.setFont('DejaVu', 8)
    canvas.setFillColor(colors.HexColor('#5a6d7f'))
    canvas.drawString(36, 18, 'Isogeny Labs | Prime31 matched auxiliary bases | Implementation / campaign not launched')
    canvas.drawRightString(doc.pagesize[0] - 36, 18, str(doc.page))

def vector_figure(width):
    """Render the fixed diagram's supported SVG primitives, no external fetches."""
    svg = ET.parse(ROOT / 'pipeline.svg').getroot()
    height = 650
    drawing = Drawing(1080, height)
    def color(value):
        if value in (None, 'none'):
            return None
        if len(value) == 4 and value.startswith('#'):
            value = '#' + ''.join(c * 2 for c in value[1:])
        return colors.HexColor(value)
    def visit(node, inherited):
        tag = node.tag.rsplit('}', 1)[-1]
        if tag in ('defs','title','desc'):
            return
        attrs = dict(inherited)
        attrs.update(node.attrib)
        fill = color(attrs.get('fill', '#000000'))
        stroke = color(attrs.get('stroke'))
        weight = float(attrs.get('stroke-width', 1))
        if tag == 'rect':
            drawing.add(Rect(float(attrs.get('x',0)),height-float(attrs.get('y',0))-float(attrs['height']),
                float(attrs['width']),float(attrs['height']),rx=float(attrs.get('rx',0)),
                ry=float(attrs.get('rx',0)),fillColor=fill,strokeColor=stroke,strokeWidth=weight))
        elif tag == 'text':
            drawing.add(String(float(attrs['x']),height-float(attrs['y']),''.join(node.itertext()),
                fontName='DejaVu-Bold' if attrs.get('font-weight')=='700' else 'DejaVu',
                fontSize=float(attrs.get('font-size',15)),fillColor=fill))
        elif tag == 'path':
            tokens = re.findall(r'[MHVLZ]|-?\d+(?:\.\d+)?',attrs['d'])
            path = DrawPath(fillColor=fill,strokeColor=stroke,strokeWidth=weight)
            x=y=0; i=0
            while i<len(tokens):
                command=tokens[i]; i+=1
                if command in ('M','L'):
                    x=float(tokens[i]); y=float(tokens[i+1]); i+=2
                    (path.moveTo if command=='M' else path.lineTo)(x,height-y)
                elif command=='H':
                    x=float(tokens[i]);i+=1;path.lineTo(x,height-y)
                elif command=='V':
                    y=float(tokens[i]);i+=1;path.lineTo(x,height-y)
                elif command=='Z':path.close()
                else:raise ValueError('Unsupported fixed-diagram path')
            drawing.add(path)
            if attrs.get('marker-end'):
                drawing.add(Polygon([x,height-y,x-4,height-y+8,x+4,height-y+8],
                                    fillColor=stroke,strokeColor=None))
        if tag in ('svg','g'):
            for child in node:visit(child,attrs)
    visit(svg,{})
    factor=width/1080
    drawing.scale(factor,factor)
    drawing.width=width;drawing.height=height*factor
    return drawing

def render():
    page = landscape(A4)
    width = page[0] - 72
    doc = SimpleDocTemplate(str(ROOT / 'REPORT.pdf'), pagesize=page,
                            rightMargin=36, leftMargin=36, topMargin=30, bottomMargin=42,
                            title='Matched auxiliary factor bases on ordinary Prime31',
                            author='Isogeny Labs', pageCompression=1)
    lines = (ROOT / 'REPORT.md').read_text().splitlines()
    story = []
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        i += 1
        if not line:
            continue
        if line == '---PAGE---':
            story.append(PageBreak())
        elif line.startswith('# '):
            story.append(Paragraph(inline(line[2:]), styles['TitleReport']))
        elif line.startswith('## '):
            story.append(Paragraph(inline(line[3:]), styles['HeadingReport']))
        elif line.startswith('!['):
            story.append(vector_figure(width-40))
            story.append(Spacer(1, 6))
        elif line.startswith('|'):
            raw = [line]
            while i < len(lines) and lines[i].strip().startswith('|'):
                raw.append(lines[i].strip()); i += 1
            rows = []
            for row in raw:
                parts = [x.strip() for x in row.strip('|').split('|')]
                if all(re.fullmatch(r'[-: ]+', x) for x in parts):
                    continue
                rows.append([Paragraph(inline(x), styles['CellReport']) for x in parts])
            table = Table(rows, colWidths=[width * .29, width * .71], repeatRows=1,
                          hAlign='LEFT')
            table.setStyle(TableStyle([
                ('BACKGROUND',(0,0),(-1,0),colors.HexColor('#e4edf7')),
                ('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.white,colors.HexColor('#f6f8fb')]),
                ('VALIGN',(0,0),(-1,-1),'TOP'),
                ('LEFTPADDING',(0,0),(-1,-1),8),('RIGHTPADDING',(0,0),(-1,-1),8),
                    ('TOPPADDING',(0,0),(-1,-1),4),('BOTTOMPADDING',(0,0),(-1,-1),4),
                ('LINEBELOW',(0,0),(-1,0),.6,colors.HexColor('#8ea7ba'))]))
            story.append(table); story.append(Spacer(1, 10))
        else:
            paragraph = [line]
            while i < len(lines) and lines[i].strip() and not lines[i].startswith(('#','|','![','---PAGE---')):
                paragraph.append(lines[i].strip()); i += 1
            story.append(Paragraph(inline(' '.join(paragraph)), styles['BodyReport']))
    doc.build(story, onFirstPage=footer, onLaterPages=footer)

if __name__ == '__main__':
    render()
