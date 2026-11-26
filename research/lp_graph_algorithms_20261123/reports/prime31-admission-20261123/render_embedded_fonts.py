#!/usr/bin/env python3
"""Render fixed document-layout commands with embedded fonts; no research arithmetic."""
import json
from pathlib import Path
import sys

from reportlab.pdfgen import canvas
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.colors import HexColor
from reportlab.lib.utils import ImageReader


def font_path(name):
    for root in (Path('/usr/share/fonts/truetype/liberation'),
                 Path('/usr/share/fonts/truetype/liberation2')):
        candidate = root / name
        if candidate.is_file():
            return str(candidate)
    raise FileNotFoundError(f'Install Liberation fonts to render: {name}')


def main():
    layout = json.loads(Path(sys.argv[1]).read_text())
    fonts = {
        'regular': 'LiberationSans-Regular.ttf',
        'bold': 'LiberationSans-Bold.ttf',
        'mono': 'LiberationMono-Regular.ttf',
    }
    for alias, filename in fonts.items():
        pdfmetrics.registerFont(TTFont(alias, font_path(filename)))
    c = canvas.Canvas(sys.argv[2], pagesize=(layout['width'], layout['height']),
                      pageCompression=1)
    c.setTitle('Prime31 group admission - native qualification')
    c.setAuthor('Isogeny Labs research')
    c.setSubject('PR #5076: source preservation, planted controls and natural-source gap')
    c.setCreator('Editable Node layout + ReportLab embedded-font rendering')
    for page in layout['pages']:
        for command in page:
            kind = command['type']
            if kind == 'text':
                c.setFont(command['font'], command['size'])
                c.setFillColor(HexColor(command['color']))
                c.drawString(command['x'], command['y'], command['text'])
            elif kind == 'rect':
                c.setFillColor(HexColor(command['fill']))
                if command['stroke']:
                    c.setStrokeColor(HexColor(command['stroke']))
                    c.setLineWidth(0.65)
                c.rect(command['x'], command['y'], command['width'], command['height'],
                       stroke=int(bool(command['stroke'])), fill=1)
            elif kind == 'line':
                c.setStrokeColor(HexColor(command['color']))
                c.setLineWidth(command['width'])
                c.line(command['x1'], command['y1'], command['x2'], command['y2'])
            elif kind == 'image':
                c.drawImage(ImageReader(command['path']), command['x'], command['y'],
                            width=command['width'], height=command['height'], mask='auto')
            elif kind == 'link':
                c.linkURL(command['url'], tuple(command['box']), relative=0, thickness=0)
            else:
                raise ValueError(f'Unknown layout command: {kind}')
        c.showPage()
    c.save()


if __name__ == '__main__':
    main()
