# Layout only: renders REPORT.md text and the recorded-value figure into REPORT.pdf.
import re, runpy
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Preformatted
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib.pagesizes import A4
from reportlab.graphics.shapes import Drawing
ns = runpy.run_path('figures/plot.py')
d = ns['d']; d.scale(0.78, 0.78); d.width *= 0.78; d.height *= 0.78
st = getSampleStyleSheet(); body = st['BodyText']; body.fontSize = 8.5; body.leading = 11
mono = st['Code']; mono.fontSize = 7
out = []; tbl = []
def flush():
    global tbl
    if tbl: out.append(Preformatted('\n'.join(tbl), mono)); tbl = []
for ln in open('REPORT.md').read().split('\n'):
    if ln.startswith('|'): tbl.append(ln); continue
    flush()
    if ln.startswith('# '): out.append(Paragraph(ln[2:], st['Title']))
    elif ln.startswith('#'): out.append(Paragraph(ln.lstrip('# '), st['Heading3']))
    elif ln.strip():
        t = ln.replace('&','&amp;').replace('<','&lt;')
        t = re.sub(r'`([^`]*)`', r'<font face="Courier">\1</font>', t)
        out.append(Paragraph(t, body))
    if ln.startswith('## Result'): out.append(d)
flush()
SimpleDocTemplate('REPORT.pdf', pagesize=A4, leftMargin=36, rightMargin=36, topMargin=36, bottomMargin=36).build(out)
