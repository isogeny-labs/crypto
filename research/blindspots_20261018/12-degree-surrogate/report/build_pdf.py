# Layout only: renders README.md and graph.png into REPORT.pdf (reportlab).
from reportlab.lib.pagesizes import A4
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, Table, TableStyle, PageBreak
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.lib import colors
import re
S='research/blindspots_20261018/12-degree-surrogate'
st=getSampleStyleSheet(); body=st['BodyText']; body.fontSize=9; body.leading=11.5
doc=SimpleDocTemplate(f'{S}/REPORT.pdf',pagesize=A4,leftMargin=40,rightMargin=40,topMargin=40,bottomMargin=40,title='Item 12: surrogate for the solving-degree surface')
fl=[]; lines=open(f'{S}/README.md').read().split('\n'); i=0
def inl(t):
    t=t.replace('&','&amp;').replace('<','&lt;').replace('>','&gt;')
    t=re.sub(r'\*\*(.+?)\*\*',r'<b>\1</b>',t); t=re.sub(r'`(.+?)`',r'<font face="Courier">\1</font>',t); return t
while i<len(lines):
    l=lines[i]
    if l.startswith('!['): i+=1; continue
    if l.startswith('# '): fl.append(Paragraph(inl(l[2:]),st['Title']))
    elif l.startswith('## '): fl.append(Spacer(1,6)); fl.append(Paragraph(inl(l[3:]),st['Heading2']))
    elif l.startswith('|'):
        rows=[]
        while i<len(lines) and lines[i].startswith('|'):
            if not set(lines[i])<=set('|- '): rows.append([Paragraph(inl(c.strip()),body) for c in lines[i].strip('|').split('|')])
            i+=1
        t=Table(rows,repeatRows=1); t.setStyle(TableStyle([('GRID',(0,0),(-1,-1),0.4,colors.grey),('BACKGROUND',(0,0),(-1,0),colors.lightgrey),('VALIGN',(0,0),(-1,-1),'TOP')])); fl.append(t); fl.append(Spacer(1,4)); continue
    elif l.startswith('* '):
        t=l[2:]
        while i+1<len(lines) and lines[i+1].startswith('  '): i+=1; t+=' '+lines[i].strip()
        fl.append(Paragraph('&bull; '+inl(t),body))
    elif l.strip():
        t=l
        while i+1<len(lines) and lines[i+1].strip() and not lines[i+1].startswith(('|','* ','#','!')): i+=1; t+=' '+lines[i]
        fl.append(Paragraph(inl(t),body))
    i+=1
fl.append(PageBreak()); fl.append(Paragraph('Graph: held-out MAE of D per fold, and the permutation null',st['Heading2']))
fl.append(Image(f'{S}/report/graph.png',width=515,height=515*520/980))
fl.append(Paragraph('Source: raw/evaluate.txt, raw/eval.kv, raw/null.kv (CARD-BLINDSPOT-12-02). Plotted values only; see report/graph-data.json.',body))
doc.build(fl)
