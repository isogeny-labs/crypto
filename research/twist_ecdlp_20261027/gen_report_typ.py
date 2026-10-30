import re
D="/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp"
def esc(s):
    out=[]
    for ch in s:
        if ch in '\\#$@*_<>~`[]':
            out.append('\\'+ch)
        else: out.append(ch)
    return ''.join(out)
SRC=open(D+"/report_src.txt").read().split("\n")
out=['#set document(title: "Twist-ECDLP study: consolidated report")',
'#set page(paper: "a4", margin: 1.8cm, numbering: "1")',
'#set text(size: 9.5pt)',
'#set par(justify: false, leading: 0.55em)',
'#show heading.where(level: 2): set text(size: 12pt)',
'= Twist-ECDLP study: consolidated report','']
table=[]
def flush():
    global table
    if table:
        out.append('#table(columns: (2.4cm, 4.2cm, 1fr), inset: 4pt, stroke: 0.4pt + gray,')
        out.append(' table.header([*ID*], [*Status*], [*Key result*]),')
        for r in table:
            out.append(' '+', '.join('['+esc(c.strip())+']' for c in r)+',')
        out.append(')')
        out.append('')
        table=[]
for ln in SRC:
    if ln.startswith('FIG'):
        flush()
        out.append('#figure(image("summary.svg", width: 100%), caption: [Summary: (a) counted-cost ratio of each twist-based arm to its matched baseline (log scale, 1 = tie; no row below 1); (b) G3 observed over structurally predicted mixed Semaev solutions. Values transcribed from the experiments\' recorded files.])')
        out.append('')
    elif ln.startswith('## '):
        flush(); out.append('== '+esc(ln[3:])); out.append('')
    elif ln.startswith('| '):
        cells=ln.strip().strip('|').split('|')
        if cells[0].strip() in ('ID',) or set(cells[0].strip())<=set('-'): continue
        table.append(cells)
    elif ln.startswith('- '):
        flush(); out.append('- '+esc(ln[2:]))
    elif ln.strip()=='' :
        flush(); out.append('')
    else:
        flush()
        m=re.match(r'^([A-Z][A-Za-z ]{2,30}\.)\s(.*)$',ln)
        if m: out.append('*'+esc(m.group(1))+'* '+esc(m.group(2))); out.append('')
        else: out.append(esc(ln)); out.append('')
flush()
open(D+"/report.typ","w").write('\n'.join(out))
